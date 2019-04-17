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

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>The <code>ActivatedRule</code> object in an <a>UpdateWebACL</a> request specifies a <code>Rule</code> that you want to insert or delete, the priority of the <code>Rule</code> in the <code>WebACL</code>, and the action that you want AWS WAF to take when a web request matches the <code>Rule</code> (<code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>).</p> <p>To specify whether to insert or delete a <code>Rule</code>, use the <code>Action</code> parameter in the <a>WebACLUpdate</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivatedRule {
    /// <p>Specifies the action that CloudFront or AWS WAF takes when a web request matches the conditions in the <code>Rule</code>. Valid values for <code>Action</code> include the following:</p> <ul> <li> <p> <code>ALLOW</code>: CloudFront responds with the requested object.</p> </li> <li> <p> <code>BLOCK</code>: CloudFront responds with an HTTP 403 (Forbidden) status code.</p> </li> <li> <p> <code>COUNT</code>: AWS WAF increments a counter of requests that match the conditions in the rule and then continues to inspect the web request based on the remaining rules in the web ACL. </p> </li> </ul> <p> <code>ActivatedRule|OverrideAction</code> applies only when updating or adding a <code>RuleGroup</code> to a <code>WebACL</code>. In this case, you do not use <code>ActivatedRule|Action</code>. For all other update requests, <code>ActivatedRule|Action</code> is used instead of <code>ActivatedRule|OverrideAction</code>.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<WafAction>,
    /// <p><p>An array of rules to exclude from a rule group. This is applicable only when the <code>ActivatedRule</code> refers to a <code>RuleGroup</code>.</p> <p>Sometimes it is necessary to troubleshoot rule groups that are blocking traffic unexpectedly (false positives). One troubleshooting technique is to identify the specific rule within the rule group that is blocking the legitimate traffic and then disable (exclude) that particular rule. You can exclude rules from both your own rule groups and AWS Marketplace rule groups that have been associated with a web ACL.</p> <p>Specifying <code>ExcludedRules</code> does not remove those rules from the rule group. Rather, it changes the action for the rules to <code>COUNT</code>. Therefore, requests that match an <code>ExcludedRule</code> are counted but not blocked. The <code>RuleGroup</code> owner will receive COUNT metrics for each <code>ExcludedRule</code>.</p> <p>If you want to exclude rules from a rule group that is already associated with a web ACL, perform the following steps:</p> <ol> <li> <p>Use the AWS WAF logs to identify the IDs of the rules that you want to exclude. For more information about the logs, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/logging.html">Logging Web ACL Traffic Information</a>.</p> </li> <li> <p>Submit an <a>UpdateWebACL</a> request that has two actions:</p> <ul> <li> <p>The first action deletes the existing rule group from the web ACL. That is, in the <a>UpdateWebACL</a> request, the first <code>Updates:Action</code> should be <code>DELETE</code> and <code>Updates:ActivatedRule:RuleId</code> should be the rule group that contains the rules that you want to exclude.</p> </li> <li> <p>The second action inserts the same rule group back in, but specifying the rules to exclude. That is, the second <code>Updates:Action</code> should be <code>INSERT</code>, <code>Updates:ActivatedRule:RuleId</code> should be the rule group that you just removed, and <code>ExcludedRules</code> should contain the rules that you want to exclude.</p> </li> </ul> </li> </ol></p>
    #[serde(rename = "ExcludedRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_rules: Option<Vec<ExcludedRule>>,
    /// <p>Use the <code>OverrideAction</code> to test your <code>RuleGroup</code>.</p> <p>Any rule in a <code>RuleGroup</code> can potentially block a request. If you set the <code>OverrideAction</code> to <code>None</code>, the <code>RuleGroup</code> will block a request if any individual rule in the <code>RuleGroup</code> matches the request and is configured to block that request. However if you first want to test the <code>RuleGroup</code>, set the <code>OverrideAction</code> to <code>Count</code>. The <code>RuleGroup</code> will then override any block action specified by individual rules contained within the group. Instead of blocking matching requests, those requests will be counted. You can view a record of counted requests using <a>GetSampledRequests</a>. </p> <p> <code>ActivatedRule|OverrideAction</code> applies only when updating or adding a <code>RuleGroup</code> to a <code>WebACL</code>. In this case you do not use <code>ActivatedRule|Action</code>. For all other update requests, <code>ActivatedRule|Action</code> is used instead of <code>ActivatedRule|OverrideAction</code>.</p>
    #[serde(rename = "OverrideAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_action: Option<WafOverrideAction>,
    /// <p>Specifies the order in which the <code>Rules</code> in a <code>WebACL</code> are evaluated. Rules with a lower value for <code>Priority</code> are evaluated before <code>Rules</code> with a higher value. The value must be a unique integer. If you add multiple <code>Rules</code> to a <code>WebACL</code>, the values don't need to be consecutive.</p>
    #[serde(rename = "Priority")]
    pub priority: i64,
    /// <p>The <code>RuleId</code> for a <code>Rule</code>. You use <code>RuleId</code> to get more information about a <code>Rule</code> (see <a>GetRule</a>), update a <code>Rule</code> (see <a>UpdateRule</a>), insert a <code>Rule</code> into a <code>WebACL</code> or delete a one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>Rule</code> from AWS WAF (see <a>DeleteRule</a>).</p> <p> <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>
    #[serde(rename = "RuleId")]
    pub rule_id: String,
    /// <p>The rule type, either <code>REGULAR</code>, as defined by <a>Rule</a>, <code>RATE_BASED</code>, as defined by <a>RateBasedRule</a>, or <code>GROUP</code>, as defined by <a>RuleGroup</a>. The default is REGULAR. Although this field is optional, be aware that if you try to add a RATE_BASED rule to a web ACL without setting the type, the <a>UpdateWebACL</a> request will fail because the request tries to add a REGULAR rule with the specified ID, which does not exist. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateWebACLRequest {
    /// <p><p>The ARN (Amazon Resource Name) of the resource to be protected, either an application load balancer or Amazon API Gateway stage. </p> <p>The ARN should be in one of the following formats:</p> <ul> <li> <p>For an Application Load Balancer: <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/app/<i>load-balancer-name</i>/<i>load-balancer-id</i> </code> </p> </li> <li> <p>For an Amazon API Gateway stage: <code>arn:aws:apigateway:<i>region</i>::/restapis/<i>api-id</i>/stages/<i>stage-name</i> </code> </p> </li> </ul></p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A unique identifier (ID) for the web ACL. </p>
    #[serde(rename = "WebACLId")]
    pub web_acl_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateWebACLResponse {}

/// <p>In a <a>GetByteMatchSet</a> request, <code>ByteMatchSet</code> is a complex type that contains the <code>ByteMatchSetId</code> and <code>Name</code> of a <code>ByteMatchSet</code>, and the values that you specified when you updated the <code>ByteMatchSet</code>. </p> <p>A complex type that contains <code>ByteMatchTuple</code> objects, which specify the parts of web requests that you want AWS WAF to inspect and the values that you want AWS WAF to search for. If a <code>ByteMatchSet</code> contains more than one <code>ByteMatchTuple</code> object, a request needs to match the settings in only one <code>ByteMatchTuple</code> to be considered a match.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ByteMatchSet {
    /// <p>The <code>ByteMatchSetId</code> for a <code>ByteMatchSet</code>. You use <code>ByteMatchSetId</code> to get information about a <code>ByteMatchSet</code> (see <a>GetByteMatchSet</a>), update a <code>ByteMatchSet</code> (see <a>UpdateByteMatchSet</a>), insert a <code>ByteMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>ByteMatchSet</code> from AWS WAF (see <a>DeleteByteMatchSet</a>).</p> <p> <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>
    #[serde(rename = "ByteMatchSetId")]
    pub byte_match_set_id: String,
    /// <p>Specifies the bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings.</p>
    #[serde(rename = "ByteMatchTuples")]
    pub byte_match_tuples: Vec<ByteMatchTuple>,
    /// <p>A friendly name or description of the <a>ByteMatchSet</a>. You can't change <code>Name</code> after you create a <code>ByteMatchSet</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Returned by <a>ListByteMatchSets</a>. Each <code>ByteMatchSetSummary</code> object includes the <code>Name</code> and <code>ByteMatchSetId</code> for one <a>ByteMatchSet</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ByteMatchSetSummary {
    /// <p>The <code>ByteMatchSetId</code> for a <code>ByteMatchSet</code>. You use <code>ByteMatchSetId</code> to get information about a <code>ByteMatchSet</code>, update a <code>ByteMatchSet</code>, remove a <code>ByteMatchSet</code> from a <code>Rule</code>, and delete a <code>ByteMatchSet</code> from AWS WAF.</p> <p> <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>
    #[serde(rename = "ByteMatchSetId")]
    pub byte_match_set_id: String,
    /// <p>A friendly name or description of the <a>ByteMatchSet</a>. You can't change <code>Name</code> after you create a <code>ByteMatchSet</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>In an <a>UpdateByteMatchSet</a> request, <code>ByteMatchSetUpdate</code> specifies whether to insert or delete a <a>ByteMatchTuple</a> and includes the settings for the <code>ByteMatchTuple</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ByteMatchSetUpdate {
    /// <p>Specifies whether to insert or delete a <a>ByteMatchTuple</a>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>Information about the part of a web request that you want AWS WAF to inspect and the value that you want AWS WAF to search for. If you specify <code>DELETE</code> for the value of <code>Action</code>, the <code>ByteMatchTuple</code> values must exactly match the values in the <code>ByteMatchTuple</code> that you want to delete from the <code>ByteMatchSet</code>.</p>
    #[serde(rename = "ByteMatchTuple")]
    pub byte_match_tuple: ByteMatchTuple,
}

/// <p>The bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByteMatchTuple {
    /// <p>The part of a web request that you want AWS WAF to search, such as a specified header or a query string. For more information, see <a>FieldToMatch</a>.</p>
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>Within the portion of a web request that you want to search (for example, in the query string, if any), specify where you want AWS WAF to search. Valid values include the following:</p> <p> <b>CONTAINS</b> </p> <p>The specified part of the web request must include the value of <code>TargetString</code>, but the location doesn't matter.</p> <p> <b>CONTAINS_WORD</b> </p> <p>The specified part of the web request must include the value of <code>TargetString</code>, and <code>TargetString</code> must contain only alphanumeric characters or underscore (A-Z, a-z, 0-9, or _). In addition, <code>TargetString</code> must be a word, which means one of the following:</p> <ul> <li> <p> <code>TargetString</code> exactly matches the value of the specified part of the web request, such as the value of a header.</p> </li> <li> <p> <code>TargetString</code> is at the beginning of the specified part of the web request and is followed by a character other than an alphanumeric character or underscore (_), for example, <code>BadBot;</code>.</p> </li> <li> <p> <code>TargetString</code> is at the end of the specified part of the web request and is preceded by a character other than an alphanumeric character or underscore (_), for example, <code>;BadBot</code>.</p> </li> <li> <p> <code>TargetString</code> is in the middle of the specified part of the web request and is preceded and followed by characters other than alphanumeric characters or underscore (_), for example, <code>-BadBot;</code>.</p> </li> </ul> <p> <b>EXACTLY</b> </p> <p>The value of the specified part of the web request must exactly match the value of <code>TargetString</code>.</p> <p> <b>STARTS_WITH</b> </p> <p>The value of <code>TargetString</code> must appear at the beginning of the specified part of the web request.</p> <p> <b>ENDS_WITH</b> </p> <p>The value of <code>TargetString</code> must appear at the end of the specified part of the web request.</p>
    #[serde(rename = "PositionalConstraint")]
    pub positional_constraint: String,
    /// <p>The value that you want AWS WAF to search for. AWS WAF searches for the specified string in the part of web requests that you specified in <code>FieldToMatch</code>. The maximum length of the value is 50 bytes.</p> <p>Valid values depend on the values that you specified for <code>FieldToMatch</code>:</p> <ul> <li> <p> <code>HEADER</code>: The value that you want AWS WAF to search for in the request header that you specified in <a>FieldToMatch</a>, for example, the value of the <code>User-Agent</code> or <code>Referer</code> header.</p> </li> <li> <p> <code>METHOD</code>: The HTTP method, which indicates the type of operation specified in the request. CloudFront supports the following methods: <code>DELETE</code>, <code>GET</code>, <code>HEAD</code>, <code>OPTIONS</code>, <code>PATCH</code>, <code>POST</code>, and <code>PUT</code>.</p> </li> <li> <p> <code>QUERY_STRING</code>: The value that you want AWS WAF to search for in the query string, which is the part of a URL that appears after a <code>?</code> character.</p> </li> <li> <p> <code>URI</code>: The value that you want AWS WAF to search for in the part of a URL that identifies a resource, for example, <code>/images/daily-ad.jpg</code>.</p> </li> <li> <p> <code>BODY</code>: The part of a request that contains any additional data that you want to send to your web server as the HTTP request body, such as data from a form. The request body immediately follows the request headers. Note that only the first <code>8192</code> bytes of the request body are forwarded to AWS WAF for inspection. To allow or block requests based on the length of the body, you can create a size constraint set. For more information, see <a>CreateSizeConstraintSet</a>. </p> </li> <li> <p> <code>SINGLE_QUERY_ARG</code>: The parameter in the query string that you will inspect, such as <i>UserName</i> or <i>SalesRegion</i>. The maximum length for <code>SINGLE_QUERY_ARG</code> is 30 characters.</p> </li> <li> <p> <code>ALL_QUERY_ARGS</code>: Similar to <code>SINGLE_QUERY_ARG</code>, but instead of inspecting a single parameter, AWS WAF inspects all parameters within the query string for the value or regex pattern that you specify in <code>TargetString</code>.</p> </li> </ul> <p>If <code>TargetString</code> includes alphabetic characters A-Z and a-z, note that the value is case sensitive.</p> <p> <b>If you're using the AWS WAF API</b> </p> <p>Specify a base64-encoded version of the value. The maximum length of the value before you base64-encode it is 50 bytes.</p> <p>For example, suppose the value of <code>Type</code> is <code>HEADER</code> and the value of <code>Data</code> is <code>User-Agent</code>. If you want to search the <code>User-Agent</code> header for the value <code>BadBot</code>, you base64-encode <code>BadBot</code> using MIME base64-encoding and include the resulting value, <code>QmFkQm90</code>, in the value of <code>TargetString</code>.</p> <p> <b>If you're using the AWS CLI or one of the AWS SDKs</b> </p> <p>The value that you want AWS WAF to search for. The SDK automatically base64 encodes the value.</p>
    #[serde(rename = "TargetString")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub target_string: Vec<u8>,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>TargetString</code> before inspecting a request for a match.</p> <p>You can only specify a single type of TextTransformation.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system command line command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \ " ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\f, formfeed, decimal 12</p> </li> <li> <p>\t, tab, decimal 9</p> </li> <li> <p>\n, newline, decimal 10</p> </li> <li> <p>\r, carriage return, decimal 13</p> </li> <li> <p>\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a "less than" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p>
    #[serde(rename = "TextTransformation")]
    pub text_transformation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateByteMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>A friendly name or description of the <a>ByteMatchSet</a>. You can't change <code>Name</code> after you create a <code>ByteMatchSet</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateByteMatchSetResponse {
    /// <p>A <a>ByteMatchSet</a> that contains no <code>ByteMatchTuple</code> objects.</p>
    #[serde(rename = "ByteMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_match_set: Option<ByteMatchSet>,
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateByteMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGeoMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>A friendly name or description of the <a>GeoMatchSet</a>. You can't change <code>Name</code> after you create the <code>GeoMatchSet</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateGeoMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateGeoMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>The <a>GeoMatchSet</a> returned in the <code>CreateGeoMatchSet</code> response. The <code>GeoMatchSet</code> contains no <code>GeoMatchConstraints</code>.</p>
    #[serde(rename = "GeoMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_match_set: Option<GeoMatchSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateIPSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>A friendly name or description of the <a>IPSet</a>. You can't change <code>Name</code> after you create the <code>IPSet</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateIPSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateIPSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>The <a>IPSet</a> returned in the <code>CreateIPSet</code> response.</p>
    #[serde(rename = "IPSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_set: Option<IPSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRateBasedRuleRequest {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>A friendly name or description for the metrics for this <code>RateBasedRule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace. You can't change the name of the metric after you create the <code>RateBasedRule</code>.</p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// <p>A friendly name or description of the <a>RateBasedRule</a>. You can't change the name of a <code>RateBasedRule</code> after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The field that AWS WAF uses to determine if requests are likely arriving from a single source and thus subject to rate monitoring. The only valid value for <code>RateKey</code> is <code>IP</code>. <code>IP</code> indicates that requests that arrive from the same IP address are subject to the <code>RateLimit</code> that is specified in the <code>RateBasedRule</code>.</p>
    #[serde(rename = "RateKey")]
    pub rate_key: String,
    /// <p>The maximum number of requests, which have an identical value in the field that is specified by <code>RateKey</code>, allowed in a five-minute period. If the number of requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule are also met, AWS WAF triggers the action that is specified for this rule.</p>
    #[serde(rename = "RateLimit")]
    pub rate_limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateRateBasedRuleResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>The <a>RateBasedRule</a> that is returned in the <code>CreateRateBasedRule</code> response.</p>
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<RateBasedRule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRegexMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>A friendly name or description of the <a>RegexMatchSet</a>. You can't change <code>Name</code> after you create a <code>RegexMatchSet</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateRegexMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRegexMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>A <a>RegexMatchSet</a> that contains no <code>RegexMatchTuple</code> objects.</p>
    #[serde(rename = "RegexMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_match_set: Option<RegexMatchSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRegexPatternSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>A friendly name or description of the <a>RegexPatternSet</a>. You can't change <code>Name</code> after you create a <code>RegexPatternSet</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateRegexPatternSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRegexPatternSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>A <a>RegexPatternSet</a> that contains no objects.</p>
    #[serde(rename = "RegexPatternSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_pattern_set: Option<RegexPatternSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRuleGroupRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>A friendly name or description for the metrics for this <code>RuleGroup</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace. You can't change the name of the metric after you create the <code>RuleGroup</code>.</p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// <p>A friendly name or description of the <a>RuleGroup</a>. You can't change <code>Name</code> after you create a <code>RuleGroup</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateRuleGroupResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRuleGroup</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>An empty <a>RuleGroup</a>.</p>
    #[serde(rename = "RuleGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group: Option<RuleGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRuleRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>A friendly name or description for the metrics for this <code>Rule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain white space. You can't change the name of the metric after you create the <code>Rule</code>.</p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// <p>A friendly name or description of the <a>Rule</a>. You can't change the name of a <code>Rule</code> after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateRuleResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>The <a>Rule</a> returned in the <code>CreateRule</code> response.</p>
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Rule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSizeConstraintSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>A friendly name or description of the <a>SizeConstraintSet</a>. You can't change <code>Name</code> after you create a <code>SizeConstraintSet</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateSizeConstraintSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateSizeConstraintSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>A <a>SizeConstraintSet</a> that contains no <code>SizeConstraint</code> objects.</p>
    #[serde(rename = "SizeConstraintSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_constraint_set: Option<SizeConstraintSet>,
}

/// <p>A request to create a <a>SqlInjectionMatchSet</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSqlInjectionMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>A friendly name or description for the <a>SqlInjectionMatchSet</a> that you're creating. You can't change <code>Name</code> after you create the <code>SqlInjectionMatchSet</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>The response to a <code>CreateSqlInjectionMatchSet</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateSqlInjectionMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateSqlInjectionMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>A <a>SqlInjectionMatchSet</a>.</p>
    #[serde(rename = "SqlInjectionMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_injection_match_set: Option<SqlInjectionMatchSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateWebACLRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The action that you want AWS WAF to take when a request doesn't match the criteria specified in any of the <code>Rule</code> objects that are associated with the <code>WebACL</code>.</p>
    #[serde(rename = "DefaultAction")]
    pub default_action: WafAction,
    /// <p>A friendly name or description for the metrics for this <code>WebACL</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain white space. You can't change <code>MetricName</code> after you create the <code>WebACL</code>.</p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// <p>A friendly name or description of the <a>WebACL</a>. You can't change <code>Name</code> after you create the <code>WebACL</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateWebACLResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateWebACL</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>The <a>WebACL</a> returned in the <code>CreateWebACL</code> response.</p>
    #[serde(rename = "WebACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl: Option<WebACL>,
}

/// <p>A request to create an <a>XssMatchSet</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateXssMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>A friendly name or description for the <a>XssMatchSet</a> that you're creating. You can't change <code>Name</code> after you create the <code>XssMatchSet</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>The response to a <code>CreateXssMatchSet</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateXssMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateXssMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>An <a>XssMatchSet</a>.</p>
    #[serde(rename = "XssMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xss_match_set: Option<XssMatchSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteByteMatchSetRequest {
    /// <p>The <code>ByteMatchSetId</code> of the <a>ByteMatchSet</a> that you want to delete. <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>
    #[serde(rename = "ByteMatchSetId")]
    pub byte_match_set_id: String,
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteByteMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteByteMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGeoMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>GeoMatchSetID</code> of the <a>GeoMatchSet</a> that you want to delete. <code>GeoMatchSetId</code> is returned by <a>CreateGeoMatchSet</a> and by <a>ListGeoMatchSets</a>.</p>
    #[serde(rename = "GeoMatchSetId")]
    pub geo_match_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteGeoMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteGeoMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIPSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>IPSetId</code> of the <a>IPSet</a> that you want to delete. <code>IPSetId</code> is returned by <a>CreateIPSet</a> and by <a>ListIPSets</a>.</p>
    #[serde(rename = "IPSetId")]
    pub ip_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteIPSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteIPSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLoggingConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the web ACL from which you want to delete the <a>LoggingConfiguration</a>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteLoggingConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePermissionPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the RuleGroup from which you want to delete the policy.</p> <p>The user making the request must be the owner of the RuleGroup.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeletePermissionPolicyResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRateBasedRuleRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>RuleId</code> of the <a>RateBasedRule</a> that you want to delete. <code>RuleId</code> is returned by <a>CreateRateBasedRule</a> and by <a>ListRateBasedRules</a>.</p>
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteRateBasedRuleResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRegexMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>RegexMatchSetId</code> of the <a>RegexMatchSet</a> that you want to delete. <code>RegexMatchSetId</code> is returned by <a>CreateRegexMatchSet</a> and by <a>ListRegexMatchSets</a>.</p>
    #[serde(rename = "RegexMatchSetId")]
    pub regex_match_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteRegexMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteRegexMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRegexPatternSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>RegexPatternSetId</code> of the <a>RegexPatternSet</a> that you want to delete. <code>RegexPatternSetId</code> is returned by <a>CreateRegexPatternSet</a> and by <a>ListRegexPatternSets</a>.</p>
    #[serde(rename = "RegexPatternSetId")]
    pub regex_pattern_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteRegexPatternSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteRegexPatternSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRuleGroupRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>RuleGroupId</code> of the <a>RuleGroup</a> that you want to delete. <code>RuleGroupId</code> is returned by <a>CreateRuleGroup</a> and by <a>ListRuleGroups</a>.</p>
    #[serde(rename = "RuleGroupId")]
    pub rule_group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteRuleGroupResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteRuleGroup</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRuleRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>RuleId</code> of the <a>Rule</a> that you want to delete. <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteRuleResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSizeConstraintSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>SizeConstraintSetId</code> of the <a>SizeConstraintSet</a> that you want to delete. <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>
    #[serde(rename = "SizeConstraintSetId")]
    pub size_constraint_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteSizeConstraintSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteSizeConstraintSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

/// <p>A request to delete a <a>SqlInjectionMatchSet</a> from AWS WAF.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSqlInjectionMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>SqlInjectionMatchSetId</code> of the <a>SqlInjectionMatchSet</a> that you want to delete. <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>
    #[serde(rename = "SqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: String,
}

/// <p>The response to a request to delete a <a>SqlInjectionMatchSet</a> from AWS WAF.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteSqlInjectionMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteSqlInjectionMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteWebACLRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>WebACLId</code> of the <a>WebACL</a> that you want to delete. <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>
    #[serde(rename = "WebACLId")]
    pub web_acl_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteWebACLResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteWebACL</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

/// <p>A request to delete an <a>XssMatchSet</a> from AWS WAF.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteXssMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>XssMatchSetId</code> of the <a>XssMatchSet</a> that you want to delete. <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>
    #[serde(rename = "XssMatchSetId")]
    pub xss_match_set_id: String,
}

/// <p>The response to a request to delete an <a>XssMatchSet</a> from AWS WAF.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteXssMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteXssMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateWebACLRequest {
    /// <p><p>The ARN (Amazon Resource Name) of the resource from which the web ACL is being removed, either an application load balancer or Amazon API Gateway stage.</p> <p>The ARN should be in one of the following formats:</p> <ul> <li> <p>For an Application Load Balancer: <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/app/<i>load-balancer-name</i>/<i>load-balancer-id</i> </code> </p> </li> <li> <p>For an Amazon API Gateway stage: <code>arn:aws:apigateway:<i>region</i>::/restapis/<i>api-id</i>/stages/<i>stage-name</i> </code> </p> </li> </ul></p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateWebACLResponse {}

/// <p>The rule to exclude from a rule group. This is applicable only when the <code>ActivatedRule</code> refers to a <code>RuleGroup</code>. The rule must belong to the <code>RuleGroup</code> that is specified by the <code>ActivatedRule</code>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExcludedRule {
    /// <p>The unique identifier for the rule to exclude from the rule group.</p>
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

/// <p>Specifies where in a web request to look for <code>TargetString</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldToMatch {
    /// <p>When the value of <code>Type</code> is <code>HEADER</code>, enter the name of the header that you want AWS WAF to search, for example, <code>User-Agent</code> or <code>Referer</code>. The name of the header is not case sensitive.</p> <p>When the value of <code>Type</code> is <code>SINGLE_QUERY_ARG</code>, enter the name of the parameter that you want AWS WAF to search, for example, <code>UserName</code> or <code>SalesRegion</code>. The parameter name is not case sensitive.</p> <p>If the value of <code>Type</code> is any other value, omit <code>Data</code>.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// <p><p>The part of the web request that you want AWS WAF to search for a specified string. Parts of a request that you can search include the following:</p> <ul> <li> <p> <code>HEADER</code>: A specified request header, for example, the value of the <code>User-Agent</code> or <code>Referer</code> header. If you choose <code>HEADER</code> for the type, specify the name of the header in <code>Data</code>.</p> </li> <li> <p> <code>METHOD</code>: The HTTP method, which indicated the type of operation that the request is asking the origin to perform. Amazon CloudFront supports the following methods: <code>DELETE</code>, <code>GET</code>, <code>HEAD</code>, <code>OPTIONS</code>, <code>PATCH</code>, <code>POST</code>, and <code>PUT</code>.</p> </li> <li> <p> <code>QUERY<em>STRING</code>: A query string, which is the part of a URL that appears after a <code>?</code> character, if any.</p> </li> <li> <p> <code>URI</code>: The part of a web request that identifies a resource, for example, <code>/images/daily-ad.jpg</code>.</p> </li> <li> <p> <code>BODY</code>: The part of a request that contains any additional data that you want to send to your web server as the HTTP request body, such as data from a form. The request body immediately follows the request headers. Note that only the first <code>8192</code> bytes of the request body are forwarded to AWS WAF for inspection. To allow or block requests based on the length of the body, you can create a size constraint set. For more information, see <a>CreateSizeConstraintSet</a>. </p> </li> <li> <p> <code>SINGLE</em>QUERY<em>ARG</code>: The parameter in the query string that you will inspect, such as <i>UserName</i> or <i>SalesRegion</i>. The maximum length for <code>SINGLE</em>QUERY<em>ARG</code> is 30 characters.</p> </li> <li> <p> <code>ALL</em>QUERY<em>ARGS</code>: Similar to <code>SINGLE</em>QUERY_ARG</code>, but rather than inspecting a single parameter, AWS WAF will inspect all parameters within the query for the value or regex pattern that you specify in <code>TargetString</code>.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>The country from which web requests originate that you want AWS WAF to search for.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoMatchConstraint {
    /// <p>The type of geographical area you want AWS WAF to search for. Currently <code>Country</code> is the only valid value.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>The country that you want AWS WAF to search for.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Contains one or more countries that AWS WAF will search for.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GeoMatchSet {
    /// <p>An array of <a>GeoMatchConstraint</a> objects, which contain the country that you want AWS WAF to search for.</p>
    #[serde(rename = "GeoMatchConstraints")]
    pub geo_match_constraints: Vec<GeoMatchConstraint>,
    /// <p>The <code>GeoMatchSetId</code> for an <code>GeoMatchSet</code>. You use <code>GeoMatchSetId</code> to get information about a <code>GeoMatchSet</code> (see <a>GeoMatchSet</a>), update a <code>GeoMatchSet</code> (see <a>UpdateGeoMatchSet</a>), insert a <code>GeoMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>GeoMatchSet</code> from AWS WAF (see <a>DeleteGeoMatchSet</a>).</p> <p> <code>GeoMatchSetId</code> is returned by <a>CreateGeoMatchSet</a> and by <a>ListGeoMatchSets</a>.</p>
    #[serde(rename = "GeoMatchSetId")]
    pub geo_match_set_id: String,
    /// <p>A friendly name or description of the <a>GeoMatchSet</a>. You can't change the name of an <code>GeoMatchSet</code> after you create it.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains the identifier and the name of the <code>GeoMatchSet</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GeoMatchSetSummary {
    /// <p>The <code>GeoMatchSetId</code> for an <a>GeoMatchSet</a>. You can use <code>GeoMatchSetId</code> in a <a>GetGeoMatchSet</a> request to get detailed information about an <a>GeoMatchSet</a>.</p>
    #[serde(rename = "GeoMatchSetId")]
    pub geo_match_set_id: String,
    /// <p>A friendly name or description of the <a>GeoMatchSet</a>. You can't change the name of an <code>GeoMatchSet</code> after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Specifies the type of update to perform to an <a>GeoMatchSet</a> with <a>UpdateGeoMatchSet</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GeoMatchSetUpdate {
    /// <p>Specifies whether to insert or delete a country with <a>UpdateGeoMatchSet</a>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>The country from which web requests originate that you want AWS WAF to search for.</p>
    #[serde(rename = "GeoMatchConstraint")]
    pub geo_match_constraint: GeoMatchConstraint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetByteMatchSetRequest {
    /// <p>The <code>ByteMatchSetId</code> of the <a>ByteMatchSet</a> that you want to get. <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>
    #[serde(rename = "ByteMatchSetId")]
    pub byte_match_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetByteMatchSetResponse {
    /// <p><p>Information about the <a>ByteMatchSet</a> that you specified in the <code>GetByteMatchSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>ByteMatchSet</a>: Contains <code>ByteMatchSetId</code>, <code>ByteMatchTuples</code>, and <code>Name</code> </p> </li> <li> <p> <code>ByteMatchTuples</code>: Contains an array of <a>ByteMatchTuple</a> objects. Each <code>ByteMatchTuple</code> object contains <a>FieldToMatch</a>, <code>PositionalConstraint</code>, <code>TargetString</code>, and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "ByteMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_match_set: Option<ByteMatchSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetChangeTokenRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetChangeTokenResponse {
    /// <p>The <code>ChangeToken</code> that you used in the request. Use this value in a <code>GetChangeTokenStatus</code> request to get the current status of the request. </p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetChangeTokenStatusRequest {
    /// <p>The change token for which you want to get the status. This change token was previously returned in the <code>GetChangeToken</code> response.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetChangeTokenStatusResponse {
    /// <p>The status of the change token.</p>
    #[serde(rename = "ChangeTokenStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGeoMatchSetRequest {
    /// <p>The <code>GeoMatchSetId</code> of the <a>GeoMatchSet</a> that you want to get. <code>GeoMatchSetId</code> is returned by <a>CreateGeoMatchSet</a> and by <a>ListGeoMatchSets</a>.</p>
    #[serde(rename = "GeoMatchSetId")]
    pub geo_match_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetGeoMatchSetResponse {
    /// <p>Information about the <a>GeoMatchSet</a> that you specified in the <code>GetGeoMatchSet</code> request. This includes the <code>Type</code>, which for a <code>GeoMatchContraint</code> is always <code>Country</code>, as well as the <code>Value</code>, which is the identifier for a specific country.</p>
    #[serde(rename = "GeoMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_match_set: Option<GeoMatchSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIPSetRequest {
    /// <p>The <code>IPSetId</code> of the <a>IPSet</a> that you want to get. <code>IPSetId</code> is returned by <a>CreateIPSet</a> and by <a>ListIPSets</a>.</p>
    #[serde(rename = "IPSetId")]
    pub ip_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetIPSetResponse {
    /// <p><p>Information about the <a>IPSet</a> that you specified in the <code>GetIPSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>IPSet</a>: Contains <code>IPSetDescriptors</code>, <code>IPSetId</code>, and <code>Name</code> </p> </li> <li> <p> <code>IPSetDescriptors</code>: Contains an array of <a>IPSetDescriptor</a> objects. Each <code>IPSetDescriptor</code> object contains <code>Type</code> and <code>Value</code> </p> </li> </ul></p>
    #[serde(rename = "IPSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_set: Option<IPSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLoggingConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the web ACL for which you want to get the <a>LoggingConfiguration</a>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetLoggingConfigurationResponse {
    /// <p>The <a>LoggingConfiguration</a> for the specified web ACL.</p>
    #[serde(rename = "LoggingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPermissionPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the RuleGroup for which you want to get the policy.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPermissionPolicyResponse {
    /// <p>The IAM policy attached to the specified RuleGroup.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRateBasedRuleManagedKeysRequest {
    /// <p>A null value and not currently used. Do not include this in your request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>The <code>RuleId</code> of the <a>RateBasedRule</a> for which you want to get a list of <code>ManagedKeys</code>. <code>RuleId</code> is returned by <a>CreateRateBasedRule</a> and by <a>ListRateBasedRules</a>.</p>
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRateBasedRuleManagedKeysResponse {
    /// <p>An array of IP addresses that currently are blocked by the specified <a>RateBasedRule</a>. </p>
    #[serde(rename = "ManagedKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_keys: Option<Vec<String>>,
    /// <p>A null value and not currently used.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRateBasedRuleRequest {
    /// <p>The <code>RuleId</code> of the <a>RateBasedRule</a> that you want to get. <code>RuleId</code> is returned by <a>CreateRateBasedRule</a> and by <a>ListRateBasedRules</a>.</p>
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRateBasedRuleResponse {
    /// <p>Information about the <a>RateBasedRule</a> that you specified in the <code>GetRateBasedRule</code> request.</p>
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<RateBasedRule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRegexMatchSetRequest {
    /// <p>The <code>RegexMatchSetId</code> of the <a>RegexMatchSet</a> that you want to get. <code>RegexMatchSetId</code> is returned by <a>CreateRegexMatchSet</a> and by <a>ListRegexMatchSets</a>.</p>
    #[serde(rename = "RegexMatchSetId")]
    pub regex_match_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRegexMatchSetResponse {
    /// <p>Information about the <a>RegexMatchSet</a> that you specified in the <code>GetRegexMatchSet</code> request. For more information, see <a>RegexMatchTuple</a>.</p>
    #[serde(rename = "RegexMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_match_set: Option<RegexMatchSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRegexPatternSetRequest {
    /// <p>The <code>RegexPatternSetId</code> of the <a>RegexPatternSet</a> that you want to get. <code>RegexPatternSetId</code> is returned by <a>CreateRegexPatternSet</a> and by <a>ListRegexPatternSets</a>.</p>
    #[serde(rename = "RegexPatternSetId")]
    pub regex_pattern_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRegexPatternSetResponse {
    /// <p>Information about the <a>RegexPatternSet</a> that you specified in the <code>GetRegexPatternSet</code> request, including the identifier of the pattern set and the regular expression patterns you want AWS WAF to search for. </p>
    #[serde(rename = "RegexPatternSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_pattern_set: Option<RegexPatternSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRuleGroupRequest {
    /// <p>The <code>RuleGroupId</code> of the <a>RuleGroup</a> that you want to get. <code>RuleGroupId</code> is returned by <a>CreateRuleGroup</a> and by <a>ListRuleGroups</a>.</p>
    #[serde(rename = "RuleGroupId")]
    pub rule_group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRuleGroupResponse {
    /// <p>Information about the <a>RuleGroup</a> that you specified in the <code>GetRuleGroup</code> request. </p>
    #[serde(rename = "RuleGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group: Option<RuleGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRuleRequest {
    /// <p>The <code>RuleId</code> of the <a>Rule</a> that you want to get. <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRuleResponse {
    /// <p><p>Information about the <a>Rule</a> that you specified in the <code>GetRule</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>Rule</a>: Contains <code>MetricName</code>, <code>Name</code>, an array of <code>Predicate</code> objects, and <code>RuleId</code> </p> </li> <li> <p> <a>Predicate</a>: Each <code>Predicate</code> object contains <code>DataId</code>, <code>Negated</code>, and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Rule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSampledRequestsRequest {
    /// <p>The number of requests that you want AWS WAF to return from among the first 5,000 requests that your AWS resource received during the time range. If your resource received fewer requests than the value of <code>MaxItems</code>, <code>GetSampledRequests</code> returns information about all of them. </p>
    #[serde(rename = "MaxItems")]
    pub max_items: i64,
    /// <p><p> <code>RuleId</code> is one of three values:</p> <ul> <li> <p>The <code>RuleId</code> of the <code>Rule</code> or the <code>RuleGroupId</code> of the <code>RuleGroup</code> for which you want <code>GetSampledRequests</code> to return a sample of requests.</p> </li> <li> <p> <code>Default_Action</code>, which causes <code>GetSampledRequests</code> to return a sample of the requests that didn&#39;t match any of the rules in the specified <code>WebACL</code>.</p> </li> </ul></p>
    #[serde(rename = "RuleId")]
    pub rule_id: String,
    /// <p>The start date and time and the end date and time of the range for which you want <code>GetSampledRequests</code> to return a sample of requests. Specify the date and time in the following format: <code>"2016-09-27T14:50Z"</code>. You can specify any time range in the previous three hours.</p>
    #[serde(rename = "TimeWindow")]
    pub time_window: TimeWindow,
    /// <p>The <code>WebACLId</code> of the <code>WebACL</code> for which you want <code>GetSampledRequests</code> to return a sample of requests.</p>
    #[serde(rename = "WebAclId")]
    pub web_acl_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSampledRequestsResponse {
    /// <p>The total number of requests from which <code>GetSampledRequests</code> got a sample of <code>MaxItems</code> requests. If <code>PopulationSize</code> is less than <code>MaxItems</code>, the sample includes every request that your AWS resource received during the specified time range.</p>
    #[serde(rename = "PopulationSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub population_size: Option<i64>,
    /// <p>A complex type that contains detailed information about each of the requests in the sample.</p>
    #[serde(rename = "SampledRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampled_requests: Option<Vec<SampledHTTPRequest>>,
    /// <p>Usually, <code>TimeWindow</code> is the time range that you specified in the <code>GetSampledRequests</code> request. However, if your AWS resource received more than 5,000 requests during the time range that you specified in the request, <code>GetSampledRequests</code> returns the time range for the first 5,000 requests.</p>
    #[serde(rename = "TimeWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window: Option<TimeWindow>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSizeConstraintSetRequest {
    /// <p>The <code>SizeConstraintSetId</code> of the <a>SizeConstraintSet</a> that you want to get. <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>
    #[serde(rename = "SizeConstraintSetId")]
    pub size_constraint_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSizeConstraintSetResponse {
    /// <p><p>Information about the <a>SizeConstraintSet</a> that you specified in the <code>GetSizeConstraintSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>SizeConstraintSet</a>: Contains <code>SizeConstraintSetId</code>, <code>SizeConstraints</code>, and <code>Name</code> </p> </li> <li> <p> <code>SizeConstraints</code>: Contains an array of <a>SizeConstraint</a> objects. Each <code>SizeConstraint</code> object contains <a>FieldToMatch</a>, <code>TextTransformation</code>, <code>ComparisonOperator</code>, and <code>Size</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "SizeConstraintSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_constraint_set: Option<SizeConstraintSet>,
}

/// <p>A request to get a <a>SqlInjectionMatchSet</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSqlInjectionMatchSetRequest {
    /// <p>The <code>SqlInjectionMatchSetId</code> of the <a>SqlInjectionMatchSet</a> that you want to get. <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>
    #[serde(rename = "SqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: String,
}

/// <p>The response to a <a>GetSqlInjectionMatchSet</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSqlInjectionMatchSetResponse {
    /// <p><p>Information about the <a>SqlInjectionMatchSet</a> that you specified in the <code>GetSqlInjectionMatchSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>SqlInjectionMatchSet</a>: Contains <code>Name</code>, <code>SqlInjectionMatchSetId</code>, and an array of <code>SqlInjectionMatchTuple</code> objects</p> </li> <li> <p> <a>SqlInjectionMatchTuple</a>: Each <code>SqlInjectionMatchTuple</code> object contains <code>FieldToMatch</code> and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "SqlInjectionMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_injection_match_set: Option<SqlInjectionMatchSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetWebACLForResourceRequest {
    /// <p><p>The ARN (Amazon Resource Name) of the resource for which to get the web ACL, either an application load balancer or Amazon API Gateway stage.</p> <p>The ARN should be in one of the following formats:</p> <ul> <li> <p>For an Application Load Balancer: <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/app/<i>load-balancer-name</i>/<i>load-balancer-id</i> </code> </p> </li> <li> <p>For an Amazon API Gateway stage: <code>arn:aws:apigateway:<i>region</i>::/restapis/<i>api-id</i>/stages/<i>stage-name</i> </code> </p> </li> </ul></p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetWebACLForResourceResponse {
    /// <p>Information about the web ACL that you specified in the <code>GetWebACLForResource</code> request. If there is no associated resource, a null WebACLSummary is returned.</p>
    #[serde(rename = "WebACLSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_summary: Option<WebACLSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetWebACLRequest {
    /// <p>The <code>WebACLId</code> of the <a>WebACL</a> that you want to get. <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>
    #[serde(rename = "WebACLId")]
    pub web_acl_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetWebACLResponse {
    /// <p><p>Information about the <a>WebACL</a> that you specified in the <code>GetWebACL</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>WebACL</a>: Contains <code>DefaultAction</code>, <code>MetricName</code>, <code>Name</code>, an array of <code>Rule</code> objects, and <code>WebACLId</code> </p> </li> <li> <p> <code>DefaultAction</code> (Data type is <a>WafAction</a>): Contains <code>Type</code> </p> </li> <li> <p> <code>Rules</code>: Contains an array of <code>ActivatedRule</code> objects, which contain <code>Action</code>, <code>Priority</code>, and <code>RuleId</code> </p> </li> <li> <p> <code>Action</code>: Contains <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "WebACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl: Option<WebACL>,
}

/// <p>A request to get an <a>XssMatchSet</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetXssMatchSetRequest {
    /// <p>The <code>XssMatchSetId</code> of the <a>XssMatchSet</a> that you want to get. <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>
    #[serde(rename = "XssMatchSetId")]
    pub xss_match_set_id: String,
}

/// <p>The response to a <a>GetXssMatchSet</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetXssMatchSetResponse {
    /// <p><p>Information about the <a>XssMatchSet</a> that you specified in the <code>GetXssMatchSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>XssMatchSet</a>: Contains <code>Name</code>, <code>XssMatchSetId</code>, and an array of <code>XssMatchTuple</code> objects</p> </li> <li> <p> <a>XssMatchTuple</a>: Each <code>XssMatchTuple</code> object contains <code>FieldToMatch</code> and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "XssMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xss_match_set: Option<XssMatchSet>,
}

/// <p>The response from a <a>GetSampledRequests</a> request includes an <code>HTTPHeader</code> complex type that appears as <code>Headers</code> in the response syntax. <code>HTTPHeader</code> contains the names and values of all of the headers that appear in one of the web requests that were returned by <code>GetSampledRequests</code>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HTTPHeader {
    /// <p>The name of one of the headers in the sampled web request.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of one of the headers in the sampled web request.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The response from a <a>GetSampledRequests</a> request includes an <code>HTTPRequest</code> complex type that appears as <code>Request</code> in the response syntax. <code>HTTPRequest</code> contains information about one of the web requests that were returned by <code>GetSampledRequests</code>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HTTPRequest {
    /// <p><p>The IP address that the request originated from. If the <code>WebACL</code> is associated with a CloudFront distribution, this is the value of one of the following fields in CloudFront access logs:</p> <ul> <li> <p> <code>c-ip</code>, if the viewer did not use an HTTP proxy or a load balancer to send the request</p> </li> <li> <p> <code>x-forwarded-for</code>, if the viewer did use an HTTP proxy or a load balancer to send the request</p> </li> </ul></p>
    #[serde(rename = "ClientIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    /// <p>The two-letter country code for the country that the request originated from. For a current list of country codes, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1 alpha-2</a>.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <p>The HTTP version specified in the sampled web request, for example, <code>HTTP/1.1</code>.</p>
    #[serde(rename = "HTTPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_version: Option<String>,
    /// <p>A complex type that contains two values for each header in the sampled web request: the name of the header and the value of the header.</p>
    #[serde(rename = "Headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HTTPHeader>>,
    /// <p>The HTTP method specified in the sampled web request. CloudFront supports the following methods: <code>DELETE</code>, <code>GET</code>, <code>HEAD</code>, <code>OPTIONS</code>, <code>PATCH</code>, <code>POST</code>, and <code>PUT</code>. </p>
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// <p>The part of a web request that identifies the resource, for example, <code>/images/daily-ad.jpg</code>.</p>
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// <p>Contains one or more IP addresses or blocks of IP addresses specified in Classless Inter-Domain Routing (CIDR) notation. AWS WAF supports IPv4 address ranges: /8 and any range between /16 through /32. AWS WAF supports IPv6 address ranges: /16, /24, /32, /48, /56, /64, and /128.</p> <p>To specify an individual IP address, you specify the four-part IP address followed by a <code>/32</code>, for example, 192.0.2.0/31. To block a range of IP addresses, you can specify /8 or any range between /16 through /32 (for IPv4) or /16, /24, /32, /48, /56, /64, or /128 (for IPv6). For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IPSet {
    /// <p>The IP address type (<code>IPV4</code> or <code>IPV6</code>) and the IP address range (in CIDR notation) that web requests originate from. If the <code>WebACL</code> is associated with a CloudFront distribution and the viewer did not use an HTTP proxy or a load balancer to send the request, this is the value of the c-ip field in the CloudFront access logs.</p>
    #[serde(rename = "IPSetDescriptors")]
    pub ip_set_descriptors: Vec<IPSetDescriptor>,
    /// <p>The <code>IPSetId</code> for an <code>IPSet</code>. You use <code>IPSetId</code> to get information about an <code>IPSet</code> (see <a>GetIPSet</a>), update an <code>IPSet</code> (see <a>UpdateIPSet</a>), insert an <code>IPSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete an <code>IPSet</code> from AWS WAF (see <a>DeleteIPSet</a>).</p> <p> <code>IPSetId</code> is returned by <a>CreateIPSet</a> and by <a>ListIPSets</a>.</p>
    #[serde(rename = "IPSetId")]
    pub ip_set_id: String,
    /// <p>A friendly name or description of the <a>IPSet</a>. You can't change the name of an <code>IPSet</code> after you create it.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Specifies the IP address type (<code>IPV4</code> or <code>IPV6</code>) and the IP address range (in CIDR format) that web requests originate from.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IPSetDescriptor {
    /// <p>Specify <code>IPV4</code> or <code>IPV6</code>.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p><p>Specify an IPv4 address by using CIDR notation. For example:</p> <ul> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from the IP address 192.0.2.44, specify <code>192.0.2.44/32</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify <code>192.0.2.0/24</code>.</p> </li> </ul> <p>For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p> <p>Specify an IPv6 address by using CIDR notation. For example:</p> <ul> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify <code>1111:0000:0000:0000:0000:0000:0000:0111/128</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify <code>1111:0000:0000:0000:0000:0000:0000:0000/64</code>.</p> </li> </ul></p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Contains the identifier and the name of the <code>IPSet</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IPSetSummary {
    /// <p>The <code>IPSetId</code> for an <a>IPSet</a>. You can use <code>IPSetId</code> in a <a>GetIPSet</a> request to get detailed information about an <a>IPSet</a>.</p>
    #[serde(rename = "IPSetId")]
    pub ip_set_id: String,
    /// <p>A friendly name or description of the <a>IPSet</a>. You can't change the name of an <code>IPSet</code> after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Specifies the type of update to perform to an <a>IPSet</a> with <a>UpdateIPSet</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct IPSetUpdate {
    /// <p>Specifies whether to insert or delete an IP address with <a>UpdateIPSet</a>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>The IP address type (<code>IPV4</code> or <code>IPV6</code>) and the IP address range (in CIDR notation) that web requests originate from.</p>
    #[serde(rename = "IPSetDescriptor")]
    pub ip_set_descriptor: IPSetDescriptor,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListActivatedRulesInRuleGroupRequest {
    /// <p>Specifies the number of <code>ActivatedRules</code> that you want AWS WAF to return for this request. If you have more <code>ActivatedRules</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>ActivatedRules</code>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>ActivatedRules</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>ActivatedRules</code>. For the second and subsequent <code>ListActivatedRulesInRuleGroup</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>ActivatedRules</code>.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>The <code>RuleGroupId</code> of the <a>RuleGroup</a> for which you want to get a list of <a>ActivatedRule</a> objects.</p>
    #[serde(rename = "RuleGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListActivatedRulesInRuleGroupResponse {
    /// <p>An array of <code>ActivatedRules</code> objects.</p>
    #[serde(rename = "ActivatedRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_rules: Option<Vec<ActivatedRule>>,
    /// <p>If you have more <code>ActivatedRules</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>ActivatedRules</code>, submit another <code>ListActivatedRulesInRuleGroup</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListByteMatchSetsRequest {
    /// <p>Specifies the number of <code>ByteMatchSet</code> objects that you want AWS WAF to return for this request. If you have more <code>ByteMatchSets</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>ByteMatchSet</code> objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>ByteMatchSets</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>ByteMatchSets</code>. For the second and subsequent <code>ListByteMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>ByteMatchSets</code>.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListByteMatchSetsResponse {
    /// <p>An array of <a>ByteMatchSetSummary</a> objects.</p>
    #[serde(rename = "ByteMatchSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_match_sets: Option<Vec<ByteMatchSetSummary>>,
    /// <p>If you have more <code>ByteMatchSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>ByteMatchSet</code> objects, submit another <code>ListByteMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGeoMatchSetsRequest {
    /// <p>Specifies the number of <code>GeoMatchSet</code> objects that you want AWS WAF to return for this request. If you have more <code>GeoMatchSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>GeoMatchSet</code> objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>GeoMatchSet</code>s than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>GeoMatchSet</code> objects. For the second and subsequent <code>ListGeoMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>GeoMatchSet</code> objects.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListGeoMatchSetsResponse {
    /// <p>An array of <a>GeoMatchSetSummary</a> objects.</p>
    #[serde(rename = "GeoMatchSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_match_sets: Option<Vec<GeoMatchSetSummary>>,
    /// <p>If you have more <code>GeoMatchSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>GeoMatchSet</code> objects, submit another <code>ListGeoMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListIPSetsRequest {
    /// <p>Specifies the number of <code>IPSet</code> objects that you want AWS WAF to return for this request. If you have more <code>IPSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>IPSet</code> objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>IPSets</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>IPSets</code>. For the second and subsequent <code>ListIPSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>IPSets</code>.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListIPSetsResponse {
    /// <p>An array of <a>IPSetSummary</a> objects.</p>
    #[serde(rename = "IPSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_sets: Option<Vec<IPSetSummary>>,
    /// <p>If you have more <code>IPSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>IPSet</code> objects, submit another <code>ListIPSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLoggingConfigurationsRequest {
    /// <p>Specifies the number of <code>LoggingConfigurations</code> that you want AWS WAF to return for this request. If you have more <code>LoggingConfigurations</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>LoggingConfigurations</code>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>LoggingConfigurations</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>LoggingConfigurations</code>. For the second and subsequent <code>ListLoggingConfigurations</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>ListLoggingConfigurations</code>.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListLoggingConfigurationsResponse {
    /// <p>An array of <a>LoggingConfiguration</a> objects.</p>
    #[serde(rename = "LoggingConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configurations: Option<Vec<LoggingConfiguration>>,
    /// <p>If you have more <code>LoggingConfigurations</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>LoggingConfigurations</code>, submit another <code>ListLoggingConfigurations</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRateBasedRulesRequest {
    /// <p>Specifies the number of <code>Rules</code> that you want AWS WAF to return for this request. If you have more <code>Rules</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>Rules</code>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>Rules</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>Rules</code>. For the second and subsequent <code>ListRateBasedRules</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>Rules</code>.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListRateBasedRulesResponse {
    /// <p>If you have more <code>Rules</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>Rules</code>, submit another <code>ListRateBasedRules</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>RuleSummary</a> objects.</p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RuleSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRegexMatchSetsRequest {
    /// <p>Specifies the number of <code>RegexMatchSet</code> objects that you want AWS WAF to return for this request. If you have more <code>RegexMatchSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>RegexMatchSet</code> objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>RegexMatchSet</code> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>ByteMatchSets</code>. For the second and subsequent <code>ListRegexMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>RegexMatchSet</code> objects.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListRegexMatchSetsResponse {
    /// <p>If you have more <code>RegexMatchSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>RegexMatchSet</code> objects, submit another <code>ListRegexMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>RegexMatchSetSummary</a> objects.</p>
    #[serde(rename = "RegexMatchSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_match_sets: Option<Vec<RegexMatchSetSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRegexPatternSetsRequest {
    /// <p>Specifies the number of <code>RegexPatternSet</code> objects that you want AWS WAF to return for this request. If you have more <code>RegexPatternSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>RegexPatternSet</code> objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>RegexPatternSet</code> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>RegexPatternSet</code> objects. For the second and subsequent <code>ListRegexPatternSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>RegexPatternSet</code> objects.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListRegexPatternSetsResponse {
    /// <p>If you have more <code>RegexPatternSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>RegexPatternSet</code> objects, submit another <code>ListRegexPatternSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>RegexPatternSetSummary</a> objects.</p>
    #[serde(rename = "RegexPatternSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_pattern_sets: Option<Vec<RegexPatternSetSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourcesForWebACLRequest {
    /// <p>The type of resource to list, either an application load balancer or Amazon API Gateway.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The unique identifier (ID) of the web ACL for which to list the associated resources.</p>
    #[serde(rename = "WebACLId")]
    pub web_acl_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListResourcesForWebACLResponse {
    /// <p>An array of ARNs (Amazon Resource Names) of the resources associated with the specified web ACL. An array with zero elements is returned if there are no resources associated with the web ACL.</p>
    #[serde(rename = "ResourceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRuleGroupsRequest {
    /// <p>Specifies the number of <code>RuleGroups</code> that you want AWS WAF to return for this request. If you have more <code>RuleGroups</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>RuleGroups</code>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>RuleGroups</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>RuleGroups</code>. For the second and subsequent <code>ListRuleGroups</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>RuleGroups</code>.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListRuleGroupsResponse {
    /// <p>If you have more <code>RuleGroups</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>RuleGroups</code>, submit another <code>ListRuleGroups</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>RuleGroup</a> objects.</p>
    #[serde(rename = "RuleGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_groups: Option<Vec<RuleGroupSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRulesRequest {
    /// <p>Specifies the number of <code>Rules</code> that you want AWS WAF to return for this request. If you have more <code>Rules</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>Rules</code>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>Rules</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>Rules</code>. For the second and subsequent <code>ListRules</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>Rules</code>.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListRulesResponse {
    /// <p>If you have more <code>Rules</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>Rules</code>, submit another <code>ListRules</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>RuleSummary</a> objects.</p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RuleSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSizeConstraintSetsRequest {
    /// <p>Specifies the number of <code>SizeConstraintSet</code> objects that you want AWS WAF to return for this request. If you have more <code>SizeConstraintSets</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>SizeConstraintSet</code> objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>SizeConstraintSets</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>SizeConstraintSets</code>. For the second and subsequent <code>ListSizeConstraintSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>SizeConstraintSets</code>.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListSizeConstraintSetsResponse {
    /// <p>If you have more <code>SizeConstraintSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>SizeConstraintSet</code> objects, submit another <code>ListSizeConstraintSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>SizeConstraintSetSummary</a> objects.</p>
    #[serde(rename = "SizeConstraintSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_constraint_sets: Option<Vec<SizeConstraintSetSummary>>,
}

/// <p>A request to list the <a>SqlInjectionMatchSet</a> objects created by the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSqlInjectionMatchSetsRequest {
    /// <p>Specifies the number of <a>SqlInjectionMatchSet</a> objects that you want AWS WAF to return for this request. If you have more <code>SqlInjectionMatchSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>Rules</code>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <a>SqlInjectionMatchSet</a> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>SqlInjectionMatchSets</code>. For the second and subsequent <code>ListSqlInjectionMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>SqlInjectionMatchSets</code>.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

/// <p>The response to a <a>ListSqlInjectionMatchSets</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListSqlInjectionMatchSetsResponse {
    /// <p>If you have more <a>SqlInjectionMatchSet</a> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>SqlInjectionMatchSet</code> objects, submit another <code>ListSqlInjectionMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>SqlInjectionMatchSetSummary</a> objects.</p>
    #[serde(rename = "SqlInjectionMatchSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_injection_match_sets: Option<Vec<SqlInjectionMatchSetSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSubscribedRuleGroupsRequest {
    /// <p>Specifies the number of subscribed rule groups that you want AWS WAF to return for this request. If you have more objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>ByteMatchSets</code>subscribed rule groups than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of subscribed rule groups. For the second and subsequent <code>ListSubscribedRuleGroupsRequest</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of subscribed rule groups.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListSubscribedRuleGroupsResponse {
    /// <p>If you have more objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more objects, submit another <code>ListSubscribedRuleGroups</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>RuleGroup</a> objects.</p>
    #[serde(rename = "RuleGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_groups: Option<Vec<SubscribedRuleGroupSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListWebACLsRequest {
    /// <p>Specifies the number of <code>WebACL</code> objects that you want AWS WAF to return for this request. If you have more <code>WebACL</code> objects than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>WebACL</code> objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>WebACL</code> objects than the number that you specify for <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>WebACL</code> objects. For the second and subsequent <code>ListWebACLs</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>WebACL</code> objects.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListWebACLsResponse {
    /// <p>If you have more <code>WebACL</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>WebACL</code> objects, submit another <code>ListWebACLs</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>WebACLSummary</a> objects.</p>
    #[serde(rename = "WebACLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ac_ls: Option<Vec<WebACLSummary>>,
}

/// <p>A request to list the <a>XssMatchSet</a> objects created by the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListXssMatchSetsRequest {
    /// <p>Specifies the number of <a>XssMatchSet</a> objects that you want AWS WAF to return for this request. If you have more <code>XssMatchSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>Rules</code>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <a>XssMatchSet</a> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>XssMatchSets</code>. For the second and subsequent <code>ListXssMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>XssMatchSets</code>.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

/// <p>The response to a <a>ListXssMatchSets</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListXssMatchSetsResponse {
    /// <p>If you have more <a>XssMatchSet</a> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>XssMatchSet</code> objects, submit another <code>ListXssMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>XssMatchSetSummary</a> objects.</p>
    #[serde(rename = "XssMatchSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xss_match_sets: Option<Vec<XssMatchSetSummary>>,
}

/// <p>The Amazon Kinesis Data Firehose, <code>RedactedFields</code> information, and the web ACL Amazon Resource Name (ARN).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingConfiguration {
    /// <p>An array of Amazon Kinesis Data Firehose ARNs.</p>
    #[serde(rename = "LogDestinationConfigs")]
    pub log_destination_configs: Vec<String>,
    /// <p>The parts of the request that you want redacted from the logs. For example, if you redact the cookie field, the cookie field in the firehose will be <code>xxx</code>. </p>
    #[serde(rename = "RedactedFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted_fields: Option<Vec<FieldToMatch>>,
    /// <p>The Amazon Resource Name (ARN) of the web ACL that you want to associate with <code>LogDestinationConfigs</code>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

/// <p>Specifies the <a>ByteMatchSet</a>, <a>IPSet</a>, <a>SqlInjectionMatchSet</a>, <a>XssMatchSet</a>, <a>RegexMatchSet</a>, <a>GeoMatchSet</a>, and <a>SizeConstraintSet</a> objects that you want to add to a <code>Rule</code> and, for each object, indicates whether you want to negate the settings, for example, requests that do NOT originate from the IP address 192.0.2.44. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Predicate {
    /// <p>A unique identifier for a predicate in a <code>Rule</code>, such as <code>ByteMatchSetId</code> or <code>IPSetId</code>. The ID is returned by the corresponding <code>Create</code> or <code>List</code> command.</p>
    #[serde(rename = "DataId")]
    pub data_id: String,
    /// <p>Set <code>Negated</code> to <code>False</code> if you want AWS WAF to allow, block, or count requests based on the settings in the specified <a>ByteMatchSet</a>, <a>IPSet</a>, <a>SqlInjectionMatchSet</a>, <a>XssMatchSet</a>, <a>RegexMatchSet</a>, <a>GeoMatchSet</a>, or <a>SizeConstraintSet</a>. For example, if an <code>IPSet</code> includes the IP address <code>192.0.2.44</code>, AWS WAF will allow or block requests based on that IP address.</p> <p>Set <code>Negated</code> to <code>True</code> if you want AWS WAF to allow or block a request based on the negation of the settings in the <a>ByteMatchSet</a>, <a>IPSet</a>, <a>SqlInjectionMatchSet</a>, <a>XssMatchSet</a>, <a>RegexMatchSet</a>, <a>GeoMatchSet</a>, or <a>SizeConstraintSet</a>. For example, if an <code>IPSet</code> includes the IP address <code>192.0.2.44</code>, AWS WAF will allow, block, or count requests based on all IP addresses <i>except</i> <code>192.0.2.44</code>.</p>
    #[serde(rename = "Negated")]
    pub negated: bool,
    /// <p>The type of predicate in a <code>Rule</code>, such as <code>ByteMatch</code> or <code>IPSet</code>.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutLoggingConfigurationRequest {
    /// <p>The Amazon Kinesis Data Firehose that contains the inspected traffic information, the redacted fields details, and the Amazon Resource Name (ARN) of the web ACL to monitor.</p>
    #[serde(rename = "LoggingConfiguration")]
    pub logging_configuration: LoggingConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutLoggingConfigurationResponse {
    /// <p>The <a>LoggingConfiguration</a> that you submitted in the request.</p>
    #[serde(rename = "LoggingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutPermissionPolicyRequest {
    /// <p>The policy to attach to the specified RuleGroup.</p>
    #[serde(rename = "Policy")]
    pub policy: String,
    /// <p>The Amazon Resource Name (ARN) of the RuleGroup to which you want to attach the policy.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutPermissionPolicyResponse {}

/// <p>A <code>RateBasedRule</code> is identical to a regular <a>Rule</a>, with one addition: a <code>RateBasedRule</code> counts the number of requests that arrive from a specified IP address every five minutes. For example, based on recent requests that you've seen from an attacker, you might create a <code>RateBasedRule</code> that includes the following conditions: </p> <ul> <li> <p>The requests come from 192.0.2.44.</p> </li> <li> <p>They contain the value <code>BadBot</code> in the <code>User-Agent</code> header.</p> </li> </ul> <p>In the rule, you also define the rate limit as 15,000.</p> <p>Requests that meet both of these conditions and exceed 15,000 requests every five minutes trigger the rule's action (block or count), which is defined in the web ACL.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RateBasedRule {
    /// <p>The <code>Predicates</code> object contains one <code>Predicate</code> element for each <a>ByteMatchSet</a>, <a>IPSet</a>, or <a>SqlInjectionMatchSet</a> object that you want to include in a <code>RateBasedRule</code>.</p>
    #[serde(rename = "MatchPredicates")]
    pub match_predicates: Vec<Predicate>,
    /// <p>A friendly name or description for the metrics for a <code>RateBasedRule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace. You can't change the name of the metric after you create the <code>RateBasedRule</code>.</p>
    #[serde(rename = "MetricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>A friendly name or description for a <code>RateBasedRule</code>. You can't change the name of a <code>RateBasedRule</code> after you create it.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The field that AWS WAF uses to determine if requests are likely arriving from single source and thus subject to rate monitoring. The only valid value for <code>RateKey</code> is <code>IP</code>. <code>IP</code> indicates that requests arriving from the same IP address are subject to the <code>RateLimit</code> that is specified in the <code>RateBasedRule</code>.</p>
    #[serde(rename = "RateKey")]
    pub rate_key: String,
    /// <p>The maximum number of requests, which have an identical value in the field specified by the <code>RateKey</code>, allowed in a five-minute period. If the number of requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule are also met, AWS WAF triggers the action that is specified for this rule.</p>
    #[serde(rename = "RateLimit")]
    pub rate_limit: i64,
    /// <p>A unique identifier for a <code>RateBasedRule</code>. You use <code>RuleId</code> to get more information about a <code>RateBasedRule</code> (see <a>GetRateBasedRule</a>), update a <code>RateBasedRule</code> (see <a>UpdateRateBasedRule</a>), insert a <code>RateBasedRule</code> into a <code>WebACL</code> or delete one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>RateBasedRule</code> from AWS WAF (see <a>DeleteRateBasedRule</a>).</p>
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

/// <p>In a <a>GetRegexMatchSet</a> request, <code>RegexMatchSet</code> is a complex type that contains the <code>RegexMatchSetId</code> and <code>Name</code> of a <code>RegexMatchSet</code>, and the values that you specified when you updated the <code>RegexMatchSet</code>.</p> <p> The values are contained in a <code>RegexMatchTuple</code> object, which specify the parts of web requests that you want AWS WAF to inspect and the values that you want AWS WAF to search for. If a <code>RegexMatchSet</code> contains more than one <code>RegexMatchTuple</code> object, a request needs to match the settings in only one <code>ByteMatchTuple</code> to be considered a match.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RegexMatchSet {
    /// <p>A friendly name or description of the <a>RegexMatchSet</a>. You can't change <code>Name</code> after you create a <code>RegexMatchSet</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The <code>RegexMatchSetId</code> for a <code>RegexMatchSet</code>. You use <code>RegexMatchSetId</code> to get information about a <code>RegexMatchSet</code> (see <a>GetRegexMatchSet</a>), update a <code>RegexMatchSet</code> (see <a>UpdateRegexMatchSet</a>), insert a <code>RegexMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>RegexMatchSet</code> from AWS WAF (see <a>DeleteRegexMatchSet</a>).</p> <p> <code>RegexMatchSetId</code> is returned by <a>CreateRegexMatchSet</a> and by <a>ListRegexMatchSets</a>.</p>
    #[serde(rename = "RegexMatchSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_match_set_id: Option<String>,
    /// <p><p>Contains an array of <a>RegexMatchTuple</a> objects. Each <code>RegexMatchTuple</code> object contains: </p> <ul> <li> <p>The part of a web request that you want AWS WAF to inspect, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The identifier of the pattern (a regular expression) that you want AWS WAF to look for. For more information, see <a>RegexPatternSet</a>.</p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul></p>
    #[serde(rename = "RegexMatchTuples")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_match_tuples: Option<Vec<RegexMatchTuple>>,
}

/// <p>Returned by <a>ListRegexMatchSets</a>. Each <code>RegexMatchSetSummary</code> object includes the <code>Name</code> and <code>RegexMatchSetId</code> for one <a>RegexMatchSet</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RegexMatchSetSummary {
    /// <p>A friendly name or description of the <a>RegexMatchSet</a>. You can't change <code>Name</code> after you create a <code>RegexMatchSet</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The <code>RegexMatchSetId</code> for a <code>RegexMatchSet</code>. You use <code>RegexMatchSetId</code> to get information about a <code>RegexMatchSet</code>, update a <code>RegexMatchSet</code>, remove a <code>RegexMatchSet</code> from a <code>Rule</code>, and delete a <code>RegexMatchSet</code> from AWS WAF.</p> <p> <code>RegexMatchSetId</code> is returned by <a>CreateRegexMatchSet</a> and by <a>ListRegexMatchSets</a>.</p>
    #[serde(rename = "RegexMatchSetId")]
    pub regex_match_set_id: String,
}

/// <p>In an <a>UpdateRegexMatchSet</a> request, <code>RegexMatchSetUpdate</code> specifies whether to insert or delete a <a>RegexMatchTuple</a> and includes the settings for the <code>RegexMatchTuple</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegexMatchSetUpdate {
    /// <p>Specifies whether to insert or delete a <a>RegexMatchTuple</a>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>Information about the part of a web request that you want AWS WAF to inspect and the identifier of the regular expression (regex) pattern that you want AWS WAF to search for. If you specify <code>DELETE</code> for the value of <code>Action</code>, the <code>RegexMatchTuple</code> values must exactly match the values in the <code>RegexMatchTuple</code> that you want to delete from the <code>RegexMatchSet</code>.</p>
    #[serde(rename = "RegexMatchTuple")]
    pub regex_match_tuple: RegexMatchTuple,
}

/// <p><p>The regular expression pattern that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings. Each <code>RegexMatchTuple</code> object contains: </p> <ul> <li> <p>The part of a web request that you want AWS WAF to inspect, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The identifier of the pattern (a regular expression) that you want AWS WAF to look for. For more information, see <a>RegexPatternSet</a>. </p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegexMatchTuple {
    /// <p>Specifies where in a web request to look for the <code>RegexPatternSet</code>.</p>
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>The <code>RegexPatternSetId</code> for a <code>RegexPatternSet</code>. You use <code>RegexPatternSetId</code> to get information about a <code>RegexPatternSet</code> (see <a>GetRegexPatternSet</a>), update a <code>RegexPatternSet</code> (see <a>UpdateRegexPatternSet</a>), insert a <code>RegexPatternSet</code> into a <code>RegexMatchSet</code> or delete one from a <code>RegexMatchSet</code> (see <a>UpdateRegexMatchSet</a>), and delete an <code>RegexPatternSet</code> from AWS WAF (see <a>DeleteRegexPatternSet</a>).</p> <p> <code>RegexPatternSetId</code> is returned by <a>CreateRegexPatternSet</a> and by <a>ListRegexPatternSets</a>.</p>
    #[serde(rename = "RegexPatternSetId")]
    pub regex_pattern_set_id: String,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>RegexPatternSet</code> before inspecting a request for a match.</p> <p>You can only specify a single type of TextTransformation.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system commandline command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \ " ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\f, formfeed, decimal 12</p> </li> <li> <p>\t, tab, decimal 9</p> </li> <li> <p>\n, newline, decimal 10</p> </li> <li> <p>\r, carriage return, decimal 13</p> </li> <li> <p>\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a "less than" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p>
    #[serde(rename = "TextTransformation")]
    pub text_transformation: String,
}

/// <p>The <code>RegexPatternSet</code> specifies the regular expression (regex) pattern that you want AWS WAF to search for, such as <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RegexPatternSet {
    /// <p>A friendly name or description of the <a>RegexPatternSet</a>. You can't change <code>Name</code> after you create a <code>RegexPatternSet</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The identifier for the <code>RegexPatternSet</code>. You use <code>RegexPatternSetId</code> to get information about a <code>RegexPatternSet</code>, update a <code>RegexPatternSet</code>, remove a <code>RegexPatternSet</code> from a <code>RegexMatchSet</code>, and delete a <code>RegexPatternSet</code> from AWS WAF.</p> <p> <code>RegexMatchSetId</code> is returned by <a>CreateRegexPatternSet</a> and by <a>ListRegexPatternSets</a>.</p>
    #[serde(rename = "RegexPatternSetId")]
    pub regex_pattern_set_id: String,
    /// <p>Specifies the regular expression (regex) patterns that you want AWS WAF to search for, such as <code>B[a@]dB[o0]t</code>.</p>
    #[serde(rename = "RegexPatternStrings")]
    pub regex_pattern_strings: Vec<String>,
}

/// <p>Returned by <a>ListRegexPatternSets</a>. Each <code>RegexPatternSetSummary</code> object includes the <code>Name</code> and <code>RegexPatternSetId</code> for one <a>RegexPatternSet</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RegexPatternSetSummary {
    /// <p>A friendly name or description of the <a>RegexPatternSet</a>. You can't change <code>Name</code> after you create a <code>RegexPatternSet</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The <code>RegexPatternSetId</code> for a <code>RegexPatternSet</code>. You use <code>RegexPatternSetId</code> to get information about a <code>RegexPatternSet</code>, update a <code>RegexPatternSet</code>, remove a <code>RegexPatternSet</code> from a <code>RegexMatchSet</code>, and delete a <code>RegexPatternSet</code> from AWS WAF.</p> <p> <code>RegexPatternSetId</code> is returned by <a>CreateRegexPatternSet</a> and by <a>ListRegexPatternSets</a>.</p>
    #[serde(rename = "RegexPatternSetId")]
    pub regex_pattern_set_id: String,
}

/// <p>In an <a>UpdateRegexPatternSet</a> request, <code>RegexPatternSetUpdate</code> specifies whether to insert or delete a <code>RegexPatternString</code> and includes the settings for the <code>RegexPatternString</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegexPatternSetUpdate {
    /// <p>Specifies whether to insert or delete a <code>RegexPatternString</code>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>Specifies the regular expression (regex) pattern that you want AWS WAF to search for, such as <code>B[a@]dB[o0]t</code>.</p>
    #[serde(rename = "RegexPatternString")]
    pub regex_pattern_string: String,
}

/// <p>A combination of <a>ByteMatchSet</a>, <a>IPSet</a>, and/or <a>SqlInjectionMatchSet</a> objects that identify the web requests that you want to allow, block, or count. For example, you might create a <code>Rule</code> that includes the following predicates:</p> <ul> <li> <p>An <code>IPSet</code> that causes AWS WAF to search for web requests that originate from the IP address <code>192.0.2.44</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that causes AWS WAF to search for web requests for which the value of the <code>User-Agent</code> header is <code>BadBot</code>.</p> </li> </ul> <p>To match the settings in this <code>Rule</code>, a request must originate from <code>192.0.2.44</code> AND include a <code>User-Agent</code> header for which the value is <code>BadBot</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Rule {
    /// <p>A friendly name or description for the metrics for this <code>Rule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace. You can't change <code>MetricName</code> after you create the <code>Rule</code>.</p>
    #[serde(rename = "MetricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>The friendly name or description for the <code>Rule</code>. You can't change the name of a <code>Rule</code> after you create it.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The <code>Predicates</code> object contains one <code>Predicate</code> element for each <a>ByteMatchSet</a>, <a>IPSet</a>, or <a>SqlInjectionMatchSet</a> object that you want to include in a <code>Rule</code>.</p>
    #[serde(rename = "Predicates")]
    pub predicates: Vec<Predicate>,
    /// <p>A unique identifier for a <code>Rule</code>. You use <code>RuleId</code> to get more information about a <code>Rule</code> (see <a>GetRule</a>), update a <code>Rule</code> (see <a>UpdateRule</a>), insert a <code>Rule</code> into a <code>WebACL</code> or delete a one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>Rule</code> from AWS WAF (see <a>DeleteRule</a>).</p> <p> <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

/// <p><p>A collection of predefined rules that you can add to a web ACL.</p> <p>Rule groups are subject to the following limits:</p> <ul> <li> <p>Three rule groups per account. You can request an increase to this limit by contacting customer support.</p> </li> <li> <p>One rule group per web ACL.</p> </li> <li> <p>Ten rules per rule group.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RuleGroup {
    /// <p>A friendly name or description for the metrics for this <code>RuleGroup</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace. You can't change the name of the metric after you create the <code>RuleGroup</code>.</p>
    #[serde(rename = "MetricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>The friendly name or description for the <code>RuleGroup</code>. You can't change the name of a <code>RuleGroup</code> after you create it.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for a <code>RuleGroup</code>. You use <code>RuleGroupId</code> to get more information about a <code>RuleGroup</code> (see <a>GetRuleGroup</a>), update a <code>RuleGroup</code> (see <a>UpdateRuleGroup</a>), insert a <code>RuleGroup</code> into a <code>WebACL</code> or delete a one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>RuleGroup</code> from AWS WAF (see <a>DeleteRuleGroup</a>).</p> <p> <code>RuleGroupId</code> is returned by <a>CreateRuleGroup</a> and by <a>ListRuleGroups</a>.</p>
    #[serde(rename = "RuleGroupId")]
    pub rule_group_id: String,
}

/// <p>Contains the identifier and the friendly name or description of the <code>RuleGroup</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RuleGroupSummary {
    /// <p>A friendly name or description of the <a>RuleGroup</a>. You can't change the name of a <code>RuleGroup</code> after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A unique identifier for a <code>RuleGroup</code>. You use <code>RuleGroupId</code> to get more information about a <code>RuleGroup</code> (see <a>GetRuleGroup</a>), update a <code>RuleGroup</code> (see <a>UpdateRuleGroup</a>), insert a <code>RuleGroup</code> into a <code>WebACL</code> or delete one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>RuleGroup</code> from AWS WAF (see <a>DeleteRuleGroup</a>).</p> <p> <code>RuleGroupId</code> is returned by <a>CreateRuleGroup</a> and by <a>ListRuleGroups</a>.</p>
    #[serde(rename = "RuleGroupId")]
    pub rule_group_id: String,
}

/// <p>Specifies an <code>ActivatedRule</code> and indicates whether you want to add it to a <code>RuleGroup</code> or delete it from a <code>RuleGroup</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RuleGroupUpdate {
    /// <p>Specify <code>INSERT</code> to add an <code>ActivatedRule</code> to a <code>RuleGroup</code>. Use <code>DELETE</code> to remove an <code>ActivatedRule</code> from a <code>RuleGroup</code>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>The <code>ActivatedRule</code> object specifies a <code>Rule</code> that you want to insert or delete, the priority of the <code>Rule</code> in the <code>WebACL</code>, and the action that you want AWS WAF to take when a web request matches the <code>Rule</code> (<code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>).</p>
    #[serde(rename = "ActivatedRule")]
    pub activated_rule: ActivatedRule,
}

/// <p>Contains the identifier and the friendly name or description of the <code>Rule</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RuleSummary {
    /// <p>A friendly name or description of the <a>Rule</a>. You can't change the name of a <code>Rule</code> after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A unique identifier for a <code>Rule</code>. You use <code>RuleId</code> to get more information about a <code>Rule</code> (see <a>GetRule</a>), update a <code>Rule</code> (see <a>UpdateRule</a>), insert a <code>Rule</code> into a <code>WebACL</code> or delete one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>Rule</code> from AWS WAF (see <a>DeleteRule</a>).</p> <p> <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

/// <p>Specifies a <code>Predicate</code> (such as an <code>IPSet</code>) and indicates whether you want to add it to a <code>Rule</code> or delete it from a <code>Rule</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RuleUpdate {
    /// <p>Specify <code>INSERT</code> to add a <code>Predicate</code> to a <code>Rule</code>. Use <code>DELETE</code> to remove a <code>Predicate</code> from a <code>Rule</code>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>The ID of the <code>Predicate</code> (such as an <code>IPSet</code>) that you want to add to a <code>Rule</code>.</p>
    #[serde(rename = "Predicate")]
    pub predicate: Predicate,
}

/// <p>The response from a <a>GetSampledRequests</a> request includes a <code>SampledHTTPRequests</code> complex type that appears as <code>SampledRequests</code> in the response syntax. <code>SampledHTTPRequests</code> contains one <code>SampledHTTPRequest</code> object for each web request that is returned by <code>GetSampledRequests</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SampledHTTPRequest {
    /// <p>The action for the <code>Rule</code> that the request matched: <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>A complex type that contains detailed information about the request.</p>
    #[serde(rename = "Request")]
    pub request: HTTPRequest,
    /// <p>This value is returned if the <code>GetSampledRequests</code> request specifies the ID of a <code>RuleGroup</code> rather than the ID of an individual rule. <code>RuleWithinRuleGroup</code> is the rule within the specified <code>RuleGroup</code> that matched the request listed in the response.</p>
    #[serde(rename = "RuleWithinRuleGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_within_rule_group: Option<String>,
    /// <p>The time at which AWS WAF received the request from your AWS resource, in Unix time format (in seconds).</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    /// <p>A value that indicates how one result in the response relates proportionally to other results in the response. A result that has a weight of <code>2</code> represents roughly twice as many CloudFront web requests as a result that has a weight of <code>1</code>.</p>
    #[serde(rename = "Weight")]
    pub weight: i64,
}

/// <p>Specifies a constraint on the size of a part of the web request. AWS WAF uses the <code>Size</code>, <code>ComparisonOperator</code>, and <code>FieldToMatch</code> to build an expression in the form of "<code>Size</code> <code>ComparisonOperator</code> size in bytes of <code>FieldToMatch</code>". If that expression is true, the <code>SizeConstraint</code> is considered to match.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeConstraint {
    /// <p>The type of comparison you want AWS WAF to perform. AWS WAF uses this in combination with the provided <code>Size</code> and <code>FieldToMatch</code> to build an expression in the form of "<code>Size</code> <code>ComparisonOperator</code> size in bytes of <code>FieldToMatch</code>". If that expression is true, the <code>SizeConstraint</code> is considered to match.</p> <p> <b>EQ</b>: Used to test if the <code>Size</code> is equal to the size of the <code>FieldToMatch</code> </p> <p> <b>NE</b>: Used to test if the <code>Size</code> is not equal to the size of the <code>FieldToMatch</code> </p> <p> <b>LE</b>: Used to test if the <code>Size</code> is less than or equal to the size of the <code>FieldToMatch</code> </p> <p> <b>LT</b>: Used to test if the <code>Size</code> is strictly less than the size of the <code>FieldToMatch</code> </p> <p> <b>GE</b>: Used to test if the <code>Size</code> is greater than or equal to the size of the <code>FieldToMatch</code> </p> <p> <b>GT</b>: Used to test if the <code>Size</code> is strictly greater than the size of the <code>FieldToMatch</code> </p>
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    /// <p>Specifies where in a web request to look for the size constraint.</p>
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>The size in bytes that you want AWS WAF to compare against the size of the specified <code>FieldToMatch</code>. AWS WAF uses this in combination with <code>ComparisonOperator</code> and <code>FieldToMatch</code> to build an expression in the form of "<code>Size</code> <code>ComparisonOperator</code> size in bytes of <code>FieldToMatch</code>". If that expression is true, the <code>SizeConstraint</code> is considered to match.</p> <p>Valid values for size are 0 - 21474836480 bytes (0 - 20 GB).</p> <p>If you specify <code>URI</code> for the value of <code>Type</code>, the / in the URI counts as one character. For example, the URI <code>/logo.jpg</code> is nine characters long.</p>
    #[serde(rename = "Size")]
    pub size: i64,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>FieldToMatch</code> before inspecting a request for a match.</p> <p>You can only specify a single type of TextTransformation.</p> <p>Note that if you choose <code>BODY</code> for the value of <code>Type</code>, you must choose <code>NONE</code> for <code>TextTransformation</code> because CloudFront forwards only the first 8192 bytes for inspection. </p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system command line command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \ " ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\f, formfeed, decimal 12</p> </li> <li> <p>\t, tab, decimal 9</p> </li> <li> <p>\n, newline, decimal 10</p> </li> <li> <p>\r, carriage return, decimal 13</p> </li> <li> <p>\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a "less than" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p>
    #[serde(rename = "TextTransformation")]
    pub text_transformation: String,
}

/// <p>A complex type that contains <code>SizeConstraint</code> objects, which specify the parts of web requests that you want AWS WAF to inspect the size of. If a <code>SizeConstraintSet</code> contains more than one <code>SizeConstraint</code> object, a request only needs to match one constraint to be considered a match.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SizeConstraintSet {
    /// <p>The name, if any, of the <code>SizeConstraintSet</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for a <code>SizeConstraintSet</code>. You use <code>SizeConstraintSetId</code> to get information about a <code>SizeConstraintSet</code> (see <a>GetSizeConstraintSet</a>), update a <code>SizeConstraintSet</code> (see <a>UpdateSizeConstraintSet</a>), insert a <code>SizeConstraintSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>SizeConstraintSet</code> from AWS WAF (see <a>DeleteSizeConstraintSet</a>).</p> <p> <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>
    #[serde(rename = "SizeConstraintSetId")]
    pub size_constraint_set_id: String,
    /// <p>Specifies the parts of web requests that you want to inspect the size of.</p>
    #[serde(rename = "SizeConstraints")]
    pub size_constraints: Vec<SizeConstraint>,
}

/// <p>The <code>Id</code> and <code>Name</code> of a <code>SizeConstraintSet</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SizeConstraintSetSummary {
    /// <p>The name of the <code>SizeConstraintSet</code>, if any.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A unique identifier for a <code>SizeConstraintSet</code>. You use <code>SizeConstraintSetId</code> to get information about a <code>SizeConstraintSet</code> (see <a>GetSizeConstraintSet</a>), update a <code>SizeConstraintSet</code> (see <a>UpdateSizeConstraintSet</a>), insert a <code>SizeConstraintSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>SizeConstraintSet</code> from AWS WAF (see <a>DeleteSizeConstraintSet</a>).</p> <p> <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>
    #[serde(rename = "SizeConstraintSetId")]
    pub size_constraint_set_id: String,
}

/// <p>Specifies the part of a web request that you want to inspect the size of and indicates whether you want to add the specification to a <a>SizeConstraintSet</a> or delete it from a <code>SizeConstraintSet</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SizeConstraintSetUpdate {
    /// <p>Specify <code>INSERT</code> to add a <a>SizeConstraintSetUpdate</a> to a <a>SizeConstraintSet</a>. Use <code>DELETE</code> to remove a <code>SizeConstraintSetUpdate</code> from a <code>SizeConstraintSet</code>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>Specifies a constraint on the size of a part of the web request. AWS WAF uses the <code>Size</code>, <code>ComparisonOperator</code>, and <code>FieldToMatch</code> to build an expression in the form of "<code>Size</code> <code>ComparisonOperator</code> size in bytes of <code>FieldToMatch</code>". If that expression is true, the <code>SizeConstraint</code> is considered to match.</p>
    #[serde(rename = "SizeConstraint")]
    pub size_constraint: SizeConstraint,
}

/// <p>A complex type that contains <code>SqlInjectionMatchTuple</code> objects, which specify the parts of web requests that you want AWS WAF to inspect for snippets of malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header. If a <code>SqlInjectionMatchSet</code> contains more than one <code>SqlInjectionMatchTuple</code> object, a request needs to include snippets of SQL code in only one of the specified parts of the request to be considered a match.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SqlInjectionMatchSet {
    /// <p>The name, if any, of the <code>SqlInjectionMatchSet</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for a <code>SqlInjectionMatchSet</code>. You use <code>SqlInjectionMatchSetId</code> to get information about a <code>SqlInjectionMatchSet</code> (see <a>GetSqlInjectionMatchSet</a>), update a <code>SqlInjectionMatchSet</code> (see <a>UpdateSqlInjectionMatchSet</a>), insert a <code>SqlInjectionMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>SqlInjectionMatchSet</code> from AWS WAF (see <a>DeleteSqlInjectionMatchSet</a>).</p> <p> <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>
    #[serde(rename = "SqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: String,
    /// <p>Specifies the parts of web requests that you want to inspect for snippets of malicious SQL code.</p>
    #[serde(rename = "SqlInjectionMatchTuples")]
    pub sql_injection_match_tuples: Vec<SqlInjectionMatchTuple>,
}

/// <p>The <code>Id</code> and <code>Name</code> of a <code>SqlInjectionMatchSet</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SqlInjectionMatchSetSummary {
    /// <p>The name of the <code>SqlInjectionMatchSet</code>, if any, specified by <code>Id</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A unique identifier for a <code>SqlInjectionMatchSet</code>. You use <code>SqlInjectionMatchSetId</code> to get information about a <code>SqlInjectionMatchSet</code> (see <a>GetSqlInjectionMatchSet</a>), update a <code>SqlInjectionMatchSet</code> (see <a>UpdateSqlInjectionMatchSet</a>), insert a <code>SqlInjectionMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>SqlInjectionMatchSet</code> from AWS WAF (see <a>DeleteSqlInjectionMatchSet</a>).</p> <p> <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>
    #[serde(rename = "SqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: String,
}

/// <p>Specifies the part of a web request that you want to inspect for snippets of malicious SQL code and indicates whether you want to add the specification to a <a>SqlInjectionMatchSet</a> or delete it from a <code>SqlInjectionMatchSet</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SqlInjectionMatchSetUpdate {
    /// <p>Specify <code>INSERT</code> to add a <a>SqlInjectionMatchSetUpdate</a> to a <a>SqlInjectionMatchSet</a>. Use <code>DELETE</code> to remove a <code>SqlInjectionMatchSetUpdate</code> from a <code>SqlInjectionMatchSet</code>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>Specifies the part of a web request that you want AWS WAF to inspect for snippets of malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header.</p>
    #[serde(rename = "SqlInjectionMatchTuple")]
    pub sql_injection_match_tuple: SqlInjectionMatchTuple,
}

/// <p>Specifies the part of a web request that you want AWS WAF to inspect for snippets of malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SqlInjectionMatchTuple {
    /// <p>Specifies where in a web request to look for snippets of malicious SQL code.</p>
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>FieldToMatch</code> before inspecting a request for a match.</p> <p>You can only specify a single type of TextTransformation.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system command line command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \ " ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\f, formfeed, decimal 12</p> </li> <li> <p>\t, tab, decimal 9</p> </li> <li> <p>\n, newline, decimal 10</p> </li> <li> <p>\r, carriage return, decimal 13</p> </li> <li> <p>\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a "less than" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p>
    #[serde(rename = "TextTransformation")]
    pub text_transformation: String,
}

/// <p>A summary of the rule groups you are subscribed to.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SubscribedRuleGroupSummary {
    /// <p>A friendly name or description for the metrics for this <code>RuleGroup</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace. You can't change the name of the metric after you create the <code>RuleGroup</code>.</p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// <p>A friendly name or description of the <code>RuleGroup</code>. You can't change the name of a <code>RuleGroup</code> after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A unique identifier for a <code>RuleGroup</code>.</p>
    #[serde(rename = "RuleGroupId")]
    pub rule_group_id: String,
}

/// <p>In a <a>GetSampledRequests</a> request, the <code>StartTime</code> and <code>EndTime</code> objects specify the time range for which you want AWS WAF to return a sample of web requests.</p> <p>In a <a>GetSampledRequests</a> response, the <code>StartTime</code> and <code>EndTime</code> objects specify the time range for which AWS WAF actually returned a sample of web requests. AWS WAF gets the specified number of requests from among the first 5,000 requests that your AWS resource receives during the specified time period. If your resource receives more than 5,000 requests during that period, AWS WAF stops sampling after the 5,000th request. In that case, <code>EndTime</code> is the time that AWS WAF received the 5,000th request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeWindow {
    /// <p>The end of the time range from which you want <code>GetSampledRequests</code> to return a sample of the requests that your AWS resource received. Specify the date and time in the following format: <code>"2016-09-27T14:50Z"</code>. You can specify any time range in the previous three hours.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p>The beginning of the time range from which you want <code>GetSampledRequests</code> to return a sample of the requests that your AWS resource received. Specify the date and time in the following format: <code>"2016-09-27T14:50Z"</code>. You can specify any time range in the previous three hours.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateByteMatchSetRequest {
    /// <p>The <code>ByteMatchSetId</code> of the <a>ByteMatchSet</a> that you want to update. <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>
    #[serde(rename = "ByteMatchSetId")]
    pub byte_match_set_id: String,
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p><p>An array of <code>ByteMatchSetUpdate</code> objects that you want to insert into or delete from a <a>ByteMatchSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>ByteMatchSetUpdate</a>: Contains <code>Action</code> and <code>ByteMatchTuple</code> </p> </li> <li> <p> <a>ByteMatchTuple</a>: Contains <code>FieldToMatch</code>, <code>PositionalConstraint</code>, <code>TargetString</code>, and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "Updates")]
    pub updates: Vec<ByteMatchSetUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateByteMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateByteMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGeoMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>GeoMatchSetId</code> of the <a>GeoMatchSet</a> that you want to update. <code>GeoMatchSetId</code> is returned by <a>CreateGeoMatchSet</a> and by <a>ListGeoMatchSets</a>.</p>
    #[serde(rename = "GeoMatchSetId")]
    pub geo_match_set_id: String,
    /// <p><p>An array of <code>GeoMatchSetUpdate</code> objects that you want to insert into or delete from an <a>GeoMatchSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>GeoMatchSetUpdate</a>: Contains <code>Action</code> and <code>GeoMatchConstraint</code> </p> </li> <li> <p> <a>GeoMatchConstraint</a>: Contains <code>Type</code> and <code>Value</code> </p> <p>You can have only one <code>Type</code> and <code>Value</code> per <code>GeoMatchConstraint</code>. To add multiple countries, include multiple <code>GeoMatchSetUpdate</code> objects in your request.</p> </li> </ul></p>
    #[serde(rename = "Updates")]
    pub updates: Vec<GeoMatchSetUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateGeoMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateGeoMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateIPSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>IPSetId</code> of the <a>IPSet</a> that you want to update. <code>IPSetId</code> is returned by <a>CreateIPSet</a> and by <a>ListIPSets</a>.</p>
    #[serde(rename = "IPSetId")]
    pub ip_set_id: String,
    /// <p>An array of <code>IPSetUpdate</code> objects that you want to insert into or delete from an <a>IPSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>IPSetUpdate</a>: Contains <code>Action</code> and <code>IPSetDescriptor</code> </p> </li> <li> <p> <a>IPSetDescriptor</a>: Contains <code>Type</code> and <code>Value</code> </p> </li> </ul> <p>You can insert a maximum of 1000 addresses in a single request.</p>
    #[serde(rename = "Updates")]
    pub updates: Vec<IPSetUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateIPSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateIPSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRateBasedRuleRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The maximum number of requests, which have an identical value in the field specified by the <code>RateKey</code>, allowed in a five-minute period. If the number of requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule are also met, AWS WAF triggers the action that is specified for this rule.</p>
    #[serde(rename = "RateLimit")]
    pub rate_limit: i64,
    /// <p>The <code>RuleId</code> of the <code>RateBasedRule</code> that you want to update. <code>RuleId</code> is returned by <code>CreateRateBasedRule</code> and by <a>ListRateBasedRules</a>.</p>
    #[serde(rename = "RuleId")]
    pub rule_id: String,
    /// <p>An array of <code>RuleUpdate</code> objects that you want to insert into or delete from a <a>RateBasedRule</a>. </p>
    #[serde(rename = "Updates")]
    pub updates: Vec<RuleUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateRateBasedRuleResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRegexMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>RegexMatchSetId</code> of the <a>RegexMatchSet</a> that you want to update. <code>RegexMatchSetId</code> is returned by <a>CreateRegexMatchSet</a> and by <a>ListRegexMatchSets</a>.</p>
    #[serde(rename = "RegexMatchSetId")]
    pub regex_match_set_id: String,
    /// <p>An array of <code>RegexMatchSetUpdate</code> objects that you want to insert into or delete from a <a>RegexMatchSet</a>. For more information, see <a>RegexMatchTuple</a>.</p>
    #[serde(rename = "Updates")]
    pub updates: Vec<RegexMatchSetUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateRegexMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateRegexMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRegexPatternSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>RegexPatternSetId</code> of the <a>RegexPatternSet</a> that you want to update. <code>RegexPatternSetId</code> is returned by <a>CreateRegexPatternSet</a> and by <a>ListRegexPatternSets</a>.</p>
    #[serde(rename = "RegexPatternSetId")]
    pub regex_pattern_set_id: String,
    /// <p>An array of <code>RegexPatternSetUpdate</code> objects that you want to insert into or delete from a <a>RegexPatternSet</a>.</p>
    #[serde(rename = "Updates")]
    pub updates: Vec<RegexPatternSetUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateRegexPatternSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateRegexPatternSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRuleGroupRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>RuleGroupId</code> of the <a>RuleGroup</a> that you want to update. <code>RuleGroupId</code> is returned by <a>CreateRuleGroup</a> and by <a>ListRuleGroups</a>.</p>
    #[serde(rename = "RuleGroupId")]
    pub rule_group_id: String,
    /// <p>An array of <code>RuleGroupUpdate</code> objects that you want to insert into or delete from a <a>RuleGroup</a>.</p> <p>You can only insert <code>REGULAR</code> rules into a rule group.</p> <p> <code>ActivatedRule|OverrideAction</code> applies only when updating or adding a <code>RuleGroup</code> to a <code>WebACL</code>. In this case you do not use <code>ActivatedRule|Action</code>. For all other update requests, <code>ActivatedRule|Action</code> is used instead of <code>ActivatedRule|OverrideAction</code>.</p>
    #[serde(rename = "Updates")]
    pub updates: Vec<RuleGroupUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateRuleGroupResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateRuleGroup</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRuleRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>RuleId</code> of the <code>Rule</code> that you want to update. <code>RuleId</code> is returned by <code>CreateRule</code> and by <a>ListRules</a>.</p>
    #[serde(rename = "RuleId")]
    pub rule_id: String,
    /// <p><p>An array of <code>RuleUpdate</code> objects that you want to insert into or delete from a <a>Rule</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>RuleUpdate</a>: Contains <code>Action</code> and <code>Predicate</code> </p> </li> <li> <p> <a>Predicate</a>: Contains <code>DataId</code>, <code>Negated</code>, and <code>Type</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "Updates")]
    pub updates: Vec<RuleUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateRuleResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSizeConstraintSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>SizeConstraintSetId</code> of the <a>SizeConstraintSet</a> that you want to update. <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>
    #[serde(rename = "SizeConstraintSetId")]
    pub size_constraint_set_id: String,
    /// <p><p>An array of <code>SizeConstraintSetUpdate</code> objects that you want to insert into or delete from a <a>SizeConstraintSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>SizeConstraintSetUpdate</a>: Contains <code>Action</code> and <code>SizeConstraint</code> </p> </li> <li> <p> <a>SizeConstraint</a>: Contains <code>FieldToMatch</code>, <code>TextTransformation</code>, <code>ComparisonOperator</code>, and <code>Size</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "Updates")]
    pub updates: Vec<SizeConstraintSetUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateSizeConstraintSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateSizeConstraintSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

/// <p>A request to update a <a>SqlInjectionMatchSet</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSqlInjectionMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>The <code>SqlInjectionMatchSetId</code> of the <code>SqlInjectionMatchSet</code> that you want to update. <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>
    #[serde(rename = "SqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: String,
    /// <p><p>An array of <code>SqlInjectionMatchSetUpdate</code> objects that you want to insert into or delete from a <a>SqlInjectionMatchSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>SqlInjectionMatchSetUpdate</a>: Contains <code>Action</code> and <code>SqlInjectionMatchTuple</code> </p> </li> <li> <p> <a>SqlInjectionMatchTuple</a>: Contains <code>FieldToMatch</code> and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "Updates")]
    pub updates: Vec<SqlInjectionMatchSetUpdate>,
}

/// <p>The response to an <a>UpdateSqlInjectionMatchSets</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateSqlInjectionMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateSqlInjectionMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateWebACLRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p>A default action for the web ACL, either ALLOW or BLOCK. AWS WAF performs the default action if a request doesn't match the criteria in any of the rules in a web ACL.</p>
    #[serde(rename = "DefaultAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<WafAction>,
    /// <p><p>An array of updates to make to the <a>WebACL</a>.</p> <p>An array of <code>WebACLUpdate</code> objects that you want to insert into or delete from a <a>WebACL</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>WebACLUpdate</a>: Contains <code>Action</code> and <code>ActivatedRule</code> </p> </li> <li> <p> <a>ActivatedRule</a>: Contains <code>Action</code>, <code>OverrideAction</code>, <code>Priority</code>, <code>RuleId</code>, and <code>Type</code>. <code>ActivatedRule|OverrideAction</code> applies only when updating or adding a <code>RuleGroup</code> to a <code>WebACL</code>. In this case, you do not use <code>ActivatedRule|Action</code>. For all other update requests, <code>ActivatedRule|Action</code> is used instead of <code>ActivatedRule|OverrideAction</code>. </p> </li> <li> <p> <a>WafAction</a>: Contains <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "Updates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<WebACLUpdate>>,
    /// <p>The <code>WebACLId</code> of the <a>WebACL</a> that you want to update. <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>
    #[serde(rename = "WebACLId")]
    pub web_acl_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateWebACLResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateWebACL</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

/// <p>A request to update an <a>XssMatchSet</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateXssMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "ChangeToken")]
    pub change_token: String,
    /// <p><p>An array of <code>XssMatchSetUpdate</code> objects that you want to insert into or delete from an <a>XssMatchSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>XssMatchSetUpdate</a>: Contains <code>Action</code> and <code>XssMatchTuple</code> </p> </li> <li> <p> <a>XssMatchTuple</a>: Contains <code>FieldToMatch</code> and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "Updates")]
    pub updates: Vec<XssMatchSetUpdate>,
    /// <p>The <code>XssMatchSetId</code> of the <code>XssMatchSet</code> that you want to update. <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>
    #[serde(rename = "XssMatchSetId")]
    pub xss_match_set_id: String,
}

/// <p>The response to an <a>UpdateXssMatchSets</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateXssMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateXssMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "ChangeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

/// <p>For the action that is associated with a rule in a <code>WebACL</code>, specifies the action that you want AWS WAF to perform when a web request matches all of the conditions in a rule. For the default action in a <code>WebACL</code>, specifies the action that you want AWS WAF to take when a web request doesn't match all of the conditions in any of the rules in a <code>WebACL</code>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WafAction {
    /// <p><p>Specifies how you want AWS WAF to respond to requests that match the settings in a <code>Rule</code>. Valid settings include the following:</p> <ul> <li> <p> <code>ALLOW</code>: AWS WAF allows requests</p> </li> <li> <p> <code>BLOCK</code>: AWS WAF blocks requests</p> </li> <li> <p> <code>COUNT</code>: AWS WAF increments a counter of the requests that match all of the conditions in the rule. AWS WAF then continues to inspect the web request based on the remaining rules in the web ACL. You can&#39;t specify <code>COUNT</code> for the default action for a <code>WebACL</code>.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>The action to take if any rule within the <code>RuleGroup</code> matches a request. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WafOverrideAction {
    /// <p> <code>COUNT</code> overrides the action specified by the individual rule within a <code>RuleGroup</code> . If set to <code>NONE</code>, the rule's action will take place.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Contains the <code>Rules</code> that identify the requests that you want to allow, block, or count. In a <code>WebACL</code>, you also specify a default action (<code>ALLOW</code> or <code>BLOCK</code>), and the action for each <code>Rule</code> that you add to a <code>WebACL</code>, for example, block requests from specified IP addresses or block requests from specified referrers. You also associate the <code>WebACL</code> with a CloudFront distribution to identify the requests that you want AWS WAF to filter. If you add more than one <code>Rule</code> to a <code>WebACL</code>, a request needs to match only one of the specifications to be allowed, blocked, or counted. For more information, see <a>UpdateWebACL</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct WebACL {
    /// <p>The action to perform if none of the <code>Rules</code> contained in the <code>WebACL</code> match. The action is specified by the <a>WafAction</a> object.</p>
    #[serde(rename = "DefaultAction")]
    pub default_action: WafAction,
    /// <p>A friendly name or description for the metrics for this <code>WebACL</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace. You can't change <code>MetricName</code> after you create the <code>WebACL</code>.</p>
    #[serde(rename = "MetricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>A friendly name or description of the <code>WebACL</code>. You can't change the name of a <code>WebACL</code> after you create it.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array that contains the action for each <code>Rule</code> in a <code>WebACL</code>, the priority of the <code>Rule</code>, and the ID of the <code>Rule</code>.</p>
    #[serde(rename = "Rules")]
    pub rules: Vec<ActivatedRule>,
    /// <p>Tha Amazon Resource Name (ARN) of the web ACL.</p>
    #[serde(rename = "WebACLArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_arn: Option<String>,
    /// <p>A unique identifier for a <code>WebACL</code>. You use <code>WebACLId</code> to get information about a <code>WebACL</code> (see <a>GetWebACL</a>), update a <code>WebACL</code> (see <a>UpdateWebACL</a>), and delete a <code>WebACL</code> from AWS WAF (see <a>DeleteWebACL</a>).</p> <p> <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>
    #[serde(rename = "WebACLId")]
    pub web_acl_id: String,
}

/// <p>Contains the identifier and the name or description of the <a>WebACL</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct WebACLSummary {
    /// <p>A friendly name or description of the <a>WebACL</a>. You can't change the name of a <code>WebACL</code> after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A unique identifier for a <code>WebACL</code>. You use <code>WebACLId</code> to get information about a <code>WebACL</code> (see <a>GetWebACL</a>), update a <code>WebACL</code> (see <a>UpdateWebACL</a>), and delete a <code>WebACL</code> from AWS WAF (see <a>DeleteWebACL</a>).</p> <p> <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>
    #[serde(rename = "WebACLId")]
    pub web_acl_id: String,
}

/// <p>Specifies whether to insert a <code>Rule</code> into or delete a <code>Rule</code> from a <code>WebACL</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WebACLUpdate {
    /// <p>Specifies whether to insert a <code>Rule</code> into or delete a <code>Rule</code> from a <code>WebACL</code>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>The <code>ActivatedRule</code> object in an <a>UpdateWebACL</a> request specifies a <code>Rule</code> that you want to insert or delete, the priority of the <code>Rule</code> in the <code>WebACL</code>, and the action that you want AWS WAF to take when a web request matches the <code>Rule</code> (<code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>).</p>
    #[serde(rename = "ActivatedRule")]
    pub activated_rule: ActivatedRule,
}

/// <p>A complex type that contains <code>XssMatchTuple</code> objects, which specify the parts of web requests that you want AWS WAF to inspect for cross-site scripting attacks and, if you want AWS WAF to inspect a header, the name of the header. If a <code>XssMatchSet</code> contains more than one <code>XssMatchTuple</code> object, a request needs to include cross-site scripting attacks in only one of the specified parts of the request to be considered a match.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct XssMatchSet {
    /// <p>The name, if any, of the <code>XssMatchSet</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for an <code>XssMatchSet</code>. You use <code>XssMatchSetId</code> to get information about an <code>XssMatchSet</code> (see <a>GetXssMatchSet</a>), update an <code>XssMatchSet</code> (see <a>UpdateXssMatchSet</a>), insert an <code>XssMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete an <code>XssMatchSet</code> from AWS WAF (see <a>DeleteXssMatchSet</a>).</p> <p> <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>
    #[serde(rename = "XssMatchSetId")]
    pub xss_match_set_id: String,
    /// <p>Specifies the parts of web requests that you want to inspect for cross-site scripting attacks.</p>
    #[serde(rename = "XssMatchTuples")]
    pub xss_match_tuples: Vec<XssMatchTuple>,
}

/// <p>The <code>Id</code> and <code>Name</code> of an <code>XssMatchSet</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct XssMatchSetSummary {
    /// <p>The name of the <code>XssMatchSet</code>, if any, specified by <code>Id</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A unique identifier for an <code>XssMatchSet</code>. You use <code>XssMatchSetId</code> to get information about a <code>XssMatchSet</code> (see <a>GetXssMatchSet</a>), update an <code>XssMatchSet</code> (see <a>UpdateXssMatchSet</a>), insert an <code>XssMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete an <code>XssMatchSet</code> from AWS WAF (see <a>DeleteXssMatchSet</a>).</p> <p> <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>
    #[serde(rename = "XssMatchSetId")]
    pub xss_match_set_id: String,
}

/// <p>Specifies the part of a web request that you want to inspect for cross-site scripting attacks and indicates whether you want to add the specification to an <a>XssMatchSet</a> or delete it from an <code>XssMatchSet</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct XssMatchSetUpdate {
    /// <p>Specify <code>INSERT</code> to add an <a>XssMatchSetUpdate</a> to an <a>XssMatchSet</a>. Use <code>DELETE</code> to remove an <code>XssMatchSetUpdate</code> from an <code>XssMatchSet</code>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>Specifies the part of a web request that you want AWS WAF to inspect for cross-site scripting attacks and, if you want AWS WAF to inspect a header, the name of the header.</p>
    #[serde(rename = "XssMatchTuple")]
    pub xss_match_tuple: XssMatchTuple,
}

/// <p>Specifies the part of a web request that you want AWS WAF to inspect for cross-site scripting attacks and, if you want AWS WAF to inspect a header, the name of the header.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XssMatchTuple {
    /// <p>Specifies where in a web request to look for cross-site scripting attacks.</p>
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>FieldToMatch</code> before inspecting a request for a match.</p> <p>You can only specify a single type of TextTransformation.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system command line command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \ " ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\f, formfeed, decimal 12</p> </li> <li> <p>\t, tab, decimal 9</p> </li> <li> <p>\n, newline, decimal 10</p> </li> <li> <p>\r, carriage return, decimal 13</p> </li> <li> <p>\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a "less than" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p>
    #[serde(rename = "TextTransformation")]
    pub text_transformation: String,
}

/// Errors returned by AssociateWebACL
#[derive(Debug, PartialEq)]
pub enum AssociateWebACLError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because the entity referenced is temporarily unavailable. Retry your request.</p>
    WAFUnavailableEntity(String),
}

impl AssociateWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateWebACLError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(AssociateWebACLError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(AssociateWebACLError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(AssociateWebACLError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(AssociateWebACLError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFUnavailableEntityException" => {
                    return RusotoError::Service(AssociateWebACLError::WAFUnavailableEntity(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateByteMatchSet
#[derive(Debug, PartialEq)]
pub enum CreateByteMatchSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateByteMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateByteMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateByteMatchSetError::WAFDisallowedName(
                        String::from(error_message),
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateByteMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(CreateByteMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateByteMatchSetError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateByteMatchSetError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateByteMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateGeoMatchSet
#[derive(Debug, PartialEq)]
pub enum CreateGeoMatchSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateGeoMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGeoMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateGeoMatchSetError::WAFDisallowedName(
                        String::from(error_message),
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateGeoMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(CreateGeoMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateGeoMatchSetError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateGeoMatchSetError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateGeoMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateGeoMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGeoMatchSetError {
    fn description(&self) -> &str {
        match *self {
            CreateGeoMatchSetError::WAFDisallowedName(ref cause) => cause,
            CreateGeoMatchSetError::WAFInternalError(ref cause) => cause,
            CreateGeoMatchSetError::WAFInvalidAccount(ref cause) => cause,
            CreateGeoMatchSetError::WAFInvalidParameter(ref cause) => cause,
            CreateGeoMatchSetError::WAFLimitsExceeded(ref cause) => cause,
            CreateGeoMatchSetError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateIPSet
#[derive(Debug, PartialEq)]
pub enum CreateIPSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIPSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateIPSetError::WAFDisallowedName(String::from(
                        error_message,
                    )))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateIPSetError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(CreateIPSetError::WAFInvalidAccount(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateIPSetError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateIPSetError::WAFLimitsExceeded(String::from(
                        error_message,
                    )))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateIPSetError::WAFStaleData(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateRateBasedRule
#[derive(Debug, PartialEq)]
pub enum CreateRateBasedRuleError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateRateBasedRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRateBasedRuleError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateRateBasedRuleError::WAFDisallowedName(
                        String::from(error_message),
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateRateBasedRuleError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateRateBasedRuleError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateRateBasedRuleError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateRateBasedRuleError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateRegexMatchSet
#[derive(Debug, PartialEq)]
pub enum CreateRegexMatchSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateRegexMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRegexMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateRegexMatchSetError::WAFDisallowedName(
                        String::from(error_message),
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateRegexMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateRegexMatchSetError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateRegexMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateRegexMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRegexMatchSetError {
    fn description(&self) -> &str {
        match *self {
            CreateRegexMatchSetError::WAFDisallowedName(ref cause) => cause,
            CreateRegexMatchSetError::WAFInternalError(ref cause) => cause,
            CreateRegexMatchSetError::WAFLimitsExceeded(ref cause) => cause,
            CreateRegexMatchSetError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRegexPatternSet
#[derive(Debug, PartialEq)]
pub enum CreateRegexPatternSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateRegexPatternSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRegexPatternSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFDisallowedName(
                        String::from(error_message),
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateRegexPatternSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRegexPatternSetError {
    fn description(&self) -> &str {
        match *self {
            CreateRegexPatternSetError::WAFDisallowedName(ref cause) => cause,
            CreateRegexPatternSetError::WAFInternalError(ref cause) => cause,
            CreateRegexPatternSetError::WAFLimitsExceeded(ref cause) => cause,
            CreateRegexPatternSetError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRule
#[derive(Debug, PartialEq)]
pub enum CreateRuleError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRuleError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateRuleError::WAFDisallowedName(String::from(
                        error_message,
                    )))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateRuleError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateRuleError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateRuleError::WAFLimitsExceeded(String::from(
                        error_message,
                    )))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateRuleError::WAFStaleData(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateRuleGroup
#[derive(Debug, PartialEq)]
pub enum CreateRuleGroupError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateRuleGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRuleGroupError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFDisallowedName(
                        String::from(error_message),
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFStaleData(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateRuleGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRuleGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateRuleGroupError::WAFDisallowedName(ref cause) => cause,
            CreateRuleGroupError::WAFInternalError(ref cause) => cause,
            CreateRuleGroupError::WAFLimitsExceeded(ref cause) => cause,
            CreateRuleGroupError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSizeConstraintSet
#[derive(Debug, PartialEq)]
pub enum CreateSizeConstraintSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateSizeConstraintSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSizeConstraintSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateSizeConstraintSetError::WAFDisallowedName(
                        String::from(error_message),
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateSizeConstraintSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(CreateSizeConstraintSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateSizeConstraintSetError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateSizeConstraintSetError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateSizeConstraintSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateSqlInjectionMatchSet
#[derive(Debug, PartialEq)]
pub enum CreateSqlInjectionMatchSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateSqlInjectionMatchSetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateSqlInjectionMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(
                        CreateSqlInjectionMatchSetError::WAFDisallowedName(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateSqlInjectionMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(
                        CreateSqlInjectionMatchSetError::WAFInvalidAccount(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        CreateSqlInjectionMatchSetError::WAFInvalidParameter(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(
                        CreateSqlInjectionMatchSetError::WAFLimitsExceeded(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateSqlInjectionMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateWebACL
#[derive(Debug, PartialEq)]
pub enum CreateWebACLError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWebACLError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateWebACLError::WAFDisallowedName(
                        String::from(error_message),
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateWebACLError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(CreateWebACLError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateWebACLError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateWebACLError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateWebACLError::WAFStaleData(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateXssMatchSet
#[derive(Debug, PartialEq)]
pub enum CreateXssMatchSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateXssMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateXssMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateXssMatchSetError::WAFDisallowedName(
                        String::from(error_message),
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateXssMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(CreateXssMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateXssMatchSetError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateXssMatchSetError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateXssMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteByteMatchSet
#[derive(Debug, PartialEq)]
pub enum DeleteByteMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteByteMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteByteMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteByteMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteByteMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteByteMatchSetError::WAFNonEmptyEntity(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteByteMatchSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteByteMatchSetError::WAFReferencedItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteByteMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteGeoMatchSet
#[derive(Debug, PartialEq)]
pub enum DeleteGeoMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteGeoMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGeoMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteGeoMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteGeoMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteGeoMatchSetError::WAFNonEmptyEntity(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteGeoMatchSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteGeoMatchSetError::WAFReferencedItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteGeoMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteGeoMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGeoMatchSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteGeoMatchSetError::WAFInternalError(ref cause) => cause,
            DeleteGeoMatchSetError::WAFInvalidAccount(ref cause) => cause,
            DeleteGeoMatchSetError::WAFNonEmptyEntity(ref cause) => cause,
            DeleteGeoMatchSetError::WAFNonexistentItem(ref cause) => cause,
            DeleteGeoMatchSetError::WAFReferencedItem(ref cause) => cause,
            DeleteGeoMatchSetError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIPSet
#[derive(Debug, PartialEq)]
pub enum DeleteIPSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIPSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFInvalidAccount(String::from(
                        error_message,
                    )))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFNonEmptyEntity(String::from(
                        error_message,
                    )))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFReferencedItem(String::from(
                        error_message,
                    )))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFStaleData(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteLoggingConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteLoggingConfigurationError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteLoggingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteLoggingConfigurationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteLoggingConfigurationError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        DeleteLoggingConfigurationError::WAFNonexistentItem(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteLoggingConfigurationError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteLoggingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLoggingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteLoggingConfigurationError::WAFInternalError(ref cause) => cause,
            DeleteLoggingConfigurationError::WAFNonexistentItem(ref cause) => cause,
            DeleteLoggingConfigurationError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePermissionPolicy
#[derive(Debug, PartialEq)]
pub enum DeletePermissionPolicyError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeletePermissionPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePermissionPolicyError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeletePermissionPolicyError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeletePermissionPolicyError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeletePermissionPolicyError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeletePermissionPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePermissionPolicyError {
    fn description(&self) -> &str {
        match *self {
            DeletePermissionPolicyError::WAFInternalError(ref cause) => cause,
            DeletePermissionPolicyError::WAFNonexistentItem(ref cause) => cause,
            DeletePermissionPolicyError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRateBasedRule
#[derive(Debug, PartialEq)]
pub enum DeleteRateBasedRuleError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteRateBasedRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRateBasedRuleError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteRateBasedRuleError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteRateBasedRuleError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteRateBasedRuleError::WAFNonEmptyEntity(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteRateBasedRuleError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteRateBasedRuleError::WAFReferencedItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteRateBasedRuleError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteRegexMatchSet
#[derive(Debug, PartialEq)]
pub enum DeleteRegexMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteRegexMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRegexMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteRegexMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteRegexMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteRegexMatchSetError::WAFNonEmptyEntity(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteRegexMatchSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteRegexMatchSetError::WAFReferencedItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteRegexMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRegexMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRegexMatchSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteRegexMatchSetError::WAFInternalError(ref cause) => cause,
            DeleteRegexMatchSetError::WAFInvalidAccount(ref cause) => cause,
            DeleteRegexMatchSetError::WAFNonEmptyEntity(ref cause) => cause,
            DeleteRegexMatchSetError::WAFNonexistentItem(ref cause) => cause,
            DeleteRegexMatchSetError::WAFReferencedItem(ref cause) => cause,
            DeleteRegexMatchSetError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRegexPatternSet
#[derive(Debug, PartialEq)]
pub enum DeleteRegexPatternSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteRegexPatternSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRegexPatternSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFNonEmptyEntity(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFReferencedItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRegexPatternSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRegexPatternSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteRegexPatternSetError::WAFInternalError(ref cause) => cause,
            DeleteRegexPatternSetError::WAFInvalidAccount(ref cause) => cause,
            DeleteRegexPatternSetError::WAFNonEmptyEntity(ref cause) => cause,
            DeleteRegexPatternSetError::WAFNonexistentItem(ref cause) => cause,
            DeleteRegexPatternSetError::WAFReferencedItem(ref cause) => cause,
            DeleteRegexPatternSetError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRule
#[derive(Debug, PartialEq)]
pub enum DeleteRuleError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRuleError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteRuleError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteRuleError::WAFInvalidAccount(String::from(
                        error_message,
                    )))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteRuleError::WAFNonEmptyEntity(String::from(
                        error_message,
                    )))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteRuleError::WAFNonexistentItem(String::from(
                        error_message,
                    )))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteRuleError::WAFReferencedItem(String::from(
                        error_message,
                    )))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteRuleError::WAFStaleData(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteRuleGroup
#[derive(Debug, PartialEq)]
pub enum DeleteRuleGroupError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteRuleGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRuleGroupError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFInvalidOperation(
                        String::from(error_message),
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFNonEmptyEntity(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFReferencedItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFStaleData(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRuleGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRuleGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteRuleGroupError::WAFInternalError(ref cause) => cause,
            DeleteRuleGroupError::WAFInvalidOperation(ref cause) => cause,
            DeleteRuleGroupError::WAFNonEmptyEntity(ref cause) => cause,
            DeleteRuleGroupError::WAFNonexistentItem(ref cause) => cause,
            DeleteRuleGroupError::WAFReferencedItem(ref cause) => cause,
            DeleteRuleGroupError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSizeConstraintSet
#[derive(Debug, PartialEq)]
pub enum DeleteSizeConstraintSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteSizeConstraintSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSizeConstraintSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteSizeConstraintSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteSizeConstraintSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteSizeConstraintSetError::WAFNonEmptyEntity(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteSizeConstraintSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteSizeConstraintSetError::WAFReferencedItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteSizeConstraintSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteSqlInjectionMatchSet
#[derive(Debug, PartialEq)]
pub enum DeleteSqlInjectionMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteSqlInjectionMatchSetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteSqlInjectionMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteSqlInjectionMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(
                        DeleteSqlInjectionMatchSetError::WAFInvalidAccount(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(
                        DeleteSqlInjectionMatchSetError::WAFNonEmptyEntity(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        DeleteSqlInjectionMatchSetError::WAFNonexistentItem(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(
                        DeleteSqlInjectionMatchSetError::WAFReferencedItem(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteSqlInjectionMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteWebACL
#[derive(Debug, PartialEq)]
pub enum DeleteWebACLError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWebACLError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFNonEmptyEntity(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFReferencedItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFStaleData(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteXssMatchSet
#[derive(Debug, PartialEq)]
pub enum DeleteXssMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteXssMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteXssMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteXssMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteXssMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteXssMatchSetError::WAFNonEmptyEntity(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteXssMatchSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteXssMatchSetError::WAFReferencedItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteXssMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DisassociateWebACL
#[derive(Debug, PartialEq)]
pub enum DisassociateWebACLError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl DisassociateWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateWebACLError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DisassociateWebACLError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DisassociateWebACLError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(DisassociateWebACLError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DisassociateWebACLError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetByteMatchSet
#[derive(Debug, PartialEq)]
pub enum GetByteMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetByteMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetByteMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetByteMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetByteMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetByteMatchSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetChangeToken
#[derive(Debug, PartialEq)]
pub enum GetChangeTokenError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
}

impl GetChangeTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetChangeTokenError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetChangeTokenError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetChangeTokenStatus
#[derive(Debug, PartialEq)]
pub enum GetChangeTokenStatusError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetChangeTokenStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetChangeTokenStatusError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetChangeTokenStatusError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetChangeTokenStatusError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetGeoMatchSet
#[derive(Debug, PartialEq)]
pub enum GetGeoMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetGeoMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGeoMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetGeoMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetGeoMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetGeoMatchSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetGeoMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGeoMatchSetError {
    fn description(&self) -> &str {
        match *self {
            GetGeoMatchSetError::WAFInternalError(ref cause) => cause,
            GetGeoMatchSetError::WAFInvalidAccount(ref cause) => cause,
            GetGeoMatchSetError::WAFNonexistentItem(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIPSet
#[derive(Debug, PartialEq)]
pub enum GetIPSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIPSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetIPSetError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetIPSetError::WAFInvalidAccount(String::from(
                        error_message,
                    )))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetIPSetError::WAFNonexistentItem(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetLoggingConfiguration
#[derive(Debug, PartialEq)]
pub enum GetLoggingConfigurationError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetLoggingConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLoggingConfigurationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetLoggingConfigurationError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetLoggingConfigurationError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetLoggingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoggingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetLoggingConfigurationError::WAFInternalError(ref cause) => cause,
            GetLoggingConfigurationError::WAFNonexistentItem(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPermissionPolicy
#[derive(Debug, PartialEq)]
pub enum GetPermissionPolicyError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetPermissionPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPermissionPolicyError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetPermissionPolicyError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetPermissionPolicyError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPermissionPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPermissionPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetPermissionPolicyError::WAFInternalError(ref cause) => cause,
            GetPermissionPolicyError::WAFNonexistentItem(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRateBasedRule
#[derive(Debug, PartialEq)]
pub enum GetRateBasedRuleError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetRateBasedRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRateBasedRuleError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetRateBasedRuleError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetRateBasedRuleError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetRateBasedRuleError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetRateBasedRuleManagedKeys
#[derive(Debug, PartialEq)]
pub enum GetRateBasedRuleManagedKeysError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetRateBasedRuleManagedKeysError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRateBasedRuleManagedKeysError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(
                        GetRateBasedRuleManagedKeysError::WAFInternalError(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(
                        GetRateBasedRuleManagedKeysError::WAFInvalidAccount(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        GetRateBasedRuleManagedKeysError::WAFInvalidParameter(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        GetRateBasedRuleManagedKeysError::WAFNonexistentItem(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetRegexMatchSet
#[derive(Debug, PartialEq)]
pub enum GetRegexMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetRegexMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRegexMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetRegexMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetRegexMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetRegexMatchSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRegexMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRegexMatchSetError {
    fn description(&self) -> &str {
        match *self {
            GetRegexMatchSetError::WAFInternalError(ref cause) => cause,
            GetRegexMatchSetError::WAFInvalidAccount(ref cause) => cause,
            GetRegexMatchSetError::WAFNonexistentItem(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRegexPatternSet
#[derive(Debug, PartialEq)]
pub enum GetRegexPatternSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetRegexPatternSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRegexPatternSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetRegexPatternSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetRegexPatternSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetRegexPatternSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRegexPatternSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRegexPatternSetError {
    fn description(&self) -> &str {
        match *self {
            GetRegexPatternSetError::WAFInternalError(ref cause) => cause,
            GetRegexPatternSetError::WAFInvalidAccount(ref cause) => cause,
            GetRegexPatternSetError::WAFNonexistentItem(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRule
#[derive(Debug, PartialEq)]
pub enum GetRuleError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRuleError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetRuleError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetRuleError::WAFInvalidAccount(String::from(
                        error_message,
                    )))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetRuleError::WAFNonexistentItem(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetRuleGroup
#[derive(Debug, PartialEq)]
pub enum GetRuleGroupError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetRuleGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRuleGroupError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetRuleGroupError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetRuleGroupError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRuleGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRuleGroupError {
    fn description(&self) -> &str {
        match *self {
            GetRuleGroupError::WAFInternalError(ref cause) => cause,
            GetRuleGroupError::WAFNonexistentItem(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSampledRequests
#[derive(Debug, PartialEq)]
pub enum GetSampledRequestsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetSampledRequestsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSampledRequestsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetSampledRequestsError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetSampledRequestsError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetSizeConstraintSet
#[derive(Debug, PartialEq)]
pub enum GetSizeConstraintSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetSizeConstraintSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSizeConstraintSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetSizeConstraintSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetSizeConstraintSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetSizeConstraintSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetSqlInjectionMatchSet
#[derive(Debug, PartialEq)]
pub enum GetSqlInjectionMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetSqlInjectionMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSqlInjectionMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetSqlInjectionMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetSqlInjectionMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetSqlInjectionMatchSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetWebACL
#[derive(Debug, PartialEq)]
pub enum GetWebACLError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetWebACLError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetWebACLError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetWebACLError::WAFInvalidAccount(String::from(
                        error_message,
                    )))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetWebACLError::WAFNonexistentItem(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetWebACLForResource
#[derive(Debug, PartialEq)]
pub enum GetWebACLForResourceError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because the entity referenced is temporarily unavailable. Retry your request.</p>
    WAFUnavailableEntity(String),
}

impl GetWebACLForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetWebACLForResourceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetWebACLForResourceError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetWebACLForResourceError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(GetWebACLForResourceError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetWebACLForResourceError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFUnavailableEntityException" => {
                    return RusotoError::Service(GetWebACLForResourceError::WAFUnavailableEntity(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetXssMatchSet
#[derive(Debug, PartialEq)]
pub enum GetXssMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetXssMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetXssMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetXssMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetXssMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetXssMatchSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListActivatedRulesInRuleGroup
#[derive(Debug, PartialEq)]
pub enum ListActivatedRulesInRuleGroupError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl ListActivatedRulesInRuleGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListActivatedRulesInRuleGroupError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(
                        ListActivatedRulesInRuleGroupError::WAFInternalError(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        ListActivatedRulesInRuleGroupError::WAFInvalidParameter(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        ListActivatedRulesInRuleGroupError::WAFNonexistentItem(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListActivatedRulesInRuleGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListActivatedRulesInRuleGroupError {
    fn description(&self) -> &str {
        match *self {
            ListActivatedRulesInRuleGroupError::WAFInternalError(ref cause) => cause,
            ListActivatedRulesInRuleGroupError::WAFInvalidParameter(ref cause) => cause,
            ListActivatedRulesInRuleGroupError::WAFNonexistentItem(ref cause) => cause,
        }
    }
}
/// Errors returned by ListByteMatchSets
#[derive(Debug, PartialEq)]
pub enum ListByteMatchSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListByteMatchSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListByteMatchSetsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListByteMatchSetsError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListByteMatchSetsError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListGeoMatchSets
#[derive(Debug, PartialEq)]
pub enum ListGeoMatchSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListGeoMatchSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGeoMatchSetsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListGeoMatchSetsError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListGeoMatchSetsError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListGeoMatchSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGeoMatchSetsError {
    fn description(&self) -> &str {
        match *self {
            ListGeoMatchSetsError::WAFInternalError(ref cause) => cause,
            ListGeoMatchSetsError::WAFInvalidAccount(ref cause) => cause,
        }
    }
}
/// Errors returned by ListIPSets
#[derive(Debug, PartialEq)]
pub enum ListIPSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListIPSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIPSetsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListIPSetsError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListIPSetsError::WAFInvalidAccount(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListLoggingConfigurations
#[derive(Debug, PartialEq)]
pub enum ListLoggingConfigurationsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl ListLoggingConfigurationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLoggingConfigurationsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListLoggingConfigurationsError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        ListLoggingConfigurationsError::WAFInvalidParameter(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        ListLoggingConfigurationsError::WAFNonexistentItem(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListLoggingConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLoggingConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListLoggingConfigurationsError::WAFInternalError(ref cause) => cause,
            ListLoggingConfigurationsError::WAFInvalidParameter(ref cause) => cause,
            ListLoggingConfigurationsError::WAFNonexistentItem(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRateBasedRules
#[derive(Debug, PartialEq)]
pub enum ListRateBasedRulesError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListRateBasedRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRateBasedRulesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListRateBasedRulesError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListRateBasedRulesError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListRegexMatchSets
#[derive(Debug, PartialEq)]
pub enum ListRegexMatchSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListRegexMatchSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRegexMatchSetsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListRegexMatchSetsError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListRegexMatchSetsError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListRegexMatchSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRegexMatchSetsError {
    fn description(&self) -> &str {
        match *self {
            ListRegexMatchSetsError::WAFInternalError(ref cause) => cause,
            ListRegexMatchSetsError::WAFInvalidAccount(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRegexPatternSets
#[derive(Debug, PartialEq)]
pub enum ListRegexPatternSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListRegexPatternSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRegexPatternSetsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListRegexPatternSetsError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListRegexPatternSetsError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListRegexPatternSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRegexPatternSetsError {
    fn description(&self) -> &str {
        match *self {
            ListRegexPatternSetsError::WAFInternalError(ref cause) => cause,
            ListRegexPatternSetsError::WAFInvalidAccount(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResourcesForWebACL
#[derive(Debug, PartialEq)]
pub enum ListResourcesForWebACLError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl ListResourcesForWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourcesForWebACLError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListResourcesForWebACLError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListResourcesForWebACLError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(ListResourcesForWebACLError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(ListResourcesForWebACLError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            ListResourcesForWebACLError::WAFInvalidParameter(ref cause) => cause,
            ListResourcesForWebACLError::WAFNonexistentItem(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRuleGroups
#[derive(Debug, PartialEq)]
pub enum ListRuleGroupsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
}

impl ListRuleGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRuleGroupsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListRuleGroupsError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListRuleGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRuleGroupsError {
    fn description(&self) -> &str {
        match *self {
            ListRuleGroupsError::WAFInternalError(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRules
#[derive(Debug, PartialEq)]
pub enum ListRulesError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRulesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListRulesError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListRulesError::WAFInvalidAccount(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListSizeConstraintSets
#[derive(Debug, PartialEq)]
pub enum ListSizeConstraintSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListSizeConstraintSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSizeConstraintSetsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListSizeConstraintSetsError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListSizeConstraintSetsError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListSqlInjectionMatchSets
#[derive(Debug, PartialEq)]
pub enum ListSqlInjectionMatchSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListSqlInjectionMatchSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSqlInjectionMatchSetsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListSqlInjectionMatchSetsError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListSqlInjectionMatchSetsError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListSubscribedRuleGroups
#[derive(Debug, PartialEq)]
pub enum ListSubscribedRuleGroupsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl ListSubscribedRuleGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSubscribedRuleGroupsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListSubscribedRuleGroupsError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(ListSubscribedRuleGroupsError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListSubscribedRuleGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSubscribedRuleGroupsError {
    fn description(&self) -> &str {
        match *self {
            ListSubscribedRuleGroupsError::WAFInternalError(ref cause) => cause,
            ListSubscribedRuleGroupsError::WAFNonexistentItem(ref cause) => cause,
        }
    }
}
/// Errors returned by ListWebACLs
#[derive(Debug, PartialEq)]
pub enum ListWebACLsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListWebACLsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWebACLsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListWebACLsError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListWebACLsError::WAFInvalidAccount(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListXssMatchSets
#[derive(Debug, PartialEq)]
pub enum ListXssMatchSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListXssMatchSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListXssMatchSetsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListXssMatchSetsError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListXssMatchSetsError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by PutLoggingConfiguration
#[derive(Debug, PartialEq)]
pub enum PutLoggingConfigurationError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF is not able to access the service linked role. This can be caused by a previous <code>PutLoggingConfiguration</code> request, which can lock the service linked role for about 20 seconds. Please try your request again. The service linked role can also be locked by a previous <code>DeleteServiceLinkedRole</code> request, which can lock the role for 15 minutes or more. If you recently made a <code>DeleteServiceLinkedRole</code>, wait at least 15 minutes and try the request again. If you receive this same exception again, you will have to wait additional time until the role is unlocked.</p>
    WAFServiceLinkedRoleError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl PutLoggingConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutLoggingConfigurationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(PutLoggingConfigurationError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(PutLoggingConfigurationError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFServiceLinkedRoleErrorException" => {
                    return RusotoError::Service(
                        PutLoggingConfigurationError::WAFServiceLinkedRoleError(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(PutLoggingConfigurationError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutLoggingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutLoggingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutLoggingConfigurationError::WAFInternalError(ref cause) => cause,
            PutLoggingConfigurationError::WAFNonexistentItem(ref cause) => cause,
            PutLoggingConfigurationError::WAFServiceLinkedRoleError(ref cause) => cause,
            PutLoggingConfigurationError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by PutPermissionPolicy
#[derive(Debug, PartialEq)]
pub enum PutPermissionPolicyError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because the specified policy is not in the proper format. </p> <p>The policy is subject to the following restrictions:</p> <ul> <li> <p>You can attach only one policy with each <code>PutPermissionPolicy</code> request.</p> </li> <li> <p>The policy must include an <code>Effect</code>, <code>Action</code> and <code>Principal</code>. </p> </li> <li> <p> <code>Effect</code> must specify <code>Allow</code>.</p> </li> <li> <p>The <code>Action</code> in the policy must be <code>waf:UpdateWebACL</code>, <code>waf-regional:UpdateWebACL</code>, <code>waf:GetRuleGroup</code> and <code>waf-regional:GetRuleGroup</code> . Any extra or wildcard actions in the policy will be rejected.</p> </li> <li> <p>The policy cannot include a <code>Resource</code> parameter.</p> </li> <li> <p>The ARN in the request must be a valid WAF RuleGroup ARN and the RuleGroup must exist in the same region.</p> </li> <li> <p>The user making the request must be the owner of the RuleGroup.</p> </li> <li> <p>Your policy must be composed using IAM Policy version 2012-10-17.</p> </li> </ul></p>
    WAFInvalidPermissionPolicy(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl PutPermissionPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutPermissionPolicyError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(PutPermissionPolicyError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidPermissionPolicyException" => {
                    return RusotoError::Service(
                        PutPermissionPolicyError::WAFInvalidPermissionPolicy(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(PutPermissionPolicyError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(PutPermissionPolicyError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutPermissionPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutPermissionPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutPermissionPolicyError::WAFInternalError(ref cause) => cause,
            PutPermissionPolicyError::WAFInvalidPermissionPolicy(ref cause) => cause,
            PutPermissionPolicyError::WAFNonexistentItem(ref cause) => cause,
            PutPermissionPolicyError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateByteMatchSet
#[derive(Debug, PartialEq)]
pub enum UpdateByteMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateByteMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateByteMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFInvalidOperation(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFNonexistentContainer(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateGeoMatchSet
#[derive(Debug, PartialEq)]
pub enum UpdateGeoMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateGeoMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGeoMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFInvalidOperation(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFNonexistentContainer(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFReferencedItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateGeoMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGeoMatchSetError {
    fn description(&self) -> &str {
        match *self {
            UpdateGeoMatchSetError::WAFInternalError(ref cause) => cause,
            UpdateGeoMatchSetError::WAFInvalidAccount(ref cause) => cause,
            UpdateGeoMatchSetError::WAFInvalidOperation(ref cause) => cause,
            UpdateGeoMatchSetError::WAFInvalidParameter(ref cause) => cause,
            UpdateGeoMatchSetError::WAFLimitsExceeded(ref cause) => cause,
            UpdateGeoMatchSetError::WAFNonexistentContainer(ref cause) => cause,
            UpdateGeoMatchSetError::WAFNonexistentItem(ref cause) => cause,
            UpdateGeoMatchSetError::WAFReferencedItem(ref cause) => cause,
            UpdateGeoMatchSetError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateIPSet
#[derive(Debug, PartialEq)]
pub enum UpdateIPSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateIPSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFInvalidAccount(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFInvalidOperation(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFLimitsExceeded(String::from(
                        error_message,
                    )))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFNonexistentContainer(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFReferencedItem(String::from(
                        error_message,
                    )))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFStaleData(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateRateBasedRule
#[derive(Debug, PartialEq)]
pub enum UpdateRateBasedRuleError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateRateBasedRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRateBasedRuleError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFInvalidOperation(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFNonexistentContainer(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFReferencedItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateRegexMatchSet
#[derive(Debug, PartialEq)]
pub enum UpdateRegexMatchSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateRegexMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRegexMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFDisallowedName(
                        String::from(error_message),
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFInvalidOperation(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFNonexistentContainer(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateRegexMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRegexMatchSetError {
    fn description(&self) -> &str {
        match *self {
            UpdateRegexMatchSetError::WAFDisallowedName(ref cause) => cause,
            UpdateRegexMatchSetError::WAFInternalError(ref cause) => cause,
            UpdateRegexMatchSetError::WAFInvalidAccount(ref cause) => cause,
            UpdateRegexMatchSetError::WAFInvalidOperation(ref cause) => cause,
            UpdateRegexMatchSetError::WAFLimitsExceeded(ref cause) => cause,
            UpdateRegexMatchSetError::WAFNonexistentContainer(ref cause) => cause,
            UpdateRegexMatchSetError::WAFNonexistentItem(ref cause) => cause,
            UpdateRegexMatchSetError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRegexPatternSet
#[derive(Debug, PartialEq)]
pub enum UpdateRegexPatternSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p>The regular expression (regex) you specified in <code>RegexPatternString</code> is invalid.</p>
    WAFInvalidRegexPattern(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateRegexPatternSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRegexPatternSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFInvalidOperation(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidRegexPatternException" => {
                    return RusotoError::Service(
                        UpdateRegexPatternSetError::WAFInvalidRegexPattern(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(
                        UpdateRegexPatternSetError::WAFNonexistentContainer(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateRegexPatternSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRegexPatternSetError {
    fn description(&self) -> &str {
        match *self {
            UpdateRegexPatternSetError::WAFInternalError(ref cause) => cause,
            UpdateRegexPatternSetError::WAFInvalidAccount(ref cause) => cause,
            UpdateRegexPatternSetError::WAFInvalidOperation(ref cause) => cause,
            UpdateRegexPatternSetError::WAFInvalidRegexPattern(ref cause) => cause,
            UpdateRegexPatternSetError::WAFLimitsExceeded(ref cause) => cause,
            UpdateRegexPatternSetError::WAFNonexistentContainer(ref cause) => cause,
            UpdateRegexPatternSetError::WAFNonexistentItem(ref cause) => cause,
            UpdateRegexPatternSetError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRule
#[derive(Debug, PartialEq)]
pub enum UpdateRuleError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRuleError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateRuleError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateRuleError::WAFInvalidAccount(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateRuleError::WAFInvalidOperation(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateRuleError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateRuleError::WAFLimitsExceeded(String::from(
                        error_message,
                    )))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateRuleError::WAFNonexistentContainer(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateRuleError::WAFNonexistentItem(String::from(
                        error_message,
                    )))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(UpdateRuleError::WAFReferencedItem(String::from(
                        error_message,
                    )))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateRuleError::WAFStaleData(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateRuleGroup
#[derive(Debug, PartialEq)]
pub enum UpdateRuleGroupError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateRuleGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRuleGroupError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFInvalidOperation(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFNonexistentContainer(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFStaleData(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateRuleGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRuleGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateRuleGroupError::WAFInternalError(ref cause) => cause,
            UpdateRuleGroupError::WAFInvalidOperation(ref cause) => cause,
            UpdateRuleGroupError::WAFInvalidParameter(ref cause) => cause,
            UpdateRuleGroupError::WAFLimitsExceeded(ref cause) => cause,
            UpdateRuleGroupError::WAFNonexistentContainer(ref cause) => cause,
            UpdateRuleGroupError::WAFNonexistentItem(ref cause) => cause,
            UpdateRuleGroupError::WAFStaleData(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSizeConstraintSet
#[derive(Debug, PartialEq)]
pub enum UpdateSizeConstraintSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateSizeConstraintSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSizeConstraintSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFInvalidOperation(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(
                        UpdateSizeConstraintSetError::WAFNonexistentContainer(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFReferencedItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateSqlInjectionMatchSet
#[derive(Debug, PartialEq)]
pub enum UpdateSqlInjectionMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateSqlInjectionMatchSetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateSqlInjectionMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateSqlInjectionMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(
                        UpdateSqlInjectionMatchSetError::WAFInvalidAccount(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(
                        UpdateSqlInjectionMatchSetError::WAFInvalidOperation(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateSqlInjectionMatchSetError::WAFInvalidParameter(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(
                        UpdateSqlInjectionMatchSetError::WAFLimitsExceeded(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(
                        UpdateSqlInjectionMatchSetError::WAFNonexistentContainer(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        UpdateSqlInjectionMatchSetError::WAFNonexistentItem(String::from(
                            error_message,
                        )),
                    )
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateSqlInjectionMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateWebACL
#[derive(Debug, PartialEq)]
pub enum UpdateWebACLError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// <p>The specified subscription does not exist.</p>
    WAFSubscriptionNotFound(String),
}

impl UpdateWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateWebACLError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFInternalError(String::from(
                        error_message,
                    )))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFInvalidOperation(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFNonexistentContainer(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFReferencedItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFStaleData(String::from(
                        error_message,
                    )))
                }
                "WAFSubscriptionNotFoundException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFSubscriptionNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            UpdateWebACLError::WAFSubscriptionNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateXssMatchSet
#[derive(Debug, PartialEq)]
pub enum UpdateXssMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateXssMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateXssMatchSetError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFInternalError(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFInvalidAccount(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFInvalidOperation(
                        String::from(error_message),
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFInvalidParameter(
                        String::from(error_message),
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFLimitsExceeded(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFNonexistentContainer(
                        String::from(error_message),
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFNonexistentItem(
                        String::from(error_message),
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFStaleData(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Trait representing the capabilities of the WAF Regional API. WAF Regional clients implement this trait.
pub trait WAFRegional {
    /// <p>Associates a web ACL with a resource, either an application load balancer or Amazon API Gateway stage.</p>
    fn associate_web_acl(
        &self,
        input: AssociateWebACLRequest,
    ) -> RusotoFuture<AssociateWebACLResponse, AssociateWebACLError>;

    /// <p>Creates a <code>ByteMatchSet</code>. You then use <a>UpdateByteMatchSet</a> to identify the part of a web request that you want AWS WAF to inspect, such as the values of the <code>User-Agent</code> header or the query string. For example, you can create a <code>ByteMatchSet</code> that matches any requests with <code>User-Agent</code> headers that contain the string <code>BadBot</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateByteMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateByteMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateByteMatchSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateByteMatchSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_byte_match_set(
        &self,
        input: CreateByteMatchSetRequest,
    ) -> RusotoFuture<CreateByteMatchSetResponse, CreateByteMatchSetError>;

    /// <p>Creates an <a>GeoMatchSet</a>, which you use to specify which web requests you want to allow or block based on the country that the requests originate from. For example, if you're receiving a lot of requests from one or more countries and you want to block the requests, you can create an <code>GeoMatchSet</code> that contains those countries and then configure AWS WAF to block the requests. </p> <p>To create and configure a <code>GeoMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateGeoMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateGeoMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateGeoMatchSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateGeoMatchSetSet</code> request to specify the countries that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_geo_match_set(
        &self,
        input: CreateGeoMatchSetRequest,
    ) -> RusotoFuture<CreateGeoMatchSetResponse, CreateGeoMatchSetError>;

    /// <p>Creates an <a>IPSet</a>, which you use to specify which web requests that you want to allow or block based on the IP addresses that the requests originate from. For example, if you're receiving a lot of requests from one or more individual IP addresses or one or more ranges of IP addresses and you want to block the requests, you can create an <code>IPSet</code> that contains those IP addresses and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>IPSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateIPSet</code> request.</p> </li> <li> <p>Submit a <code>CreateIPSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateIPSet</code> request to specify the IP addresses that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_ip_set(
        &self,
        input: CreateIPSetRequest,
    ) -> RusotoFuture<CreateIPSetResponse, CreateIPSetError>;

    /// <p>Creates a <a>RateBasedRule</a>. The <code>RateBasedRule</code> contains a <code>RateLimit</code>, which specifies the maximum number of requests that AWS WAF allows from a specified IP address in a five-minute period. The <code>RateBasedRule</code> also contains the <code>IPSet</code> objects, <code>ByteMatchSet</code> objects, and other predicates that identify the requests that you want to count or block if these requests exceed the <code>RateLimit</code>.</p> <p>If you add more than one predicate to a <code>RateBasedRule</code>, a request not only must exceed the <code>RateLimit</code>, but it also must match all the specifications to be counted or blocked. For example, suppose you add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>You then add the <code>RateBasedRule</code> to a <code>WebACL</code> and specify that you want to block requests that meet the conditions in the rule. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>. Further, requests that match these two conditions must be received at a rate of more than 15,000 requests every five minutes. If both conditions are met and the rate is exceeded, AWS WAF blocks the requests. If the rate drops below 15,000 for a five-minute period, AWS WAF no longer blocks the requests.</p> <p>As a second example, suppose you want to limit requests to a particular page on your site. To do this, you could add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>A <code>ByteMatchSet</code> with <code>FieldToMatch</code> of <code>URI</code> </p> </li> <li> <p>A <code>PositionalConstraint</code> of <code>STARTS_WITH</code> </p> </li> <li> <p>A <code>TargetString</code> of <code>login</code> </p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>By adding this <code>RateBasedRule</code> to a <code>WebACL</code>, you could limit requests to your login page without affecting the rest of your site.</p> <p>To create and configure a <code>RateBasedRule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the rule. For more information, see <a>CreateByteMatchSet</a>, <a>CreateIPSet</a>, and <a>CreateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRule</code> request.</p> </li> <li> <p>Submit a <code>CreateRateBasedRule</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRateBasedRule</code> request to specify the predicates that you want to include in the rule.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>RateBasedRule</code>. For more information, see <a>CreateWebACL</a>.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_rate_based_rule(
        &self,
        input: CreateRateBasedRuleRequest,
    ) -> RusotoFuture<CreateRateBasedRuleResponse, CreateRateBasedRuleError>;

    /// <p>Creates a <a>RegexMatchSet</a>. You then use <a>UpdateRegexMatchSet</a> to identify the part of a web request that you want AWS WAF to inspect, such as the values of the <code>User-Agent</code> header or the query string. For example, you can create a <code>RegexMatchSet</code> that contains a <code>RegexMatchTuple</code> that looks for any requests with <code>User-Agent</code> headers that match a <code>RegexPatternSet</code> with pattern <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>RegexMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRegexMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateRegexMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexMatchSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateRegexMatchSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value, using a <code>RegexPatternSet</code>, that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_regex_match_set(
        &self,
        input: CreateRegexMatchSetRequest,
    ) -> RusotoFuture<CreateRegexMatchSetResponse, CreateRegexMatchSetError>;

    /// <p>Creates a <code>RegexPatternSet</code>. You then use <a>UpdateRegexPatternSet</a> to specify the regular expression (regex) pattern that you want AWS WAF to search for, such as <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>RegexPatternSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRegexPatternSet</code> request.</p> </li> <li> <p>Submit a <code>CreateRegexPatternSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexPatternSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateRegexPatternSet</a> request to specify the string that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_regex_pattern_set(
        &self,
        input: CreateRegexPatternSetRequest,
    ) -> RusotoFuture<CreateRegexPatternSetResponse, CreateRegexPatternSetError>;

    /// <p>Creates a <code>Rule</code>, which contains the <code>IPSet</code> objects, <code>ByteMatchSet</code> objects, and other predicates that identify the requests that you want to block. If you add more than one predicate to a <code>Rule</code>, a request must match all of the specifications to be allowed or blocked. For example, suppose that you add the following to a <code>Rule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>You then add the <code>Rule</code> to a <code>WebACL</code> and specify that you want to blocks requests that satisfy the <code>Rule</code>. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>.</p> <p>To create and configure a <code>Rule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the <code>Rule</code>. For more information, see <a>CreateByteMatchSet</a>, <a>CreateIPSet</a>, and <a>CreateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRule</code> request.</p> </li> <li> <p>Submit a <code>CreateRule</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRule</code> request to specify the predicates that you want to include in the <code>Rule</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>Rule</code>. For more information, see <a>CreateWebACL</a>.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_rule(
        &self,
        input: CreateRuleRequest,
    ) -> RusotoFuture<CreateRuleResponse, CreateRuleError>;

    /// <p>Creates a <code>RuleGroup</code>. A rule group is a collection of predefined rules that you add to a web ACL. You use <a>UpdateRuleGroup</a> to add rules to the rule group.</p> <p>Rule groups are subject to the following limits:</p> <ul> <li> <p>Three rule groups per account. You can request an increase to this limit by contacting customer support.</p> </li> <li> <p>One rule group per web ACL.</p> </li> <li> <p>Ten rules per rule group.</p> </li> </ul> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_rule_group(
        &self,
        input: CreateRuleGroupRequest,
    ) -> RusotoFuture<CreateRuleGroupResponse, CreateRuleGroupError>;

    /// <p>Creates a <code>SizeConstraintSet</code>. You then use <a>UpdateSizeConstraintSet</a> to identify the part of a web request that you want AWS WAF to check for length, such as the length of the <code>User-Agent</code> header or the length of the query string. For example, you can create a <code>SizeConstraintSet</code> that matches any requests that have a query string that is longer than 100 bytes. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit a <code>CreateSizeConstraintSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateSizeConstraintSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_size_constraint_set(
        &self,
        input: CreateSizeConstraintSetRequest,
    ) -> RusotoFuture<CreateSizeConstraintSetResponse, CreateSizeConstraintSetError>;

    /// <p>Creates a <a>SqlInjectionMatchSet</a>, which you use to allow, block, or count requests that contain snippets of SQL code in a specified part of web requests. AWS WAF searches for character sequences that are likely to be malicious strings.</p> <p>To create and configure a <code>SqlInjectionMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateSqlInjectionMatchSet</a> request.</p> </li> <li> <p>Submit an <a>UpdateSqlInjectionMatchSet</a> request to specify the parts of web requests in which you want to allow, block, or count malicious SQL code.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_sql_injection_match_set(
        &self,
        input: CreateSqlInjectionMatchSetRequest,
    ) -> RusotoFuture<CreateSqlInjectionMatchSetResponse, CreateSqlInjectionMatchSetError>;

    /// <p>Creates a <code>WebACL</code>, which contains the <code>Rules</code> that identify the CloudFront web requests that you want to allow, block, or count. AWS WAF evaluates <code>Rules</code> in order based on the value of <code>Priority</code> for each <code>Rule</code>.</p> <p>You also specify a default action, either <code>ALLOW</code> or <code>BLOCK</code>. If a web request doesn't match any of the <code>Rules</code> in a <code>WebACL</code>, AWS WAF responds to the request with the default action. </p> <p>To create and configure a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Create and update the <code>ByteMatchSet</code> objects and other predicates that you want to include in <code>Rules</code>. For more information, see <a>CreateByteMatchSet</a>, <a>UpdateByteMatchSet</a>, <a>CreateIPSet</a>, <a>UpdateIPSet</a>, <a>CreateSqlInjectionMatchSet</a>, and <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>WebACL</code>. For more information, see <a>CreateRule</a> and <a>UpdateRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateWebACL</code> request.</p> </li> <li> <p>Submit a <code>CreateWebACL</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateWebACL</a> request.</p> </li> <li> <p>Submit an <a>UpdateWebACL</a> request to specify the <code>Rules</code> that you want to include in the <code>WebACL</code>, to specify the default action, and to associate the <code>WebACL</code> with a CloudFront distribution.</p> </li> </ol> <p>For more information about how to use the AWS WAF API, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_web_acl(
        &self,
        input: CreateWebACLRequest,
    ) -> RusotoFuture<CreateWebACLResponse, CreateWebACLError>;

    /// <p>Creates an <a>XssMatchSet</a>, which you use to allow, block, or count requests that contain cross-site scripting attacks in the specified part of web requests. AWS WAF searches for character sequences that are likely to be malicious strings.</p> <p>To create and configure an <code>XssMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateXssMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateXssMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateXssMatchSet</a> request.</p> </li> <li> <p>Submit an <a>UpdateXssMatchSet</a> request to specify the parts of web requests in which you want to allow, block, or count cross-site scripting attacks.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_xss_match_set(
        &self,
        input: CreateXssMatchSetRequest,
    ) -> RusotoFuture<CreateXssMatchSetResponse, CreateXssMatchSetError>;

    /// <p><p>Permanently deletes a <a>ByteMatchSet</a>. You can&#39;t delete a <code>ByteMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any <a>ByteMatchTuple</a> objects (any filters).</p> <p>If you just want to remove a <code>ByteMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>ByteMatchSet</code> to remove filters, if any. For more information, see <a>UpdateByteMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteByteMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteByteMatchSet</code> request.</p> </li> </ol></p>
    fn delete_byte_match_set(
        &self,
        input: DeleteByteMatchSetRequest,
    ) -> RusotoFuture<DeleteByteMatchSetResponse, DeleteByteMatchSetError>;

    /// <p><p>Permanently deletes a <a>GeoMatchSet</a>. You can&#39;t delete a <code>GeoMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any countries.</p> <p>If you just want to remove a <code>GeoMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>GeoMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>GeoMatchSet</code> to remove any countries. For more information, see <a>UpdateGeoMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteGeoMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteGeoMatchSet</code> request.</p> </li> </ol></p>
    fn delete_geo_match_set(
        &self,
        input: DeleteGeoMatchSetRequest,
    ) -> RusotoFuture<DeleteGeoMatchSetResponse, DeleteGeoMatchSetError>;

    /// <p><p>Permanently deletes an <a>IPSet</a>. You can&#39;t delete an <code>IPSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any IP addresses.</p> <p>If you just want to remove an <code>IPSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete an <code>IPSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>IPSet</code> to remove IP address ranges, if any. For more information, see <a>UpdateIPSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteIPSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteIPSet</code> request.</p> </li> </ol></p>
    fn delete_ip_set(
        &self,
        input: DeleteIPSetRequest,
    ) -> RusotoFuture<DeleteIPSetResponse, DeleteIPSetError>;

    /// <p>Permanently deletes the <a>LoggingConfiguration</a> from the specified web ACL.</p>
    fn delete_logging_configuration(
        &self,
        input: DeleteLoggingConfigurationRequest,
    ) -> RusotoFuture<DeleteLoggingConfigurationResponse, DeleteLoggingConfigurationError>;

    /// <p>Permanently deletes an IAM policy from the specified RuleGroup.</p> <p>The user making the request must be the owner of the RuleGroup.</p>
    fn delete_permission_policy(
        &self,
        input: DeletePermissionPolicyRequest,
    ) -> RusotoFuture<DeletePermissionPolicyResponse, DeletePermissionPolicyError>;

    /// <p><p>Permanently deletes a <a>RateBasedRule</a>. You can&#39;t delete a rule if it&#39;s still used in any <code>WebACL</code> objects or if it still includes any predicates, such as <code>ByteMatchSet</code> objects.</p> <p>If you just want to remove a rule from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>RateBasedRule</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>RateBasedRule</code> to remove predicates, if any. For more information, see <a>UpdateRateBasedRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRateBasedRule</code> request.</p> </li> <li> <p>Submit a <code>DeleteRateBasedRule</code> request.</p> </li> </ol></p>
    fn delete_rate_based_rule(
        &self,
        input: DeleteRateBasedRuleRequest,
    ) -> RusotoFuture<DeleteRateBasedRuleResponse, DeleteRateBasedRuleError>;

    /// <p><p>Permanently deletes a <a>RegexMatchSet</a>. You can&#39;t delete a <code>RegexMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any <code>RegexMatchTuples</code> objects (any filters).</p> <p>If you just want to remove a <code>RegexMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>RegexMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>RegexMatchSet</code> to remove filters, if any. For more information, see <a>UpdateRegexMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRegexMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteRegexMatchSet</code> request.</p> </li> </ol></p>
    fn delete_regex_match_set(
        &self,
        input: DeleteRegexMatchSetRequest,
    ) -> RusotoFuture<DeleteRegexMatchSetResponse, DeleteRegexMatchSetError>;

    /// <p>Permanently deletes a <a>RegexPatternSet</a>. You can't delete a <code>RegexPatternSet</code> if it's still used in any <code>RegexMatchSet</code> or if the <code>RegexPatternSet</code> is not empty. </p>
    fn delete_regex_pattern_set(
        &self,
        input: DeleteRegexPatternSetRequest,
    ) -> RusotoFuture<DeleteRegexPatternSetResponse, DeleteRegexPatternSetError>;

    /// <p><p>Permanently deletes a <a>Rule</a>. You can&#39;t delete a <code>Rule</code> if it&#39;s still used in any <code>WebACL</code> objects or if it still includes any predicates, such as <code>ByteMatchSet</code> objects.</p> <p>If you just want to remove a <code>Rule</code> from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>Rule</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>Rule</code> to remove predicates, if any. For more information, see <a>UpdateRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRule</code> request.</p> </li> <li> <p>Submit a <code>DeleteRule</code> request.</p> </li> </ol></p>
    fn delete_rule(
        &self,
        input: DeleteRuleRequest,
    ) -> RusotoFuture<DeleteRuleResponse, DeleteRuleError>;

    /// <p><p>Permanently deletes a <a>RuleGroup</a>. You can&#39;t delete a <code>RuleGroup</code> if it&#39;s still used in any <code>WebACL</code> objects or if it still includes any rules.</p> <p>If you just want to remove a <code>RuleGroup</code> from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>RuleGroup</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>RuleGroup</code> to remove rules, if any. For more information, see <a>UpdateRuleGroup</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRuleGroup</code> request.</p> </li> <li> <p>Submit a <code>DeleteRuleGroup</code> request.</p> </li> </ol></p>
    fn delete_rule_group(
        &self,
        input: DeleteRuleGroupRequest,
    ) -> RusotoFuture<DeleteRuleGroupResponse, DeleteRuleGroupError>;

    /// <p><p>Permanently deletes a <a>SizeConstraintSet</a>. You can&#39;t delete a <code>SizeConstraintSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any <a>SizeConstraint</a> objects (any filters).</p> <p>If you just want to remove a <code>SizeConstraintSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>SizeConstraintSet</code> to remove filters, if any. For more information, see <a>UpdateSizeConstraintSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteSizeConstraintSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteSizeConstraintSet</code> request.</p> </li> </ol></p>
    fn delete_size_constraint_set(
        &self,
        input: DeleteSizeConstraintSetRequest,
    ) -> RusotoFuture<DeleteSizeConstraintSetResponse, DeleteSizeConstraintSetError>;

    /// <p><p>Permanently deletes a <a>SqlInjectionMatchSet</a>. You can&#39;t delete a <code>SqlInjectionMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still contains any <a>SqlInjectionMatchTuple</a> objects.</p> <p>If you just want to remove a <code>SqlInjectionMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>SqlInjectionMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>SqlInjectionMatchSet</code> to remove filters, if any. For more information, see <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteSqlInjectionMatchSet</code> request.</p> </li> </ol></p>
    fn delete_sql_injection_match_set(
        &self,
        input: DeleteSqlInjectionMatchSetRequest,
    ) -> RusotoFuture<DeleteSqlInjectionMatchSetResponse, DeleteSqlInjectionMatchSetError>;

    /// <p><p>Permanently deletes a <a>WebACL</a>. You can&#39;t delete a <code>WebACL</code> if it still contains any <code>Rules</code>.</p> <p>To delete a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>WebACL</code> to remove <code>Rules</code>, if any. For more information, see <a>UpdateWebACL</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteWebACL</code> request.</p> </li> <li> <p>Submit a <code>DeleteWebACL</code> request.</p> </li> </ol></p>
    fn delete_web_acl(
        &self,
        input: DeleteWebACLRequest,
    ) -> RusotoFuture<DeleteWebACLResponse, DeleteWebACLError>;

    /// <p><p>Permanently deletes an <a>XssMatchSet</a>. You can&#39;t delete an <code>XssMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still contains any <a>XssMatchTuple</a> objects.</p> <p>If you just want to remove an <code>XssMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete an <code>XssMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>XssMatchSet</code> to remove filters, if any. For more information, see <a>UpdateXssMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteXssMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteXssMatchSet</code> request.</p> </li> </ol></p>
    fn delete_xss_match_set(
        &self,
        input: DeleteXssMatchSetRequest,
    ) -> RusotoFuture<DeleteXssMatchSetResponse, DeleteXssMatchSetError>;

    /// <p>Removes a web ACL from the specified resource, either an application load balancer or Amazon API Gateway stage.</p>
    fn disassociate_web_acl(
        &self,
        input: DisassociateWebACLRequest,
    ) -> RusotoFuture<DisassociateWebACLResponse, DisassociateWebACLError>;

    /// <p>Returns the <a>ByteMatchSet</a> specified by <code>ByteMatchSetId</code>.</p>
    fn get_byte_match_set(
        &self,
        input: GetByteMatchSetRequest,
    ) -> RusotoFuture<GetByteMatchSetResponse, GetByteMatchSetError>;

    /// <p>When you want to create, update, or delete AWS WAF objects, get a change token and include the change token in the create, update, or delete request. Change tokens ensure that your application doesn't submit conflicting requests to AWS WAF.</p> <p>Each create, update, or delete request must use a unique change token. If your application submits a <code>GetChangeToken</code> request and then submits a second <code>GetChangeToken</code> request before submitting a create, update, or delete request, the second <code>GetChangeToken</code> request returns the same value as the first <code>GetChangeToken</code> request.</p> <p>When you use a change token in a create, update, or delete request, the status of the change token changes to <code>PENDING</code>, which indicates that AWS WAF is propagating the change to all AWS WAF servers. Use <code>GetChangeTokenStatus</code> to determine the status of your change token.</p>
    fn get_change_token(&self) -> RusotoFuture<GetChangeTokenResponse, GetChangeTokenError>;

    /// <p><p>Returns the status of a <code>ChangeToken</code> that you got by calling <a>GetChangeToken</a>. <code>ChangeTokenStatus</code> is one of the following values:</p> <ul> <li> <p> <code>PROVISIONED</code>: You requested the change token by calling <code>GetChangeToken</code>, but you haven&#39;t used it yet in a call to create, update, or delete an AWS WAF object.</p> </li> <li> <p> <code>PENDING</code>: AWS WAF is propagating the create, update, or delete request to all AWS WAF servers.</p> </li> <li> <p> <code>IN_SYNC</code>: Propagation is complete.</p> </li> </ul></p>
    fn get_change_token_status(
        &self,
        input: GetChangeTokenStatusRequest,
    ) -> RusotoFuture<GetChangeTokenStatusResponse, GetChangeTokenStatusError>;

    /// <p>Returns the <a>GeoMatchSet</a> that is specified by <code>GeoMatchSetId</code>.</p>
    fn get_geo_match_set(
        &self,
        input: GetGeoMatchSetRequest,
    ) -> RusotoFuture<GetGeoMatchSetResponse, GetGeoMatchSetError>;

    /// <p>Returns the <a>IPSet</a> that is specified by <code>IPSetId</code>.</p>
    fn get_ip_set(&self, input: GetIPSetRequest) -> RusotoFuture<GetIPSetResponse, GetIPSetError>;

    /// <p>Returns the <a>LoggingConfiguration</a> for the specified web ACL.</p>
    fn get_logging_configuration(
        &self,
        input: GetLoggingConfigurationRequest,
    ) -> RusotoFuture<GetLoggingConfigurationResponse, GetLoggingConfigurationError>;

    /// <p>Returns the IAM policy attached to the RuleGroup.</p>
    fn get_permission_policy(
        &self,
        input: GetPermissionPolicyRequest,
    ) -> RusotoFuture<GetPermissionPolicyResponse, GetPermissionPolicyError>;

    /// <p>Returns the <a>RateBasedRule</a> that is specified by the <code>RuleId</code> that you included in the <code>GetRateBasedRule</code> request.</p>
    fn get_rate_based_rule(
        &self,
        input: GetRateBasedRuleRequest,
    ) -> RusotoFuture<GetRateBasedRuleResponse, GetRateBasedRuleError>;

    /// <p>Returns an array of IP addresses currently being blocked by the <a>RateBasedRule</a> that is specified by the <code>RuleId</code>. The maximum number of managed keys that will be blocked is 10,000. If more than 10,000 addresses exceed the rate limit, the 10,000 addresses with the highest rates will be blocked.</p>
    fn get_rate_based_rule_managed_keys(
        &self,
        input: GetRateBasedRuleManagedKeysRequest,
    ) -> RusotoFuture<GetRateBasedRuleManagedKeysResponse, GetRateBasedRuleManagedKeysError>;

    /// <p>Returns the <a>RegexMatchSet</a> specified by <code>RegexMatchSetId</code>.</p>
    fn get_regex_match_set(
        &self,
        input: GetRegexMatchSetRequest,
    ) -> RusotoFuture<GetRegexMatchSetResponse, GetRegexMatchSetError>;

    /// <p>Returns the <a>RegexPatternSet</a> specified by <code>RegexPatternSetId</code>.</p>
    fn get_regex_pattern_set(
        &self,
        input: GetRegexPatternSetRequest,
    ) -> RusotoFuture<GetRegexPatternSetResponse, GetRegexPatternSetError>;

    /// <p>Returns the <a>Rule</a> that is specified by the <code>RuleId</code> that you included in the <code>GetRule</code> request.</p>
    fn get_rule(&self, input: GetRuleRequest) -> RusotoFuture<GetRuleResponse, GetRuleError>;

    /// <p>Returns the <a>RuleGroup</a> that is specified by the <code>RuleGroupId</code> that you included in the <code>GetRuleGroup</code> request.</p> <p>To view the rules in a rule group, use <a>ListActivatedRulesInRuleGroup</a>.</p>
    fn get_rule_group(
        &self,
        input: GetRuleGroupRequest,
    ) -> RusotoFuture<GetRuleGroupResponse, GetRuleGroupError>;

    /// <p>Gets detailed information about a specified number of requests--a sample--that AWS WAF randomly selects from among the first 5,000 requests that your AWS resource received during a time range that you choose. You can specify a sample size of up to 500 requests, and you can specify any time range in the previous three hours.</p> <p> <code>GetSampledRequests</code> returns a time range, which is usually the time range that you specified. However, if your resource (such as a CloudFront distribution) received 5,000 requests before the specified time range elapsed, <code>GetSampledRequests</code> returns an updated time range. This new time range indicates the actual period during which AWS WAF selected the requests in the sample.</p>
    fn get_sampled_requests(
        &self,
        input: GetSampledRequestsRequest,
    ) -> RusotoFuture<GetSampledRequestsResponse, GetSampledRequestsError>;

    /// <p>Returns the <a>SizeConstraintSet</a> specified by <code>SizeConstraintSetId</code>.</p>
    fn get_size_constraint_set(
        &self,
        input: GetSizeConstraintSetRequest,
    ) -> RusotoFuture<GetSizeConstraintSetResponse, GetSizeConstraintSetError>;

    /// <p>Returns the <a>SqlInjectionMatchSet</a> that is specified by <code>SqlInjectionMatchSetId</code>.</p>
    fn get_sql_injection_match_set(
        &self,
        input: GetSqlInjectionMatchSetRequest,
    ) -> RusotoFuture<GetSqlInjectionMatchSetResponse, GetSqlInjectionMatchSetError>;

    /// <p>Returns the <a>WebACL</a> that is specified by <code>WebACLId</code>.</p>
    fn get_web_acl(
        &self,
        input: GetWebACLRequest,
    ) -> RusotoFuture<GetWebACLResponse, GetWebACLError>;

    /// <p>Returns the web ACL for the specified resource, either an application load balancer or Amazon API Gateway stage.</p>
    fn get_web_acl_for_resource(
        &self,
        input: GetWebACLForResourceRequest,
    ) -> RusotoFuture<GetWebACLForResourceResponse, GetWebACLForResourceError>;

    /// <p>Returns the <a>XssMatchSet</a> that is specified by <code>XssMatchSetId</code>.</p>
    fn get_xss_match_set(
        &self,
        input: GetXssMatchSetRequest,
    ) -> RusotoFuture<GetXssMatchSetResponse, GetXssMatchSetError>;

    /// <p>Returns an array of <a>ActivatedRule</a> objects.</p>
    fn list_activated_rules_in_rule_group(
        &self,
        input: ListActivatedRulesInRuleGroupRequest,
    ) -> RusotoFuture<ListActivatedRulesInRuleGroupResponse, ListActivatedRulesInRuleGroupError>;

    /// <p>Returns an array of <a>ByteMatchSetSummary</a> objects.</p>
    fn list_byte_match_sets(
        &self,
        input: ListByteMatchSetsRequest,
    ) -> RusotoFuture<ListByteMatchSetsResponse, ListByteMatchSetsError>;

    /// <p>Returns an array of <a>GeoMatchSetSummary</a> objects in the response.</p>
    fn list_geo_match_sets(
        &self,
        input: ListGeoMatchSetsRequest,
    ) -> RusotoFuture<ListGeoMatchSetsResponse, ListGeoMatchSetsError>;

    /// <p>Returns an array of <a>IPSetSummary</a> objects in the response.</p>
    fn list_ip_sets(
        &self,
        input: ListIPSetsRequest,
    ) -> RusotoFuture<ListIPSetsResponse, ListIPSetsError>;

    /// <p>Returns an array of <a>LoggingConfiguration</a> objects.</p>
    fn list_logging_configurations(
        &self,
        input: ListLoggingConfigurationsRequest,
    ) -> RusotoFuture<ListLoggingConfigurationsResponse, ListLoggingConfigurationsError>;

    /// <p>Returns an array of <a>RuleSummary</a> objects.</p>
    fn list_rate_based_rules(
        &self,
        input: ListRateBasedRulesRequest,
    ) -> RusotoFuture<ListRateBasedRulesResponse, ListRateBasedRulesError>;

    /// <p>Returns an array of <a>RegexMatchSetSummary</a> objects.</p>
    fn list_regex_match_sets(
        &self,
        input: ListRegexMatchSetsRequest,
    ) -> RusotoFuture<ListRegexMatchSetsResponse, ListRegexMatchSetsError>;

    /// <p>Returns an array of <a>RegexPatternSetSummary</a> objects.</p>
    fn list_regex_pattern_sets(
        &self,
        input: ListRegexPatternSetsRequest,
    ) -> RusotoFuture<ListRegexPatternSetsResponse, ListRegexPatternSetsError>;

    /// <p>Returns an array of resources associated with the specified web ACL.</p>
    fn list_resources_for_web_acl(
        &self,
        input: ListResourcesForWebACLRequest,
    ) -> RusotoFuture<ListResourcesForWebACLResponse, ListResourcesForWebACLError>;

    /// <p>Returns an array of <a>RuleGroup</a> objects.</p>
    fn list_rule_groups(
        &self,
        input: ListRuleGroupsRequest,
    ) -> RusotoFuture<ListRuleGroupsResponse, ListRuleGroupsError>;

    /// <p>Returns an array of <a>RuleSummary</a> objects.</p>
    fn list_rules(
        &self,
        input: ListRulesRequest,
    ) -> RusotoFuture<ListRulesResponse, ListRulesError>;

    /// <p>Returns an array of <a>SizeConstraintSetSummary</a> objects.</p>
    fn list_size_constraint_sets(
        &self,
        input: ListSizeConstraintSetsRequest,
    ) -> RusotoFuture<ListSizeConstraintSetsResponse, ListSizeConstraintSetsError>;

    /// <p>Returns an array of <a>SqlInjectionMatchSet</a> objects.</p>
    fn list_sql_injection_match_sets(
        &self,
        input: ListSqlInjectionMatchSetsRequest,
    ) -> RusotoFuture<ListSqlInjectionMatchSetsResponse, ListSqlInjectionMatchSetsError>;

    /// <p>Returns an array of <a>RuleGroup</a> objects that you are subscribed to.</p>
    fn list_subscribed_rule_groups(
        &self,
        input: ListSubscribedRuleGroupsRequest,
    ) -> RusotoFuture<ListSubscribedRuleGroupsResponse, ListSubscribedRuleGroupsError>;

    /// <p>Returns an array of <a>WebACLSummary</a> objects in the response.</p>
    fn list_web_ac_ls(
        &self,
        input: ListWebACLsRequest,
    ) -> RusotoFuture<ListWebACLsResponse, ListWebACLsError>;

    /// <p>Returns an array of <a>XssMatchSet</a> objects.</p>
    fn list_xss_match_sets(
        &self,
        input: ListXssMatchSetsRequest,
    ) -> RusotoFuture<ListXssMatchSetsResponse, ListXssMatchSetsError>;

    /// <p>Associates a <a>LoggingConfiguration</a> with a specified web ACL.</p> <p>You can access information about all traffic that AWS WAF inspects using the following steps:</p> <ol> <li> <p>Create an Amazon Kinesis Data Firehose . </p> </li> <li> <p>Associate that firehose to your web ACL using a <code>PutLoggingConfiguration</code> request.</p> </li> </ol> <p>When you successfully enable logging using a <code>PutLoggingConfiguration</code> request, AWS WAF will create a service linked role with the necessary permissions to write logs to the Amazon Kinesis Data Firehose. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/logging.html">Logging Web ACL Traffic Information</a> in the <i>AWS WAF Developer Guide</i>.</p>
    fn put_logging_configuration(
        &self,
        input: PutLoggingConfigurationRequest,
    ) -> RusotoFuture<PutLoggingConfigurationResponse, PutLoggingConfigurationError>;

    /// <p>Attaches a IAM policy to the specified resource. The only supported use for this action is to share a RuleGroup across accounts.</p> <p>The <code>PutPermissionPolicy</code> is subject to the following restrictions:</p> <ul> <li> <p>You can attach only one policy with each <code>PutPermissionPolicy</code> request.</p> </li> <li> <p>The policy must include an <code>Effect</code>, <code>Action</code> and <code>Principal</code>. </p> </li> <li> <p> <code>Effect</code> must specify <code>Allow</code>.</p> </li> <li> <p>The <code>Action</code> in the policy must be <code>waf:UpdateWebACL</code>, <code>waf-regional:UpdateWebACL</code>, <code>waf:GetRuleGroup</code> and <code>waf-regional:GetRuleGroup</code> . Any extra or wildcard actions in the policy will be rejected.</p> </li> <li> <p>The policy cannot include a <code>Resource</code> parameter.</p> </li> <li> <p>The ARN in the request must be a valid WAF RuleGroup ARN and the RuleGroup must exist in the same region.</p> </li> <li> <p>The user making the request must be the owner of the RuleGroup.</p> </li> <li> <p>Your policy must be composed using IAM Policy version 2012-10-17.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html">IAM Policies</a>. </p> <p>An example of a valid policy parameter is shown in the Examples section below.</p>
    fn put_permission_policy(
        &self,
        input: PutPermissionPolicyRequest,
    ) -> RusotoFuture<PutPermissionPolicyResponse, PutPermissionPolicyError>;

    /// <p>Inserts or deletes <a>ByteMatchTuple</a> objects (filters) in a <a>ByteMatchSet</a>. For each <code>ByteMatchTuple</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>ByteMatchSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to inspect, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to look for. For more information, including how you specify the values for the AWS WAF API and the AWS CLI or SDKs, see <code>TargetString</code> in the <a>ByteMatchTuple</a> data type. </p> </li> <li> <p>Where to look, such as at the beginning or the end of a query string.</p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul> <p>For example, you can add a <code>ByteMatchSetUpdate</code> object that matches web requests in which <code>User-Agent</code> headers contain the string <code>BadBot</code>. You can then configure AWS WAF to block those requests.</p> <p>To create and configure a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>ByteMatchSet.</code> For more information, see <a>CreateByteMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateByteMatchSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateByteMatchSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_byte_match_set(
        &self,
        input: UpdateByteMatchSetRequest,
    ) -> RusotoFuture<UpdateByteMatchSetResponse, UpdateByteMatchSetError>;

    /// <p>Inserts or deletes <a>GeoMatchConstraint</a> objects in an <code>GeoMatchSet</code>. For each <code>GeoMatchConstraint</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change an <code>GeoMatchConstraint</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The <code>Type</code>. The only valid value for <code>Type</code> is <code>Country</code>.</p> </li> <li> <p>The <code>Value</code>, which is a two character code for the country to add to the <code>GeoMatchConstraint</code> object. Valid codes are listed in <a>GeoMatchConstraint$Value</a>.</p> </li> </ul> <p>To create and configure an <code>GeoMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateGeoMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateGeoMatchSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateGeoMatchSet</code> request to specify the country that you want AWS WAF to watch for.</p> </li> </ol> <p>When you update an <code>GeoMatchSet</code>, you specify the country that you want to add and/or the country that you want to delete. If you want to change a country, you delete the existing country and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_geo_match_set(
        &self,
        input: UpdateGeoMatchSetRequest,
    ) -> RusotoFuture<UpdateGeoMatchSetResponse, UpdateGeoMatchSetError>;

    /// <p>Inserts or deletes <a>IPSetDescriptor</a> objects in an <code>IPSet</code>. For each <code>IPSetDescriptor</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change an <code>IPSetDescriptor</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The IP address version, <code>IPv4</code> or <code>IPv6</code>. </p> </li> <li> <p>The IP address in CIDR notation, for example, <code>192.0.2.0/24</code> (for the range of IP addresses from <code>192.0.2.0</code> to <code>192.0.2.255</code>) or <code>192.0.2.44/32</code> (for the individual IP address <code>192.0.2.44</code>). </p> </li> </ul> <p>AWS WAF supports IPv4 address ranges: /8 and any range between /16 through /32. AWS WAF supports IPv6 address ranges: /16, /24, /32, /48, /56, /64, and /128. For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p> <p>IPv6 addresses can be represented using any of the following formats:</p> <ul> <li> <p>1111:0000:0000:0000:0000:0000:0000:0111/128</p> </li> <li> <p>1111:0:0:0:0:0:0:0111/128</p> </li> <li> <p>1111::0111/128</p> </li> <li> <p>1111::111/128</p> </li> </ul> <p>You use an <code>IPSet</code> to specify which web requests you want to allow or block based on the IP addresses that the requests originated from. For example, if you're receiving a lot of requests from one or a small number of IP addresses and you want to block the requests, you can create an <code>IPSet</code> that specifies those IP addresses, and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>IPSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateIPSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateIPSet</code> request to specify the IP addresses that you want AWS WAF to watch for.</p> </li> </ol> <p>When you update an <code>IPSet</code>, you specify the IP addresses that you want to add and/or the IP addresses that you want to delete. If you want to change an IP address, you delete the existing IP address and add the new one.</p> <p>You can insert a maximum of 1000 addresses in a single request.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_ip_set(
        &self,
        input: UpdateIPSetRequest,
    ) -> RusotoFuture<UpdateIPSetResponse, UpdateIPSetError>;

    /// <p>Inserts or deletes <a>Predicate</a> objects in a rule and updates the <code>RateLimit</code> in the rule. </p> <p>Each <code>Predicate</code> object identifies a predicate, such as a <a>ByteMatchSet</a> or an <a>IPSet</a>, that specifies the web requests that you want to block or count. The <code>RateLimit</code> specifies the number of requests every five minutes that triggers the rule.</p> <p>If you add more than one predicate to a <code>RateBasedRule</code>, a request must match all the predicates and exceed the <code>RateLimit</code> to be counted or blocked. For example, suppose you add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>You then add the <code>RateBasedRule</code> to a <code>WebACL</code> and specify that you want to block requests that satisfy the rule. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>. Further, requests that match these two conditions much be received at a rate of more than 15,000 every five minutes. If the rate drops below this limit, AWS WAF no longer blocks the requests.</p> <p>As a second example, suppose you want to limit requests to a particular page on your site. To do this, you could add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>A <code>ByteMatchSet</code> with <code>FieldToMatch</code> of <code>URI</code> </p> </li> <li> <p>A <code>PositionalConstraint</code> of <code>STARTS_WITH</code> </p> </li> <li> <p>A <code>TargetString</code> of <code>login</code> </p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>By adding this <code>RateBasedRule</code> to a <code>WebACL</code>, you could limit requests to your login page without affecting the rest of your site.</p>
    fn update_rate_based_rule(
        &self,
        input: UpdateRateBasedRuleRequest,
    ) -> RusotoFuture<UpdateRateBasedRuleResponse, UpdateRateBasedRuleError>;

    /// <p>Inserts or deletes <a>RegexMatchTuple</a> objects (filters) in a <a>RegexMatchSet</a>. For each <code>RegexMatchSetUpdate</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>RegexMatchSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to inspectupdate, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The identifier of the pattern (a regular expression) that you want AWS WAF to look for. For more information, see <a>RegexPatternSet</a>. </p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul> <p> For example, you can create a <code>RegexPatternSet</code> that matches any requests with <code>User-Agent</code> headers that contain the string <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>RegexMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>RegexMatchSet.</code> For more information, see <a>CreateRegexMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexMatchSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateRegexMatchSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the identifier of the <code>RegexPatternSet</code> that contain the regular expression patters you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_regex_match_set(
        &self,
        input: UpdateRegexMatchSetRequest,
    ) -> RusotoFuture<UpdateRegexMatchSetResponse, UpdateRegexMatchSetError>;

    /// <p>Inserts or deletes <code>RegexPatternString</code> objects in a <a>RegexPatternSet</a>. For each <code>RegexPatternString</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the <code>RegexPatternString</code>.</p> </li> <li> <p>The regular expression pattern that you want to insert or delete. For more information, see <a>RegexPatternSet</a>. </p> </li> </ul> <p> For example, you can create a <code>RegexPatternString</code> such as <code>B[a@]dB[o0]t</code>. AWS WAF will match this <code>RegexPatternString</code> to:</p> <ul> <li> <p>BadBot</p> </li> <li> <p>BadB0t</p> </li> <li> <p>B@dBot</p> </li> <li> <p>B@dB0t</p> </li> </ul> <p>To create and configure a <code>RegexPatternSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>RegexPatternSet.</code> For more information, see <a>CreateRegexPatternSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexPatternSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateRegexPatternSet</code> request to specify the regular expression pattern that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_regex_pattern_set(
        &self,
        input: UpdateRegexPatternSetRequest,
    ) -> RusotoFuture<UpdateRegexPatternSetResponse, UpdateRegexPatternSetError>;

    /// <p>Inserts or deletes <a>Predicate</a> objects in a <code>Rule</code>. Each <code>Predicate</code> object identifies a predicate, such as a <a>ByteMatchSet</a> or an <a>IPSet</a>, that specifies the web requests that you want to allow, block, or count. If you add more than one predicate to a <code>Rule</code>, a request must match all of the specifications to be allowed, blocked, or counted. For example, suppose that you add the following to a <code>Rule</code>: </p> <ul> <li> <p>A <code>ByteMatchSet</code> that matches the value <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44</code> </p> </li> </ul> <p>You then add the <code>Rule</code> to a <code>WebACL</code> and specify that you want to block requests that satisfy the <code>Rule</code>. For a request to be blocked, the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code> <i>and</i> the request must originate from the IP address 192.0.2.44.</p> <p>To create and configure a <code>Rule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the <code>Rule</code>.</p> </li> <li> <p>Create the <code>Rule</code>. See <a>CreateRule</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRule</code> request to add predicates to the <code>Rule</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>Rule</code>. See <a>CreateWebACL</a>.</p> </li> </ol> <p>If you want to replace one <code>ByteMatchSet</code> or <code>IPSet</code> with another, you delete the existing one and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_rule(
        &self,
        input: UpdateRuleRequest,
    ) -> RusotoFuture<UpdateRuleResponse, UpdateRuleError>;

    /// <p>Inserts or deletes <a>ActivatedRule</a> objects in a <code>RuleGroup</code>.</p> <p>You can only insert <code>REGULAR</code> rules into a rule group.</p> <p>You can have a maximum of ten rules per rule group.</p> <p>To create and configure a <code>RuleGroup</code>, perform the following steps:</p> <ol> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>RuleGroup</code>. See <a>CreateRule</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRuleGroup</a> request.</p> </li> <li> <p>Submit an <code>UpdateRuleGroup</code> request to add <code>Rules</code> to the <code>RuleGroup</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>RuleGroup</code>. See <a>CreateWebACL</a>.</p> </li> </ol> <p>If you want to replace one <code>Rule</code> with another, you delete the existing one and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_rule_group(
        &self,
        input: UpdateRuleGroupRequest,
    ) -> RusotoFuture<UpdateRuleGroupResponse, UpdateRuleGroupError>;

    /// <p>Inserts or deletes <a>SizeConstraint</a> objects (filters) in a <a>SizeConstraintSet</a>. For each <code>SizeConstraint</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>SizeConstraintSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to evaluate, such as the length of a query string or the length of the <code>User-Agent</code> header.</p> </li> <li> <p>Whether to perform any transformations on the request, such as converting it to lowercase, before checking its length. Note that transformations of the request body are not supported because the AWS resource forwards only the first <code>8192</code> bytes of your request to AWS WAF.</p> <p>You can only specify a single type of TextTransformation.</p> </li> <li> <p>A <code>ComparisonOperator</code> used for evaluating the selected part of the request against the specified <code>Size</code>, such as equals, greater than, less than, and so on.</p> </li> <li> <p>The length, in bytes, that you want AWS WAF to watch for in selected part of the request. The length is computed after applying the transformation.</p> </li> </ul> <p>For example, you can add a <code>SizeConstraintSetUpdate</code> object that matches web requests in which the length of the <code>User-Agent</code> header is greater than 100 bytes. You can then configure AWS WAF to block those requests.</p> <p>To create and configure a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>SizeConstraintSet.</code> For more information, see <a>CreateSizeConstraintSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateSizeConstraintSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_size_constraint_set(
        &self,
        input: UpdateSizeConstraintSetRequest,
    ) -> RusotoFuture<UpdateSizeConstraintSetResponse, UpdateSizeConstraintSetError>;

    /// <p>Inserts or deletes <a>SqlInjectionMatchTuple</a> objects (filters) in a <a>SqlInjectionMatchSet</a>. For each <code>SqlInjectionMatchTuple</code> object, you specify the following values:</p> <ul> <li> <p> <code>Action</code>: Whether to insert the object into or delete the object from the array. To change a <code>SqlInjectionMatchTuple</code>, you delete the existing object and add a new one.</p> </li> <li> <p> <code>FieldToMatch</code>: The part of web requests that you want AWS WAF to inspect and, if you want AWS WAF to inspect a header or custom query parameter, the name of the header or parameter.</p> </li> <li> <p> <code>TextTransformation</code>: Which text transformation, if any, to perform on the web request before inspecting the request for snippets of malicious SQL code.</p> <p>You can only specify a single type of TextTransformation.</p> </li> </ul> <p>You use <code>SqlInjectionMatchSet</code> objects to specify which CloudFront requests that you want to allow, block, or count. For example, if you're receiving requests that contain snippets of SQL code in the query string and you want to block the requests, you can create a <code>SqlInjectionMatchSet</code> with the applicable settings, and then configure AWS WAF to block the requests. </p> <p>To create and configure a <code>SqlInjectionMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateSqlInjectionMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateSqlInjectionMatchSet</code> request to specify the parts of web requests that you want AWS WAF to inspect for snippets of SQL code.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_sql_injection_match_set(
        &self,
        input: UpdateSqlInjectionMatchSetRequest,
    ) -> RusotoFuture<UpdateSqlInjectionMatchSetResponse, UpdateSqlInjectionMatchSetError>;

    /// <p>Inserts or deletes <a>ActivatedRule</a> objects in a <code>WebACL</code>. Each <code>Rule</code> identifies web requests that you want to allow, block, or count. When you update a <code>WebACL</code>, you specify the following values:</p> <ul> <li> <p>A default action for the <code>WebACL</code>, either <code>ALLOW</code> or <code>BLOCK</code>. AWS WAF performs the default action if a request doesn't match the criteria in any of the <code>Rules</code> in a <code>WebACL</code>.</p> </li> <li> <p>The <code>Rules</code> that you want to add or delete. If you want to replace one <code>Rule</code> with another, you delete the existing <code>Rule</code> and add the new one.</p> </li> <li> <p>For each <code>Rule</code>, whether you want AWS WAF to allow requests, block requests, or count requests that match the conditions in the <code>Rule</code>.</p> </li> <li> <p>The order in which you want AWS WAF to evaluate the <code>Rules</code> in a <code>WebACL</code>. If you add more than one <code>Rule</code> to a <code>WebACL</code>, AWS WAF evaluates each request against the <code>Rules</code> in order based on the value of <code>Priority</code>. (The <code>Rule</code> that has the lowest value for <code>Priority</code> is evaluated first.) When a web request matches all the predicates (such as <code>ByteMatchSets</code> and <code>IPSets</code>) in a <code>Rule</code>, AWS WAF immediately takes the corresponding action, allow or block, and doesn't evaluate the request against the remaining <code>Rules</code> in the <code>WebACL</code>, if any. </p> </li> </ul> <p>To create and configure a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in <code>Rules</code>. For more information, see <a>CreateByteMatchSet</a>, <a>UpdateByteMatchSet</a>, <a>CreateIPSet</a>, <a>UpdateIPSet</a>, <a>CreateSqlInjectionMatchSet</a>, and <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>WebACL</code>. For more information, see <a>CreateRule</a> and <a>UpdateRule</a>.</p> </li> <li> <p>Create a <code>WebACL</code>. See <a>CreateWebACL</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateWebACL</a> request.</p> </li> <li> <p>Submit an <code>UpdateWebACL</code> request to specify the <code>Rules</code> that you want to include in the <code>WebACL</code>, to specify the default action, and to associate the <code>WebACL</code> with a CloudFront distribution. </p> <p>The <code>ActivatedRule</code> can be a rule group. If you specify a rule group as your <code>ActivatedRule</code>, you can exclude specific rules from that rule group.</p> <p>If you already have a rule group associated with a web ACL and want to submit an <code>UpdateWebACL</code> request to exclude certain rules from that rule group, you must first remove the rule group from the web ACL, the re-insert it again, specifying the excluded rules. For details, see <a>ActivatedRule$ExcludedRules</a>. </p> </li> </ol> <p>Be aware that if you try to add a RATE_BASED rule to a web ACL without setting the rule type when first creating the rule, the <a>UpdateWebACL</a> request will fail because the request tries to add a REGULAR rule (the default rule type) with the specified ID, which does not exist. </p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_web_acl(
        &self,
        input: UpdateWebACLRequest,
    ) -> RusotoFuture<UpdateWebACLResponse, UpdateWebACLError>;

    /// <p>Inserts or deletes <a>XssMatchTuple</a> objects (filters) in an <a>XssMatchSet</a>. For each <code>XssMatchTuple</code> object, you specify the following values:</p> <ul> <li> <p> <code>Action</code>: Whether to insert the object into or delete the object from the array. To change an <code>XssMatchTuple</code>, you delete the existing object and add a new one.</p> </li> <li> <p> <code>FieldToMatch</code>: The part of web requests that you want AWS WAF to inspect and, if you want AWS WAF to inspect a header or custom query parameter, the name of the header or parameter.</p> </li> <li> <p> <code>TextTransformation</code>: Which text transformation, if any, to perform on the web request before inspecting the request for cross-site scripting attacks.</p> <p>You can only specify a single type of TextTransformation.</p> </li> </ul> <p>You use <code>XssMatchSet</code> objects to specify which CloudFront requests that you want to allow, block, or count. For example, if you're receiving requests that contain cross-site scripting attacks in the request body and you want to block the requests, you can create an <code>XssMatchSet</code> with the applicable settings, and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>XssMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateXssMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateXssMatchSet</code> request to specify the parts of web requests that you want AWS WAF to inspect for cross-site scripting attacks.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_xss_match_set(
        &self,
        input: UpdateXssMatchSetRequest,
    ) -> RusotoFuture<UpdateXssMatchSetResponse, UpdateXssMatchSetError>;
}
/// A client for the WAF Regional API.
#[derive(Clone)]
pub struct WAFRegionalClient {
    client: Client,
    region: region::Region,
}

impl WAFRegionalClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> WAFRegionalClient {
        WAFRegionalClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> WAFRegionalClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        WAFRegionalClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl WAFRegional for WAFRegionalClient {
    /// <p>Associates a web ACL with a resource, either an application load balancer or Amazon API Gateway stage.</p>
    fn associate_web_acl(
        &self,
        input: AssociateWebACLRequest,
    ) -> RusotoFuture<AssociateWebACLResponse, AssociateWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.AssociateWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateWebACLResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AssociateWebACLError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a <code>ByteMatchSet</code>. You then use <a>UpdateByteMatchSet</a> to identify the part of a web request that you want AWS WAF to inspect, such as the values of the <code>User-Agent</code> header or the query string. For example, you can create a <code>ByteMatchSet</code> that matches any requests with <code>User-Agent</code> headers that contain the string <code>BadBot</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateByteMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateByteMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateByteMatchSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateByteMatchSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_byte_match_set(
        &self,
        input: CreateByteMatchSetRequest,
    ) -> RusotoFuture<CreateByteMatchSetResponse, CreateByteMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.CreateByteMatchSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateByteMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateByteMatchSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an <a>GeoMatchSet</a>, which you use to specify which web requests you want to allow or block based on the country that the requests originate from. For example, if you're receiving a lot of requests from one or more countries and you want to block the requests, you can create an <code>GeoMatchSet</code> that contains those countries and then configure AWS WAF to block the requests. </p> <p>To create and configure a <code>GeoMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateGeoMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateGeoMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateGeoMatchSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateGeoMatchSetSet</code> request to specify the countries that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_geo_match_set(
        &self,
        input: CreateGeoMatchSetRequest,
    ) -> RusotoFuture<CreateGeoMatchSetResponse, CreateGeoMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.CreateGeoMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateGeoMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateGeoMatchSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an <a>IPSet</a>, which you use to specify which web requests that you want to allow or block based on the IP addresses that the requests originate from. For example, if you're receiving a lot of requests from one or more individual IP addresses or one or more ranges of IP addresses and you want to block the requests, you can create an <code>IPSet</code> that contains those IP addresses and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>IPSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateIPSet</code> request.</p> </li> <li> <p>Submit a <code>CreateIPSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateIPSet</code> request to specify the IP addresses that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_ip_set(
        &self,
        input: CreateIPSetRequest,
    ) -> RusotoFuture<CreateIPSetResponse, CreateIPSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.CreateIPSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateIPSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateIPSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a <a>RateBasedRule</a>. The <code>RateBasedRule</code> contains a <code>RateLimit</code>, which specifies the maximum number of requests that AWS WAF allows from a specified IP address in a five-minute period. The <code>RateBasedRule</code> also contains the <code>IPSet</code> objects, <code>ByteMatchSet</code> objects, and other predicates that identify the requests that you want to count or block if these requests exceed the <code>RateLimit</code>.</p> <p>If you add more than one predicate to a <code>RateBasedRule</code>, a request not only must exceed the <code>RateLimit</code>, but it also must match all the specifications to be counted or blocked. For example, suppose you add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>You then add the <code>RateBasedRule</code> to a <code>WebACL</code> and specify that you want to block requests that meet the conditions in the rule. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>. Further, requests that match these two conditions must be received at a rate of more than 15,000 requests every five minutes. If both conditions are met and the rate is exceeded, AWS WAF blocks the requests. If the rate drops below 15,000 for a five-minute period, AWS WAF no longer blocks the requests.</p> <p>As a second example, suppose you want to limit requests to a particular page on your site. To do this, you could add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>A <code>ByteMatchSet</code> with <code>FieldToMatch</code> of <code>URI</code> </p> </li> <li> <p>A <code>PositionalConstraint</code> of <code>STARTS_WITH</code> </p> </li> <li> <p>A <code>TargetString</code> of <code>login</code> </p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>By adding this <code>RateBasedRule</code> to a <code>WebACL</code>, you could limit requests to your login page without affecting the rest of your site.</p> <p>To create and configure a <code>RateBasedRule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the rule. For more information, see <a>CreateByteMatchSet</a>, <a>CreateIPSet</a>, and <a>CreateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRule</code> request.</p> </li> <li> <p>Submit a <code>CreateRateBasedRule</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRateBasedRule</code> request to specify the predicates that you want to include in the rule.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>RateBasedRule</code>. For more information, see <a>CreateWebACL</a>.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_rate_based_rule(
        &self,
        input: CreateRateBasedRuleRequest,
    ) -> RusotoFuture<CreateRateBasedRuleResponse, CreateRateBasedRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.CreateRateBasedRule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateRateBasedRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateRateBasedRuleError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a <a>RegexMatchSet</a>. You then use <a>UpdateRegexMatchSet</a> to identify the part of a web request that you want AWS WAF to inspect, such as the values of the <code>User-Agent</code> header or the query string. For example, you can create a <code>RegexMatchSet</code> that contains a <code>RegexMatchTuple</code> that looks for any requests with <code>User-Agent</code> headers that match a <code>RegexPatternSet</code> with pattern <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>RegexMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRegexMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateRegexMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexMatchSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateRegexMatchSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value, using a <code>RegexPatternSet</code>, that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_regex_match_set(
        &self,
        input: CreateRegexMatchSetRequest,
    ) -> RusotoFuture<CreateRegexMatchSetResponse, CreateRegexMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.CreateRegexMatchSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateRegexMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateRegexMatchSetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a <code>RegexPatternSet</code>. You then use <a>UpdateRegexPatternSet</a> to specify the regular expression (regex) pattern that you want AWS WAF to search for, such as <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>RegexPatternSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRegexPatternSet</code> request.</p> </li> <li> <p>Submit a <code>CreateRegexPatternSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexPatternSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateRegexPatternSet</a> request to specify the string that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_regex_pattern_set(
        &self,
        input: CreateRegexPatternSetRequest,
    ) -> RusotoFuture<CreateRegexPatternSetResponse, CreateRegexPatternSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.CreateRegexPatternSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateRegexPatternSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateRegexPatternSetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a <code>Rule</code>, which contains the <code>IPSet</code> objects, <code>ByteMatchSet</code> objects, and other predicates that identify the requests that you want to block. If you add more than one predicate to a <code>Rule</code>, a request must match all of the specifications to be allowed or blocked. For example, suppose that you add the following to a <code>Rule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>You then add the <code>Rule</code> to a <code>WebACL</code> and specify that you want to blocks requests that satisfy the <code>Rule</code>. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>.</p> <p>To create and configure a <code>Rule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the <code>Rule</code>. For more information, see <a>CreateByteMatchSet</a>, <a>CreateIPSet</a>, and <a>CreateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRule</code> request.</p> </li> <li> <p>Submit a <code>CreateRule</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRule</code> request to specify the predicates that you want to include in the <code>Rule</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>Rule</code>. For more information, see <a>CreateWebACL</a>.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_rule(
        &self,
        input: CreateRuleRequest,
    ) -> RusotoFuture<CreateRuleResponse, CreateRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.CreateRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateRuleError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a <code>RuleGroup</code>. A rule group is a collection of predefined rules that you add to a web ACL. You use <a>UpdateRuleGroup</a> to add rules to the rule group.</p> <p>Rule groups are subject to the following limits:</p> <ul> <li> <p>Three rule groups per account. You can request an increase to this limit by contacting customer support.</p> </li> <li> <p>One rule group per web ACL.</p> </li> <li> <p>Ten rules per rule group.</p> </li> </ul> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_rule_group(
        &self,
        input: CreateRuleGroupRequest,
    ) -> RusotoFuture<CreateRuleGroupResponse, CreateRuleGroupError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.CreateRuleGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateRuleGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateRuleGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a <code>SizeConstraintSet</code>. You then use <a>UpdateSizeConstraintSet</a> to identify the part of a web request that you want AWS WAF to check for length, such as the length of the <code>User-Agent</code> header or the length of the query string. For example, you can create a <code>SizeConstraintSet</code> that matches any requests that have a query string that is longer than 100 bytes. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit a <code>CreateSizeConstraintSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateSizeConstraintSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_size_constraint_set(
        &self,
        input: CreateSizeConstraintSetRequest,
    ) -> RusotoFuture<CreateSizeConstraintSetResponse, CreateSizeConstraintSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.CreateSizeConstraintSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateSizeConstraintSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateSizeConstraintSetError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a <a>SqlInjectionMatchSet</a>, which you use to allow, block, or count requests that contain snippets of SQL code in a specified part of web requests. AWS WAF searches for character sequences that are likely to be malicious strings.</p> <p>To create and configure a <code>SqlInjectionMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateSqlInjectionMatchSet</a> request.</p> </li> <li> <p>Submit an <a>UpdateSqlInjectionMatchSet</a> request to specify the parts of web requests in which you want to allow, block, or count malicious SQL code.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_sql_injection_match_set(
        &self,
        input: CreateSqlInjectionMatchSetRequest,
    ) -> RusotoFuture<CreateSqlInjectionMatchSetResponse, CreateSqlInjectionMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.CreateSqlInjectionMatchSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateSqlInjectionMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateSqlInjectionMatchSetError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a <code>WebACL</code>, which contains the <code>Rules</code> that identify the CloudFront web requests that you want to allow, block, or count. AWS WAF evaluates <code>Rules</code> in order based on the value of <code>Priority</code> for each <code>Rule</code>.</p> <p>You also specify a default action, either <code>ALLOW</code> or <code>BLOCK</code>. If a web request doesn't match any of the <code>Rules</code> in a <code>WebACL</code>, AWS WAF responds to the request with the default action. </p> <p>To create and configure a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Create and update the <code>ByteMatchSet</code> objects and other predicates that you want to include in <code>Rules</code>. For more information, see <a>CreateByteMatchSet</a>, <a>UpdateByteMatchSet</a>, <a>CreateIPSet</a>, <a>UpdateIPSet</a>, <a>CreateSqlInjectionMatchSet</a>, and <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>WebACL</code>. For more information, see <a>CreateRule</a> and <a>UpdateRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateWebACL</code> request.</p> </li> <li> <p>Submit a <code>CreateWebACL</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateWebACL</a> request.</p> </li> <li> <p>Submit an <a>UpdateWebACL</a> request to specify the <code>Rules</code> that you want to include in the <code>WebACL</code>, to specify the default action, and to associate the <code>WebACL</code> with a CloudFront distribution.</p> </li> </ol> <p>For more information about how to use the AWS WAF API, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_web_acl(
        &self,
        input: CreateWebACLRequest,
    ) -> RusotoFuture<CreateWebACLResponse, CreateWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.CreateWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateWebACLResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateWebACLError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an <a>XssMatchSet</a>, which you use to allow, block, or count requests that contain cross-site scripting attacks in the specified part of web requests. AWS WAF searches for character sequences that are likely to be malicious strings.</p> <p>To create and configure an <code>XssMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateXssMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateXssMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateXssMatchSet</a> request.</p> </li> <li> <p>Submit an <a>UpdateXssMatchSet</a> request to specify the parts of web requests in which you want to allow, block, or count cross-site scripting attacks.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn create_xss_match_set(
        &self,
        input: CreateXssMatchSetRequest,
    ) -> RusotoFuture<CreateXssMatchSetResponse, CreateXssMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.CreateXssMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateXssMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateXssMatchSetError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Permanently deletes a <a>ByteMatchSet</a>. You can&#39;t delete a <code>ByteMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any <a>ByteMatchTuple</a> objects (any filters).</p> <p>If you just want to remove a <code>ByteMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>ByteMatchSet</code> to remove filters, if any. For more information, see <a>UpdateByteMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteByteMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteByteMatchSet</code> request.</p> </li> </ol></p>
    fn delete_byte_match_set(
        &self,
        input: DeleteByteMatchSetRequest,
    ) -> RusotoFuture<DeleteByteMatchSetResponse, DeleteByteMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.DeleteByteMatchSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteByteMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteByteMatchSetError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Permanently deletes a <a>GeoMatchSet</a>. You can&#39;t delete a <code>GeoMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any countries.</p> <p>If you just want to remove a <code>GeoMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>GeoMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>GeoMatchSet</code> to remove any countries. For more information, see <a>UpdateGeoMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteGeoMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteGeoMatchSet</code> request.</p> </li> </ol></p>
    fn delete_geo_match_set(
        &self,
        input: DeleteGeoMatchSetRequest,
    ) -> RusotoFuture<DeleteGeoMatchSetResponse, DeleteGeoMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.DeleteGeoMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteGeoMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteGeoMatchSetError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Permanently deletes an <a>IPSet</a>. You can&#39;t delete an <code>IPSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any IP addresses.</p> <p>If you just want to remove an <code>IPSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete an <code>IPSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>IPSet</code> to remove IP address ranges, if any. For more information, see <a>UpdateIPSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteIPSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteIPSet</code> request.</p> </li> </ol></p>
    fn delete_ip_set(
        &self,
        input: DeleteIPSetRequest,
    ) -> RusotoFuture<DeleteIPSetResponse, DeleteIPSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.DeleteIPSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteIPSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteIPSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Permanently deletes the <a>LoggingConfiguration</a> from the specified web ACL.</p>
    fn delete_logging_configuration(
        &self,
        input: DeleteLoggingConfigurationRequest,
    ) -> RusotoFuture<DeleteLoggingConfigurationResponse, DeleteLoggingConfigurationError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.DeleteLoggingConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteLoggingConfigurationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLoggingConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Permanently deletes an IAM policy from the specified RuleGroup.</p> <p>The user making the request must be the owner of the RuleGroup.</p>
    fn delete_permission_policy(
        &self,
        input: DeletePermissionPolicyRequest,
    ) -> RusotoFuture<DeletePermissionPolicyResponse, DeletePermissionPolicyError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.DeletePermissionPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeletePermissionPolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeletePermissionPolicyError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Permanently deletes a <a>RateBasedRule</a>. You can&#39;t delete a rule if it&#39;s still used in any <code>WebACL</code> objects or if it still includes any predicates, such as <code>ByteMatchSet</code> objects.</p> <p>If you just want to remove a rule from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>RateBasedRule</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>RateBasedRule</code> to remove predicates, if any. For more information, see <a>UpdateRateBasedRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRateBasedRule</code> request.</p> </li> <li> <p>Submit a <code>DeleteRateBasedRule</code> request.</p> </li> </ol></p>
    fn delete_rate_based_rule(
        &self,
        input: DeleteRateBasedRuleRequest,
    ) -> RusotoFuture<DeleteRateBasedRuleResponse, DeleteRateBasedRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.DeleteRateBasedRule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRateBasedRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteRateBasedRuleError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Permanently deletes a <a>RegexMatchSet</a>. You can&#39;t delete a <code>RegexMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any <code>RegexMatchTuples</code> objects (any filters).</p> <p>If you just want to remove a <code>RegexMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>RegexMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>RegexMatchSet</code> to remove filters, if any. For more information, see <a>UpdateRegexMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRegexMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteRegexMatchSet</code> request.</p> </li> </ol></p>
    fn delete_regex_match_set(
        &self,
        input: DeleteRegexMatchSetRequest,
    ) -> RusotoFuture<DeleteRegexMatchSetResponse, DeleteRegexMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.DeleteRegexMatchSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRegexMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteRegexMatchSetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Permanently deletes a <a>RegexPatternSet</a>. You can't delete a <code>RegexPatternSet</code> if it's still used in any <code>RegexMatchSet</code> or if the <code>RegexPatternSet</code> is not empty. </p>
    fn delete_regex_pattern_set(
        &self,
        input: DeleteRegexPatternSetRequest,
    ) -> RusotoFuture<DeleteRegexPatternSetResponse, DeleteRegexPatternSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.DeleteRegexPatternSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRegexPatternSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteRegexPatternSetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Permanently deletes a <a>Rule</a>. You can&#39;t delete a <code>Rule</code> if it&#39;s still used in any <code>WebACL</code> objects or if it still includes any predicates, such as <code>ByteMatchSet</code> objects.</p> <p>If you just want to remove a <code>Rule</code> from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>Rule</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>Rule</code> to remove predicates, if any. For more information, see <a>UpdateRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRule</code> request.</p> </li> <li> <p>Submit a <code>DeleteRule</code> request.</p> </li> </ol></p>
    fn delete_rule(
        &self,
        input: DeleteRuleRequest,
    ) -> RusotoFuture<DeleteRuleResponse, DeleteRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.DeleteRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteRuleError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Permanently deletes a <a>RuleGroup</a>. You can&#39;t delete a <code>RuleGroup</code> if it&#39;s still used in any <code>WebACL</code> objects or if it still includes any rules.</p> <p>If you just want to remove a <code>RuleGroup</code> from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>RuleGroup</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>RuleGroup</code> to remove rules, if any. For more information, see <a>UpdateRuleGroup</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRuleGroup</code> request.</p> </li> <li> <p>Submit a <code>DeleteRuleGroup</code> request.</p> </li> </ol></p>
    fn delete_rule_group(
        &self,
        input: DeleteRuleGroupRequest,
    ) -> RusotoFuture<DeleteRuleGroupResponse, DeleteRuleGroupError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.DeleteRuleGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRuleGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteRuleGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Permanently deletes a <a>SizeConstraintSet</a>. You can&#39;t delete a <code>SizeConstraintSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any <a>SizeConstraint</a> objects (any filters).</p> <p>If you just want to remove a <code>SizeConstraintSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>SizeConstraintSet</code> to remove filters, if any. For more information, see <a>UpdateSizeConstraintSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteSizeConstraintSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteSizeConstraintSet</code> request.</p> </li> </ol></p>
    fn delete_size_constraint_set(
        &self,
        input: DeleteSizeConstraintSetRequest,
    ) -> RusotoFuture<DeleteSizeConstraintSetResponse, DeleteSizeConstraintSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.DeleteSizeConstraintSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteSizeConstraintSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSizeConstraintSetError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Permanently deletes a <a>SqlInjectionMatchSet</a>. You can&#39;t delete a <code>SqlInjectionMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still contains any <a>SqlInjectionMatchTuple</a> objects.</p> <p>If you just want to remove a <code>SqlInjectionMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>SqlInjectionMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>SqlInjectionMatchSet</code> to remove filters, if any. For more information, see <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteSqlInjectionMatchSet</code> request.</p> </li> </ol></p>
    fn delete_sql_injection_match_set(
        &self,
        input: DeleteSqlInjectionMatchSetRequest,
    ) -> RusotoFuture<DeleteSqlInjectionMatchSetResponse, DeleteSqlInjectionMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.DeleteSqlInjectionMatchSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteSqlInjectionMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSqlInjectionMatchSetError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Permanently deletes a <a>WebACL</a>. You can&#39;t delete a <code>WebACL</code> if it still contains any <code>Rules</code>.</p> <p>To delete a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>WebACL</code> to remove <code>Rules</code>, if any. For more information, see <a>UpdateWebACL</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteWebACL</code> request.</p> </li> <li> <p>Submit a <code>DeleteWebACL</code> request.</p> </li> </ol></p>
    fn delete_web_acl(
        &self,
        input: DeleteWebACLRequest,
    ) -> RusotoFuture<DeleteWebACLResponse, DeleteWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.DeleteWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteWebACLResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteWebACLError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Permanently deletes an <a>XssMatchSet</a>. You can&#39;t delete an <code>XssMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still contains any <a>XssMatchTuple</a> objects.</p> <p>If you just want to remove an <code>XssMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete an <code>XssMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>XssMatchSet</code> to remove filters, if any. For more information, see <a>UpdateXssMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteXssMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteXssMatchSet</code> request.</p> </li> </ol></p>
    fn delete_xss_match_set(
        &self,
        input: DeleteXssMatchSetRequest,
    ) -> RusotoFuture<DeleteXssMatchSetResponse, DeleteXssMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.DeleteXssMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteXssMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteXssMatchSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes a web ACL from the specified resource, either an application load balancer or Amazon API Gateway stage.</p>
    fn disassociate_web_acl(
        &self,
        input: DisassociateWebACLRequest,
    ) -> RusotoFuture<DisassociateWebACLResponse, DisassociateWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.DisassociateWebACL",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateWebACLResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DisassociateWebACLError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the <a>ByteMatchSet</a> specified by <code>ByteMatchSetId</code>.</p>
    fn get_byte_match_set(
        &self,
        input: GetByteMatchSetRequest,
    ) -> RusotoFuture<GetByteMatchSetResponse, GetByteMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetByteMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetByteMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetByteMatchSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>When you want to create, update, or delete AWS WAF objects, get a change token and include the change token in the create, update, or delete request. Change tokens ensure that your application doesn't submit conflicting requests to AWS WAF.</p> <p>Each create, update, or delete request must use a unique change token. If your application submits a <code>GetChangeToken</code> request and then submits a second <code>GetChangeToken</code> request before submitting a create, update, or delete request, the second <code>GetChangeToken</code> request returns the same value as the first <code>GetChangeToken</code> request.</p> <p>When you use a change token in a create, update, or delete request, the status of the change token changes to <code>PENDING</code>, which indicates that AWS WAF is propagating the change to all AWS WAF servers. Use <code>GetChangeTokenStatus</code> to determine the status of your change token.</p>
    fn get_change_token(&self) -> RusotoFuture<GetChangeTokenResponse, GetChangeTokenError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetChangeToken");
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetChangeTokenResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetChangeTokenError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Returns the status of a <code>ChangeToken</code> that you got by calling <a>GetChangeToken</a>. <code>ChangeTokenStatus</code> is one of the following values:</p> <ul> <li> <p> <code>PROVISIONED</code>: You requested the change token by calling <code>GetChangeToken</code>, but you haven&#39;t used it yet in a call to create, update, or delete an AWS WAF object.</p> </li> <li> <p> <code>PENDING</code>: AWS WAF is propagating the create, update, or delete request to all AWS WAF servers.</p> </li> <li> <p> <code>IN_SYNC</code>: Propagation is complete.</p> </li> </ul></p>
    fn get_change_token_status(
        &self,
        input: GetChangeTokenStatusRequest,
    ) -> RusotoFuture<GetChangeTokenStatusResponse, GetChangeTokenStatusError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.GetChangeTokenStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetChangeTokenStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetChangeTokenStatusError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the <a>GeoMatchSet</a> that is specified by <code>GeoMatchSetId</code>.</p>
    fn get_geo_match_set(
        &self,
        input: GetGeoMatchSetRequest,
    ) -> RusotoFuture<GetGeoMatchSetResponse, GetGeoMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetGeoMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetGeoMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGeoMatchSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the <a>IPSet</a> that is specified by <code>IPSetId</code>.</p>
    fn get_ip_set(&self, input: GetIPSetRequest) -> RusotoFuture<GetIPSetResponse, GetIPSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetIPSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetIPSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetIPSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the <a>LoggingConfiguration</a> for the specified web ACL.</p>
    fn get_logging_configuration(
        &self,
        input: GetLoggingConfigurationRequest,
    ) -> RusotoFuture<GetLoggingConfigurationResponse, GetLoggingConfigurationError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.GetLoggingConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetLoggingConfigurationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetLoggingConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns the IAM policy attached to the RuleGroup.</p>
    fn get_permission_policy(
        &self,
        input: GetPermissionPolicyRequest,
    ) -> RusotoFuture<GetPermissionPolicyResponse, GetPermissionPolicyError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.GetPermissionPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetPermissionPolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetPermissionPolicyError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the <a>RateBasedRule</a> that is specified by the <code>RuleId</code> that you included in the <code>GetRateBasedRule</code> request.</p>
    fn get_rate_based_rule(
        &self,
        input: GetRateBasedRuleRequest,
    ) -> RusotoFuture<GetRateBasedRuleResponse, GetRateBasedRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetRateBasedRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRateBasedRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRateBasedRuleError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of IP addresses currently being blocked by the <a>RateBasedRule</a> that is specified by the <code>RuleId</code>. The maximum number of managed keys that will be blocked is 10,000. If more than 10,000 addresses exceed the rate limit, the 10,000 addresses with the highest rates will be blocked.</p>
    fn get_rate_based_rule_managed_keys(
        &self,
        input: GetRateBasedRuleManagedKeysRequest,
    ) -> RusotoFuture<GetRateBasedRuleManagedKeysResponse, GetRateBasedRuleManagedKeysError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.GetRateBasedRuleManagedKeys",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRateBasedRuleManagedKeysResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRateBasedRuleManagedKeysError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns the <a>RegexMatchSet</a> specified by <code>RegexMatchSetId</code>.</p>
    fn get_regex_match_set(
        &self,
        input: GetRegexMatchSetRequest,
    ) -> RusotoFuture<GetRegexMatchSetResponse, GetRegexMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetRegexMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRegexMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRegexMatchSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the <a>RegexPatternSet</a> specified by <code>RegexPatternSetId</code>.</p>
    fn get_regex_pattern_set(
        &self,
        input: GetRegexPatternSetRequest,
    ) -> RusotoFuture<GetRegexPatternSetResponse, GetRegexPatternSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.GetRegexPatternSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRegexPatternSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRegexPatternSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the <a>Rule</a> that is specified by the <code>RuleId</code> that you included in the <code>GetRule</code> request.</p>
    fn get_rule(&self, input: GetRuleRequest) -> RusotoFuture<GetRuleResponse, GetRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRuleError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the <a>RuleGroup</a> that is specified by the <code>RuleGroupId</code> that you included in the <code>GetRuleGroup</code> request.</p> <p>To view the rules in a rule group, use <a>ListActivatedRulesInRuleGroup</a>.</p>
    fn get_rule_group(
        &self,
        input: GetRuleGroupRequest,
    ) -> RusotoFuture<GetRuleGroupResponse, GetRuleGroupError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetRuleGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRuleGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRuleGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets detailed information about a specified number of requests--a sample--that AWS WAF randomly selects from among the first 5,000 requests that your AWS resource received during a time range that you choose. You can specify a sample size of up to 500 requests, and you can specify any time range in the previous three hours.</p> <p> <code>GetSampledRequests</code> returns a time range, which is usually the time range that you specified. However, if your resource (such as a CloudFront distribution) received 5,000 requests before the specified time range elapsed, <code>GetSampledRequests</code> returns an updated time range. This new time range indicates the actual period during which AWS WAF selected the requests in the sample.</p>
    fn get_sampled_requests(
        &self,
        input: GetSampledRequestsRequest,
    ) -> RusotoFuture<GetSampledRequestsResponse, GetSampledRequestsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.GetSampledRequests",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetSampledRequestsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSampledRequestsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the <a>SizeConstraintSet</a> specified by <code>SizeConstraintSetId</code>.</p>
    fn get_size_constraint_set(
        &self,
        input: GetSizeConstraintSetRequest,
    ) -> RusotoFuture<GetSizeConstraintSetResponse, GetSizeConstraintSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.GetSizeConstraintSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetSizeConstraintSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetSizeConstraintSetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the <a>SqlInjectionMatchSet</a> that is specified by <code>SqlInjectionMatchSetId</code>.</p>
    fn get_sql_injection_match_set(
        &self,
        input: GetSqlInjectionMatchSetRequest,
    ) -> RusotoFuture<GetSqlInjectionMatchSetResponse, GetSqlInjectionMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.GetSqlInjectionMatchSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetSqlInjectionMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSqlInjectionMatchSetError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns the <a>WebACL</a> that is specified by <code>WebACLId</code>.</p>
    fn get_web_acl(
        &self,
        input: GetWebACLRequest,
    ) -> RusotoFuture<GetWebACLResponse, GetWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetWebACLResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetWebACLError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the web ACL for the specified resource, either an application load balancer or Amazon API Gateway stage.</p>
    fn get_web_acl_for_resource(
        &self,
        input: GetWebACLForResourceRequest,
    ) -> RusotoFuture<GetWebACLForResourceResponse, GetWebACLForResourceError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.GetWebACLForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetWebACLForResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetWebACLForResourceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the <a>XssMatchSet</a> that is specified by <code>XssMatchSetId</code>.</p>
    fn get_xss_match_set(
        &self,
        input: GetXssMatchSetRequest,
    ) -> RusotoFuture<GetXssMatchSetResponse, GetXssMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetXssMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetXssMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetXssMatchSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <a>ActivatedRule</a> objects.</p>
    fn list_activated_rules_in_rule_group(
        &self,
        input: ListActivatedRulesInRuleGroupRequest,
    ) -> RusotoFuture<ListActivatedRulesInRuleGroupResponse, ListActivatedRulesInRuleGroupError>
    {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.ListActivatedRulesInRuleGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListActivatedRulesInRuleGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListActivatedRulesInRuleGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns an array of <a>ByteMatchSetSummary</a> objects.</p>
    fn list_byte_match_sets(
        &self,
        input: ListByteMatchSetsRequest,
    ) -> RusotoFuture<ListByteMatchSetsResponse, ListByteMatchSetsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.ListByteMatchSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListByteMatchSetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListByteMatchSetsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <a>GeoMatchSetSummary</a> objects in the response.</p>
    fn list_geo_match_sets(
        &self,
        input: ListGeoMatchSetsRequest,
    ) -> RusotoFuture<ListGeoMatchSetsResponse, ListGeoMatchSetsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.ListGeoMatchSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListGeoMatchSetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListGeoMatchSetsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <a>IPSetSummary</a> objects in the response.</p>
    fn list_ip_sets(
        &self,
        input: ListIPSetsRequest,
    ) -> RusotoFuture<ListIPSetsResponse, ListIPSetsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.ListIPSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListIPSetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListIPSetsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <a>LoggingConfiguration</a> objects.</p>
    fn list_logging_configurations(
        &self,
        input: ListLoggingConfigurationsRequest,
    ) -> RusotoFuture<ListLoggingConfigurationsResponse, ListLoggingConfigurationsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.ListLoggingConfigurations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListLoggingConfigurationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListLoggingConfigurationsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns an array of <a>RuleSummary</a> objects.</p>
    fn list_rate_based_rules(
        &self,
        input: ListRateBasedRulesRequest,
    ) -> RusotoFuture<ListRateBasedRulesResponse, ListRateBasedRulesError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.ListRateBasedRules",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListRateBasedRulesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListRateBasedRulesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <a>RegexMatchSetSummary</a> objects.</p>
    fn list_regex_match_sets(
        &self,
        input: ListRegexMatchSetsRequest,
    ) -> RusotoFuture<ListRegexMatchSetsResponse, ListRegexMatchSetsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.ListRegexMatchSets",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListRegexMatchSetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListRegexMatchSetsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <a>RegexPatternSetSummary</a> objects.</p>
    fn list_regex_pattern_sets(
        &self,
        input: ListRegexPatternSetsRequest,
    ) -> RusotoFuture<ListRegexPatternSetsResponse, ListRegexPatternSetsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.ListRegexPatternSets",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListRegexPatternSetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListRegexPatternSetsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns an array of resources associated with the specified web ACL.</p>
    fn list_resources_for_web_acl(
        &self,
        input: ListResourcesForWebACLRequest,
    ) -> RusotoFuture<ListResourcesForWebACLResponse, ListResourcesForWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.ListResourcesForWebACL",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListResourcesForWebACLResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListResourcesForWebACLError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns an array of <a>RuleGroup</a> objects.</p>
    fn list_rule_groups(
        &self,
        input: ListRuleGroupsRequest,
    ) -> RusotoFuture<ListRuleGroupsResponse, ListRuleGroupsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.ListRuleGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListRuleGroupsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListRuleGroupsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <a>RuleSummary</a> objects.</p>
    fn list_rules(
        &self,
        input: ListRulesRequest,
    ) -> RusotoFuture<ListRulesResponse, ListRulesError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.ListRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListRulesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListRulesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <a>SizeConstraintSetSummary</a> objects.</p>
    fn list_size_constraint_sets(
        &self,
        input: ListSizeConstraintSetsRequest,
    ) -> RusotoFuture<ListSizeConstraintSetsResponse, ListSizeConstraintSetsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.ListSizeConstraintSets",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListSizeConstraintSetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListSizeConstraintSetsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns an array of <a>SqlInjectionMatchSet</a> objects.</p>
    fn list_sql_injection_match_sets(
        &self,
        input: ListSqlInjectionMatchSetsRequest,
    ) -> RusotoFuture<ListSqlInjectionMatchSetsResponse, ListSqlInjectionMatchSetsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.ListSqlInjectionMatchSets",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListSqlInjectionMatchSetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSqlInjectionMatchSetsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns an array of <a>RuleGroup</a> objects that you are subscribed to.</p>
    fn list_subscribed_rule_groups(
        &self,
        input: ListSubscribedRuleGroupsRequest,
    ) -> RusotoFuture<ListSubscribedRuleGroupsResponse, ListSubscribedRuleGroupsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.ListSubscribedRuleGroups",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListSubscribedRuleGroupsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSubscribedRuleGroupsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns an array of <a>WebACLSummary</a> objects in the response.</p>
    fn list_web_ac_ls(
        &self,
        input: ListWebACLsRequest,
    ) -> RusotoFuture<ListWebACLsResponse, ListWebACLsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.ListWebACLs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListWebACLsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListWebACLsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <a>XssMatchSet</a> objects.</p>
    fn list_xss_match_sets(
        &self,
        input: ListXssMatchSetsRequest,
    ) -> RusotoFuture<ListXssMatchSetsResponse, ListXssMatchSetsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.ListXssMatchSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListXssMatchSetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListXssMatchSetsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Associates a <a>LoggingConfiguration</a> with a specified web ACL.</p> <p>You can access information about all traffic that AWS WAF inspects using the following steps:</p> <ol> <li> <p>Create an Amazon Kinesis Data Firehose . </p> </li> <li> <p>Associate that firehose to your web ACL using a <code>PutLoggingConfiguration</code> request.</p> </li> </ol> <p>When you successfully enable logging using a <code>PutLoggingConfiguration</code> request, AWS WAF will create a service linked role with the necessary permissions to write logs to the Amazon Kinesis Data Firehose. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/logging.html">Logging Web ACL Traffic Information</a> in the <i>AWS WAF Developer Guide</i>.</p>
    fn put_logging_configuration(
        &self,
        input: PutLoggingConfigurationRequest,
    ) -> RusotoFuture<PutLoggingConfigurationResponse, PutLoggingConfigurationError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.PutLoggingConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutLoggingConfigurationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutLoggingConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Attaches a IAM policy to the specified resource. The only supported use for this action is to share a RuleGroup across accounts.</p> <p>The <code>PutPermissionPolicy</code> is subject to the following restrictions:</p> <ul> <li> <p>You can attach only one policy with each <code>PutPermissionPolicy</code> request.</p> </li> <li> <p>The policy must include an <code>Effect</code>, <code>Action</code> and <code>Principal</code>. </p> </li> <li> <p> <code>Effect</code> must specify <code>Allow</code>.</p> </li> <li> <p>The <code>Action</code> in the policy must be <code>waf:UpdateWebACL</code>, <code>waf-regional:UpdateWebACL</code>, <code>waf:GetRuleGroup</code> and <code>waf-regional:GetRuleGroup</code> . Any extra or wildcard actions in the policy will be rejected.</p> </li> <li> <p>The policy cannot include a <code>Resource</code> parameter.</p> </li> <li> <p>The ARN in the request must be a valid WAF RuleGroup ARN and the RuleGroup must exist in the same region.</p> </li> <li> <p>The user making the request must be the owner of the RuleGroup.</p> </li> <li> <p>Your policy must be composed using IAM Policy version 2012-10-17.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html">IAM Policies</a>. </p> <p>An example of a valid policy parameter is shown in the Examples section below.</p>
    fn put_permission_policy(
        &self,
        input: PutPermissionPolicyRequest,
    ) -> RusotoFuture<PutPermissionPolicyResponse, PutPermissionPolicyError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.PutPermissionPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutPermissionPolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutPermissionPolicyError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Inserts or deletes <a>ByteMatchTuple</a> objects (filters) in a <a>ByteMatchSet</a>. For each <code>ByteMatchTuple</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>ByteMatchSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to inspect, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to look for. For more information, including how you specify the values for the AWS WAF API and the AWS CLI or SDKs, see <code>TargetString</code> in the <a>ByteMatchTuple</a> data type. </p> </li> <li> <p>Where to look, such as at the beginning or the end of a query string.</p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul> <p>For example, you can add a <code>ByteMatchSetUpdate</code> object that matches web requests in which <code>User-Agent</code> headers contain the string <code>BadBot</code>. You can then configure AWS WAF to block those requests.</p> <p>To create and configure a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>ByteMatchSet.</code> For more information, see <a>CreateByteMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateByteMatchSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateByteMatchSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_byte_match_set(
        &self,
        input: UpdateByteMatchSetRequest,
    ) -> RusotoFuture<UpdateByteMatchSetResponse, UpdateByteMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.UpdateByteMatchSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateByteMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateByteMatchSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Inserts or deletes <a>GeoMatchConstraint</a> objects in an <code>GeoMatchSet</code>. For each <code>GeoMatchConstraint</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change an <code>GeoMatchConstraint</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The <code>Type</code>. The only valid value for <code>Type</code> is <code>Country</code>.</p> </li> <li> <p>The <code>Value</code>, which is a two character code for the country to add to the <code>GeoMatchConstraint</code> object. Valid codes are listed in <a>GeoMatchConstraint$Value</a>.</p> </li> </ul> <p>To create and configure an <code>GeoMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateGeoMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateGeoMatchSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateGeoMatchSet</code> request to specify the country that you want AWS WAF to watch for.</p> </li> </ol> <p>When you update an <code>GeoMatchSet</code>, you specify the country that you want to add and/or the country that you want to delete. If you want to change a country, you delete the existing country and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_geo_match_set(
        &self,
        input: UpdateGeoMatchSetRequest,
    ) -> RusotoFuture<UpdateGeoMatchSetResponse, UpdateGeoMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.UpdateGeoMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateGeoMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateGeoMatchSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Inserts or deletes <a>IPSetDescriptor</a> objects in an <code>IPSet</code>. For each <code>IPSetDescriptor</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change an <code>IPSetDescriptor</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The IP address version, <code>IPv4</code> or <code>IPv6</code>. </p> </li> <li> <p>The IP address in CIDR notation, for example, <code>192.0.2.0/24</code> (for the range of IP addresses from <code>192.0.2.0</code> to <code>192.0.2.255</code>) or <code>192.0.2.44/32</code> (for the individual IP address <code>192.0.2.44</code>). </p> </li> </ul> <p>AWS WAF supports IPv4 address ranges: /8 and any range between /16 through /32. AWS WAF supports IPv6 address ranges: /16, /24, /32, /48, /56, /64, and /128. For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p> <p>IPv6 addresses can be represented using any of the following formats:</p> <ul> <li> <p>1111:0000:0000:0000:0000:0000:0000:0111/128</p> </li> <li> <p>1111:0:0:0:0:0:0:0111/128</p> </li> <li> <p>1111::0111/128</p> </li> <li> <p>1111::111/128</p> </li> </ul> <p>You use an <code>IPSet</code> to specify which web requests you want to allow or block based on the IP addresses that the requests originated from. For example, if you're receiving a lot of requests from one or a small number of IP addresses and you want to block the requests, you can create an <code>IPSet</code> that specifies those IP addresses, and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>IPSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateIPSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateIPSet</code> request to specify the IP addresses that you want AWS WAF to watch for.</p> </li> </ol> <p>When you update an <code>IPSet</code>, you specify the IP addresses that you want to add and/or the IP addresses that you want to delete. If you want to change an IP address, you delete the existing IP address and add the new one.</p> <p>You can insert a maximum of 1000 addresses in a single request.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_ip_set(
        &self,
        input: UpdateIPSetRequest,
    ) -> RusotoFuture<UpdateIPSetResponse, UpdateIPSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.UpdateIPSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateIPSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateIPSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Inserts or deletes <a>Predicate</a> objects in a rule and updates the <code>RateLimit</code> in the rule. </p> <p>Each <code>Predicate</code> object identifies a predicate, such as a <a>ByteMatchSet</a> or an <a>IPSet</a>, that specifies the web requests that you want to block or count. The <code>RateLimit</code> specifies the number of requests every five minutes that triggers the rule.</p> <p>If you add more than one predicate to a <code>RateBasedRule</code>, a request must match all the predicates and exceed the <code>RateLimit</code> to be counted or blocked. For example, suppose you add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>You then add the <code>RateBasedRule</code> to a <code>WebACL</code> and specify that you want to block requests that satisfy the rule. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>. Further, requests that match these two conditions much be received at a rate of more than 15,000 every five minutes. If the rate drops below this limit, AWS WAF no longer blocks the requests.</p> <p>As a second example, suppose you want to limit requests to a particular page on your site. To do this, you could add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>A <code>ByteMatchSet</code> with <code>FieldToMatch</code> of <code>URI</code> </p> </li> <li> <p>A <code>PositionalConstraint</code> of <code>STARTS_WITH</code> </p> </li> <li> <p>A <code>TargetString</code> of <code>login</code> </p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>By adding this <code>RateBasedRule</code> to a <code>WebACL</code>, you could limit requests to your login page without affecting the rest of your site.</p>
    fn update_rate_based_rule(
        &self,
        input: UpdateRateBasedRuleRequest,
    ) -> RusotoFuture<UpdateRateBasedRuleResponse, UpdateRateBasedRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.UpdateRateBasedRule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateRateBasedRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateRateBasedRuleError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Inserts or deletes <a>RegexMatchTuple</a> objects (filters) in a <a>RegexMatchSet</a>. For each <code>RegexMatchSetUpdate</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>RegexMatchSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to inspectupdate, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The identifier of the pattern (a regular expression) that you want AWS WAF to look for. For more information, see <a>RegexPatternSet</a>. </p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul> <p> For example, you can create a <code>RegexPatternSet</code> that matches any requests with <code>User-Agent</code> headers that contain the string <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>RegexMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>RegexMatchSet.</code> For more information, see <a>CreateRegexMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexMatchSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateRegexMatchSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the identifier of the <code>RegexPatternSet</code> that contain the regular expression patters you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_regex_match_set(
        &self,
        input: UpdateRegexMatchSetRequest,
    ) -> RusotoFuture<UpdateRegexMatchSetResponse, UpdateRegexMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.UpdateRegexMatchSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateRegexMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateRegexMatchSetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Inserts or deletes <code>RegexPatternString</code> objects in a <a>RegexPatternSet</a>. For each <code>RegexPatternString</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the <code>RegexPatternString</code>.</p> </li> <li> <p>The regular expression pattern that you want to insert or delete. For more information, see <a>RegexPatternSet</a>. </p> </li> </ul> <p> For example, you can create a <code>RegexPatternString</code> such as <code>B[a@]dB[o0]t</code>. AWS WAF will match this <code>RegexPatternString</code> to:</p> <ul> <li> <p>BadBot</p> </li> <li> <p>BadB0t</p> </li> <li> <p>B@dBot</p> </li> <li> <p>B@dB0t</p> </li> </ul> <p>To create and configure a <code>RegexPatternSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>RegexPatternSet.</code> For more information, see <a>CreateRegexPatternSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexPatternSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateRegexPatternSet</code> request to specify the regular expression pattern that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_regex_pattern_set(
        &self,
        input: UpdateRegexPatternSetRequest,
    ) -> RusotoFuture<UpdateRegexPatternSetResponse, UpdateRegexPatternSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.UpdateRegexPatternSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateRegexPatternSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateRegexPatternSetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Inserts or deletes <a>Predicate</a> objects in a <code>Rule</code>. Each <code>Predicate</code> object identifies a predicate, such as a <a>ByteMatchSet</a> or an <a>IPSet</a>, that specifies the web requests that you want to allow, block, or count. If you add more than one predicate to a <code>Rule</code>, a request must match all of the specifications to be allowed, blocked, or counted. For example, suppose that you add the following to a <code>Rule</code>: </p> <ul> <li> <p>A <code>ByteMatchSet</code> that matches the value <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44</code> </p> </li> </ul> <p>You then add the <code>Rule</code> to a <code>WebACL</code> and specify that you want to block requests that satisfy the <code>Rule</code>. For a request to be blocked, the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code> <i>and</i> the request must originate from the IP address 192.0.2.44.</p> <p>To create and configure a <code>Rule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the <code>Rule</code>.</p> </li> <li> <p>Create the <code>Rule</code>. See <a>CreateRule</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRule</code> request to add predicates to the <code>Rule</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>Rule</code>. See <a>CreateWebACL</a>.</p> </li> </ol> <p>If you want to replace one <code>ByteMatchSet</code> or <code>IPSet</code> with another, you delete the existing one and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_rule(
        &self,
        input: UpdateRuleRequest,
    ) -> RusotoFuture<UpdateRuleResponse, UpdateRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.UpdateRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateRuleError::from_response(response))),
                )
            }
        })
    }

    /// <p>Inserts or deletes <a>ActivatedRule</a> objects in a <code>RuleGroup</code>.</p> <p>You can only insert <code>REGULAR</code> rules into a rule group.</p> <p>You can have a maximum of ten rules per rule group.</p> <p>To create and configure a <code>RuleGroup</code>, perform the following steps:</p> <ol> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>RuleGroup</code>. See <a>CreateRule</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRuleGroup</a> request.</p> </li> <li> <p>Submit an <code>UpdateRuleGroup</code> request to add <code>Rules</code> to the <code>RuleGroup</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>RuleGroup</code>. See <a>CreateWebACL</a>.</p> </li> </ol> <p>If you want to replace one <code>Rule</code> with another, you delete the existing one and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_rule_group(
        &self,
        input: UpdateRuleGroupRequest,
    ) -> RusotoFuture<UpdateRuleGroupResponse, UpdateRuleGroupError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.UpdateRuleGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateRuleGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateRuleGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Inserts or deletes <a>SizeConstraint</a> objects (filters) in a <a>SizeConstraintSet</a>. For each <code>SizeConstraint</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>SizeConstraintSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to evaluate, such as the length of a query string or the length of the <code>User-Agent</code> header.</p> </li> <li> <p>Whether to perform any transformations on the request, such as converting it to lowercase, before checking its length. Note that transformations of the request body are not supported because the AWS resource forwards only the first <code>8192</code> bytes of your request to AWS WAF.</p> <p>You can only specify a single type of TextTransformation.</p> </li> <li> <p>A <code>ComparisonOperator</code> used for evaluating the selected part of the request against the specified <code>Size</code>, such as equals, greater than, less than, and so on.</p> </li> <li> <p>The length, in bytes, that you want AWS WAF to watch for in selected part of the request. The length is computed after applying the transformation.</p> </li> </ul> <p>For example, you can add a <code>SizeConstraintSetUpdate</code> object that matches web requests in which the length of the <code>User-Agent</code> header is greater than 100 bytes. You can then configure AWS WAF to block those requests.</p> <p>To create and configure a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>SizeConstraintSet.</code> For more information, see <a>CreateSizeConstraintSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateSizeConstraintSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_size_constraint_set(
        &self,
        input: UpdateSizeConstraintSetRequest,
    ) -> RusotoFuture<UpdateSizeConstraintSetResponse, UpdateSizeConstraintSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.UpdateSizeConstraintSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateSizeConstraintSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateSizeConstraintSetError::from_response(response))
                }))
            }
        })
    }

    /// <p>Inserts or deletes <a>SqlInjectionMatchTuple</a> objects (filters) in a <a>SqlInjectionMatchSet</a>. For each <code>SqlInjectionMatchTuple</code> object, you specify the following values:</p> <ul> <li> <p> <code>Action</code>: Whether to insert the object into or delete the object from the array. To change a <code>SqlInjectionMatchTuple</code>, you delete the existing object and add a new one.</p> </li> <li> <p> <code>FieldToMatch</code>: The part of web requests that you want AWS WAF to inspect and, if you want AWS WAF to inspect a header or custom query parameter, the name of the header or parameter.</p> </li> <li> <p> <code>TextTransformation</code>: Which text transformation, if any, to perform on the web request before inspecting the request for snippets of malicious SQL code.</p> <p>You can only specify a single type of TextTransformation.</p> </li> </ul> <p>You use <code>SqlInjectionMatchSet</code> objects to specify which CloudFront requests that you want to allow, block, or count. For example, if you're receiving requests that contain snippets of SQL code in the query string and you want to block the requests, you can create a <code>SqlInjectionMatchSet</code> with the applicable settings, and then configure AWS WAF to block the requests. </p> <p>To create and configure a <code>SqlInjectionMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateSqlInjectionMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateSqlInjectionMatchSet</code> request to specify the parts of web requests that you want AWS WAF to inspect for snippets of SQL code.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_sql_injection_match_set(
        &self,
        input: UpdateSqlInjectionMatchSetRequest,
    ) -> RusotoFuture<UpdateSqlInjectionMatchSetResponse, UpdateSqlInjectionMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_Regional_20161128.UpdateSqlInjectionMatchSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateSqlInjectionMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateSqlInjectionMatchSetError::from_response(response))
                }))
            }
        })
    }

    /// <p>Inserts or deletes <a>ActivatedRule</a> objects in a <code>WebACL</code>. Each <code>Rule</code> identifies web requests that you want to allow, block, or count. When you update a <code>WebACL</code>, you specify the following values:</p> <ul> <li> <p>A default action for the <code>WebACL</code>, either <code>ALLOW</code> or <code>BLOCK</code>. AWS WAF performs the default action if a request doesn't match the criteria in any of the <code>Rules</code> in a <code>WebACL</code>.</p> </li> <li> <p>The <code>Rules</code> that you want to add or delete. If you want to replace one <code>Rule</code> with another, you delete the existing <code>Rule</code> and add the new one.</p> </li> <li> <p>For each <code>Rule</code>, whether you want AWS WAF to allow requests, block requests, or count requests that match the conditions in the <code>Rule</code>.</p> </li> <li> <p>The order in which you want AWS WAF to evaluate the <code>Rules</code> in a <code>WebACL</code>. If you add more than one <code>Rule</code> to a <code>WebACL</code>, AWS WAF evaluates each request against the <code>Rules</code> in order based on the value of <code>Priority</code>. (The <code>Rule</code> that has the lowest value for <code>Priority</code> is evaluated first.) When a web request matches all the predicates (such as <code>ByteMatchSets</code> and <code>IPSets</code>) in a <code>Rule</code>, AWS WAF immediately takes the corresponding action, allow or block, and doesn't evaluate the request against the remaining <code>Rules</code> in the <code>WebACL</code>, if any. </p> </li> </ul> <p>To create and configure a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in <code>Rules</code>. For more information, see <a>CreateByteMatchSet</a>, <a>UpdateByteMatchSet</a>, <a>CreateIPSet</a>, <a>UpdateIPSet</a>, <a>CreateSqlInjectionMatchSet</a>, and <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>WebACL</code>. For more information, see <a>CreateRule</a> and <a>UpdateRule</a>.</p> </li> <li> <p>Create a <code>WebACL</code>. See <a>CreateWebACL</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateWebACL</a> request.</p> </li> <li> <p>Submit an <code>UpdateWebACL</code> request to specify the <code>Rules</code> that you want to include in the <code>WebACL</code>, to specify the default action, and to associate the <code>WebACL</code> with a CloudFront distribution. </p> <p>The <code>ActivatedRule</code> can be a rule group. If you specify a rule group as your <code>ActivatedRule</code>, you can exclude specific rules from that rule group.</p> <p>If you already have a rule group associated with a web ACL and want to submit an <code>UpdateWebACL</code> request to exclude certain rules from that rule group, you must first remove the rule group from the web ACL, the re-insert it again, specifying the excluded rules. For details, see <a>ActivatedRule$ExcludedRules</a>. </p> </li> </ol> <p>Be aware that if you try to add a RATE_BASED rule to a web ACL without setting the rule type when first creating the rule, the <a>UpdateWebACL</a> request will fail because the request tries to add a REGULAR rule (the default rule type) with the specified ID, which does not exist. </p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_web_acl(
        &self,
        input: UpdateWebACLRequest,
    ) -> RusotoFuture<UpdateWebACLResponse, UpdateWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.UpdateWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateWebACLResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateWebACLError::from_response(response))),
                )
            }
        })
    }

    /// <p>Inserts or deletes <a>XssMatchTuple</a> objects (filters) in an <a>XssMatchSet</a>. For each <code>XssMatchTuple</code> object, you specify the following values:</p> <ul> <li> <p> <code>Action</code>: Whether to insert the object into or delete the object from the array. To change an <code>XssMatchTuple</code>, you delete the existing object and add a new one.</p> </li> <li> <p> <code>FieldToMatch</code>: The part of web requests that you want AWS WAF to inspect and, if you want AWS WAF to inspect a header or custom query parameter, the name of the header or parameter.</p> </li> <li> <p> <code>TextTransformation</code>: Which text transformation, if any, to perform on the web request before inspecting the request for cross-site scripting attacks.</p> <p>You can only specify a single type of TextTransformation.</p> </li> </ul> <p>You use <code>XssMatchSet</code> objects to specify which CloudFront requests that you want to allow, block, or count. For example, if you're receiving requests that contain cross-site scripting attacks in the request body and you want to block the requests, you can create an <code>XssMatchSet</code> with the applicable settings, and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>XssMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateXssMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateXssMatchSet</code> request to specify the parts of web requests that you want AWS WAF to inspect for cross-site scripting attacks.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
    fn update_xss_match_set(
        &self,
        input: UpdateXssMatchSetRequest,
    ) -> RusotoFuture<UpdateXssMatchSetResponse, UpdateXssMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.UpdateXssMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateXssMatchSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateXssMatchSetError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
