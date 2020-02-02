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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>All query arguments of a web request. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllQueryArguments {}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Specifies that AWS WAF should allow requests.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowAction {}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A logical rule statement used to combine other rule statements with AND logic. You provide more than one <a>Statement</a> within the <code>AndStatement</code>. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AndStatement {
    /// <p>The statements to combine with AND logic. You can use any statements that can be nested. </p>
    #[serde(rename = "Statements")]
    pub statements: Vec<Statement>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateWebACLRequest {
    /// <p><p>The Amazon Resource Name (ARN) of the resource to associate with the web ACL. </p> <p>The ARN must be in one of the following formats:</p> <ul> <li> <p>For a CloudFront distribution: <code>arn:aws:cloudfront::<i>account-id</i>:distribution/<i>distribution-id</i> </code> </p> </li> <li> <p>For an Application Load Balancer: <code>arn:aws:elasticloadbalancing: <i>region</i>:<i>account-id</i>:loadbalancer/app/<i>load-balancer-name</i> /<i>load-balancer-id</i> </code> </p> </li> <li> <p>For an Amazon API Gateway stage: <code>arn:aws:apigateway:<i>region</i> ::/restapis/<i>api-id</i>/stages/<i>stage-name</i> </code> </p> </li> </ul></p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the Web ACL that you want to associate with the resource.</p>
    #[serde(rename = "WebACLArn")]
    pub web_acl_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateWebACLResponse {}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Specifies that AWS WAF should block requests.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockAction {}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>The body of a web request. This immediately follows the request headers.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A rule statement that defines a string match search for AWS WAF to apply to web requests. The byte match statement provides the bytes to search for, the location in requests that you want AWS WAF to search, and other settings. The bytes to search for are typically a string that corresponds with ASCII characters. In the AWS WAF console and the developer guide, this is refered to as a string match statement.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByteMatchStatement {
    /// <p>The part of a web request that you want AWS WAF to inspect. For more information, see <a>FieldToMatch</a>. </p>
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>The area within the portion of a web request that you want AWS WAF to search for <code>SearchString</code>. Valid values include the following:</p> <p> <b>CONTAINS</b> </p> <p>The specified part of the web request must include the value of <code>SearchString</code>, but the location doesn't matter.</p> <p> <b>CONTAINS_WORD</b> </p> <p>The specified part of the web request must include the value of <code>SearchString</code>, and <code>SearchString</code> must contain only alphanumeric characters or underscore (A-Z, a-z, 0-9, or _). In addition, <code>SearchString</code> must be a word, which means that both of the following are true:</p> <ul> <li> <p> <code>SearchString</code> is at the beginning of the specified part of the web request or is preceded by a character other than an alphanumeric character or underscore (_). Examples include the value of a header and <code>;BadBot</code>.</p> </li> <li> <p> <code>SearchString</code> is at the end of the specified part of the web request or is followed by a character other than an alphanumeric character or underscore (_), for example, <code>BadBot;</code> and <code>-BadBot;</code>.</p> </li> </ul> <p> <b>EXACTLY</b> </p> <p>The value of the specified part of the web request must exactly match the value of <code>SearchString</code>.</p> <p> <b>STARTS_WITH</b> </p> <p>The value of <code>SearchString</code> must appear at the beginning of the specified part of the web request.</p> <p> <b>ENDS_WITH</b> </p> <p>The value of <code>SearchString</code> must appear at the end of the specified part of the web request.</p>
    #[serde(rename = "PositionalConstraint")]
    pub positional_constraint: String,
    /// <p>A string value that you want AWS WAF to search for. AWS WAF searches only in the part of web requests that you designate for inspection in <a>FieldToMatch</a>. The maximum length of the value is 50 bytes.</p> <p>Valid values depend on the areas that you specify for inspection in <code>FieldToMatch</code>:</p> <ul> <li> <p> <code>Method</code>: The HTTP method that you want AWS WAF to search for. This indicates the type of operation specified in the request. </p> </li> <li> <p> <code>UriPath</code>: The value that you want AWS WAF to search for in the URI path, for example, <code>/images/daily-ad.jpg</code>. </p> </li> </ul> <p>If <code>SearchString</code> includes alphabetic characters A-Z and a-z, note that the value is case sensitive.</p> <p> <b>If you're using the AWS WAF API</b> </p> <p>Specify a base64-encoded version of the value. The maximum length of the value before you base64-encode it is 50 bytes.</p> <p>For example, suppose the value of <code>Type</code> is <code>HEADER</code> and the value of <code>Data</code> is <code>User-Agent</code>. If you want to search the <code>User-Agent</code> header for the value <code>BadBot</code>, you base64-encode <code>BadBot</code> using MIME base64-encoding and include the resulting value, <code>QmFkQm90</code>, in the value of <code>SearchString</code>.</p> <p> <b>If you're using the AWS CLI or one of the AWS SDKs</b> </p> <p>The value that you want AWS WAF to search for. The SDK automatically base64 encodes the value.</p>
    #[serde(rename = "SearchString")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub search_string: bytes::Bytes,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. If you specify one or more transformations in a rule statement, AWS WAF performs all transformations on the content identified by <code>FieldToMatch</code>, starting from the lowest priority setting, before inspecting the content for a match.</p>
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CheckCapacityRequest {
    /// <p>An array of <a>Rule</a> that you're configuring to use in a rule group or web ACL. </p>
    #[serde(rename = "Rules")]
    pub rules: Vec<Rule>,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CheckCapacityResponse {
    /// <p>The capacity required by the rules and scope.</p>
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Specifies that AWS WAF should count requests.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountAction {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateIPSetRequest {
    /// <p>Contains an array of strings that specify one or more IP addresses or blocks of IP addresses in Classless Inter-Domain Routing (CIDR) notation. AWS WAF supports all address ranges for IP versions IPv4 and IPv6. </p> <p>Examples: </p> <ul> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from the IP address 192.0.2.44, specify <code>192.0.2.44/32</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify <code>192.0.2.0/24</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify <code>1111:0000:0000:0000:0000:0000:0000:0111/128</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify <code>1111:0000:0000:0000:0000:0000:0000:0000/64</code>.</p> </li> </ul> <p>For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p>
    #[serde(rename = "Addresses")]
    pub addresses: Vec<String>,
    /// <p>A friendly description of the IP set. You cannot change the description of an IP set after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Specify IPV4 or IPV6. </p>
    #[serde(rename = "IPAddressVersion")]
    pub ip_address_version: String,
    /// <p>A friendly name of the IP set. You cannot change the name of an <code>IPSet</code> after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
    /// <p>An array of key:value pairs to associate with the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIPSetResponse {
    /// <p>High-level information about an <a>IPSet</a>, returned by operations like create and list. This provides information like the ID, that you can use to retrieve and manage an <code>IPSet</code>, and the ARN, that you provide to the <a>IPSetReferenceStatement</a> to use the address set in a <a>Rule</a>.</p>
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<IPSetSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRegexPatternSetRequest {
    /// <p>A friendly description of the set. You cannot change the description of a set after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A friendly name of the set. You cannot change the name after you create the set.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Array of regular expression strings. </p>
    #[serde(rename = "RegularExpressionList")]
    pub regular_expression_list: Vec<Regex>,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
    /// <p>An array of key:value pairs to associate with the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRegexPatternSetResponse {
    /// <p>High-level information about a <a>RegexPatternSet</a>, returned by operations like create and list. This provides information like the ID, that you can use to retrieve and manage a <code>RegexPatternSet</code>, and the ARN, that you provide to the <a>RegexPatternSetReferenceStatement</a> to use the pattern set in a <a>Rule</a>.</p>
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<RegexPatternSetSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRuleGroupRequest {
    /// <p>The web ACL capacity units (WCUs) required for this rule group.</p> <p>When you create your own rule group, you define this, and you cannot change it after creation. When you add or modify the rules in a rule group, AWS WAF enforces this limit. You can check the capacity for a set of rules using <a>CheckCapacity</a>.</p> <p>AWS WAF uses WCUs to calculate and control the operating resources that are used to run your rules, rule groups, and web ACLs. AWS WAF calculates capacity differently for each rule type, to reflect the relative cost of each rule. Simple rules that cost little to run use fewer WCUs than more complex rules that use more processing power. Rule group capacity is fixed at creation, which helps users plan their web ACL WCU usage when they use a rule group. The WCU limit for web ACLs is 1,500. </p>
    #[serde(rename = "Capacity")]
    pub capacity: i64,
    /// <p>A friendly description of the rule group. You cannot change the description of a rule group after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A friendly name of the rule group. You cannot change the name of a rule group after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The <a>Rule</a> statements used to identify the web requests that you want to allow, block, or count. Each rule includes one top-level statement that AWS WAF uses to identify matching web requests, and parameters that govern how AWS WAF handles them. </p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
    /// <p>An array of key:value pairs to associate with the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Defines and enables Amazon CloudWatch metrics and web request sample collection. </p>
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: VisibilityConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRuleGroupResponse {
    /// <p>High-level information about a <a>RuleGroup</a>, returned by operations like create and list. This provides information like the ID, that you can use to retrieve and manage a <code>RuleGroup</code>, and the ARN, that you provide to the <a>RuleGroupReferenceStatement</a> to use the rule group in a <a>Rule</a>.</p>
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<RuleGroupSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateWebACLRequest {
    /// <p>The action to perform if none of the <code>Rules</code> contained in the <code>WebACL</code> match. </p>
    #[serde(rename = "DefaultAction")]
    pub default_action: DefaultAction,
    /// <p>A friendly description of the Web ACL. You cannot change the description of a Web ACL after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A friendly name of the Web ACL. You cannot change the name of a Web ACL after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The <a>Rule</a> statements used to identify the web requests that you want to allow, block, or count. Each rule includes one top-level statement that AWS WAF uses to identify matching web requests, and parameters that govern how AWS WAF handles them. </p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
    /// <p>An array of key:value pairs to associate with the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Defines and enables Amazon CloudWatch metrics and web request sample collection. </p>
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: VisibilityConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateWebACLResponse {
    /// <p>High-level information about a <a>WebACL</a>, returned by operations like create and list. This provides information like the ID, that you can use to retrieve and manage a <code>WebACL</code>, and the ARN, that you provide to operations like <a>AssociateWebACL</a>.</p>
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<WebACLSummary>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>In a <code>WebACL</code>, this is the action that you want AWS WAF to perform when a web request doesn&#39;t match any of the rules in the <code>WebACL</code>. The default action must be a terminating action, so count is not allowed.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefaultAction {
    /// <p>Specifies that AWS WAF should allow requests by default.</p>
    #[serde(rename = "Allow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<AllowAction>,
    /// <p>Specifies that AWS WAF should block requests by default. </p>
    #[serde(rename = "Block")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<BlockAction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIPSetRequest {
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    pub lock_token: String,
    /// <p>A friendly name of the IP set. You cannot change the name of an <code>IPSet</code> after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteIPSetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLoggingConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the web ACL from which you want to delete the <a>LoggingConfiguration</a>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLoggingConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRegexPatternSetRequest {
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    pub lock_token: String,
    /// <p>A friendly name of the set. You cannot change the name after you create the set.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRegexPatternSetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRuleGroupRequest {
    /// <p>A unique identifier for the rule group. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    pub lock_token: String,
    /// <p>A friendly name of the rule group. You cannot change the name of a rule group after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRuleGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteWebACLRequest {
    /// <p>The unique identifier for the Web ACL. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    pub lock_token: String,
    /// <p>A friendly name of the Web ACL. You cannot change the name of a Web ACL after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteWebACLResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeManagedRuleGroupRequest {
    /// <p>The name of the managed rule group. You use this, along with the vendor name, to identify the rule group.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
    /// <p>The name of the managed rule group vendor. You use this, along with the rule group name, to identify the rule group.</p>
    #[serde(rename = "VendorName")]
    pub vendor_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeManagedRuleGroupResponse {
    /// <p>The web ACL capacity units (WCUs) required for this rule group. AWS WAF uses web ACL capacity units (WCU) to calculate and control the operating resources that are used to run your rules, rule groups, and web ACLs. AWS WAF calculates capacity differently for each rule type, to reflect each rule's relative cost. Rule group capacity is fixed at creation, so users can plan their web ACL WCU usage when they use a rule group. The WCU limit for web ACLs is 1,500. </p>
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RuleSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateWebACLRequest {
    /// <p><p>The Amazon Resource Name (ARN) of the resource to disassociate from the web ACL. </p> <p>The ARN must be in one of the following formats:</p> <ul> <li> <p>For a CloudFront distribution: <code>arn:aws:cloudfront::<i>account-id</i>:distribution/<i>distribution-id</i> </code> </p> </li> <li> <p>For an Application Load Balancer: <code>arn:aws:elasticloadbalancing: <i>region</i>:<i>account-id</i>:loadbalancer/app/<i>load-balancer-name</i> /<i>load-balancer-id</i> </code> </p> </li> <li> <p>For an Amazon API Gateway stage: <code>arn:aws:apigateway:<i>region</i> ::/restapis/<i>api-id</i>/stages/<i>stage-name</i> </code> </p> </li> </ul></p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateWebACLResponse {}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Specifies a single rule to exclude from the rule group. Excluding a rule overrides its action setting for the rule group in the web ACL, setting it to <code>COUNT</code>. This effectively excludes the rule from acting on web requests. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExcludedRule {
    /// <p>The name of the rule to exclude.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>The part of a web request that you want AWS WAF to inspect. Include the <code>FieldToMatch</code> types that you want to inspect, with additional specifications as needed, according to the type. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldToMatch {
    /// <p>Inspect all query arguments. </p>
    #[serde(rename = "AllQueryArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_query_arguments: Option<AllQueryArguments>,
    /// <p>Inspect the request body, which immediately follows the request headers. This is the part of a request that contains any additional data that you want to send to your web server as the HTTP request body, such as data from a form. </p> <p>Note that only the first 8 KB (8192 bytes) of the request body are forwarded to AWS WAF for inspection. If you don't need to inspect more than 8 KB, you can guarantee that you don't allow additional bytes in by combining a statement that inspects the body of the web request, such as <a>ByteMatchStatement</a> or <a>RegexPatternSetReferenceStatement</a>, with a <a>SizeConstraintStatement</a> that enforces an 8 KB size limit on the body of the request. AWS WAF doesn't support inspecting the entire contents of web requests whose bodies exceed the 8 KB limit.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,
    /// <p>Inspect the HTTP method. The method indicates the type of operation that the request is asking the origin to perform. </p>
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<Method>,
    /// <p>Inspect the query string. This is the part of a URL that appears after a <code>?</code> character, if any.</p>
    #[serde(rename = "QueryString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<QueryString>,
    /// <p>Inspect a single header. Provide the name of the header to inspect, for example, <code>User-Agent</code> or <code>Referer</code>. This setting isn't case sensitive.</p>
    #[serde(rename = "SingleHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_header: Option<SingleHeader>,
    /// <p>Inspect a single query argument. Provide the name of the query argument to inspect, such as <i>UserName</i> or <i>SalesRegion</i>. The name can be up to 30 characters long and isn't case sensitive. </p>
    #[serde(rename = "SingleQueryArgument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_query_argument: Option<SingleQueryArgument>,
    /// <p>Inspect the request URI path. This is the part of a web request that identifies a resource, for example, <code>/images/daily-ad.jpg</code>.</p>
    #[serde(rename = "UriPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri_path: Option<UriPath>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A rule statement used to identify web requests based on country of origin. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoMatchStatement {
    /// <p>An array of two-character country codes, for example, <code>[ "US", "CN" ]</code>, from the alpha-2 country ISO codes of the ISO 3166 international standard. </p>
    #[serde(rename = "CountryCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIPSetRequest {
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A friendly name of the IP set. You cannot change the name of an <code>IPSet</code> after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetIPSetResponse {
    /// <p><p/></p>
    #[serde(rename = "IPSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_set: Option<IPSet>,
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLoggingConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the web ACL for which you want to get the <a>LoggingConfiguration</a>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLoggingConfigurationResponse {
    /// <p>The <a>LoggingConfiguration</a> for the specified web ACL.</p>
    #[serde(rename = "LoggingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRateBasedStatementManagedKeysRequest {
    /// <p>The name of the rate-based rule to get the keys for.</p>
    #[serde(rename = "RuleName")]
    pub rule_name: String,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
    /// <p>The unique identifier for the Web ACL. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "WebACLId")]
    pub web_acl_id: String,
    /// <p>A friendly name of the Web ACL. You cannot change the name of a Web ACL after you create it.</p>
    #[serde(rename = "WebACLName")]
    pub web_acl_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRateBasedStatementManagedKeysResponse {
    /// <p>The keys that are of Internet Protocol version 4 (IPv4). </p>
    #[serde(rename = "ManagedKeysIPV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_keys_ipv4: Option<RateBasedStatementManagedKeysIPSet>,
    /// <p>The keys that are of Internet Protocol version 6 (IPv6). </p>
    #[serde(rename = "ManagedKeysIPV6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_keys_ipv6: Option<RateBasedStatementManagedKeysIPSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRegexPatternSetRequest {
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A friendly name of the set. You cannot change the name after you create the set.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRegexPatternSetResponse {
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "RegexPatternSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_pattern_set: Option<RegexPatternSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRuleGroupRequest {
    /// <p>A unique identifier for the rule group. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A friendly name of the rule group. You cannot change the name of a rule group after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRuleGroupResponse {
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "RuleGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group: Option<RuleGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSampledRequestsRequest {
    /// <p>The number of requests that you want AWS WAF to return from among the first 5,000 requests that your AWS resource received during the time range. If your resource received fewer requests than the value of <code>MaxItems</code>, <code>GetSampledRequests</code> returns information about all of them. </p>
    #[serde(rename = "MaxItems")]
    pub max_items: i64,
    /// <p>The metric name assigned to the <code>Rule</code> or <code>RuleGroup</code> for which you want a sample of requests.</p>
    #[serde(rename = "RuleMetricName")]
    pub rule_metric_name: String,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
    /// <p>The start date and time and the end date and time of the range for which you want <code>GetSampledRequests</code> to return a sample of requests. Specify the date and time in the following format: <code>"2016-09-27T14:50Z"</code>. You can specify any time range in the previous three hours.</p>
    #[serde(rename = "TimeWindow")]
    pub time_window: TimeWindow,
    /// <p>The Amazon resource name (ARN) of the <code>WebACL</code> for which you want a sample of requests.</p>
    #[serde(rename = "WebAclArn")]
    pub web_acl_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetWebACLForResourceRequest {
    /// <p>The ARN (Amazon Resource Name) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetWebACLForResourceResponse {
    /// <p>The Web ACL that is associated with the resource. If there is no associated resource, AWS WAF returns a null Web ACL.</p>
    #[serde(rename = "WebACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl: Option<WebACL>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetWebACLRequest {
    /// <p>The unique identifier for the Web ACL. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A friendly name of the Web ACL. You cannot change the name of a Web ACL after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetWebACLResponse {
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    /// <p>The Web ACL specification. You can modify the settings in this Web ACL and use it to update this Web ACL or create a new one.</p>
    #[serde(rename = "WebACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl: Option<WebACL>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Part of the response from <a>GetSampledRequests</a>. This is a complex type that appears as <code>Headers</code> in the response syntax. <code>HTTPHeader</code> contains the names and values of all of the headers that appear in one of the web requests. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HTTPHeader {
    /// <p>The name of the HTTP header.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the HTTP header.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Part of the response from <a>GetSampledRequests</a>. This is a complex type that appears as <code>Request</code> in the response syntax. <code>HTTPRequest</code> contains information about one of the web requests. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HTTPRequest {
    /// <p><p>The IP address that the request originated from. If the web ACL is associated with a CloudFront distribution, this is the value of one of the following fields in CloudFront access logs:</p> <ul> <li> <p> <code>c-ip</code>, if the viewer did not use an HTTP proxy or a load balancer to send the request</p> </li> <li> <p> <code>x-forwarded-for</code>, if the viewer did use an HTTP proxy or a load balancer to send the request</p> </li> </ul></p>
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
    /// <p>A complex type that contains the name and value for each header in the sampled web request.</p>
    #[serde(rename = "Headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HTTPHeader>>,
    /// <p>The HTTP method specified in the sampled web request. </p>
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// <p>The URI path of the request, which identifies the resource, for example, <code>/images/daily-ad.jpg</code>.</p>
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Contains one or more IP addresses or blocks of IP addresses specified in Classless Inter-Domain Routing (CIDR) notation. AWS WAF supports any CIDR range. For information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>. </p> <p>AWS WAF assigns an ARN to each <code>IPSet</code> that you create. To use an IP set in a rule, you provide the ARN to the <a>Rule</a> statement <a>IPSetReferenceStatement</a>. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IPSet {
    /// <p>The Amazon Resource Name (ARN) of the entity.</p>
    #[serde(rename = "ARN")]
    pub arn: String,
    /// <p>Contains an array of strings that specify one or more IP addresses or blocks of IP addresses in Classless Inter-Domain Routing (CIDR) notation. AWS WAF supports all address ranges for IP versions IPv4 and IPv6. </p> <p>Examples: </p> <ul> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from the IP address 192.0.2.44, specify <code>192.0.2.44/32</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify <code>192.0.2.0/24</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify <code>1111:0000:0000:0000:0000:0000:0000:0111/128</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify <code>1111:0000:0000:0000:0000:0000:0000:0000/64</code>.</p> </li> </ul> <p>For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p>
    #[serde(rename = "Addresses")]
    pub addresses: Vec<String>,
    /// <p>A friendly description of the IP set. You cannot change the description of an IP set after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Specify IPV4 or IPV6. </p>
    #[serde(rename = "IPAddressVersion")]
    pub ip_address_version: String,
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A friendly name of the IP set. You cannot change the name of an <code>IPSet</code> after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A rule statement used to detect web requests coming from particular IP addresses or address ranges. To use this, create an <a>IPSet</a> that specifies the addresses you want to detect, then use the ARN of that set in this statement. To create an IP set, see <a>CreateIPSet</a>.</p> <p>Each IP set rule statement references an IP set. You create and maintain the set independent of your rules. This allows you to use the single set in multiple rules. When you update the referenced set, AWS WAF automatically updates all rules that reference it.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IPSetReferenceStatement {
    /// <p>The Amazon Resource Name (ARN) of the <a>IPSet</a> that this statement references.</p>
    #[serde(rename = "ARN")]
    pub arn: String,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>High-level information about an <a>IPSet</a>, returned by operations like create and list. This provides information like the ID, that you can use to retrieve and manage an <code>IPSet</code>, and the ARN, that you provide to the <a>IPSetReferenceStatement</a> to use the address set in a <a>Rule</a>.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IPSetSummary {
    /// <p>The Amazon Resource Name (ARN) of the entity.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A friendly description of the IP set. You cannot change the description of an IP set after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    /// <p>A friendly name of the IP set. You cannot change the name of an <code>IPSet</code> after you create it.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAvailableManagedRuleGroupsRequest {
    /// <p>The maximum number of objects that you want AWS WAF to return for this request. If more objects are available, in the response, AWS WAF provides a <code>NextMarker</code> value that you can use in a subsequent call to get the next batch of objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAvailableManagedRuleGroupsResponse {
    /// <p><p/></p>
    #[serde(rename = "ManagedRuleGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_rule_groups: Option<Vec<ManagedRuleGroupSummary>>,
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListIPSetsRequest {
    /// <p>The maximum number of objects that you want AWS WAF to return for this request. If more objects are available, in the response, AWS WAF provides a <code>NextMarker</code> value that you can use in a subsequent call to get the next batch of objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListIPSetsResponse {
    /// <p>Array of IPSets. This may not be the full list of IPSets that you have defined. See the <code>Limit</code> specification for this request.</p>
    #[serde(rename = "IPSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_sets: Option<Vec<IPSetSummary>>,
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLoggingConfigurationsRequest {
    /// <p>The maximum number of objects that you want AWS WAF to return for this request. If more objects are available, in the response, AWS WAF provides a <code>NextMarker</code> value that you can use in a subsequent call to get the next batch of objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLoggingConfigurationsResponse {
    /// <p><p/></p>
    #[serde(rename = "LoggingConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configurations: Option<Vec<LoggingConfiguration>>,
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRegexPatternSetsRequest {
    /// <p>The maximum number of objects that you want AWS WAF to return for this request. If more objects are available, in the response, AWS WAF provides a <code>NextMarker</code> value that you can use in a subsequent call to get the next batch of objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRegexPatternSetsResponse {
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "RegexPatternSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_pattern_sets: Option<Vec<RegexPatternSetSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResourcesForWebACLRequest {
    /// <p>Used for web ACLs that are scoped for regional applications. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Web ACL.</p>
    #[serde(rename = "WebACLArn")]
    pub web_acl_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourcesForWebACLResponse {
    /// <p>The array of Amazon Resource Names (ARNs) of the associated resources.</p>
    #[serde(rename = "ResourceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRuleGroupsRequest {
    /// <p>The maximum number of objects that you want AWS WAF to return for this request. If more objects are available, in the response, AWS WAF provides a <code>NextMarker</code> value that you can use in a subsequent call to get the next batch of objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRuleGroupsResponse {
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "RuleGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_groups: Option<Vec<RuleGroupSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The maximum number of objects that you want AWS WAF to return for this request. If more objects are available, in the response, AWS WAF provides a <code>NextMarker</code> value that you can use in a subsequent call to get the next batch of objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>The collection of tagging definitions for the resource. </p>
    #[serde(rename = "TagInfoForResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_info_for_resource: Option<TagInfoForResource>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListWebACLsRequest {
    /// <p>The maximum number of objects that you want AWS WAF to return for this request. If more objects are available, in the response, AWS WAF provides a <code>NextMarker</code> value that you can use in a subsequent call to get the next batch of objects.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListWebACLsResponse {
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, AWS WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "WebACLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ac_ls: Option<Vec<WebACLSummary>>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Defines an association between Amazon Kinesis Data Firehose destinations and a web ACL resource, for logging from AWS WAF. As part of the association, you can specify parts of the standard logging fields to keep out of the logs. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingConfiguration {
    /// <p>The Amazon Kinesis Data Firehose Amazon Resource Name (ARNs) that you want to associate with the web ACL.</p>
    #[serde(rename = "LogDestinationConfigs")]
    pub log_destination_configs: Vec<String>,
    /// <p>The parts of the request that you want to keep out of the logs. For example, if you redact the cookie field, the cookie field in the firehose will be <code>xxx</code>. </p>
    #[serde(rename = "RedactedFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted_fields: Option<Vec<FieldToMatch>>,
    /// <p>The Amazon Resource Name (ARN) of the web ACL that you want to associate with <code>LogDestinationConfigs</code>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A rule statement used to run the rules that are defined in a managed rule group. To use this, provide the vendor name and the name of the rule group in this statement. You can retrieve the required names by calling <a>ListAvailableManagedRuleGroups</a>.</p> <p>You can&#39;t nest a <code>ManagedRuleGroupStatement</code>, for example for use inside a <code>NotStatement</code> or <code>OrStatement</code>. It can only be referenced as a top-level statement within a rule.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManagedRuleGroupStatement {
    /// <p>The rules whose actions are set to <code>COUNT</code> by the web ACL, regardless of the action that is set on the rule. This effectively excludes the rule from acting on web requests. </p>
    #[serde(rename = "ExcludedRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_rules: Option<Vec<ExcludedRule>>,
    /// <p>The name of the managed rule group. You use this, along with the vendor name, to identify the rule group.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The name of the managed rule group vendor. You use this, along with the rule group name, to identify the rule group.</p>
    #[serde(rename = "VendorName")]
    pub vendor_name: String,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>High-level information about a managed rule group, returned by <a>ListAvailableManagedRuleGroups</a>. This provides information like the name and vendor name, that you provide when you add a <a>ManagedRuleGroupStatement</a> to a web ACL. Managed rule groups include AWS managed rule groups, which are free of charge to AWS WAF customers, and AWS Marketplace managed rule groups, which you can subscribe to through AWS Marketplace. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ManagedRuleGroupSummary {
    /// <p>The description of the managed rule group, provided by AWS or the AWS Marketplace seller who manages it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the managed rule group. You use this, along with the vendor name, to identify the rule group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The name of the managed rule group vendor. You use this, along with the rule group name, to identify the rule group.</p>
    #[serde(rename = "VendorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>The HTTP method of a web request. The method indicates the type of operation that the request is asking the origin to perform. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Method {}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Specifies that AWS WAF should do nothing. This is generally used to try out a rule without performing any actions. You set the <code>OverrideAction</code> on the <a>Rule</a>, and override the actions that are set at the statement level. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoneAction {}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A logical rule statement used to negate the results of another rule statement. You provide one <a>Statement</a> within the <code>NotStatement</code>.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotStatement {
    /// <p>The statement to negate. You can use any statement that can be nested.</p>
    #[serde(rename = "Statement")]
    pub statement: Statement,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A logical rule statement used to combine other rule statements with OR logic. You provide more than one <a>Statement</a> within the <code>OrStatement</code>. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrStatement {
    /// <p>The statements to combine with OR logic. You can use any statements that can be nested.</p>
    #[serde(rename = "Statements")]
    pub statements: Vec<Statement>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>The action to use to override the rule&#39;s <code>Action</code> setting. You can use no override action, in which case the rule action is in effect, or count, in which case, if the rule matches a web request, it only counts the match.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverrideAction {
    /// <p>Override the rule action setting to count.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<CountAction>,
    /// <p>Don't override the rule action setting.</p>
    #[serde(rename = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub none: Option<NoneAction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutLoggingConfigurationRequest {
    /// <p><p/></p>
    #[serde(rename = "LoggingConfiguration")]
    pub logging_configuration: LoggingConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutLoggingConfigurationResponse {
    /// <p><p/></p>
    #[serde(rename = "LoggingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>The query string of a web request. This is the part of a URL that appears after a <code>?</code> character, if any.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryString {}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A rate-based rule tracks the rate of requests for each originating IP address, and triggers the rule action when the rate exceeds a limit that you specify on the number of requests in any 5-minute time span. You can use this to put a temporary block on requests from an IP address that is sending excessive requests.</p> <p>When the rule action triggers, AWS WAF blocks additional requests from the IP address until the request rate falls below the limit.</p> <p>You can optionally nest another statement inside the rate-based statement, to narrow the scope of the rule so that it only counts requests that match the nested statement. For example, based on recent requests that you have seen from an attacker, you might create a rate-based rule with a nested AND rule statement that contains the following nested statements:</p> <ul> <li> <p>An IP match statement with an IP set that specified the address 192.0.2.44.</p> </li> <li> <p>A string match statement that searches in the User-Agent header for the string BadBot.</p> </li> </ul> <p>In this rate-based rule, you also define a rate limit. For this example, the rate limit is 1,000. Requests that meet both of the conditions in the statements are counted. If the count exceeds 1,000 requests per five minutes, the rule action triggers. Requests that do not meet both conditions are not counted towards the rate limit and are not affected by this rule.</p> <p>You cannot nest a <code>RateBasedStatement</code>, for example for use inside a <code>NotStatement</code> or <code>OrStatement</code>. It can only be referenced as a top-level statement within a rule.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateBasedStatement {
    /// <p>Setting that indicates how to aggregate the request counts. Currently, you must set this to <code>IP</code>. The request counts are aggregated on IP addresses. </p>
    #[serde(rename = "AggregateKeyType")]
    pub aggregate_key_type: String,
    /// <p>The limit on requests per 5-minute period for a single originating IP address. If the statement includes a <code>ScopDownStatement</code>, this limit is applied only to the requests that match the statement.</p>
    #[serde(rename = "Limit")]
    pub limit: i64,
    /// <p>An optional nested statement that narrows the scope of the rate-based statement to matching web requests. This can be any nestable statement, and you can nest statements at any level below this scope-down statement.</p>
    #[serde(rename = "ScopeDownStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_down_statement: Option<Statement>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>The set of IP addresses that are currently blocked for a rate-based statement.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RateBasedStatementManagedKeysIPSet {
    /// <p>The IP addresses that are currently blocked.</p>
    #[serde(rename = "Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    #[serde(rename = "IPAddressVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_version: Option<String>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A single regular expression. This is used in a <a>RegexPatternSet</a>.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Regex {
    /// <p>The string representing the regular expression.</p>
    #[serde(rename = "RegexString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_string: Option<String>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Contains one or more regular expressions. </p> <p>AWS WAF assigns an ARN to each <code>RegexPatternSet</code> that you create. To use a set in a rule, you provide the ARN to the <a>Rule</a> statement <a>RegexPatternSetReferenceStatement</a>. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegexPatternSet {
    /// <p>The Amazon Resource Name (ARN) of the entity.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A friendly description of the set. You cannot change the description of a set after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A friendly name of the set. You cannot change the name after you create the set.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The regular expression patterns in the set.</p>
    #[serde(rename = "RegularExpressionList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_expression_list: Option<Vec<Regex>>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A rule statement used to search web request components for matches with regular expressions. To use this, create a <a>RegexPatternSet</a> that specifies the expressions that you want to detect, then use the ARN of that set in this statement. A web request matches the pattern set rule statement if the request component matches any of the patterns in the set. To create a regex pattern set, see <a>CreateRegexPatternSet</a>.</p> <p>Each regex pattern set rule statement references a regex pattern set. You create and maintain the set independent of your rules. This allows you to use the single set in multiple rules. When you update the referenced set, AWS WAF automatically updates all rules that reference it.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegexPatternSetReferenceStatement {
    /// <p>The Amazon Resource Name (ARN) of the <a>RegexPatternSet</a> that this statement references.</p>
    #[serde(rename = "ARN")]
    pub arn: String,
    /// <p>The part of a web request that you want AWS WAF to inspect. For more information, see <a>FieldToMatch</a>. </p>
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. If you specify one or more transformations in a rule statement, AWS WAF performs all transformations on the content identified by <code>FieldToMatch</code>, starting from the lowest priority setting, before inspecting the content for a match.</p>
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>High-level information about a <a>RegexPatternSet</a>, returned by operations like create and list. This provides information like the ID, that you can use to retrieve and manage a <code>RegexPatternSet</code>, and the ARN, that you provide to the <a>RegexPatternSetReferenceStatement</a> to use the pattern set in a <a>Rule</a>.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegexPatternSetSummary {
    /// <p>The Amazon Resource Name (ARN) of the entity.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A friendly description of the set. You cannot change the description of a set after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    /// <p>A friendly name of the data type instance. You cannot change the name after you create the instance.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A single rule, which you can use in a <a>WebACL</a> or <a>RuleGroup</a> to identify web requests that you want to allow, block, or count. Each rule includes one top-level <a>Statement</a> that AWS WAF uses to identify matching web requests, and parameters that govern how AWS WAF handles them. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    /// <p>The action that AWS WAF should take on a web request when it matches the rule's statement. Settings at the web ACL level can override the rule action setting. </p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    /// <p>A friendly name of the rule. You can't change the name of a <code>Rule</code> after you create it. </p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The action to use to override the rule's <code>Action</code> setting. You can use no override action, in which case the rule action is in effect, or count action, in which case, if the rule matches a web request, it only counts the match.</p>
    #[serde(rename = "OverrideAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_action: Option<OverrideAction>,
    /// <p>If you define more than one <code>Rule</code> in a <code>WebACL</code>, AWS WAF evaluates each request against the <code>Rules</code> in order based on the value of <code>Priority</code>. AWS WAF processes rules with lower priority first. The priorities don't need to be consecutive, but they must all be different.</p>
    #[serde(rename = "Priority")]
    pub priority: i64,
    /// <p>The AWS WAF processing statement for the rule, for example <a>ByteMatchStatement</a> or <a>SizeConstraintStatement</a>. </p>
    #[serde(rename = "Statement")]
    pub statement: Statement,
    /// <p>Defines and enables Amazon CloudWatch metrics and web request sample collection. </p>
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: VisibilityConfig,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>The action that AWS WAF should take on a web request when it matches a rule&#39;s statement. Settings at the web ACL level can override the rule action setting. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleAction {
    /// <p>Instructs AWS WAF to allow the web request.</p>
    #[serde(rename = "Allow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<AllowAction>,
    /// <p>Instructs AWS WAF to block the web request.</p>
    #[serde(rename = "Block")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<BlockAction>,
    /// <p>Instructs AWS WAF to count the web request and allow it.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<CountAction>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p> A rule group defines a collection of rules to inspect and control web requests that you can use in a <a>WebACL</a>. When you create a rule group, you define an immutable capacity limit. If you update a rule group, you must stay within the capacity. This allows others to reuse the rule group with confidence in its capacity requirements. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RuleGroup {
    /// <p>The Amazon Resource Name (ARN) of the entity.</p>
    #[serde(rename = "ARN")]
    pub arn: String,
    /// <p>The web ACL capacity units (WCUs) required for this rule group.</p> <p>When you create your own rule group, you define this, and you cannot change it after creation. When you add or modify the rules in a rule group, AWS WAF enforces this limit. You can check the capacity for a set of rules using <a>CheckCapacity</a>.</p> <p>AWS WAF uses WCUs to calculate and control the operating resources that are used to run your rules, rule groups, and web ACLs. AWS WAF calculates capacity differently for each rule type, to reflect the relative cost of each rule. Simple rules that cost little to run use fewer WCUs than more complex rules that use more processing power. Rule group capacity is fixed at creation, which helps users plan their web ACL WCU usage when they use a rule group. The WCU limit for web ACLs is 1,500. </p>
    #[serde(rename = "Capacity")]
    pub capacity: i64,
    /// <p>A friendly description of the rule group. You cannot change the description of a rule group after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier for the rule group. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A friendly name of the rule group. You cannot change the name of a rule group after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The <a>Rule</a> statements used to identify the web requests that you want to allow, block, or count. Each rule includes one top-level statement that AWS WAF uses to identify matching web requests, and parameters that govern how AWS WAF handles them. </p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    /// <p>Defines and enables Amazon CloudWatch metrics and web request sample collection. </p>
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: VisibilityConfig,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A rule statement used to run the rules that are defined in a <a>RuleGroup</a>. To use this, create a rule group with your rules, then provide the ARN of the rule group in this statement.</p> <p>You cannot nest a <code>RuleGroupReferenceStatement</code>, for example for use inside a <code>NotStatement</code> or <code>OrStatement</code>. It can only be referenced as a top-level statement within a rule.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleGroupReferenceStatement {
    /// <p>The Amazon Resource Name (ARN) of the entity.</p>
    #[serde(rename = "ARN")]
    pub arn: String,
    /// <p>The names of rules that are in the referenced rule group, but that you want AWS WAF to exclude from processing for this rule statement. </p>
    #[serde(rename = "ExcludedRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_rules: Option<Vec<ExcludedRule>>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>High-level information about a <a>RuleGroup</a>, returned by operations like create and list. This provides information like the ID, that you can use to retrieve and manage a <code>RuleGroup</code>, and the ARN, that you provide to the <a>RuleGroupReferenceStatement</a> to use the rule group in a <a>Rule</a>.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RuleGroupSummary {
    /// <p>The Amazon Resource Name (ARN) of the entity.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A friendly description of the rule group. You cannot change the description of a rule group after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier for the rule group. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    /// <p>A friendly name of the data type instance. You cannot change the name after you create the instance.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>High-level information about a <a>Rule</a>, returned by operations like <a>DescribeManagedRuleGroup</a>. This provides information like the ID, that you can use to retrieve and manage a <code>RuleGroup</code>, and the ARN, that you provide to the <a>RuleGroupReferenceStatement</a> to use the rule group in a <a>Rule</a>.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RuleSummary {
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    /// <p>The name of the rule. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Represents a single sampled web request. The response from <a>GetSampledRequests</a> includes a <code>SampledHTTPRequests</code> complex type that appears as <code>SampledRequests</code> in the response syntax. <code>SampledHTTPRequests</code> contains an array of <code>SampledHTTPRequest</code> objects.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SampledHTTPRequest {
    /// <p>The action for the <code>Rule</code> that the request matched: <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>A complex type that contains detailed information about the request.</p>
    #[serde(rename = "Request")]
    pub request: HTTPRequest,
    /// <p>The name of the <code>Rule</code> that the request matched. For managed rule groups, the format for this name is <code>&lt;vendor name&gt;#&lt;managed rule group name&gt;#&lt;rule name&gt;</code>. For your own rule groups, the format for this name is <code>&lt;rule group name&gt;#&lt;rule name&gt;</code>. If the rule is not in a rule group, the format is <code>&lt;rule name&gt;</code>. </p>
    #[serde(rename = "RuleNameWithinRuleGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name_within_rule_group: Option<String>,
    /// <p>The time at which AWS WAF received the request from your AWS resource, in Unix time format (in seconds).</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    /// <p>A value that indicates how one result in the response relates proportionally to other results in the response. For example, a result that has a weight of <code>2</code> represents roughly twice as many web requests as a result that has a weight of <code>1</code>.</p>
    #[serde(rename = "Weight")]
    pub weight: i64,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>One of the headers in a web request, identified by name, for example, <code>User-Agent</code> or <code>Referer</code>. This setting isn&#39;t case sensitive.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleHeader {
    /// <p>The name of the query header to inspect.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>One query argument in a web request, identified by name, for example <i>UserName</i> or <i>SalesRegion</i>. The name can be up to 30 characters long and isn&#39;t case sensitive. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleQueryArgument {
    /// <p>The name of the query argument to inspect.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A rule statement that compares a number of bytes against the size of a request component, using a comparison operator, such as greater than (&gt;) or less than (&lt;). For example, you can use a size constraint statement to look for query strings that are longer than 100 bytes. </p> <p>If you configure AWS WAF to inspect the request body, AWS WAF inspects only the first 8192 bytes (8 KB). If the request body for your web requests never exceeds 8192 bytes, you can create a size constraint condition and block requests that have a request body greater than 8192 bytes.</p> <p>If you choose URI for the value of Part of the request to filter on, the slash (/) in the URI counts as one character. For example, the URI <code>/logo.jpg</code> is nine characters long.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeConstraintStatement {
    /// <p>The operator to use to compare the request part to the size setting. </p>
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    /// <p>The part of a web request that you want AWS WAF to inspect. For more information, see <a>FieldToMatch</a>. </p>
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>The size, in byte, to compare to the request part, after any transformations.</p>
    #[serde(rename = "Size")]
    pub size: i64,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. If you specify one or more transformations in a rule statement, AWS WAF performs all transformations on the content identified by <code>FieldToMatch</code>, starting from the lowest priority setting, before inspecting the content for a match.</p>
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Attackers sometimes insert malicious SQL code into web requests in an effort to extract data from your database. To allow or block web requests that appear to contain malicious SQL code, create one or more SQL injection match conditions. An SQL injection match condition identifies the part of web requests, such as the URI or the query string, that you want AWS WAF to inspect. Later in the process, when you create a web ACL, you specify whether to allow or block requests that appear to contain malicious SQL code.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SqliMatchStatement {
    /// <p>The part of a web request that you want AWS WAF to inspect. For more information, see <a>FieldToMatch</a>. </p>
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. If you specify one or more transformations in a rule statement, AWS WAF performs all transformations on the content identified by <code>FieldToMatch</code>, starting from the lowest priority setting, before inspecting the content for a match.</p>
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>The processing guidance for a <a>Rule</a>, used by AWS WAF to determine whether a web request matches the rule. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statement {
    /// <p>A logical rule statement used to combine other rule statements with AND logic. You provide more than one <a>Statement</a> within the <code>AndStatement</code>. </p>
    #[serde(rename = "AndStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_statement: Option<AndStatement>,
    /// <p>A rule statement that defines a string match search for AWS WAF to apply to web requests. The byte match statement provides the bytes to search for, the location in requests that you want AWS WAF to search, and other settings. The bytes to search for are typically a string that corresponds with ASCII characters. In the AWS WAF console and the developer guide, this is refered to as a string match statement.</p>
    #[serde(rename = "ByteMatchStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_match_statement: Option<ByteMatchStatement>,
    /// <p>A rule statement used to identify web requests based on country of origin. </p>
    #[serde(rename = "GeoMatchStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_match_statement: Option<GeoMatchStatement>,
    /// <p>A rule statement used to detect web requests coming from particular IP addresses or address ranges. To use this, create an <a>IPSet</a> that specifies the addresses you want to detect, then use the ARN of that set in this statement. To create an IP set, see <a>CreateIPSet</a>.</p> <p>Each IP set rule statement references an IP set. You create and maintain the set independent of your rules. This allows you to use the single set in multiple rules. When you update the referenced set, AWS WAF automatically updates all rules that reference it.</p>
    #[serde(rename = "IPSetReferenceStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_set_reference_statement: Option<IPSetReferenceStatement>,
    /// <p>A rule statement used to run the rules that are defined in a managed rule group. To use this, provide the vendor name and the name of the rule group in this statement. You can retrieve the required names by calling <a>ListAvailableManagedRuleGroups</a>.</p> <p>You can't nest a <code>ManagedRuleGroupStatement</code>, for example for use inside a <code>NotStatement</code> or <code>OrStatement</code>. It can only be referenced as a top-level statement within a rule.</p>
    #[serde(rename = "ManagedRuleGroupStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_rule_group_statement: Option<ManagedRuleGroupStatement>,
    /// <p>A logical rule statement used to negate the results of another rule statement. You provide one <a>Statement</a> within the <code>NotStatement</code>.</p>
    #[serde(rename = "NotStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_statement: Option<NotStatement>,
    /// <p>A logical rule statement used to combine other rule statements with OR logic. You provide more than one <a>Statement</a> within the <code>OrStatement</code>. </p>
    #[serde(rename = "OrStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_statement: Option<OrStatement>,
    /// <p>A rate-based rule tracks the rate of requests for each originating IP address, and triggers the rule action when the rate exceeds a limit that you specify on the number of requests in any 5-minute time span. You can use this to put a temporary block on requests from an IP address that is sending excessive requests.</p> <p>When the rule action triggers, AWS WAF blocks additional requests from the IP address until the request rate falls below the limit.</p> <p>You can optionally nest another statement inside the rate-based statement, to narrow the scope of the rule so that it only counts requests that match the nested statement. For example, based on recent requests that you have seen from an attacker, you might create a rate-based rule with a nested AND rule statement that contains the following nested statements:</p> <ul> <li> <p>An IP match statement with an IP set that specified the address 192.0.2.44.</p> </li> <li> <p>A string match statement that searches in the User-Agent header for the string BadBot.</p> </li> </ul> <p>In this rate-based rule, you also define a rate limit. For this example, the rate limit is 1,000. Requests that meet both of the conditions in the statements are counted. If the count exceeds 1,000 requests per five minutes, the rule action triggers. Requests that do not meet both conditions are not counted towards the rate limit and are not affected by this rule.</p> <p>You cannot nest a <code>RateBasedStatement</code>, for example for use inside a <code>NotStatement</code> or <code>OrStatement</code>. It can only be referenced as a top-level statement within a rule.</p>
    #[serde(rename = "RateBasedStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_based_statement: Option<RateBasedStatement>,
    /// <p>A rule statement used to search web request components for matches with regular expressions. To use this, create a <a>RegexPatternSet</a> that specifies the expressions that you want to detect, then use the ARN of that set in this statement. A web request matches the pattern set rule statement if the request component matches any of the patterns in the set. To create a regex pattern set, see <a>CreateRegexPatternSet</a>.</p> <p>Each regex pattern set rule statement references a regex pattern set. You create and maintain the set independent of your rules. This allows you to use the single set in multiple rules. When you update the referenced set, AWS WAF automatically updates all rules that reference it.</p>
    #[serde(rename = "RegexPatternSetReferenceStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_pattern_set_reference_statement: Option<RegexPatternSetReferenceStatement>,
    /// <p>A rule statement used to run the rules that are defined in a <a>RuleGroup</a>. To use this, create a rule group with your rules, then provide the ARN of the rule group in this statement.</p> <p>You cannot nest a <code>RuleGroupReferenceStatement</code>, for example for use inside a <code>NotStatement</code> or <code>OrStatement</code>. It can only be referenced as a top-level statement within a rule.</p>
    #[serde(rename = "RuleGroupReferenceStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_reference_statement: Option<RuleGroupReferenceStatement>,
    /// <p>A rule statement that compares a number of bytes against the size of a request component, using a comparison operator, such as greater than (&gt;) or less than (&lt;). For example, you can use a size constraint statement to look for query strings that are longer than 100 bytes. </p> <p>If you configure AWS WAF to inspect the request body, AWS WAF inspects only the first 8192 bytes (8 KB). If the request body for your web requests never exceeds 8192 bytes, you can create a size constraint condition and block requests that have a request body greater than 8192 bytes.</p> <p>If you choose URI for the value of Part of the request to filter on, the slash (/) in the URI counts as one character. For example, the URI <code>/logo.jpg</code> is nine characters long.</p>
    #[serde(rename = "SizeConstraintStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_constraint_statement: Option<SizeConstraintStatement>,
    /// <p>Attackers sometimes insert malicious SQL code into web requests in an effort to extract data from your database. To allow or block web requests that appear to contain malicious SQL code, create one or more SQL injection match conditions. An SQL injection match condition identifies the part of web requests, such as the URI or the query string, that you want AWS WAF to inspect. Later in the process, when you create a web ACL, you specify whether to allow or block requests that appear to contain malicious SQL code.</p>
    #[serde(rename = "SqliMatchStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqli_match_statement: Option<SqliMatchStatement>,
    /// <p>A rule statement that defines a cross-site scripting (XSS) match search for AWS WAF to apply to web requests. XSS attacks are those where the attacker uses vulnerabilities in a benign website as a vehicle to inject malicious client-site scripts into other legitimate web browsers. The XSS match statement provides the location in requests that you want AWS WAF to search and text transformations to use on the search area before AWS WAF searches for character sequences that are likely to be malicious strings. </p>
    #[serde(rename = "XssMatchStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xss_match_statement: Option<XssMatchStatement>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A collection of key:value pairs associated with an AWS resource. The key:value pair can be anything you define. Typically, the tag key represents a category (such as &quot;environment&quot;) and the tag value represents a specific value within that category (such as &quot;test,&quot; &quot;development,&quot; or &quot;production&quot;). You can add up to 50 tags to each AWS resource. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>Part of the key:value pair that defines a tag. You can use a tag key to describe a category of information, such as "customer." Tag keys are case-sensitive.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>Part of the key:value pair that defines a tag. You can use a tag value to describe a specific value within a category, such as "companyA" or "companyB." Tag values are case-sensitive.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>The collection of tagging definitions for an AWS resource. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagInfoForResource {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The array of <a>Tag</a> objects defined for the resource. </p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>An array of key:value pairs to associate with the resource.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextTransformation {
    /// <p>Sets the relative processing order for multiple transformations that are defined for a rule statement. AWS WAF processes all transformations, from lowest priority to highest, before inspecting the transformed content. The priorities don't need to be consecutive, but they must all be different. </p>
    #[serde(rename = "Priority")]
    pub priority: i64,
    /// <p>You can specify the following transformation types:</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system command line command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \ " ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\f, formfeed, decimal 12</p> </li> <li> <p>\t, tab, decimal 9</p> </li> <li> <p>\n, newline, decimal 10</p> </li> <li> <p>\r, carriage return, decimal 13</p> </li> <li> <p>\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a "less than" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want any text transformations.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>In a <a>GetSampledRequests</a> request, the <code>StartTime</code> and <code>EndTime</code> objects specify the time range for which you want AWS WAF to return a sample of web requests.</p> <p>In a <a>GetSampledRequests</a> response, the <code>StartTime</code> and <code>EndTime</code> objects specify the time range for which AWS WAF actually returned a sample of web requests. AWS WAF gets the specified number of requests from among the first 5,000 requests that your AWS resource receives during the specified time period. If your resource receives more than 5,000 requests during that period, AWS WAF stops sampling after the 5,000th request. In that case, <code>EndTime</code> is the time that AWS WAF received the 5,000th request. </p></p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>An array of keys identifying the tags to disassociate from the resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateIPSetRequest {
    /// <p>Contains an array of strings that specify one or more IP addresses or blocks of IP addresses in Classless Inter-Domain Routing (CIDR) notation. AWS WAF supports all address ranges for IP versions IPv4 and IPv6. </p> <p>Examples: </p> <ul> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from the IP address 192.0.2.44, specify <code>192.0.2.44/32</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify <code>192.0.2.0/24</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify <code>1111:0000:0000:0000:0000:0000:0000:0111/128</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify <code>1111:0000:0000:0000:0000:0000:0000:0000/64</code>.</p> </li> </ul> <p>For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p>
    #[serde(rename = "Addresses")]
    pub addresses: Vec<String>,
    /// <p>A friendly description of the IP set. You cannot change the description of an IP set after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    pub lock_token: String,
    /// <p>A friendly name of the IP set. You cannot change the name of an <code>IPSet</code> after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateIPSetResponse {
    /// <p>A token used for optimistic locking. AWS WAF returns this token to your update requests. You use <code>NextLockToken</code> in the same manner as you use <code>LockToken</code>. </p>
    #[serde(rename = "NextLockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_lock_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRegexPatternSetRequest {
    /// <p>A friendly description of the set. You cannot change the description of a set after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    pub lock_token: String,
    /// <p>A friendly name of the set. You cannot change the name after you create the set.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p/></p>
    #[serde(rename = "RegularExpressionList")]
    pub regular_expression_list: Vec<Regex>,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRegexPatternSetResponse {
    /// <p>A token used for optimistic locking. AWS WAF returns this token to your update requests. You use <code>NextLockToken</code> in the same manner as you use <code>LockToken</code>. </p>
    #[serde(rename = "NextLockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_lock_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRuleGroupRequest {
    /// <p>A friendly description of the rule group. You cannot change the description of a rule group after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier for the rule group. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    pub lock_token: String,
    /// <p>A friendly name of the rule group. You cannot change the name of a rule group after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The <a>Rule</a> statements used to identify the web requests that you want to allow, block, or count. Each rule includes one top-level statement that AWS WAF uses to identify matching web requests, and parameters that govern how AWS WAF handles them. </p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
    /// <p>Defines and enables Amazon CloudWatch metrics and web request sample collection. </p>
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: VisibilityConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRuleGroupResponse {
    /// <p>A token used for optimistic locking. AWS WAF returns this token to your update requests. You use <code>NextLockToken</code> in the same manner as you use <code>LockToken</code>. </p>
    #[serde(rename = "NextLockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_lock_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateWebACLRequest {
    /// <p>The action to perform if none of the <code>Rules</code> contained in the <code>WebACL</code> match. </p>
    #[serde(rename = "DefaultAction")]
    pub default_action: DefaultAction,
    /// <p>A friendly description of the Web ACL. You cannot change the description of a Web ACL after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique identifier for the Web ACL. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    pub lock_token: String,
    /// <p>A friendly name of the Web ACL. You cannot change the name of a Web ACL after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The <a>Rule</a> statements used to identify the web requests that you want to allow, block, or count. Each rule includes one top-level statement that AWS WAF uses to identify matching web requests, and parameters that govern how AWS WAF handles them. </p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    /// <p><p>Specifies whether this is for an AWS CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p> <ul> <li> <p>CLI - Specify the region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li> <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li> </ul></p>
    #[serde(rename = "Scope")]
    pub scope: String,
    /// <p>Defines and enables Amazon CloudWatch metrics and web request sample collection. </p>
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: VisibilityConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateWebACLResponse {
    /// <p>A token used for optimistic locking. AWS WAF returns this token to your update requests. You use <code>NextLockToken</code> in the same manner as you use <code>LockToken</code>. </p>
    #[serde(rename = "NextLockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_lock_token: Option<String>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>The path component of the URI of a web request. This is the part of a web request that identifies a resource, for example, <code>/images/daily-ad.jpg</code>.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UriPath {}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Defines and enables Amazon CloudWatch metrics and web request sample collection. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisibilityConfig {
    /// <p>A boolean indicating whether the associated resource sends metrics to CloudWatch. For the list of available metrics, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/monitoring-cloudwatch.html#waf-metrics">AWS WAF Metrics</a>.</p>
    #[serde(rename = "CloudWatchMetricsEnabled")]
    pub cloud_watch_metrics_enabled: bool,
    /// <p>A friendly name of the CloudWatch metric. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with length from one to 128 characters. It can't contain whitespace or metric names reserved for AWS WAF, for example "All" and "Default_Action." You can't change a <code>MetricName</code> after you create a <code>VisibilityConfig</code>.</p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// <p>A boolean indicating whether AWS WAF should store a sampling of the web requests that match the rules. You can view the sampled requests through the AWS WAF console. </p>
    #[serde(rename = "SampledRequestsEnabled")]
    pub sampled_requests_enabled: bool,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p> A Web ACL defines a collection of rules to use to inspect and control web requests. Each rule has an action defined (allow, block, or count) for requests that match the statement of the rule. In the Web ACL, you assign a default action to take (allow, block) for any request that does not match any of the rules. The rules in a Web ACL can be a combination of the types <a>Rule</a>, <a>RuleGroup</a>, and managed rule group. You can associate a Web ACL with one or more AWS resources to protect. The resources can be Amazon CloudFront, an Amazon API Gateway API, or an Application Load Balancer. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WebACL {
    /// <p>The Amazon Resource Name (ARN) of the Web ACL that you want to associate with the resource.</p>
    #[serde(rename = "ARN")]
    pub arn: String,
    /// <p>The web ACL capacity units (WCUs) currently being used by this web ACL. </p> <p>AWS WAF uses WCUs to calculate and control the operating resources that are used to run your rules, rule groups, and web ACLs. AWS WAF calculates capacity differently for each rule type, to reflect the relative cost of each rule. Simple rules that cost little to run use fewer WCUs than more complex rules that use more processing power. Rule group capacity is fixed at creation, which helps users plan their web ACL WCU usage when they use a rule group. The WCU limit for web ACLs is 1,500. </p>
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// <p>The action to perform if none of the <code>Rules</code> contained in the <code>WebACL</code> match. </p>
    #[serde(rename = "DefaultAction")]
    pub default_action: DefaultAction,
    /// <p>A friendly description of the Web ACL. You cannot change the description of a Web ACL after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier for the <code>WebACL</code>. This ID is returned in the responses to create and list commands. You use this ID to do things like get, update, and delete a <code>WebACL</code>.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A friendly name of the Web ACL. You cannot change the name of a Web ACL after you create it.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The <a>Rule</a> statements used to identify the web requests that you want to allow, block, or count. Each rule includes one top-level statement that AWS WAF uses to identify matching web requests, and parameters that govern how AWS WAF handles them. </p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    /// <p>Defines and enables Amazon CloudWatch metrics and web request sample collection. </p>
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: VisibilityConfig,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>High-level information about a <a>WebACL</a>, returned by operations like create and list. This provides information like the ID, that you can use to retrieve and manage a <code>WebACL</code>, and the ARN, that you provide to operations like <a>AssociateWebACL</a>.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WebACLSummary {
    /// <p>The Amazon Resource Name (ARN) of the entity.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A friendly description of the Web ACL. You cannot change the description of a Web ACL after you create it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique identifier for the Web ACL. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A token used for optimistic locking. AWS WAF returns a token to your get and list requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like update and delete. AWS WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another get, and use the new token returned by that operation. </p>
    #[serde(rename = "LockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_token: Option<String>,
    /// <p>A friendly name of the Web ACL. You cannot change the name of a Web ACL after you create it.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>A rule statement that defines a cross-site scripting (XSS) match search for AWS WAF to apply to web requests. XSS attacks are those where the attacker uses vulnerabilities in a benign website as a vehicle to inject malicious client-site scripts into other legitimate web browsers. The XSS match statement provides the location in requests that you want AWS WAF to search and text transformations to use on the search area before AWS WAF searches for character sequences that are likely to be malicious strings. </p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XssMatchStatement {
    /// <p>The part of a web request that you want AWS WAF to inspect. For more information, see <a>FieldToMatch</a>. </p>
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. If you specify one or more transformations in a rule statement, AWS WAF performs all transformations on the content identified by <code>FieldToMatch</code>, starting from the lowest priority setting, before inspecting the content for a match.</p>
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
}

/// Errors returned by AssociateWebACL
#[derive(Debug, PartialEq)]
pub enum AssociateWebACLError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t retrieve the resource that you requested. Retry your request.</p>
    WAFUnavailableEntity(String),
}

impl AssociateWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateWebACLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(AssociateWebACLError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(AssociateWebACLError::WAFInvalidParameter(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(AssociateWebACLError::WAFNonexistentItem(err.msg))
                }
                "WAFUnavailableEntityException" => {
                    return RusotoError::Service(AssociateWebACLError::WAFUnavailableEntity(
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
impl fmt::Display for AssociateWebACLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateWebACLError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            AssociateWebACLError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateWebACLError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            AssociateWebACLError::WAFUnavailableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateWebACLError {}
/// Errors returned by CheckCapacity
#[derive(Debug, PartialEq)]
pub enum CheckCapacityError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because the resource that you requested isn’t valid. Check the resource, and try again.</p>
    WAFInvalidResource(String),
    /// <p>AWS WAF couldn’t perform the operation because you exceeded your resource limit. For example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t retrieve the resource that you requested. Retry your request.</p>
    WAFUnavailableEntity(String),
}

impl CheckCapacityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CheckCapacityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CheckCapacityError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CheckCapacityError::WAFInvalidParameter(err.msg))
                }
                "WAFInvalidResourceException" => {
                    return RusotoError::Service(CheckCapacityError::WAFInvalidResource(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CheckCapacityError::WAFLimitsExceeded(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(CheckCapacityError::WAFNonexistentItem(err.msg))
                }
                "WAFUnavailableEntityException" => {
                    return RusotoError::Service(CheckCapacityError::WAFUnavailableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CheckCapacityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CheckCapacityError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CheckCapacityError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            CheckCapacityError::WAFInvalidResource(ref cause) => write!(f, "{}", cause),
            CheckCapacityError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CheckCapacityError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            CheckCapacityError::WAFUnavailableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CheckCapacityError {}
/// Errors returned by CreateIPSet
#[derive(Debug, PartialEq)]
pub enum CreateIPSetError {
    /// <p>AWS WAF couldn’t perform the operation because the resource that you tried to save is a duplicate of an existing one.</p>
    WAFDuplicateItem(String),
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because you exceeded your resource limit. For example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
    /// <p>An error occurred during the tagging operation. Retry your request.</p>
    WAFTagOperation(String),
    /// <p>AWS WAF couldn’t perform your tagging operation because of an internal error. Retry your request.</p>
    WAFTagOperationInternalError(String),
}

impl CreateIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIPSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDuplicateItemException" => {
                    return RusotoError::Service(CreateIPSetError::WAFDuplicateItem(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateIPSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateIPSetError::WAFInvalidParameter(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateIPSetError::WAFLimitsExceeded(err.msg))
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(CreateIPSetError::WAFOptimisticLock(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(CreateIPSetError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(CreateIPSetError::WAFTagOperationInternalError(
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
impl fmt::Display for CreateIPSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateIPSetError::WAFDuplicateItem(ref cause) => write!(f, "{}", cause),
            CreateIPSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateIPSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateIPSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateIPSetError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
            CreateIPSetError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            CreateIPSetError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIPSetError {}
/// Errors returned by CreateRegexPatternSet
#[derive(Debug, PartialEq)]
pub enum CreateRegexPatternSetError {
    /// <p>AWS WAF couldn’t perform the operation because the resource that you tried to save is a duplicate of an existing one.</p>
    WAFDuplicateItem(String),
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because you exceeded your resource limit. For example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
    /// <p>An error occurred during the tagging operation. Retry your request.</p>
    WAFTagOperation(String),
    /// <p>AWS WAF couldn’t perform your tagging operation because of an internal error. Retry your request.</p>
    WAFTagOperationInternalError(String),
}

impl CreateRegexPatternSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRegexPatternSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDuplicateItemException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFDuplicateItem(
                        err.msg,
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFLimitsExceeded(
                        err.msg,
                    ))
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFOptimisticLock(
                        err.msg,
                    ))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFTagOperation(
                        err.msg,
                    ))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(
                        CreateRegexPatternSetError::WAFTagOperationInternalError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRegexPatternSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRegexPatternSetError::WAFDuplicateItem(ref cause) => write!(f, "{}", cause),
            CreateRegexPatternSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateRegexPatternSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateRegexPatternSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateRegexPatternSetError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
            CreateRegexPatternSetError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            CreateRegexPatternSetError::WAFTagOperationInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateRegexPatternSetError {}
/// Errors returned by CreateRuleGroup
#[derive(Debug, PartialEq)]
pub enum CreateRuleGroupError {
    /// <p>AWS WAF couldn’t perform the operation because the resource that you tried to save is a duplicate of an existing one.</p>
    WAFDuplicateItem(String),
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because you exceeded your resource limit. For example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
    /// <p>An error occurred during the tagging operation. Retry your request.</p>
    WAFTagOperation(String),
    /// <p>AWS WAF couldn’t perform your tagging operation because of an internal error. Retry your request.</p>
    WAFTagOperationInternalError(String),
    /// <p>AWS WAF couldn’t retrieve the resource that you requested. Retry your request.</p>
    WAFUnavailableEntity(String),
}

impl CreateRuleGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRuleGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDuplicateItemException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFDuplicateItem(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFInvalidParameter(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFLimitsExceeded(err.msg))
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFOptimisticLock(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(
                        CreateRuleGroupError::WAFTagOperationInternalError(err.msg),
                    )
                }
                "WAFUnavailableEntityException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFUnavailableEntity(
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
impl fmt::Display for CreateRuleGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRuleGroupError::WAFDuplicateItem(ref cause) => write!(f, "{}", cause),
            CreateRuleGroupError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateRuleGroupError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateRuleGroupError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateRuleGroupError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
            CreateRuleGroupError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            CreateRuleGroupError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
            CreateRuleGroupError::WAFUnavailableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRuleGroupError {}
/// Errors returned by CreateWebACL
#[derive(Debug, PartialEq)]
pub enum CreateWebACLError {
    /// <p>AWS WAF couldn’t perform the operation because the resource that you tried to save is a duplicate of an existing one.</p>
    WAFDuplicateItem(String),
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because the resource that you requested isn’t valid. Check the resource, and try again.</p>
    WAFInvalidResource(String),
    /// <p>AWS WAF couldn’t perform the operation because you exceeded your resource limit. For example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
    /// <p>An error occurred during the tagging operation. Retry your request.</p>
    WAFTagOperation(String),
    /// <p>AWS WAF couldn’t perform your tagging operation because of an internal error. Retry your request.</p>
    WAFTagOperationInternalError(String),
    /// <p>AWS WAF couldn’t retrieve the resource that you requested. Retry your request.</p>
    WAFUnavailableEntity(String),
}

impl CreateWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWebACLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDuplicateItemException" => {
                    return RusotoError::Service(CreateWebACLError::WAFDuplicateItem(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateWebACLError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateWebACLError::WAFInvalidParameter(err.msg))
                }
                "WAFInvalidResourceException" => {
                    return RusotoError::Service(CreateWebACLError::WAFInvalidResource(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateWebACLError::WAFLimitsExceeded(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(CreateWebACLError::WAFNonexistentItem(err.msg))
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(CreateWebACLError::WAFOptimisticLock(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(CreateWebACLError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(CreateWebACLError::WAFTagOperationInternalError(
                        err.msg,
                    ))
                }
                "WAFUnavailableEntityException" => {
                    return RusotoError::Service(CreateWebACLError::WAFUnavailableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateWebACLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateWebACLError::WAFDuplicateItem(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFInvalidResource(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFUnavailableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateWebACLError {}
/// Errors returned by DeleteIPSet
#[derive(Debug, PartialEq)]
pub enum DeleteIPSetError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
    /// <p>An error occurred during the tagging operation. Retry your request.</p>
    WAFTagOperation(String),
    /// <p>AWS WAF couldn’t perform your tagging operation because of an internal error. Retry your request.</p>
    WAFTagOperationInternalError(String),
}

impl DeleteIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIPSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFInvalidParameter(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFNonexistentItem(err.msg))
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFOptimisticLock(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFTagOperationInternalError(
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
impl fmt::Display for DeleteIPSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteIPSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteIPSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteIPSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteIPSetError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
            DeleteIPSetError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            DeleteIPSetError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteIPSetError {}
/// Errors returned by DeleteLoggingConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteLoggingConfigurationError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
}

impl DeleteLoggingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteLoggingConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteLoggingConfigurationError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        DeleteLoggingConfigurationError::WAFNonexistentItem(err.msg),
                    )
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(
                        DeleteLoggingConfigurationError::WAFOptimisticLock(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLoggingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLoggingConfigurationError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteLoggingConfigurationError::WAFNonexistentItem(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteLoggingConfigurationError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLoggingConfigurationError {}
/// Errors returned by DeleteRegexPatternSet
#[derive(Debug, PartialEq)]
pub enum DeleteRegexPatternSetError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
    /// <p>An error occurred during the tagging operation. Retry your request.</p>
    WAFTagOperation(String),
    /// <p>AWS WAF couldn’t perform your tagging operation because of an internal error. Retry your request.</p>
    WAFTagOperationInternalError(String),
}

impl DeleteRegexPatternSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRegexPatternSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFOptimisticLock(
                        err.msg,
                    ))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFTagOperation(
                        err.msg,
                    ))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(
                        DeleteRegexPatternSetError::WAFTagOperationInternalError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRegexPatternSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRegexPatternSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteRegexPatternSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteRegexPatternSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteRegexPatternSetError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
            DeleteRegexPatternSetError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            DeleteRegexPatternSetError::WAFTagOperationInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteRegexPatternSetError {}
/// Errors returned by DeleteRuleGroup
#[derive(Debug, PartialEq)]
pub enum DeleteRuleGroupError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
    /// <p>An error occurred during the tagging operation. Retry your request.</p>
    WAFTagOperation(String),
    /// <p>AWS WAF couldn’t perform your tagging operation because of an internal error. Retry your request.</p>
    WAFTagOperationInternalError(String),
}

impl DeleteRuleGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRuleGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFInvalidParameter(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFNonexistentItem(err.msg))
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFOptimisticLock(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(
                        DeleteRuleGroupError::WAFTagOperationInternalError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRuleGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRuleGroupError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteRuleGroupError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteRuleGroupError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteRuleGroupError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
            DeleteRuleGroupError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            DeleteRuleGroupError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRuleGroupError {}
/// Errors returned by DeleteWebACL
#[derive(Debug, PartialEq)]
pub enum DeleteWebACLError {
    /// <p>AWS WAF couldn’t perform the operation because your resource is being used by another resource or it’s associated with another resource. </p>
    WAFAssociatedItem(String),
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
    /// <p>An error occurred during the tagging operation. Retry your request.</p>
    WAFTagOperation(String),
    /// <p>AWS WAF couldn’t perform your tagging operation because of an internal error. Retry your request.</p>
    WAFTagOperationInternalError(String),
}

impl DeleteWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWebACLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFAssociatedItemException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFAssociatedItem(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFInvalidParameter(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFNonexistentItem(err.msg))
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFOptimisticLock(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFTagOperationInternalError(
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
impl fmt::Display for DeleteWebACLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteWebACLError::WAFAssociatedItem(ref cause) => write!(f, "{}", cause),
            DeleteWebACLError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteWebACLError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteWebACLError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteWebACLError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
            DeleteWebACLError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            DeleteWebACLError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteWebACLError {}
/// Errors returned by DescribeManagedRuleGroup
#[derive(Debug, PartialEq)]
pub enum DescribeManagedRuleGroupError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because the resource that you requested isn’t valid. Check the resource, and try again.</p>
    WAFInvalidResource(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
}

impl DescribeManagedRuleGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeManagedRuleGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DescribeManagedRuleGroupError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        DescribeManagedRuleGroupError::WAFInvalidParameter(err.msg),
                    )
                }
                "WAFInvalidResourceException" => {
                    return RusotoError::Service(DescribeManagedRuleGroupError::WAFInvalidResource(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DescribeManagedRuleGroupError::WAFNonexistentItem(
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
impl fmt::Display for DescribeManagedRuleGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeManagedRuleGroupError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DescribeManagedRuleGroupError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeManagedRuleGroupError::WAFInvalidResource(ref cause) => write!(f, "{}", cause),
            DescribeManagedRuleGroupError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeManagedRuleGroupError {}
/// Errors returned by DisassociateWebACL
#[derive(Debug, PartialEq)]
pub enum DisassociateWebACLError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
}

impl DisassociateWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateWebACLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DisassociateWebACLError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(DisassociateWebACLError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DisassociateWebACLError::WAFNonexistentItem(
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
impl fmt::Display for DisassociateWebACLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateWebACLError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DisassociateWebACLError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            DisassociateWebACLError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateWebACLError {}
/// Errors returned by GetIPSet
#[derive(Debug, PartialEq)]
pub enum GetIPSetError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
}

impl GetIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIPSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetIPSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(GetIPSetError::WAFInvalidParameter(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetIPSetError::WAFNonexistentItem(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetIPSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetIPSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetIPSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            GetIPSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetIPSetError {}
/// Errors returned by GetLoggingConfiguration
#[derive(Debug, PartialEq)]
pub enum GetLoggingConfigurationError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
}

impl GetLoggingConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLoggingConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetLoggingConfigurationError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetLoggingConfigurationError::WAFNonexistentItem(
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
impl fmt::Display for GetLoggingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLoggingConfigurationError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetLoggingConfigurationError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLoggingConfigurationError {}
/// Errors returned by GetRateBasedStatementManagedKeys
#[derive(Debug, PartialEq)]
pub enum GetRateBasedStatementManagedKeysError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
}

impl GetRateBasedStatementManagedKeysError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRateBasedStatementManagedKeysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(
                        GetRateBasedStatementManagedKeysError::WAFInternalError(err.msg),
                    )
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        GetRateBasedStatementManagedKeysError::WAFInvalidParameter(err.msg),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        GetRateBasedStatementManagedKeysError::WAFNonexistentItem(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRateBasedStatementManagedKeysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRateBasedStatementManagedKeysError::WAFInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRateBasedStatementManagedKeysError::WAFInvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRateBasedStatementManagedKeysError::WAFNonexistentItem(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetRateBasedStatementManagedKeysError {}
/// Errors returned by GetRegexPatternSet
#[derive(Debug, PartialEq)]
pub enum GetRegexPatternSetError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
}

impl GetRegexPatternSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRegexPatternSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetRegexPatternSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(GetRegexPatternSetError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetRegexPatternSetError::WAFNonexistentItem(
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
impl fmt::Display for GetRegexPatternSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRegexPatternSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetRegexPatternSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            GetRegexPatternSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRegexPatternSetError {}
/// Errors returned by GetRuleGroup
#[derive(Debug, PartialEq)]
pub enum GetRuleGroupError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
}

impl GetRuleGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRuleGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetRuleGroupError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(GetRuleGroupError::WAFInvalidParameter(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetRuleGroupError::WAFNonexistentItem(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRuleGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRuleGroupError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetRuleGroupError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            GetRuleGroupError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRuleGroupError {}
/// Errors returned by GetSampledRequests
#[derive(Debug, PartialEq)]
pub enum GetSampledRequestsError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
}

impl GetSampledRequestsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSampledRequestsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetSampledRequestsError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(GetSampledRequestsError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetSampledRequestsError::WAFNonexistentItem(
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
impl fmt::Display for GetSampledRequestsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSampledRequestsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetSampledRequestsError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            GetSampledRequestsError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSampledRequestsError {}
/// Errors returned by GetWebACL
#[derive(Debug, PartialEq)]
pub enum GetWebACLError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
}

impl GetWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetWebACLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetWebACLError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(GetWebACLError::WAFInvalidParameter(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetWebACLError::WAFNonexistentItem(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetWebACLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetWebACLError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetWebACLError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            GetWebACLError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetWebACLError {}
/// Errors returned by GetWebACLForResource
#[derive(Debug, PartialEq)]
pub enum GetWebACLForResourceError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t retrieve the resource that you requested. Retry your request.</p>
    WAFUnavailableEntity(String),
}

impl GetWebACLForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetWebACLForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetWebACLForResourceError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(GetWebACLForResourceError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetWebACLForResourceError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFUnavailableEntityException" => {
                    return RusotoError::Service(GetWebACLForResourceError::WAFUnavailableEntity(
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
impl fmt::Display for GetWebACLForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetWebACLForResourceError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetWebACLForResourceError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            GetWebACLForResourceError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            GetWebACLForResourceError::WAFUnavailableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetWebACLForResourceError {}
/// Errors returned by ListAvailableManagedRuleGroups
#[derive(Debug, PartialEq)]
pub enum ListAvailableManagedRuleGroupsError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
}

impl ListAvailableManagedRuleGroupsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAvailableManagedRuleGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(
                        ListAvailableManagedRuleGroupsError::WAFInternalError(err.msg),
                    )
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        ListAvailableManagedRuleGroupsError::WAFInvalidParameter(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAvailableManagedRuleGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAvailableManagedRuleGroupsError::WAFInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAvailableManagedRuleGroupsError::WAFInvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListAvailableManagedRuleGroupsError {}
/// Errors returned by ListIPSets
#[derive(Debug, PartialEq)]
pub enum ListIPSetsError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
}

impl ListIPSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIPSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListIPSetsError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(ListIPSetsError::WAFInvalidParameter(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListIPSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIPSetsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListIPSetsError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListIPSetsError {}
/// Errors returned by ListLoggingConfigurations
#[derive(Debug, PartialEq)]
pub enum ListLoggingConfigurationsError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
}

impl ListLoggingConfigurationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLoggingConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListLoggingConfigurationsError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        ListLoggingConfigurationsError::WAFInvalidParameter(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLoggingConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLoggingConfigurationsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListLoggingConfigurationsError::WAFInvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListLoggingConfigurationsError {}
/// Errors returned by ListRegexPatternSets
#[derive(Debug, PartialEq)]
pub enum ListRegexPatternSetsError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
}

impl ListRegexPatternSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRegexPatternSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListRegexPatternSetsError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(ListRegexPatternSetsError::WAFInvalidParameter(
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
impl fmt::Display for ListRegexPatternSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRegexPatternSetsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListRegexPatternSetsError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRegexPatternSetsError {}
/// Errors returned by ListResourcesForWebACL
#[derive(Debug, PartialEq)]
pub enum ListResourcesForWebACLError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
}

impl ListResourcesForWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourcesForWebACLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListResourcesForWebACLError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(ListResourcesForWebACLError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(ListResourcesForWebACLError::WAFNonexistentItem(
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
impl fmt::Display for ListResourcesForWebACLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourcesForWebACLError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListResourcesForWebACLError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            ListResourcesForWebACLError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResourcesForWebACLError {}
/// Errors returned by ListRuleGroups
#[derive(Debug, PartialEq)]
pub enum ListRuleGroupsError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
}

impl ListRuleGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRuleGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListRuleGroupsError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(ListRuleGroupsError::WAFInvalidParameter(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRuleGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRuleGroupsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListRuleGroupsError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRuleGroupsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>An error occurred during the tagging operation. Retry your request.</p>
    WAFTagOperation(String),
    /// <p>AWS WAF couldn’t perform your tagging operation because of an internal error. Retry your request.</p>
    WAFTagOperationInternalError(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(ListTagsForResourceError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(ListTagsForResourceError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(
                        ListTagsForResourceError::WAFTagOperationInternalError(err.msg),
                    )
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
            ListTagsForResourceError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::WAFTagOperationInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListWebACLs
#[derive(Debug, PartialEq)]
pub enum ListWebACLsError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
}

impl ListWebACLsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWebACLsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListWebACLsError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(ListWebACLsError::WAFInvalidParameter(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListWebACLsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListWebACLsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListWebACLsError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListWebACLsError {}
/// Errors returned by PutLoggingConfiguration
#[derive(Debug, PartialEq)]
pub enum PutLoggingConfigurationError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
    /// <p>AWS WAF is not able to access the service linked role. This can be caused by a previous <code>PutLoggingConfiguration</code> request, which can lock the service linked role for about 20 seconds. Please try your request again. The service linked role can also be locked by a previous <code>DeleteServiceLinkedRole</code> request, which can lock the role for 15 minutes or more. If you recently made a call to <code>DeleteServiceLinkedRole</code>, wait at least 15 minutes and try the request again. If you receive this same exception again, you will have to wait additional time until the role is unlocked.</p>
    WAFServiceLinkedRoleError(String),
}

impl PutLoggingConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutLoggingConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(PutLoggingConfigurationError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(PutLoggingConfigurationError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(PutLoggingConfigurationError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(PutLoggingConfigurationError::WAFOptimisticLock(
                        err.msg,
                    ))
                }
                "WAFServiceLinkedRoleErrorException" => {
                    return RusotoError::Service(
                        PutLoggingConfigurationError::WAFServiceLinkedRoleError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutLoggingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutLoggingConfigurationError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            PutLoggingConfigurationError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            PutLoggingConfigurationError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            PutLoggingConfigurationError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
            PutLoggingConfigurationError::WAFServiceLinkedRoleError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutLoggingConfigurationError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because you exceeded your resource limit. For example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>An error occurred during the tagging operation. Retry your request.</p>
    WAFTagOperation(String),
    /// <p>AWS WAF couldn’t perform your tagging operation because of an internal error. Retry your request.</p>
    WAFTagOperationInternalError(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(TagResourceError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::WAFInvalidParameter(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(TagResourceError::WAFLimitsExceeded(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(TagResourceError::WAFNonexistentItem(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(TagResourceError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(TagResourceError::WAFTagOperationInternalError(
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
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            TagResourceError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            TagResourceError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            TagResourceError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>An error occurred during the tagging operation. Retry your request.</p>
    WAFTagOperation(String),
    /// <p>AWS WAF couldn’t perform your tagging operation because of an internal error. Retry your request.</p>
    WAFTagOperationInternalError(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UntagResourceError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::WAFInvalidParameter(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UntagResourceError::WAFNonexistentItem(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(UntagResourceError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(UntagResourceError::WAFTagOperationInternalError(
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
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UntagResourceError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            UntagResourceError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateIPSet
#[derive(Debug, PartialEq)]
pub enum UpdateIPSetError {
    /// <p>AWS WAF couldn’t perform the operation because the resource that you tried to save is a duplicate of an existing one.</p>
    WAFDuplicateItem(String),
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because you exceeded your resource limit. For example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
}

impl UpdateIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateIPSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDuplicateItemException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFDuplicateItem(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFInvalidParameter(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFLimitsExceeded(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFNonexistentItem(err.msg))
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFOptimisticLock(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateIPSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateIPSetError::WAFDuplicateItem(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateIPSetError {}
/// Errors returned by UpdateRegexPatternSet
#[derive(Debug, PartialEq)]
pub enum UpdateRegexPatternSetError {
    /// <p>AWS WAF couldn’t perform the operation because the resource that you tried to save is a duplicate of an existing one.</p>
    WAFDuplicateItem(String),
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because you exceeded your resource limit. For example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
}

impl UpdateRegexPatternSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRegexPatternSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDuplicateItemException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFDuplicateItem(
                        err.msg,
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFLimitsExceeded(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFOptimisticLock(
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
impl fmt::Display for UpdateRegexPatternSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRegexPatternSetError::WAFDuplicateItem(ref cause) => write!(f, "{}", cause),
            UpdateRegexPatternSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateRegexPatternSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateRegexPatternSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateRegexPatternSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateRegexPatternSetError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRegexPatternSetError {}
/// Errors returned by UpdateRuleGroup
#[derive(Debug, PartialEq)]
pub enum UpdateRuleGroupError {
    /// <p>AWS WAF couldn’t perform the operation because the resource that you tried to save is a duplicate of an existing one.</p>
    WAFDuplicateItem(String),
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because you exceeded your resource limit. For example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
    /// <p>AWS WAF couldn’t retrieve the resource that you requested. Retry your request.</p>
    WAFUnavailableEntity(String),
}

impl UpdateRuleGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRuleGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDuplicateItemException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFDuplicateItem(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFInvalidParameter(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFLimitsExceeded(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFNonexistentItem(err.msg))
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFOptimisticLock(err.msg))
                }
                "WAFUnavailableEntityException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFUnavailableEntity(
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
impl fmt::Display for UpdateRuleGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRuleGroupError::WAFDuplicateItem(ref cause) => write!(f, "{}", cause),
            UpdateRuleGroupError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateRuleGroupError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateRuleGroupError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateRuleGroupError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateRuleGroupError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
            UpdateRuleGroupError::WAFUnavailableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRuleGroupError {}
/// Errors returned by UpdateWebACL
#[derive(Debug, PartialEq)]
pub enum UpdateWebACLError {
    /// <p>AWS WAF couldn’t perform the operation because the resource that you tried to save is a duplicate of an existing one.</p>
    WAFDuplicateItem(String),
    /// <p>Your request is valid, but AWS WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example: </p> <ul> <li> <p>You specified an invalid parameter name or value.</p> </li> <li> <p>Your nested statement isn&#39;t valid. You might have tried to nest a statement that can’t be nested. </p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn&#39;t among the types available at <a>DefaultAction</a>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a Web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>AWS WAF couldn’t perform the operation because the resource that you requested isn’t valid. Check the resource, and try again.</p>
    WAFInvalidResource(String),
    /// <p>AWS WAF couldn’t perform the operation because you exceeded your resource limit. For example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>AWS WAF couldn’t perform the operation because your resource doesn’t exist. </p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WAFOptimisticLock(String),
    /// <p>AWS WAF couldn’t retrieve the resource that you requested. Retry your request.</p>
    WAFUnavailableEntity(String),
}

impl UpdateWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateWebACLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDuplicateItemException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFDuplicateItem(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFInvalidParameter(err.msg))
                }
                "WAFInvalidResourceException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFInvalidResource(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFLimitsExceeded(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFNonexistentItem(err.msg))
                }
                "WAFOptimisticLockException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFOptimisticLock(err.msg))
                }
                "WAFUnavailableEntityException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFUnavailableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateWebACLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateWebACLError::WAFDuplicateItem(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFInvalidResource(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFOptimisticLock(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFUnavailableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateWebACLError {}
/// Trait representing the capabilities of the WAFV2 API. WAFV2 clients implement this trait.
#[async_trait]
pub trait WafV2 {
    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Associates a Web ACL with a regional application resource, to protect the resource. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>For AWS CloudFront, you can associate the Web ACL by providing the <code>Id</code> of the <a>WebACL</a> to the CloudFront API call <code>UpdateDistribution</code>. For information, see <a href="https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_UpdateDistribution.html">UpdateDistribution</a>.</p></p>
    async fn associate_web_acl(
        &self,
        input: AssociateWebACLRequest,
    ) -> Result<AssociateWebACLResponse, RusotoError<AssociateWebACLError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Returns the web ACL capacity unit (WCU) requirements for a specified scope and set of rules. You can use this to check the capacity requirements for the rules you want to use in a <a>RuleGroup</a> or <a>WebACL</a>. </p> <p>AWS WAF uses WCUs to calculate and control the operating resources that are used to run your rules, rule groups, and web ACLs. AWS WAF calculates capacity differently for each rule type, to reflect the relative cost of each rule. Simple rules that cost little to run use fewer WCUs than more complex rules that use more processing power. Rule group capacity is fixed at creation, which helps users plan their web ACL WCU usage when they use a rule group. The WCU limit for web ACLs is 1,500. </p></p>
    async fn check_capacity(
        &self,
        input: CheckCapacityRequest,
    ) -> Result<CheckCapacityResponse, RusotoError<CheckCapacityError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Creates an <a>IPSet</a>, which you use to identify web requests that originate from specific IP addresses or ranges of IP addresses. For example, if you&#39;re receiving a lot of requests from a ranges of IP addresses, you can configure AWS WAF to block them using an IPSet that lists those IP addresses. </p></p>
    async fn create_ip_set(
        &self,
        input: CreateIPSetRequest,
    ) -> Result<CreateIPSetResponse, RusotoError<CreateIPSetError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Creates a <a>RegexPatternSet</a> per the specifications provided.</p></p>
    async fn create_regex_pattern_set(
        &self,
        input: CreateRegexPatternSetRequest,
    ) -> Result<CreateRegexPatternSetResponse, RusotoError<CreateRegexPatternSetError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Creates a <a>RuleGroup</a> per the specifications provided. </p> <p> A rule group defines a collection of rules to inspect and control web requests that you can use in a <a>WebACL</a>. When you create a rule group, you define an immutable capacity limit. If you update a rule group, you must stay within the capacity. This allows others to reuse the rule group with confidence in its capacity requirements. </p></p>
    async fn create_rule_group(
        &self,
        input: CreateRuleGroupRequest,
    ) -> Result<CreateRuleGroupResponse, RusotoError<CreateRuleGroupError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Creates a <a>WebACL</a> per the specifications provided.</p> <p> A Web ACL defines a collection of rules to use to inspect and control web requests. Each rule has an action defined (allow, block, or count) for requests that match the statement of the rule. In the Web ACL, you assign a default action to take (allow, block) for any request that does not match any of the rules. The rules in a Web ACL can be a combination of the types <a>Rule</a>, <a>RuleGroup</a>, and managed rule group. You can associate a Web ACL with one or more AWS resources to protect. The resources can be Amazon CloudFront, an Amazon API Gateway API, or an Application Load Balancer. </p></p>
    async fn create_web_acl(
        &self,
        input: CreateWebACLRequest,
    ) -> Result<CreateWebACLResponse, RusotoError<CreateWebACLError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Deletes the specified <a>IPSet</a>. </p></p>
    async fn delete_ip_set(
        &self,
        input: DeleteIPSetRequest,
    ) -> Result<DeleteIPSetResponse, RusotoError<DeleteIPSetError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Deletes the <a>LoggingConfiguration</a> from the specified web ACL.</p></p>
    async fn delete_logging_configuration(
        &self,
        input: DeleteLoggingConfigurationRequest,
    ) -> Result<DeleteLoggingConfigurationResponse, RusotoError<DeleteLoggingConfigurationError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Deletes the specified <a>RegexPatternSet</a>.</p></p>
    async fn delete_regex_pattern_set(
        &self,
        input: DeleteRegexPatternSetRequest,
    ) -> Result<DeleteRegexPatternSetResponse, RusotoError<DeleteRegexPatternSetError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Deletes the specified <a>RuleGroup</a>.</p></p>
    async fn delete_rule_group(
        &self,
        input: DeleteRuleGroupRequest,
    ) -> Result<DeleteRuleGroupResponse, RusotoError<DeleteRuleGroupError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Deletes the specified <a>WebACL</a>.</p></p>
    async fn delete_web_acl(
        &self,
        input: DeleteWebACLRequest,
    ) -> Result<DeleteWebACLResponse, RusotoError<DeleteWebACLError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Provides high-level information for a managed rule group, including descriptions of the rules. </p></p>
    async fn describe_managed_rule_group(
        &self,
        input: DescribeManagedRuleGroupRequest,
    ) -> Result<DescribeManagedRuleGroupResponse, RusotoError<DescribeManagedRuleGroupError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Disassociates a Web ACL from a regional application resource. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>For AWS CloudFront, you can disassociate the Web ACL by providing an empty <code>WebACLId</code> in the CloudFront API call <code>UpdateDistribution</code>. For information, see <a href="https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_UpdateDistribution.html">UpdateDistribution</a>.</p></p>
    async fn disassociate_web_acl(
        &self,
        input: DisassociateWebACLRequest,
    ) -> Result<DisassociateWebACLResponse, RusotoError<DisassociateWebACLError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the specified <a>IPSet</a>.</p></p>
    async fn get_ip_set(
        &self,
        input: GetIPSetRequest,
    ) -> Result<GetIPSetResponse, RusotoError<GetIPSetError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Returns the <a>LoggingConfiguration</a> for the specified web ACL.</p></p>
    async fn get_logging_configuration(
        &self,
        input: GetLoggingConfigurationRequest,
    ) -> Result<GetLoggingConfigurationResponse, RusotoError<GetLoggingConfigurationError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the keys that are currently blocked by a rate-based rule. The maximum number of managed keys that can be blocked for a single rate-based rule is 10,000. If more than 10,000 addresses exceed the rate limit, those with the highest rates are blocked.</p></p>
    async fn get_rate_based_statement_managed_keys(
        &self,
        input: GetRateBasedStatementManagedKeysRequest,
    ) -> Result<
        GetRateBasedStatementManagedKeysResponse,
        RusotoError<GetRateBasedStatementManagedKeysError>,
    >;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the specified <a>RegexPatternSet</a>.</p></p>
    async fn get_regex_pattern_set(
        &self,
        input: GetRegexPatternSetRequest,
    ) -> Result<GetRegexPatternSetResponse, RusotoError<GetRegexPatternSetError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the specified <a>RuleGroup</a>.</p></p>
    async fn get_rule_group(
        &self,
        input: GetRuleGroupRequest,
    ) -> Result<GetRuleGroupResponse, RusotoError<GetRuleGroupError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Gets detailed information about a specified number of requests--a sample--that AWS WAF randomly selects from among the first 5,000 requests that your AWS resource received during a time range that you choose. You can specify a sample size of up to 500 requests, and you can specify any time range in the previous three hours.</p> <p> <code>GetSampledRequests</code> returns a time range, which is usually the time range that you specified. However, if your resource (such as a CloudFront distribution) received 5,000 requests before the specified time range elapsed, <code>GetSampledRequests</code> returns an updated time range. This new time range indicates the actual period during which AWS WAF selected the requests in the sample.</p></p>
    async fn get_sampled_requests(
        &self,
        input: GetSampledRequestsRequest,
    ) -> Result<GetSampledRequestsResponse, RusotoError<GetSampledRequestsError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the specified <a>WebACL</a>.</p></p>
    async fn get_web_acl(
        &self,
        input: GetWebACLRequest,
    ) -> Result<GetWebACLResponse, RusotoError<GetWebACLError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the <a>WebACL</a> for the specified resource. </p></p>
    async fn get_web_acl_for_resource(
        &self,
        input: GetWebACLForResourceRequest,
    ) -> Result<GetWebACLForResourceResponse, RusotoError<GetWebACLForResourceError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of managed rule groups that are available for you to use. This list includes all AWS managed rule groups and the AWS Marketplace managed rule groups that you&#39;re subscribed to.</p></p>
    async fn list_available_managed_rule_groups(
        &self,
        input: ListAvailableManagedRuleGroupsRequest,
    ) -> Result<
        ListAvailableManagedRuleGroupsResponse,
        RusotoError<ListAvailableManagedRuleGroupsError>,
    >;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of <a>IPSetSummary</a> objects for the IP sets that you manage.</p></p>
    async fn list_ip_sets(
        &self,
        input: ListIPSetsRequest,
    ) -> Result<ListIPSetsResponse, RusotoError<ListIPSetsError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of your <a>LoggingConfiguration</a> objects.</p></p>
    async fn list_logging_configurations(
        &self,
        input: ListLoggingConfigurationsRequest,
    ) -> Result<ListLoggingConfigurationsResponse, RusotoError<ListLoggingConfigurationsError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of <a>RegexPatternSetSummary</a> objects for the regex pattern sets that you manage.</p></p>
    async fn list_regex_pattern_sets(
        &self,
        input: ListRegexPatternSetsRequest,
    ) -> Result<ListRegexPatternSetsResponse, RusotoError<ListRegexPatternSetsError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of the Amazon Resource Names (ARNs) for the regional resources that are associated with the specified web ACL. If you want the list of AWS CloudFront resources, use the AWS CloudFront call <code>ListDistributionsByWebACLId</code>. </p></p>
    async fn list_resources_for_web_acl(
        &self,
        input: ListResourcesForWebACLRequest,
    ) -> Result<ListResourcesForWebACLResponse, RusotoError<ListResourcesForWebACLError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of <a>RuleGroupSummary</a> objects for the rule groups that you manage. </p></p>
    async fn list_rule_groups(
        &self,
        input: ListRuleGroupsRequest,
    ) -> Result<ListRuleGroupsResponse, RusotoError<ListRuleGroupsError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the <a>TagInfoForResource</a> for the specified resource. </p></p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of <a>WebACLSummary</a> objects for the web ACLs that you manage.</p></p>
    async fn list_web_ac_ls(
        &self,
        input: ListWebACLsRequest,
    ) -> Result<ListWebACLsResponse, RusotoError<ListWebACLsError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Enables the specified <a>LoggingConfiguration</a>, to start logging from a web ACL, according to the configuration provided.</p> <p>You can access information about all traffic that AWS WAF inspects using the following steps:</p> <ol> <li> <p>Create an Amazon Kinesis Data Firehose. </p> <p>Create the data firehose with a PUT source and in the region that you are operating. If you are capturing logs for Amazon CloudFront, always create the firehose in US East (N. Virginia). </p> <note> <p>Do not create the data firehose using a <code>Kinesis stream</code> as your source.</p> </note> </li> <li> <p>Associate that firehose to your web ACL using a <code>PutLoggingConfiguration</code> request.</p> </li> </ol> <p>When you successfully enable logging using a <code>PutLoggingConfiguration</code> request, AWS WAF will create a service linked role with the necessary permissions to write logs to the Amazon Kinesis Data Firehose. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/logging.html">Logging Web ACL Traffic Information</a> in the <i>AWS WAF Developer Guide</i>.</p></p>
    async fn put_logging_configuration(
        &self,
        input: PutLoggingConfigurationRequest,
    ) -> Result<PutLoggingConfigurationResponse, RusotoError<PutLoggingConfigurationError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Associates tags with the specified AWS resource. Tags are key:value pairs that you can associate with AWS resources. For example, the tag key might be &quot;customer&quot; and the tag value might be &quot;companyA.&quot; You can specify one or more tags to add to each container. You can add up to 50 tags to each AWS resource.</p></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Disassociates tags from an AWS resource. Tags are key:value pairs that you can associate with AWS resources. For example, the tag key might be &quot;customer&quot; and the tag value might be &quot;companyA.&quot; You can specify one or more tags to add to each container. You can add up to 50 tags to each AWS resource.</p></p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Updates the specified <a>IPSet</a>.</p></p>
    async fn update_ip_set(
        &self,
        input: UpdateIPSetRequest,
    ) -> Result<UpdateIPSetResponse, RusotoError<UpdateIPSetError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Updates the specified <a>RegexPatternSet</a>.</p></p>
    async fn update_regex_pattern_set(
        &self,
        input: UpdateRegexPatternSetRequest,
    ) -> Result<UpdateRegexPatternSetResponse, RusotoError<UpdateRegexPatternSetError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Updates the specified <a>RuleGroup</a>.</p> <p> A rule group defines a collection of rules to inspect and control web requests that you can use in a <a>WebACL</a>. When you create a rule group, you define an immutable capacity limit. If you update a rule group, you must stay within the capacity. This allows others to reuse the rule group with confidence in its capacity requirements. </p></p>
    async fn update_rule_group(
        &self,
        input: UpdateRuleGroupRequest,
    ) -> Result<UpdateRuleGroupResponse, RusotoError<UpdateRuleGroupError>>;

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Updates the specified <a>WebACL</a>.</p> <p> A Web ACL defines a collection of rules to use to inspect and control web requests. Each rule has an action defined (allow, block, or count) for requests that match the statement of the rule. In the Web ACL, you assign a default action to take (allow, block) for any request that does not match any of the rules. The rules in a Web ACL can be a combination of the types <a>Rule</a>, <a>RuleGroup</a>, and managed rule group. You can associate a Web ACL with one or more AWS resources to protect. The resources can be Amazon CloudFront, an Amazon API Gateway API, or an Application Load Balancer. </p></p>
    async fn update_web_acl(
        &self,
        input: UpdateWebACLRequest,
    ) -> Result<UpdateWebACLResponse, RusotoError<UpdateWebACLError>>;
}
/// A client for the WAFV2 API.
#[derive(Clone)]
pub struct WafV2Client {
    client: Client,
    region: region::Region,
}

impl WafV2Client {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> WafV2Client {
        WafV2Client {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> WafV2Client
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        WafV2Client {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> WafV2Client {
        WafV2Client { client, region }
    }
}

#[async_trait]
impl WafV2 for WafV2Client {
    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Associates a Web ACL with a regional application resource, to protect the resource. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>For AWS CloudFront, you can associate the Web ACL by providing the <code>Id</code> of the <a>WebACL</a> to the CloudFront API call <code>UpdateDistribution</code>. For information, see <a href="https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_UpdateDistribution.html">UpdateDistribution</a>.</p></p>
    async fn associate_web_acl(
        &self,
        input: AssociateWebACLRequest,
    ) -> Result<AssociateWebACLResponse, RusotoError<AssociateWebACLError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.AssociateWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AssociateWebACLResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateWebACLError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Returns the web ACL capacity unit (WCU) requirements for a specified scope and set of rules. You can use this to check the capacity requirements for the rules you want to use in a <a>RuleGroup</a> or <a>WebACL</a>. </p> <p>AWS WAF uses WCUs to calculate and control the operating resources that are used to run your rules, rule groups, and web ACLs. AWS WAF calculates capacity differently for each rule type, to reflect the relative cost of each rule. Simple rules that cost little to run use fewer WCUs than more complex rules that use more processing power. Rule group capacity is fixed at creation, which helps users plan their web ACL WCU usage when they use a rule group. The WCU limit for web ACLs is 1,500. </p></p>
    async fn check_capacity(
        &self,
        input: CheckCapacityRequest,
    ) -> Result<CheckCapacityResponse, RusotoError<CheckCapacityError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.CheckCapacity");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CheckCapacityResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CheckCapacityError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Creates an <a>IPSet</a>, which you use to identify web requests that originate from specific IP addresses or ranges of IP addresses. For example, if you&#39;re receiving a lot of requests from a ranges of IP addresses, you can configure AWS WAF to block them using an IPSet that lists those IP addresses. </p></p>
    async fn create_ip_set(
        &self,
        input: CreateIPSetRequest,
    ) -> Result<CreateIPSetResponse, RusotoError<CreateIPSetError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.CreateIPSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateIPSetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIPSetError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Creates a <a>RegexPatternSet</a> per the specifications provided.</p></p>
    async fn create_regex_pattern_set(
        &self,
        input: CreateRegexPatternSetRequest,
    ) -> Result<CreateRegexPatternSetResponse, RusotoError<CreateRegexPatternSetError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.CreateRegexPatternSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateRegexPatternSetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRegexPatternSetError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Creates a <a>RuleGroup</a> per the specifications provided. </p> <p> A rule group defines a collection of rules to inspect and control web requests that you can use in a <a>WebACL</a>. When you create a rule group, you define an immutable capacity limit. If you update a rule group, you must stay within the capacity. This allows others to reuse the rule group with confidence in its capacity requirements. </p></p>
    async fn create_rule_group(
        &self,
        input: CreateRuleGroupRequest,
    ) -> Result<CreateRuleGroupResponse, RusotoError<CreateRuleGroupError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.CreateRuleGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateRuleGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRuleGroupError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Creates a <a>WebACL</a> per the specifications provided.</p> <p> A Web ACL defines a collection of rules to use to inspect and control web requests. Each rule has an action defined (allow, block, or count) for requests that match the statement of the rule. In the Web ACL, you assign a default action to take (allow, block) for any request that does not match any of the rules. The rules in a Web ACL can be a combination of the types <a>Rule</a>, <a>RuleGroup</a>, and managed rule group. You can associate a Web ACL with one or more AWS resources to protect. The resources can be Amazon CloudFront, an Amazon API Gateway API, or an Application Load Balancer. </p></p>
    async fn create_web_acl(
        &self,
        input: CreateWebACLRequest,
    ) -> Result<CreateWebACLResponse, RusotoError<CreateWebACLError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.CreateWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateWebACLResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateWebACLError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Deletes the specified <a>IPSet</a>. </p></p>
    async fn delete_ip_set(
        &self,
        input: DeleteIPSetRequest,
    ) -> Result<DeleteIPSetResponse, RusotoError<DeleteIPSetError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.DeleteIPSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteIPSetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteIPSetError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Deletes the <a>LoggingConfiguration</a> from the specified web ACL.</p></p>
    async fn delete_logging_configuration(
        &self,
        input: DeleteLoggingConfigurationRequest,
    ) -> Result<DeleteLoggingConfigurationResponse, RusotoError<DeleteLoggingConfigurationError>>
    {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.DeleteLoggingConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteLoggingConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLoggingConfigurationError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Deletes the specified <a>RegexPatternSet</a>.</p></p>
    async fn delete_regex_pattern_set(
        &self,
        input: DeleteRegexPatternSetRequest,
    ) -> Result<DeleteRegexPatternSetResponse, RusotoError<DeleteRegexPatternSetError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.DeleteRegexPatternSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteRegexPatternSetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRegexPatternSetError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Deletes the specified <a>RuleGroup</a>.</p></p>
    async fn delete_rule_group(
        &self,
        input: DeleteRuleGroupRequest,
    ) -> Result<DeleteRuleGroupResponse, RusotoError<DeleteRuleGroupError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.DeleteRuleGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteRuleGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRuleGroupError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Deletes the specified <a>WebACL</a>.</p></p>
    async fn delete_web_acl(
        &self,
        input: DeleteWebACLRequest,
    ) -> Result<DeleteWebACLResponse, RusotoError<DeleteWebACLError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.DeleteWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteWebACLResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteWebACLError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Provides high-level information for a managed rule group, including descriptions of the rules. </p></p>
    async fn describe_managed_rule_group(
        &self,
        input: DescribeManagedRuleGroupRequest,
    ) -> Result<DescribeManagedRuleGroupResponse, RusotoError<DescribeManagedRuleGroupError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.DescribeManagedRuleGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeManagedRuleGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeManagedRuleGroupError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Disassociates a Web ACL from a regional application resource. A regional application can be an Application Load Balancer (ALB) or an API Gateway stage. </p> <p>For AWS CloudFront, you can disassociate the Web ACL by providing an empty <code>WebACLId</code> in the CloudFront API call <code>UpdateDistribution</code>. For information, see <a href="https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_UpdateDistribution.html">UpdateDistribution</a>.</p></p>
    async fn disassociate_web_acl(
        &self,
        input: DisassociateWebACLRequest,
    ) -> Result<DisassociateWebACLResponse, RusotoError<DisassociateWebACLError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.DisassociateWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateWebACLResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateWebACLError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the specified <a>IPSet</a>.</p></p>
    async fn get_ip_set(
        &self,
        input: GetIPSetRequest,
    ) -> Result<GetIPSetResponse, RusotoError<GetIPSetError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.GetIPSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetIPSetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetIPSetError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Returns the <a>LoggingConfiguration</a> for the specified web ACL.</p></p>
    async fn get_logging_configuration(
        &self,
        input: GetLoggingConfigurationRequest,
    ) -> Result<GetLoggingConfigurationResponse, RusotoError<GetLoggingConfigurationError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.GetLoggingConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLoggingConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetLoggingConfigurationError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the keys that are currently blocked by a rate-based rule. The maximum number of managed keys that can be blocked for a single rate-based rule is 10,000. If more than 10,000 addresses exceed the rate limit, those with the highest rates are blocked.</p></p>
    async fn get_rate_based_statement_managed_keys(
        &self,
        input: GetRateBasedStatementManagedKeysRequest,
    ) -> Result<
        GetRateBasedStatementManagedKeysResponse,
        RusotoError<GetRateBasedStatementManagedKeysError>,
    > {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_20190729.GetRateBasedStatementManagedKeys",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRateBasedStatementManagedKeysResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRateBasedStatementManagedKeysError::from_response(
                response,
            ))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the specified <a>RegexPatternSet</a>.</p></p>
    async fn get_regex_pattern_set(
        &self,
        input: GetRegexPatternSetRequest,
    ) -> Result<GetRegexPatternSetResponse, RusotoError<GetRegexPatternSetError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.GetRegexPatternSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRegexPatternSetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRegexPatternSetError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the specified <a>RuleGroup</a>.</p></p>
    async fn get_rule_group(
        &self,
        input: GetRuleGroupRequest,
    ) -> Result<GetRuleGroupResponse, RusotoError<GetRuleGroupError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.GetRuleGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetRuleGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRuleGroupError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Gets detailed information about a specified number of requests--a sample--that AWS WAF randomly selects from among the first 5,000 requests that your AWS resource received during a time range that you choose. You can specify a sample size of up to 500 requests, and you can specify any time range in the previous three hours.</p> <p> <code>GetSampledRequests</code> returns a time range, which is usually the time range that you specified. However, if your resource (such as a CloudFront distribution) received 5,000 requests before the specified time range elapsed, <code>GetSampledRequests</code> returns an updated time range. This new time range indicates the actual period during which AWS WAF selected the requests in the sample.</p></p>
    async fn get_sampled_requests(
        &self,
        input: GetSampledRequestsRequest,
    ) -> Result<GetSampledRequestsResponse, RusotoError<GetSampledRequestsError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.GetSampledRequests");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSampledRequestsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetSampledRequestsError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the specified <a>WebACL</a>.</p></p>
    async fn get_web_acl(
        &self,
        input: GetWebACLRequest,
    ) -> Result<GetWebACLResponse, RusotoError<GetWebACLError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.GetWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetWebACLResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetWebACLError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the <a>WebACL</a> for the specified resource. </p></p>
    async fn get_web_acl_for_resource(
        &self,
        input: GetWebACLForResourceRequest,
    ) -> Result<GetWebACLForResourceResponse, RusotoError<GetWebACLForResourceError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.GetWebACLForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetWebACLForResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetWebACLForResourceError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of managed rule groups that are available for you to use. This list includes all AWS managed rule groups and the AWS Marketplace managed rule groups that you&#39;re subscribed to.</p></p>
    async fn list_available_managed_rule_groups(
        &self,
        input: ListAvailableManagedRuleGroupsRequest,
    ) -> Result<
        ListAvailableManagedRuleGroupsResponse,
        RusotoError<ListAvailableManagedRuleGroupsError>,
    > {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSWAF_20190729.ListAvailableManagedRuleGroups",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAvailableManagedRuleGroupsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListAvailableManagedRuleGroupsError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of <a>IPSetSummary</a> objects for the IP sets that you manage.</p></p>
    async fn list_ip_sets(
        &self,
        input: ListIPSetsRequest,
    ) -> Result<ListIPSetsResponse, RusotoError<ListIPSetsError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.ListIPSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListIPSetsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListIPSetsError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of your <a>LoggingConfiguration</a> objects.</p></p>
    async fn list_logging_configurations(
        &self,
        input: ListLoggingConfigurationsRequest,
    ) -> Result<ListLoggingConfigurationsResponse, RusotoError<ListLoggingConfigurationsError>>
    {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.ListLoggingConfigurations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListLoggingConfigurationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListLoggingConfigurationsError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of <a>RegexPatternSetSummary</a> objects for the regex pattern sets that you manage.</p></p>
    async fn list_regex_pattern_sets(
        &self,
        input: ListRegexPatternSetsRequest,
    ) -> Result<ListRegexPatternSetsResponse, RusotoError<ListRegexPatternSetsError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.ListRegexPatternSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRegexPatternSetsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListRegexPatternSetsError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of the Amazon Resource Names (ARNs) for the regional resources that are associated with the specified web ACL. If you want the list of AWS CloudFront resources, use the AWS CloudFront call <code>ListDistributionsByWebACLId</code>. </p></p>
    async fn list_resources_for_web_acl(
        &self,
        input: ListResourcesForWebACLRequest,
    ) -> Result<ListResourcesForWebACLResponse, RusotoError<ListResourcesForWebACLError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.ListResourcesForWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListResourcesForWebACLResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListResourcesForWebACLError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of <a>RuleGroupSummary</a> objects for the rule groups that you manage. </p></p>
    async fn list_rule_groups(
        &self,
        input: ListRuleGroupsRequest,
    ) -> Result<ListRuleGroupsResponse, RusotoError<ListRuleGroupsError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.ListRuleGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListRuleGroupsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListRuleGroupsError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves the <a>TagInfoForResource</a> for the specified resource. </p></p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Retrieves an array of <a>WebACLSummary</a> objects for the web ACLs that you manage.</p></p>
    async fn list_web_ac_ls(
        &self,
        input: ListWebACLsRequest,
    ) -> Result<ListWebACLsResponse, RusotoError<ListWebACLsError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.ListWebACLs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListWebACLsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListWebACLsError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Enables the specified <a>LoggingConfiguration</a>, to start logging from a web ACL, according to the configuration provided.</p> <p>You can access information about all traffic that AWS WAF inspects using the following steps:</p> <ol> <li> <p>Create an Amazon Kinesis Data Firehose. </p> <p>Create the data firehose with a PUT source and in the region that you are operating. If you are capturing logs for Amazon CloudFront, always create the firehose in US East (N. Virginia). </p> <note> <p>Do not create the data firehose using a <code>Kinesis stream</code> as your source.</p> </note> </li> <li> <p>Associate that firehose to your web ACL using a <code>PutLoggingConfiguration</code> request.</p> </li> </ol> <p>When you successfully enable logging using a <code>PutLoggingConfiguration</code> request, AWS WAF will create a service linked role with the necessary permissions to write logs to the Amazon Kinesis Data Firehose. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/logging.html">Logging Web ACL Traffic Information</a> in the <i>AWS WAF Developer Guide</i>.</p></p>
    async fn put_logging_configuration(
        &self,
        input: PutLoggingConfigurationRequest,
    ) -> Result<PutLoggingConfigurationResponse, RusotoError<PutLoggingConfigurationError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.PutLoggingConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<PutLoggingConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutLoggingConfigurationError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Associates tags with the specified AWS resource. Tags are key:value pairs that you can associate with AWS resources. For example, the tag key might be &quot;customer&quot; and the tag value might be &quot;companyA.&quot; You can specify one or more tags to add to each container. You can add up to 50 tags to each AWS resource.</p></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Disassociates tags from an AWS resource. Tags are key:value pairs that you can associate with AWS resources. For example, the tag key might be &quot;customer&quot; and the tag value might be &quot;companyA.&quot; You can specify one or more tags to add to each container. You can add up to 50 tags to each AWS resource.</p></p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Updates the specified <a>IPSet</a>.</p></p>
    async fn update_ip_set(
        &self,
        input: UpdateIPSetRequest,
    ) -> Result<UpdateIPSetResponse, RusotoError<UpdateIPSetError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.UpdateIPSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateIPSetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateIPSetError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Updates the specified <a>RegexPatternSet</a>.</p></p>
    async fn update_regex_pattern_set(
        &self,
        input: UpdateRegexPatternSetRequest,
    ) -> Result<UpdateRegexPatternSetResponse, RusotoError<UpdateRegexPatternSetError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.UpdateRegexPatternSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateRegexPatternSetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRegexPatternSetError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Updates the specified <a>RuleGroup</a>.</p> <p> A rule group defines a collection of rules to inspect and control web requests that you can use in a <a>WebACL</a>. When you create a rule group, you define an immutable capacity limit. If you update a rule group, you must stay within the capacity. This allows others to reuse the rule group with confidence in its capacity requirements. </p></p>
    async fn update_rule_group(
        &self,
        input: UpdateRuleGroupRequest,
    ) -> Result<UpdateRuleGroupResponse, RusotoError<UpdateRuleGroupError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.UpdateRuleGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateRuleGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRuleGroupError::from_response(response))
        }
    }

    /// <p><note> <p>This is the latest version of <b>AWS WAF</b>, named AWS WAFV2, released in November, 2019. For information, including how to migrate your AWS WAF resources from the prior release, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p> </note> <p>Updates the specified <a>WebACL</a>.</p> <p> A Web ACL defines a collection of rules to use to inspect and control web requests. Each rule has an action defined (allow, block, or count) for requests that match the statement of the rule. In the Web ACL, you assign a default action to take (allow, block) for any request that does not match any of the rules. The rules in a Web ACL can be a combination of the types <a>Rule</a>, <a>RuleGroup</a>, and managed rule group. You can associate a Web ACL with one or more AWS resources to protect. The resources can be Amazon CloudFront, an Amazon API Gateway API, or an Application Load Balancer. </p></p>
    async fn update_web_acl(
        &self,
        input: UpdateWebACLRequest,
    ) -> Result<UpdateWebACLResponse, RusotoError<UpdateWebACLError>> {
        let mut request = SignedRequest::new("POST", "wafv2", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_20190729.UpdateWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateWebACLResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateWebACLError::from_response(response))
        }
    }
}
