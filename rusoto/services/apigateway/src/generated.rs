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
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>Access log settings, including the access log format and access log destination ARN.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AccessLogSettings {
    /// <p>The ARN of the CloudWatch Logs log group to receive access logs.</p>
    #[serde(rename = "destinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    /// <p>A single line format of the access logs of data, as specified by selected <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-mapping-template-reference.html#context-variable-reference">$context variables</a>. The format must include at least <code>$context.requestId</code>.</p>
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// <p><p>Represents an AWS account that is associated with API Gateway.</p> <div class="remarks"> <p>To view the account info, call <code>GET</code> on this resource.</p> <h4>Error Codes</h4> <p>The following exception may be thrown when the request fails.</p> <ul> <li>UnauthorizedException</li> <li>NotFoundException</li> <li>TooManyRequestsException</li> </ul> <p>For detailed error code information, including the corresponding HTTP Status Codes, see <a href="http://docs.aws.amazon.com/apigateway/api-reference/handling-errors/#api-error-codes">API Gateway Error Codes</a></p> <h4>Example: Get the information about an account.</h4> <h5>Request</h5> <pre><code>GET /account HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com X-Amz-Date: 20160531T184618Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash} </code></pre> <h5>Response</h5> <p>The successful response returns a <code>200 OK</code> status code and a payload similar to the following:</p> <pre><code>{ &quot;_links&quot;: { &quot;curies&quot;: { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/account-apigateway-{rel}.html&quot;, &quot;name&quot;: &quot;account&quot;, &quot;templated&quot;: true }, &quot;self&quot;: { &quot;href&quot;: &quot;/account&quot; }, &quot;account:update&quot;: { &quot;href&quot;: &quot;/account&quot; } }, &quot;cloudwatchRoleArn&quot;: &quot;arn:aws:iam::123456789012:role/apigAwsProxyRole&quot;, &quot;throttleSettings&quot;: { &quot;rateLimit&quot;: 500, &quot;burstLimit&quot;: 1000 } } </code></pre> <p>In addition to making the REST API call directly, you can use the AWS CLI and an AWS SDK to access this resource.</p> </div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-limits.html">API Gateway Limits</a> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/welcome.html">Developer Guide</a>, <a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/get-account.html">AWS CLI</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Account {
    /// <p>The version of the API keys used for the account.</p>
    #[serde(rename = "apiKeyVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_version: Option<String>,
    /// <p>The ARN of an Amazon CloudWatch role for the current <a>Account</a>. </p>
    #[serde(rename = "cloudwatchRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_role_arn: Option<String>,
    /// <p>A list of features supported for the account. When usage plans are enabled, the features list will include an entry of <code>"UsagePlans"</code>.</p>
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    /// <p>Specifies the API request limits configured for the current <a>Account</a>.</p>
    #[serde(rename = "throttleSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_settings: Option<ThrottleSettings>,
}

/// <p><p>A resource that can be distributed to callers for executing <a>Method</a> resources that require an API key. API keys can be mapped to any <a>Stage</a> on any <a>RestApi</a>, which indicates that the callers with the API key can make requests to that stage.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-api-keys.html">Use API Keys</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ApiKey {
    /// <p>The timestamp when the API Key was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>An AWS Marketplace customer identifier , when integrating with the AWS SaaS Marketplace.</p>
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// <p>The description of the API Key.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Specifies whether the API Key can be used by callers.</p>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The identifier of the API Key.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The timestamp when the API Key was last updated.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the API Key.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of <a>Stage</a> resources that are associated with the <a>ApiKey</a> resource.</p>
    #[serde(rename = "stageKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_keys: Option<Vec<String>>,
    /// <p>The value of the API Key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The identifier of an <a>ApiKey</a> used in a <a>UsagePlan</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ApiKeyIds {
    /// <p>A list of all the <a>ApiKey</a> identifiers.</p>
    #[serde(rename = "ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// <p>A list of warning messages.</p>
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p><p>Represents a collection of API keys as represented by an <a>ApiKeys</a> resource.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-api-keys.html">Use API Keys</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ApiKeys {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ApiKey>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>A list of warning messages logged during the import of API keys when the <code>failOnWarnings</code> option is set to true.</p>
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p>API stage name of the associated API stage in a usage plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiStage {
    /// <p>API Id of the associated API stage in a usage plan.</p>
    #[serde(rename = "apiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>API stage name of the associated API stage in a usage plan.</p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// <p>Map containing method level throttling information for API stage in a usage plan.</p>
    #[serde(rename = "throttle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<::std::collections::HashMap<String, ThrottleSettings>>,
}

/// <p><p>Represents an authorization layer for methods. If enabled on a method, API Gateway will activate the authorizer when a client calls the method.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/use-custom-authorizer.html">Enable custom authorization</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Authorizer {
    /// <p>Optional customer-defined field, used in Swagger imports and exports without functional impact.</p>
    #[serde(rename = "authType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null.</p>
    #[serde(rename = "authorizerCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials: Option<String>,
    /// <p>The TTL in seconds of cached authorizer results. If it equals 0, authorization caching is disabled. If it is greater than 0, API Gateway will cache authorizer responses. If this field is not set, the default value is 300. The maximum value is 3600, or 1 hour.</p>
    #[serde(rename = "authorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>Specifies the authorizer's Uniform Resource Identifier (URI). For <code>TOKEN</code> or <code>REQUEST</code> authorizers, this must be a well-formed Lambda function URI, for example, <code>arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account_id}:function:{lambda_function_name}/invocations</code>. In general, the URI has this form <code>arn:aws:apigateway:{region}:lambda:path/{service_api}</code>, where <code>{region}</code> is the same as the region hosting the Lambda function, <code>path</code> indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial <code>/</code>. For Lambda functions, this is usually of the form <code>/2015-03-31/functions/[FunctionARN]/invocations</code>.</p>
    #[serde(rename = "authorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identifier for the authorizer resource.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The identity source for which authorization is requested. <ul><li>For a <code>TOKEN</code> or <code>COGNITO_USER_POOLS</code> authorizer, this is required and specifies the request header mapping expression for the custom header holding the authorization token submitted by the client. For example, if the token header name is <code>Auth</code>, the header mapping expression is <code>method.request.header.Auth</code>.</li><li>For the <code>REQUEST</code> authorizer, this is required when authorization caching is enabled. The value is a comma-separated string of one or more mapping expressions of the specified request parameters. For example, if an <code>Auth</code> header, a <code>Name</code> query string parameter are defined as identity sources, this value is <code>method.request.header.Auth, method.request.querystring.Name</code>. These parameters will be used to derive the authorization caching key and to perform runtime validation of the <code>REQUEST</code> authorizer by verifying all of the identity-related request parameters are present, not null and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda function, otherwise, it returns a 401 Unauthorized response without calling the Lambda function. The valid value is a string of comma-separated mapping expressions of the specified request parameters. When the authorization caching is not enabled, this property is optional.</li></ul></p>
    #[serde(rename = "identitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<String>,
    /// <p>A validation expression for the incoming identity token. For <code>TOKEN</code> authorizers, this value is a regular expression. API Gateway will match the <code>aud</code> field of the incoming token from the client against the specified regular expression. It will invoke the authorizer's Lambda function when there is a match. Otherwise, it will return a 401 Unauthorized response without calling the Lambda function. The validation expression does not apply to the <code>REQUEST</code> authorizer.</p>
    #[serde(rename = "identityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>[Required] The name of the authorizer.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of the Amazon Cognito user pool ARNs for the <code>COGNITO_USER_POOLS</code> authorizer. Each element is of this format: <code>arn:aws:cognito-idp:{region}:{account_id}:userpool/{user_pool_id}</code>. For a <code>TOKEN</code> or <code>REQUEST</code> authorizer, this is not defined. </p>
    #[serde(rename = "providerARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_ar_ns: Option<Vec<String>>,
    /// <p>The authorizer type. Valid values are <code>TOKEN</code> for a Lambda function using a single authorization token submitted in a custom header, <code>REQUEST</code> for a Lambda function using incoming request parameters, and <code>COGNITO_USER_POOLS</code> for using an Amazon Cognito user pool.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p><p>Represents a collection of <a>Authorizer</a> resources.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/use-custom-authorizer.html">Enable custom authorization</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Authorizers {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Authorizer>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>Represents the base path that callers of the API must provide as part of the URL after the domain name.</p> <div class="remarks">A custom domain name plus a <code>BasePathMapping</code> specification identifies a deployed <a>RestApi</a> in a given stage of the owner <a>Account</a>.</div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-custom-domains.html">Use Custom Domain Names</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BasePathMapping {
    /// <p>The base path name that callers of the API must provide as part of the URL after the domain name.</p>
    #[serde(rename = "basePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    /// <p>The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_api_id: Option<String>,
    /// <p>The name of the associated stage.</p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

/// <p><p>Represents a collection of <a>BasePathMapping</a> resources.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-custom-domains.html">Use Custom Domain Names</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BasePathMappings {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BasePathMapping>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Configuration settings of a canary deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanarySettings {
    /// <p>The ID of the canary deployment.</p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The percent (0-100) of traffic diverted to a canary deployment.</p>
    #[serde(rename = "percentTraffic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_traffic: Option<f64>,
    /// <p>Stage variables overridden for a canary release deployment, including new stage variables introduced in the canary. These stage variables are represented as a string-to-string map between stage variable names and their values.</p>
    #[serde(rename = "stageVariableOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variable_overrides: Option<::std::collections::HashMap<String, String>>,
    /// <p>A Boolean flag to indicate whether the canary deployment uses the stage cache or not.</p>
    #[serde(rename = "useStageCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stage_cache: Option<bool>,
}

/// <p><p>Represents a client certificate used to configure client-side SSL authentication while sending requests to the integration endpoint.</p> <div class="remarks">Client certificates are used to authenticate an API by the backend server. To authenticate an API client (or user), use IAM roles and policies, a custom <a>Authorizer</a> or an Amazon Cognito user pool.</div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/getting-started-client-side-ssl-authentication.html">Use Client-Side Certificate</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ClientCertificate {
    /// <p>The identifier of the client certificate.</p>
    #[serde(rename = "clientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>The timestamp when the client certificate was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the client certificate.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The timestamp when the client certificate will expire.</p>
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    /// <p>The PEM-encoded public key of the client certificate, which can be used to configure certificate authentication in the integration endpoint .</p>
    #[serde(rename = "pemEncodedCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem_encoded_certificate: Option<String>,
}

/// <p><p>Represents a collection of <a>ClientCertificate</a> resources.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/getting-started-client-side-ssl-authentication.html">Use Client-Side Certificate</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ClientCertificates {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ClientCertificate>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Request to create an <a>ApiKey</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateApiKeyRequest {
    /// <p>An AWS Marketplace customer identifier , when integrating with the AWS SaaS Marketplace.</p>
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// <p>The description of the <a>ApiKey</a>.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Specifies whether the <a>ApiKey</a> can be used by callers.</p>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Specifies whether (<code>true</code>) or not (<code>false</code>) the key identifier is distinct from the created API key value.</p>
    #[serde(rename = "generateDistinctId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_distinct_id: Option<bool>,
    /// <p>The name of the <a>ApiKey</a>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>DEPRECATED FOR USAGE PLANS - Specifies stages associated with the API key.</p>
    #[serde(rename = "stageKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_keys: Option<Vec<StageKey>>,
    /// <p>Specifies a value of the API key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Request to add a new <a>Authorizer</a> to an existing <a>RestApi</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAuthorizerRequest {
    /// <p>Optional customer-defined field, used in Swagger imports and exports without functional impact.</p>
    #[serde(rename = "authType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null.</p>
    #[serde(rename = "authorizerCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials: Option<String>,
    /// <p>The TTL in seconds of cached authorizer results. If it equals 0, authorization caching is disabled. If it is greater than 0, API Gateway will cache authorizer responses. If this field is not set, the default value is 300. The maximum value is 3600, or 1 hour.</p>
    #[serde(rename = "authorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>Specifies the authorizer's Uniform Resource Identifier (URI). For <code>TOKEN</code> or <code>REQUEST</code> authorizers, this must be a well-formed Lambda function URI, for example, <code>arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account_id}:function:{lambda_function_name}/invocations</code>. In general, the URI has this form <code>arn:aws:apigateway:{region}:lambda:path/{service_api}</code>, where <code>{region}</code> is the same as the region hosting the Lambda function, <code>path</code> indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial <code>/</code>. For Lambda functions, this is usually of the form <code>/2015-03-31/functions/[FunctionARN]/invocations</code>.</p>
    #[serde(rename = "authorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identity source for which authorization is requested. <ul><li>For a <code>TOKEN</code> or <code>COGNITO_USER_POOLS</code> authorizer, this is required and specifies the request header mapping expression for the custom header holding the authorization token submitted by the client. For example, if the token header name is <code>Auth</code>, the header mapping expression is <code>method.request.header.Auth</code>.</li><li>For the <code>REQUEST</code> authorizer, this is required when authorization caching is enabled. The value is a comma-separated string of one or more mapping expressions of the specified request parameters. For example, if an <code>Auth</code> header, a <code>Name</code> query string parameter are defined as identity sources, this value is <code>method.request.header.Auth, method.request.querystring.Name</code>. These parameters will be used to derive the authorization caching key and to perform runtime validation of the <code>REQUEST</code> authorizer by verifying all of the identity-related request parameters are present, not null and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda function, otherwise, it returns a 401 Unauthorized response without calling the Lambda function. The valid value is a string of comma-separated mapping expressions of the specified request parameters. When the authorization caching is not enabled, this property is optional.</li></ul></p>
    #[serde(rename = "identitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<String>,
    /// <p>A validation expression for the incoming identity token. For <code>TOKEN</code> authorizers, this value is a regular expression. API Gateway will match the <code>aud</code> field of the incoming token from the client against the specified regular expression. It will invoke the authorizer's Lambda function when there is a match. Otherwise, it will return a 401 Unauthorized response without calling the Lambda function. The validation expression does not apply to the <code>REQUEST</code> authorizer.</p>
    #[serde(rename = "identityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>[Required] The name of the authorizer.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A list of the Amazon Cognito user pool ARNs for the <code>COGNITO_USER_POOLS</code> authorizer. Each element is of this format: <code>arn:aws:cognito-idp:{region}:{account_id}:userpool/{user_pool_id}</code>. For a <code>TOKEN</code> or <code>REQUEST</code> authorizer, this is not defined. </p>
    #[serde(rename = "providerARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_ar_ns: Option<Vec<String>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The authorizer type. Valid values are <code>TOKEN</code> for a Lambda function using a single authorization token submitted in a custom header, <code>REQUEST</code> for a Lambda function using incoming request parameters, and <code>COGNITO_USER_POOLS</code> for using an Amazon Cognito user pool.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Requests API Gateway to create a new <a>BasePathMapping</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateBasePathMappingRequest {
    /// <p>The base path name that callers of the API must provide as part of the URL after the domain name. This value must be unique for all of the mappings across a single API. Leave this blank if you do not want callers to specify a base path name after the domain name.</p>
    #[serde(rename = "basePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    /// <p>[Required] The domain name of the <a>BasePathMapping</a> resource to create.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The name of the API's stage that you want to use for this mapping. Leave this blank if you do not want callers to explicitly specify the stage name after any base path name.</p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

/// <p>Requests API Gateway to create a <a>Deployment</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeploymentRequest {
    /// <p>Enables a cache cluster for the <a>Stage</a> resource specified in the input.</p>
    #[serde(rename = "cacheClusterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_enabled: Option<bool>,
    /// <p>Specifies the cache cluster size for the <a>Stage</a> resource specified in the input, if a cache cluster is enabled.</p>
    #[serde(rename = "cacheClusterSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_size: Option<String>,
    /// <p>The input configuration for the canary deployment when the deployment is a canary release deployment. </p>
    #[serde(rename = "canarySettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_settings: Option<DeploymentCanarySettings>,
    /// <p>The description for the <a>Deployment</a> resource to create.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The description of the <a>Stage</a> resource for the <a>Deployment</a> resource to create.</p>
    #[serde(rename = "stageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_description: Option<String>,
    /// <p>The name of the <a>Stage</a> resource for the <a>Deployment</a> resource to create.</p>
    #[serde(rename = "stageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    /// <p>A map that defines the stage variables for the <a>Stage</a> resource that is associated with the new deployment. Variable names can have alphanumeric and underscore characters, and the values must match <code>[A-Za-z0-9-._~:/?#&amp;=,]+</code>.</p>
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Creates a new documentation part of a given API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDocumentationPartRequest {
    /// <p>[Required] The location of the targeted API entity of the to-be-created documentation part.</p>
    #[serde(rename = "location")]
    pub location: DocumentationPartLocation,
    /// <p>[Required] The new documentation content map of the targeted API entity. Enclosed key-value pairs are API-specific, but only Swagger-compliant key-value pairs can be exported and, hence, published.</p>
    #[serde(rename = "properties")]
    pub properties: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Creates a new documentation version of a given API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDocumentationVersionRequest {
    /// <p>A description about the new documentation snapshot.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>[Required] The version identifier of the new snapshot.</p>
    #[serde(rename = "documentationVersion")]
    pub documentation_version: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The stage name to be associated with the new documentation snapshot.</p>
    #[serde(rename = "stageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

/// <p>A request to create a new domain name.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDomainNameRequest {
    /// <p>The reference to an AWS-managed certificate that will be used by edge-optimized endpoint for this domain name. AWS Certificate Manager is the only supported source.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>[Deprecated] The body of the server certificate that will be used by edge-optimized endpoint for this domain name provided by your certificate authority.</p>
    #[serde(rename = "certificateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_body: Option<String>,
    /// <p>[Deprecated] The intermediate certificates and optionally the root certificate, one after the other without any blank lines, used by an edge-optimized endpoint for this domain name. If you include the root certificate, your certificate chain must start with intermediate certificates and end with the root certificate. Use the intermediate certificates that were provided by your certificate authority. Do not include any intermediaries that are not in the chain of trust path.</p>
    #[serde(rename = "certificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    /// <p>The user-friendly name of the certificate that will be used by edge-optimized endpoint for this domain name.</p>
    #[serde(rename = "certificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    /// <p>[Deprecated] Your edge-optimized endpoint's domain name certificate's private key.</p>
    #[serde(rename = "certificatePrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_private_key: Option<String>,
    /// <p>[Required] The name of the <a>DomainName</a> resource.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p>The endpoint configuration of this <a>DomainName</a> showing the endpoint types of the domain name. </p>
    #[serde(rename = "endpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    /// <p>The reference to an AWS-managed certificate that will be used by regional endpoint for this domain name. AWS Certificate Manager is the only supported source.</p>
    #[serde(rename = "regionalCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_certificate_arn: Option<String>,
    /// <p>The user-friendly name of the certificate that will be used by regional endpoint for this domain name.</p>
    #[serde(rename = "regionalCertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_certificate_name: Option<String>,
}

/// <p>Request to add a new <a>Model</a> to an existing <a>RestApi</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateModelRequest {
    /// <p>[Required] The content-type for the model.</p>
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// <p>The description of the model.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>[Required] The name of the model. Must be alphanumeric.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>[Required] The <a>RestApi</a> identifier under which the <a>Model</a> will be created.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The schema for the model. For <code>application/json</code> models, this should be <a href="https://tools.ietf.org/html/draft-zyp-json-schema-04" target="_blank">JSON schema draft 4</a> model.</p>
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

/// <p>Creates a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRequestValidatorRequest {
    /// <p>The name of the to-be-created <a>RequestValidator</a>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>A Boolean flag to indicate whether to validate request body according to the configured model schema for the method (<code>true</code>) or not (<code>false</code>).</p>
    #[serde(rename = "validateRequestBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_request_body: Option<bool>,
    /// <p>A Boolean flag to indicate whether to validate request parameters, <code>true</code>, or not <code>false</code>.</p>
    #[serde(rename = "validateRequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_request_parameters: Option<bool>,
}

/// <p>Requests API Gateway to create a <a>Resource</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateResourceRequest {
    /// <p>[Required] The parent resource's identifier.</p>
    #[serde(rename = "parentId")]
    pub parent_id: String,
    /// <p>The last path segment for this resource.</p>
    #[serde(rename = "pathPart")]
    pub path_part: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>The POST Request to add a new <a>RestApi</a> resource to your collection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRestApiRequest {
    /// <p>The source of the API key for metering requests according to a usage plan. Valid values are: <ul><li><code>HEADER</code> to read the API key from the <code>X-API-Key</code> header of a request. </li><li><code>AUTHORIZER</code> to read the API key from the <code>UsageIdentifierKey</code> from a custom authorizer.</li></ul> </p>
    #[serde(rename = "apiKeySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_source: Option<String>,
    /// <p>The list of binary media types supported by the <a>RestApi</a>. By default, the <a>RestApi</a> supports only UTF-8-encoded text payloads.</p>
    #[serde(rename = "binaryMediaTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_media_types: Option<Vec<String>>,
    /// <p>The ID of the <a>RestApi</a> that you want to clone from.</p>
    #[serde(rename = "cloneFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_from: Option<String>,
    /// <p>The description of the <a>RestApi</a>.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The endpoint configuration of this <a>RestApi</a> showing the endpoint types of the API. </p>
    #[serde(rename = "endpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    /// <p>A nullable integer that is used to enable compression (with non-negative between 0 and 10485760 (10M) bytes, inclusive) or disable compression (with a null value) on an API. When compression is enabled, compression or decompression is not applied on the payload if the payload size is smaller than this value. Setting it to zero allows compression for any payload size.</p>
    #[serde(rename = "minimumCompressionSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_compression_size: Option<i64>,
    /// <p>[Required] The name of the <a>RestApi</a>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A stringified JSON policy document that applies to this RestApi regardless of the caller and <a>Method</a> configuration.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Requests API Gateway to create a <a>Stage</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateStageRequest {
    /// <p>Whether cache clustering is enabled for the stage.</p>
    #[serde(rename = "cacheClusterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_enabled: Option<bool>,
    /// <p>The stage's cache cluster size.</p>
    #[serde(rename = "cacheClusterSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_size: Option<String>,
    /// <p>The canary deployment settings of this stage.</p>
    #[serde(rename = "canarySettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_settings: Option<CanarySettings>,
    /// <p>[Required] The identifier of the <a>Deployment</a> resource for the <a>Stage</a> resource.</p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p>The description of the <a>Stage</a> resource.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The version of the associated API documentation.</p>
    #[serde(rename = "documentationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_version: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The name for the <a>Stage</a> resource.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
    /// <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A map that defines the stage variables for the new <a>Stage</a> resource. Variable names can have alphanumeric and underscore characters, and the values must match <code>[A-Za-z0-9-._~:/?#&amp;=,]+</code>.</p>
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The POST request to create a usage plan key for adding an existing API key to a usage plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUsagePlanKeyRequest {
    /// <p>[Required] The identifier of a <a>UsagePlanKey</a> resource for a plan customer.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>[Required] The type of a <a>UsagePlanKey</a> resource for a plan customer.</p>
    #[serde(rename = "keyType")]
    pub key_type: String,
    /// <p>[Required] The Id of the <a>UsagePlan</a> resource representing the usage plan containing the to-be-created <a>UsagePlanKey</a> resource representing a plan customer.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>The POST request to create a usage plan with the name, description, throttle limits and quota limits, as well as the associated API stages, specified in the payload.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUsagePlanRequest {
    /// <p>The associated API stages of the usage plan.</p>
    #[serde(rename = "apiStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_stages: Option<Vec<ApiStage>>,
    /// <p>The description of the usage plan.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>[Required] The name of the usage plan.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The quota of the usage plan.</p>
    #[serde(rename = "quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<QuotaSettings>,
    /// <p>The throttling limits of the usage plan.</p>
    #[serde(rename = "throttle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<ThrottleSettings>,
}

/// <p>Creates a VPC link, under the caller's account in a selected region, in an asynchronous operation that typically takes 2-4 minutes to complete and become operational. The caller must have permissions to create and update VPC Endpoint services.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateVpcLinkRequest {
    /// <p>The description of the VPC link.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>[Required] The name used to label and identify the VPC link.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>[Required] The ARNs of network load balancers of the VPC targeted by the VPC link. The network load balancers must be owned by the same AWS account of the API owner.</p>
    #[serde(rename = "targetArns")]
    pub target_arns: Vec<String>,
}

/// <p>A request to delete the <a>ApiKey</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApiKeyRequest {
    /// <p>[Required] The identifier of the <a>ApiKey</a> resource to be deleted.</p>
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

/// <p>Request to delete an existing <a>Authorizer</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAuthorizerRequest {
    /// <p>[Required] The identifier of the <a>Authorizer</a> resource.</p>
    #[serde(rename = "authorizerId")]
    pub authorizer_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A request to delete the <a>BasePathMapping</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBasePathMappingRequest {
    /// <p>[Required] The base path name of the <a>BasePathMapping</a> resource to delete.</p>
    #[serde(rename = "basePath")]
    pub base_path: String,
    /// <p>[Required] The domain name of the <a>BasePathMapping</a> resource to delete.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

/// <p>A request to delete the <a>ClientCertificate</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteClientCertificateRequest {
    /// <p>[Required] The identifier of the <a>ClientCertificate</a> resource to be deleted.</p>
    #[serde(rename = "clientCertificateId")]
    pub client_certificate_id: String,
}

/// <p>Requests API Gateway to delete a <a>Deployment</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDeploymentRequest {
    /// <p>[Required] The identifier of the <a>Deployment</a> resource to delete.</p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Deletes an existing documentation part of an API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDocumentationPartRequest {
    /// <p>[Required] The identifier of the to-be-deleted documentation part.</p>
    #[serde(rename = "documentationPartId")]
    pub documentation_part_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Deletes an existing documentation version of an API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDocumentationVersionRequest {
    /// <p>[Required] The version identifier of a to-be-deleted documentation snapshot.</p>
    #[serde(rename = "documentationVersion")]
    pub documentation_version: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A request to delete the <a>DomainName</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDomainNameRequest {
    /// <p>[Required] The name of the <a>DomainName</a> resource to be deleted.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

/// <p>Clears any customization of a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a> and resets it with the default settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGatewayResponseRequest {
    /// <p>[Required] <p>The response type of the associated <a>GatewayResponse</a>. Valid values are <ul><li>ACCESS_DENIED</li><li>API_CONFIGURATION_ERROR</li><li>AUTHORIZER_FAILURE</li><li> AUTHORIZER_CONFIGURATION_ERROR</li><li>BAD_REQUEST_PARAMETERS</li><li>BAD_REQUEST_BODY</li><li>DEFAULT_4XX</li><li>DEFAULT_5XX</li><li>EXPIRED_TOKEN</li><li>INVALID_SIGNATURE</li><li>INTEGRATION_FAILURE</li><li>INTEGRATION_TIMEOUT</li><li>INVALID_API_KEY</li><li>MISSING_AUTHENTICATION_TOKEN</li><li> QUOTA_EXCEEDED</li><li>REQUEST_TOO_LARGE</li><li>RESOURCE_NOT_FOUND</li><li>THROTTLED</li><li>UNAUTHORIZED</li><li>UNSUPPORTED_MEDIA_TYPE</li></ul> </p></p>
    #[serde(rename = "responseType")]
    pub response_type: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Represents a delete integration request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIntegrationRequest {
    /// <p>[Required] Specifies a delete integration request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] Specifies a delete integration request's resource identifier.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Represents a delete integration response request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIntegrationResponseRequest {
    /// <p>[Required] Specifies a delete integration response request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] Specifies a delete integration response request's resource identifier.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] Specifies a delete integration response request's status code.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to delete an existing <a>Method</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteMethodRequest {
    /// <p>[Required] The HTTP verb of the <a>Method</a> resource.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>Method</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A request to delete an existing <a>MethodResponse</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteMethodResponseRequest {
    /// <p>[Required] The HTTP verb of the <a>Method</a> resource.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>MethodResponse</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The status code identifier for the <a>MethodResponse</a> resource.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to delete an existing model in an existing <a>RestApi</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteModelRequest {
    /// <p>[Required] The name of the model to delete.</p>
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Deletes a specified <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRequestValidatorRequest {
    /// <p>[Required] The identifier of the <a>RequestValidator</a> to be deleted.</p>
    #[serde(rename = "requestValidatorId")]
    pub request_validator_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to delete a <a>Resource</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteResourceRequest {
    /// <p>[Required] The identifier of the <a>Resource</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to delete the specified API from your collection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRestApiRequest {
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Requests API Gateway to delete a <a>Stage</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteStageRequest {
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The name of the <a>Stage</a> resource to delete.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>The DELETE request to delete a usage plan key and remove the underlying API key from the associated usage plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUsagePlanKeyRequest {
    /// <p>[Required] The Id of the <a>UsagePlanKey</a> resource to be deleted.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>[Required] The Id of the <a>UsagePlan</a> resource representing the usage plan containing the to-be-deleted <a>UsagePlanKey</a> resource representing a plan customer.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>The DELETE request to delete a usage plan of a given plan Id.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUsagePlanRequest {
    /// <p>[Required] The Id of the to-be-deleted usage plan.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>Deletes an existing <a>VpcLink</a> of a specified identifier.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVpcLinkRequest {
    /// <p>[Required] The identifier of the <a>VpcLink</a>. It is used in an <a>Integration</a> to reference this <a>VpcLink</a>.</p>
    #[serde(rename = "vpcLinkId")]
    pub vpc_link_id: String,
}

/// <p><p>An immutable representation of a <a>RestApi</a> resource that can be called by users using <a>Stages</a>. A deployment must be associated with a <a>Stage</a> for it to be callable over the Internet.</p> <div class="remarks">To create a deployment, call <code>POST</code> on the <a>Deployments</a> resource of a <a>RestApi</a>. To view, update, or delete a deployment, call <code>GET</code>, <code>PATCH</code>, or <code>DELETE</code> on the specified deployment resource (<code>/restapis/{restapi<em>id}/deployments/{deployment</em>id}</code>).</div> <div class="seeAlso"><a>RestApi</a>, <a>Deployments</a>, <a>Stage</a>, <a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/get-deployment.html">AWS CLI</a>, <a href="https://aws.amazon.com/tools/">AWS SDKs</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Deployment {
    /// <p>A summary of the <a>RestApi</a> at the date and time that the deployment resource was created.</p>
    #[serde(rename = "apiSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_summary: Option<
        ::std::collections::HashMap<String, ::std::collections::HashMap<String, MethodSnapshot>>,
    >,
    /// <p>The date and time that the deployment resource was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description for the deployment resource.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier for the deployment resource.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>The input configuration for a canary deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeploymentCanarySettings {
    /// <p>The percentage (0.0-100.0) of traffic routed to the canary deployment.</p>
    #[serde(rename = "percentTraffic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_traffic: Option<f64>,
    /// <p>A stage variable overrides used for the canary release deployment. They can override existing stage variables or add new stage variables for the canary release deployment. These stage variables are represented as a string-to-string map between stage variable names and their values.</p>
    #[serde(rename = "stageVariableOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variable_overrides: Option<::std::collections::HashMap<String, String>>,
    /// <p>A Boolean flag to indicate whether the canary release deployment uses the stage cache or not.</p>
    #[serde(rename = "useStageCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stage_cache: Option<bool>,
}

/// <p><p>Represents a collection resource that contains zero or more references to your existing deployments, and links that guide you on how to interact with your collection. The collection offers a paginated view of the contained deployments.</p> <div class="remarks">To create a new deployment of a <a>RestApi</a>, make a <code>POST</code> request against this resource. To view, update, or delete an existing deployment, make a <code>GET</code>, <code>PATCH</code>, or <code>DELETE</code> request, respectively, on a specified <a>Deployment</a> resource.</div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-deploy-api.html">Deploying an API</a>, <a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/get-deployment.html">AWS CLI</a>, <a href="https://aws.amazon.com/tools/">AWS SDKs</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Deployments {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Deployment>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>A documentation part for a targeted API entity.</p> <div class="remarks"> <p>A documentation part consists of a content map (<code>properties</code>) and a target (<code>location</code>). The target specifies an API entity to which the documentation content applies. The supported API entity types are <code>API</code>, <code>AUTHORIZER</code>, <code>MODEL</code>, <code>RESOURCE</code>, <code>METHOD</code>, <code>PATH<em>PARAMETER</code>, <code>QUERY</em>PARAMETER</code>, <code>REQUEST<em>HEADER</code>, <code>REQUEST</em>BODY</code>, <code>RESPONSE</code>, <code>RESPONSE<em>HEADER</code>, and <code>RESPONSE</em>BODY</code>. Valid <code>location</code> fields depend on the API entity type. All valid fields are not required.</p> <p>The content map is a JSON string of API-specific key-value pairs. Although an API can use any shape for the content map, only the Swagger-compliant documentation fields will be injected into the associated API entity definition in the exported Swagger definition file.</p></div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api.html">Documenting an API</a>, <a>DocumentationParts</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DocumentationPart {
    /// <p>The <a>DocumentationPart</a> identifier, generated by API Gateway when the <code>DocumentationPart</code> is created.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The location of the API entity to which the documentation applies. Valid fields depend on the targeted API entity type. All the valid location fields are not required. If not explicitly specified, a valid location field is treated as a wildcard and associated documentation content may be inherited by matching entities, unless overridden.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<DocumentationPartLocation>,
    /// <p>A content map of API-specific key-value pairs describing the targeted API entity. The map must be encoded as a JSON string, e.g., <code>"{ \"description\": \"The API does ...\" }"</code>. Only Swagger-compliant documentation-related fields from the <literal>properties</literal> map are exported and, hence, published as part of the API entity definitions, while the original documentation parts are exported in a Swagger extension of <code>x-amazon-apigateway-documentation</code>.</p>
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
}

/// <p><p>A collection of the imported <a>DocumentationPart</a> identifiers.</p> <div class="remarks">This is used to return the result when documentation parts in an external (e.g., Swagger) file are imported into API Gateway</div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api.html">Documenting an API</a>, <a href="http://docs.aws.amazon.com/apigateway/api-reference/link-relation/documentationpart-import/">documentationpart:import</a>, <a>DocumentationPart</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DocumentationPartIds {
    /// <p>A list of the returned documentation part identifiers.</p>
    #[serde(rename = "ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// <p>A list of warning messages reported during import of documentation parts.</p>
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p>Specifies the target API entity to which the documentation applies.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentationPartLocation {
    /// <p>The HTTP verb of a method. It is a valid field for the API entity types of <code>METHOD</code>, <code>PATH_PARAMETER</code>, <code>QUERY_PARAMETER</code>, <code>REQUEST_HEADER</code>, <code>REQUEST_BODY</code>, <code>RESPONSE</code>, <code>RESPONSE_HEADER</code>, and <code>RESPONSE_BODY</code>. The default value is <code>*</code> for any method. When an applicable child entity inherits the content of an entity of the same type with more general specifications of the other <code>location</code> attributes, the child entity's <code>method</code> attribute must match that of the parent entity exactly.</p>
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// <p>The name of the targeted API entity. It is a valid and required field for the API entity types of <code>AUTHORIZER</code>, <code>MODEL</code>, <code>PATH_PARAMETER</code>, <code>QUERY_PARAMETER</code>, <code>REQUEST_HEADER</code>, <code>REQUEST_BODY</code> and <code>RESPONSE_HEADER</code>. It is an invalid field for any other entity type.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The URL path of the target. It is a valid field for the API entity types of <code>RESOURCE</code>, <code>METHOD</code>, <code>PATH_PARAMETER</code>, <code>QUERY_PARAMETER</code>, <code>REQUEST_HEADER</code>, <code>REQUEST_BODY</code>, <code>RESPONSE</code>, <code>RESPONSE_HEADER</code>, and <code>RESPONSE_BODY</code>. The default value is <code>/</code> for the root resource. When an applicable child entity inherits the content of another entity of the same type with more general specifications of the other <code>location</code> attributes, the child entity's <code>path</code> attribute must match that of the parent entity as a prefix.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The HTTP status code of a response. It is a valid field for the API entity types of <code>RESPONSE</code>, <code>RESPONSE_HEADER</code>, and <code>RESPONSE_BODY</code>. The default value is <code>*</code> for any status code. When an applicable child entity inherits the content of an entity of the same type with more general specifications of the other <code>location</code> attributes, the child entity's <code>statusCode</code> attribute must match that of the parent entity exactly.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    /// <p>[Required] The type of API entity to which the documentation content applies. Valid values are <code>API</code>, <code>AUTHORIZER</code>, <code>MODEL</code>, <code>RESOURCE</code>, <code>METHOD</code>, <code>PATH_PARAMETER</code>, <code>QUERY_PARAMETER</code>, <code>REQUEST_HEADER</code>, <code>REQUEST_BODY</code>, <code>RESPONSE</code>, <code>RESPONSE_HEADER</code>, and <code>RESPONSE_BODY</code>. Content inheritance does not apply to any entity of the <code>API</code>, <code>AUTHORIZER</code>, <code>METHOD</code>, <code>MODEL</code>, <code>REQUEST_BODY</code>, or <code>RESOURCE</code> type.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p><p>The collection of documentation parts of an API.</p> <div class="remarks"/> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api.html">Documenting an API</a>, <a>DocumentationPart</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DocumentationParts {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DocumentationPart>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>A snapshot of the documentation of an API.</p> <div class="remarks"><p>Publishing API documentation involves creating a documentation version associated with an API stage and exporting the versioned documentation to an external (e.g., Swagger) file.</p></div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api.html">Documenting an API</a>, <a>DocumentationPart</a>, <a>DocumentationVersions</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DocumentationVersion {
    /// <p>The date when the API documentation snapshot is created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the API documentation snapshot.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The version identifier of the API documentation snapshot.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p><p>The collection of documentation snapshots of an API. </p> <div class="remarks"><p>Use the <a>DocumentationVersions</a> to manage documentation snapshots associated with various API stages.</p></div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api.html">Documenting an API</a>, <a>DocumentationPart</a>, <a>DocumentationVersion</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DocumentationVersions {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DocumentationVersion>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>Represents a custom domain name as a user-friendly host name of an API (<a>RestApi</a>).</p> <div class="Remarks"> <p>When you deploy an API, API Gateway creates a default host name for the API. This default API host name is of the <code>{restapi-id}.execute-api.{region}.amazonaws.com</code> format. With the default host name, you can access the API&#39;s root resource with the URL of <code>https://{restapi-id}.execute-api.{region}.amazonaws.com/{stage}/</code>. When you set up a custom domain name of <code>apis.example.com</code> for this API, you can then access the same resource using the URL of the <code>https://apis.examples.com/myApi</code>, where <code>myApi</code> is the base path mapping (<a>BasePathMapping</a>) of your API under the custom domain name. </p> </div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-custom-domains.html">Set a Custom Host Name for an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DomainName {
    /// <p>The reference to an AWS-managed certificate that will be used by edge-optimized endpoint for this domain name. AWS Certificate Manager is the only supported source.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The name of the certificate that will be used by edge-optimized endpoint for this domain name.</p>
    #[serde(rename = "certificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    /// <p>The timestamp when the certificate that was used by edge-optimized endpoint for this domain name was uploaded.</p>
    #[serde(rename = "certificateUploadDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_upload_date: Option<f64>,
    /// <p>The domain name of the Amazon CloudFront distribution associated with this custom domain name for an edge-optimized endpoint. You set up this association when adding a DNS record pointing the custom domain name to this distribution name. For more information about CloudFront distributions, see the <a href="http://aws.amazon.com/documentation/cloudfront/" target="_blank">Amazon CloudFront documentation</a>.</p>
    #[serde(rename = "distributionDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_domain_name: Option<String>,
    /// <p>The region-agnostic Amazon Route 53 Hosted Zone ID of the edge-optimized endpoint. The valid value is <code>Z2FDTNDATAQYW2</code> for all the regions. For more information, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-regional-api-custom-domain-create.html">Set up a Regional Custom Domain Name</a> and <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#apigateway_region">AWS Regions and Endpoints for API Gateway</a>. </p>
    #[serde(rename = "distributionHostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_hosted_zone_id: Option<String>,
    /// <p>The custom domain name as an API host name, for example, <code>my-api.example.com</code>.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The endpoint configuration of this <a>DomainName</a> showing the endpoint types of the domain name. </p>
    #[serde(rename = "endpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    /// <p>The reference to an AWS-managed certificate that will be used for validating the regional domain name. AWS Certificate Manager is the only supported source.</p>
    #[serde(rename = "regionalCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_certificate_arn: Option<String>,
    /// <p>The name of the certificate that will be used for validating the regional domain name.</p>
    #[serde(rename = "regionalCertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_certificate_name: Option<String>,
    /// <p>The domain name associated with the regional endpoint for this custom domain name. You set up this association by adding a DNS record that points the custom domain name to this regional domain name. The regional domain name is returned by API Gateway when you create a regional endpoint.</p>
    #[serde(rename = "regionalDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_domain_name: Option<String>,
    /// <p>The region-specific Amazon Route 53 Hosted Zone ID of the regional endpoint. For more information, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-regional-api-custom-domain-create.html">Set up a Regional Custom Domain Name</a> and <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#apigateway_region">AWS Regions and Endpoints for API Gateway</a>. </p>
    #[serde(rename = "regionalHostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_hosted_zone_id: Option<String>,
}

/// <p><p>Represents a collection of <a>DomainName</a> resources.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-custom-domains.html">Use Client-Side Certificate</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DomainNames {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DomainName>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>The endpoint configuration to indicate the types of endpoints an API (<a>RestApi</a>) or its custom domain name (<a>DomainName</a>) has. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointConfiguration {
    /// <p>A list of endpoint types of an API (<a>RestApi</a>) or its custom domain name (<a>DomainName</a>). For an edge-optimized API and its custom domain name, the endpoint type is <code>"EDGE"</code>. For a regional API and its custom domain name, the endpoint type is <code>REGIONAL</code>. For a private API, the endpoint type is <code>PRIVATE</code>.</p>
    #[serde(rename = "types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

/// <p>The binary blob response to <a>GetExport</a>, which contains the generated SDK.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExportResponse {
    /// <p>The binary blob response to <a>GetExport</a>, which contains the export.</p>
    pub body: Option<Vec<u8>>,
    /// <p>The content-disposition header value in the HTTP response.</p>
    pub content_disposition: Option<String>,
    /// <p>The content-type header value in the HTTP response. This will correspond to a valid 'accept' type in the request.</p>
    pub content_type: Option<String>,
}

/// <p>Request to flush authorizer cache entries on a specified stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FlushStageAuthorizersCacheRequest {
    /// <p>The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The name of the stage to flush.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>Requests API Gateway to flush a stage's cache.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FlushStageCacheRequest {
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The name of the stage to flush its cache.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p><p>A gateway response of a given response type and status code, with optional response parameters and mapping templates.</p> <div class="remarks"> For more information about valid gateway response types, see <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/supported-gateway-response-types.html">Gateway Response Types Supported by API Gateway</a> <div class="example"> <h4>Example: Get a Gateway Response of a given response type</h4> <h5>Request</h5> <p>This example shows how to get a gateway response of the <code>MISSING<em>AUTHENTICATION</em>TOKEN</code> type.</p> <pre><code>GET /restapis/o81lxisefl/gatewayresponses/MISSING<em>AUTHENTICATION</em>TOKEN HTTP/1.1 Host: beta-apigateway.us-east-1.amazonaws.com Content-Type: application/json X-Amz-Date: 20170503T202516Z Authorization: AWS4-HMAC-SHA256 Credential={access-key-id}/20170503/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature=1b52460e3159c1a26cff29093855d50ea141c1c5b937528fecaf60f51129697a Cache-Control: no-cache Postman-Token: 3b2a1ce9-c848-2e26-2e2f-9c2caefbed45 </code></pre> <p>The response type is specified as a URL path.</p> <h5>Response</h5> <p>The successful operation returns the <code>200 OK</code> status code and a payload similar to the following:</p> <pre><code>{ &quot;</em>links&quot;: { &quot;curies&quot;: { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-gatewayresponse-{rel}.html&quot;, &quot;name&quot;: &quot;gatewayresponse&quot;, &quot;templated&quot;: true }, &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/MISSING<em>AUTHENTICATION</em>TOKEN&quot; }, &quot;gatewayresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/MISSING<em>AUTHENTICATION</em>TOKEN&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response<em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/MISSING</em>AUTHENTICATION<em>TOKEN&quot; } }, &quot;defaultResponse&quot;: false, &quot;responseParameters&quot;: { &quot;gatewayresponse.header.x-request-path&quot;: &quot;method.request.path.petId&quot;, &quot;gatewayresponse.header.Access-Control-Allow-Origin&quot;: &quot;&apos;a.b.c&apos;&quot;, &quot;gatewayresponse.header.x-request-query&quot;: &quot;method.request.querystring.q&quot;, &quot;gatewayresponse.header.x-request-header&quot;: &quot;method.request.header.Accept&quot; }, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{\n &quot;message&quot;: $context.error.messageString,\n &quot;type&quot;: &quot;$context.error.responseType&quot;,\n &quot;stage&quot;: &quot;$context.stage&quot;,\n &quot;resourcePath&quot;: &quot;$context.resourcePath&quot;,\n &quot;stageVariables.a&quot;: &quot;$stageVariables.a&quot;,\n &quot;statusCode&quot;: &quot;&apos;404&apos;&quot;\n}&quot; }, &quot;responseType&quot;: &quot;MISSING</em>AUTHENTICATION_TOKEN&quot;, &quot;statusCode&quot;: &quot;404&quot; }</code></pre> <p></p> </div> </div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/customize-gateway-responses.html">Customize Gateway Responses</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GatewayResponse {
    /// <p>A Boolean flag to indicate whether this <a>GatewayResponse</a> is the default gateway response (<code>true</code>) or not (<code>false</code>). A default gateway response is one generated by API Gateway without any customization by an API developer. </p>
    #[serde(rename = "defaultResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_response: Option<bool>,
    /// <p>Response parameters (paths, query strings and headers) of the <a>GatewayResponse</a> as a string-to-string map of key-value pairs.</p>
    #[serde(rename = "responseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Response templates of the <a>GatewayResponse</a> as a string-to-string map of key-value pairs.</p>
    #[serde(rename = "responseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The response type of the associated <a>GatewayResponse</a>. Valid values are <ul><li>ACCESS_DENIED</li><li>API_CONFIGURATION_ERROR</li><li>AUTHORIZER_FAILURE</li><li> AUTHORIZER_CONFIGURATION_ERROR</li><li>BAD_REQUEST_PARAMETERS</li><li>BAD_REQUEST_BODY</li><li>DEFAULT_4XX</li><li>DEFAULT_5XX</li><li>EXPIRED_TOKEN</li><li>INVALID_SIGNATURE</li><li>INTEGRATION_FAILURE</li><li>INTEGRATION_TIMEOUT</li><li>INVALID_API_KEY</li><li>MISSING_AUTHENTICATION_TOKEN</li><li> QUOTA_EXCEEDED</li><li>REQUEST_TOO_LARGE</li><li>RESOURCE_NOT_FOUND</li><li>THROTTLED</li><li>UNAUTHORIZED</li><li>UNSUPPORTED_MEDIA_TYPE</li></ul> </p>
    #[serde(rename = "responseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<String>,
    /// <p>The HTTP status code for this <a>GatewayResponse</a>.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

/// <p><p>The collection of the <a>GatewayResponse</a> instances of a <a>RestApi</a> as a <code>responseType</code>-to-<a>GatewayResponse</a> object map of key-value pairs. As such, pagination is not supported for querying this collection.</p> <div class="remarks"> For more information about valid gateway response types, see <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/supported-gateway-response-types.html">Gateway Response Types Supported by API Gateway</a> <div class="example"> <h4>Example: Get the collection of gateway responses of an API</h4> <h5>Request</h5> <p>This example request shows how to retrieve the <a>GatewayResponses</a> collection from an API.</p> <pre><code>GET /restapis/o81lxisefl/gatewayresponses HTTP/1.1 Host: beta-apigateway.us-east-1.amazonaws.com Content-Type: application/json X-Amz-Date: 20170503T220604Z Authorization: AWS4-HMAC-SHA256 Credential={access-key-id}/20170503/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature=59b42fe54a76a5de8adf2c67baa6d39206f8e9ad49a1d77ccc6a5da3103a398a Cache-Control: no-cache Postman-Token: 5637af27-dc29-fc5c-9dfe-0645d52cb515 </code></pre> <p></p> <h5>Response</h5> <p>The successful operation returns the <code>200 OK</code> status code and a payload similar to the following:</p> <pre><code>{ &quot;</em>links&quot;: { &quot;curies&quot;: { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-gatewayresponse-{rel}.html&quot;, &quot;name&quot;: &quot;gatewayresponse&quot;, &quot;templated&quot;: true }, &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses&quot; }, &quot;first&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses&quot; }, &quot;gatewayresponse:by-type&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response<em>type}&quot;, &quot;templated&quot;: true }, &quot;item&quot;: [ { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INTEGRATION</em>FAILURE&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/RESOURCE<em>NOT</em>FOUND&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/REQUEST<em>TOO</em>LARGE&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/THROTTLED&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/UNSUPPORTED<em>MEDIA</em>TYPE&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/AUTHORIZER<em>CONFIGURATION</em>ERROR&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/DEFAULT<em>5XX&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/DEFAULT</em>4XX&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/BAD<em>REQUEST</em>PARAMETERS&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/BAD<em>REQUEST</em>BODY&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/EXPIRED<em>TOKEN&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/ACCESS</em>DENIED&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INVALID<em>API</em>KEY&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/UNAUTHORIZED&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/API<em>CONFIGURATION</em>ERROR&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/QUOTA<em>EXCEEDED&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INTEGRATION</em>TIMEOUT&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/MISSING<em>AUTHENTICATION</em>TOKEN&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INVALID<em>SIGNATURE&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/AUTHORIZER</em>FAILURE&quot; } ] }, &quot;<em>embedded&quot;: { &quot;item&quot;: [ { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INTEGRATION<em>FAILURE&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INTEGRATION<em>FAILURE&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;INTEGRATION</em>FAILURE&quot;, &quot;statusCode&quot;: &quot;504&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/RESOURCE</em>NOT<em>FOUND&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/RESOURCE<em>NOT</em>FOUND&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;RESOURCE<em>NOT</em>FOUND&quot;, &quot;statusCode&quot;: &quot;404&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/REQUEST</em>TOO<em>LARGE&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/REQUEST<em>TOO</em>LARGE&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;REQUEST<em>TOO</em>LARGE&quot;, &quot;statusCode&quot;: &quot;413&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/THROTTLED&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/THROTTLED&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;THROTTLED&quot;, &quot;statusCode&quot;: &quot;429&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/UNSUPPORTED</em>MEDIA<em>TYPE&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/UNSUPPORTED<em>MEDIA</em>TYPE&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;UNSUPPORTED<em>MEDIA</em>TYPE&quot;, &quot;statusCode&quot;: &quot;415&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/AUTHORIZER</em>CONFIGURATION<em>ERROR&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/AUTHORIZER<em>CONFIGURATION</em>ERROR&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;AUTHORIZER<em>CONFIGURATION</em>ERROR&quot;, &quot;statusCode&quot;: &quot;500&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/DEFAULT</em>5XX&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response<em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/DEFAULT</em>5XX&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;DEFAULT<em>5XX&quot; }, { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/DEFAULT<em>4XX&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/DEFAULT<em>4XX&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;DEFAULT</em>4XX&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/BAD</em>REQUEST<em>PARAMETERS&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/BAD<em>REQUEST</em>PARAMETERS&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;BAD<em>REQUEST</em>PARAMETERS&quot;, &quot;statusCode&quot;: &quot;400&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/BAD</em>REQUEST<em>BODY&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/BAD<em>REQUEST</em>BODY&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;BAD<em>REQUEST</em>BODY&quot;, &quot;statusCode&quot;: &quot;400&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/EXPIRED</em>TOKEN&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response<em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/EXPIRED</em>TOKEN&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;EXPIRED<em>TOKEN&quot;, &quot;statusCode&quot;: &quot;403&quot; }, { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/ACCESS<em>DENIED&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/ACCESS<em>DENIED&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;ACCESS</em>DENIED&quot;, &quot;statusCode&quot;: &quot;403&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INVALID</em>API<em>KEY&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INVALID<em>API</em>KEY&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;INVALID<em>API</em>KEY&quot;, &quot;statusCode&quot;: &quot;403&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/UNAUTHORIZED&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/UNAUTHORIZED&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;UNAUTHORIZED&quot;, &quot;statusCode&quot;: &quot;401&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/API</em>CONFIGURATION<em>ERROR&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/API<em>CONFIGURATION</em>ERROR&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;API<em>CONFIGURATION</em>ERROR&quot;, &quot;statusCode&quot;: &quot;500&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/QUOTA</em>EXCEEDED&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response<em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/QUOTA</em>EXCEEDED&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;QUOTA<em>EXCEEDED&quot;, &quot;statusCode&quot;: &quot;429&quot; }, { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INTEGRATION<em>TIMEOUT&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INTEGRATION<em>TIMEOUT&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;INTEGRATION</em>TIMEOUT&quot;, &quot;statusCode&quot;: &quot;504&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/MISSING</em>AUTHENTICATION<em>TOKEN&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/MISSING<em>AUTHENTICATION</em>TOKEN&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;MISSING<em>AUTHENTICATION</em>TOKEN&quot;, &quot;statusCode&quot;: &quot;403&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INVALID</em>SIGNATURE&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response<em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INVALID</em>SIGNATURE&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;INVALID<em>SIGNATURE&quot;, &quot;statusCode&quot;: &quot;403&quot; }, { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/AUTHORIZER<em>FAILURE&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/AUTHORIZER<em>FAILURE&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;AUTHORIZER</em>FAILURE&quot;, &quot;statusCode&quot;: &quot;500&quot; } ] } }</code></pre> <p></p> </div> </div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/customize-gateway-responses.html">Customize Gateway Responses</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GatewayResponses {
    /// <p>Returns the entire collection, because of no pagination support.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<GatewayResponse>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>A request to generate a <a>ClientCertificate</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateClientCertificateRequest {
    /// <p>The description of the <a>ClientCertificate</a>.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// <p>Requests API Gateway to get information about the current <a>Account</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAccountRequest {}

/// <p>A request to get information about the current <a>ApiKey</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApiKeyRequest {
    /// <p>[Required] The identifier of the <a>ApiKey</a> resource.</p>
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// <p>A boolean flag to specify whether (<code>true</code>) or not (<code>false</code>) the result contains the key value.</p>
    #[serde(rename = "includeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_value: Option<bool>,
}

/// <p>A request to get information about the current <a>ApiKeys</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApiKeysRequest {
    /// <p>The identifier of a customer in AWS Marketplace or an external system, such as a developer portal.</p>
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// <p>A boolean flag to specify whether (<code>true</code>) or not (<code>false</code>) the result contains key values.</p>
    #[serde(rename = "includeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_values: Option<bool>,
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of queried API keys.</p>
    #[serde(rename = "nameQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_query: Option<String>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Request to describe an existing <a>Authorizer</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAuthorizerRequest {
    /// <p>[Required] The identifier of the <a>Authorizer</a> resource.</p>
    #[serde(rename = "authorizerId")]
    pub authorizer_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to describe an existing <a>Authorizers</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAuthorizersRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to describe a <a>BasePathMapping</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBasePathMappingRequest {
    /// <p>[Required] The base path name that callers of the API must provide as part of the URL after the domain name. This value must be unique for all of the mappings across a single API. Leave this blank if you do not want callers to specify any base path name after the domain name.</p>
    #[serde(rename = "basePath")]
    pub base_path: String,
    /// <p>[Required] The domain name of the <a>BasePathMapping</a> resource to be described.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

/// <p>A request to get information about a collection of <a>BasePathMapping</a> resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBasePathMappingsRequest {
    /// <p>[Required] The domain name of a <a>BasePathMapping</a> resource.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>A request to get information about the current <a>ClientCertificate</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetClientCertificateRequest {
    /// <p>[Required] The identifier of the <a>ClientCertificate</a> resource to be described.</p>
    #[serde(rename = "clientCertificateId")]
    pub client_certificate_id: String,
}

/// <p>A request to get information about a collection of <a>ClientCertificate</a> resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetClientCertificatesRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Requests API Gateway to get information about a <a>Deployment</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeploymentRequest {
    /// <p>[Required] The identifier of the <a>Deployment</a> resource to get information about.</p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p>A query parameter to retrieve the specified embedded resources of the returned <a>Deployment</a> resource in the response. In a REST API call, this <code>embed</code> parameter value is a list of comma-separated strings, as in <code>GET /restapis/{restapi_id}/deployments/{deployment_id}?embed=var1,var2</code>. The SDK and other platform-dependent libraries might use a different format for the list. Currently, this request supports only retrieval of the embedded API summary this way. Hence, the parameter value must be a single-valued list containing only the <code>"apisummary"</code> string. For example, <code>GET /restapis/{restapi_id}/deployments/{deployment_id}?embed=apisummary</code>.</p>
    #[serde(rename = "embed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Vec<String>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Requests API Gateway to get information about a <a>Deployments</a> collection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeploymentsRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets a specified documentation part of a given API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDocumentationPartRequest {
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "documentationPartId")]
    pub documentation_part_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets the documentation parts of an API. The result may be filtered by the type, name, or path of API entities (targets).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDocumentationPartsRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The status of the API documentation parts to retrieve. Valid values are <code>DOCUMENTED</code> for retrieving <a>DocumentationPart</a> resources with content and <code>UNDOCUMENTED</code> for <a>DocumentationPart</a> resources without content.</p>
    #[serde(rename = "locationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_status: Option<String>,
    /// <p>The name of API entities of the to-be-retrieved documentation parts.</p>
    #[serde(rename = "nameQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_query: Option<String>,
    /// <p>The path of API entities of the to-be-retrieved documentation parts.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The type of API entities of the to-be-retrieved documentation parts. </p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Gets a documentation snapshot of an API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDocumentationVersionRequest {
    /// <p>[Required] The version identifier of the to-be-retrieved documentation snapshot.</p>
    #[serde(rename = "documentationVersion")]
    pub documentation_version: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets the documentation versions of an API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDocumentationVersionsRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to get the name of a <a>DomainName</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDomainNameRequest {
    /// <p>[Required] The name of the <a>DomainName</a> resource.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

/// <p>Request to describe a collection of <a>DomainName</a> resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDomainNamesRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Request a new export of a <a>RestApi</a> for a particular <a>Stage</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetExportRequest {
    /// <p>The content-type of the export, for example <code>application/json</code>. Currently <code>application/json</code> and <code>application/yaml</code> are supported for <code>exportType</code> of <code>swagger</code>. This should be specified in the <code>Accept</code> header for direct API requests.</p>
    #[serde(rename = "accepts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts: Option<String>,
    /// <p>[Required] The type of export. Currently only 'swagger' is supported.</p>
    #[serde(rename = "exportType")]
    pub export_type: String,
    /// <p>A key-value map of query string parameters that specify properties of the export, depending on the requested <code>exportType</code>. For <code>exportType</code> <code>swagger</code>, any combination of the following parameters are supported: <code>extensions='integrations'</code> or <code>extensions='apigateway'</code> will export the API with x-amazon-apigateway-integration extensions. <code>extensions='authorizers'</code> will export the API with x-amazon-apigateway-authorizer extensions. <code>postman</code> will export the API with Postman extensions, allowing for import to the Postman tool</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The name of the <a>Stage</a> that will be exported.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>Gets a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGatewayResponseRequest {
    /// <p>[Required] <p>The response type of the associated <a>GatewayResponse</a>. Valid values are <ul><li>ACCESS_DENIED</li><li>API_CONFIGURATION_ERROR</li><li>AUTHORIZER_FAILURE</li><li> AUTHORIZER_CONFIGURATION_ERROR</li><li>BAD_REQUEST_PARAMETERS</li><li>BAD_REQUEST_BODY</li><li>DEFAULT_4XX</li><li>DEFAULT_5XX</li><li>EXPIRED_TOKEN</li><li>INVALID_SIGNATURE</li><li>INTEGRATION_FAILURE</li><li>INTEGRATION_TIMEOUT</li><li>INVALID_API_KEY</li><li>MISSING_AUTHENTICATION_TOKEN</li><li> QUOTA_EXCEEDED</li><li>REQUEST_TOO_LARGE</li><li>RESOURCE_NOT_FOUND</li><li>THROTTLED</li><li>UNAUTHORIZED</li><li>UNSUPPORTED_MEDIA_TYPE</li></ul> </p></p>
    #[serde(rename = "responseType")]
    pub response_type: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets the <a>GatewayResponses</a> collection on the given <a>RestApi</a>. If an API developer has not added any definitions for gateway responses, the result will be the API Gateway-generated default <a>GatewayResponses</a> collection for the supported response types.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGatewayResponsesRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500. The <a>GatewayResponses</a> collection does not support pagination and the limit does not apply here.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set. The <a>GatewayResponse</a> collection does not support pagination and the position does not apply here.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Represents a request to get the integration configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntegrationRequest {
    /// <p>[Required] Specifies a get integration request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] Specifies a get integration request's resource identifier</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Represents a get integration response request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntegrationResponseRequest {
    /// <p>[Required] Specifies a get integration response request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] Specifies a get integration response request's resource identifier.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] Specifies a get integration response request's status code.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to describe an existing <a>Method</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMethodRequest {
    /// <p>[Required] Specifies the method request's HTTP method type.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>Method</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to describe a <a>MethodResponse</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMethodResponseRequest {
    /// <p>[Required] The HTTP verb of the <a>Method</a> resource.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>MethodResponse</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The status code for the <a>MethodResponse</a> resource.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to list information about a model in an existing <a>RestApi</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetModelRequest {
    /// <p>A query parameter of a Boolean value to resolve (<code>true</code>) all external model references and returns a flattened model schema or not (<code>false</code>) The default is <code>false</code>.</p>
    #[serde(rename = "flatten")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten: Option<bool>,
    /// <p>[Required] The name of the model as an identifier.</p>
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// <p>[Required] The <a>RestApi</a> identifier under which the <a>Model</a> exists.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to generate a sample mapping template used to transform the payload.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetModelTemplateRequest {
    /// <p>[Required] The name of the model for which to generate a template.</p>
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to list existing <a>Models</a> defined for a <a>RestApi</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetModelsRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRequestValidatorRequest {
    /// <p>[Required] The identifier of the <a>RequestValidator</a> to be retrieved.</p>
    #[serde(rename = "requestValidatorId")]
    pub request_validator_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets the <a>RequestValidators</a> collection of a given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRequestValidatorsRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to list information about a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourceRequest {
    /// <p>A query parameter to retrieve the specified resources embedded in the returned <a>Resource</a> representation in the response. This <code>embed</code> parameter value is a list of comma-separated strings. Currently, the request supports only retrieval of the embedded <a>Method</a> resources this way. The query parameter value must be a single-valued list and contain the <code>"methods"</code> string. For example, <code>GET /restapis/{restapi_id}/resources/{resource_id}?embed=methods</code>.</p>
    #[serde(rename = "embed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Vec<String>>,
    /// <p>[Required] The identifier for the <a>Resource</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to list information about a collection of resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourcesRequest {
    /// <p>A query parameter used to retrieve the specified resources embedded in the returned <a>Resources</a> resource in the response. This <code>embed</code> parameter value is a list of comma-separated strings. Currently, the request supports only retrieval of the embedded <a>Method</a> resources this way. The query parameter value must be a single-valued list and contain the <code>"methods"</code> string. For example, <code>GET /restapis/{restapi_id}/resources?embed=methods</code>.</p>
    #[serde(rename = "embed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Vec<String>>,
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>The GET request to list an existing <a>RestApi</a> defined for your collection. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRestApiRequest {
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>The GET request to list existing <a>RestApis</a> defined for your collection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRestApisRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Request a new generated client SDK for a <a>RestApi</a> and <a>Stage</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSdkRequest {
    /// <p>A string-to-string key-value map of query parameters <code>sdkType</code>-dependent properties of the SDK. For <code>sdkType</code> of <code>objectivec</code> or <code>swift</code>, a parameter named <code>classPrefix</code> is required. For <code>sdkType</code> of <code>android</code>, parameters named <code>groupId</code>, <code>artifactId</code>, <code>artifactVersion</code>, and <code>invokerPackage</code> are required. For <code>sdkType</code> of <code>java</code>, parameters named <code>serviceName</code> and <code>javaPackageName</code> are required. </p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The language for the generated SDK. Currently <code>java</code>, <code>javascript</code>, <code>android</code>, <code>objectivec</code> (for iOS), <code>swift</code> (for iOS), and <code>ruby</code> are supported.</p>
    #[serde(rename = "sdkType")]
    pub sdk_type: String,
    /// <p>[Required] The name of the <a>Stage</a> that the SDK will use.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>Get an <a>SdkType</a> instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSdkTypeRequest {
    /// <p>[Required] The identifier of the queried <a>SdkType</a> instance.</p>
    #[serde(rename = "id")]
    pub id: String,
}

/// <p>Get the <a>SdkTypes</a> collection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSdkTypesRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Requests API Gateway to get information about a <a>Stage</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetStageRequest {
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The name of the <a>Stage</a> resource to get information about.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>Requests API Gateway to get information about one or more <a>Stage</a> resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetStagesRequest {
    /// <p>The stages' deployment identifiers.</p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets the <a>Tags</a> collection for a given resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTagsRequest {
    /// <p>(Not currently supported) The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>(Not currently supported) The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The ARN of a resource that can be tagged. The resource ARN must be URL-encoded. At present, <a>Stage</a> is the only taggable resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

/// <p>The GET request to get a usage plan key of a given key identifier.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUsagePlanKeyRequest {
    /// <p>[Required] The key Id of the to-be-retrieved <a>UsagePlanKey</a> resource representing a plan customer.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>[Required] The Id of the <a>UsagePlan</a> resource representing the usage plan containing the to-be-retrieved <a>UsagePlanKey</a> resource representing a plan customer.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>The GET request to get all the usage plan keys representing the API keys added to a specified usage plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUsagePlanKeysRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A query parameter specifying the name of the to-be-returned usage plan keys.</p>
    #[serde(rename = "nameQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_query: Option<String>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The Id of the <a>UsagePlan</a> resource representing the usage plan containing the to-be-retrieved <a>UsagePlanKey</a> resource representing a plan customer.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>The GET request to get a usage plan of a given plan identifier.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUsagePlanRequest {
    /// <p>[Required] The identifier of the <a>UsagePlan</a> resource to be retrieved.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>The GET request to get all the usage plans of the caller's account.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUsagePlansRequest {
    /// <p>The identifier of the API key associated with the usage plans.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>The GET request to get the usage data of a usage plan in a specified time interval.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUsageRequest {
    /// <p>[Required] The ending date (e.g., 2016-12-31) of the usage data.</p>
    #[serde(rename = "endDate")]
    pub end_date: String,
    /// <p>The Id of the API key associated with the resultant usage data.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The starting date (e.g., 2016-01-01) of the usage data.</p>
    #[serde(rename = "startDate")]
    pub start_date: String,
    /// <p>[Required] The Id of the usage plan associated with the usage data.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>Gets a specified VPC link under the caller's account in a region.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVpcLinkRequest {
    /// <p>[Required] The identifier of the <a>VpcLink</a>. It is used in an <a>Integration</a> to reference this <a>VpcLink</a>.</p>
    #[serde(rename = "vpcLinkId")]
    pub vpc_link_id: String,
}

/// <p>Gets the <a>VpcLinks</a> collection under the caller's account in a selected region.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVpcLinksRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>The POST request to import API keys from an external source, such as a CSV-formatted file.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportApiKeysRequest {
    /// <p>The payload of the POST request to import API keys. For the payload format, see <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-key-file-format.html">API Key File Format</a>.</p>
    #[serde(rename = "body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default,
    )]
    pub body: Vec<u8>,
    /// <p>A query parameter to indicate whether to rollback <a>ApiKey</a> importation (<code>true</code>) or not (<code>false</code>) when error is encountered.</p>
    #[serde(rename = "failOnWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
    /// <p>A query parameter to specify the input format to imported API keys. Currently, only the <code>csv</code> format is supported.</p>
    #[serde(rename = "format")]
    pub format: String,
}

/// <p>Import documentation parts from an external (e.g., Swagger) definition file. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportDocumentationPartsRequest {
    /// <p>[Required] Raw byte array representing the to-be-imported documentation parts. To import from a Swagger file, this is a JSON object.</p>
    #[serde(rename = "body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default,
    )]
    pub body: Vec<u8>,
    /// <p>A query parameter to specify whether to rollback the documentation importation (<code>true</code>) or not (<code>false</code>) when a warning is encountered. The default value is <code>false</code>.</p>
    #[serde(rename = "failOnWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
    /// <p>A query parameter to indicate whether to overwrite (<code>OVERWRITE</code>) any existing <a>DocumentationParts</a> definition or to merge (<code>MERGE</code>) the new definition into the existing one. The default value is <code>MERGE</code>.</p>
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A POST request to import an API to API Gateway using an input of an API definition file.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportRestApiRequest {
    /// <p>[Required] The POST request body containing external API definitions. Currently, only Swagger definition JSON files are supported. The maximum size of the API definition file is 2MB.</p>
    #[serde(rename = "body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default,
    )]
    pub body: Vec<u8>,
    /// <p>A query parameter to indicate whether to rollback the API creation (<code>true</code>) or not (<code>false</code>) when a warning is encountered. The default value is <code>false</code>.</p>
    #[serde(rename = "failOnWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
    /// <p><p>A key-value map of context-specific query string parameters specifying the behavior of different API importing operations. The following shows operation-specific parameters and their supported values.</p> <p> To exclude <a>DocumentationParts</a> from the import, set <code>parameters</code> as <code>ignore=documentation</code>.</p> <p> To configure the endpoint type, set <code>parameters</code> as <code>endpointConfigurationTypes=EDGE</code>, <code>endpointConfigurationTypes=REGIONAL</code>, or <code>endpointConfigurationTypes=PRIVATE</code>. The default endpoint type is <code>EDGE</code>.</p> <p> To handle imported <code>basePath</code>, set <code>parameters</code> as <code>basePath=ignore</code>, <code>basePath=prepend</code> or <code>basePath=split</code>.</p> <p>For example, the AWS CLI command to exclude documentation from the imported API is:</p> <pre><code>aws apigateway import-rest-api --parameters ignore=documentation --body &#39;file:///path/to/imported-api-body.json&#39;</code></pre> <p>The AWS CLI command to set the regional endpoint on the imported API is:</p> <pre><code>aws apigateway import-rest-api --parameters endpointConfigurationTypes=REGIONAL --body &#39;file:///path/to/imported-api-body.json&#39;</code></pre></p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p>Represents an HTTP, HTTP<em>PROXY, AWS, AWS</em>PROXY, or Mock integration.</p> <div class="remarks">In the API Gateway console, the built-in Lambda integration is an AWS integration.</div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Creating an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Integration {
    /// <p>Specifies the integration's cache key parameters.</p>
    #[serde(rename = "cacheKeyParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_key_parameters: Option<Vec<String>>,
    /// <p>Specifies the integration's cache namespace.</p>
    #[serde(rename = "cacheNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_namespace: Option<String>,
    /// <p>The (<a href="http://docs.aws.amazon.com/apigateway/api-reference/resource/vpc-link/#id"><code>id</code></a>) of the <a>VpcLink</a> used for the integration when <code>connectionType=VPC_LINK</code> and undefined, otherwise.</p>
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. The valid value is <code>INTERNET</code> for connections through the public routable internet or <code>VPC_LINK</code> for private connections between API Gateway and a network load balancer in a VPC. The default value is <code>INTERNET</code>.</p>
    #[serde(rename = "connectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Specifies how to handle request payload content type conversions. Supported values are <code>CONVERT_TO_BINARY</code> and <code>CONVERT_TO_TEXT</code>, with the following behaviors:</p> <ul> <li><p><code>CONVERT_TO_BINARY</code>: Converts a request payload from a Base64-encoded string to the corresponding binary blob.</p></li> <li><p><code>CONVERT_TO_TEXT</code>: Converts a request payload from a binary blob to a Base64-encoded string.</p></li> </ul> <p>If this property is not defined, the request payload will be passed through from the method request to integration request without modification, provided that the <code>passthroughBehaviors</code> is configured to support payload pass-through.</p>
    #[serde(rename = "contentHandling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string <code>arn:aws:iam::\*:user/\*</code>. To use resource-based permissions on supported AWS services, specify null.</p>
    #[serde(rename = "credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    /// <p>Specifies the integration's HTTP method type.</p>
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p><p>Specifies the integration&#39;s responses.</p> <div class="remarks"> <p/> <h4>Example: Get integration responses of a method</h4> <h5>Request</h5> <p/> <pre><code>GET /restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200 HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com X-Amz-Date: 20160607T191449Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/20160607/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash} </code></pre> <h5>Response</h5> <p>The successful response returns <code>200 OK</code> status and a payload as follows:</p> <pre><code>{ &quot;_links&quot;: { &quot;curies&quot;: { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-response-{rel}.html&quot;, &quot;name&quot;: &quot;integrationresponse&quot;, &quot;templated&quot;: true }, &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integrationresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot; }, &quot;integrationresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot; } }, &quot;responseParameters&quot;: { &quot;method.response.header.Content-Type&quot;: &quot;&#39;application/xml&#39;&quot; }, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;$util.urlDecode(&quot;%3CkinesisStreams%3E#foreach($stream in $input.path(&#39;$.StreamNames&#39;))%3Cstream%3E%3Cname%3E$stream%3C/name%3E%3C/stream%3E#end%3C/kinesisStreams%3E&quot;)\n&quot; }, &quot;statusCode&quot;: &quot;200&quot; }</code></pre> <p/> </div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Creating an API</a> </div></p>
    #[serde(rename = "integrationResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_responses: Option<::std::collections::HashMap<String, IntegrationResponse>>,
    /// <div> <p> Specifies how the method request body of an unmapped content type will be passed through the integration request to the back end without transformation. A content type is unmapped if no mapping template is defined in the integration or the content type does not match any of the mapped content types, as specified in <code>requestTemplates</code>. The valid value is one of the following: </p> <ul> <li> <code>WHEN_NO_MATCH</code>: passes the method request body through the integration request to the back end without transformation when the method request content type does not match any content type associated with the mapping templates defined in the integration request. </li> <li> <code>WHEN_NO_TEMPLATES</code>: passes the method request body through the integration request to the back end without transformation when no mapping template is defined in the integration request. If a template is defined when this option is selected, the method request of an unmapped content-type will be rejected with an HTTP <code>415 Unsupported Media Type</code> response. </li> <li> <code>NEVER</code>: rejects the method request with an HTTP <code>415 Unsupported Media Type</code> response when either the method request content type does not match any content type associated with the mapping templates defined in the integration request or no mapping template is defined in the integration request. </li> </ul> </div>
    #[serde(rename = "passthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method request to the back end. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the back end. The method request parameter value must match the pattern of <code>method.request.{location}.{name}</code>, where <code>location</code> is <code>querystring</code>, <code>path</code>, or <code>header</code> and <code>name</code> must be a valid and unique method request parameter name.</p>
    #[serde(rename = "requestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value.</p>
    #[serde(rename = "requestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds.</p>
    #[serde(rename = "timeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
    /// <p>Specifies an API method integration type. The valid value is one of the following:</p> <ul> <li><code>AWS</code>: for integrating the API method request with an AWS service action, including the Lambda function-invoking action. With the Lambda function-invoking action, this is referred to as the Lambda custom integration. With any other AWS service action, this is known as AWS integration.</li> <li><code>AWS_PROXY</code>: for integrating the API method request with the Lambda function-invoking action with the client request passed through as-is. This integration is also referred to as the Lambda proxy integration.</li> <li><code>HTTP</code>: for integrating the API method request with an HTTP endpoint, including a private HTTP endpoint within a VPC. This integration is also referred to as the HTTP custom integration.</li> <li><code>HTTP_PROXY</code>: for integrating the API method request with an HTTP endpoint, including a private HTTP endpoint within a VPC, with the client request passed through as-is. This is also referred to as the HTTP proxy integration.</li> <li><code>MOCK</code>: for integrating the API method request with API Gateway as a "loop-back" endpoint without invoking any backend.</li> </ul> <p>For the HTTP and HTTP proxy integrations, each integration can specify a protocol (<code>http/https</code>), port and path. Standard 80 and 443 ports are supported as well as custom ports above 1024. An HTTP or HTTP proxy integration with a <code>connectionType</code> of <code>VPC_LINK</code> is referred to as a private integration and uses a <a>VpcLink</a> to connect API Gateway to a network load balancer of a VPC.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p><p>Specifies Uniform Resource Identifier (URI) of the integration endpoint.</p> <ul> <li><p> For <code>HTTP</code> or <code>HTTP<em>PROXY</code> integrations, the URI must be a fully formed, encoded HTTP(S) URL according to the &lt;a target=&quot;</em>blank&quot; href=&quot;https://en.wikipedia.org/wiki/Uniform<em>Resource</em>Identifier&quot;&gt;RFC-3986 specification</a>, for either standard integration, where <code>connectionType</code> is not <code>VPC<em>LINK</code>, or private integration, where <code>connectionType</code> is <code>VPC</em>LINK</code>. For a private HTTP integration, the URI is not used for routing. </p> </li> <li><p> For <code>AWS</code> or <code>AWS<em>PROXY</code> integrations, the URI is of the form <code>arn:aws:apigateway:{region}:{subdomain.service|service}:path|action/{service</em>api}</code>. Here, <code>{Region}</code> is the API Gateway region (e.g., <code>us-east-1</code>); <code>{service}</code> is the name of the integrated AWS service (e.g., <code>s3</code>); and <code>{subdomain}</code> is a designated subdomain supported by certain AWS service for fast host-name lookup. <code>action</code> can be used for an AWS service action-based API, using an <code>Action={name}&amp;{p1}={v1}&amp;p2={v2}...</code> query string. The ensuing <code>{service<em>api}</code> refers to a supported action <code>{name}</code> plus any required input parameters. Alternatively, <code>path</code> can be used for an AWS service path-based API. The ensuing <code>service</em>api</code> refers to the path to an AWS service resource, including the region of the integrated AWS service, if applicable. For example, for integration with the S3 API of <code><a href="http://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectGET.html">GetObject</a></code>, the <code>uri</code> can be either <code>arn:aws:apigateway:us-west-2:s3:action/GetObject&amp;Bucket={bucket}&amp;Key={key}</code> or <code>arn:aws:apigateway:us-west-2:s3:path/{bucket}/{key}</code></p> </li></ul></p>
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// <p><p>Represents an integration response. The status code must map to an existing <a>MethodResponse</a>, and parameters and templates can be used to transform the back-end response.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Creating an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct IntegrationResponse {
    /// <p>Specifies how to handle response payload content type conversions. Supported values are <code>CONVERT_TO_BINARY</code> and <code>CONVERT_TO_TEXT</code>, with the following behaviors:</p> <ul> <li><p><code>CONVERT_TO_BINARY</code>: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p></li> <li><p><code>CONVERT_TO_TEXT</code>: Converts a response payload from a binary blob to a Base64-encoded string.</p></li> </ul> <p>If this property is not defined, the response payload will be passed through from the integration response to the method response without modification.</p>
    #[serde(rename = "contentHandling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<String>,
    /// <p>A key-value map specifying response parameters that are passed to the method response from the back end. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of <code>method.response.header.{name}</code>, where <code>name</code> is a valid and unique header name. The mapped non-static value must match the pattern of <code>integration.response.header.{name}</code> or <code>integration.response.body.{JSON-expression}</code>, where <code>name</code> is a valid and unique response header name and <code>JSON-expression</code> is a valid JSON expression without the <code>$</code> prefix.</p>
    #[serde(rename = "responseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies the templates used to transform the integration response body. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "responseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies the regular expression (regex) pattern used to choose an integration response based on the response from the back end. For example, if the success response returns nothing and the error response returns some string, you could use the <code>.+</code> regex to match error response. However, make sure that the error response does not contain any newline (<code>\n</code>) character in such cases. If the back end is an AWS Lambda function, the AWS Lambda function error header is matched. For all other HTTP and AWS back ends, the HTTP status code is matched.</p>
    #[serde(rename = "selectionPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_pattern: Option<String>,
    /// <p>Specifies the status code that is used to map the integration response to an existing <a>MethodResponse</a>.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

/// <p><p> Represents a client-facing interface by which the client calls the API to access back-end resources. A <b>Method</b> resource is integrated with an <a>Integration</a> resource. Both consist of a request and one or more responses. The method request takes the client input that is passed to the back end through the integration request. A method response returns the output from the back end to the client through an integration response. A method request is embodied in a <b>Method</b> resource, whereas an integration request is embodied in an <a>Integration</a> resource. On the other hand, a method response is represented by a <a>MethodResponse</a> resource, whereas an integration response is represented by an <a>IntegrationResponse</a> resource. </p> <div class="remarks"> <p/> <h4>Example: Retrive the GET method on a specified resource</h4> <h5>Request</h5> <p>The following example request retrieves the information about the GET method on an API resource (<code>3kzxbg5sa2</code>) of an API (<code>fugvjdxtri</code>). </p> <pre><code>GET /restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com X-Amz-Date: 20160603T210259Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/20160603/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash}</code></pre> <h5>Response</h5> <p>The successful response returns a <code>200 OK</code> status code and a payload similar to the following:</p> <pre><code>{ &quot;<em>links&quot;: { &quot;curies&quot;: [ { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-{rel}.html&quot;, &quot;name&quot;: &quot;integration&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-response-{rel}.html&quot;, &quot;name&quot;: &quot;integrationresponse&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-method-{rel}.html&quot;, &quot;name&quot;: &quot;method&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-method-response-{rel}.html&quot;, &quot;name&quot;: &quot;methodresponse&quot;, &quot;templated&quot;: true } ], &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET&quot;, &quot;name&quot;: &quot;GET&quot;, &quot;title&quot;: &quot;GET&quot; }, &quot;integration:put&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;method:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET&quot; }, &quot;method:integration&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;method:responses&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;method:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET&quot; }, &quot;methodresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/{status</em>code}&quot;, &quot;templated&quot;: true } }, &quot;apiKeyRequired&quot;: true, &quot;authorizationType&quot;: &quot;NONE&quot;, &quot;httpMethod&quot;: &quot;GET&quot;, &quot;<em>embedded&quot;: { &quot;method:integration&quot;: { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;integration:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;integration:responses&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integration:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;integrationresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/{status<em>code}&quot;, &quot;templated&quot;: true } }, &quot;cacheKeyParameters&quot;: [], &quot;cacheNamespace&quot;: &quot;3kzxbg5sa2&quot;, &quot;credentials&quot;: &quot;arn:aws:iam::123456789012:role/apigAwsProxyRole&quot;, &quot;httpMethod&quot;: &quot;POST&quot;, &quot;passthroughBehavior&quot;: &quot;WHEN</em>NO<em>MATCH&quot;, &quot;requestParameters&quot;: { &quot;integration.request.header.Content-Type&quot;: &quot;&#39;application/x-amz-json-1.1&#39;&quot; }, &quot;requestTemplates&quot;: { &quot;application/json&quot;: &quot;{\n}&quot; }, &quot;type&quot;: &quot;AWS&quot;, &quot;uri&quot;: &quot;arn:aws:apigateway:us-east-1:kinesis:action/ListStreams&quot;, &quot;</em>embedded&quot;: { &quot;integration:responses&quot;: { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integrationresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot; }, &quot;integrationresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot; } }, &quot;responseParameters&quot;: { &quot;method.response.header.Content-Type&quot;: &quot;&#39;application/xml&#39;&quot; }, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;$util.urlDecode(&quot;%3CkinesisStreams%3E%23foreach(%24stream%20in%20%24input.path(%27%24.StreamNames%27))%3Cstream%3E%3Cname%3E%24stream%3C%2Fname%3E%3C%2Fstream%3E%23end%3C%2FkinesisStreams%3E&quot;)&quot; }, &quot;statusCode&quot;: &quot;200&quot; } } }, &quot;method:responses&quot;: { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;methodresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot; }, &quot;methodresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot; } }, &quot;responseModels&quot;: { &quot;application/json&quot;: &quot;Empty&quot; }, &quot;responseParameters&quot;: { &quot;method.response.header.Content-Type&quot;: false }, &quot;statusCode&quot;: &quot;200&quot; } } }</code></pre> <p>In the example above, the response template for the <code>200 OK</code> response maps the JSON output from the <code>ListStreams</code> action in the back end to an XML output. The mapping template is URL-encoded as <code>%3CkinesisStreams%3E%23foreach(%24stream%20in%20%24input.path(%27%24.StreamNames%27))%3Cstream%3E%3Cname%3E%24stream%3C%2Fname%3E%3C%2Fstream%3E%23end%3C%2FkinesisStreams%3E</code> and the output is decoded using the <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-mapping-template-reference.html#util-templat-reference">$util.urlDecode()</a> helper function.</p> </div> <div class="seeAlso"> <a>MethodResponse</a>, <a>Integration</a>, <a>IntegrationResponse</a>, <a>Resource</a>, <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-method-settings.html">Set up an API&#39;s method</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Method {
    /// <p>A boolean flag specifying whether a valid <a>ApiKey</a> is required to invoke this method.</p>
    #[serde(rename = "apiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>A list of authorization scopes configured on the method. The scopes are used with a <code>COGNITO_USER_POOLS</code> authorizer to authorize the method invocation. The authorization works by matching the method scopes against the scopes parsed from the access token in the incoming request. The method invocation is authorized if any method scopes matches a claimed scope in the access token. Otherwise, the invocation is not authorized. When the method scope is configured, the client must provide an access token instead of an identity token for authorization purposes.</p>
    #[serde(rename = "authorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The method's authorization type. Valid values are <code>NONE</code> for open access, <code>AWS_IAM</code> for using AWS IAM permissions, <code>CUSTOM</code> for using a custom authorizer, or <code>COGNITO_USER_POOLS</code> for using a Cognito user pool.</p>
    #[serde(rename = "authorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of an <a>Authorizer</a> to use on this method. The <code>authorizationType</code> must be <code>CUSTOM</code>.</p>
    #[serde(rename = "authorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The method's HTTP verb.</p>
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p><p>Gets the method&#39;s integration responsible for passing the client-submitted request to the back end and performing necessary transformations to make the request compliant with the back end.</p> <div class="remarks"> <p/> <h4>Example: </h4> <h5>Request</h5> <p/> <pre><code>GET /restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com Content-Length: 117 X-Amz-Date: 20160613T213210Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/20160613/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash}</code></pre> <h5>Response</h5> <p>The successful response returns a <code>200 OK</code> status code and a payload similar to the following:</p> <pre><code>{ &quot;<em>links&quot;: { &quot;curies&quot;: [ { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-{rel}.html&quot;, &quot;name&quot;: &quot;integration&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-response-{rel}.html&quot;, &quot;name&quot;: &quot;integrationresponse&quot;, &quot;templated&quot;: true } ], &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration&quot; }, &quot;integration:delete&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration&quot; }, &quot;integration:responses&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integration:update&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration&quot; }, &quot;integrationresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration/responses/{status</em>code}&quot;, &quot;templated&quot;: true } }, &quot;cacheKeyParameters&quot;: [], &quot;cacheNamespace&quot;: &quot;0cjtch&quot;, &quot;credentials&quot;: &quot;arn:aws:iam::123456789012:role/apigAwsProxyRole&quot;, &quot;httpMethod&quot;: &quot;POST&quot;, &quot;passthroughBehavior&quot;: &quot;WHEN<em>NO</em>MATCH&quot;, &quot;requestTemplates&quot;: { &quot;application/json&quot;: &quot;{\n &quot;a&quot;: &quot;$input.params(&#39;operand1&#39;)&quot;,\n &quot;b&quot;: &quot;$input.params(&#39;operand2&#39;)&quot;, \n &quot;op&quot;: &quot;$input.params(&#39;operator&#39;)&quot; \n}&quot; }, &quot;type&quot;: &quot;AWS&quot;, &quot;uri&quot;: &quot;arn:aws:apigateway:us-west-2:lambda:path//2015-03-31/functions/arn:aws:lambda:us-west-2:123456789012:function:Calc/invocations&quot;, &quot;<em>embedded&quot;: { &quot;integration:responses&quot;: { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integrationresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration/responses/200&quot; }, &quot;integrationresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration/responses/200&quot; } }, &quot;responseParameters&quot;: { &quot;method.response.header.operator&quot;: &quot;integration.response.body.op&quot;, &quot;method.response.header.operand<em>2&quot;: &quot;integration.response.body.b&quot;, &quot;method.response.header.operand</em>1&quot;: &quot;integration.response.body.a&quot; }, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;#set($res = $input.path(&#39;$&#39;))\n{\n &quot;result&quot;: &quot;$res.a, $res.b, $res.op =&gt; $res.c&quot;,\n &quot;a&quot; : &quot;$res.a&quot;,\n &quot;b&quot; : &quot;$res.b&quot;,\n &quot;op&quot; : &quot;$res.op&quot;,\n &quot;c&quot; : &quot;$res.c&quot;\n}&quot; }, &quot;selectionPattern&quot;: &quot;&quot;, &quot;statusCode&quot;: &quot;200&quot; } } }</code></pre> <p/> </div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/get-integration.html">AWS CLI</a> </div></p>
    #[serde(rename = "methodIntegration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_integration: Option<Integration>,
    /// <p><p>Gets a method response associated with a given HTTP status code. </p> <div class="remarks"> <p>The collection of method responses are encapsulated in a key-value map, where the key is a response&#39;s HTTP status code and the value is a <a>MethodResponse</a> resource that specifies the response returned to the caller from the back end through the integration response.</p> <h4>Example: Get a 200 OK response of a GET method</h4> <h5>Request</h5> <p/> <pre><code>GET /restapis/uojnr9hd57/resources/0cjtch/methods/GET/responses/200 HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com Content-Length: 117 X-Amz-Date: 20160613T215008Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/20160613/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash}</code></pre> <h5>Response</h5> <p>The successful response returns a <code>200 OK</code> status code and a payload similar to the following:</p> <pre><code>{ &quot;<em>links&quot;: { &quot;curies&quot;: { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-method-response-{rel}.html&quot;, &quot;name&quot;: &quot;methodresponse&quot;, &quot;templated&quot;: true }, &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/responses/200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;methodresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/responses/200&quot; }, &quot;methodresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/responses/200&quot; } }, &quot;responseModels&quot;: { &quot;application/json&quot;: &quot;Empty&quot; }, &quot;responseParameters&quot;: { &quot;method.response.header.operator&quot;: false, &quot;method.response.header.operand</em>2&quot;: false, &quot;method.response.header.operand_1&quot;: false }, &quot;statusCode&quot;: &quot;200&quot; }</code></pre> <p/> </div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/get-method-response.html">AWS CLI</a> </div></p>
    #[serde(rename = "methodResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_responses: Option<::std::collections::HashMap<String, MethodResponse>>,
    /// <p>A human-friendly operation identifier for the method. For example, you can assign the <code>operationName</code> of <code>ListPets</code> for the <code>GET /pets</code> method in <a href="http://petstore-demo-endpoint.execute-api.com/petstore/pets">PetStore</a> example.</p>
    #[serde(rename = "operationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>A key-value map specifying data schemas, represented by <a>Model</a> resources, (as the mapped value) of the request payloads of given content types (as the mapping key).</p>
    #[serde(rename = "requestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>A key-value map defining required or optional method request parameters that can be accepted by API Gateway. A key is a method request parameter name matching the pattern of <code>method.request.{location}.{name}</code>, where <code>location</code> is <code>querystring</code>, <code>path</code>, or <code>header</code> and <code>name</code> is a valid and unique parameter name. The value associated with the key is a Boolean flag indicating whether the parameter is required (<code>true</code>) or optional (<code>false</code>). The method request parameter names defined here are available in <a>Integration</a> to be mapped to integration request parameters or templates.</p>
    #[serde(rename = "requestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, bool>>,
    /// <p>The identifier of a <a>RequestValidator</a> for request validation.</p>
    #[serde(rename = "requestValidatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_validator_id: Option<String>,
}

/// <p><p>Represents a method response of a given HTTP status code returned to the client. The method response is passed from the back end through the associated integration response that can be transformed using a mapping template. </p> <div class="remarks"> <p/> <h4>Example: A <b>MethodResponse</b> instance of an API</h4> <h5>Request</h5> <p>The example request retrieves a <b>MethodResponse</b> of the 200 status code.</p> <pre><code>GET /restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200 HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com X-Amz-Date: 20160603T222952Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/20160603/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash}</code></pre> <h5>Response</h5> <p>The successful response returns <code>200 OK</code> status and a payload as follows:</p> <pre><code>{ &quot;_links&quot;: { &quot;curies&quot;: { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-method-response-{rel}.html&quot;, &quot;name&quot;: &quot;methodresponse&quot;, &quot;templated&quot;: true }, &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;methodresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot; }, &quot;methodresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot; } }, &quot;responseModels&quot;: { &quot;application/json&quot;: &quot;Empty&quot; }, &quot;responseParameters&quot;: { &quot;method.response.header.Content-Type&quot;: false }, &quot;statusCode&quot;: &quot;200&quot; }</code></pre> <p/> </div> <div class="seeAlso"> <a>Method</a>, <a>IntegrationResponse</a>, <a>Integration</a> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Creating an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MethodResponse {
    /// <p>Specifies the <a>Model</a> resources used for the response's content-type. Response models are represented as a key/value map, with a content-type as the key and a <a>Model</a> name as the value.</p>
    #[serde(rename = "responseModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>A key-value map specifying required or optional response parameters that API Gateway can send back to the caller. A key defines a method response header and the value specifies whether the associated method response header is required or not. The expression of the key must match the pattern <code>method.response.header.{name}</code>, where <code>name</code> is a valid and unique header name. API Gateway passes certain integration response data to the method response headers specified here according to the mapping you prescribe in the API's <a>IntegrationResponse</a>. The integration response data that can be mapped include an integration response header expressed in <code>integration.response.header.{name}</code>, a static value enclosed within a pair of single quotes (e.g., <code>'application/json'</code>), or a JSON expression from the back-end response payload in the form of <code>integration.response.body.{JSON-expression}</code>, where <code>JSON-expression</code> is a valid JSON expression without the <code>$</code> prefix.)</p>
    #[serde(rename = "responseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, bool>>,
    /// <p>The method response's status code.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

/// <p>Specifies the method setting properties.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MethodSetting {
    /// <p>Specifies whether the cached responses are encrypted. The PATCH path for this setting is <code>/{method_setting_key}/caching/dataEncrypted</code>, and the value is a Boolean.</p>
    #[serde(rename = "cacheDataEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_data_encrypted: Option<bool>,
    /// <p>Specifies the time to live (TTL), in seconds, for cached responses. The higher the TTL, the longer the response will be cached. The PATCH path for this setting is <code>/{method_setting_key}/caching/ttlInSeconds</code>, and the value is an integer.</p>
    #[serde(rename = "cacheTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ttl_in_seconds: Option<i64>,
    /// <p>Specifies whether responses should be cached and returned for requests. A cache cluster must be enabled on the stage for responses to be cached. The PATCH path for this setting is <code>/{method_setting_key}/caching/enabled</code>, and the value is a Boolean.</p>
    #[serde(rename = "cachingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_enabled: Option<bool>,
    /// <p>Specifies whether data trace logging is enabled for this method, which effects the log entries pushed to Amazon CloudWatch Logs. The PATCH path for this setting is <code>/{method_setting_key}/logging/dataTrace</code>, and the value is a Boolean.</p>
    #[serde(rename = "dataTraceEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_trace_enabled: Option<bool>,
    /// <p>Specifies the logging level for this method, which effects the log entries pushed to Amazon CloudWatch Logs. The PATCH path for this setting is <code>/{method_setting_key}/logging/loglevel</code>, and the available levels are <code>OFF</code>, <code>ERROR</code>, and <code>INFO</code>.</p>
    #[serde(rename = "loggingLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    /// <p>Specifies whether Amazon CloudWatch metrics are enabled for this method. The PATCH path for this setting is <code>/{method_setting_key}/metrics/enabled</code>, and the value is a Boolean.</p>
    #[serde(rename = "metricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_enabled: Option<bool>,
    /// <p>Specifies whether authorization is required for a cache invalidation request. The PATCH path for this setting is <code>/{method_setting_key}/caching/requireAuthorizationForCacheControl</code>, and the value is a Boolean.</p>
    #[serde(rename = "requireAuthorizationForCacheControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_authorization_for_cache_control: Option<bool>,
    /// <p>Specifies the throttling burst limit. The PATCH path for this setting is <code>/{method_setting_key}/throttling/burstLimit</code>, and the value is an integer.</p>
    #[serde(rename = "throttlingBurstLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_burst_limit: Option<i64>,
    /// <p>Specifies the throttling rate limit. The PATCH path for this setting is <code>/{method_setting_key}/throttling/rateLimit</code>, and the value is a double.</p>
    #[serde(rename = "throttlingRateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_rate_limit: Option<f64>,
    /// <p>Specifies how to handle unauthorized requests for cache invalidation. The PATCH path for this setting is <code>/{method_setting_key}/caching/unauthorizedCacheControlHeaderStrategy</code>, and the available values are <code>FAIL_WITH_403</code>, <code>SUCCEED_WITH_RESPONSE_HEADER</code>, <code>SUCCEED_WITHOUT_RESPONSE_HEADER</code>.</p>
    #[serde(rename = "unauthorizedCacheControlHeaderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unauthorized_cache_control_header_strategy: Option<String>,
}

/// <p>Represents a summary of a <a>Method</a> resource, given a particular date and time.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MethodSnapshot {
    /// <p>Specifies whether the method requires a valid <a>ApiKey</a>.</p>
    #[serde(rename = "apiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>The method's authorization type. Valid values are <code>NONE</code> for open access, <code>AWS_IAM</code> for using AWS IAM permissions, <code>CUSTOM</code> for using a custom authorizer, or <code>COGNITO_USER_POOLS</code> for using a Cognito user pool.</p>
    #[serde(rename = "authorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
}

/// <p><p>Represents the data structure of a method&#39;s request or response payload.</p> <div class="remarks"> <p>A request model defines the data structure of the client-supplied request payload. A response model defines the data structure of the response payload returned by the back end. Although not required, models are useful for mapping payloads between the front end and back end.</p> <p>A model is used for generating an API&#39;s SDK, validating the input request body, and creating a skeletal mapping template.</p> </div> <div class="seeAlso"> <a>Method</a>, <a>MethodResponse</a>, <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/models-mappings.html">Models and Mappings</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Model {
    /// <p>The content-type for the model.</p>
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The description of the model.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier for the model resource.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the model. Must be an alphanumeric string.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The schema for the model. For <code>application/json</code> models, this should be <a href="https://tools.ietf.org/html/draft-zyp-json-schema-04" target="_blank">JSON schema draft 4</a> model. Do not include "\*/" characters in the description of any properties because such "\*/" characters may be interpreted as the closing marker for comments in some languages, such as Java or JavaScript, causing the installation of your API's SDK generated by API Gateway to fail.</p>
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

/// <p><p>Represents a collection of <a>Model</a> resources.</p> <div class="seeAlso"> <a>Method</a>, <a>MethodResponse</a>, <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/models-mappings.html">Models and Mappings</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Models {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Model>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>A single patch operation to apply to the specified resource. Please refer to http://tools.ietf.org/html/rfc6902#section-4 for an explanation of how each operation is used.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PatchOperation {
    /// <p>The <code>copy</code> update operation's source as identified by a <code>JSON-Pointer</code> value referencing the location within the targeted resource to copy the value from. For example, to promote a canary deployment, you copy the canary deployment ID to the affiliated deployment ID by calling a PATCH request on a <a>Stage</a> resource with <code>"op":"copy"</code>, <code>"from":"/canarySettings/deploymentId"</code> and <code>"path":"/deploymentId"</code>.</p>
    #[serde(rename = "from")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// <p> An update operation to be performed with this PATCH request. The valid value can be <code>add</code>, <code>remove</code>, <code>replace</code> or <code>copy</code>. Not all valid operations are supported for a given resource. Support of the operations depends on specific operational contexts. Attempts to apply an unsupported operation on a resource will return an error message.</p>
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// <p>The <code>op</code> operation's target, as identified by a <a href="https://tools.ietf.org/html/draft-ietf-appsawg-json-pointer-08">JSON Pointer</a> value that references a location within the targeted resource. For example, if the target resource has an updateable property of <code>{"name":"value"}</code>, the path for this property is <code>/name</code>. If the <code>name</code> property value is a JSON object (e.g., <code>{"name": {"child/name": "child-value"}}</code>), the path for the <code>child/name</code> property will be <code>/name/child~1name</code>. Any slash ("/") character appearing in path names must be escaped with "~1", as shown in the example above. Each <code>op</code> operation can have only one <code>path</code> associated with it.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The new target value of the update operation. It is applicable for the <code>add</code> or <code>replace</code> operation. When using AWS CLI to update a property of a JSON value, enclose the JSON object with a pair of single quotes in a Linux shell, e.g., '{"a": ...}'. In a Windows shell, see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#cli-using-param-json">Using JSON for Parameters</a>.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Creates a customization of a <a>GatewayResponse</a> of a specified response type and status code on the given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutGatewayResponseRequest {
    /// <p><p>Response parameters (paths, query strings and headers) of the <a>GatewayResponse</a> as a string-to-string map of key-value pairs.</p></p>
    #[serde(rename = "responseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p><p>Response templates of the <a>GatewayResponse</a> as a string-to-string map of key-value pairs.</p></p>
    #[serde(rename = "responseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] <p>The response type of the associated <a>GatewayResponse</a>. Valid values are <ul><li>ACCESS_DENIED</li><li>API_CONFIGURATION_ERROR</li><li>AUTHORIZER_FAILURE</li><li> AUTHORIZER_CONFIGURATION_ERROR</li><li>BAD_REQUEST_PARAMETERS</li><li>BAD_REQUEST_BODY</li><li>DEFAULT_4XX</li><li>DEFAULT_5XX</li><li>EXPIRED_TOKEN</li><li>INVALID_SIGNATURE</li><li>INTEGRATION_FAILURE</li><li>INTEGRATION_TIMEOUT</li><li>INVALID_API_KEY</li><li>MISSING_AUTHENTICATION_TOKEN</li><li> QUOTA_EXCEEDED</li><li>REQUEST_TOO_LARGE</li><li>RESOURCE_NOT_FOUND</li><li>THROTTLED</li><li>UNAUTHORIZED</li><li>UNSUPPORTED_MEDIA_TYPE</li></ul> </p></p>
    #[serde(rename = "responseType")]
    pub response_type: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The HTTP status code of the <a>GatewayResponse</a>.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

/// <p>Sets up a method's integration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutIntegrationRequest {
    /// <p>Specifies a put integration input's cache key parameters.</p>
    #[serde(rename = "cacheKeyParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_key_parameters: Option<Vec<String>>,
    /// <p>Specifies a put integration input's cache namespace.</p>
    #[serde(rename = "cacheNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_namespace: Option<String>,
    /// <p>The (<a href="http://docs.aws.amazon.com/apigateway/api-reference/resource/vpc-link/#id"><code>id</code></a>) of the <a>VpcLink</a> used for the integration when <code>connectionType=VPC_LINK</code> and undefined, otherwise.</p>
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. The valid value is <code>INTERNET</code> for connections through the public routable internet or <code>VPC_LINK</code> for private connections between API Gateway and a network load balancer in a VPC. The default value is <code>INTERNET</code>.</p>
    #[serde(rename = "connectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Specifies how to handle request payload content type conversions. Supported values are <code>CONVERT_TO_BINARY</code> and <code>CONVERT_TO_TEXT</code>, with the following behaviors:</p> <ul> <li><p><code>CONVERT_TO_BINARY</code>: Converts a request payload from a Base64-encoded string to the corresponding binary blob.</p></li> <li><p><code>CONVERT_TO_TEXT</code>: Converts a request payload from a binary blob to a Base64-encoded string.</p></li> </ul> <p>If this property is not defined, the request payload will be passed through from the method request to integration request without modification, provided that the <code>passthroughBehaviors</code> is configured to support payload pass-through.</p>
    #[serde(rename = "contentHandling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<String>,
    /// <p>Specifies whether credentials are required for a put integration.</p>
    #[serde(rename = "credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    /// <p>[Required] Specifies a put integration request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>Specifies a put integration HTTP method. When the integration type is HTTP or AWS, this field is required.</p>
    #[serde(rename = "integrationHttpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_http_method: Option<String>,
    /// <p><p>Specifies the pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the <code>requestTemplates</code> property on the Integration resource. There are three valid values: <code>WHEN<em>NO</em>MATCH</code>, <code>WHEN<em>NO</em>TEMPLATES</code>, and <code>NEVER</code>. </p> <ul> <li><p><code>WHEN<em>NO</em>MATCH</code> passes the request body for unmapped content types through to the integration back end without transformation.</p></li> <li><p><code>NEVER</code> rejects unmapped content types with an HTTP 415 &#39;Unsupported Media Type&#39; response.</p></li> <li><p><code>WHEN<em>NO</em>TEMPLATES</code> allows pass-through when the integration has NO content types mapped to templates. However if there is at least one content type defined, unmapped content types will be rejected with the same 415 response.</p></li> </ul></p>
    #[serde(rename = "passthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method request to the back end. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the back end. The method request parameter value must match the pattern of <code>method.request.{location}.{name}</code>, where <code>location</code> is <code>querystring</code>, <code>path</code>, or <code>header</code> and <code>name</code> must be a valid and unique method request parameter name.</p>
    #[serde(rename = "requestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value.</p>
    #[serde(rename = "requestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] Specifies a put integration request's resource ID.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds.</p>
    #[serde(rename = "timeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
    /// <p>[Required] Specifies a put integration input's type.</p>
    #[serde(rename = "type")]
    pub type_: String,
    /// <p><p>Specifies Uniform Resource Identifier (URI) of the integration endpoint.</p> <ul> <li><p> For <code>HTTP</code> or <code>HTTP<em>PROXY</code> integrations, the URI must be a fully formed, encoded HTTP(S) URL according to the &lt;a target=&quot;</em>blank&quot; href=&quot;https://en.wikipedia.org/wiki/Uniform<em>Resource</em>Identifier&quot;&gt;RFC-3986 specification</a>, for either standard integration, where <code>connectionType</code> is not <code>VPC<em>LINK</code>, or private integration, where <code>connectionType</code> is <code>VPC</em>LINK</code>. For a private HTTP integration, the URI is not used for routing. </p> </li> <li><p> For <code>AWS</code> or <code>AWS<em>PROXY</code> integrations, the URI is of the form <code>arn:aws:apigateway:{region}:{subdomain.service|service}:path|action/{service</em>api}</code>. Here, <code>{Region}</code> is the API Gateway region (e.g., <code>us-east-1</code>); <code>{service}</code> is the name of the integrated AWS service (e.g., <code>s3</code>); and <code>{subdomain}</code> is a designated subdomain supported by certain AWS service for fast host-name lookup. <code>action</code> can be used for an AWS service action-based API, using an <code>Action={name}&amp;{p1}={v1}&amp;p2={v2}...</code> query string. The ensuing <code>{service<em>api}</code> refers to a supported action <code>{name}</code> plus any required input parameters. Alternatively, <code>path</code> can be used for an AWS service path-based API. The ensuing <code>service</em>api</code> refers to the path to an AWS service resource, including the region of the integrated AWS service, if applicable. For example, for integration with the S3 API of <code><a href="http://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectGET.html">GetObject</a></code>, the <code>uri</code> can be either <code>arn:aws:apigateway:us-west-2:s3:action/GetObject&amp;Bucket={bucket}&amp;Key={key}</code> or <code>arn:aws:apigateway:us-west-2:s3:path/{bucket}/{key}</code></p> </li></ul></p>
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// <p>Represents a put integration response request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutIntegrationResponseRequest {
    /// <p>Specifies how to handle response payload content type conversions. Supported values are <code>CONVERT_TO_BINARY</code> and <code>CONVERT_TO_TEXT</code>, with the following behaviors:</p> <ul> <li><p><code>CONVERT_TO_BINARY</code>: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p></li> <li><p><code>CONVERT_TO_TEXT</code>: Converts a response payload from a binary blob to a Base64-encoded string.</p></li> </ul> <p>If this property is not defined, the response payload will be passed through from the integration response to the method response without modification.</p>
    #[serde(rename = "contentHandling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<String>,
    /// <p>[Required] Specifies a put integration response request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] Specifies a put integration response request's resource identifier.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>A key-value map specifying response parameters that are passed to the method response from the back end. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of <code>method.response.header.{name}</code>, where <code>name</code> is a valid and unique header name. The mapped non-static value must match the pattern of <code>integration.response.header.{name}</code> or <code>integration.response.body.{JSON-expression}</code>, where <code>name</code> must be a valid and unique response header name and <code>JSON-expression</code> a valid JSON expression without the <code>$</code> prefix.</p>
    #[serde(rename = "responseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies a put integration response's templates.</p>
    #[serde(rename = "responseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>Specifies the selection pattern of a put integration response.</p>
    #[serde(rename = "selectionPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_pattern: Option<String>,
    /// <p>[Required] Specifies the status code that is used to map the integration response to an existing <a>MethodResponse</a>.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to add a method to an existing <a>Resource</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutMethodRequest {
    /// <p>Specifies whether the method required a valid <a>ApiKey</a>.</p>
    #[serde(rename = "apiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>A list of authorization scopes configured on the method. The scopes are used with a <code>COGNITO_USER_POOLS</code> authorizer to authorize the method invocation. The authorization works by matching the method scopes against the scopes parsed from the access token in the incoming request. The method invocation is authorized if any method scopes matches a claimed scope in the access token. Otherwise, the invocation is not authorized. When the method scope is configured, the client must provide an access token instead of an identity token for authorization purposes.</p>
    #[serde(rename = "authorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>[Required] The method's authorization type. Valid values are <code>NONE</code> for open access, <code>AWS_IAM</code> for using AWS IAM permissions, <code>CUSTOM</code> for using a custom authorizer, or <code>COGNITO_USER_POOLS</code> for using a Cognito user pool.</p>
    #[serde(rename = "authorizationType")]
    pub authorization_type: String,
    /// <p>Specifies the identifier of an <a>Authorizer</a> to use on this Method, if the type is CUSTOM or COGNITO_USER_POOLS. The authorizer identifier is generated by API Gateway when you created the authorizer.</p>
    #[serde(rename = "authorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>[Required] Specifies the method request's HTTP method type.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>A human-friendly operation identifier for the method. For example, you can assign the <code>operationName</code> of <code>ListPets</code> for the <code>GET /pets</code> method in <a href="http://petstore-demo-endpoint.execute-api.com/petstore/pets">PetStore</a> example.</p>
    #[serde(rename = "operationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>Specifies the <a>Model</a> resources used for the request's content type. Request models are represented as a key/value map, with a content type as the key and a <a>Model</a> name as the value.</p>
    #[serde(rename = "requestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>A key-value map defining required or optional method request parameters that can be accepted by API Gateway. A key defines a method request parameter name matching the pattern of <code>method.request.{location}.{name}</code>, where <code>location</code> is <code>querystring</code>, <code>path</code>, or <code>header</code> and <code>name</code> is a valid and unique parameter name. The value associated with the key is a Boolean flag indicating whether the parameter is required (<code>true</code>) or optional (<code>false</code>). The method request parameter names defined here are available in <a>Integration</a> to be mapped to integration request parameters or body-mapping templates.</p>
    #[serde(rename = "requestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, bool>>,
    /// <p>The identifier of a <a>RequestValidator</a> for validating the method request.</p>
    #[serde(rename = "requestValidatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_validator_id: Option<String>,
    /// <p>[Required] The <a>Resource</a> identifier for the new <a>Method</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to add a <a>MethodResponse</a> to an existing <a>Method</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutMethodResponseRequest {
    /// <p>[Required] The HTTP verb of the <a>Method</a> resource.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>Method</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>Specifies the <a>Model</a> resources used for the response's content type. Response models are represented as a key/value map, with a content type as the key and a <a>Model</a> name as the value.</p>
    #[serde(rename = "responseModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>A key-value map specifying required or optional response parameters that API Gateway can send back to the caller. A key defines a method response header name and the associated value is a Boolean flag indicating whether the method response parameter is required or not. The method response header names must match the pattern of <code>method.response.header.{name}</code>, where <code>name</code> is a valid and unique header name. The response parameter names defined here are available in the integration response to be mapped from an integration response header expressed in <code>integration.response.header.{name}</code>, a static value enclosed within a pair of single quotes (e.g., <code>'application/json'</code>), or a JSON expression from the back-end response payload in the form of <code>integration.response.body.{JSON-expression}</code>, where <code>JSON-expression</code> is a valid JSON expression without the <code>$</code> prefix.)</p>
    #[serde(rename = "responseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, bool>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The method response's status code.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>A PUT request to update an existing API, with external API definitions specified as the request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutRestApiRequest {
    /// <p>[Required] The PUT request body containing external API definitions. Currently, only Swagger definition JSON files are supported. The maximum size of the API definition file is 2MB.</p>
    #[serde(rename = "body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default,
    )]
    pub body: Vec<u8>,
    /// <p>A query parameter to indicate whether to rollback the API update (<code>true</code>) or not (<code>false</code>) when a warning is encountered. The default value is <code>false</code>.</p>
    #[serde(rename = "failOnWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
    /// <p>The <code>mode</code> query parameter to specify the update mode. Valid values are "merge" and "overwrite". By default, the update mode is "merge".</p>
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>Custom header parameters as part of the request. For example, to exclude <a>DocumentationParts</a> from an imported API, set <code>ignore=documentation</code> as a <code>parameters</code> value, as in the AWS CLI command of <code>aws apigateway import-rest-api --parameters ignore=documentation --body 'file:///path/to/imported-api-body.json'</code>.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Quotas configured for a usage plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuotaSettings {
    /// <p>The maximum number of requests that can be made in a given time period.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The number of requests subtracted from the given limit in the initial time period.</p>
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// <p>The time period in which the limit applies. Valid values are "DAY", "WEEK" or "MONTH".</p>
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// <p><p>A set of validation rules for incoming <a>Method</a> requests.</p> <div class="remarks"> <p>In Swagger, a <a>RequestValidator</a> of an API is defined by the <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions.html#api-gateway-swagger-extensions-request-validators.requestValidator.html">x-amazon-apigateway-request-validators.requestValidator</a> object. It the referenced using the <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions.html#api-gateway-swagger-extensions-request-validator">x-amazon-apigateway-request-validator</a> property.</p> </div> <div class="seeAlso"><a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-method-request-validation.html">Enable Basic Request Validation in API Gateway</a></div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RequestValidator {
    /// <p>The identifier of this <a>RequestValidator</a>.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of this <a>RequestValidator</a></p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A Boolean flag to indicate whether to validate a request body according to the configured <a>Model</a> schema.</p>
    #[serde(rename = "validateRequestBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_request_body: Option<bool>,
    /// <p>A Boolean flag to indicate whether to validate request parameters (<code>true</code>) or not (<code>false</code>).</p>
    #[serde(rename = "validateRequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_request_parameters: Option<bool>,
}

/// <p><p>A collection of <a>RequestValidator</a> resources of a given <a>RestApi</a>.</p> <div class="remarks"> <p>In Swagger, the <a>RequestValidators</a> of an API is defined by the <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions.html#api-gateway-swagger-extensions-request-validators.html">x-amazon-apigateway-request-validators</a> extension.</p> </div> <div class="seeAlso"><a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-method-request-validation.html">Enable Basic Request Validation in API Gateway</a></div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RequestValidators {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RequestValidator>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>Represents an API resource.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Create an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Resource {
    /// <p>The resource's identifier.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The parent resource's identifier.</p>
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// <p>The full path for this resource.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The last path segment for this resource.</p>
    #[serde(rename = "pathPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_part: Option<String>,
    /// <p><p>Gets an API resource&#39;s method of a given HTTP verb.</p> <div class="remarks"> <p>The resource methods are a map of methods indexed by methods&#39; HTTP verbs enabled on the resource. This method map is included in the <code>200 OK</code> response of the <code>GET /restapis/{restapi<em>id}/resources/{resource</em>id}</code> or <code>GET /restapis/{restapi<em>id}/resources/{resource</em>id}?embed=methods</code> request.</p> <h4>Example: Get the GET method of an API resource</h4> <h5>Request</h5> <pre><code>GET /restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com X-Amz-Date: 20170223T031827Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/20170223/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash}</code></pre> <h5>Response</h5> <pre><code>{ &quot;<em>links&quot;: { &quot;curies&quot;: [ { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-{rel}.html&quot;, &quot;name&quot;: &quot;integration&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-response-{rel}.html&quot;, &quot;name&quot;: &quot;integrationresponse&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-method-{rel}.html&quot;, &quot;name&quot;: &quot;method&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-method-response-{rel}.html&quot;, &quot;name&quot;: &quot;methodresponse&quot;, &quot;templated&quot;: true } ], &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET&quot;, &quot;name&quot;: &quot;GET&quot;, &quot;title&quot;: &quot;GET&quot; }, &quot;integration:put&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;method:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET&quot; }, &quot;method:integration&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;method:responses&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;method:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET&quot; }, &quot;methodresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/{status</em>code}&quot;, &quot;templated&quot;: true } }, &quot;apiKeyRequired&quot;: false, &quot;authorizationType&quot;: &quot;NONE&quot;, &quot;httpMethod&quot;: &quot;GET&quot;, &quot;<em>embedded&quot;: { &quot;method:integration&quot;: { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;integration:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;integration:responses&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integration:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;integrationresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/{status<em>code}&quot;, &quot;templated&quot;: true } }, &quot;cacheKeyParameters&quot;: [], &quot;cacheNamespace&quot;: &quot;3kzxbg5sa2&quot;, &quot;credentials&quot;: &quot;arn:aws:iam::123456789012:role/apigAwsProxyRole&quot;, &quot;httpMethod&quot;: &quot;POST&quot;, &quot;passthroughBehavior&quot;: &quot;WHEN</em>NO<em>MATCH&quot;, &quot;requestParameters&quot;: { &quot;integration.request.header.Content-Type&quot;: &quot;&#39;application/x-amz-json-1.1&#39;&quot; }, &quot;requestTemplates&quot;: { &quot;application/json&quot;: &quot;{\n}&quot; }, &quot;type&quot;: &quot;AWS&quot;, &quot;uri&quot;: &quot;arn:aws:apigateway:us-east-1:kinesis:action/ListStreams&quot;, &quot;</em>embedded&quot;: { &quot;integration:responses&quot;: { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integrationresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot; }, &quot;integrationresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot; } }, &quot;responseParameters&quot;: { &quot;method.response.header.Content-Type&quot;: &quot;&#39;application/xml&#39;&quot; }, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;$util.urlDecode(&quot;%3CkinesisStreams%3E#foreach($stream in $input.path(&#39;$.StreamNames&#39;))%3Cstream%3E%3Cname%3E$stream%3C/name%3E%3C/stream%3E#end%3C/kinesisStreams%3E&quot;)\n&quot; }, &quot;statusCode&quot;: &quot;200&quot; } } }, &quot;method:responses&quot;: { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;methodresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot; }, &quot;methodresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot; } }, &quot;responseModels&quot;: { &quot;application/json&quot;: &quot;Empty&quot; }, &quot;responseParameters&quot;: { &quot;method.response.header.Content-Type&quot;: false }, &quot;statusCode&quot;: &quot;200&quot; } } }</code></pre> <p>If the <code>OPTIONS</code> is enabled on the resource, you can follow the example here to get that method. Just replace the <code>GET</code> of the last path segment in the request URL with <code>OPTIONS</code>.</p> </div> <div class="seeAlso"> </div></p>
    #[serde(rename = "resourceMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_methods: Option<::std::collections::HashMap<String, Method>>,
}

/// <p><p>Represents a collection of <a>Resource</a> resources.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Create an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Resources {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Resource>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>Represents a REST API.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Create an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RestApi {
    /// <p>The source of the API key for metering requests according to a usage plan. Valid values are: <ul><li><code>HEADER</code> to read the API key from the <code>X-API-Key</code> header of a request. </li><li><code>AUTHORIZER</code> to read the API key from the <code>UsageIdentifierKey</code> from a custom authorizer.</li></ul> </p>
    #[serde(rename = "apiKeySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_source: Option<String>,
    /// <p>The list of binary media types supported by the <a>RestApi</a>. By default, the <a>RestApi</a> supports only UTF-8-encoded text payloads.</p>
    #[serde(rename = "binaryMediaTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_media_types: Option<Vec<String>>,
    /// <p>The timestamp when the API was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The API's description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The endpoint configuration of this <a>RestApi</a> showing the endpoint types of the API. </p>
    #[serde(rename = "endpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    /// <p>The API's identifier. This identifier is unique across all of your APIs in API Gateway.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A nullable integer that is used to enable compression (with non-negative between 0 and 10485760 (10M) bytes, inclusive) or disable compression (with a null value) on an API. When compression is enabled, compression or decompression is not applied on the payload if the payload size is smaller than this value. Setting it to zero allows compression for any payload size.</p>
    #[serde(rename = "minimumCompressionSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_compression_size: Option<i64>,
    /// <p>The API's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A stringified JSON policy document that applies to this RestApi regardless of the caller and <a>Method</a> configuration.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The warning messages reported when <code>failonwarnings</code> is turned on during API import.</p>
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p><p>Contains references to your APIs and links that guide you in how to interact with your collection. A collection offers a paginated view of your APIs.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Create an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RestApis {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RestApi>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>A configuration property of an SDK type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SdkConfigurationProperty {
    /// <p>The default value of an <a>SdkType</a> configuration property.</p>
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p>The description of an <a>SdkType</a> configuration property.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The user-friendly name of an <a>SdkType</a> configuration property.</p>
    #[serde(rename = "friendlyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// <p>The name of a an <a>SdkType</a> configuration property.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A boolean flag of an <a>SdkType</a> configuration property to indicate if the associated SDK configuration property is required (<code>true</code>) or not (<code>false</code>).</p>
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// <p>The binary blob response to <a>GetSdk</a>, which contains the generated SDK.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SdkResponse {
    /// <p>The binary blob response to <a>GetSdk</a>, which contains the generated SDK.</p>
    pub body: Option<Vec<u8>>,
    /// <p>The content-disposition header value in the HTTP response.</p>
    pub content_disposition: Option<String>,
    /// <p>The content-type header value in the HTTP response.</p>
    pub content_type: Option<String>,
}

/// <p>A type of SDK that API Gateway can generate.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SdkType {
    /// <p>A list of configuration properties of an <a>SdkType</a>.</p>
    #[serde(rename = "configurationProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_properties: Option<Vec<SdkConfigurationProperty>>,
    /// <p>The description of an <a>SdkType</a>.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The user-friendly name of an <a>SdkType</a> instance.</p>
    #[serde(rename = "friendlyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// <p>The identifier of an <a>SdkType</a> instance.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>The collection of <a>SdkType</a> instances.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SdkTypes {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<SdkType>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>Represents a unique identifier for a version of a deployed <a>RestApi</a> that is callable by users.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-deploy-api.html">Deploy an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Stage {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "accessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>Specifies whether a cache cluster is enabled for the stage.</p>
    #[serde(rename = "cacheClusterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_enabled: Option<bool>,
    /// <p>The size of the cache cluster for the stage, if enabled.</p>
    #[serde(rename = "cacheClusterSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_size: Option<String>,
    /// <p>The status of the cache cluster for the stage, if enabled.</p>
    #[serde(rename = "cacheClusterStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_status: Option<String>,
    /// <p>Settings for the canary deployment in this stage.</p>
    #[serde(rename = "canarySettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_settings: Option<CanarySettings>,
    /// <p>The identifier of a client certificate for an API stage.</p>
    #[serde(rename = "clientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>The timestamp when the stage was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The identifier of the <a>Deployment</a> that the stage points to.</p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The stage's description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The version of the associated API documentation.</p>
    #[serde(rename = "documentationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_version: Option<String>,
    /// <p>The timestamp when the stage last updated.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>A map that defines the method settings for a <a>Stage</a> resource. Keys (designated as <code>/{method_setting_key</code> below) are method paths defined as <code>{resource_path}/{http_method}</code> for an individual method override, or <code>/\*/\*</code> for overriding all methods in the stage. </p>
    #[serde(rename = "methodSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_settings: Option<::std::collections::HashMap<String, MethodSetting>>,
    /// <p>The name of the stage is the first path segment in the Uniform Resource Identifier (URI) of a call to API Gateway.</p>
    #[serde(rename = "stageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A map that defines the stage variables for a <a>Stage</a> resource. Variable names can have alphanumeric and underscore characters, and the values must match <code>[A-Za-z0-9-._~:/?#&amp;=,]+</code>.</p>
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A reference to a unique stage identified in the format <code>{restApiId}/{stage}</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StageKey {
    /// <p>The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_api_id: Option<String>,
    /// <p>The stage name associated with the stage key.</p>
    #[serde(rename = "stageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

/// <p><p>A list of <a>Stage</a> resources that are associated with the <a>ApiKey</a> resource.</p> <div class="seeAlso"><a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/stages.html">Deploying API in Stages</a></div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Stages {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<Stage>>,
}

/// <p>Adds or updates a tag on a given resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>[Required] The ARN of a resource that can be tagged. The resource ARN must be URL-encoded. At present, <a>Stage</a> is the only taggable resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>[Required] The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>The collection of tags. Each tag element is associated with a given resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Tags {
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p>Represents a mapping template used to transform a payload.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/models-mappings.html#models-mappings-mappings">Mapping Templates</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Template {
    /// <p>The Apache <a href="http://velocity.apache.org/engine/devel/vtl-reference-guide.html" target="_blank">Velocity Template Language (VTL)</a> template content used for the template resource.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Make a request to simulate the execution of an <a>Authorizer</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TestInvokeAuthorizerRequest {
    /// <p>[Optional] A key-value map of additional context variables.</p>
    #[serde(rename = "additionalContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] Specifies a test invoke authorizer request's <a>Authorizer</a> ID.</p>
    #[serde(rename = "authorizerId")]
    pub authorizer_id: String,
    /// <p>[Optional] The simulated request body of an incoming invocation request.</p>
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>[Required] A key-value map of headers to simulate an incoming invocation request. This is where the incoming authorization token, or identity source, should be specified.</p>
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Optional] The URI path, including query string, of the simulated invocation request. Use this to specify path parameters and query string parameters.</p>
    #[serde(rename = "pathWithQueryString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_with_query_string: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>A key-value map of stage variables to simulate an invocation on a deployed <a>Stage</a>.</p>
    #[serde(rename = "stageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Represents the response of the test invoke request for a custom <a>Authorizer</a></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TestInvokeAuthorizerResponse {
    #[serde(rename = "authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The <a href="http://openid.net/specs/openid-connect-core-1_0.html#StandardClaims">open identity claims</a>, with any supported custom attributes, returned from the Cognito Your User Pool configured for the API.</p>
    #[serde(rename = "claims")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<::std::collections::HashMap<String, String>>,
    /// <p>The HTTP status code that the client would have received. Value is 0 if the authorizer succeeded.</p>
    #[serde(rename = "clientStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_status: Option<i64>,
    /// <p>The execution latency of the test authorizer request.</p>
    #[serde(rename = "latency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<i64>,
    /// <p>The API Gateway execution log for the test authorizer request.</p>
    #[serde(rename = "log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    /// <p>The JSON policy document returned by the <a>Authorizer</a></p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>The principal identity returned by the <a>Authorizer</a></p>
    #[serde(rename = "principalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}

/// <p>Make a request to simulate the execution of a <a>Method</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TestInvokeMethodRequest {
    /// <p>The simulated request body of an incoming invocation request.</p>
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>A <a>ClientCertificate</a> identifier to use in the test invocation. API Gateway will use the certificate when making the HTTPS request to the defined back-end endpoint.</p>
    #[serde(rename = "clientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>A key-value map of headers to simulate an incoming invocation request.</p>
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] Specifies a test invoke method request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>The URI path, including query string, of the simulated invocation request. Use this to specify path parameters and query string parameters.</p>
    #[serde(rename = "pathWithQueryString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_with_query_string: Option<String>,
    /// <p>[Required] Specifies a test invoke method request's resource ID.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>A key-value map of stage variables to simulate an invocation on a deployed <a>Stage</a>.</p>
    #[serde(rename = "stageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p>Represents the response of the test invoke request in the HTTP method.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-test-method.html#how-to-test-method-console">Test API using the API Gateway console</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TestInvokeMethodResponse {
    /// <p>The body of the HTTP response.</p>
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The headers of the HTTP response.</p>
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, String>>,
    /// <p>The execution latency of the test invoke request.</p>
    #[serde(rename = "latency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<i64>,
    /// <p>The API Gateway execution log for the test invoke request.</p>
    #[serde(rename = "log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    /// <p>The HTTP status code.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

/// <p> The API request rate limits.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrottleSettings {
    /// <p>The API request burst limit, the maximum rate limit over a time ranging from one to a few seconds, depending upon whether the underlying token bucket is at its full capacity.</p>
    #[serde(rename = "burstLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burst_limit: Option<i64>,
    /// <p>The API request steady-state rate limit.</p>
    #[serde(rename = "rateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<f64>,
}

/// <p>Removes a tag from a given resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>[Required] The ARN of a resource that can be tagged. The resource ARN must be URL-encoded. At present, <a>Stage</a> is the only taggable resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>[Required] The Tag keys to delete.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>Requests API Gateway to change information about the current <a>Account</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAccountRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

/// <p>A request to change information about an <a>ApiKey</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApiKeyRequest {
    /// <p>[Required] The identifier of the <a>ApiKey</a> resource to be updated.</p>
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

/// <p>Request to update an existing <a>Authorizer</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAuthorizerRequest {
    /// <p>[Required] The identifier of the <a>Authorizer</a> resource.</p>
    #[serde(rename = "authorizerId")]
    pub authorizer_id: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A request to change information about the <a>BasePathMapping</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateBasePathMappingRequest {
    /// <p>[Required] The base path of the <a>BasePathMapping</a> resource to change.</p>
    #[serde(rename = "basePath")]
    pub base_path: String,
    /// <p>[Required] The domain name of the <a>BasePathMapping</a> resource to change.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

/// <p>A request to change information about an <a>ClientCertificate</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateClientCertificateRequest {
    /// <p>[Required] The identifier of the <a>ClientCertificate</a> resource to be updated.</p>
    #[serde(rename = "clientCertificateId")]
    pub client_certificate_id: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

/// <p>Requests API Gateway to change information about a <a>Deployment</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDeploymentRequest {
    /// <p>The replacement identifier for the <a>Deployment</a> resource to change information about.</p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Updates an existing documentation part of a given API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDocumentationPartRequest {
    /// <p>[Required] The identifier of the to-be-updated documentation part.</p>
    #[serde(rename = "documentationPartId")]
    pub documentation_part_id: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Updates an existing documentation version of an API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDocumentationVersionRequest {
    /// <p>[Required] The version identifier of the to-be-updated documentation version.</p>
    #[serde(rename = "documentationVersion")]
    pub documentation_version: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>..</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A request to change information about the <a>DomainName</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDomainNameRequest {
    /// <p>[Required] The name of the <a>DomainName</a> resource to be changed.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

/// <p>Updates a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGatewayResponseRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] <p>The response type of the associated <a>GatewayResponse</a>. Valid values are <ul><li>ACCESS_DENIED</li><li>API_CONFIGURATION_ERROR</li><li>AUTHORIZER_FAILURE</li><li> AUTHORIZER_CONFIGURATION_ERROR</li><li>BAD_REQUEST_PARAMETERS</li><li>BAD_REQUEST_BODY</li><li>DEFAULT_4XX</li><li>DEFAULT_5XX</li><li>EXPIRED_TOKEN</li><li>INVALID_SIGNATURE</li><li>INTEGRATION_FAILURE</li><li>INTEGRATION_TIMEOUT</li><li>INVALID_API_KEY</li><li>MISSING_AUTHENTICATION_TOKEN</li><li> QUOTA_EXCEEDED</li><li>REQUEST_TOO_LARGE</li><li>RESOURCE_NOT_FOUND</li><li>THROTTLED</li><li>UNAUTHORIZED</li><li>UNSUPPORTED_MEDIA_TYPE</li></ul> </p></p>
    #[serde(rename = "responseType")]
    pub response_type: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Represents an update integration request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateIntegrationRequest {
    /// <p>[Required] Represents an update integration request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] Represents an update integration request's resource identifier.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Represents an update integration response request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateIntegrationResponseRequest {
    /// <p>[Required] Specifies an update integration response request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] Specifies an update integration response request's resource identifier.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] Specifies an update integration response request's status code.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to update an existing <a>Method</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateMethodRequest {
    /// <p>[Required] The HTTP verb of the <a>Method</a> resource.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>Method</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A request to update an existing <a>MethodResponse</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateMethodResponseRequest {
    /// <p>[Required] The HTTP verb of the <a>Method</a> resource.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>MethodResponse</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The status code for the <a>MethodResponse</a> resource.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to update an existing model in an existing <a>RestApi</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateModelRequest {
    /// <p>[Required] The name of the model to update.</p>
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Updates a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRequestValidatorRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The identifier of <a>RequestValidator</a> to be updated.</p>
    #[serde(rename = "requestValidatorId")]
    pub request_validator_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to change information about a <a>Resource</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateResourceRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The identifier of the <a>Resource</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to update an existing <a>RestApi</a> resource in your collection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRestApiRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Requests API Gateway to change information about a <a>Stage</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateStageRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The name of the <a>Stage</a> resource to change information about.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>The PATCH request to update a usage plan of a given plan Id.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUsagePlanRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The Id of the to-be-updated usage plan.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>The PATCH request to grant a temporary extension to the remaining quota of a usage plan associated with a specified API key.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUsageRequest {
    /// <p>[Required] The identifier of the API key associated with the usage plan in which a temporary extension is granted to the remaining quota.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The Id of the usage plan associated with the usage data.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>Updates an existing <a>VpcLink</a> of a specified identifier.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateVpcLinkRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The identifier of the <a>VpcLink</a>. It is used in an <a>Integration</a> to reference this <a>VpcLink</a>.</p>
    #[serde(rename = "vpcLinkId")]
    pub vpc_link_id: String,
}

/// <p><p>Represents the usage data of a usage plan.</p> <div class="remarks"/> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-api-usage-plans.html">Create and Use Usage Plans</a>, <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-create-usage-plans-with-console.html#api-gateway-usage-plan-manage-usage">Manage Usage in a Usage Plan</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Usage {
    /// <p>The ending date of the usage data.</p>
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// <p>The usage data, as daily logs of used and remaining quotas, over the specified time interval indexed over the API keys in a usage plan. For example, <code>{..., "values" : { "{api_key}" : [ [0, 100], [10, 90], [100, 10]]}</code>, where <code>{api_key}</code> stands for an API key value and the daily log entry is of the format <code>[used quota, remaining quota]</code>.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<::std::collections::HashMap<String, Vec<Vec<i64>>>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>The starting date of the usage data.</p>
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// <p>The plan Id associated with this usage data.</p>
    #[serde(rename = "usagePlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_plan_id: Option<String>,
}

/// <p><p>Represents a usage plan than can specify who can assess associated API stages with specified request limits and quotas.</p> <div class="remarks"> <p>In a usage plan, you associate an API by specifying the API&#39;s Id and a stage name of the specified API. You add plan customers by adding API keys to the plan. </p> </div> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-api-usage-plans.html">Create and Use Usage Plans</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UsagePlan {
    /// <p>The associated API stages of a usage plan.</p>
    #[serde(rename = "apiStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_stages: Option<Vec<ApiStage>>,
    /// <p>The description of a usage plan.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of a <a>UsagePlan</a> resource.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of a usage plan.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The AWS Markeplace product identifier to associate with the usage plan as a SaaS product on AWS Marketplace.</p>
    #[serde(rename = "productCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// <p>The maximum number of permitted requests per a given unit time interval.</p>
    #[serde(rename = "quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<QuotaSettings>,
    /// <p>The request throttle limits of a usage plan.</p>
    #[serde(rename = "throttle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<ThrottleSettings>,
}

/// <p><p>Represents a usage plan key to identify a plan customer.</p> <div class="remarks"> <p>To associate an API stage with a selected API key in a usage plan, you must create a UsagePlanKey resource to represent the selected <a>ApiKey</a>.</p> </div>&quot; <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-api-usage-plans.html">Create and Use Usage Plans</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UsagePlanKey {
    /// <p>The Id of a usage plan key.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of a usage plan key.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of a usage plan key. Currently, the valid key type is <code>API_KEY</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value of a usage plan key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p><p>Represents the collection of usage plan keys added to usage plans for the associated API keys and, possibly, other types of keys.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-api-usage-plans.html">Create and Use Usage Plans</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UsagePlanKeys {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UsagePlanKey>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>Represents a collection of usage plans for an AWS account.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-api-usage-plans.html">Create and Use Usage Plans</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UsagePlans {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UsagePlan>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>A API Gateway VPC link for a <a>RestApi</a> to access resources in an Amazon Virtual Private Cloud (VPC).</p> <div class="remarks"> <p><p>To enable access to a resource in an Amazon Virtual Private Cloud through Amazon API Gateway, you, as an API developer, create a <a>VpcLink</a> resource targeted for one or more network load balancers of the VPC and then integrate an API method with a private integration that uses the <a>VpcLink</a>. The private integration has an integration type of <code>HTTP</code> or <code>HTTP<em>PROXY</code> and has a connection type of <code>VPC</em>LINK</code>. The integration uses the <code>connectionId</code> property to identify the <a>VpcLink</a> used.</p> </p> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VpcLink {
    /// <p>The description of the VPC link.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the <a>VpcLink</a>. It is used in an <a>Integration</a> to reference this <a>VpcLink</a>.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name used to label and identify the VPC link.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the VPC link. The valid values are <code>AVAILABLE</code>, <code>PENDING</code>, <code>DELETING</code>, or <code>FAILED</code>. Deploying an API will wait if the status is <code>PENDING</code> and will fail if the status is <code>DELETING</code>. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A description about the VPC link status.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The ARNs of network load balancers of the VPC targeted by the VPC link. The network load balancers must be owned by the same AWS account of the API owner.</p>
    #[serde(rename = "targetArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arns: Option<Vec<String>>,
}

/// <p><p>The collection of VPC links under the caller&#39;s account in a region.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/getting-started-with-private-integration.html">Getting Started with Private Integrations</a>, <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-private-integration.html">Set up Private Integrations</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VpcLinks {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<VpcLink>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// Errors returned by CreateApiKey
#[derive(Debug, PartialEq)]
pub enum CreateApiKeyError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateApiKeyError {
    pub fn from_body(body: &str) -> CreateApiKeyError {
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
                    "BadRequestException" => {
                        CreateApiKeyError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => CreateApiKeyError::Conflict(String::from(error_message)),
                    "LimitExceededException" => {
                        CreateApiKeyError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => CreateApiKeyError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        CreateApiKeyError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateApiKeyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateApiKeyError::Validation(error_message.to_string())
                    }
                    _ => CreateApiKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateApiKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateApiKeyError {
    fn from(err: serde_json::error::Error) -> CreateApiKeyError {
        CreateApiKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateApiKeyError {
    fn from(err: CredentialsError) -> CreateApiKeyError {
        CreateApiKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateApiKeyError {
    fn from(err: HttpDispatchError) -> CreateApiKeyError {
        CreateApiKeyError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateApiKeyError {
    fn from(err: io::Error) -> CreateApiKeyError {
        CreateApiKeyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateApiKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApiKeyError {
    fn description(&self) -> &str {
        match *self {
            CreateApiKeyError::BadRequest(ref cause) => cause,
            CreateApiKeyError::Conflict(ref cause) => cause,
            CreateApiKeyError::LimitExceeded(ref cause) => cause,
            CreateApiKeyError::NotFound(ref cause) => cause,
            CreateApiKeyError::TooManyRequests(ref cause) => cause,
            CreateApiKeyError::Unauthorized(ref cause) => cause,
            CreateApiKeyError::Validation(ref cause) => cause,
            CreateApiKeyError::Credentials(ref err) => err.description(),
            CreateApiKeyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateApiKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAuthorizer
#[derive(Debug, PartialEq)]
pub enum CreateAuthorizerError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateAuthorizerError {
    pub fn from_body(body: &str) -> CreateAuthorizerError {
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
                    "BadRequestException" => {
                        CreateAuthorizerError::BadRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateAuthorizerError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateAuthorizerError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateAuthorizerError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateAuthorizerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateAuthorizerError::Validation(error_message.to_string())
                    }
                    _ => CreateAuthorizerError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAuthorizerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAuthorizerError {
    fn from(err: serde_json::error::Error) -> CreateAuthorizerError {
        CreateAuthorizerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAuthorizerError {
    fn from(err: CredentialsError) -> CreateAuthorizerError {
        CreateAuthorizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAuthorizerError {
    fn from(err: HttpDispatchError) -> CreateAuthorizerError {
        CreateAuthorizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAuthorizerError {
    fn from(err: io::Error) -> CreateAuthorizerError {
        CreateAuthorizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            CreateAuthorizerError::BadRequest(ref cause) => cause,
            CreateAuthorizerError::LimitExceeded(ref cause) => cause,
            CreateAuthorizerError::NotFound(ref cause) => cause,
            CreateAuthorizerError::TooManyRequests(ref cause) => cause,
            CreateAuthorizerError::Unauthorized(ref cause) => cause,
            CreateAuthorizerError::Validation(ref cause) => cause,
            CreateAuthorizerError::Credentials(ref err) => err.description(),
            CreateAuthorizerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAuthorizerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateBasePathMapping
#[derive(Debug, PartialEq)]
pub enum CreateBasePathMappingError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateBasePathMappingError {
    pub fn from_body(body: &str) -> CreateBasePathMappingError {
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
                    "BadRequestException" => {
                        CreateBasePathMappingError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateBasePathMappingError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateBasePathMappingError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateBasePathMappingError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateBasePathMappingError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateBasePathMappingError::Validation(error_message.to_string())
                    }
                    _ => CreateBasePathMappingError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateBasePathMappingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateBasePathMappingError {
    fn from(err: serde_json::error::Error) -> CreateBasePathMappingError {
        CreateBasePathMappingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBasePathMappingError {
    fn from(err: CredentialsError) -> CreateBasePathMappingError {
        CreateBasePathMappingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBasePathMappingError {
    fn from(err: HttpDispatchError) -> CreateBasePathMappingError {
        CreateBasePathMappingError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBasePathMappingError {
    fn from(err: io::Error) -> CreateBasePathMappingError {
        CreateBasePathMappingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBasePathMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBasePathMappingError {
    fn description(&self) -> &str {
        match *self {
            CreateBasePathMappingError::BadRequest(ref cause) => cause,
            CreateBasePathMappingError::Conflict(ref cause) => cause,
            CreateBasePathMappingError::NotFound(ref cause) => cause,
            CreateBasePathMappingError::TooManyRequests(ref cause) => cause,
            CreateBasePathMappingError::Unauthorized(ref cause) => cause,
            CreateBasePathMappingError::Validation(ref cause) => cause,
            CreateBasePathMappingError::Credentials(ref err) => err.description(),
            CreateBasePathMappingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateBasePathMappingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDeployment
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The requested service is not available. For details see the accompanying error message. Retry after the specified time period.</p>
    ServiceUnavailable(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateDeploymentError {
    pub fn from_body(body: &str) -> CreateDeploymentError {
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
                    "BadRequestException" => {
                        CreateDeploymentError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateDeploymentError::Conflict(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateDeploymentError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateDeploymentError::NotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateDeploymentError::ServiceUnavailable(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateDeploymentError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateDeploymentError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDeploymentError::Validation(error_message.to_string())
                    }
                    _ => CreateDeploymentError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDeploymentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDeploymentError {
    fn from(err: serde_json::error::Error) -> CreateDeploymentError {
        CreateDeploymentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDeploymentError {
    fn from(err: CredentialsError) -> CreateDeploymentError {
        CreateDeploymentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDeploymentError {
    fn from(err: HttpDispatchError) -> CreateDeploymentError {
        CreateDeploymentError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDeploymentError {
    fn from(err: io::Error) -> CreateDeploymentError {
        CreateDeploymentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDeploymentError {
    fn description(&self) -> &str {
        match *self {
            CreateDeploymentError::BadRequest(ref cause) => cause,
            CreateDeploymentError::Conflict(ref cause) => cause,
            CreateDeploymentError::LimitExceeded(ref cause) => cause,
            CreateDeploymentError::NotFound(ref cause) => cause,
            CreateDeploymentError::ServiceUnavailable(ref cause) => cause,
            CreateDeploymentError::TooManyRequests(ref cause) => cause,
            CreateDeploymentError::Unauthorized(ref cause) => cause,
            CreateDeploymentError::Validation(ref cause) => cause,
            CreateDeploymentError::Credentials(ref err) => err.description(),
            CreateDeploymentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDeploymentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDocumentationPart
#[derive(Debug, PartialEq)]
pub enum CreateDocumentationPartError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateDocumentationPartError {
    pub fn from_body(body: &str) -> CreateDocumentationPartError {
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
                    "BadRequestException" => {
                        CreateDocumentationPartError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateDocumentationPartError::Conflict(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateDocumentationPartError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateDocumentationPartError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateDocumentationPartError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateDocumentationPartError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDocumentationPartError::Validation(error_message.to_string())
                    }
                    _ => CreateDocumentationPartError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDocumentationPartError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDocumentationPartError {
    fn from(err: serde_json::error::Error) -> CreateDocumentationPartError {
        CreateDocumentationPartError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDocumentationPartError {
    fn from(err: CredentialsError) -> CreateDocumentationPartError {
        CreateDocumentationPartError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDocumentationPartError {
    fn from(err: HttpDispatchError) -> CreateDocumentationPartError {
        CreateDocumentationPartError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDocumentationPartError {
    fn from(err: io::Error) -> CreateDocumentationPartError {
        CreateDocumentationPartError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDocumentationPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDocumentationPartError {
    fn description(&self) -> &str {
        match *self {
            CreateDocumentationPartError::BadRequest(ref cause) => cause,
            CreateDocumentationPartError::Conflict(ref cause) => cause,
            CreateDocumentationPartError::LimitExceeded(ref cause) => cause,
            CreateDocumentationPartError::NotFound(ref cause) => cause,
            CreateDocumentationPartError::TooManyRequests(ref cause) => cause,
            CreateDocumentationPartError::Unauthorized(ref cause) => cause,
            CreateDocumentationPartError::Validation(ref cause) => cause,
            CreateDocumentationPartError::Credentials(ref err) => err.description(),
            CreateDocumentationPartError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDocumentationPartError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDocumentationVersion
#[derive(Debug, PartialEq)]
pub enum CreateDocumentationVersionError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateDocumentationVersionError {
    pub fn from_body(body: &str) -> CreateDocumentationVersionError {
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
                    "BadRequestException" => {
                        CreateDocumentationVersionError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateDocumentationVersionError::Conflict(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateDocumentationVersionError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateDocumentationVersionError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => CreateDocumentationVersionError::TooManyRequests(
                        String::from(error_message),
                    ),
                    "UnauthorizedException" => {
                        CreateDocumentationVersionError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDocumentationVersionError::Validation(error_message.to_string())
                    }
                    _ => CreateDocumentationVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDocumentationVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDocumentationVersionError {
    fn from(err: serde_json::error::Error) -> CreateDocumentationVersionError {
        CreateDocumentationVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDocumentationVersionError {
    fn from(err: CredentialsError) -> CreateDocumentationVersionError {
        CreateDocumentationVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDocumentationVersionError {
    fn from(err: HttpDispatchError) -> CreateDocumentationVersionError {
        CreateDocumentationVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDocumentationVersionError {
    fn from(err: io::Error) -> CreateDocumentationVersionError {
        CreateDocumentationVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDocumentationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDocumentationVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateDocumentationVersionError::BadRequest(ref cause) => cause,
            CreateDocumentationVersionError::Conflict(ref cause) => cause,
            CreateDocumentationVersionError::LimitExceeded(ref cause) => cause,
            CreateDocumentationVersionError::NotFound(ref cause) => cause,
            CreateDocumentationVersionError::TooManyRequests(ref cause) => cause,
            CreateDocumentationVersionError::Unauthorized(ref cause) => cause,
            CreateDocumentationVersionError::Validation(ref cause) => cause,
            CreateDocumentationVersionError::Credentials(ref err) => err.description(),
            CreateDocumentationVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDocumentationVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDomainName
#[derive(Debug, PartialEq)]
pub enum CreateDomainNameError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateDomainNameError {
    pub fn from_body(body: &str) -> CreateDomainNameError {
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
                    "BadRequestException" => {
                        CreateDomainNameError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateDomainNameError::Conflict(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateDomainNameError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateDomainNameError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDomainNameError::Validation(error_message.to_string())
                    }
                    _ => CreateDomainNameError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDomainNameError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDomainNameError {
    fn from(err: serde_json::error::Error) -> CreateDomainNameError {
        CreateDomainNameError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDomainNameError {
    fn from(err: CredentialsError) -> CreateDomainNameError {
        CreateDomainNameError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDomainNameError {
    fn from(err: HttpDispatchError) -> CreateDomainNameError {
        CreateDomainNameError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDomainNameError {
    fn from(err: io::Error) -> CreateDomainNameError {
        CreateDomainNameError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDomainNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDomainNameError {
    fn description(&self) -> &str {
        match *self {
            CreateDomainNameError::BadRequest(ref cause) => cause,
            CreateDomainNameError::Conflict(ref cause) => cause,
            CreateDomainNameError::TooManyRequests(ref cause) => cause,
            CreateDomainNameError::Unauthorized(ref cause) => cause,
            CreateDomainNameError::Validation(ref cause) => cause,
            CreateDomainNameError::Credentials(ref err) => err.description(),
            CreateDomainNameError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDomainNameError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateModel
#[derive(Debug, PartialEq)]
pub enum CreateModelError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateModelError {
    pub fn from_body(body: &str) -> CreateModelError {
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
                    "BadRequestException" => {
                        CreateModelError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => CreateModelError::Conflict(String::from(error_message)),
                    "LimitExceededException" => {
                        CreateModelError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => CreateModelError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        CreateModelError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateModelError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateModelError::Validation(error_message.to_string())
                    }
                    _ => CreateModelError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateModelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateModelError {
    fn from(err: serde_json::error::Error) -> CreateModelError {
        CreateModelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateModelError {
    fn from(err: CredentialsError) -> CreateModelError {
        CreateModelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateModelError {
    fn from(err: HttpDispatchError) -> CreateModelError {
        CreateModelError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateModelError {
    fn from(err: io::Error) -> CreateModelError {
        CreateModelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateModelError {
    fn description(&self) -> &str {
        match *self {
            CreateModelError::BadRequest(ref cause) => cause,
            CreateModelError::Conflict(ref cause) => cause,
            CreateModelError::LimitExceeded(ref cause) => cause,
            CreateModelError::NotFound(ref cause) => cause,
            CreateModelError::TooManyRequests(ref cause) => cause,
            CreateModelError::Unauthorized(ref cause) => cause,
            CreateModelError::Validation(ref cause) => cause,
            CreateModelError::Credentials(ref err) => err.description(),
            CreateModelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateModelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRequestValidator
#[derive(Debug, PartialEq)]
pub enum CreateRequestValidatorError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateRequestValidatorError {
    pub fn from_body(body: &str) -> CreateRequestValidatorError {
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
                    "BadRequestException" => {
                        CreateRequestValidatorError::BadRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateRequestValidatorError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateRequestValidatorError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateRequestValidatorError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateRequestValidatorError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateRequestValidatorError::Validation(error_message.to_string())
                    }
                    _ => CreateRequestValidatorError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateRequestValidatorError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateRequestValidatorError {
    fn from(err: serde_json::error::Error) -> CreateRequestValidatorError {
        CreateRequestValidatorError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateRequestValidatorError {
    fn from(err: CredentialsError) -> CreateRequestValidatorError {
        CreateRequestValidatorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateRequestValidatorError {
    fn from(err: HttpDispatchError) -> CreateRequestValidatorError {
        CreateRequestValidatorError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateRequestValidatorError {
    fn from(err: io::Error) -> CreateRequestValidatorError {
        CreateRequestValidatorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateRequestValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRequestValidatorError {
    fn description(&self) -> &str {
        match *self {
            CreateRequestValidatorError::BadRequest(ref cause) => cause,
            CreateRequestValidatorError::LimitExceeded(ref cause) => cause,
            CreateRequestValidatorError::NotFound(ref cause) => cause,
            CreateRequestValidatorError::TooManyRequests(ref cause) => cause,
            CreateRequestValidatorError::Unauthorized(ref cause) => cause,
            CreateRequestValidatorError::Validation(ref cause) => cause,
            CreateRequestValidatorError::Credentials(ref err) => err.description(),
            CreateRequestValidatorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateRequestValidatorError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateResource
#[derive(Debug, PartialEq)]
pub enum CreateResourceError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateResourceError {
    pub fn from_body(body: &str) -> CreateResourceError {
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
                    "BadRequestException" => {
                        CreateResourceError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateResourceError::Conflict(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateResourceError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateResourceError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateResourceError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateResourceError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateResourceError::Validation(error_message.to_string())
                    }
                    _ => CreateResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateResourceError {
    fn from(err: serde_json::error::Error) -> CreateResourceError {
        CreateResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateResourceError {
    fn from(err: CredentialsError) -> CreateResourceError {
        CreateResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateResourceError {
    fn from(err: HttpDispatchError) -> CreateResourceError {
        CreateResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateResourceError {
    fn from(err: io::Error) -> CreateResourceError {
        CreateResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResourceError {
    fn description(&self) -> &str {
        match *self {
            CreateResourceError::BadRequest(ref cause) => cause,
            CreateResourceError::Conflict(ref cause) => cause,
            CreateResourceError::LimitExceeded(ref cause) => cause,
            CreateResourceError::NotFound(ref cause) => cause,
            CreateResourceError::TooManyRequests(ref cause) => cause,
            CreateResourceError::Unauthorized(ref cause) => cause,
            CreateResourceError::Validation(ref cause) => cause,
            CreateResourceError::Credentials(ref err) => err.description(),
            CreateResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRestApi
#[derive(Debug, PartialEq)]
pub enum CreateRestApiError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateRestApiError {
    pub fn from_body(body: &str) -> CreateRestApiError {
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
                    "BadRequestException" => {
                        CreateRestApiError::BadRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateRestApiError::LimitExceeded(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateRestApiError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateRestApiError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateRestApiError::Validation(error_message.to_string())
                    }
                    _ => CreateRestApiError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateRestApiError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateRestApiError {
    fn from(err: serde_json::error::Error) -> CreateRestApiError {
        CreateRestApiError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateRestApiError {
    fn from(err: CredentialsError) -> CreateRestApiError {
        CreateRestApiError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateRestApiError {
    fn from(err: HttpDispatchError) -> CreateRestApiError {
        CreateRestApiError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateRestApiError {
    fn from(err: io::Error) -> CreateRestApiError {
        CreateRestApiError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateRestApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRestApiError {
    fn description(&self) -> &str {
        match *self {
            CreateRestApiError::BadRequest(ref cause) => cause,
            CreateRestApiError::LimitExceeded(ref cause) => cause,
            CreateRestApiError::TooManyRequests(ref cause) => cause,
            CreateRestApiError::Unauthorized(ref cause) => cause,
            CreateRestApiError::Validation(ref cause) => cause,
            CreateRestApiError::Credentials(ref err) => err.description(),
            CreateRestApiError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateRestApiError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStage
#[derive(Debug, PartialEq)]
pub enum CreateStageError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateStageError {
    pub fn from_body(body: &str) -> CreateStageError {
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
                    "BadRequestException" => {
                        CreateStageError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => CreateStageError::Conflict(String::from(error_message)),
                    "LimitExceededException" => {
                        CreateStageError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => CreateStageError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        CreateStageError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateStageError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateStageError::Validation(error_message.to_string())
                    }
                    _ => CreateStageError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateStageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateStageError {
    fn from(err: serde_json::error::Error) -> CreateStageError {
        CreateStageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateStageError {
    fn from(err: CredentialsError) -> CreateStageError {
        CreateStageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStageError {
    fn from(err: HttpDispatchError) -> CreateStageError {
        CreateStageError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStageError {
    fn from(err: io::Error) -> CreateStageError {
        CreateStageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStageError {
    fn description(&self) -> &str {
        match *self {
            CreateStageError::BadRequest(ref cause) => cause,
            CreateStageError::Conflict(ref cause) => cause,
            CreateStageError::LimitExceeded(ref cause) => cause,
            CreateStageError::NotFound(ref cause) => cause,
            CreateStageError::TooManyRequests(ref cause) => cause,
            CreateStageError::Unauthorized(ref cause) => cause,
            CreateStageError::Validation(ref cause) => cause,
            CreateStageError::Credentials(ref err) => err.description(),
            CreateStageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateStageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUsagePlan
#[derive(Debug, PartialEq)]
pub enum CreateUsagePlanError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateUsagePlanError {
    pub fn from_body(body: &str) -> CreateUsagePlanError {
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
                    "BadRequestException" => {
                        CreateUsagePlanError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateUsagePlanError::Conflict(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateUsagePlanError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateUsagePlanError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateUsagePlanError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateUsagePlanError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateUsagePlanError::Validation(error_message.to_string())
                    }
                    _ => CreateUsagePlanError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateUsagePlanError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateUsagePlanError {
    fn from(err: serde_json::error::Error) -> CreateUsagePlanError {
        CreateUsagePlanError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUsagePlanError {
    fn from(err: CredentialsError) -> CreateUsagePlanError {
        CreateUsagePlanError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUsagePlanError {
    fn from(err: HttpDispatchError) -> CreateUsagePlanError {
        CreateUsagePlanError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateUsagePlanError {
    fn from(err: io::Error) -> CreateUsagePlanError {
        CreateUsagePlanError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateUsagePlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUsagePlanError {
    fn description(&self) -> &str {
        match *self {
            CreateUsagePlanError::BadRequest(ref cause) => cause,
            CreateUsagePlanError::Conflict(ref cause) => cause,
            CreateUsagePlanError::LimitExceeded(ref cause) => cause,
            CreateUsagePlanError::NotFound(ref cause) => cause,
            CreateUsagePlanError::TooManyRequests(ref cause) => cause,
            CreateUsagePlanError::Unauthorized(ref cause) => cause,
            CreateUsagePlanError::Validation(ref cause) => cause,
            CreateUsagePlanError::Credentials(ref err) => err.description(),
            CreateUsagePlanError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateUsagePlanError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUsagePlanKey
#[derive(Debug, PartialEq)]
pub enum CreateUsagePlanKeyError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateUsagePlanKeyError {
    pub fn from_body(body: &str) -> CreateUsagePlanKeyError {
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
                    "BadRequestException" => {
                        CreateUsagePlanKeyError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateUsagePlanKeyError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateUsagePlanKeyError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateUsagePlanKeyError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateUsagePlanKeyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateUsagePlanKeyError::Validation(error_message.to_string())
                    }
                    _ => CreateUsagePlanKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateUsagePlanKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateUsagePlanKeyError {
    fn from(err: serde_json::error::Error) -> CreateUsagePlanKeyError {
        CreateUsagePlanKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUsagePlanKeyError {
    fn from(err: CredentialsError) -> CreateUsagePlanKeyError {
        CreateUsagePlanKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUsagePlanKeyError {
    fn from(err: HttpDispatchError) -> CreateUsagePlanKeyError {
        CreateUsagePlanKeyError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateUsagePlanKeyError {
    fn from(err: io::Error) -> CreateUsagePlanKeyError {
        CreateUsagePlanKeyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateUsagePlanKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUsagePlanKeyError {
    fn description(&self) -> &str {
        match *self {
            CreateUsagePlanKeyError::BadRequest(ref cause) => cause,
            CreateUsagePlanKeyError::Conflict(ref cause) => cause,
            CreateUsagePlanKeyError::NotFound(ref cause) => cause,
            CreateUsagePlanKeyError::TooManyRequests(ref cause) => cause,
            CreateUsagePlanKeyError::Unauthorized(ref cause) => cause,
            CreateUsagePlanKeyError::Validation(ref cause) => cause,
            CreateUsagePlanKeyError::Credentials(ref err) => err.description(),
            CreateUsagePlanKeyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateUsagePlanKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateVpcLink
#[derive(Debug, PartialEq)]
pub enum CreateVpcLinkError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl CreateVpcLinkError {
    pub fn from_body(body: &str) -> CreateVpcLinkError {
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
                    "BadRequestException" => {
                        CreateVpcLinkError::BadRequest(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateVpcLinkError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateVpcLinkError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateVpcLinkError::Validation(error_message.to_string())
                    }
                    _ => CreateVpcLinkError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateVpcLinkError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateVpcLinkError {
    fn from(err: serde_json::error::Error) -> CreateVpcLinkError {
        CreateVpcLinkError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateVpcLinkError {
    fn from(err: CredentialsError) -> CreateVpcLinkError {
        CreateVpcLinkError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateVpcLinkError {
    fn from(err: HttpDispatchError) -> CreateVpcLinkError {
        CreateVpcLinkError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateVpcLinkError {
    fn from(err: io::Error) -> CreateVpcLinkError {
        CreateVpcLinkError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateVpcLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateVpcLinkError {
    fn description(&self) -> &str {
        match *self {
            CreateVpcLinkError::BadRequest(ref cause) => cause,
            CreateVpcLinkError::TooManyRequests(ref cause) => cause,
            CreateVpcLinkError::Unauthorized(ref cause) => cause,
            CreateVpcLinkError::Validation(ref cause) => cause,
            CreateVpcLinkError::Credentials(ref err) => err.description(),
            CreateVpcLinkError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateVpcLinkError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApiKey
#[derive(Debug, PartialEq)]
pub enum DeleteApiKeyError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteApiKeyError {
    pub fn from_body(body: &str) -> DeleteApiKeyError {
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
                    "NotFoundException" => DeleteApiKeyError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        DeleteApiKeyError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteApiKeyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteApiKeyError::Validation(error_message.to_string())
                    }
                    _ => DeleteApiKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteApiKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteApiKeyError {
    fn from(err: serde_json::error::Error) -> DeleteApiKeyError {
        DeleteApiKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApiKeyError {
    fn from(err: CredentialsError) -> DeleteApiKeyError {
        DeleteApiKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApiKeyError {
    fn from(err: HttpDispatchError) -> DeleteApiKeyError {
        DeleteApiKeyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApiKeyError {
    fn from(err: io::Error) -> DeleteApiKeyError {
        DeleteApiKeyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApiKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApiKeyError {
    fn description(&self) -> &str {
        match *self {
            DeleteApiKeyError::NotFound(ref cause) => cause,
            DeleteApiKeyError::TooManyRequests(ref cause) => cause,
            DeleteApiKeyError::Unauthorized(ref cause) => cause,
            DeleteApiKeyError::Validation(ref cause) => cause,
            DeleteApiKeyError::Credentials(ref err) => err.description(),
            DeleteApiKeyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteApiKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAuthorizer
#[derive(Debug, PartialEq)]
pub enum DeleteAuthorizerError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteAuthorizerError {
    pub fn from_body(body: &str) -> DeleteAuthorizerError {
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
                    "BadRequestException" => {
                        DeleteAuthorizerError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DeleteAuthorizerError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteAuthorizerError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteAuthorizerError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteAuthorizerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteAuthorizerError::Validation(error_message.to_string())
                    }
                    _ => DeleteAuthorizerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAuthorizerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAuthorizerError {
    fn from(err: serde_json::error::Error) -> DeleteAuthorizerError {
        DeleteAuthorizerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAuthorizerError {
    fn from(err: CredentialsError) -> DeleteAuthorizerError {
        DeleteAuthorizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAuthorizerError {
    fn from(err: HttpDispatchError) -> DeleteAuthorizerError {
        DeleteAuthorizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAuthorizerError {
    fn from(err: io::Error) -> DeleteAuthorizerError {
        DeleteAuthorizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            DeleteAuthorizerError::BadRequest(ref cause) => cause,
            DeleteAuthorizerError::Conflict(ref cause) => cause,
            DeleteAuthorizerError::NotFound(ref cause) => cause,
            DeleteAuthorizerError::TooManyRequests(ref cause) => cause,
            DeleteAuthorizerError::Unauthorized(ref cause) => cause,
            DeleteAuthorizerError::Validation(ref cause) => cause,
            DeleteAuthorizerError::Credentials(ref err) => err.description(),
            DeleteAuthorizerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAuthorizerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBasePathMapping
#[derive(Debug, PartialEq)]
pub enum DeleteBasePathMappingError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteBasePathMappingError {
    pub fn from_body(body: &str) -> DeleteBasePathMappingError {
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
                    "BadRequestException" => {
                        DeleteBasePathMappingError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DeleteBasePathMappingError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteBasePathMappingError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteBasePathMappingError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteBasePathMappingError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteBasePathMappingError::Validation(error_message.to_string())
                    }
                    _ => DeleteBasePathMappingError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBasePathMappingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteBasePathMappingError {
    fn from(err: serde_json::error::Error) -> DeleteBasePathMappingError {
        DeleteBasePathMappingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBasePathMappingError {
    fn from(err: CredentialsError) -> DeleteBasePathMappingError {
        DeleteBasePathMappingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBasePathMappingError {
    fn from(err: HttpDispatchError) -> DeleteBasePathMappingError {
        DeleteBasePathMappingError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBasePathMappingError {
    fn from(err: io::Error) -> DeleteBasePathMappingError {
        DeleteBasePathMappingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBasePathMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBasePathMappingError {
    fn description(&self) -> &str {
        match *self {
            DeleteBasePathMappingError::BadRequest(ref cause) => cause,
            DeleteBasePathMappingError::Conflict(ref cause) => cause,
            DeleteBasePathMappingError::NotFound(ref cause) => cause,
            DeleteBasePathMappingError::TooManyRequests(ref cause) => cause,
            DeleteBasePathMappingError::Unauthorized(ref cause) => cause,
            DeleteBasePathMappingError::Validation(ref cause) => cause,
            DeleteBasePathMappingError::Credentials(ref err) => err.description(),
            DeleteBasePathMappingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBasePathMappingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteClientCertificate
#[derive(Debug, PartialEq)]
pub enum DeleteClientCertificateError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteClientCertificateError {
    pub fn from_body(body: &str) -> DeleteClientCertificateError {
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
                    "BadRequestException" => {
                        DeleteClientCertificateError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteClientCertificateError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteClientCertificateError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteClientCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteClientCertificateError::Validation(error_message.to_string())
                    }
                    _ => DeleteClientCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteClientCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteClientCertificateError {
    fn from(err: serde_json::error::Error) -> DeleteClientCertificateError {
        DeleteClientCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteClientCertificateError {
    fn from(err: CredentialsError) -> DeleteClientCertificateError {
        DeleteClientCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteClientCertificateError {
    fn from(err: HttpDispatchError) -> DeleteClientCertificateError {
        DeleteClientCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteClientCertificateError {
    fn from(err: io::Error) -> DeleteClientCertificateError {
        DeleteClientCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteClientCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClientCertificateError {
    fn description(&self) -> &str {
        match *self {
            DeleteClientCertificateError::BadRequest(ref cause) => cause,
            DeleteClientCertificateError::NotFound(ref cause) => cause,
            DeleteClientCertificateError::TooManyRequests(ref cause) => cause,
            DeleteClientCertificateError::Unauthorized(ref cause) => cause,
            DeleteClientCertificateError::Validation(ref cause) => cause,
            DeleteClientCertificateError::Credentials(ref err) => err.description(),
            DeleteClientCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteClientCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDeployment
#[derive(Debug, PartialEq)]
pub enum DeleteDeploymentError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteDeploymentError {
    pub fn from_body(body: &str) -> DeleteDeploymentError {
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
                    "BadRequestException" => {
                        DeleteDeploymentError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteDeploymentError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteDeploymentError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteDeploymentError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDeploymentError::Validation(error_message.to_string())
                    }
                    _ => DeleteDeploymentError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDeploymentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDeploymentError {
    fn from(err: serde_json::error::Error) -> DeleteDeploymentError {
        DeleteDeploymentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDeploymentError {
    fn from(err: CredentialsError) -> DeleteDeploymentError {
        DeleteDeploymentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDeploymentError {
    fn from(err: HttpDispatchError) -> DeleteDeploymentError {
        DeleteDeploymentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDeploymentError {
    fn from(err: io::Error) -> DeleteDeploymentError {
        DeleteDeploymentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDeploymentError {
    fn description(&self) -> &str {
        match *self {
            DeleteDeploymentError::BadRequest(ref cause) => cause,
            DeleteDeploymentError::NotFound(ref cause) => cause,
            DeleteDeploymentError::TooManyRequests(ref cause) => cause,
            DeleteDeploymentError::Unauthorized(ref cause) => cause,
            DeleteDeploymentError::Validation(ref cause) => cause,
            DeleteDeploymentError::Credentials(ref err) => err.description(),
            DeleteDeploymentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDeploymentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDocumentationPart
#[derive(Debug, PartialEq)]
pub enum DeleteDocumentationPartError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteDocumentationPartError {
    pub fn from_body(body: &str) -> DeleteDocumentationPartError {
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
                    "BadRequestException" => {
                        DeleteDocumentationPartError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DeleteDocumentationPartError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteDocumentationPartError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteDocumentationPartError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteDocumentationPartError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDocumentationPartError::Validation(error_message.to_string())
                    }
                    _ => DeleteDocumentationPartError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDocumentationPartError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDocumentationPartError {
    fn from(err: serde_json::error::Error) -> DeleteDocumentationPartError {
        DeleteDocumentationPartError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDocumentationPartError {
    fn from(err: CredentialsError) -> DeleteDocumentationPartError {
        DeleteDocumentationPartError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDocumentationPartError {
    fn from(err: HttpDispatchError) -> DeleteDocumentationPartError {
        DeleteDocumentationPartError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDocumentationPartError {
    fn from(err: io::Error) -> DeleteDocumentationPartError {
        DeleteDocumentationPartError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDocumentationPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDocumentationPartError {
    fn description(&self) -> &str {
        match *self {
            DeleteDocumentationPartError::BadRequest(ref cause) => cause,
            DeleteDocumentationPartError::Conflict(ref cause) => cause,
            DeleteDocumentationPartError::NotFound(ref cause) => cause,
            DeleteDocumentationPartError::TooManyRequests(ref cause) => cause,
            DeleteDocumentationPartError::Unauthorized(ref cause) => cause,
            DeleteDocumentationPartError::Validation(ref cause) => cause,
            DeleteDocumentationPartError::Credentials(ref err) => err.description(),
            DeleteDocumentationPartError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDocumentationPartError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDocumentationVersion
#[derive(Debug, PartialEq)]
pub enum DeleteDocumentationVersionError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteDocumentationVersionError {
    pub fn from_body(body: &str) -> DeleteDocumentationVersionError {
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
                    "BadRequestException" => {
                        DeleteDocumentationVersionError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DeleteDocumentationVersionError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteDocumentationVersionError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => DeleteDocumentationVersionError::TooManyRequests(
                        String::from(error_message),
                    ),
                    "UnauthorizedException" => {
                        DeleteDocumentationVersionError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDocumentationVersionError::Validation(error_message.to_string())
                    }
                    _ => DeleteDocumentationVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDocumentationVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDocumentationVersionError {
    fn from(err: serde_json::error::Error) -> DeleteDocumentationVersionError {
        DeleteDocumentationVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDocumentationVersionError {
    fn from(err: CredentialsError) -> DeleteDocumentationVersionError {
        DeleteDocumentationVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDocumentationVersionError {
    fn from(err: HttpDispatchError) -> DeleteDocumentationVersionError {
        DeleteDocumentationVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDocumentationVersionError {
    fn from(err: io::Error) -> DeleteDocumentationVersionError {
        DeleteDocumentationVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDocumentationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDocumentationVersionError {
    fn description(&self) -> &str {
        match *self {
            DeleteDocumentationVersionError::BadRequest(ref cause) => cause,
            DeleteDocumentationVersionError::Conflict(ref cause) => cause,
            DeleteDocumentationVersionError::NotFound(ref cause) => cause,
            DeleteDocumentationVersionError::TooManyRequests(ref cause) => cause,
            DeleteDocumentationVersionError::Unauthorized(ref cause) => cause,
            DeleteDocumentationVersionError::Validation(ref cause) => cause,
            DeleteDocumentationVersionError::Credentials(ref err) => err.description(),
            DeleteDocumentationVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDocumentationVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDomainName
#[derive(Debug, PartialEq)]
pub enum DeleteDomainNameError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteDomainNameError {
    pub fn from_body(body: &str) -> DeleteDomainNameError {
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
                    "NotFoundException" => {
                        DeleteDomainNameError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteDomainNameError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteDomainNameError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDomainNameError::Validation(error_message.to_string())
                    }
                    _ => DeleteDomainNameError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDomainNameError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDomainNameError {
    fn from(err: serde_json::error::Error) -> DeleteDomainNameError {
        DeleteDomainNameError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDomainNameError {
    fn from(err: CredentialsError) -> DeleteDomainNameError {
        DeleteDomainNameError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDomainNameError {
    fn from(err: HttpDispatchError) -> DeleteDomainNameError {
        DeleteDomainNameError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDomainNameError {
    fn from(err: io::Error) -> DeleteDomainNameError {
        DeleteDomainNameError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDomainNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDomainNameError {
    fn description(&self) -> &str {
        match *self {
            DeleteDomainNameError::NotFound(ref cause) => cause,
            DeleteDomainNameError::TooManyRequests(ref cause) => cause,
            DeleteDomainNameError::Unauthorized(ref cause) => cause,
            DeleteDomainNameError::Validation(ref cause) => cause,
            DeleteDomainNameError::Credentials(ref err) => err.description(),
            DeleteDomainNameError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDomainNameError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteGatewayResponse
#[derive(Debug, PartialEq)]
pub enum DeleteGatewayResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteGatewayResponseError {
    pub fn from_body(body: &str) -> DeleteGatewayResponseError {
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
                    "BadRequestException" => {
                        DeleteGatewayResponseError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DeleteGatewayResponseError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteGatewayResponseError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteGatewayResponseError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteGatewayResponseError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteGatewayResponseError::Validation(error_message.to_string())
                    }
                    _ => DeleteGatewayResponseError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteGatewayResponseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteGatewayResponseError {
    fn from(err: serde_json::error::Error) -> DeleteGatewayResponseError {
        DeleteGatewayResponseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteGatewayResponseError {
    fn from(err: CredentialsError) -> DeleteGatewayResponseError {
        DeleteGatewayResponseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteGatewayResponseError {
    fn from(err: HttpDispatchError) -> DeleteGatewayResponseError {
        DeleteGatewayResponseError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteGatewayResponseError {
    fn from(err: io::Error) -> DeleteGatewayResponseError {
        DeleteGatewayResponseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteGatewayResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGatewayResponseError {
    fn description(&self) -> &str {
        match *self {
            DeleteGatewayResponseError::BadRequest(ref cause) => cause,
            DeleteGatewayResponseError::Conflict(ref cause) => cause,
            DeleteGatewayResponseError::NotFound(ref cause) => cause,
            DeleteGatewayResponseError::TooManyRequests(ref cause) => cause,
            DeleteGatewayResponseError::Unauthorized(ref cause) => cause,
            DeleteGatewayResponseError::Validation(ref cause) => cause,
            DeleteGatewayResponseError::Credentials(ref err) => err.description(),
            DeleteGatewayResponseError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteGatewayResponseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIntegration
#[derive(Debug, PartialEq)]
pub enum DeleteIntegrationError {
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteIntegrationError {
    pub fn from_body(body: &str) -> DeleteIntegrationError {
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
                    "ConflictException" => {
                        DeleteIntegrationError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteIntegrationError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteIntegrationError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteIntegrationError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteIntegrationError::Validation(error_message.to_string())
                    }
                    _ => DeleteIntegrationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteIntegrationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteIntegrationError {
    fn from(err: serde_json::error::Error) -> DeleteIntegrationError {
        DeleteIntegrationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteIntegrationError {
    fn from(err: CredentialsError) -> DeleteIntegrationError {
        DeleteIntegrationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteIntegrationError {
    fn from(err: HttpDispatchError) -> DeleteIntegrationError {
        DeleteIntegrationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteIntegrationError {
    fn from(err: io::Error) -> DeleteIntegrationError {
        DeleteIntegrationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteIntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIntegrationError {
    fn description(&self) -> &str {
        match *self {
            DeleteIntegrationError::Conflict(ref cause) => cause,
            DeleteIntegrationError::NotFound(ref cause) => cause,
            DeleteIntegrationError::TooManyRequests(ref cause) => cause,
            DeleteIntegrationError::Unauthorized(ref cause) => cause,
            DeleteIntegrationError::Validation(ref cause) => cause,
            DeleteIntegrationError::Credentials(ref err) => err.description(),
            DeleteIntegrationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteIntegrationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum DeleteIntegrationResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteIntegrationResponseError {
    pub fn from_body(body: &str) -> DeleteIntegrationResponseError {
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
                    "BadRequestException" => {
                        DeleteIntegrationResponseError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DeleteIntegrationResponseError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteIntegrationResponseError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteIntegrationResponseError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteIntegrationResponseError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteIntegrationResponseError::Validation(error_message.to_string())
                    }
                    _ => DeleteIntegrationResponseError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteIntegrationResponseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteIntegrationResponseError {
    fn from(err: serde_json::error::Error) -> DeleteIntegrationResponseError {
        DeleteIntegrationResponseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteIntegrationResponseError {
    fn from(err: CredentialsError) -> DeleteIntegrationResponseError {
        DeleteIntegrationResponseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteIntegrationResponseError {
    fn from(err: HttpDispatchError) -> DeleteIntegrationResponseError {
        DeleteIntegrationResponseError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteIntegrationResponseError {
    fn from(err: io::Error) -> DeleteIntegrationResponseError {
        DeleteIntegrationResponseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteIntegrationResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIntegrationResponseError {
    fn description(&self) -> &str {
        match *self {
            DeleteIntegrationResponseError::BadRequest(ref cause) => cause,
            DeleteIntegrationResponseError::Conflict(ref cause) => cause,
            DeleteIntegrationResponseError::NotFound(ref cause) => cause,
            DeleteIntegrationResponseError::TooManyRequests(ref cause) => cause,
            DeleteIntegrationResponseError::Unauthorized(ref cause) => cause,
            DeleteIntegrationResponseError::Validation(ref cause) => cause,
            DeleteIntegrationResponseError::Credentials(ref err) => err.description(),
            DeleteIntegrationResponseError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteIntegrationResponseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteMethod
#[derive(Debug, PartialEq)]
pub enum DeleteMethodError {
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteMethodError {
    pub fn from_body(body: &str) -> DeleteMethodError {
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
                    "ConflictException" => DeleteMethodError::Conflict(String::from(error_message)),
                    "NotFoundException" => DeleteMethodError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        DeleteMethodError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteMethodError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteMethodError::Validation(error_message.to_string())
                    }
                    _ => DeleteMethodError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteMethodError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteMethodError {
    fn from(err: serde_json::error::Error) -> DeleteMethodError {
        DeleteMethodError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteMethodError {
    fn from(err: CredentialsError) -> DeleteMethodError {
        DeleteMethodError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteMethodError {
    fn from(err: HttpDispatchError) -> DeleteMethodError {
        DeleteMethodError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteMethodError {
    fn from(err: io::Error) -> DeleteMethodError {
        DeleteMethodError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMethodError {
    fn description(&self) -> &str {
        match *self {
            DeleteMethodError::Conflict(ref cause) => cause,
            DeleteMethodError::NotFound(ref cause) => cause,
            DeleteMethodError::TooManyRequests(ref cause) => cause,
            DeleteMethodError::Unauthorized(ref cause) => cause,
            DeleteMethodError::Validation(ref cause) => cause,
            DeleteMethodError::Credentials(ref err) => err.description(),
            DeleteMethodError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteMethodError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteMethodResponse
#[derive(Debug, PartialEq)]
pub enum DeleteMethodResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteMethodResponseError {
    pub fn from_body(body: &str) -> DeleteMethodResponseError {
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
                    "BadRequestException" => {
                        DeleteMethodResponseError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DeleteMethodResponseError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteMethodResponseError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteMethodResponseError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteMethodResponseError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteMethodResponseError::Validation(error_message.to_string())
                    }
                    _ => DeleteMethodResponseError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteMethodResponseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteMethodResponseError {
    fn from(err: serde_json::error::Error) -> DeleteMethodResponseError {
        DeleteMethodResponseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteMethodResponseError {
    fn from(err: CredentialsError) -> DeleteMethodResponseError {
        DeleteMethodResponseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteMethodResponseError {
    fn from(err: HttpDispatchError) -> DeleteMethodResponseError {
        DeleteMethodResponseError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteMethodResponseError {
    fn from(err: io::Error) -> DeleteMethodResponseError {
        DeleteMethodResponseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteMethodResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMethodResponseError {
    fn description(&self) -> &str {
        match *self {
            DeleteMethodResponseError::BadRequest(ref cause) => cause,
            DeleteMethodResponseError::Conflict(ref cause) => cause,
            DeleteMethodResponseError::NotFound(ref cause) => cause,
            DeleteMethodResponseError::TooManyRequests(ref cause) => cause,
            DeleteMethodResponseError::Unauthorized(ref cause) => cause,
            DeleteMethodResponseError::Validation(ref cause) => cause,
            DeleteMethodResponseError::Credentials(ref err) => err.description(),
            DeleteMethodResponseError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteMethodResponseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteModel
#[derive(Debug, PartialEq)]
pub enum DeleteModelError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteModelError {
    pub fn from_body(body: &str) -> DeleteModelError {
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
                    "BadRequestException" => {
                        DeleteModelError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => DeleteModelError::Conflict(String::from(error_message)),
                    "NotFoundException" => DeleteModelError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        DeleteModelError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteModelError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteModelError::Validation(error_message.to_string())
                    }
                    _ => DeleteModelError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteModelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteModelError {
    fn from(err: serde_json::error::Error) -> DeleteModelError {
        DeleteModelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteModelError {
    fn from(err: CredentialsError) -> DeleteModelError {
        DeleteModelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteModelError {
    fn from(err: HttpDispatchError) -> DeleteModelError {
        DeleteModelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteModelError {
    fn from(err: io::Error) -> DeleteModelError {
        DeleteModelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteModelError {
    fn description(&self) -> &str {
        match *self {
            DeleteModelError::BadRequest(ref cause) => cause,
            DeleteModelError::Conflict(ref cause) => cause,
            DeleteModelError::NotFound(ref cause) => cause,
            DeleteModelError::TooManyRequests(ref cause) => cause,
            DeleteModelError::Unauthorized(ref cause) => cause,
            DeleteModelError::Validation(ref cause) => cause,
            DeleteModelError::Credentials(ref err) => err.description(),
            DeleteModelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteModelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRequestValidator
#[derive(Debug, PartialEq)]
pub enum DeleteRequestValidatorError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteRequestValidatorError {
    pub fn from_body(body: &str) -> DeleteRequestValidatorError {
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
                    "BadRequestException" => {
                        DeleteRequestValidatorError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DeleteRequestValidatorError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteRequestValidatorError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteRequestValidatorError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteRequestValidatorError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteRequestValidatorError::Validation(error_message.to_string())
                    }
                    _ => DeleteRequestValidatorError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRequestValidatorError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRequestValidatorError {
    fn from(err: serde_json::error::Error) -> DeleteRequestValidatorError {
        DeleteRequestValidatorError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRequestValidatorError {
    fn from(err: CredentialsError) -> DeleteRequestValidatorError {
        DeleteRequestValidatorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRequestValidatorError {
    fn from(err: HttpDispatchError) -> DeleteRequestValidatorError {
        DeleteRequestValidatorError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRequestValidatorError {
    fn from(err: io::Error) -> DeleteRequestValidatorError {
        DeleteRequestValidatorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteRequestValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRequestValidatorError {
    fn description(&self) -> &str {
        match *self {
            DeleteRequestValidatorError::BadRequest(ref cause) => cause,
            DeleteRequestValidatorError::Conflict(ref cause) => cause,
            DeleteRequestValidatorError::NotFound(ref cause) => cause,
            DeleteRequestValidatorError::TooManyRequests(ref cause) => cause,
            DeleteRequestValidatorError::Unauthorized(ref cause) => cause,
            DeleteRequestValidatorError::Validation(ref cause) => cause,
            DeleteRequestValidatorError::Credentials(ref err) => err.description(),
            DeleteRequestValidatorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteRequestValidatorError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteResource
#[derive(Debug, PartialEq)]
pub enum DeleteResourceError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteResourceError {
    pub fn from_body(body: &str) -> DeleteResourceError {
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
                    "BadRequestException" => {
                        DeleteResourceError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DeleteResourceError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteResourceError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteResourceError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteResourceError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteResourceError::Validation(error_message.to_string())
                    }
                    _ => DeleteResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteResourceError {
    fn from(err: serde_json::error::Error) -> DeleteResourceError {
        DeleteResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteResourceError {
    fn from(err: CredentialsError) -> DeleteResourceError {
        DeleteResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteResourceError {
    fn from(err: HttpDispatchError) -> DeleteResourceError {
        DeleteResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteResourceError {
    fn from(err: io::Error) -> DeleteResourceError {
        DeleteResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteResourceError {
    fn description(&self) -> &str {
        match *self {
            DeleteResourceError::BadRequest(ref cause) => cause,
            DeleteResourceError::Conflict(ref cause) => cause,
            DeleteResourceError::NotFound(ref cause) => cause,
            DeleteResourceError::TooManyRequests(ref cause) => cause,
            DeleteResourceError::Unauthorized(ref cause) => cause,
            DeleteResourceError::Validation(ref cause) => cause,
            DeleteResourceError::Credentials(ref err) => err.description(),
            DeleteResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRestApi
#[derive(Debug, PartialEq)]
pub enum DeleteRestApiError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteRestApiError {
    pub fn from_body(body: &str) -> DeleteRestApiError {
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
                    "BadRequestException" => {
                        DeleteRestApiError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteRestApiError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteRestApiError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteRestApiError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteRestApiError::Validation(error_message.to_string())
                    }
                    _ => DeleteRestApiError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRestApiError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRestApiError {
    fn from(err: serde_json::error::Error) -> DeleteRestApiError {
        DeleteRestApiError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRestApiError {
    fn from(err: CredentialsError) -> DeleteRestApiError {
        DeleteRestApiError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRestApiError {
    fn from(err: HttpDispatchError) -> DeleteRestApiError {
        DeleteRestApiError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRestApiError {
    fn from(err: io::Error) -> DeleteRestApiError {
        DeleteRestApiError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteRestApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRestApiError {
    fn description(&self) -> &str {
        match *self {
            DeleteRestApiError::BadRequest(ref cause) => cause,
            DeleteRestApiError::NotFound(ref cause) => cause,
            DeleteRestApiError::TooManyRequests(ref cause) => cause,
            DeleteRestApiError::Unauthorized(ref cause) => cause,
            DeleteRestApiError::Validation(ref cause) => cause,
            DeleteRestApiError::Credentials(ref err) => err.description(),
            DeleteRestApiError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteRestApiError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteStage
#[derive(Debug, PartialEq)]
pub enum DeleteStageError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteStageError {
    pub fn from_body(body: &str) -> DeleteStageError {
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
                    "BadRequestException" => {
                        DeleteStageError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => DeleteStageError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        DeleteStageError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteStageError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteStageError::Validation(error_message.to_string())
                    }
                    _ => DeleteStageError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteStageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteStageError {
    fn from(err: serde_json::error::Error) -> DeleteStageError {
        DeleteStageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteStageError {
    fn from(err: CredentialsError) -> DeleteStageError {
        DeleteStageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteStageError {
    fn from(err: HttpDispatchError) -> DeleteStageError {
        DeleteStageError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteStageError {
    fn from(err: io::Error) -> DeleteStageError {
        DeleteStageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteStageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteStageError {
    fn description(&self) -> &str {
        match *self {
            DeleteStageError::BadRequest(ref cause) => cause,
            DeleteStageError::NotFound(ref cause) => cause,
            DeleteStageError::TooManyRequests(ref cause) => cause,
            DeleteStageError::Unauthorized(ref cause) => cause,
            DeleteStageError::Validation(ref cause) => cause,
            DeleteStageError::Credentials(ref err) => err.description(),
            DeleteStageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteStageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUsagePlan
#[derive(Debug, PartialEq)]
pub enum DeleteUsagePlanError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteUsagePlanError {
    pub fn from_body(body: &str) -> DeleteUsagePlanError {
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
                    "BadRequestException" => {
                        DeleteUsagePlanError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteUsagePlanError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteUsagePlanError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteUsagePlanError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteUsagePlanError::Validation(error_message.to_string())
                    }
                    _ => DeleteUsagePlanError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteUsagePlanError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteUsagePlanError {
    fn from(err: serde_json::error::Error) -> DeleteUsagePlanError {
        DeleteUsagePlanError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUsagePlanError {
    fn from(err: CredentialsError) -> DeleteUsagePlanError {
        DeleteUsagePlanError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUsagePlanError {
    fn from(err: HttpDispatchError) -> DeleteUsagePlanError {
        DeleteUsagePlanError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteUsagePlanError {
    fn from(err: io::Error) -> DeleteUsagePlanError {
        DeleteUsagePlanError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteUsagePlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUsagePlanError {
    fn description(&self) -> &str {
        match *self {
            DeleteUsagePlanError::BadRequest(ref cause) => cause,
            DeleteUsagePlanError::NotFound(ref cause) => cause,
            DeleteUsagePlanError::TooManyRequests(ref cause) => cause,
            DeleteUsagePlanError::Unauthorized(ref cause) => cause,
            DeleteUsagePlanError::Validation(ref cause) => cause,
            DeleteUsagePlanError::Credentials(ref err) => err.description(),
            DeleteUsagePlanError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteUsagePlanError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUsagePlanKey
#[derive(Debug, PartialEq)]
pub enum DeleteUsagePlanKeyError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteUsagePlanKeyError {
    pub fn from_body(body: &str) -> DeleteUsagePlanKeyError {
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
                    "BadRequestException" => {
                        DeleteUsagePlanKeyError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        DeleteUsagePlanKeyError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteUsagePlanKeyError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteUsagePlanKeyError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteUsagePlanKeyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteUsagePlanKeyError::Validation(error_message.to_string())
                    }
                    _ => DeleteUsagePlanKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteUsagePlanKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteUsagePlanKeyError {
    fn from(err: serde_json::error::Error) -> DeleteUsagePlanKeyError {
        DeleteUsagePlanKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUsagePlanKeyError {
    fn from(err: CredentialsError) -> DeleteUsagePlanKeyError {
        DeleteUsagePlanKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUsagePlanKeyError {
    fn from(err: HttpDispatchError) -> DeleteUsagePlanKeyError {
        DeleteUsagePlanKeyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteUsagePlanKeyError {
    fn from(err: io::Error) -> DeleteUsagePlanKeyError {
        DeleteUsagePlanKeyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteUsagePlanKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUsagePlanKeyError {
    fn description(&self) -> &str {
        match *self {
            DeleteUsagePlanKeyError::BadRequest(ref cause) => cause,
            DeleteUsagePlanKeyError::Conflict(ref cause) => cause,
            DeleteUsagePlanKeyError::NotFound(ref cause) => cause,
            DeleteUsagePlanKeyError::TooManyRequests(ref cause) => cause,
            DeleteUsagePlanKeyError::Unauthorized(ref cause) => cause,
            DeleteUsagePlanKeyError::Validation(ref cause) => cause,
            DeleteUsagePlanKeyError::Credentials(ref err) => err.description(),
            DeleteUsagePlanKeyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteUsagePlanKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVpcLink
#[derive(Debug, PartialEq)]
pub enum DeleteVpcLinkError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl DeleteVpcLinkError {
    pub fn from_body(body: &str) -> DeleteVpcLinkError {
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
                    "BadRequestException" => {
                        DeleteVpcLinkError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteVpcLinkError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteVpcLinkError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteVpcLinkError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteVpcLinkError::Validation(error_message.to_string())
                    }
                    _ => DeleteVpcLinkError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteVpcLinkError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteVpcLinkError {
    fn from(err: serde_json::error::Error) -> DeleteVpcLinkError {
        DeleteVpcLinkError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteVpcLinkError {
    fn from(err: CredentialsError) -> DeleteVpcLinkError {
        DeleteVpcLinkError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteVpcLinkError {
    fn from(err: HttpDispatchError) -> DeleteVpcLinkError {
        DeleteVpcLinkError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteVpcLinkError {
    fn from(err: io::Error) -> DeleteVpcLinkError {
        DeleteVpcLinkError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteVpcLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVpcLinkError {
    fn description(&self) -> &str {
        match *self {
            DeleteVpcLinkError::BadRequest(ref cause) => cause,
            DeleteVpcLinkError::NotFound(ref cause) => cause,
            DeleteVpcLinkError::TooManyRequests(ref cause) => cause,
            DeleteVpcLinkError::Unauthorized(ref cause) => cause,
            DeleteVpcLinkError::Validation(ref cause) => cause,
            DeleteVpcLinkError::Credentials(ref err) => err.description(),
            DeleteVpcLinkError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteVpcLinkError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by FlushStageAuthorizersCache
#[derive(Debug, PartialEq)]
pub enum FlushStageAuthorizersCacheError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl FlushStageAuthorizersCacheError {
    pub fn from_body(body: &str) -> FlushStageAuthorizersCacheError {
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
                    "BadRequestException" => {
                        FlushStageAuthorizersCacheError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        FlushStageAuthorizersCacheError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => FlushStageAuthorizersCacheError::TooManyRequests(
                        String::from(error_message),
                    ),
                    "UnauthorizedException" => {
                        FlushStageAuthorizersCacheError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        FlushStageAuthorizersCacheError::Validation(error_message.to_string())
                    }
                    _ => FlushStageAuthorizersCacheError::Unknown(String::from(body)),
                }
            }
            Err(_) => FlushStageAuthorizersCacheError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for FlushStageAuthorizersCacheError {
    fn from(err: serde_json::error::Error) -> FlushStageAuthorizersCacheError {
        FlushStageAuthorizersCacheError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for FlushStageAuthorizersCacheError {
    fn from(err: CredentialsError) -> FlushStageAuthorizersCacheError {
        FlushStageAuthorizersCacheError::Credentials(err)
    }
}
impl From<HttpDispatchError> for FlushStageAuthorizersCacheError {
    fn from(err: HttpDispatchError) -> FlushStageAuthorizersCacheError {
        FlushStageAuthorizersCacheError::HttpDispatch(err)
    }
}
impl From<io::Error> for FlushStageAuthorizersCacheError {
    fn from(err: io::Error) -> FlushStageAuthorizersCacheError {
        FlushStageAuthorizersCacheError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for FlushStageAuthorizersCacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for FlushStageAuthorizersCacheError {
    fn description(&self) -> &str {
        match *self {
            FlushStageAuthorizersCacheError::BadRequest(ref cause) => cause,
            FlushStageAuthorizersCacheError::NotFound(ref cause) => cause,
            FlushStageAuthorizersCacheError::TooManyRequests(ref cause) => cause,
            FlushStageAuthorizersCacheError::Unauthorized(ref cause) => cause,
            FlushStageAuthorizersCacheError::Validation(ref cause) => cause,
            FlushStageAuthorizersCacheError::Credentials(ref err) => err.description(),
            FlushStageAuthorizersCacheError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            FlushStageAuthorizersCacheError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by FlushStageCache
#[derive(Debug, PartialEq)]
pub enum FlushStageCacheError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl FlushStageCacheError {
    pub fn from_body(body: &str) -> FlushStageCacheError {
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
                    "BadRequestException" => {
                        FlushStageCacheError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        FlushStageCacheError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        FlushStageCacheError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        FlushStageCacheError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        FlushStageCacheError::Validation(error_message.to_string())
                    }
                    _ => FlushStageCacheError::Unknown(String::from(body)),
                }
            }
            Err(_) => FlushStageCacheError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for FlushStageCacheError {
    fn from(err: serde_json::error::Error) -> FlushStageCacheError {
        FlushStageCacheError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for FlushStageCacheError {
    fn from(err: CredentialsError) -> FlushStageCacheError {
        FlushStageCacheError::Credentials(err)
    }
}
impl From<HttpDispatchError> for FlushStageCacheError {
    fn from(err: HttpDispatchError) -> FlushStageCacheError {
        FlushStageCacheError::HttpDispatch(err)
    }
}
impl From<io::Error> for FlushStageCacheError {
    fn from(err: io::Error) -> FlushStageCacheError {
        FlushStageCacheError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for FlushStageCacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for FlushStageCacheError {
    fn description(&self) -> &str {
        match *self {
            FlushStageCacheError::BadRequest(ref cause) => cause,
            FlushStageCacheError::NotFound(ref cause) => cause,
            FlushStageCacheError::TooManyRequests(ref cause) => cause,
            FlushStageCacheError::Unauthorized(ref cause) => cause,
            FlushStageCacheError::Validation(ref cause) => cause,
            FlushStageCacheError::Credentials(ref err) => err.description(),
            FlushStageCacheError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            FlushStageCacheError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GenerateClientCertificate
#[derive(Debug, PartialEq)]
pub enum GenerateClientCertificateError {
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GenerateClientCertificateError {
    pub fn from_body(body: &str) -> GenerateClientCertificateError {
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
                    "LimitExceededException" => {
                        GenerateClientCertificateError::LimitExceeded(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GenerateClientCertificateError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GenerateClientCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GenerateClientCertificateError::Validation(error_message.to_string())
                    }
                    _ => GenerateClientCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => GenerateClientCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GenerateClientCertificateError {
    fn from(err: serde_json::error::Error) -> GenerateClientCertificateError {
        GenerateClientCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GenerateClientCertificateError {
    fn from(err: CredentialsError) -> GenerateClientCertificateError {
        GenerateClientCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GenerateClientCertificateError {
    fn from(err: HttpDispatchError) -> GenerateClientCertificateError {
        GenerateClientCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GenerateClientCertificateError {
    fn from(err: io::Error) -> GenerateClientCertificateError {
        GenerateClientCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GenerateClientCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GenerateClientCertificateError {
    fn description(&self) -> &str {
        match *self {
            GenerateClientCertificateError::LimitExceeded(ref cause) => cause,
            GenerateClientCertificateError::TooManyRequests(ref cause) => cause,
            GenerateClientCertificateError::Unauthorized(ref cause) => cause,
            GenerateClientCertificateError::Validation(ref cause) => cause,
            GenerateClientCertificateError::Credentials(ref err) => err.description(),
            GenerateClientCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GenerateClientCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAccount
#[derive(Debug, PartialEq)]
pub enum GetAccountError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetAccountError {
    pub fn from_body(body: &str) -> GetAccountError {
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
                    "NotFoundException" => GetAccountError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetAccountError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetAccountError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetAccountError::Validation(error_message.to_string()),
                    _ => GetAccountError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAccountError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAccountError {
    fn from(err: serde_json::error::Error) -> GetAccountError {
        GetAccountError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAccountError {
    fn from(err: CredentialsError) -> GetAccountError {
        GetAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAccountError {
    fn from(err: HttpDispatchError) -> GetAccountError {
        GetAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAccountError {
    fn from(err: io::Error) -> GetAccountError {
        GetAccountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAccountError {
    fn description(&self) -> &str {
        match *self {
            GetAccountError::NotFound(ref cause) => cause,
            GetAccountError::TooManyRequests(ref cause) => cause,
            GetAccountError::Unauthorized(ref cause) => cause,
            GetAccountError::Validation(ref cause) => cause,
            GetAccountError::Credentials(ref err) => err.description(),
            GetAccountError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAccountError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApiKey
#[derive(Debug, PartialEq)]
pub enum GetApiKeyError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetApiKeyError {
    pub fn from_body(body: &str) -> GetApiKeyError {
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
                    "NotFoundException" => GetApiKeyError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetApiKeyError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetApiKeyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetApiKeyError::Validation(error_message.to_string()),
                    _ => GetApiKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetApiKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetApiKeyError {
    fn from(err: serde_json::error::Error) -> GetApiKeyError {
        GetApiKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApiKeyError {
    fn from(err: CredentialsError) -> GetApiKeyError {
        GetApiKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApiKeyError {
    fn from(err: HttpDispatchError) -> GetApiKeyError {
        GetApiKeyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApiKeyError {
    fn from(err: io::Error) -> GetApiKeyError {
        GetApiKeyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApiKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApiKeyError {
    fn description(&self) -> &str {
        match *self {
            GetApiKeyError::NotFound(ref cause) => cause,
            GetApiKeyError::TooManyRequests(ref cause) => cause,
            GetApiKeyError::Unauthorized(ref cause) => cause,
            GetApiKeyError::Validation(ref cause) => cause,
            GetApiKeyError::Credentials(ref err) => err.description(),
            GetApiKeyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetApiKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApiKeys
#[derive(Debug, PartialEq)]
pub enum GetApiKeysError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetApiKeysError {
    pub fn from_body(body: &str) -> GetApiKeysError {
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
                    "BadRequestException" => {
                        GetApiKeysError::BadRequest(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetApiKeysError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetApiKeysError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetApiKeysError::Validation(error_message.to_string()),
                    _ => GetApiKeysError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetApiKeysError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetApiKeysError {
    fn from(err: serde_json::error::Error) -> GetApiKeysError {
        GetApiKeysError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApiKeysError {
    fn from(err: CredentialsError) -> GetApiKeysError {
        GetApiKeysError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApiKeysError {
    fn from(err: HttpDispatchError) -> GetApiKeysError {
        GetApiKeysError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApiKeysError {
    fn from(err: io::Error) -> GetApiKeysError {
        GetApiKeysError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApiKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApiKeysError {
    fn description(&self) -> &str {
        match *self {
            GetApiKeysError::BadRequest(ref cause) => cause,
            GetApiKeysError::TooManyRequests(ref cause) => cause,
            GetApiKeysError::Unauthorized(ref cause) => cause,
            GetApiKeysError::Validation(ref cause) => cause,
            GetApiKeysError::Credentials(ref err) => err.description(),
            GetApiKeysError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetApiKeysError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAuthorizer
#[derive(Debug, PartialEq)]
pub enum GetAuthorizerError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetAuthorizerError {
    pub fn from_body(body: &str) -> GetAuthorizerError {
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
                    "NotFoundException" => {
                        GetAuthorizerError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetAuthorizerError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetAuthorizerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetAuthorizerError::Validation(error_message.to_string())
                    }
                    _ => GetAuthorizerError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAuthorizerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAuthorizerError {
    fn from(err: serde_json::error::Error) -> GetAuthorizerError {
        GetAuthorizerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAuthorizerError {
    fn from(err: CredentialsError) -> GetAuthorizerError {
        GetAuthorizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAuthorizerError {
    fn from(err: HttpDispatchError) -> GetAuthorizerError {
        GetAuthorizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAuthorizerError {
    fn from(err: io::Error) -> GetAuthorizerError {
        GetAuthorizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            GetAuthorizerError::NotFound(ref cause) => cause,
            GetAuthorizerError::TooManyRequests(ref cause) => cause,
            GetAuthorizerError::Unauthorized(ref cause) => cause,
            GetAuthorizerError::Validation(ref cause) => cause,
            GetAuthorizerError::Credentials(ref err) => err.description(),
            GetAuthorizerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAuthorizerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAuthorizers
#[derive(Debug, PartialEq)]
pub enum GetAuthorizersError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetAuthorizersError {
    pub fn from_body(body: &str) -> GetAuthorizersError {
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
                    "BadRequestException" => {
                        GetAuthorizersError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetAuthorizersError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetAuthorizersError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetAuthorizersError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetAuthorizersError::Validation(error_message.to_string())
                    }
                    _ => GetAuthorizersError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAuthorizersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAuthorizersError {
    fn from(err: serde_json::error::Error) -> GetAuthorizersError {
        GetAuthorizersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAuthorizersError {
    fn from(err: CredentialsError) -> GetAuthorizersError {
        GetAuthorizersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAuthorizersError {
    fn from(err: HttpDispatchError) -> GetAuthorizersError {
        GetAuthorizersError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAuthorizersError {
    fn from(err: io::Error) -> GetAuthorizersError {
        GetAuthorizersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAuthorizersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAuthorizersError {
    fn description(&self) -> &str {
        match *self {
            GetAuthorizersError::BadRequest(ref cause) => cause,
            GetAuthorizersError::NotFound(ref cause) => cause,
            GetAuthorizersError::TooManyRequests(ref cause) => cause,
            GetAuthorizersError::Unauthorized(ref cause) => cause,
            GetAuthorizersError::Validation(ref cause) => cause,
            GetAuthorizersError::Credentials(ref err) => err.description(),
            GetAuthorizersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAuthorizersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBasePathMapping
#[derive(Debug, PartialEq)]
pub enum GetBasePathMappingError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetBasePathMappingError {
    pub fn from_body(body: &str) -> GetBasePathMappingError {
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
                    "NotFoundException" => {
                        GetBasePathMappingError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetBasePathMappingError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetBasePathMappingError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetBasePathMappingError::Validation(error_message.to_string())
                    }
                    _ => GetBasePathMappingError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBasePathMappingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetBasePathMappingError {
    fn from(err: serde_json::error::Error) -> GetBasePathMappingError {
        GetBasePathMappingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBasePathMappingError {
    fn from(err: CredentialsError) -> GetBasePathMappingError {
        GetBasePathMappingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBasePathMappingError {
    fn from(err: HttpDispatchError) -> GetBasePathMappingError {
        GetBasePathMappingError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBasePathMappingError {
    fn from(err: io::Error) -> GetBasePathMappingError {
        GetBasePathMappingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBasePathMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBasePathMappingError {
    fn description(&self) -> &str {
        match *self {
            GetBasePathMappingError::NotFound(ref cause) => cause,
            GetBasePathMappingError::TooManyRequests(ref cause) => cause,
            GetBasePathMappingError::Unauthorized(ref cause) => cause,
            GetBasePathMappingError::Validation(ref cause) => cause,
            GetBasePathMappingError::Credentials(ref err) => err.description(),
            GetBasePathMappingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBasePathMappingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBasePathMappings
#[derive(Debug, PartialEq)]
pub enum GetBasePathMappingsError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetBasePathMappingsError {
    pub fn from_body(body: &str) -> GetBasePathMappingsError {
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
                    "NotFoundException" => {
                        GetBasePathMappingsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetBasePathMappingsError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetBasePathMappingsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetBasePathMappingsError::Validation(error_message.to_string())
                    }
                    _ => GetBasePathMappingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBasePathMappingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetBasePathMappingsError {
    fn from(err: serde_json::error::Error) -> GetBasePathMappingsError {
        GetBasePathMappingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBasePathMappingsError {
    fn from(err: CredentialsError) -> GetBasePathMappingsError {
        GetBasePathMappingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBasePathMappingsError {
    fn from(err: HttpDispatchError) -> GetBasePathMappingsError {
        GetBasePathMappingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBasePathMappingsError {
    fn from(err: io::Error) -> GetBasePathMappingsError {
        GetBasePathMappingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBasePathMappingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBasePathMappingsError {
    fn description(&self) -> &str {
        match *self {
            GetBasePathMappingsError::NotFound(ref cause) => cause,
            GetBasePathMappingsError::TooManyRequests(ref cause) => cause,
            GetBasePathMappingsError::Unauthorized(ref cause) => cause,
            GetBasePathMappingsError::Validation(ref cause) => cause,
            GetBasePathMappingsError::Credentials(ref err) => err.description(),
            GetBasePathMappingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBasePathMappingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetClientCertificate
#[derive(Debug, PartialEq)]
pub enum GetClientCertificateError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetClientCertificateError {
    pub fn from_body(body: &str) -> GetClientCertificateError {
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
                    "NotFoundException" => {
                        GetClientCertificateError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetClientCertificateError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetClientCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetClientCertificateError::Validation(error_message.to_string())
                    }
                    _ => GetClientCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetClientCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetClientCertificateError {
    fn from(err: serde_json::error::Error) -> GetClientCertificateError {
        GetClientCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetClientCertificateError {
    fn from(err: CredentialsError) -> GetClientCertificateError {
        GetClientCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetClientCertificateError {
    fn from(err: HttpDispatchError) -> GetClientCertificateError {
        GetClientCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetClientCertificateError {
    fn from(err: io::Error) -> GetClientCertificateError {
        GetClientCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetClientCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetClientCertificateError {
    fn description(&self) -> &str {
        match *self {
            GetClientCertificateError::NotFound(ref cause) => cause,
            GetClientCertificateError::TooManyRequests(ref cause) => cause,
            GetClientCertificateError::Unauthorized(ref cause) => cause,
            GetClientCertificateError::Validation(ref cause) => cause,
            GetClientCertificateError::Credentials(ref err) => err.description(),
            GetClientCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetClientCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetClientCertificates
#[derive(Debug, PartialEq)]
pub enum GetClientCertificatesError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetClientCertificatesError {
    pub fn from_body(body: &str) -> GetClientCertificatesError {
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
                    "BadRequestException" => {
                        GetClientCertificatesError::BadRequest(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetClientCertificatesError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetClientCertificatesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetClientCertificatesError::Validation(error_message.to_string())
                    }
                    _ => GetClientCertificatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetClientCertificatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetClientCertificatesError {
    fn from(err: serde_json::error::Error) -> GetClientCertificatesError {
        GetClientCertificatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetClientCertificatesError {
    fn from(err: CredentialsError) -> GetClientCertificatesError {
        GetClientCertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetClientCertificatesError {
    fn from(err: HttpDispatchError) -> GetClientCertificatesError {
        GetClientCertificatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetClientCertificatesError {
    fn from(err: io::Error) -> GetClientCertificatesError {
        GetClientCertificatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetClientCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetClientCertificatesError {
    fn description(&self) -> &str {
        match *self {
            GetClientCertificatesError::BadRequest(ref cause) => cause,
            GetClientCertificatesError::TooManyRequests(ref cause) => cause,
            GetClientCertificatesError::Unauthorized(ref cause) => cause,
            GetClientCertificatesError::Validation(ref cause) => cause,
            GetClientCertificatesError::Credentials(ref err) => err.description(),
            GetClientCertificatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetClientCertificatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeployment
#[derive(Debug, PartialEq)]
pub enum GetDeploymentError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The requested service is not available. For details see the accompanying error message. Retry after the specified time period.</p>
    ServiceUnavailable(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetDeploymentError {
    pub fn from_body(body: &str) -> GetDeploymentError {
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
                    "NotFoundException" => {
                        GetDeploymentError::NotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetDeploymentError::ServiceUnavailable(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetDeploymentError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetDeploymentError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDeploymentError::Validation(error_message.to_string())
                    }
                    _ => GetDeploymentError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDeploymentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDeploymentError {
    fn from(err: serde_json::error::Error) -> GetDeploymentError {
        GetDeploymentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeploymentError {
    fn from(err: CredentialsError) -> GetDeploymentError {
        GetDeploymentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeploymentError {
    fn from(err: HttpDispatchError) -> GetDeploymentError {
        GetDeploymentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeploymentError {
    fn from(err: io::Error) -> GetDeploymentError {
        GetDeploymentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeploymentError {
    fn description(&self) -> &str {
        match *self {
            GetDeploymentError::NotFound(ref cause) => cause,
            GetDeploymentError::ServiceUnavailable(ref cause) => cause,
            GetDeploymentError::TooManyRequests(ref cause) => cause,
            GetDeploymentError::Unauthorized(ref cause) => cause,
            GetDeploymentError::Validation(ref cause) => cause,
            GetDeploymentError::Credentials(ref err) => err.description(),
            GetDeploymentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDeploymentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeployments
#[derive(Debug, PartialEq)]
pub enum GetDeploymentsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested service is not available. For details see the accompanying error message. Retry after the specified time period.</p>
    ServiceUnavailable(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetDeploymentsError {
    pub fn from_body(body: &str) -> GetDeploymentsError {
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
                    "BadRequestException" => {
                        GetDeploymentsError::BadRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetDeploymentsError::ServiceUnavailable(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetDeploymentsError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetDeploymentsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDeploymentsError::Validation(error_message.to_string())
                    }
                    _ => GetDeploymentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDeploymentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDeploymentsError {
    fn from(err: serde_json::error::Error) -> GetDeploymentsError {
        GetDeploymentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeploymentsError {
    fn from(err: CredentialsError) -> GetDeploymentsError {
        GetDeploymentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeploymentsError {
    fn from(err: HttpDispatchError) -> GetDeploymentsError {
        GetDeploymentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeploymentsError {
    fn from(err: io::Error) -> GetDeploymentsError {
        GetDeploymentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeploymentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeploymentsError {
    fn description(&self) -> &str {
        match *self {
            GetDeploymentsError::BadRequest(ref cause) => cause,
            GetDeploymentsError::ServiceUnavailable(ref cause) => cause,
            GetDeploymentsError::TooManyRequests(ref cause) => cause,
            GetDeploymentsError::Unauthorized(ref cause) => cause,
            GetDeploymentsError::Validation(ref cause) => cause,
            GetDeploymentsError::Credentials(ref err) => err.description(),
            GetDeploymentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDeploymentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDocumentationPart
#[derive(Debug, PartialEq)]
pub enum GetDocumentationPartError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetDocumentationPartError {
    pub fn from_body(body: &str) -> GetDocumentationPartError {
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
                    "NotFoundException" => {
                        GetDocumentationPartError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetDocumentationPartError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetDocumentationPartError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDocumentationPartError::Validation(error_message.to_string())
                    }
                    _ => GetDocumentationPartError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDocumentationPartError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDocumentationPartError {
    fn from(err: serde_json::error::Error) -> GetDocumentationPartError {
        GetDocumentationPartError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDocumentationPartError {
    fn from(err: CredentialsError) -> GetDocumentationPartError {
        GetDocumentationPartError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDocumentationPartError {
    fn from(err: HttpDispatchError) -> GetDocumentationPartError {
        GetDocumentationPartError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDocumentationPartError {
    fn from(err: io::Error) -> GetDocumentationPartError {
        GetDocumentationPartError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDocumentationPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDocumentationPartError {
    fn description(&self) -> &str {
        match *self {
            GetDocumentationPartError::NotFound(ref cause) => cause,
            GetDocumentationPartError::TooManyRequests(ref cause) => cause,
            GetDocumentationPartError::Unauthorized(ref cause) => cause,
            GetDocumentationPartError::Validation(ref cause) => cause,
            GetDocumentationPartError::Credentials(ref err) => err.description(),
            GetDocumentationPartError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDocumentationPartError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDocumentationParts
#[derive(Debug, PartialEq)]
pub enum GetDocumentationPartsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetDocumentationPartsError {
    pub fn from_body(body: &str) -> GetDocumentationPartsError {
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
                    "BadRequestException" => {
                        GetDocumentationPartsError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetDocumentationPartsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetDocumentationPartsError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetDocumentationPartsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDocumentationPartsError::Validation(error_message.to_string())
                    }
                    _ => GetDocumentationPartsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDocumentationPartsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDocumentationPartsError {
    fn from(err: serde_json::error::Error) -> GetDocumentationPartsError {
        GetDocumentationPartsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDocumentationPartsError {
    fn from(err: CredentialsError) -> GetDocumentationPartsError {
        GetDocumentationPartsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDocumentationPartsError {
    fn from(err: HttpDispatchError) -> GetDocumentationPartsError {
        GetDocumentationPartsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDocumentationPartsError {
    fn from(err: io::Error) -> GetDocumentationPartsError {
        GetDocumentationPartsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDocumentationPartsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDocumentationPartsError {
    fn description(&self) -> &str {
        match *self {
            GetDocumentationPartsError::BadRequest(ref cause) => cause,
            GetDocumentationPartsError::NotFound(ref cause) => cause,
            GetDocumentationPartsError::TooManyRequests(ref cause) => cause,
            GetDocumentationPartsError::Unauthorized(ref cause) => cause,
            GetDocumentationPartsError::Validation(ref cause) => cause,
            GetDocumentationPartsError::Credentials(ref err) => err.description(),
            GetDocumentationPartsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDocumentationPartsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDocumentationVersion
#[derive(Debug, PartialEq)]
pub enum GetDocumentationVersionError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetDocumentationVersionError {
    pub fn from_body(body: &str) -> GetDocumentationVersionError {
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
                    "NotFoundException" => {
                        GetDocumentationVersionError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetDocumentationVersionError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetDocumentationVersionError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDocumentationVersionError::Validation(error_message.to_string())
                    }
                    _ => GetDocumentationVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDocumentationVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDocumentationVersionError {
    fn from(err: serde_json::error::Error) -> GetDocumentationVersionError {
        GetDocumentationVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDocumentationVersionError {
    fn from(err: CredentialsError) -> GetDocumentationVersionError {
        GetDocumentationVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDocumentationVersionError {
    fn from(err: HttpDispatchError) -> GetDocumentationVersionError {
        GetDocumentationVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDocumentationVersionError {
    fn from(err: io::Error) -> GetDocumentationVersionError {
        GetDocumentationVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDocumentationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDocumentationVersionError {
    fn description(&self) -> &str {
        match *self {
            GetDocumentationVersionError::NotFound(ref cause) => cause,
            GetDocumentationVersionError::TooManyRequests(ref cause) => cause,
            GetDocumentationVersionError::Unauthorized(ref cause) => cause,
            GetDocumentationVersionError::Validation(ref cause) => cause,
            GetDocumentationVersionError::Credentials(ref err) => err.description(),
            GetDocumentationVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDocumentationVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDocumentationVersions
#[derive(Debug, PartialEq)]
pub enum GetDocumentationVersionsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetDocumentationVersionsError {
    pub fn from_body(body: &str) -> GetDocumentationVersionsError {
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
                    "BadRequestException" => {
                        GetDocumentationVersionsError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetDocumentationVersionsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetDocumentationVersionsError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetDocumentationVersionsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDocumentationVersionsError::Validation(error_message.to_string())
                    }
                    _ => GetDocumentationVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDocumentationVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDocumentationVersionsError {
    fn from(err: serde_json::error::Error) -> GetDocumentationVersionsError {
        GetDocumentationVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDocumentationVersionsError {
    fn from(err: CredentialsError) -> GetDocumentationVersionsError {
        GetDocumentationVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDocumentationVersionsError {
    fn from(err: HttpDispatchError) -> GetDocumentationVersionsError {
        GetDocumentationVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDocumentationVersionsError {
    fn from(err: io::Error) -> GetDocumentationVersionsError {
        GetDocumentationVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDocumentationVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDocumentationVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetDocumentationVersionsError::BadRequest(ref cause) => cause,
            GetDocumentationVersionsError::NotFound(ref cause) => cause,
            GetDocumentationVersionsError::TooManyRequests(ref cause) => cause,
            GetDocumentationVersionsError::Unauthorized(ref cause) => cause,
            GetDocumentationVersionsError::Validation(ref cause) => cause,
            GetDocumentationVersionsError::Credentials(ref err) => err.description(),
            GetDocumentationVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDocumentationVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDomainName
#[derive(Debug, PartialEq)]
pub enum GetDomainNameError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The requested service is not available. For details see the accompanying error message. Retry after the specified time period.</p>
    ServiceUnavailable(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetDomainNameError {
    pub fn from_body(body: &str) -> GetDomainNameError {
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
                    "NotFoundException" => {
                        GetDomainNameError::NotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetDomainNameError::ServiceUnavailable(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetDomainNameError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetDomainNameError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDomainNameError::Validation(error_message.to_string())
                    }
                    _ => GetDomainNameError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDomainNameError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDomainNameError {
    fn from(err: serde_json::error::Error) -> GetDomainNameError {
        GetDomainNameError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDomainNameError {
    fn from(err: CredentialsError) -> GetDomainNameError {
        GetDomainNameError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDomainNameError {
    fn from(err: HttpDispatchError) -> GetDomainNameError {
        GetDomainNameError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDomainNameError {
    fn from(err: io::Error) -> GetDomainNameError {
        GetDomainNameError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDomainNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDomainNameError {
    fn description(&self) -> &str {
        match *self {
            GetDomainNameError::NotFound(ref cause) => cause,
            GetDomainNameError::ServiceUnavailable(ref cause) => cause,
            GetDomainNameError::TooManyRequests(ref cause) => cause,
            GetDomainNameError::Unauthorized(ref cause) => cause,
            GetDomainNameError::Validation(ref cause) => cause,
            GetDomainNameError::Credentials(ref err) => err.description(),
            GetDomainNameError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDomainNameError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDomainNames
#[derive(Debug, PartialEq)]
pub enum GetDomainNamesError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetDomainNamesError {
    pub fn from_body(body: &str) -> GetDomainNamesError {
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
                    "BadRequestException" => {
                        GetDomainNamesError::BadRequest(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetDomainNamesError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetDomainNamesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDomainNamesError::Validation(error_message.to_string())
                    }
                    _ => GetDomainNamesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDomainNamesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDomainNamesError {
    fn from(err: serde_json::error::Error) -> GetDomainNamesError {
        GetDomainNamesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDomainNamesError {
    fn from(err: CredentialsError) -> GetDomainNamesError {
        GetDomainNamesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDomainNamesError {
    fn from(err: HttpDispatchError) -> GetDomainNamesError {
        GetDomainNamesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDomainNamesError {
    fn from(err: io::Error) -> GetDomainNamesError {
        GetDomainNamesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDomainNamesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDomainNamesError {
    fn description(&self) -> &str {
        match *self {
            GetDomainNamesError::BadRequest(ref cause) => cause,
            GetDomainNamesError::TooManyRequests(ref cause) => cause,
            GetDomainNamesError::Unauthorized(ref cause) => cause,
            GetDomainNamesError::Validation(ref cause) => cause,
            GetDomainNamesError::Credentials(ref err) => err.description(),
            GetDomainNamesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDomainNamesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetExport
#[derive(Debug, PartialEq)]
pub enum GetExportError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetExportError {
    pub fn from_body(body: &str) -> GetExportError {
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
                    "BadRequestException" => {
                        GetExportError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => GetExportError::Conflict(String::from(error_message)),
                    "NotFoundException" => GetExportError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetExportError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetExportError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetExportError::Validation(error_message.to_string()),
                    _ => GetExportError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetExportError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetExportError {
    fn from(err: serde_json::error::Error) -> GetExportError {
        GetExportError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetExportError {
    fn from(err: CredentialsError) -> GetExportError {
        GetExportError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetExportError {
    fn from(err: HttpDispatchError) -> GetExportError {
        GetExportError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetExportError {
    fn from(err: io::Error) -> GetExportError {
        GetExportError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetExportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetExportError {
    fn description(&self) -> &str {
        match *self {
            GetExportError::BadRequest(ref cause) => cause,
            GetExportError::Conflict(ref cause) => cause,
            GetExportError::NotFound(ref cause) => cause,
            GetExportError::TooManyRequests(ref cause) => cause,
            GetExportError::Unauthorized(ref cause) => cause,
            GetExportError::Validation(ref cause) => cause,
            GetExportError::Credentials(ref err) => err.description(),
            GetExportError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetExportError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGatewayResponse
#[derive(Debug, PartialEq)]
pub enum GetGatewayResponseError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetGatewayResponseError {
    pub fn from_body(body: &str) -> GetGatewayResponseError {
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
                    "NotFoundException" => {
                        GetGatewayResponseError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetGatewayResponseError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetGatewayResponseError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetGatewayResponseError::Validation(error_message.to_string())
                    }
                    _ => GetGatewayResponseError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetGatewayResponseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetGatewayResponseError {
    fn from(err: serde_json::error::Error) -> GetGatewayResponseError {
        GetGatewayResponseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGatewayResponseError {
    fn from(err: CredentialsError) -> GetGatewayResponseError {
        GetGatewayResponseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGatewayResponseError {
    fn from(err: HttpDispatchError) -> GetGatewayResponseError {
        GetGatewayResponseError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGatewayResponseError {
    fn from(err: io::Error) -> GetGatewayResponseError {
        GetGatewayResponseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGatewayResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGatewayResponseError {
    fn description(&self) -> &str {
        match *self {
            GetGatewayResponseError::NotFound(ref cause) => cause,
            GetGatewayResponseError::TooManyRequests(ref cause) => cause,
            GetGatewayResponseError::Unauthorized(ref cause) => cause,
            GetGatewayResponseError::Validation(ref cause) => cause,
            GetGatewayResponseError::Credentials(ref err) => err.description(),
            GetGatewayResponseError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetGatewayResponseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGatewayResponses
#[derive(Debug, PartialEq)]
pub enum GetGatewayResponsesError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetGatewayResponsesError {
    pub fn from_body(body: &str) -> GetGatewayResponsesError {
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
                    "BadRequestException" => {
                        GetGatewayResponsesError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetGatewayResponsesError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetGatewayResponsesError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetGatewayResponsesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetGatewayResponsesError::Validation(error_message.to_string())
                    }
                    _ => GetGatewayResponsesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetGatewayResponsesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetGatewayResponsesError {
    fn from(err: serde_json::error::Error) -> GetGatewayResponsesError {
        GetGatewayResponsesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGatewayResponsesError {
    fn from(err: CredentialsError) -> GetGatewayResponsesError {
        GetGatewayResponsesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGatewayResponsesError {
    fn from(err: HttpDispatchError) -> GetGatewayResponsesError {
        GetGatewayResponsesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGatewayResponsesError {
    fn from(err: io::Error) -> GetGatewayResponsesError {
        GetGatewayResponsesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGatewayResponsesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGatewayResponsesError {
    fn description(&self) -> &str {
        match *self {
            GetGatewayResponsesError::BadRequest(ref cause) => cause,
            GetGatewayResponsesError::NotFound(ref cause) => cause,
            GetGatewayResponsesError::TooManyRequests(ref cause) => cause,
            GetGatewayResponsesError::Unauthorized(ref cause) => cause,
            GetGatewayResponsesError::Validation(ref cause) => cause,
            GetGatewayResponsesError::Credentials(ref err) => err.description(),
            GetGatewayResponsesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetGatewayResponsesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIntegration
#[derive(Debug, PartialEq)]
pub enum GetIntegrationError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetIntegrationError {
    pub fn from_body(body: &str) -> GetIntegrationError {
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
                    "NotFoundException" => {
                        GetIntegrationError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetIntegrationError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetIntegrationError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetIntegrationError::Validation(error_message.to_string())
                    }
                    _ => GetIntegrationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetIntegrationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetIntegrationError {
    fn from(err: serde_json::error::Error) -> GetIntegrationError {
        GetIntegrationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetIntegrationError {
    fn from(err: CredentialsError) -> GetIntegrationError {
        GetIntegrationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIntegrationError {
    fn from(err: HttpDispatchError) -> GetIntegrationError {
        GetIntegrationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetIntegrationError {
    fn from(err: io::Error) -> GetIntegrationError {
        GetIntegrationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetIntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIntegrationError {
    fn description(&self) -> &str {
        match *self {
            GetIntegrationError::NotFound(ref cause) => cause,
            GetIntegrationError::TooManyRequests(ref cause) => cause,
            GetIntegrationError::Unauthorized(ref cause) => cause,
            GetIntegrationError::Validation(ref cause) => cause,
            GetIntegrationError::Credentials(ref err) => err.description(),
            GetIntegrationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetIntegrationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum GetIntegrationResponseError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetIntegrationResponseError {
    pub fn from_body(body: &str) -> GetIntegrationResponseError {
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
                    "NotFoundException" => {
                        GetIntegrationResponseError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetIntegrationResponseError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetIntegrationResponseError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetIntegrationResponseError::Validation(error_message.to_string())
                    }
                    _ => GetIntegrationResponseError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetIntegrationResponseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetIntegrationResponseError {
    fn from(err: serde_json::error::Error) -> GetIntegrationResponseError {
        GetIntegrationResponseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetIntegrationResponseError {
    fn from(err: CredentialsError) -> GetIntegrationResponseError {
        GetIntegrationResponseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIntegrationResponseError {
    fn from(err: HttpDispatchError) -> GetIntegrationResponseError {
        GetIntegrationResponseError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetIntegrationResponseError {
    fn from(err: io::Error) -> GetIntegrationResponseError {
        GetIntegrationResponseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetIntegrationResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIntegrationResponseError {
    fn description(&self) -> &str {
        match *self {
            GetIntegrationResponseError::NotFound(ref cause) => cause,
            GetIntegrationResponseError::TooManyRequests(ref cause) => cause,
            GetIntegrationResponseError::Unauthorized(ref cause) => cause,
            GetIntegrationResponseError::Validation(ref cause) => cause,
            GetIntegrationResponseError::Credentials(ref err) => err.description(),
            GetIntegrationResponseError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetIntegrationResponseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMethod
#[derive(Debug, PartialEq)]
pub enum GetMethodError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetMethodError {
    pub fn from_body(body: &str) -> GetMethodError {
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
                    "NotFoundException" => GetMethodError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetMethodError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetMethodError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetMethodError::Validation(error_message.to_string()),
                    _ => GetMethodError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetMethodError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetMethodError {
    fn from(err: serde_json::error::Error) -> GetMethodError {
        GetMethodError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMethodError {
    fn from(err: CredentialsError) -> GetMethodError {
        GetMethodError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMethodError {
    fn from(err: HttpDispatchError) -> GetMethodError {
        GetMethodError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMethodError {
    fn from(err: io::Error) -> GetMethodError {
        GetMethodError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMethodError {
    fn description(&self) -> &str {
        match *self {
            GetMethodError::NotFound(ref cause) => cause,
            GetMethodError::TooManyRequests(ref cause) => cause,
            GetMethodError::Unauthorized(ref cause) => cause,
            GetMethodError::Validation(ref cause) => cause,
            GetMethodError::Credentials(ref err) => err.description(),
            GetMethodError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetMethodError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMethodResponse
#[derive(Debug, PartialEq)]
pub enum GetMethodResponseError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetMethodResponseError {
    pub fn from_body(body: &str) -> GetMethodResponseError {
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
                    "NotFoundException" => {
                        GetMethodResponseError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetMethodResponseError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetMethodResponseError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetMethodResponseError::Validation(error_message.to_string())
                    }
                    _ => GetMethodResponseError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetMethodResponseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetMethodResponseError {
    fn from(err: serde_json::error::Error) -> GetMethodResponseError {
        GetMethodResponseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMethodResponseError {
    fn from(err: CredentialsError) -> GetMethodResponseError {
        GetMethodResponseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMethodResponseError {
    fn from(err: HttpDispatchError) -> GetMethodResponseError {
        GetMethodResponseError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMethodResponseError {
    fn from(err: io::Error) -> GetMethodResponseError {
        GetMethodResponseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMethodResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMethodResponseError {
    fn description(&self) -> &str {
        match *self {
            GetMethodResponseError::NotFound(ref cause) => cause,
            GetMethodResponseError::TooManyRequests(ref cause) => cause,
            GetMethodResponseError::Unauthorized(ref cause) => cause,
            GetMethodResponseError::Validation(ref cause) => cause,
            GetMethodResponseError::Credentials(ref err) => err.description(),
            GetMethodResponseError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetMethodResponseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetModel
#[derive(Debug, PartialEq)]
pub enum GetModelError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetModelError {
    pub fn from_body(body: &str) -> GetModelError {
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
                    "NotFoundException" => GetModelError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetModelError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetModelError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetModelError::Validation(error_message.to_string()),
                    _ => GetModelError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetModelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetModelError {
    fn from(err: serde_json::error::Error) -> GetModelError {
        GetModelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetModelError {
    fn from(err: CredentialsError) -> GetModelError {
        GetModelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetModelError {
    fn from(err: HttpDispatchError) -> GetModelError {
        GetModelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetModelError {
    fn from(err: io::Error) -> GetModelError {
        GetModelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetModelError {
    fn description(&self) -> &str {
        match *self {
            GetModelError::NotFound(ref cause) => cause,
            GetModelError::TooManyRequests(ref cause) => cause,
            GetModelError::Unauthorized(ref cause) => cause,
            GetModelError::Validation(ref cause) => cause,
            GetModelError::Credentials(ref err) => err.description(),
            GetModelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetModelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetModelTemplate
#[derive(Debug, PartialEq)]
pub enum GetModelTemplateError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetModelTemplateError {
    pub fn from_body(body: &str) -> GetModelTemplateError {
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
                    "BadRequestException" => {
                        GetModelTemplateError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetModelTemplateError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetModelTemplateError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetModelTemplateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetModelTemplateError::Validation(error_message.to_string())
                    }
                    _ => GetModelTemplateError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetModelTemplateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetModelTemplateError {
    fn from(err: serde_json::error::Error) -> GetModelTemplateError {
        GetModelTemplateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetModelTemplateError {
    fn from(err: CredentialsError) -> GetModelTemplateError {
        GetModelTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetModelTemplateError {
    fn from(err: HttpDispatchError) -> GetModelTemplateError {
        GetModelTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetModelTemplateError {
    fn from(err: io::Error) -> GetModelTemplateError {
        GetModelTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetModelTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetModelTemplateError {
    fn description(&self) -> &str {
        match *self {
            GetModelTemplateError::BadRequest(ref cause) => cause,
            GetModelTemplateError::NotFound(ref cause) => cause,
            GetModelTemplateError::TooManyRequests(ref cause) => cause,
            GetModelTemplateError::Unauthorized(ref cause) => cause,
            GetModelTemplateError::Validation(ref cause) => cause,
            GetModelTemplateError::Credentials(ref err) => err.description(),
            GetModelTemplateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetModelTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetModels
#[derive(Debug, PartialEq)]
pub enum GetModelsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetModelsError {
    pub fn from_body(body: &str) -> GetModelsError {
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
                    "BadRequestException" => {
                        GetModelsError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => GetModelsError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetModelsError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetModelsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetModelsError::Validation(error_message.to_string()),
                    _ => GetModelsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetModelsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetModelsError {
    fn from(err: serde_json::error::Error) -> GetModelsError {
        GetModelsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetModelsError {
    fn from(err: CredentialsError) -> GetModelsError {
        GetModelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetModelsError {
    fn from(err: HttpDispatchError) -> GetModelsError {
        GetModelsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetModelsError {
    fn from(err: io::Error) -> GetModelsError {
        GetModelsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetModelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetModelsError {
    fn description(&self) -> &str {
        match *self {
            GetModelsError::BadRequest(ref cause) => cause,
            GetModelsError::NotFound(ref cause) => cause,
            GetModelsError::TooManyRequests(ref cause) => cause,
            GetModelsError::Unauthorized(ref cause) => cause,
            GetModelsError::Validation(ref cause) => cause,
            GetModelsError::Credentials(ref err) => err.description(),
            GetModelsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetModelsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRequestValidator
#[derive(Debug, PartialEq)]
pub enum GetRequestValidatorError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetRequestValidatorError {
    pub fn from_body(body: &str) -> GetRequestValidatorError {
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
                    "NotFoundException" => {
                        GetRequestValidatorError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetRequestValidatorError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetRequestValidatorError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetRequestValidatorError::Validation(error_message.to_string())
                    }
                    _ => GetRequestValidatorError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRequestValidatorError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRequestValidatorError {
    fn from(err: serde_json::error::Error) -> GetRequestValidatorError {
        GetRequestValidatorError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRequestValidatorError {
    fn from(err: CredentialsError) -> GetRequestValidatorError {
        GetRequestValidatorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRequestValidatorError {
    fn from(err: HttpDispatchError) -> GetRequestValidatorError {
        GetRequestValidatorError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRequestValidatorError {
    fn from(err: io::Error) -> GetRequestValidatorError {
        GetRequestValidatorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRequestValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRequestValidatorError {
    fn description(&self) -> &str {
        match *self {
            GetRequestValidatorError::NotFound(ref cause) => cause,
            GetRequestValidatorError::TooManyRequests(ref cause) => cause,
            GetRequestValidatorError::Unauthorized(ref cause) => cause,
            GetRequestValidatorError::Validation(ref cause) => cause,
            GetRequestValidatorError::Credentials(ref err) => err.description(),
            GetRequestValidatorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetRequestValidatorError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRequestValidators
#[derive(Debug, PartialEq)]
pub enum GetRequestValidatorsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetRequestValidatorsError {
    pub fn from_body(body: &str) -> GetRequestValidatorsError {
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
                    "BadRequestException" => {
                        GetRequestValidatorsError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetRequestValidatorsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetRequestValidatorsError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetRequestValidatorsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetRequestValidatorsError::Validation(error_message.to_string())
                    }
                    _ => GetRequestValidatorsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRequestValidatorsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRequestValidatorsError {
    fn from(err: serde_json::error::Error) -> GetRequestValidatorsError {
        GetRequestValidatorsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRequestValidatorsError {
    fn from(err: CredentialsError) -> GetRequestValidatorsError {
        GetRequestValidatorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRequestValidatorsError {
    fn from(err: HttpDispatchError) -> GetRequestValidatorsError {
        GetRequestValidatorsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRequestValidatorsError {
    fn from(err: io::Error) -> GetRequestValidatorsError {
        GetRequestValidatorsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRequestValidatorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRequestValidatorsError {
    fn description(&self) -> &str {
        match *self {
            GetRequestValidatorsError::BadRequest(ref cause) => cause,
            GetRequestValidatorsError::NotFound(ref cause) => cause,
            GetRequestValidatorsError::TooManyRequests(ref cause) => cause,
            GetRequestValidatorsError::Unauthorized(ref cause) => cause,
            GetRequestValidatorsError::Validation(ref cause) => cause,
            GetRequestValidatorsError::Credentials(ref err) => err.description(),
            GetRequestValidatorsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetRequestValidatorsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetResource
#[derive(Debug, PartialEq)]
pub enum GetResourceError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetResourceError {
    pub fn from_body(body: &str) -> GetResourceError {
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
                    "NotFoundException" => GetResourceError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetResourceError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetResourceError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetResourceError::Validation(error_message.to_string())
                    }
                    _ => GetResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetResourceError {
    fn from(err: serde_json::error::Error) -> GetResourceError {
        GetResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetResourceError {
    fn from(err: CredentialsError) -> GetResourceError {
        GetResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetResourceError {
    fn from(err: HttpDispatchError) -> GetResourceError {
        GetResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetResourceError {
    fn from(err: io::Error) -> GetResourceError {
        GetResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourceError {
    fn description(&self) -> &str {
        match *self {
            GetResourceError::NotFound(ref cause) => cause,
            GetResourceError::TooManyRequests(ref cause) => cause,
            GetResourceError::Unauthorized(ref cause) => cause,
            GetResourceError::Validation(ref cause) => cause,
            GetResourceError::Credentials(ref err) => err.description(),
            GetResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetResources
#[derive(Debug, PartialEq)]
pub enum GetResourcesError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetResourcesError {
    pub fn from_body(body: &str) -> GetResourcesError {
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
                    "BadRequestException" => {
                        GetResourcesError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => GetResourcesError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetResourcesError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetResourcesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetResourcesError::Validation(error_message.to_string())
                    }
                    _ => GetResourcesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetResourcesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetResourcesError {
    fn from(err: serde_json::error::Error) -> GetResourcesError {
        GetResourcesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetResourcesError {
    fn from(err: CredentialsError) -> GetResourcesError {
        GetResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetResourcesError {
    fn from(err: HttpDispatchError) -> GetResourcesError {
        GetResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetResourcesError {
    fn from(err: io::Error) -> GetResourcesError {
        GetResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourcesError {
    fn description(&self) -> &str {
        match *self {
            GetResourcesError::BadRequest(ref cause) => cause,
            GetResourcesError::NotFound(ref cause) => cause,
            GetResourcesError::TooManyRequests(ref cause) => cause,
            GetResourcesError::Unauthorized(ref cause) => cause,
            GetResourcesError::Validation(ref cause) => cause,
            GetResourcesError::Credentials(ref err) => err.description(),
            GetResourcesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetResourcesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRestApi
#[derive(Debug, PartialEq)]
pub enum GetRestApiError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetRestApiError {
    pub fn from_body(body: &str) -> GetRestApiError {
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
                    "NotFoundException" => GetRestApiError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetRestApiError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetRestApiError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetRestApiError::Validation(error_message.to_string()),
                    _ => GetRestApiError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRestApiError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRestApiError {
    fn from(err: serde_json::error::Error) -> GetRestApiError {
        GetRestApiError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRestApiError {
    fn from(err: CredentialsError) -> GetRestApiError {
        GetRestApiError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRestApiError {
    fn from(err: HttpDispatchError) -> GetRestApiError {
        GetRestApiError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRestApiError {
    fn from(err: io::Error) -> GetRestApiError {
        GetRestApiError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRestApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRestApiError {
    fn description(&self) -> &str {
        match *self {
            GetRestApiError::NotFound(ref cause) => cause,
            GetRestApiError::TooManyRequests(ref cause) => cause,
            GetRestApiError::Unauthorized(ref cause) => cause,
            GetRestApiError::Validation(ref cause) => cause,
            GetRestApiError::Credentials(ref err) => err.description(),
            GetRestApiError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetRestApiError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRestApis
#[derive(Debug, PartialEq)]
pub enum GetRestApisError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetRestApisError {
    pub fn from_body(body: &str) -> GetRestApisError {
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
                    "BadRequestException" => {
                        GetRestApisError::BadRequest(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetRestApisError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetRestApisError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetRestApisError::Validation(error_message.to_string())
                    }
                    _ => GetRestApisError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRestApisError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRestApisError {
    fn from(err: serde_json::error::Error) -> GetRestApisError {
        GetRestApisError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRestApisError {
    fn from(err: CredentialsError) -> GetRestApisError {
        GetRestApisError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRestApisError {
    fn from(err: HttpDispatchError) -> GetRestApisError {
        GetRestApisError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRestApisError {
    fn from(err: io::Error) -> GetRestApisError {
        GetRestApisError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRestApisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRestApisError {
    fn description(&self) -> &str {
        match *self {
            GetRestApisError::BadRequest(ref cause) => cause,
            GetRestApisError::TooManyRequests(ref cause) => cause,
            GetRestApisError::Unauthorized(ref cause) => cause,
            GetRestApisError::Validation(ref cause) => cause,
            GetRestApisError::Credentials(ref err) => err.description(),
            GetRestApisError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetRestApisError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSdk
#[derive(Debug, PartialEq)]
pub enum GetSdkError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetSdkError {
    pub fn from_body(body: &str) -> GetSdkError {
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
                    "BadRequestException" => GetSdkError::BadRequest(String::from(error_message)),
                    "ConflictException" => GetSdkError::Conflict(String::from(error_message)),
                    "NotFoundException" => GetSdkError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetSdkError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetSdkError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetSdkError::Validation(error_message.to_string()),
                    _ => GetSdkError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSdkError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSdkError {
    fn from(err: serde_json::error::Error) -> GetSdkError {
        GetSdkError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSdkError {
    fn from(err: CredentialsError) -> GetSdkError {
        GetSdkError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSdkError {
    fn from(err: HttpDispatchError) -> GetSdkError {
        GetSdkError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSdkError {
    fn from(err: io::Error) -> GetSdkError {
        GetSdkError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSdkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSdkError {
    fn description(&self) -> &str {
        match *self {
            GetSdkError::BadRequest(ref cause) => cause,
            GetSdkError::Conflict(ref cause) => cause,
            GetSdkError::NotFound(ref cause) => cause,
            GetSdkError::TooManyRequests(ref cause) => cause,
            GetSdkError::Unauthorized(ref cause) => cause,
            GetSdkError::Validation(ref cause) => cause,
            GetSdkError::Credentials(ref err) => err.description(),
            GetSdkError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSdkError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSdkType
#[derive(Debug, PartialEq)]
pub enum GetSdkTypeError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetSdkTypeError {
    pub fn from_body(body: &str) -> GetSdkTypeError {
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
                    "NotFoundException" => GetSdkTypeError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetSdkTypeError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetSdkTypeError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetSdkTypeError::Validation(error_message.to_string()),
                    _ => GetSdkTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSdkTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSdkTypeError {
    fn from(err: serde_json::error::Error) -> GetSdkTypeError {
        GetSdkTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSdkTypeError {
    fn from(err: CredentialsError) -> GetSdkTypeError {
        GetSdkTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSdkTypeError {
    fn from(err: HttpDispatchError) -> GetSdkTypeError {
        GetSdkTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSdkTypeError {
    fn from(err: io::Error) -> GetSdkTypeError {
        GetSdkTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSdkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSdkTypeError {
    fn description(&self) -> &str {
        match *self {
            GetSdkTypeError::NotFound(ref cause) => cause,
            GetSdkTypeError::TooManyRequests(ref cause) => cause,
            GetSdkTypeError::Unauthorized(ref cause) => cause,
            GetSdkTypeError::Validation(ref cause) => cause,
            GetSdkTypeError::Credentials(ref err) => err.description(),
            GetSdkTypeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSdkTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSdkTypes
#[derive(Debug, PartialEq)]
pub enum GetSdkTypesError {
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetSdkTypesError {
    pub fn from_body(body: &str) -> GetSdkTypesError {
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
                    "TooManyRequestsException" => {
                        GetSdkTypesError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetSdkTypesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSdkTypesError::Validation(error_message.to_string())
                    }
                    _ => GetSdkTypesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSdkTypesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSdkTypesError {
    fn from(err: serde_json::error::Error) -> GetSdkTypesError {
        GetSdkTypesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSdkTypesError {
    fn from(err: CredentialsError) -> GetSdkTypesError {
        GetSdkTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSdkTypesError {
    fn from(err: HttpDispatchError) -> GetSdkTypesError {
        GetSdkTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSdkTypesError {
    fn from(err: io::Error) -> GetSdkTypesError {
        GetSdkTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSdkTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSdkTypesError {
    fn description(&self) -> &str {
        match *self {
            GetSdkTypesError::TooManyRequests(ref cause) => cause,
            GetSdkTypesError::Unauthorized(ref cause) => cause,
            GetSdkTypesError::Validation(ref cause) => cause,
            GetSdkTypesError::Credentials(ref err) => err.description(),
            GetSdkTypesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSdkTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetStage
#[derive(Debug, PartialEq)]
pub enum GetStageError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetStageError {
    pub fn from_body(body: &str) -> GetStageError {
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
                    "NotFoundException" => GetStageError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetStageError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetStageError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetStageError::Validation(error_message.to_string()),
                    _ => GetStageError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetStageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetStageError {
    fn from(err: serde_json::error::Error) -> GetStageError {
        GetStageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetStageError {
    fn from(err: CredentialsError) -> GetStageError {
        GetStageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetStageError {
    fn from(err: HttpDispatchError) -> GetStageError {
        GetStageError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetStageError {
    fn from(err: io::Error) -> GetStageError {
        GetStageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetStageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetStageError {
    fn description(&self) -> &str {
        match *self {
            GetStageError::NotFound(ref cause) => cause,
            GetStageError::TooManyRequests(ref cause) => cause,
            GetStageError::Unauthorized(ref cause) => cause,
            GetStageError::Validation(ref cause) => cause,
            GetStageError::Credentials(ref err) => err.description(),
            GetStageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetStageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetStages
#[derive(Debug, PartialEq)]
pub enum GetStagesError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetStagesError {
    pub fn from_body(body: &str) -> GetStagesError {
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
                    "NotFoundException" => GetStagesError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetStagesError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetStagesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetStagesError::Validation(error_message.to_string()),
                    _ => GetStagesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetStagesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetStagesError {
    fn from(err: serde_json::error::Error) -> GetStagesError {
        GetStagesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetStagesError {
    fn from(err: CredentialsError) -> GetStagesError {
        GetStagesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetStagesError {
    fn from(err: HttpDispatchError) -> GetStagesError {
        GetStagesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetStagesError {
    fn from(err: io::Error) -> GetStagesError {
        GetStagesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetStagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetStagesError {
    fn description(&self) -> &str {
        match *self {
            GetStagesError::NotFound(ref cause) => cause,
            GetStagesError::TooManyRequests(ref cause) => cause,
            GetStagesError::Unauthorized(ref cause) => cause,
            GetStagesError::Validation(ref cause) => cause,
            GetStagesError::Credentials(ref err) => err.description(),
            GetStagesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetStagesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTags
#[derive(Debug, PartialEq)]
pub enum GetTagsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetTagsError {
    pub fn from_body(body: &str) -> GetTagsError {
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
                    "BadRequestException" => GetTagsError::BadRequest(String::from(error_message)),
                    "LimitExceededException" => {
                        GetTagsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => GetTagsError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetTagsError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetTagsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetTagsError::Validation(error_message.to_string()),
                    _ => GetTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTagsError {
    fn from(err: serde_json::error::Error) -> GetTagsError {
        GetTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTagsError {
    fn from(err: CredentialsError) -> GetTagsError {
        GetTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTagsError {
    fn from(err: HttpDispatchError) -> GetTagsError {
        GetTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTagsError {
    fn from(err: io::Error) -> GetTagsError {
        GetTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTagsError {
    fn description(&self) -> &str {
        match *self {
            GetTagsError::BadRequest(ref cause) => cause,
            GetTagsError::LimitExceeded(ref cause) => cause,
            GetTagsError::NotFound(ref cause) => cause,
            GetTagsError::TooManyRequests(ref cause) => cause,
            GetTagsError::Unauthorized(ref cause) => cause,
            GetTagsError::Validation(ref cause) => cause,
            GetTagsError::Credentials(ref err) => err.description(),
            GetTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUsage
#[derive(Debug, PartialEq)]
pub enum GetUsageError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetUsageError {
    pub fn from_body(body: &str) -> GetUsageError {
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
                    "BadRequestException" => GetUsageError::BadRequest(String::from(error_message)),
                    "NotFoundException" => GetUsageError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetUsageError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetUsageError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetUsageError::Validation(error_message.to_string()),
                    _ => GetUsageError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetUsageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetUsageError {
    fn from(err: serde_json::error::Error) -> GetUsageError {
        GetUsageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetUsageError {
    fn from(err: CredentialsError) -> GetUsageError {
        GetUsageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetUsageError {
    fn from(err: HttpDispatchError) -> GetUsageError {
        GetUsageError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetUsageError {
    fn from(err: io::Error) -> GetUsageError {
        GetUsageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUsageError {
    fn description(&self) -> &str {
        match *self {
            GetUsageError::BadRequest(ref cause) => cause,
            GetUsageError::NotFound(ref cause) => cause,
            GetUsageError::TooManyRequests(ref cause) => cause,
            GetUsageError::Unauthorized(ref cause) => cause,
            GetUsageError::Validation(ref cause) => cause,
            GetUsageError::Credentials(ref err) => err.description(),
            GetUsageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetUsageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUsagePlan
#[derive(Debug, PartialEq)]
pub enum GetUsagePlanError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetUsagePlanError {
    pub fn from_body(body: &str) -> GetUsagePlanError {
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
                    "BadRequestException" => {
                        GetUsagePlanError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => GetUsagePlanError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetUsagePlanError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetUsagePlanError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetUsagePlanError::Validation(error_message.to_string())
                    }
                    _ => GetUsagePlanError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetUsagePlanError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetUsagePlanError {
    fn from(err: serde_json::error::Error) -> GetUsagePlanError {
        GetUsagePlanError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetUsagePlanError {
    fn from(err: CredentialsError) -> GetUsagePlanError {
        GetUsagePlanError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetUsagePlanError {
    fn from(err: HttpDispatchError) -> GetUsagePlanError {
        GetUsagePlanError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetUsagePlanError {
    fn from(err: io::Error) -> GetUsagePlanError {
        GetUsagePlanError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetUsagePlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUsagePlanError {
    fn description(&self) -> &str {
        match *self {
            GetUsagePlanError::BadRequest(ref cause) => cause,
            GetUsagePlanError::NotFound(ref cause) => cause,
            GetUsagePlanError::TooManyRequests(ref cause) => cause,
            GetUsagePlanError::Unauthorized(ref cause) => cause,
            GetUsagePlanError::Validation(ref cause) => cause,
            GetUsagePlanError::Credentials(ref err) => err.description(),
            GetUsagePlanError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetUsagePlanError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUsagePlanKey
#[derive(Debug, PartialEq)]
pub enum GetUsagePlanKeyError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetUsagePlanKeyError {
    pub fn from_body(body: &str) -> GetUsagePlanKeyError {
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
                    "BadRequestException" => {
                        GetUsagePlanKeyError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetUsagePlanKeyError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetUsagePlanKeyError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetUsagePlanKeyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetUsagePlanKeyError::Validation(error_message.to_string())
                    }
                    _ => GetUsagePlanKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetUsagePlanKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetUsagePlanKeyError {
    fn from(err: serde_json::error::Error) -> GetUsagePlanKeyError {
        GetUsagePlanKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetUsagePlanKeyError {
    fn from(err: CredentialsError) -> GetUsagePlanKeyError {
        GetUsagePlanKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetUsagePlanKeyError {
    fn from(err: HttpDispatchError) -> GetUsagePlanKeyError {
        GetUsagePlanKeyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetUsagePlanKeyError {
    fn from(err: io::Error) -> GetUsagePlanKeyError {
        GetUsagePlanKeyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetUsagePlanKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUsagePlanKeyError {
    fn description(&self) -> &str {
        match *self {
            GetUsagePlanKeyError::BadRequest(ref cause) => cause,
            GetUsagePlanKeyError::NotFound(ref cause) => cause,
            GetUsagePlanKeyError::TooManyRequests(ref cause) => cause,
            GetUsagePlanKeyError::Unauthorized(ref cause) => cause,
            GetUsagePlanKeyError::Validation(ref cause) => cause,
            GetUsagePlanKeyError::Credentials(ref err) => err.description(),
            GetUsagePlanKeyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetUsagePlanKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUsagePlanKeys
#[derive(Debug, PartialEq)]
pub enum GetUsagePlanKeysError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetUsagePlanKeysError {
    pub fn from_body(body: &str) -> GetUsagePlanKeysError {
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
                    "BadRequestException" => {
                        GetUsagePlanKeysError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetUsagePlanKeysError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetUsagePlanKeysError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetUsagePlanKeysError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetUsagePlanKeysError::Validation(error_message.to_string())
                    }
                    _ => GetUsagePlanKeysError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetUsagePlanKeysError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetUsagePlanKeysError {
    fn from(err: serde_json::error::Error) -> GetUsagePlanKeysError {
        GetUsagePlanKeysError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetUsagePlanKeysError {
    fn from(err: CredentialsError) -> GetUsagePlanKeysError {
        GetUsagePlanKeysError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetUsagePlanKeysError {
    fn from(err: HttpDispatchError) -> GetUsagePlanKeysError {
        GetUsagePlanKeysError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetUsagePlanKeysError {
    fn from(err: io::Error) -> GetUsagePlanKeysError {
        GetUsagePlanKeysError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetUsagePlanKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUsagePlanKeysError {
    fn description(&self) -> &str {
        match *self {
            GetUsagePlanKeysError::BadRequest(ref cause) => cause,
            GetUsagePlanKeysError::NotFound(ref cause) => cause,
            GetUsagePlanKeysError::TooManyRequests(ref cause) => cause,
            GetUsagePlanKeysError::Unauthorized(ref cause) => cause,
            GetUsagePlanKeysError::Validation(ref cause) => cause,
            GetUsagePlanKeysError::Credentials(ref err) => err.description(),
            GetUsagePlanKeysError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetUsagePlanKeysError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUsagePlans
#[derive(Debug, PartialEq)]
pub enum GetUsagePlansError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetUsagePlansError {
    pub fn from_body(body: &str) -> GetUsagePlansError {
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
                    "BadRequestException" => {
                        GetUsagePlansError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        GetUsagePlansError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetUsagePlansError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetUsagePlansError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetUsagePlansError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetUsagePlansError::Validation(error_message.to_string())
                    }
                    _ => GetUsagePlansError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetUsagePlansError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetUsagePlansError {
    fn from(err: serde_json::error::Error) -> GetUsagePlansError {
        GetUsagePlansError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetUsagePlansError {
    fn from(err: CredentialsError) -> GetUsagePlansError {
        GetUsagePlansError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetUsagePlansError {
    fn from(err: HttpDispatchError) -> GetUsagePlansError {
        GetUsagePlansError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetUsagePlansError {
    fn from(err: io::Error) -> GetUsagePlansError {
        GetUsagePlansError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetUsagePlansError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUsagePlansError {
    fn description(&self) -> &str {
        match *self {
            GetUsagePlansError::BadRequest(ref cause) => cause,
            GetUsagePlansError::Conflict(ref cause) => cause,
            GetUsagePlansError::NotFound(ref cause) => cause,
            GetUsagePlansError::TooManyRequests(ref cause) => cause,
            GetUsagePlansError::Unauthorized(ref cause) => cause,
            GetUsagePlansError::Validation(ref cause) => cause,
            GetUsagePlansError::Credentials(ref err) => err.description(),
            GetUsagePlansError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetUsagePlansError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetVpcLink
#[derive(Debug, PartialEq)]
pub enum GetVpcLinkError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetVpcLinkError {
    pub fn from_body(body: &str) -> GetVpcLinkError {
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
                    "NotFoundException" => GetVpcLinkError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetVpcLinkError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetVpcLinkError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetVpcLinkError::Validation(error_message.to_string()),
                    _ => GetVpcLinkError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetVpcLinkError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetVpcLinkError {
    fn from(err: serde_json::error::Error) -> GetVpcLinkError {
        GetVpcLinkError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetVpcLinkError {
    fn from(err: CredentialsError) -> GetVpcLinkError {
        GetVpcLinkError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetVpcLinkError {
    fn from(err: HttpDispatchError) -> GetVpcLinkError {
        GetVpcLinkError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetVpcLinkError {
    fn from(err: io::Error) -> GetVpcLinkError {
        GetVpcLinkError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetVpcLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVpcLinkError {
    fn description(&self) -> &str {
        match *self {
            GetVpcLinkError::NotFound(ref cause) => cause,
            GetVpcLinkError::TooManyRequests(ref cause) => cause,
            GetVpcLinkError::Unauthorized(ref cause) => cause,
            GetVpcLinkError::Validation(ref cause) => cause,
            GetVpcLinkError::Credentials(ref err) => err.description(),
            GetVpcLinkError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetVpcLinkError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetVpcLinks
#[derive(Debug, PartialEq)]
pub enum GetVpcLinksError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl GetVpcLinksError {
    pub fn from_body(body: &str) -> GetVpcLinksError {
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
                    "BadRequestException" => {
                        GetVpcLinksError::BadRequest(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetVpcLinksError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetVpcLinksError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetVpcLinksError::Validation(error_message.to_string())
                    }
                    _ => GetVpcLinksError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetVpcLinksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetVpcLinksError {
    fn from(err: serde_json::error::Error) -> GetVpcLinksError {
        GetVpcLinksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetVpcLinksError {
    fn from(err: CredentialsError) -> GetVpcLinksError {
        GetVpcLinksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetVpcLinksError {
    fn from(err: HttpDispatchError) -> GetVpcLinksError {
        GetVpcLinksError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetVpcLinksError {
    fn from(err: io::Error) -> GetVpcLinksError {
        GetVpcLinksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetVpcLinksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVpcLinksError {
    fn description(&self) -> &str {
        match *self {
            GetVpcLinksError::BadRequest(ref cause) => cause,
            GetVpcLinksError::TooManyRequests(ref cause) => cause,
            GetVpcLinksError::Unauthorized(ref cause) => cause,
            GetVpcLinksError::Validation(ref cause) => cause,
            GetVpcLinksError::Credentials(ref err) => err.description(),
            GetVpcLinksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetVpcLinksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportApiKeys
#[derive(Debug, PartialEq)]
pub enum ImportApiKeysError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl ImportApiKeysError {
    pub fn from_body(body: &str) -> ImportApiKeysError {
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
                    "BadRequestException" => {
                        ImportApiKeysError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        ImportApiKeysError::Conflict(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ImportApiKeysError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ImportApiKeysError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ImportApiKeysError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ImportApiKeysError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ImportApiKeysError::Validation(error_message.to_string())
                    }
                    _ => ImportApiKeysError::Unknown(String::from(body)),
                }
            }
            Err(_) => ImportApiKeysError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ImportApiKeysError {
    fn from(err: serde_json::error::Error) -> ImportApiKeysError {
        ImportApiKeysError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ImportApiKeysError {
    fn from(err: CredentialsError) -> ImportApiKeysError {
        ImportApiKeysError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ImportApiKeysError {
    fn from(err: HttpDispatchError) -> ImportApiKeysError {
        ImportApiKeysError::HttpDispatch(err)
    }
}
impl From<io::Error> for ImportApiKeysError {
    fn from(err: io::Error) -> ImportApiKeysError {
        ImportApiKeysError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ImportApiKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportApiKeysError {
    fn description(&self) -> &str {
        match *self {
            ImportApiKeysError::BadRequest(ref cause) => cause,
            ImportApiKeysError::Conflict(ref cause) => cause,
            ImportApiKeysError::LimitExceeded(ref cause) => cause,
            ImportApiKeysError::NotFound(ref cause) => cause,
            ImportApiKeysError::TooManyRequests(ref cause) => cause,
            ImportApiKeysError::Unauthorized(ref cause) => cause,
            ImportApiKeysError::Validation(ref cause) => cause,
            ImportApiKeysError::Credentials(ref err) => err.description(),
            ImportApiKeysError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ImportApiKeysError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportDocumentationParts
#[derive(Debug, PartialEq)]
pub enum ImportDocumentationPartsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl ImportDocumentationPartsError {
    pub fn from_body(body: &str) -> ImportDocumentationPartsError {
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
                    "BadRequestException" => {
                        ImportDocumentationPartsError::BadRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ImportDocumentationPartsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ImportDocumentationPartsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ImportDocumentationPartsError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ImportDocumentationPartsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ImportDocumentationPartsError::Validation(error_message.to_string())
                    }
                    _ => ImportDocumentationPartsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ImportDocumentationPartsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ImportDocumentationPartsError {
    fn from(err: serde_json::error::Error) -> ImportDocumentationPartsError {
        ImportDocumentationPartsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ImportDocumentationPartsError {
    fn from(err: CredentialsError) -> ImportDocumentationPartsError {
        ImportDocumentationPartsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ImportDocumentationPartsError {
    fn from(err: HttpDispatchError) -> ImportDocumentationPartsError {
        ImportDocumentationPartsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ImportDocumentationPartsError {
    fn from(err: io::Error) -> ImportDocumentationPartsError {
        ImportDocumentationPartsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ImportDocumentationPartsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportDocumentationPartsError {
    fn description(&self) -> &str {
        match *self {
            ImportDocumentationPartsError::BadRequest(ref cause) => cause,
            ImportDocumentationPartsError::LimitExceeded(ref cause) => cause,
            ImportDocumentationPartsError::NotFound(ref cause) => cause,
            ImportDocumentationPartsError::TooManyRequests(ref cause) => cause,
            ImportDocumentationPartsError::Unauthorized(ref cause) => cause,
            ImportDocumentationPartsError::Validation(ref cause) => cause,
            ImportDocumentationPartsError::Credentials(ref err) => err.description(),
            ImportDocumentationPartsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ImportDocumentationPartsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportRestApi
#[derive(Debug, PartialEq)]
pub enum ImportRestApiError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl ImportRestApiError {
    pub fn from_body(body: &str) -> ImportRestApiError {
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
                    "BadRequestException" => {
                        ImportRestApiError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        ImportRestApiError::Conflict(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ImportRestApiError::LimitExceeded(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ImportRestApiError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ImportRestApiError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ImportRestApiError::Validation(error_message.to_string())
                    }
                    _ => ImportRestApiError::Unknown(String::from(body)),
                }
            }
            Err(_) => ImportRestApiError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ImportRestApiError {
    fn from(err: serde_json::error::Error) -> ImportRestApiError {
        ImportRestApiError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ImportRestApiError {
    fn from(err: CredentialsError) -> ImportRestApiError {
        ImportRestApiError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ImportRestApiError {
    fn from(err: HttpDispatchError) -> ImportRestApiError {
        ImportRestApiError::HttpDispatch(err)
    }
}
impl From<io::Error> for ImportRestApiError {
    fn from(err: io::Error) -> ImportRestApiError {
        ImportRestApiError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ImportRestApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportRestApiError {
    fn description(&self) -> &str {
        match *self {
            ImportRestApiError::BadRequest(ref cause) => cause,
            ImportRestApiError::Conflict(ref cause) => cause,
            ImportRestApiError::LimitExceeded(ref cause) => cause,
            ImportRestApiError::TooManyRequests(ref cause) => cause,
            ImportRestApiError::Unauthorized(ref cause) => cause,
            ImportRestApiError::Validation(ref cause) => cause,
            ImportRestApiError::Credentials(ref err) => err.description(),
            ImportRestApiError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ImportRestApiError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutGatewayResponse
#[derive(Debug, PartialEq)]
pub enum PutGatewayResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl PutGatewayResponseError {
    pub fn from_body(body: &str) -> PutGatewayResponseError {
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
                    "BadRequestException" => {
                        PutGatewayResponseError::BadRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutGatewayResponseError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        PutGatewayResponseError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        PutGatewayResponseError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        PutGatewayResponseError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutGatewayResponseError::Validation(error_message.to_string())
                    }
                    _ => PutGatewayResponseError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutGatewayResponseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutGatewayResponseError {
    fn from(err: serde_json::error::Error) -> PutGatewayResponseError {
        PutGatewayResponseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutGatewayResponseError {
    fn from(err: CredentialsError) -> PutGatewayResponseError {
        PutGatewayResponseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutGatewayResponseError {
    fn from(err: HttpDispatchError) -> PutGatewayResponseError {
        PutGatewayResponseError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutGatewayResponseError {
    fn from(err: io::Error) -> PutGatewayResponseError {
        PutGatewayResponseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutGatewayResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutGatewayResponseError {
    fn description(&self) -> &str {
        match *self {
            PutGatewayResponseError::BadRequest(ref cause) => cause,
            PutGatewayResponseError::LimitExceeded(ref cause) => cause,
            PutGatewayResponseError::NotFound(ref cause) => cause,
            PutGatewayResponseError::TooManyRequests(ref cause) => cause,
            PutGatewayResponseError::Unauthorized(ref cause) => cause,
            PutGatewayResponseError::Validation(ref cause) => cause,
            PutGatewayResponseError::Credentials(ref err) => err.description(),
            PutGatewayResponseError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutGatewayResponseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutIntegration
#[derive(Debug, PartialEq)]
pub enum PutIntegrationError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl PutIntegrationError {
    pub fn from_body(body: &str) -> PutIntegrationError {
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
                    "BadRequestException" => {
                        PutIntegrationError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        PutIntegrationError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        PutIntegrationError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        PutIntegrationError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        PutIntegrationError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutIntegrationError::Validation(error_message.to_string())
                    }
                    _ => PutIntegrationError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutIntegrationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutIntegrationError {
    fn from(err: serde_json::error::Error) -> PutIntegrationError {
        PutIntegrationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutIntegrationError {
    fn from(err: CredentialsError) -> PutIntegrationError {
        PutIntegrationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutIntegrationError {
    fn from(err: HttpDispatchError) -> PutIntegrationError {
        PutIntegrationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutIntegrationError {
    fn from(err: io::Error) -> PutIntegrationError {
        PutIntegrationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutIntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutIntegrationError {
    fn description(&self) -> &str {
        match *self {
            PutIntegrationError::BadRequest(ref cause) => cause,
            PutIntegrationError::Conflict(ref cause) => cause,
            PutIntegrationError::NotFound(ref cause) => cause,
            PutIntegrationError::TooManyRequests(ref cause) => cause,
            PutIntegrationError::Unauthorized(ref cause) => cause,
            PutIntegrationError::Validation(ref cause) => cause,
            PutIntegrationError::Credentials(ref err) => err.description(),
            PutIntegrationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutIntegrationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum PutIntegrationResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl PutIntegrationResponseError {
    pub fn from_body(body: &str) -> PutIntegrationResponseError {
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
                    "BadRequestException" => {
                        PutIntegrationResponseError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        PutIntegrationResponseError::Conflict(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutIntegrationResponseError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        PutIntegrationResponseError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        PutIntegrationResponseError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        PutIntegrationResponseError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutIntegrationResponseError::Validation(error_message.to_string())
                    }
                    _ => PutIntegrationResponseError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutIntegrationResponseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutIntegrationResponseError {
    fn from(err: serde_json::error::Error) -> PutIntegrationResponseError {
        PutIntegrationResponseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutIntegrationResponseError {
    fn from(err: CredentialsError) -> PutIntegrationResponseError {
        PutIntegrationResponseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutIntegrationResponseError {
    fn from(err: HttpDispatchError) -> PutIntegrationResponseError {
        PutIntegrationResponseError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutIntegrationResponseError {
    fn from(err: io::Error) -> PutIntegrationResponseError {
        PutIntegrationResponseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutIntegrationResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutIntegrationResponseError {
    fn description(&self) -> &str {
        match *self {
            PutIntegrationResponseError::BadRequest(ref cause) => cause,
            PutIntegrationResponseError::Conflict(ref cause) => cause,
            PutIntegrationResponseError::LimitExceeded(ref cause) => cause,
            PutIntegrationResponseError::NotFound(ref cause) => cause,
            PutIntegrationResponseError::TooManyRequests(ref cause) => cause,
            PutIntegrationResponseError::Unauthorized(ref cause) => cause,
            PutIntegrationResponseError::Validation(ref cause) => cause,
            PutIntegrationResponseError::Credentials(ref err) => err.description(),
            PutIntegrationResponseError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutIntegrationResponseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutMethod
#[derive(Debug, PartialEq)]
pub enum PutMethodError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl PutMethodError {
    pub fn from_body(body: &str) -> PutMethodError {
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
                    "BadRequestException" => {
                        PutMethodError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => PutMethodError::Conflict(String::from(error_message)),
                    "LimitExceededException" => {
                        PutMethodError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => PutMethodError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        PutMethodError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        PutMethodError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => PutMethodError::Validation(error_message.to_string()),
                    _ => PutMethodError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutMethodError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutMethodError {
    fn from(err: serde_json::error::Error) -> PutMethodError {
        PutMethodError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutMethodError {
    fn from(err: CredentialsError) -> PutMethodError {
        PutMethodError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutMethodError {
    fn from(err: HttpDispatchError) -> PutMethodError {
        PutMethodError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutMethodError {
    fn from(err: io::Error) -> PutMethodError {
        PutMethodError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutMethodError {
    fn description(&self) -> &str {
        match *self {
            PutMethodError::BadRequest(ref cause) => cause,
            PutMethodError::Conflict(ref cause) => cause,
            PutMethodError::LimitExceeded(ref cause) => cause,
            PutMethodError::NotFound(ref cause) => cause,
            PutMethodError::TooManyRequests(ref cause) => cause,
            PutMethodError::Unauthorized(ref cause) => cause,
            PutMethodError::Validation(ref cause) => cause,
            PutMethodError::Credentials(ref err) => err.description(),
            PutMethodError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutMethodError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutMethodResponse
#[derive(Debug, PartialEq)]
pub enum PutMethodResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl PutMethodResponseError {
    pub fn from_body(body: &str) -> PutMethodResponseError {
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
                    "BadRequestException" => {
                        PutMethodResponseError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        PutMethodResponseError::Conflict(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutMethodResponseError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        PutMethodResponseError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        PutMethodResponseError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        PutMethodResponseError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutMethodResponseError::Validation(error_message.to_string())
                    }
                    _ => PutMethodResponseError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutMethodResponseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutMethodResponseError {
    fn from(err: serde_json::error::Error) -> PutMethodResponseError {
        PutMethodResponseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutMethodResponseError {
    fn from(err: CredentialsError) -> PutMethodResponseError {
        PutMethodResponseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutMethodResponseError {
    fn from(err: HttpDispatchError) -> PutMethodResponseError {
        PutMethodResponseError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutMethodResponseError {
    fn from(err: io::Error) -> PutMethodResponseError {
        PutMethodResponseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutMethodResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutMethodResponseError {
    fn description(&self) -> &str {
        match *self {
            PutMethodResponseError::BadRequest(ref cause) => cause,
            PutMethodResponseError::Conflict(ref cause) => cause,
            PutMethodResponseError::LimitExceeded(ref cause) => cause,
            PutMethodResponseError::NotFound(ref cause) => cause,
            PutMethodResponseError::TooManyRequests(ref cause) => cause,
            PutMethodResponseError::Unauthorized(ref cause) => cause,
            PutMethodResponseError::Validation(ref cause) => cause,
            PutMethodResponseError::Credentials(ref err) => err.description(),
            PutMethodResponseError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutMethodResponseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutRestApi
#[derive(Debug, PartialEq)]
pub enum PutRestApiError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl PutRestApiError {
    pub fn from_body(body: &str) -> PutRestApiError {
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
                    "BadRequestException" => {
                        PutRestApiError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => PutRestApiError::Conflict(String::from(error_message)),
                    "LimitExceededException" => {
                        PutRestApiError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => PutRestApiError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        PutRestApiError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        PutRestApiError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => PutRestApiError::Validation(error_message.to_string()),
                    _ => PutRestApiError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutRestApiError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutRestApiError {
    fn from(err: serde_json::error::Error) -> PutRestApiError {
        PutRestApiError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutRestApiError {
    fn from(err: CredentialsError) -> PutRestApiError {
        PutRestApiError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutRestApiError {
    fn from(err: HttpDispatchError) -> PutRestApiError {
        PutRestApiError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutRestApiError {
    fn from(err: io::Error) -> PutRestApiError {
        PutRestApiError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutRestApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRestApiError {
    fn description(&self) -> &str {
        match *self {
            PutRestApiError::BadRequest(ref cause) => cause,
            PutRestApiError::Conflict(ref cause) => cause,
            PutRestApiError::LimitExceeded(ref cause) => cause,
            PutRestApiError::NotFound(ref cause) => cause,
            PutRestApiError::TooManyRequests(ref cause) => cause,
            PutRestApiError::Unauthorized(ref cause) => cause,
            PutRestApiError::Validation(ref cause) => cause,
            PutRestApiError::Credentials(ref err) => err.description(),
            PutRestApiError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutRestApiError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl TagResourceError {
    pub fn from_body(body: &str) -> TagResourceError {
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
                    "BadRequestException" => {
                        TagResourceError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => TagResourceError::Conflict(String::from(error_message)),
                    "LimitExceededException" => {
                        TagResourceError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => TagResourceError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        TagResourceError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        TagResourceError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        TagResourceError::Validation(error_message.to_string())
                    }
                    _ => TagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => TagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourceError {
    fn from(err: CredentialsError) -> TagResourceError {
        TagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourceError {
    fn from(err: HttpDispatchError) -> TagResourceError {
        TagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagResourceError {
    fn from(err: io::Error) -> TagResourceError {
        TagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::BadRequest(ref cause) => cause,
            TagResourceError::Conflict(ref cause) => cause,
            TagResourceError::LimitExceeded(ref cause) => cause,
            TagResourceError::NotFound(ref cause) => cause,
            TagResourceError::TooManyRequests(ref cause) => cause,
            TagResourceError::Unauthorized(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TestInvokeAuthorizer
#[derive(Debug, PartialEq)]
pub enum TestInvokeAuthorizerError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl TestInvokeAuthorizerError {
    pub fn from_body(body: &str) -> TestInvokeAuthorizerError {
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
                    "BadRequestException" => {
                        TestInvokeAuthorizerError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        TestInvokeAuthorizerError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        TestInvokeAuthorizerError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        TestInvokeAuthorizerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        TestInvokeAuthorizerError::Validation(error_message.to_string())
                    }
                    _ => TestInvokeAuthorizerError::Unknown(String::from(body)),
                }
            }
            Err(_) => TestInvokeAuthorizerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TestInvokeAuthorizerError {
    fn from(err: serde_json::error::Error) -> TestInvokeAuthorizerError {
        TestInvokeAuthorizerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TestInvokeAuthorizerError {
    fn from(err: CredentialsError) -> TestInvokeAuthorizerError {
        TestInvokeAuthorizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TestInvokeAuthorizerError {
    fn from(err: HttpDispatchError) -> TestInvokeAuthorizerError {
        TestInvokeAuthorizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for TestInvokeAuthorizerError {
    fn from(err: io::Error) -> TestInvokeAuthorizerError {
        TestInvokeAuthorizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TestInvokeAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestInvokeAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            TestInvokeAuthorizerError::BadRequest(ref cause) => cause,
            TestInvokeAuthorizerError::NotFound(ref cause) => cause,
            TestInvokeAuthorizerError::TooManyRequests(ref cause) => cause,
            TestInvokeAuthorizerError::Unauthorized(ref cause) => cause,
            TestInvokeAuthorizerError::Validation(ref cause) => cause,
            TestInvokeAuthorizerError::Credentials(ref err) => err.description(),
            TestInvokeAuthorizerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TestInvokeAuthorizerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TestInvokeMethod
#[derive(Debug, PartialEq)]
pub enum TestInvokeMethodError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl TestInvokeMethodError {
    pub fn from_body(body: &str) -> TestInvokeMethodError {
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
                    "BadRequestException" => {
                        TestInvokeMethodError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        TestInvokeMethodError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        TestInvokeMethodError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        TestInvokeMethodError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        TestInvokeMethodError::Validation(error_message.to_string())
                    }
                    _ => TestInvokeMethodError::Unknown(String::from(body)),
                }
            }
            Err(_) => TestInvokeMethodError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TestInvokeMethodError {
    fn from(err: serde_json::error::Error) -> TestInvokeMethodError {
        TestInvokeMethodError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TestInvokeMethodError {
    fn from(err: CredentialsError) -> TestInvokeMethodError {
        TestInvokeMethodError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TestInvokeMethodError {
    fn from(err: HttpDispatchError) -> TestInvokeMethodError {
        TestInvokeMethodError::HttpDispatch(err)
    }
}
impl From<io::Error> for TestInvokeMethodError {
    fn from(err: io::Error) -> TestInvokeMethodError {
        TestInvokeMethodError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TestInvokeMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestInvokeMethodError {
    fn description(&self) -> &str {
        match *self {
            TestInvokeMethodError::BadRequest(ref cause) => cause,
            TestInvokeMethodError::NotFound(ref cause) => cause,
            TestInvokeMethodError::TooManyRequests(ref cause) => cause,
            TestInvokeMethodError::Unauthorized(ref cause) => cause,
            TestInvokeMethodError::Validation(ref cause) => cause,
            TestInvokeMethodError::Credentials(ref err) => err.description(),
            TestInvokeMethodError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TestInvokeMethodError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UntagResourceError {
    pub fn from_body(body: &str) -> UntagResourceError {
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
                    "BadRequestException" => {
                        UntagResourceError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UntagResourceError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UntagResourceError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UntagResourceError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UntagResourceError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UntagResourceError::Validation(error_message.to_string())
                    }
                    _ => UntagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UntagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourceError {
    fn from(err: CredentialsError) -> UntagResourceError {
        UntagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourceError {
    fn from(err: HttpDispatchError) -> UntagResourceError {
        UntagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagResourceError {
    fn from(err: io::Error) -> UntagResourceError {
        UntagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::BadRequest(ref cause) => cause,
            UntagResourceError::Conflict(ref cause) => cause,
            UntagResourceError::NotFound(ref cause) => cause,
            UntagResourceError::TooManyRequests(ref cause) => cause,
            UntagResourceError::Unauthorized(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAccount
#[derive(Debug, PartialEq)]
pub enum UpdateAccountError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateAccountError {
    pub fn from_body(body: &str) -> UpdateAccountError {
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
                    "BadRequestException" => {
                        UpdateAccountError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateAccountError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateAccountError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateAccountError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateAccountError::Validation(error_message.to_string())
                    }
                    _ => UpdateAccountError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateAccountError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateAccountError {
    fn from(err: serde_json::error::Error) -> UpdateAccountError {
        UpdateAccountError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAccountError {
    fn from(err: CredentialsError) -> UpdateAccountError {
        UpdateAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAccountError {
    fn from(err: HttpDispatchError) -> UpdateAccountError {
        UpdateAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAccountError {
    fn from(err: io::Error) -> UpdateAccountError {
        UpdateAccountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAccountError {
    fn description(&self) -> &str {
        match *self {
            UpdateAccountError::BadRequest(ref cause) => cause,
            UpdateAccountError::NotFound(ref cause) => cause,
            UpdateAccountError::TooManyRequests(ref cause) => cause,
            UpdateAccountError::Unauthorized(ref cause) => cause,
            UpdateAccountError::Validation(ref cause) => cause,
            UpdateAccountError::Credentials(ref err) => err.description(),
            UpdateAccountError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateAccountError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApiKey
#[derive(Debug, PartialEq)]
pub enum UpdateApiKeyError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateApiKeyError {
    pub fn from_body(body: &str) -> UpdateApiKeyError {
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
                    "BadRequestException" => {
                        UpdateApiKeyError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => UpdateApiKeyError::Conflict(String::from(error_message)),
                    "NotFoundException" => UpdateApiKeyError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        UpdateApiKeyError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateApiKeyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateApiKeyError::Validation(error_message.to_string())
                    }
                    _ => UpdateApiKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateApiKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateApiKeyError {
    fn from(err: serde_json::error::Error) -> UpdateApiKeyError {
        UpdateApiKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApiKeyError {
    fn from(err: CredentialsError) -> UpdateApiKeyError {
        UpdateApiKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApiKeyError {
    fn from(err: HttpDispatchError) -> UpdateApiKeyError {
        UpdateApiKeyError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApiKeyError {
    fn from(err: io::Error) -> UpdateApiKeyError {
        UpdateApiKeyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApiKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApiKeyError {
    fn description(&self) -> &str {
        match *self {
            UpdateApiKeyError::BadRequest(ref cause) => cause,
            UpdateApiKeyError::Conflict(ref cause) => cause,
            UpdateApiKeyError::NotFound(ref cause) => cause,
            UpdateApiKeyError::TooManyRequests(ref cause) => cause,
            UpdateApiKeyError::Unauthorized(ref cause) => cause,
            UpdateApiKeyError::Validation(ref cause) => cause,
            UpdateApiKeyError::Credentials(ref err) => err.description(),
            UpdateApiKeyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateApiKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAuthorizer
#[derive(Debug, PartialEq)]
pub enum UpdateAuthorizerError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateAuthorizerError {
    pub fn from_body(body: &str) -> UpdateAuthorizerError {
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
                    "BadRequestException" => {
                        UpdateAuthorizerError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateAuthorizerError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateAuthorizerError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateAuthorizerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateAuthorizerError::Validation(error_message.to_string())
                    }
                    _ => UpdateAuthorizerError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateAuthorizerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateAuthorizerError {
    fn from(err: serde_json::error::Error) -> UpdateAuthorizerError {
        UpdateAuthorizerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAuthorizerError {
    fn from(err: CredentialsError) -> UpdateAuthorizerError {
        UpdateAuthorizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAuthorizerError {
    fn from(err: HttpDispatchError) -> UpdateAuthorizerError {
        UpdateAuthorizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAuthorizerError {
    fn from(err: io::Error) -> UpdateAuthorizerError {
        UpdateAuthorizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            UpdateAuthorizerError::BadRequest(ref cause) => cause,
            UpdateAuthorizerError::NotFound(ref cause) => cause,
            UpdateAuthorizerError::TooManyRequests(ref cause) => cause,
            UpdateAuthorizerError::Unauthorized(ref cause) => cause,
            UpdateAuthorizerError::Validation(ref cause) => cause,
            UpdateAuthorizerError::Credentials(ref err) => err.description(),
            UpdateAuthorizerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateAuthorizerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateBasePathMapping
#[derive(Debug, PartialEq)]
pub enum UpdateBasePathMappingError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateBasePathMappingError {
    pub fn from_body(body: &str) -> UpdateBasePathMappingError {
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
                    "BadRequestException" => {
                        UpdateBasePathMappingError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateBasePathMappingError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateBasePathMappingError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateBasePathMappingError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateBasePathMappingError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateBasePathMappingError::Validation(error_message.to_string())
                    }
                    _ => UpdateBasePathMappingError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateBasePathMappingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateBasePathMappingError {
    fn from(err: serde_json::error::Error) -> UpdateBasePathMappingError {
        UpdateBasePathMappingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateBasePathMappingError {
    fn from(err: CredentialsError) -> UpdateBasePathMappingError {
        UpdateBasePathMappingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateBasePathMappingError {
    fn from(err: HttpDispatchError) -> UpdateBasePathMappingError {
        UpdateBasePathMappingError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateBasePathMappingError {
    fn from(err: io::Error) -> UpdateBasePathMappingError {
        UpdateBasePathMappingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateBasePathMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBasePathMappingError {
    fn description(&self) -> &str {
        match *self {
            UpdateBasePathMappingError::BadRequest(ref cause) => cause,
            UpdateBasePathMappingError::Conflict(ref cause) => cause,
            UpdateBasePathMappingError::NotFound(ref cause) => cause,
            UpdateBasePathMappingError::TooManyRequests(ref cause) => cause,
            UpdateBasePathMappingError::Unauthorized(ref cause) => cause,
            UpdateBasePathMappingError::Validation(ref cause) => cause,
            UpdateBasePathMappingError::Credentials(ref err) => err.description(),
            UpdateBasePathMappingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateBasePathMappingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateClientCertificate
#[derive(Debug, PartialEq)]
pub enum UpdateClientCertificateError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateClientCertificateError {
    pub fn from_body(body: &str) -> UpdateClientCertificateError {
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
                    "BadRequestException" => {
                        UpdateClientCertificateError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateClientCertificateError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateClientCertificateError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateClientCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateClientCertificateError::Validation(error_message.to_string())
                    }
                    _ => UpdateClientCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateClientCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateClientCertificateError {
    fn from(err: serde_json::error::Error) -> UpdateClientCertificateError {
        UpdateClientCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateClientCertificateError {
    fn from(err: CredentialsError) -> UpdateClientCertificateError {
        UpdateClientCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateClientCertificateError {
    fn from(err: HttpDispatchError) -> UpdateClientCertificateError {
        UpdateClientCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateClientCertificateError {
    fn from(err: io::Error) -> UpdateClientCertificateError {
        UpdateClientCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateClientCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateClientCertificateError {
    fn description(&self) -> &str {
        match *self {
            UpdateClientCertificateError::BadRequest(ref cause) => cause,
            UpdateClientCertificateError::NotFound(ref cause) => cause,
            UpdateClientCertificateError::TooManyRequests(ref cause) => cause,
            UpdateClientCertificateError::Unauthorized(ref cause) => cause,
            UpdateClientCertificateError::Validation(ref cause) => cause,
            UpdateClientCertificateError::Credentials(ref err) => err.description(),
            UpdateClientCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateClientCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDeployment
#[derive(Debug, PartialEq)]
pub enum UpdateDeploymentError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The requested service is not available. For details see the accompanying error message. Retry after the specified time period.</p>
    ServiceUnavailable(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateDeploymentError {
    pub fn from_body(body: &str) -> UpdateDeploymentError {
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
                    "BadRequestException" => {
                        UpdateDeploymentError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateDeploymentError::NotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateDeploymentError::ServiceUnavailable(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateDeploymentError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateDeploymentError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDeploymentError::Validation(error_message.to_string())
                    }
                    _ => UpdateDeploymentError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDeploymentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDeploymentError {
    fn from(err: serde_json::error::Error) -> UpdateDeploymentError {
        UpdateDeploymentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDeploymentError {
    fn from(err: CredentialsError) -> UpdateDeploymentError {
        UpdateDeploymentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDeploymentError {
    fn from(err: HttpDispatchError) -> UpdateDeploymentError {
        UpdateDeploymentError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDeploymentError {
    fn from(err: io::Error) -> UpdateDeploymentError {
        UpdateDeploymentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDeploymentError {
    fn description(&self) -> &str {
        match *self {
            UpdateDeploymentError::BadRequest(ref cause) => cause,
            UpdateDeploymentError::NotFound(ref cause) => cause,
            UpdateDeploymentError::ServiceUnavailable(ref cause) => cause,
            UpdateDeploymentError::TooManyRequests(ref cause) => cause,
            UpdateDeploymentError::Unauthorized(ref cause) => cause,
            UpdateDeploymentError::Validation(ref cause) => cause,
            UpdateDeploymentError::Credentials(ref err) => err.description(),
            UpdateDeploymentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateDeploymentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDocumentationPart
#[derive(Debug, PartialEq)]
pub enum UpdateDocumentationPartError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateDocumentationPartError {
    pub fn from_body(body: &str) -> UpdateDocumentationPartError {
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
                    "BadRequestException" => {
                        UpdateDocumentationPartError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateDocumentationPartError::Conflict(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateDocumentationPartError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateDocumentationPartError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateDocumentationPartError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateDocumentationPartError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDocumentationPartError::Validation(error_message.to_string())
                    }
                    _ => UpdateDocumentationPartError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDocumentationPartError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDocumentationPartError {
    fn from(err: serde_json::error::Error) -> UpdateDocumentationPartError {
        UpdateDocumentationPartError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDocumentationPartError {
    fn from(err: CredentialsError) -> UpdateDocumentationPartError {
        UpdateDocumentationPartError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDocumentationPartError {
    fn from(err: HttpDispatchError) -> UpdateDocumentationPartError {
        UpdateDocumentationPartError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDocumentationPartError {
    fn from(err: io::Error) -> UpdateDocumentationPartError {
        UpdateDocumentationPartError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDocumentationPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDocumentationPartError {
    fn description(&self) -> &str {
        match *self {
            UpdateDocumentationPartError::BadRequest(ref cause) => cause,
            UpdateDocumentationPartError::Conflict(ref cause) => cause,
            UpdateDocumentationPartError::LimitExceeded(ref cause) => cause,
            UpdateDocumentationPartError::NotFound(ref cause) => cause,
            UpdateDocumentationPartError::TooManyRequests(ref cause) => cause,
            UpdateDocumentationPartError::Unauthorized(ref cause) => cause,
            UpdateDocumentationPartError::Validation(ref cause) => cause,
            UpdateDocumentationPartError::Credentials(ref err) => err.description(),
            UpdateDocumentationPartError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDocumentationPartError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDocumentationVersion
#[derive(Debug, PartialEq)]
pub enum UpdateDocumentationVersionError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateDocumentationVersionError {
    pub fn from_body(body: &str) -> UpdateDocumentationVersionError {
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
                    "BadRequestException" => {
                        UpdateDocumentationVersionError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateDocumentationVersionError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateDocumentationVersionError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => UpdateDocumentationVersionError::TooManyRequests(
                        String::from(error_message),
                    ),
                    "UnauthorizedException" => {
                        UpdateDocumentationVersionError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDocumentationVersionError::Validation(error_message.to_string())
                    }
                    _ => UpdateDocumentationVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDocumentationVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDocumentationVersionError {
    fn from(err: serde_json::error::Error) -> UpdateDocumentationVersionError {
        UpdateDocumentationVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDocumentationVersionError {
    fn from(err: CredentialsError) -> UpdateDocumentationVersionError {
        UpdateDocumentationVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDocumentationVersionError {
    fn from(err: HttpDispatchError) -> UpdateDocumentationVersionError {
        UpdateDocumentationVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDocumentationVersionError {
    fn from(err: io::Error) -> UpdateDocumentationVersionError {
        UpdateDocumentationVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDocumentationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDocumentationVersionError {
    fn description(&self) -> &str {
        match *self {
            UpdateDocumentationVersionError::BadRequest(ref cause) => cause,
            UpdateDocumentationVersionError::Conflict(ref cause) => cause,
            UpdateDocumentationVersionError::NotFound(ref cause) => cause,
            UpdateDocumentationVersionError::TooManyRequests(ref cause) => cause,
            UpdateDocumentationVersionError::Unauthorized(ref cause) => cause,
            UpdateDocumentationVersionError::Validation(ref cause) => cause,
            UpdateDocumentationVersionError::Credentials(ref err) => err.description(),
            UpdateDocumentationVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDocumentationVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDomainName
#[derive(Debug, PartialEq)]
pub enum UpdateDomainNameError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateDomainNameError {
    pub fn from_body(body: &str) -> UpdateDomainNameError {
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
                    "BadRequestException" => {
                        UpdateDomainNameError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateDomainNameError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateDomainNameError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateDomainNameError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateDomainNameError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDomainNameError::Validation(error_message.to_string())
                    }
                    _ => UpdateDomainNameError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDomainNameError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDomainNameError {
    fn from(err: serde_json::error::Error) -> UpdateDomainNameError {
        UpdateDomainNameError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDomainNameError {
    fn from(err: CredentialsError) -> UpdateDomainNameError {
        UpdateDomainNameError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDomainNameError {
    fn from(err: HttpDispatchError) -> UpdateDomainNameError {
        UpdateDomainNameError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDomainNameError {
    fn from(err: io::Error) -> UpdateDomainNameError {
        UpdateDomainNameError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDomainNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDomainNameError {
    fn description(&self) -> &str {
        match *self {
            UpdateDomainNameError::BadRequest(ref cause) => cause,
            UpdateDomainNameError::Conflict(ref cause) => cause,
            UpdateDomainNameError::NotFound(ref cause) => cause,
            UpdateDomainNameError::TooManyRequests(ref cause) => cause,
            UpdateDomainNameError::Unauthorized(ref cause) => cause,
            UpdateDomainNameError::Validation(ref cause) => cause,
            UpdateDomainNameError::Credentials(ref err) => err.description(),
            UpdateDomainNameError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateDomainNameError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGatewayResponse
#[derive(Debug, PartialEq)]
pub enum UpdateGatewayResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateGatewayResponseError {
    pub fn from_body(body: &str) -> UpdateGatewayResponseError {
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
                    "BadRequestException" => {
                        UpdateGatewayResponseError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateGatewayResponseError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateGatewayResponseError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateGatewayResponseError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateGatewayResponseError::Validation(error_message.to_string())
                    }
                    _ => UpdateGatewayResponseError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateGatewayResponseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateGatewayResponseError {
    fn from(err: serde_json::error::Error) -> UpdateGatewayResponseError {
        UpdateGatewayResponseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGatewayResponseError {
    fn from(err: CredentialsError) -> UpdateGatewayResponseError {
        UpdateGatewayResponseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGatewayResponseError {
    fn from(err: HttpDispatchError) -> UpdateGatewayResponseError {
        UpdateGatewayResponseError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateGatewayResponseError {
    fn from(err: io::Error) -> UpdateGatewayResponseError {
        UpdateGatewayResponseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateGatewayResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGatewayResponseError {
    fn description(&self) -> &str {
        match *self {
            UpdateGatewayResponseError::BadRequest(ref cause) => cause,
            UpdateGatewayResponseError::NotFound(ref cause) => cause,
            UpdateGatewayResponseError::TooManyRequests(ref cause) => cause,
            UpdateGatewayResponseError::Unauthorized(ref cause) => cause,
            UpdateGatewayResponseError::Validation(ref cause) => cause,
            UpdateGatewayResponseError::Credentials(ref err) => err.description(),
            UpdateGatewayResponseError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateGatewayResponseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateIntegration
#[derive(Debug, PartialEq)]
pub enum UpdateIntegrationError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateIntegrationError {
    pub fn from_body(body: &str) -> UpdateIntegrationError {
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
                    "BadRequestException" => {
                        UpdateIntegrationError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateIntegrationError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateIntegrationError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateIntegrationError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateIntegrationError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateIntegrationError::Validation(error_message.to_string())
                    }
                    _ => UpdateIntegrationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateIntegrationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateIntegrationError {
    fn from(err: serde_json::error::Error) -> UpdateIntegrationError {
        UpdateIntegrationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateIntegrationError {
    fn from(err: CredentialsError) -> UpdateIntegrationError {
        UpdateIntegrationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateIntegrationError {
    fn from(err: HttpDispatchError) -> UpdateIntegrationError {
        UpdateIntegrationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateIntegrationError {
    fn from(err: io::Error) -> UpdateIntegrationError {
        UpdateIntegrationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateIntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateIntegrationError {
    fn description(&self) -> &str {
        match *self {
            UpdateIntegrationError::BadRequest(ref cause) => cause,
            UpdateIntegrationError::Conflict(ref cause) => cause,
            UpdateIntegrationError::NotFound(ref cause) => cause,
            UpdateIntegrationError::TooManyRequests(ref cause) => cause,
            UpdateIntegrationError::Unauthorized(ref cause) => cause,
            UpdateIntegrationError::Validation(ref cause) => cause,
            UpdateIntegrationError::Credentials(ref err) => err.description(),
            UpdateIntegrationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateIntegrationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum UpdateIntegrationResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateIntegrationResponseError {
    pub fn from_body(body: &str) -> UpdateIntegrationResponseError {
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
                    "BadRequestException" => {
                        UpdateIntegrationResponseError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateIntegrationResponseError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateIntegrationResponseError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateIntegrationResponseError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateIntegrationResponseError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateIntegrationResponseError::Validation(error_message.to_string())
                    }
                    _ => UpdateIntegrationResponseError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateIntegrationResponseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateIntegrationResponseError {
    fn from(err: serde_json::error::Error) -> UpdateIntegrationResponseError {
        UpdateIntegrationResponseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateIntegrationResponseError {
    fn from(err: CredentialsError) -> UpdateIntegrationResponseError {
        UpdateIntegrationResponseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateIntegrationResponseError {
    fn from(err: HttpDispatchError) -> UpdateIntegrationResponseError {
        UpdateIntegrationResponseError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateIntegrationResponseError {
    fn from(err: io::Error) -> UpdateIntegrationResponseError {
        UpdateIntegrationResponseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateIntegrationResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateIntegrationResponseError {
    fn description(&self) -> &str {
        match *self {
            UpdateIntegrationResponseError::BadRequest(ref cause) => cause,
            UpdateIntegrationResponseError::Conflict(ref cause) => cause,
            UpdateIntegrationResponseError::NotFound(ref cause) => cause,
            UpdateIntegrationResponseError::TooManyRequests(ref cause) => cause,
            UpdateIntegrationResponseError::Unauthorized(ref cause) => cause,
            UpdateIntegrationResponseError::Validation(ref cause) => cause,
            UpdateIntegrationResponseError::Credentials(ref err) => err.description(),
            UpdateIntegrationResponseError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateIntegrationResponseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMethod
#[derive(Debug, PartialEq)]
pub enum UpdateMethodError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateMethodError {
    pub fn from_body(body: &str) -> UpdateMethodError {
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
                    "BadRequestException" => {
                        UpdateMethodError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => UpdateMethodError::Conflict(String::from(error_message)),
                    "NotFoundException" => UpdateMethodError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        UpdateMethodError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateMethodError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateMethodError::Validation(error_message.to_string())
                    }
                    _ => UpdateMethodError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateMethodError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateMethodError {
    fn from(err: serde_json::error::Error) -> UpdateMethodError {
        UpdateMethodError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateMethodError {
    fn from(err: CredentialsError) -> UpdateMethodError {
        UpdateMethodError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateMethodError {
    fn from(err: HttpDispatchError) -> UpdateMethodError {
        UpdateMethodError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateMethodError {
    fn from(err: io::Error) -> UpdateMethodError {
        UpdateMethodError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMethodError {
    fn description(&self) -> &str {
        match *self {
            UpdateMethodError::BadRequest(ref cause) => cause,
            UpdateMethodError::Conflict(ref cause) => cause,
            UpdateMethodError::NotFound(ref cause) => cause,
            UpdateMethodError::TooManyRequests(ref cause) => cause,
            UpdateMethodError::Unauthorized(ref cause) => cause,
            UpdateMethodError::Validation(ref cause) => cause,
            UpdateMethodError::Credentials(ref err) => err.description(),
            UpdateMethodError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateMethodError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMethodResponse
#[derive(Debug, PartialEq)]
pub enum UpdateMethodResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateMethodResponseError {
    pub fn from_body(body: &str) -> UpdateMethodResponseError {
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
                    "BadRequestException" => {
                        UpdateMethodResponseError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateMethodResponseError::Conflict(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateMethodResponseError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateMethodResponseError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateMethodResponseError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateMethodResponseError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateMethodResponseError::Validation(error_message.to_string())
                    }
                    _ => UpdateMethodResponseError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateMethodResponseError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateMethodResponseError {
    fn from(err: serde_json::error::Error) -> UpdateMethodResponseError {
        UpdateMethodResponseError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateMethodResponseError {
    fn from(err: CredentialsError) -> UpdateMethodResponseError {
        UpdateMethodResponseError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateMethodResponseError {
    fn from(err: HttpDispatchError) -> UpdateMethodResponseError {
        UpdateMethodResponseError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateMethodResponseError {
    fn from(err: io::Error) -> UpdateMethodResponseError {
        UpdateMethodResponseError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateMethodResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMethodResponseError {
    fn description(&self) -> &str {
        match *self {
            UpdateMethodResponseError::BadRequest(ref cause) => cause,
            UpdateMethodResponseError::Conflict(ref cause) => cause,
            UpdateMethodResponseError::LimitExceeded(ref cause) => cause,
            UpdateMethodResponseError::NotFound(ref cause) => cause,
            UpdateMethodResponseError::TooManyRequests(ref cause) => cause,
            UpdateMethodResponseError::Unauthorized(ref cause) => cause,
            UpdateMethodResponseError::Validation(ref cause) => cause,
            UpdateMethodResponseError::Credentials(ref err) => err.description(),
            UpdateMethodResponseError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateMethodResponseError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateModel
#[derive(Debug, PartialEq)]
pub enum UpdateModelError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateModelError {
    pub fn from_body(body: &str) -> UpdateModelError {
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
                    "BadRequestException" => {
                        UpdateModelError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => UpdateModelError::Conflict(String::from(error_message)),
                    "NotFoundException" => UpdateModelError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        UpdateModelError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateModelError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateModelError::Validation(error_message.to_string())
                    }
                    _ => UpdateModelError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateModelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateModelError {
    fn from(err: serde_json::error::Error) -> UpdateModelError {
        UpdateModelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateModelError {
    fn from(err: CredentialsError) -> UpdateModelError {
        UpdateModelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateModelError {
    fn from(err: HttpDispatchError) -> UpdateModelError {
        UpdateModelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateModelError {
    fn from(err: io::Error) -> UpdateModelError {
        UpdateModelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateModelError {
    fn description(&self) -> &str {
        match *self {
            UpdateModelError::BadRequest(ref cause) => cause,
            UpdateModelError::Conflict(ref cause) => cause,
            UpdateModelError::NotFound(ref cause) => cause,
            UpdateModelError::TooManyRequests(ref cause) => cause,
            UpdateModelError::Unauthorized(ref cause) => cause,
            UpdateModelError::Validation(ref cause) => cause,
            UpdateModelError::Credentials(ref err) => err.description(),
            UpdateModelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateModelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRequestValidator
#[derive(Debug, PartialEq)]
pub enum UpdateRequestValidatorError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateRequestValidatorError {
    pub fn from_body(body: &str) -> UpdateRequestValidatorError {
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
                    "BadRequestException" => {
                        UpdateRequestValidatorError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateRequestValidatorError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateRequestValidatorError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateRequestValidatorError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateRequestValidatorError::Validation(error_message.to_string())
                    }
                    _ => UpdateRequestValidatorError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateRequestValidatorError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateRequestValidatorError {
    fn from(err: serde_json::error::Error) -> UpdateRequestValidatorError {
        UpdateRequestValidatorError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateRequestValidatorError {
    fn from(err: CredentialsError) -> UpdateRequestValidatorError {
        UpdateRequestValidatorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateRequestValidatorError {
    fn from(err: HttpDispatchError) -> UpdateRequestValidatorError {
        UpdateRequestValidatorError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateRequestValidatorError {
    fn from(err: io::Error) -> UpdateRequestValidatorError {
        UpdateRequestValidatorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateRequestValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRequestValidatorError {
    fn description(&self) -> &str {
        match *self {
            UpdateRequestValidatorError::BadRequest(ref cause) => cause,
            UpdateRequestValidatorError::NotFound(ref cause) => cause,
            UpdateRequestValidatorError::TooManyRequests(ref cause) => cause,
            UpdateRequestValidatorError::Unauthorized(ref cause) => cause,
            UpdateRequestValidatorError::Validation(ref cause) => cause,
            UpdateRequestValidatorError::Credentials(ref err) => err.description(),
            UpdateRequestValidatorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateRequestValidatorError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateResource
#[derive(Debug, PartialEq)]
pub enum UpdateResourceError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateResourceError {
    pub fn from_body(body: &str) -> UpdateResourceError {
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
                    "BadRequestException" => {
                        UpdateResourceError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateResourceError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateResourceError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateResourceError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateResourceError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateResourceError::Validation(error_message.to_string())
                    }
                    _ => UpdateResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateResourceError {
    fn from(err: serde_json::error::Error) -> UpdateResourceError {
        UpdateResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateResourceError {
    fn from(err: CredentialsError) -> UpdateResourceError {
        UpdateResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateResourceError {
    fn from(err: HttpDispatchError) -> UpdateResourceError {
        UpdateResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateResourceError {
    fn from(err: io::Error) -> UpdateResourceError {
        UpdateResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateResourceError {
    fn description(&self) -> &str {
        match *self {
            UpdateResourceError::BadRequest(ref cause) => cause,
            UpdateResourceError::Conflict(ref cause) => cause,
            UpdateResourceError::NotFound(ref cause) => cause,
            UpdateResourceError::TooManyRequests(ref cause) => cause,
            UpdateResourceError::Unauthorized(ref cause) => cause,
            UpdateResourceError::Validation(ref cause) => cause,
            UpdateResourceError::Credentials(ref err) => err.description(),
            UpdateResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRestApi
#[derive(Debug, PartialEq)]
pub enum UpdateRestApiError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateRestApiError {
    pub fn from_body(body: &str) -> UpdateRestApiError {
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
                    "BadRequestException" => {
                        UpdateRestApiError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateRestApiError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateRestApiError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateRestApiError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateRestApiError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateRestApiError::Validation(error_message.to_string())
                    }
                    _ => UpdateRestApiError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateRestApiError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateRestApiError {
    fn from(err: serde_json::error::Error) -> UpdateRestApiError {
        UpdateRestApiError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateRestApiError {
    fn from(err: CredentialsError) -> UpdateRestApiError {
        UpdateRestApiError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateRestApiError {
    fn from(err: HttpDispatchError) -> UpdateRestApiError {
        UpdateRestApiError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateRestApiError {
    fn from(err: io::Error) -> UpdateRestApiError {
        UpdateRestApiError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateRestApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRestApiError {
    fn description(&self) -> &str {
        match *self {
            UpdateRestApiError::BadRequest(ref cause) => cause,
            UpdateRestApiError::Conflict(ref cause) => cause,
            UpdateRestApiError::NotFound(ref cause) => cause,
            UpdateRestApiError::TooManyRequests(ref cause) => cause,
            UpdateRestApiError::Unauthorized(ref cause) => cause,
            UpdateRestApiError::Validation(ref cause) => cause,
            UpdateRestApiError::Credentials(ref err) => err.description(),
            UpdateRestApiError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateRestApiError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateStage
#[derive(Debug, PartialEq)]
pub enum UpdateStageError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateStageError {
    pub fn from_body(body: &str) -> UpdateStageError {
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
                    "BadRequestException" => {
                        UpdateStageError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => UpdateStageError::Conflict(String::from(error_message)),
                    "NotFoundException" => UpdateStageError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        UpdateStageError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateStageError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateStageError::Validation(error_message.to_string())
                    }
                    _ => UpdateStageError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateStageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateStageError {
    fn from(err: serde_json::error::Error) -> UpdateStageError {
        UpdateStageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateStageError {
    fn from(err: CredentialsError) -> UpdateStageError {
        UpdateStageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateStageError {
    fn from(err: HttpDispatchError) -> UpdateStageError {
        UpdateStageError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateStageError {
    fn from(err: io::Error) -> UpdateStageError {
        UpdateStageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateStageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateStageError {
    fn description(&self) -> &str {
        match *self {
            UpdateStageError::BadRequest(ref cause) => cause,
            UpdateStageError::Conflict(ref cause) => cause,
            UpdateStageError::NotFound(ref cause) => cause,
            UpdateStageError::TooManyRequests(ref cause) => cause,
            UpdateStageError::Unauthorized(ref cause) => cause,
            UpdateStageError::Validation(ref cause) => cause,
            UpdateStageError::Credentials(ref err) => err.description(),
            UpdateStageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateStageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUsage
#[derive(Debug, PartialEq)]
pub enum UpdateUsageError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateUsageError {
    pub fn from_body(body: &str) -> UpdateUsageError {
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
                    "BadRequestException" => {
                        UpdateUsageError::BadRequest(String::from(error_message))
                    }
                    "NotFoundException" => UpdateUsageError::NotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        UpdateUsageError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateUsageError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateUsageError::Validation(error_message.to_string())
                    }
                    _ => UpdateUsageError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateUsageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateUsageError {
    fn from(err: serde_json::error::Error) -> UpdateUsageError {
        UpdateUsageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUsageError {
    fn from(err: CredentialsError) -> UpdateUsageError {
        UpdateUsageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUsageError {
    fn from(err: HttpDispatchError) -> UpdateUsageError {
        UpdateUsageError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateUsageError {
    fn from(err: io::Error) -> UpdateUsageError {
        UpdateUsageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUsageError {
    fn description(&self) -> &str {
        match *self {
            UpdateUsageError::BadRequest(ref cause) => cause,
            UpdateUsageError::NotFound(ref cause) => cause,
            UpdateUsageError::TooManyRequests(ref cause) => cause,
            UpdateUsageError::Unauthorized(ref cause) => cause,
            UpdateUsageError::Validation(ref cause) => cause,
            UpdateUsageError::Credentials(ref err) => err.description(),
            UpdateUsageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateUsageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUsagePlan
#[derive(Debug, PartialEq)]
pub enum UpdateUsagePlanError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateUsagePlanError {
    pub fn from_body(body: &str) -> UpdateUsagePlanError {
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
                    "BadRequestException" => {
                        UpdateUsagePlanError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateUsagePlanError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateUsagePlanError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateUsagePlanError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateUsagePlanError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateUsagePlanError::Validation(error_message.to_string())
                    }
                    _ => UpdateUsagePlanError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateUsagePlanError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateUsagePlanError {
    fn from(err: serde_json::error::Error) -> UpdateUsagePlanError {
        UpdateUsagePlanError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUsagePlanError {
    fn from(err: CredentialsError) -> UpdateUsagePlanError {
        UpdateUsagePlanError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUsagePlanError {
    fn from(err: HttpDispatchError) -> UpdateUsagePlanError {
        UpdateUsagePlanError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateUsagePlanError {
    fn from(err: io::Error) -> UpdateUsagePlanError {
        UpdateUsagePlanError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateUsagePlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUsagePlanError {
    fn description(&self) -> &str {
        match *self {
            UpdateUsagePlanError::BadRequest(ref cause) => cause,
            UpdateUsagePlanError::Conflict(ref cause) => cause,
            UpdateUsagePlanError::NotFound(ref cause) => cause,
            UpdateUsagePlanError::TooManyRequests(ref cause) => cause,
            UpdateUsagePlanError::Unauthorized(ref cause) => cause,
            UpdateUsagePlanError::Validation(ref cause) => cause,
            UpdateUsagePlanError::Credentials(ref err) => err.description(),
            UpdateUsagePlanError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateUsagePlanError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateVpcLink
#[derive(Debug, PartialEq)]
pub enum UpdateVpcLinkError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
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

impl UpdateVpcLinkError {
    pub fn from_body(body: &str) -> UpdateVpcLinkError {
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
                    "BadRequestException" => {
                        UpdateVpcLinkError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateVpcLinkError::Conflict(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateVpcLinkError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateVpcLinkError::TooManyRequests(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateVpcLinkError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateVpcLinkError::Validation(error_message.to_string())
                    }
                    _ => UpdateVpcLinkError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateVpcLinkError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateVpcLinkError {
    fn from(err: serde_json::error::Error) -> UpdateVpcLinkError {
        UpdateVpcLinkError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateVpcLinkError {
    fn from(err: CredentialsError) -> UpdateVpcLinkError {
        UpdateVpcLinkError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateVpcLinkError {
    fn from(err: HttpDispatchError) -> UpdateVpcLinkError {
        UpdateVpcLinkError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateVpcLinkError {
    fn from(err: io::Error) -> UpdateVpcLinkError {
        UpdateVpcLinkError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateVpcLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateVpcLinkError {
    fn description(&self) -> &str {
        match *self {
            UpdateVpcLinkError::BadRequest(ref cause) => cause,
            UpdateVpcLinkError::Conflict(ref cause) => cause,
            UpdateVpcLinkError::NotFound(ref cause) => cause,
            UpdateVpcLinkError::TooManyRequests(ref cause) => cause,
            UpdateVpcLinkError::Unauthorized(ref cause) => cause,
            UpdateVpcLinkError::Validation(ref cause) => cause,
            UpdateVpcLinkError::Credentials(ref err) => err.description(),
            UpdateVpcLinkError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateVpcLinkError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon API Gateway API. Amazon API Gateway clients implement this trait.
pub trait ApiGateway {
    /// <p><p>Create an <a>ApiKey</a> resource. </p> <div class="seeAlso"><a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/create-api-key.html">AWS CLI</a></div></p>
    fn create_api_key(&self, input: CreateApiKeyRequest)
        -> RusotoFuture<ApiKey, CreateApiKeyError>;

    /// <p><p>Adds a new <a>Authorizer</a> resource to an existing <a>RestApi</a> resource.</p> <div class="seeAlso"><a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/create-authorizer.html">AWS CLI</a></div></p>
    fn create_authorizer(
        &self,
        input: CreateAuthorizerRequest,
    ) -> RusotoFuture<Authorizer, CreateAuthorizerError>;

    /// <p>Creates a new <a>BasePathMapping</a> resource.</p>
    fn create_base_path_mapping(
        &self,
        input: CreateBasePathMappingRequest,
    ) -> RusotoFuture<BasePathMapping, CreateBasePathMappingError>;

    /// <p>Creates a <a>Deployment</a> resource, which makes a specified <a>RestApi</a> callable over the internet.</p>
    fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> RusotoFuture<Deployment, CreateDeploymentError>;

    fn create_documentation_part(
        &self,
        input: CreateDocumentationPartRequest,
    ) -> RusotoFuture<DocumentationPart, CreateDocumentationPartError>;

    fn create_documentation_version(
        &self,
        input: CreateDocumentationVersionRequest,
    ) -> RusotoFuture<DocumentationVersion, CreateDocumentationVersionError>;

    /// <p>Creates a new domain name.</p>
    fn create_domain_name(
        &self,
        input: CreateDomainNameRequest,
    ) -> RusotoFuture<DomainName, CreateDomainNameError>;

    /// <p>Adds a new <a>Model</a> resource to an existing <a>RestApi</a> resource.</p>
    fn create_model(&self, input: CreateModelRequest) -> RusotoFuture<Model, CreateModelError>;

    /// <p>Creates a <a>ReqeustValidator</a> of a given <a>RestApi</a>.</p>
    fn create_request_validator(
        &self,
        input: CreateRequestValidatorRequest,
    ) -> RusotoFuture<RequestValidator, CreateRequestValidatorError>;

    /// <p>Creates a <a>Resource</a> resource.</p>
    fn create_resource(
        &self,
        input: CreateResourceRequest,
    ) -> RusotoFuture<Resource, CreateResourceError>;

    /// <p>Creates a new <a>RestApi</a> resource.</p>
    fn create_rest_api(
        &self,
        input: CreateRestApiRequest,
    ) -> RusotoFuture<RestApi, CreateRestApiError>;

    /// <p>Creates a new <a>Stage</a> resource that references a pre-existing <a>Deployment</a> for the API. </p>
    fn create_stage(&self, input: CreateStageRequest) -> RusotoFuture<Stage, CreateStageError>;

    /// <p>Creates a usage plan with the throttle and quota limits, as well as the associated API stages, specified in the payload. </p>
    fn create_usage_plan(
        &self,
        input: CreateUsagePlanRequest,
    ) -> RusotoFuture<UsagePlan, CreateUsagePlanError>;

    /// <p>Creates a usage plan key for adding an existing API key to a usage plan.</p>
    fn create_usage_plan_key(
        &self,
        input: CreateUsagePlanKeyRequest,
    ) -> RusotoFuture<UsagePlanKey, CreateUsagePlanKeyError>;

    /// <p>Creates a VPC link, under the caller's account in a selected region, in an asynchronous operation that typically takes 2-4 minutes to complete and become operational. The caller must have permissions to create and update VPC Endpoint services.</p>
    fn create_vpc_link(
        &self,
        input: CreateVpcLinkRequest,
    ) -> RusotoFuture<VpcLink, CreateVpcLinkError>;

    /// <p>Deletes the <a>ApiKey</a> resource.</p>
    fn delete_api_key(&self, input: DeleteApiKeyRequest) -> RusotoFuture<(), DeleteApiKeyError>;

    /// <p><p>Deletes an existing <a>Authorizer</a> resource.</p> <div class="seeAlso"><a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/delete-authorizer.html">AWS CLI</a></div></p>
    fn delete_authorizer(
        &self,
        input: DeleteAuthorizerRequest,
    ) -> RusotoFuture<(), DeleteAuthorizerError>;

    /// <p>Deletes the <a>BasePathMapping</a> resource.</p>
    fn delete_base_path_mapping(
        &self,
        input: DeleteBasePathMappingRequest,
    ) -> RusotoFuture<(), DeleteBasePathMappingError>;

    /// <p>Deletes the <a>ClientCertificate</a> resource.</p>
    fn delete_client_certificate(
        &self,
        input: DeleteClientCertificateRequest,
    ) -> RusotoFuture<(), DeleteClientCertificateError>;

    /// <p>Deletes a <a>Deployment</a> resource. Deleting a deployment will only succeed if there are no <a>Stage</a> resources associated with it.</p>
    fn delete_deployment(
        &self,
        input: DeleteDeploymentRequest,
    ) -> RusotoFuture<(), DeleteDeploymentError>;

    fn delete_documentation_part(
        &self,
        input: DeleteDocumentationPartRequest,
    ) -> RusotoFuture<(), DeleteDocumentationPartError>;

    fn delete_documentation_version(
        &self,
        input: DeleteDocumentationVersionRequest,
    ) -> RusotoFuture<(), DeleteDocumentationVersionError>;

    /// <p>Deletes the <a>DomainName</a> resource.</p>
    fn delete_domain_name(
        &self,
        input: DeleteDomainNameRequest,
    ) -> RusotoFuture<(), DeleteDomainNameError>;

    /// <p>Clears any customization of a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a> and resets it with the default settings.</p>
    fn delete_gateway_response(
        &self,
        input: DeleteGatewayResponseRequest,
    ) -> RusotoFuture<(), DeleteGatewayResponseError>;

    /// <p>Represents a delete integration.</p>
    fn delete_integration(
        &self,
        input: DeleteIntegrationRequest,
    ) -> RusotoFuture<(), DeleteIntegrationError>;

    /// <p>Represents a delete integration response.</p>
    fn delete_integration_response(
        &self,
        input: DeleteIntegrationResponseRequest,
    ) -> RusotoFuture<(), DeleteIntegrationResponseError>;

    /// <p>Deletes an existing <a>Method</a> resource.</p>
    fn delete_method(&self, input: DeleteMethodRequest) -> RusotoFuture<(), DeleteMethodError>;

    /// <p>Deletes an existing <a>MethodResponse</a> resource.</p>
    fn delete_method_response(
        &self,
        input: DeleteMethodResponseRequest,
    ) -> RusotoFuture<(), DeleteMethodResponseError>;

    /// <p>Deletes a model.</p>
    fn delete_model(&self, input: DeleteModelRequest) -> RusotoFuture<(), DeleteModelError>;

    /// <p>Deletes a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
    fn delete_request_validator(
        &self,
        input: DeleteRequestValidatorRequest,
    ) -> RusotoFuture<(), DeleteRequestValidatorError>;

    /// <p>Deletes a <a>Resource</a> resource.</p>
    fn delete_resource(
        &self,
        input: DeleteResourceRequest,
    ) -> RusotoFuture<(), DeleteResourceError>;

    /// <p>Deletes the specified API.</p>
    fn delete_rest_api(&self, input: DeleteRestApiRequest) -> RusotoFuture<(), DeleteRestApiError>;

    /// <p>Deletes a <a>Stage</a> resource.</p>
    fn delete_stage(&self, input: DeleteStageRequest) -> RusotoFuture<(), DeleteStageError>;

    /// <p>Deletes a usage plan of a given plan Id.</p>
    fn delete_usage_plan(
        &self,
        input: DeleteUsagePlanRequest,
    ) -> RusotoFuture<(), DeleteUsagePlanError>;

    /// <p>Deletes a usage plan key and remove the underlying API key from the associated usage plan.</p>
    fn delete_usage_plan_key(
        &self,
        input: DeleteUsagePlanKeyRequest,
    ) -> RusotoFuture<(), DeleteUsagePlanKeyError>;

    /// <p>Deletes an existing <a>VpcLink</a> of a specified identifier.</p>
    fn delete_vpc_link(&self, input: DeleteVpcLinkRequest) -> RusotoFuture<(), DeleteVpcLinkError>;

    /// <p>Flushes all authorizer cache entries on a stage.</p>
    fn flush_stage_authorizers_cache(
        &self,
        input: FlushStageAuthorizersCacheRequest,
    ) -> RusotoFuture<(), FlushStageAuthorizersCacheError>;

    /// <p>Flushes a stage's cache.</p>
    fn flush_stage_cache(
        &self,
        input: FlushStageCacheRequest,
    ) -> RusotoFuture<(), FlushStageCacheError>;

    /// <p>Generates a <a>ClientCertificate</a> resource.</p>
    fn generate_client_certificate(
        &self,
        input: GenerateClientCertificateRequest,
    ) -> RusotoFuture<ClientCertificate, GenerateClientCertificateError>;

    /// <p>Gets information about the current <a>Account</a> resource.</p>
    fn get_account(&self) -> RusotoFuture<Account, GetAccountError>;

    /// <p>Gets information about the current <a>ApiKey</a> resource.</p>
    fn get_api_key(&self, input: GetApiKeyRequest) -> RusotoFuture<ApiKey, GetApiKeyError>;

    /// <p>Gets information about the current <a>ApiKeys</a> resource.</p>
    fn get_api_keys(&self, input: GetApiKeysRequest) -> RusotoFuture<ApiKeys, GetApiKeysError>;

    /// <p><p>Describe an existing <a>Authorizer</a> resource.</p> <div class="seeAlso"><a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/get-authorizer.html">AWS CLI</a></div></p>
    fn get_authorizer(
        &self,
        input: GetAuthorizerRequest,
    ) -> RusotoFuture<Authorizer, GetAuthorizerError>;

    /// <p><p>Describe an existing <a>Authorizers</a> resource.</p> <div class="seeAlso"><a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/get-authorizers.html">AWS CLI</a></div></p>
    fn get_authorizers(
        &self,
        input: GetAuthorizersRequest,
    ) -> RusotoFuture<Authorizers, GetAuthorizersError>;

    /// <p>Describe a <a>BasePathMapping</a> resource.</p>
    fn get_base_path_mapping(
        &self,
        input: GetBasePathMappingRequest,
    ) -> RusotoFuture<BasePathMapping, GetBasePathMappingError>;

    /// <p>Represents a collection of <a>BasePathMapping</a> resources.</p>
    fn get_base_path_mappings(
        &self,
        input: GetBasePathMappingsRequest,
    ) -> RusotoFuture<BasePathMappings, GetBasePathMappingsError>;

    /// <p>Gets information about the current <a>ClientCertificate</a> resource.</p>
    fn get_client_certificate(
        &self,
        input: GetClientCertificateRequest,
    ) -> RusotoFuture<ClientCertificate, GetClientCertificateError>;

    /// <p>Gets a collection of <a>ClientCertificate</a> resources.</p>
    fn get_client_certificates(
        &self,
        input: GetClientCertificatesRequest,
    ) -> RusotoFuture<ClientCertificates, GetClientCertificatesError>;

    /// <p>Gets information about a <a>Deployment</a> resource.</p>
    fn get_deployment(
        &self,
        input: GetDeploymentRequest,
    ) -> RusotoFuture<Deployment, GetDeploymentError>;

    /// <p>Gets information about a <a>Deployments</a> collection.</p>
    fn get_deployments(
        &self,
        input: GetDeploymentsRequest,
    ) -> RusotoFuture<Deployments, GetDeploymentsError>;

    fn get_documentation_part(
        &self,
        input: GetDocumentationPartRequest,
    ) -> RusotoFuture<DocumentationPart, GetDocumentationPartError>;

    fn get_documentation_parts(
        &self,
        input: GetDocumentationPartsRequest,
    ) -> RusotoFuture<DocumentationParts, GetDocumentationPartsError>;

    fn get_documentation_version(
        &self,
        input: GetDocumentationVersionRequest,
    ) -> RusotoFuture<DocumentationVersion, GetDocumentationVersionError>;

    fn get_documentation_versions(
        &self,
        input: GetDocumentationVersionsRequest,
    ) -> RusotoFuture<DocumentationVersions, GetDocumentationVersionsError>;

    /// <p>Represents a domain name that is contained in a simpler, more intuitive URL that can be called.</p>
    fn get_domain_name(
        &self,
        input: GetDomainNameRequest,
    ) -> RusotoFuture<DomainName, GetDomainNameError>;

    /// <p>Represents a collection of <a>DomainName</a> resources.</p>
    fn get_domain_names(
        &self,
        input: GetDomainNamesRequest,
    ) -> RusotoFuture<DomainNames, GetDomainNamesError>;

    /// <p>Exports a deployed version of a <a>RestApi</a> in a specified format.</p>
    fn get_export(&self, input: GetExportRequest) -> RusotoFuture<ExportResponse, GetExportError>;

    /// <p>Gets a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a>.</p>
    fn get_gateway_response(
        &self,
        input: GetGatewayResponseRequest,
    ) -> RusotoFuture<GatewayResponse, GetGatewayResponseError>;

    /// <p>Gets the <a>GatewayResponses</a> collection on the given <a>RestApi</a>. If an API developer has not added any definitions for gateway responses, the result will be the API Gateway-generated default <a>GatewayResponses</a> collection for the supported response types.</p>
    fn get_gateway_responses(
        &self,
        input: GetGatewayResponsesRequest,
    ) -> RusotoFuture<GatewayResponses, GetGatewayResponsesError>;

    /// <p>Get the integration settings.</p>
    fn get_integration(
        &self,
        input: GetIntegrationRequest,
    ) -> RusotoFuture<Integration, GetIntegrationError>;

    /// <p>Represents a get integration response.</p>
    fn get_integration_response(
        &self,
        input: GetIntegrationResponseRequest,
    ) -> RusotoFuture<IntegrationResponse, GetIntegrationResponseError>;

    /// <p>Describe an existing <a>Method</a> resource.</p>
    fn get_method(&self, input: GetMethodRequest) -> RusotoFuture<Method, GetMethodError>;

    /// <p>Describes a <a>MethodResponse</a> resource.</p>
    fn get_method_response(
        &self,
        input: GetMethodResponseRequest,
    ) -> RusotoFuture<MethodResponse, GetMethodResponseError>;

    /// <p>Describes an existing model defined for a <a>RestApi</a> resource.</p>
    fn get_model(&self, input: GetModelRequest) -> RusotoFuture<Model, GetModelError>;

    /// <p>Generates a sample mapping template that can be used to transform a payload into the structure of a model.</p>
    fn get_model_template(
        &self,
        input: GetModelTemplateRequest,
    ) -> RusotoFuture<Template, GetModelTemplateError>;

    /// <p>Describes existing <a>Models</a> defined for a <a>RestApi</a> resource.</p>
    fn get_models(&self, input: GetModelsRequest) -> RusotoFuture<Models, GetModelsError>;

    /// <p>Gets a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
    fn get_request_validator(
        &self,
        input: GetRequestValidatorRequest,
    ) -> RusotoFuture<RequestValidator, GetRequestValidatorError>;

    /// <p>Gets the <a>RequestValidators</a> collection of a given <a>RestApi</a>.</p>
    fn get_request_validators(
        &self,
        input: GetRequestValidatorsRequest,
    ) -> RusotoFuture<RequestValidators, GetRequestValidatorsError>;

    /// <p>Lists information about a resource.</p>
    fn get_resource(&self, input: GetResourceRequest) -> RusotoFuture<Resource, GetResourceError>;

    /// <p>Lists information about a collection of <a>Resource</a> resources.</p>
    fn get_resources(
        &self,
        input: GetResourcesRequest,
    ) -> RusotoFuture<Resources, GetResourcesError>;

    /// <p>Lists the <a>RestApi</a> resource in the collection.</p>
    fn get_rest_api(&self, input: GetRestApiRequest) -> RusotoFuture<RestApi, GetRestApiError>;

    /// <p>Lists the <a>RestApis</a> resources for your collection.</p>
    fn get_rest_apis(&self, input: GetRestApisRequest) -> RusotoFuture<RestApis, GetRestApisError>;

    /// <p>Generates a client SDK for a <a>RestApi</a> and <a>Stage</a>.</p>
    fn get_sdk(&self, input: GetSdkRequest) -> RusotoFuture<SdkResponse, GetSdkError>;

    fn get_sdk_type(&self, input: GetSdkTypeRequest) -> RusotoFuture<SdkType, GetSdkTypeError>;

    fn get_sdk_types(&self, input: GetSdkTypesRequest) -> RusotoFuture<SdkTypes, GetSdkTypesError>;

    /// <p>Gets information about a <a>Stage</a> resource.</p>
    fn get_stage(&self, input: GetStageRequest) -> RusotoFuture<Stage, GetStageError>;

    /// <p>Gets information about one or more <a>Stage</a> resources.</p>
    fn get_stages(&self, input: GetStagesRequest) -> RusotoFuture<Stages, GetStagesError>;

    /// <p>Gets the <a>Tags</a> collection for a given resource.</p>
    fn get_tags(&self, input: GetTagsRequest) -> RusotoFuture<Tags, GetTagsError>;

    /// <p>Gets the usage data of a usage plan in a specified time interval.</p>
    fn get_usage(&self, input: GetUsageRequest) -> RusotoFuture<Usage, GetUsageError>;

    /// <p>Gets a usage plan of a given plan identifier.</p>
    fn get_usage_plan(
        &self,
        input: GetUsagePlanRequest,
    ) -> RusotoFuture<UsagePlan, GetUsagePlanError>;

    /// <p>Gets a usage plan key of a given key identifier.</p>
    fn get_usage_plan_key(
        &self,
        input: GetUsagePlanKeyRequest,
    ) -> RusotoFuture<UsagePlanKey, GetUsagePlanKeyError>;

    /// <p>Gets all the usage plan keys representing the API keys added to a specified usage plan.</p>
    fn get_usage_plan_keys(
        &self,
        input: GetUsagePlanKeysRequest,
    ) -> RusotoFuture<UsagePlanKeys, GetUsagePlanKeysError>;

    /// <p>Gets all the usage plans of the caller's account.</p>
    fn get_usage_plans(
        &self,
        input: GetUsagePlansRequest,
    ) -> RusotoFuture<UsagePlans, GetUsagePlansError>;

    /// <p>Gets a specified VPC link under the caller's account in a region.</p>
    fn get_vpc_link(&self, input: GetVpcLinkRequest) -> RusotoFuture<VpcLink, GetVpcLinkError>;

    /// <p>Gets the <a>VpcLinks</a> collection under the caller's account in a selected region.</p>
    fn get_vpc_links(&self, input: GetVpcLinksRequest) -> RusotoFuture<VpcLinks, GetVpcLinksError>;

    /// <p>Import API keys from an external source, such as a CSV-formatted file.</p>
    fn import_api_keys(
        &self,
        input: ImportApiKeysRequest,
    ) -> RusotoFuture<ApiKeyIds, ImportApiKeysError>;

    fn import_documentation_parts(
        &self,
        input: ImportDocumentationPartsRequest,
    ) -> RusotoFuture<DocumentationPartIds, ImportDocumentationPartsError>;

    /// <p>A feature of the API Gateway control service for creating a new API from an external API definition file.</p>
    fn import_rest_api(
        &self,
        input: ImportRestApiRequest,
    ) -> RusotoFuture<RestApi, ImportRestApiError>;

    /// <p>Creates a customization of a <a>GatewayResponse</a> of a specified response type and status code on the given <a>RestApi</a>.</p>
    fn put_gateway_response(
        &self,
        input: PutGatewayResponseRequest,
    ) -> RusotoFuture<GatewayResponse, PutGatewayResponseError>;

    /// <p>Sets up a method's integration.</p>
    fn put_integration(
        &self,
        input: PutIntegrationRequest,
    ) -> RusotoFuture<Integration, PutIntegrationError>;

    /// <p>Represents a put integration.</p>
    fn put_integration_response(
        &self,
        input: PutIntegrationResponseRequest,
    ) -> RusotoFuture<IntegrationResponse, PutIntegrationResponseError>;

    /// <p>Add a method to an existing <a>Resource</a> resource.</p>
    fn put_method(&self, input: PutMethodRequest) -> RusotoFuture<Method, PutMethodError>;

    /// <p>Adds a <a>MethodResponse</a> to an existing <a>Method</a> resource.</p>
    fn put_method_response(
        &self,
        input: PutMethodResponseRequest,
    ) -> RusotoFuture<MethodResponse, PutMethodResponseError>;

    /// <p>A feature of the API Gateway control service for updating an existing API with an input of external API definitions. The update can take the form of merging the supplied definition into the existing API or overwriting the existing API.</p>
    fn put_rest_api(&self, input: PutRestApiRequest) -> RusotoFuture<RestApi, PutRestApiError>;

    /// <p>Adds or updates a tag on a given resource.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError>;

    /// <p><p>Simulate the execution of an <a>Authorizer</a> in your <a>RestApi</a> with headers, parameters, and an incoming request body.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/use-custom-authorizer.html">Enable custom authorizers</a> </div></p>
    fn test_invoke_authorizer(
        &self,
        input: TestInvokeAuthorizerRequest,
    ) -> RusotoFuture<TestInvokeAuthorizerResponse, TestInvokeAuthorizerError>;

    /// <p>Simulate the execution of a <a>Method</a> in your <a>RestApi</a> with headers, parameters, and an incoming request body.</p>
    fn test_invoke_method(
        &self,
        input: TestInvokeMethodRequest,
    ) -> RusotoFuture<TestInvokeMethodResponse, TestInvokeMethodError>;

    /// <p>Removes a tag from a given resource.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError>;

    /// <p>Changes information about the current <a>Account</a> resource.</p>
    fn update_account(
        &self,
        input: UpdateAccountRequest,
    ) -> RusotoFuture<Account, UpdateAccountError>;

    /// <p>Changes information about an <a>ApiKey</a> resource.</p>
    fn update_api_key(&self, input: UpdateApiKeyRequest)
        -> RusotoFuture<ApiKey, UpdateApiKeyError>;

    /// <p><p>Updates an existing <a>Authorizer</a> resource.</p> <div class="seeAlso"><a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/update-authorizer.html">AWS CLI</a></div></p>
    fn update_authorizer(
        &self,
        input: UpdateAuthorizerRequest,
    ) -> RusotoFuture<Authorizer, UpdateAuthorizerError>;

    /// <p>Changes information about the <a>BasePathMapping</a> resource.</p>
    fn update_base_path_mapping(
        &self,
        input: UpdateBasePathMappingRequest,
    ) -> RusotoFuture<BasePathMapping, UpdateBasePathMappingError>;

    /// <p>Changes information about an <a>ClientCertificate</a> resource.</p>
    fn update_client_certificate(
        &self,
        input: UpdateClientCertificateRequest,
    ) -> RusotoFuture<ClientCertificate, UpdateClientCertificateError>;

    /// <p>Changes information about a <a>Deployment</a> resource.</p>
    fn update_deployment(
        &self,
        input: UpdateDeploymentRequest,
    ) -> RusotoFuture<Deployment, UpdateDeploymentError>;

    fn update_documentation_part(
        &self,
        input: UpdateDocumentationPartRequest,
    ) -> RusotoFuture<DocumentationPart, UpdateDocumentationPartError>;

    fn update_documentation_version(
        &self,
        input: UpdateDocumentationVersionRequest,
    ) -> RusotoFuture<DocumentationVersion, UpdateDocumentationVersionError>;

    /// <p>Changes information about the <a>DomainName</a> resource.</p>
    fn update_domain_name(
        &self,
        input: UpdateDomainNameRequest,
    ) -> RusotoFuture<DomainName, UpdateDomainNameError>;

    /// <p>Updates a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a>.</p>
    fn update_gateway_response(
        &self,
        input: UpdateGatewayResponseRequest,
    ) -> RusotoFuture<GatewayResponse, UpdateGatewayResponseError>;

    /// <p>Represents an update integration.</p>
    fn update_integration(
        &self,
        input: UpdateIntegrationRequest,
    ) -> RusotoFuture<Integration, UpdateIntegrationError>;

    /// <p>Represents an update integration response.</p>
    fn update_integration_response(
        &self,
        input: UpdateIntegrationResponseRequest,
    ) -> RusotoFuture<IntegrationResponse, UpdateIntegrationResponseError>;

    /// <p>Updates an existing <a>Method</a> resource.</p>
    fn update_method(&self, input: UpdateMethodRequest) -> RusotoFuture<Method, UpdateMethodError>;

    /// <p>Updates an existing <a>MethodResponse</a> resource.</p>
    fn update_method_response(
        &self,
        input: UpdateMethodResponseRequest,
    ) -> RusotoFuture<MethodResponse, UpdateMethodResponseError>;

    /// <p>Changes information about a model.</p>
    fn update_model(&self, input: UpdateModelRequest) -> RusotoFuture<Model, UpdateModelError>;

    /// <p>Updates a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
    fn update_request_validator(
        &self,
        input: UpdateRequestValidatorRequest,
    ) -> RusotoFuture<RequestValidator, UpdateRequestValidatorError>;

    /// <p>Changes information about a <a>Resource</a> resource.</p>
    fn update_resource(
        &self,
        input: UpdateResourceRequest,
    ) -> RusotoFuture<Resource, UpdateResourceError>;

    /// <p>Changes information about the specified API.</p>
    fn update_rest_api(
        &self,
        input: UpdateRestApiRequest,
    ) -> RusotoFuture<RestApi, UpdateRestApiError>;

    /// <p>Changes information about a <a>Stage</a> resource.</p>
    fn update_stage(&self, input: UpdateStageRequest) -> RusotoFuture<Stage, UpdateStageError>;

    /// <p>Grants a temporary extension to the remaining quota of a usage plan associated with a specified API key.</p>
    fn update_usage(&self, input: UpdateUsageRequest) -> RusotoFuture<Usage, UpdateUsageError>;

    /// <p>Updates a usage plan of a given plan Id.</p>
    fn update_usage_plan(
        &self,
        input: UpdateUsagePlanRequest,
    ) -> RusotoFuture<UsagePlan, UpdateUsagePlanError>;

    /// <p>Updates an existing <a>VpcLink</a> of a specified identifier.</p>
    fn update_vpc_link(
        &self,
        input: UpdateVpcLinkRequest,
    ) -> RusotoFuture<VpcLink, UpdateVpcLinkError>;
}
/// A client for the Amazon API Gateway API.
pub struct ApiGatewayClient {
    client: Client,
    region: region::Region,
}

impl ApiGatewayClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ApiGatewayClient {
        ApiGatewayClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ApiGatewayClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ApiGatewayClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl ApiGateway for ApiGatewayClient {
    /// <p><p>Create an <a>ApiKey</a> resource. </p> <div class="seeAlso"><a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/create-api-key.html">AWS CLI</a></div></p>
    fn create_api_key(
        &self,
        input: CreateApiKeyRequest,
    ) -> RusotoFuture<ApiKey, CreateApiKeyError> {
        let request_uri = "/apikeys";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ApiKey>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateApiKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p><p>Adds a new <a>Authorizer</a> resource to an existing <a>RestApi</a> resource.</p> <div class="seeAlso"><a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/create-authorizer.html">AWS CLI</a></div></p>
    fn create_authorizer(
        &self,
        input: CreateAuthorizerRequest,
    ) -> RusotoFuture<Authorizer, CreateAuthorizerError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/authorizers",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Authorizer>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateAuthorizerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a new <a>BasePathMapping</a> resource.</p>
    fn create_base_path_mapping(
        &self,
        input: CreateBasePathMappingRequest,
    ) -> RusotoFuture<BasePathMapping, CreateBasePathMappingError> {
        let request_uri = format!(
            "/domainnames/{domain_name}/basepathmappings",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<BasePathMapping>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateBasePathMappingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a <a>Deployment</a> resource, which makes a specified <a>RestApi</a> callable over the internet.</p>
    fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> RusotoFuture<Deployment, CreateDeploymentError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/deployments",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Deployment>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDeploymentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    fn create_documentation_part(
        &self,
        input: CreateDocumentationPartRequest,
    ) -> RusotoFuture<DocumentationPart, CreateDocumentationPartError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/parts",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DocumentationPart>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDocumentationPartError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    fn create_documentation_version(
        &self,
        input: CreateDocumentationVersionRequest,
    ) -> RusotoFuture<DocumentationVersion, CreateDocumentationVersionError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/versions",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DocumentationVersion>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDocumentationVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a new domain name.</p>
    fn create_domain_name(
        &self,
        input: CreateDomainNameRequest,
    ) -> RusotoFuture<DomainName, CreateDomainNameError> {
        let request_uri = "/domainnames";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DomainName>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDomainNameError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Adds a new <a>Model</a> resource to an existing <a>RestApi</a> resource.</p>
    fn create_model(&self, input: CreateModelRequest) -> RusotoFuture<Model, CreateModelError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/models",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Model>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateModelError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a <a>ReqeustValidator</a> of a given <a>RestApi</a>.</p>
    fn create_request_validator(
        &self,
        input: CreateRequestValidatorRequest,
    ) -> RusotoFuture<RequestValidator, CreateRequestValidatorError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/requestvalidators",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<RequestValidator>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateRequestValidatorError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a <a>Resource</a> resource.</p>
    fn create_resource(
        &self,
        input: CreateResourceRequest,
    ) -> RusotoFuture<Resource, CreateResourceError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{parent_id}",
            parent_id = input.parent_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Resource>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a new <a>RestApi</a> resource.</p>
    fn create_rest_api(
        &self,
        input: CreateRestApiRequest,
    ) -> RusotoFuture<RestApi, CreateRestApiError> {
        let request_uri = "/restapis";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<RestApi>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateRestApiError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a new <a>Stage</a> resource that references a pre-existing <a>Deployment</a> for the API. </p>
    fn create_stage(&self, input: CreateStageRequest) -> RusotoFuture<Stage, CreateStageError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Stage>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateStageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a usage plan with the throttle and quota limits, as well as the associated API stages, specified in the payload. </p>
    fn create_usage_plan(
        &self,
        input: CreateUsagePlanRequest,
    ) -> RusotoFuture<UsagePlan, CreateUsagePlanError> {
        let request_uri = "/usageplans";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UsagePlan>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateUsagePlanError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a usage plan key for adding an existing API key to a usage plan.</p>
    fn create_usage_plan_key(
        &self,
        input: CreateUsagePlanKeyRequest,
    ) -> RusotoFuture<UsagePlanKey, CreateUsagePlanKeyError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}/keys",
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UsagePlanKey>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateUsagePlanKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a VPC link, under the caller's account in a selected region, in an asynchronous operation that typically takes 2-4 minutes to complete and become operational. The caller must have permissions to create and update VPC Endpoint services.</p>
    fn create_vpc_link(
        &self,
        input: CreateVpcLinkRequest,
    ) -> RusotoFuture<VpcLink, CreateVpcLinkError> {
        let request_uri = "/vpclinks";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<VpcLink>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateVpcLinkError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the <a>ApiKey</a> resource.</p>
    fn delete_api_key(&self, input: DeleteApiKeyRequest) -> RusotoFuture<(), DeleteApiKeyError> {
        let request_uri = format!("/apikeys/{api_key}", api_key = input.api_key);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApiKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p><p>Deletes an existing <a>Authorizer</a> resource.</p> <div class="seeAlso"><a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/delete-authorizer.html">AWS CLI</a></div></p>
    fn delete_authorizer(
        &self,
        input: DeleteAuthorizerRequest,
    ) -> RusotoFuture<(), DeleteAuthorizerError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/authorizers/{authorizer_id}",
            authorizer_id = input.authorizer_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAuthorizerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the <a>BasePathMapping</a> resource.</p>
    fn delete_base_path_mapping(
        &self,
        input: DeleteBasePathMappingRequest,
    ) -> RusotoFuture<(), DeleteBasePathMappingError> {
        let request_uri = format!(
            "/domainnames/{domain_name}/basepathmappings/{base_path}",
            base_path = input.base_path,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBasePathMappingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the <a>ClientCertificate</a> resource.</p>
    fn delete_client_certificate(
        &self,
        input: DeleteClientCertificateRequest,
    ) -> RusotoFuture<(), DeleteClientCertificateError> {
        let request_uri = format!(
            "/clientcertificates/{clientcertificate_id}",
            clientcertificate_id = input.client_certificate_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteClientCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a <a>Deployment</a> resource. Deleting a deployment will only succeed if there are no <a>Stage</a> resources associated with it.</p>
    fn delete_deployment(
        &self,
        input: DeleteDeploymentRequest,
    ) -> RusotoFuture<(), DeleteDeploymentError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/deployments/{deployment_id}",
            deployment_id = input.deployment_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDeploymentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    fn delete_documentation_part(
        &self,
        input: DeleteDocumentationPartRequest,
    ) -> RusotoFuture<(), DeleteDocumentationPartError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/parts/{part_id}",
            part_id = input.documentation_part_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDocumentationPartError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    fn delete_documentation_version(
        &self,
        input: DeleteDocumentationVersionRequest,
    ) -> RusotoFuture<(), DeleteDocumentationVersionError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/versions/{doc_version}",
            doc_version = input.documentation_version,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDocumentationVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the <a>DomainName</a> resource.</p>
    fn delete_domain_name(
        &self,
        input: DeleteDomainNameRequest,
    ) -> RusotoFuture<(), DeleteDomainNameError> {
        let request_uri = format!(
            "/domainnames/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDomainNameError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Clears any customization of a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a> and resets it with the default settings.</p>
    fn delete_gateway_response(
        &self,
        input: DeleteGatewayResponseRequest,
    ) -> RusotoFuture<(), DeleteGatewayResponseError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/gatewayresponses/{response_type}",
            response_type = input.response_type,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteGatewayResponseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Represents a delete integration.</p>
    fn delete_integration(
        &self,
        input: DeleteIntegrationRequest,
    ) -> RusotoFuture<(), DeleteIntegrationError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteIntegrationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Represents a delete integration response.</p>
    fn delete_integration_response(
        &self,
        input: DeleteIntegrationResponseRequest,
    ) -> RusotoFuture<(), DeleteIntegrationResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteIntegrationResponseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes an existing <a>Method</a> resource.</p>
    fn delete_method(&self, input: DeleteMethodRequest) -> RusotoFuture<(), DeleteMethodError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteMethodError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes an existing <a>MethodResponse</a> resource.</p>
    fn delete_method_response(
        &self,
        input: DeleteMethodResponseRequest,
    ) -> RusotoFuture<(), DeleteMethodResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteMethodResponseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a model.</p>
    fn delete_model(&self, input: DeleteModelRequest) -> RusotoFuture<(), DeleteModelError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/models/{model_name}",
            model_name = input.model_name,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteModelError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
    fn delete_request_validator(
        &self,
        input: DeleteRequestValidatorRequest,
    ) -> RusotoFuture<(), DeleteRequestValidatorError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/requestvalidators/{requestvalidator_id}",
            requestvalidator_id = input.request_validator_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRequestValidatorError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a <a>Resource</a> resource.</p>
    fn delete_resource(
        &self,
        input: DeleteResourceRequest,
    ) -> RusotoFuture<(), DeleteResourceError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}",
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the specified API.</p>
    fn delete_rest_api(&self, input: DeleteRestApiRequest) -> RusotoFuture<(), DeleteRestApiError> {
        let request_uri = format!("/restapis/{restapi_id}", restapi_id = input.rest_api_id);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRestApiError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a <a>Stage</a> resource.</p>
    fn delete_stage(&self, input: DeleteStageRequest) -> RusotoFuture<(), DeleteStageError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}",
            restapi_id = input.rest_api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteStageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a usage plan of a given plan Id.</p>
    fn delete_usage_plan(
        &self,
        input: DeleteUsagePlanRequest,
    ) -> RusotoFuture<(), DeleteUsagePlanError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}",
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteUsagePlanError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a usage plan key and remove the underlying API key from the associated usage plan.</p>
    fn delete_usage_plan_key(
        &self,
        input: DeleteUsagePlanKeyRequest,
    ) -> RusotoFuture<(), DeleteUsagePlanKeyError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}/keys/{key_id}",
            key_id = input.key_id,
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteUsagePlanKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes an existing <a>VpcLink</a> of a specified identifier.</p>
    fn delete_vpc_link(&self, input: DeleteVpcLinkRequest) -> RusotoFuture<(), DeleteVpcLinkError> {
        let request_uri = format!("/vpclinks/{vpclink_id}", vpclink_id = input.vpc_link_id);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVpcLinkError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Flushes all authorizer cache entries on a stage.</p>
    fn flush_stage_authorizers_cache(
        &self,
        input: FlushStageAuthorizersCacheRequest,
    ) -> RusotoFuture<(), FlushStageAuthorizersCacheError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}/cache/authorizers",
            restapi_id = input.rest_api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(FlushStageAuthorizersCacheError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Flushes a stage's cache.</p>
    fn flush_stage_cache(
        &self,
        input: FlushStageCacheRequest,
    ) -> RusotoFuture<(), FlushStageCacheError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}/cache/data",
            restapi_id = input.rest_api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(FlushStageCacheError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Generates a <a>ClientCertificate</a> resource.</p>
    fn generate_client_certificate(
        &self,
        input: GenerateClientCertificateRequest,
    ) -> RusotoFuture<ClientCertificate, GenerateClientCertificateError> {
        let request_uri = "/clientcertificates";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ClientCertificate>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GenerateClientCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about the current <a>Account</a> resource.</p>
    fn get_account(&self) -> RusotoFuture<Account, GetAccountError> {
        let request_uri = "/account";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Account>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAccountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about the current <a>ApiKey</a> resource.</p>
    fn get_api_key(&self, input: GetApiKeyRequest) -> RusotoFuture<ApiKey, GetApiKeyError> {
        let request_uri = format!("/apikeys/{api_key}", api_key = input.api_key);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.include_value {
            params.put("includeValue", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ApiKey>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetApiKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about the current <a>ApiKeys</a> resource.</p>
    fn get_api_keys(&self, input: GetApiKeysRequest) -> RusotoFuture<ApiKeys, GetApiKeysError> {
        let request_uri = "/apikeys";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.customer_id {
            params.put("customerId", x);
        }
        if let Some(ref x) = input.include_values {
            params.put("includeValues", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.name_query {
            params.put("name", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ApiKeys>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetApiKeysError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p><p>Describe an existing <a>Authorizer</a> resource.</p> <div class="seeAlso"><a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/get-authorizer.html">AWS CLI</a></div></p>
    fn get_authorizer(
        &self,
        input: GetAuthorizerRequest,
    ) -> RusotoFuture<Authorizer, GetAuthorizerError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/authorizers/{authorizer_id}",
            authorizer_id = input.authorizer_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Authorizer>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAuthorizerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p><p>Describe an existing <a>Authorizers</a> resource.</p> <div class="seeAlso"><a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/get-authorizers.html">AWS CLI</a></div></p>
    fn get_authorizers(
        &self,
        input: GetAuthorizersRequest,
    ) -> RusotoFuture<Authorizers, GetAuthorizersError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/authorizers",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Authorizers>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAuthorizersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Describe a <a>BasePathMapping</a> resource.</p>
    fn get_base_path_mapping(
        &self,
        input: GetBasePathMappingRequest,
    ) -> RusotoFuture<BasePathMapping, GetBasePathMappingError> {
        let request_uri = format!(
            "/domainnames/{domain_name}/basepathmappings/{base_path}",
            base_path = input.base_path,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<BasePathMapping>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBasePathMappingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Represents a collection of <a>BasePathMapping</a> resources.</p>
    fn get_base_path_mappings(
        &self,
        input: GetBasePathMappingsRequest,
    ) -> RusotoFuture<BasePathMappings, GetBasePathMappingsError> {
        let request_uri = format!(
            "/domainnames/{domain_name}/basepathmappings",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<BasePathMappings>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBasePathMappingsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about the current <a>ClientCertificate</a> resource.</p>
    fn get_client_certificate(
        &self,
        input: GetClientCertificateRequest,
    ) -> RusotoFuture<ClientCertificate, GetClientCertificateError> {
        let request_uri = format!(
            "/clientcertificates/{clientcertificate_id}",
            clientcertificate_id = input.client_certificate_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ClientCertificate>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetClientCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets a collection of <a>ClientCertificate</a> resources.</p>
    fn get_client_certificates(
        &self,
        input: GetClientCertificatesRequest,
    ) -> RusotoFuture<ClientCertificates, GetClientCertificatesError> {
        let request_uri = "/clientcertificates";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ClientCertificates>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetClientCertificatesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about a <a>Deployment</a> resource.</p>
    fn get_deployment(
        &self,
        input: GetDeploymentRequest,
    ) -> RusotoFuture<Deployment, GetDeploymentError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/deployments/{deployment_id}",
            deployment_id = input.deployment_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.embed {
            for item in x.iter() {
                params.put("embed", item);
            }
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Deployment>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDeploymentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about a <a>Deployments</a> collection.</p>
    fn get_deployments(
        &self,
        input: GetDeploymentsRequest,
    ) -> RusotoFuture<Deployments, GetDeploymentsError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/deployments",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Deployments>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDeploymentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    fn get_documentation_part(
        &self,
        input: GetDocumentationPartRequest,
    ) -> RusotoFuture<DocumentationPart, GetDocumentationPartError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/parts/{part_id}",
            part_id = input.documentation_part_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DocumentationPart>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDocumentationPartError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    fn get_documentation_parts(
        &self,
        input: GetDocumentationPartsRequest,
    ) -> RusotoFuture<DocumentationParts, GetDocumentationPartsError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/parts",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.location_status {
            params.put("locationStatus", x);
        }
        if let Some(ref x) = input.name_query {
            params.put("name", x);
        }
        if let Some(ref x) = input.path {
            params.put("path", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        if let Some(ref x) = input.type_ {
            params.put("type", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DocumentationParts>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDocumentationPartsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    fn get_documentation_version(
        &self,
        input: GetDocumentationVersionRequest,
    ) -> RusotoFuture<DocumentationVersion, GetDocumentationVersionError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/versions/{doc_version}",
            doc_version = input.documentation_version,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DocumentationVersion>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDocumentationVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    fn get_documentation_versions(
        &self,
        input: GetDocumentationVersionsRequest,
    ) -> RusotoFuture<DocumentationVersions, GetDocumentationVersionsError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/versions",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DocumentationVersions>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDocumentationVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Represents a domain name that is contained in a simpler, more intuitive URL that can be called.</p>
    fn get_domain_name(
        &self,
        input: GetDomainNameRequest,
    ) -> RusotoFuture<DomainName, GetDomainNameError> {
        let request_uri = format!(
            "/domainnames/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DomainName>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDomainNameError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Represents a collection of <a>DomainName</a> resources.</p>
    fn get_domain_names(
        &self,
        input: GetDomainNamesRequest,
    ) -> RusotoFuture<DomainNames, GetDomainNamesError> {
        let request_uri = "/domainnames";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DomainNames>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDomainNamesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Exports a deployed version of a <a>RestApi</a> in a specified format.</p>
    fn get_export(&self, input: GetExportRequest) -> RusotoFuture<ExportResponse, GetExportError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}/exports/{export_type}",
            export_type = input.export_type,
            restapi_id = input.rest_api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref accepts) = input.accepts {
            request.add_header("Accept", &accepts.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.parameters {
            for (key, val) in x.iter() {
                params.put(key, val);
            }
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut result = ExportResponse::default();
                    result.body = Some(response.body);

                    if let Some(content_disposition) = response.headers.get("Content-Disposition") {
                        let value = content_disposition.to_owned();
                        result.content_disposition = Some(value)
                    };
                    if let Some(content_type) = response.headers.get("Content-Type") {
                        let value = content_type.to_owned();
                        result.content_type = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetExportError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a>.</p>
    fn get_gateway_response(
        &self,
        input: GetGatewayResponseRequest,
    ) -> RusotoFuture<GatewayResponse, GetGatewayResponseError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/gatewayresponses/{response_type}",
            response_type = input.response_type,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GatewayResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetGatewayResponseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets the <a>GatewayResponses</a> collection on the given <a>RestApi</a>. If an API developer has not added any definitions for gateway responses, the result will be the API Gateway-generated default <a>GatewayResponses</a> collection for the supported response types.</p>
    fn get_gateway_responses(
        &self,
        input: GetGatewayResponsesRequest,
    ) -> RusotoFuture<GatewayResponses, GetGatewayResponsesError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/gatewayresponses",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GatewayResponses>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetGatewayResponsesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Get the integration settings.</p>
    fn get_integration(
        &self,
        input: GetIntegrationRequest,
    ) -> RusotoFuture<Integration, GetIntegrationError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Integration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetIntegrationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Represents a get integration response.</p>
    fn get_integration_response(
        &self,
        input: GetIntegrationResponseRequest,
    ) -> RusotoFuture<IntegrationResponse, GetIntegrationResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<IntegrationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetIntegrationResponseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Describe an existing <a>Method</a> resource.</p>
    fn get_method(&self, input: GetMethodRequest) -> RusotoFuture<Method, GetMethodError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Method>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetMethodError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Describes a <a>MethodResponse</a> resource.</p>
    fn get_method_response(
        &self,
        input: GetMethodResponseRequest,
    ) -> RusotoFuture<MethodResponse, GetMethodResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<MethodResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetMethodResponseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Describes an existing model defined for a <a>RestApi</a> resource.</p>
    fn get_model(&self, input: GetModelRequest) -> RusotoFuture<Model, GetModelError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/models/{model_name}",
            model_name = input.model_name,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.flatten {
            params.put("flatten", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Model>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetModelError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Generates a sample mapping template that can be used to transform a payload into the structure of a model.</p>
    fn get_model_template(
        &self,
        input: GetModelTemplateRequest,
    ) -> RusotoFuture<Template, GetModelTemplateError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/models/{model_name}/default_template",
            model_name = input.model_name,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Template>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetModelTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Describes existing <a>Models</a> defined for a <a>RestApi</a> resource.</p>
    fn get_models(&self, input: GetModelsRequest) -> RusotoFuture<Models, GetModelsError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/models",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Models>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetModelsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
    fn get_request_validator(
        &self,
        input: GetRequestValidatorRequest,
    ) -> RusotoFuture<RequestValidator, GetRequestValidatorError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/requestvalidators/{requestvalidator_id}",
            requestvalidator_id = input.request_validator_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<RequestValidator>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRequestValidatorError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets the <a>RequestValidators</a> collection of a given <a>RestApi</a>.</p>
    fn get_request_validators(
        &self,
        input: GetRequestValidatorsRequest,
    ) -> RusotoFuture<RequestValidators, GetRequestValidatorsError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/requestvalidators",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<RequestValidators>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRequestValidatorsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists information about a resource.</p>
    fn get_resource(&self, input: GetResourceRequest) -> RusotoFuture<Resource, GetResourceError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}",
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.embed {
            for item in x.iter() {
                params.put("embed", item);
            }
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Resource>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists information about a collection of <a>Resource</a> resources.</p>
    fn get_resources(
        &self,
        input: GetResourcesRequest,
    ) -> RusotoFuture<Resources, GetResourcesError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.embed {
            for item in x.iter() {
                params.put("embed", item);
            }
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Resources>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetResourcesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists the <a>RestApi</a> resource in the collection.</p>
    fn get_rest_api(&self, input: GetRestApiRequest) -> RusotoFuture<RestApi, GetRestApiError> {
        let request_uri = format!("/restapis/{restapi_id}", restapi_id = input.rest_api_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<RestApi>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRestApiError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists the <a>RestApis</a> resources for your collection.</p>
    fn get_rest_apis(&self, input: GetRestApisRequest) -> RusotoFuture<RestApis, GetRestApisError> {
        let request_uri = "/restapis";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<RestApis>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRestApisError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Generates a client SDK for a <a>RestApi</a> and <a>Stage</a>.</p>
    fn get_sdk(&self, input: GetSdkRequest) -> RusotoFuture<SdkResponse, GetSdkError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}/sdks/{sdk_type}",
            restapi_id = input.rest_api_id,
            sdk_type = input.sdk_type,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.parameters {
            for (key, val) in x.iter() {
                params.put(key, val);
            }
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut result = SdkResponse::default();
                    result.body = Some(response.body);

                    if let Some(content_disposition) = response.headers.get("Content-Disposition") {
                        let value = content_disposition.to_owned();
                        result.content_disposition = Some(value)
                    };
                    if let Some(content_type) = response.headers.get("Content-Type") {
                        let value = content_type.to_owned();
                        result.content_type = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSdkError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    fn get_sdk_type(&self, input: GetSdkTypeRequest) -> RusotoFuture<SdkType, GetSdkTypeError> {
        let request_uri = format!("/sdktypes/{sdktype_id}", sdktype_id = input.id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<SdkType>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSdkTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    fn get_sdk_types(&self, input: GetSdkTypesRequest) -> RusotoFuture<SdkTypes, GetSdkTypesError> {
        let request_uri = "/sdktypes";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<SdkTypes>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSdkTypesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about a <a>Stage</a> resource.</p>
    fn get_stage(&self, input: GetStageRequest) -> RusotoFuture<Stage, GetStageError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}",
            restapi_id = input.rest_api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Stage>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetStageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about one or more <a>Stage</a> resources.</p>
    fn get_stages(&self, input: GetStagesRequest) -> RusotoFuture<Stages, GetStagesError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.deployment_id {
            params.put("deploymentId", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Stages>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetStagesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets the <a>Tags</a> collection for a given resource.</p>
    fn get_tags(&self, input: GetTagsRequest) -> RusotoFuture<Tags, GetTagsError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Tags>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets the usage data of a usage plan in a specified time interval.</p>
    fn get_usage(&self, input: GetUsageRequest) -> RusotoFuture<Usage, GetUsageError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}/usage",
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("endDate", &input.end_date);
        if let Some(ref x) = input.key_id {
            params.put("keyId", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        params.put("startDate", &input.start_date);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Usage>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetUsageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets a usage plan of a given plan identifier.</p>
    fn get_usage_plan(
        &self,
        input: GetUsagePlanRequest,
    ) -> RusotoFuture<UsagePlan, GetUsagePlanError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}",
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UsagePlan>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetUsagePlanError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets a usage plan key of a given key identifier.</p>
    fn get_usage_plan_key(
        &self,
        input: GetUsagePlanKeyRequest,
    ) -> RusotoFuture<UsagePlanKey, GetUsagePlanKeyError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}/keys/{key_id}",
            key_id = input.key_id,
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UsagePlanKey>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetUsagePlanKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets all the usage plan keys representing the API keys added to a specified usage plan.</p>
    fn get_usage_plan_keys(
        &self,
        input: GetUsagePlanKeysRequest,
    ) -> RusotoFuture<UsagePlanKeys, GetUsagePlanKeysError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}/keys",
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.name_query {
            params.put("name", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UsagePlanKeys>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetUsagePlanKeysError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets all the usage plans of the caller's account.</p>
    fn get_usage_plans(
        &self,
        input: GetUsagePlansRequest,
    ) -> RusotoFuture<UsagePlans, GetUsagePlansError> {
        let request_uri = "/usageplans";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.key_id {
            params.put("keyId", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UsagePlans>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetUsagePlansError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets a specified VPC link under the caller's account in a region.</p>
    fn get_vpc_link(&self, input: GetVpcLinkRequest) -> RusotoFuture<VpcLink, GetVpcLinkError> {
        let request_uri = format!("/vpclinks/{vpclink_id}", vpclink_id = input.vpc_link_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<VpcLink>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetVpcLinkError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets the <a>VpcLinks</a> collection under the caller's account in a selected region.</p>
    fn get_vpc_links(&self, input: GetVpcLinksRequest) -> RusotoFuture<VpcLinks, GetVpcLinksError> {
        let request_uri = "/vpclinks";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<VpcLinks>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetVpcLinksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Import API keys from an external source, such as a CSV-formatted file.</p>
    fn import_api_keys(
        &self,
        input: ImportApiKeysRequest,
    ) -> RusotoFuture<ApiKeyIds, ImportApiKeysError> {
        let request_uri = "/apikeys";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.body.to_owned());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.fail_on_warnings {
            params.put("failonwarnings", x);
        }
        params.put("format", &input.format);
        params.put("mode", "import");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ApiKeyIds>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ImportApiKeysError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    fn import_documentation_parts(
        &self,
        input: ImportDocumentationPartsRequest,
    ) -> RusotoFuture<DocumentationPartIds, ImportDocumentationPartsError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/parts",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.body.to_owned());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.fail_on_warnings {
            params.put("failonwarnings", x);
        }
        if let Some(ref x) = input.mode {
            params.put("mode", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DocumentationPartIds>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ImportDocumentationPartsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>A feature of the API Gateway control service for creating a new API from an external API definition file.</p>
    fn import_rest_api(
        &self,
        input: ImportRestApiRequest,
    ) -> RusotoFuture<RestApi, ImportRestApiError> {
        let request_uri = "/restapis";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.body.to_owned());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.fail_on_warnings {
            params.put("failonwarnings", x);
        }
        if let Some(ref x) = input.parameters {
            for (key, val) in x.iter() {
                params.put(key, val);
            }
        }
        params.put("mode", "import");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<RestApi>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ImportRestApiError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a customization of a <a>GatewayResponse</a> of a specified response type and status code on the given <a>RestApi</a>.</p>
    fn put_gateway_response(
        &self,
        input: PutGatewayResponseRequest,
    ) -> RusotoFuture<GatewayResponse, PutGatewayResponseError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/gatewayresponses/{response_type}",
            response_type = input.response_type,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GatewayResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutGatewayResponseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Sets up a method's integration.</p>
    fn put_integration(
        &self,
        input: PutIntegrationRequest,
    ) -> RusotoFuture<Integration, PutIntegrationError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Integration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutIntegrationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Represents a put integration.</p>
    fn put_integration_response(
        &self,
        input: PutIntegrationResponseRequest,
    ) -> RusotoFuture<IntegrationResponse, PutIntegrationResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<IntegrationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutIntegrationResponseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Add a method to an existing <a>Resource</a> resource.</p>
    fn put_method(&self, input: PutMethodRequest) -> RusotoFuture<Method, PutMethodError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Method>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutMethodError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Adds a <a>MethodResponse</a> to an existing <a>Method</a> resource.</p>
    fn put_method_response(
        &self,
        input: PutMethodResponseRequest,
    ) -> RusotoFuture<MethodResponse, PutMethodResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<MethodResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutMethodResponseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>A feature of the API Gateway control service for updating an existing API with an input of external API definitions. The update can take the form of merging the supplied definition into the existing API or overwriting the existing API.</p>
    fn put_rest_api(&self, input: PutRestApiRequest) -> RusotoFuture<RestApi, PutRestApiError> {
        let request_uri = format!("/restapis/{restapi_id}", restapi_id = input.rest_api_id);

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.body.to_owned());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.fail_on_warnings {
            params.put("failonwarnings", x);
        }
        if let Some(ref x) = input.mode {
            params.put("mode", x);
        }
        if let Some(ref x) = input.parameters {
            for (key, val) in x.iter() {
                params.put(key, val);
            }
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<RestApi>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutRestApiError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Adds or updates a tag on a given resource.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(TagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p><p>Simulate the execution of an <a>Authorizer</a> in your <a>RestApi</a> with headers, parameters, and an incoming request body.</p> <div class="seeAlso"> <a href="http://docs.aws.amazon.com/apigateway/latest/developerguide/use-custom-authorizer.html">Enable custom authorizers</a> </div></p>
    fn test_invoke_authorizer(
        &self,
        input: TestInvokeAuthorizerRequest,
    ) -> RusotoFuture<TestInvokeAuthorizerResponse, TestInvokeAuthorizerError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/authorizers/{authorizer_id}",
            authorizer_id = input.authorizer_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<TestInvokeAuthorizerResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(TestInvokeAuthorizerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Simulate the execution of a <a>Method</a> in your <a>RestApi</a> with headers, parameters, and an incoming request body.</p>
    fn test_invoke_method(
        &self,
        input: TestInvokeMethodRequest,
    ) -> RusotoFuture<TestInvokeMethodResponse, TestInvokeMethodError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<TestInvokeMethodResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(TestInvokeMethodError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Removes a tag from a given resource.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UntagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Changes information about the current <a>Account</a> resource.</p>
    fn update_account(
        &self,
        input: UpdateAccountRequest,
    ) -> RusotoFuture<Account, UpdateAccountError> {
        let request_uri = "/account";

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Account>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAccountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Changes information about an <a>ApiKey</a> resource.</p>
    fn update_api_key(
        &self,
        input: UpdateApiKeyRequest,
    ) -> RusotoFuture<ApiKey, UpdateApiKeyError> {
        let request_uri = format!("/apikeys/{api_key}", api_key = input.api_key);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ApiKey>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateApiKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p><p>Updates an existing <a>Authorizer</a> resource.</p> <div class="seeAlso"><a href="http://docs.aws.amazon.com/cli/latest/reference/apigateway/update-authorizer.html">AWS CLI</a></div></p>
    fn update_authorizer(
        &self,
        input: UpdateAuthorizerRequest,
    ) -> RusotoFuture<Authorizer, UpdateAuthorizerError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/authorizers/{authorizer_id}",
            authorizer_id = input.authorizer_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Authorizer>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAuthorizerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Changes information about the <a>BasePathMapping</a> resource.</p>
    fn update_base_path_mapping(
        &self,
        input: UpdateBasePathMappingRequest,
    ) -> RusotoFuture<BasePathMapping, UpdateBasePathMappingError> {
        let request_uri = format!(
            "/domainnames/{domain_name}/basepathmappings/{base_path}",
            base_path = input.base_path,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<BasePathMapping>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateBasePathMappingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Changes information about an <a>ClientCertificate</a> resource.</p>
    fn update_client_certificate(
        &self,
        input: UpdateClientCertificateRequest,
    ) -> RusotoFuture<ClientCertificate, UpdateClientCertificateError> {
        let request_uri = format!(
            "/clientcertificates/{clientcertificate_id}",
            clientcertificate_id = input.client_certificate_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ClientCertificate>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateClientCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Changes information about a <a>Deployment</a> resource.</p>
    fn update_deployment(
        &self,
        input: UpdateDeploymentRequest,
    ) -> RusotoFuture<Deployment, UpdateDeploymentError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/deployments/{deployment_id}",
            deployment_id = input.deployment_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Deployment>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDeploymentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    fn update_documentation_part(
        &self,
        input: UpdateDocumentationPartRequest,
    ) -> RusotoFuture<DocumentationPart, UpdateDocumentationPartError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/parts/{part_id}",
            part_id = input.documentation_part_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DocumentationPart>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDocumentationPartError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    fn update_documentation_version(
        &self,
        input: UpdateDocumentationVersionRequest,
    ) -> RusotoFuture<DocumentationVersion, UpdateDocumentationVersionError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/versions/{doc_version}",
            doc_version = input.documentation_version,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DocumentationVersion>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDocumentationVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Changes information about the <a>DomainName</a> resource.</p>
    fn update_domain_name(
        &self,
        input: UpdateDomainNameRequest,
    ) -> RusotoFuture<DomainName, UpdateDomainNameError> {
        let request_uri = format!(
            "/domainnames/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DomainName>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDomainNameError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a>.</p>
    fn update_gateway_response(
        &self,
        input: UpdateGatewayResponseRequest,
    ) -> RusotoFuture<GatewayResponse, UpdateGatewayResponseError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/gatewayresponses/{response_type}",
            response_type = input.response_type,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GatewayResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateGatewayResponseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Represents an update integration.</p>
    fn update_integration(
        &self,
        input: UpdateIntegrationRequest,
    ) -> RusotoFuture<Integration, UpdateIntegrationError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Integration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateIntegrationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Represents an update integration response.</p>
    fn update_integration_response(
        &self,
        input: UpdateIntegrationResponseRequest,
    ) -> RusotoFuture<IntegrationResponse, UpdateIntegrationResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<IntegrationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateIntegrationResponseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates an existing <a>Method</a> resource.</p>
    fn update_method(&self, input: UpdateMethodRequest) -> RusotoFuture<Method, UpdateMethodError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Method>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateMethodError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates an existing <a>MethodResponse</a> resource.</p>
    fn update_method_response(
        &self,
        input: UpdateMethodResponseRequest,
    ) -> RusotoFuture<MethodResponse, UpdateMethodResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<MethodResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateMethodResponseError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Changes information about a model.</p>
    fn update_model(&self, input: UpdateModelRequest) -> RusotoFuture<Model, UpdateModelError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/models/{model_name}",
            model_name = input.model_name,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Model>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateModelError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
    fn update_request_validator(
        &self,
        input: UpdateRequestValidatorRequest,
    ) -> RusotoFuture<RequestValidator, UpdateRequestValidatorError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/requestvalidators/{requestvalidator_id}",
            requestvalidator_id = input.request_validator_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<RequestValidator>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateRequestValidatorError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Changes information about a <a>Resource</a> resource.</p>
    fn update_resource(
        &self,
        input: UpdateResourceRequest,
    ) -> RusotoFuture<Resource, UpdateResourceError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}",
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Resource>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Changes information about the specified API.</p>
    fn update_rest_api(
        &self,
        input: UpdateRestApiRequest,
    ) -> RusotoFuture<RestApi, UpdateRestApiError> {
        let request_uri = format!("/restapis/{restapi_id}", restapi_id = input.rest_api_id);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<RestApi>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateRestApiError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Changes information about a <a>Stage</a> resource.</p>
    fn update_stage(&self, input: UpdateStageRequest) -> RusotoFuture<Stage, UpdateStageError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}",
            restapi_id = input.rest_api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Stage>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateStageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Grants a temporary extension to the remaining quota of a usage plan associated with a specified API key.</p>
    fn update_usage(&self, input: UpdateUsageRequest) -> RusotoFuture<Usage, UpdateUsageError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}/keys/{key_id}/usage",
            key_id = input.key_id,
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Usage>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateUsageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates a usage plan of a given plan Id.</p>
    fn update_usage_plan(
        &self,
        input: UpdateUsagePlanRequest,
    ) -> RusotoFuture<UsagePlan, UpdateUsagePlanError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}",
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UsagePlan>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateUsagePlanError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates an existing <a>VpcLink</a> of a specified identifier.</p>
    fn update_vpc_link(
        &self,
        input: UpdateVpcLinkRequest,
    ) -> RusotoFuture<VpcLink, UpdateVpcLinkError> {
        let request_uri = format!("/vpclinks/{vpclink_id}", vpclink_id = input.vpc_link_id);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<VpcLink>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateVpcLinkError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
