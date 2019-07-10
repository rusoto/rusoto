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
use rusoto_core::compression::CompressRequestPayload;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>Settings for logging access in a stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessLogSettings {
    /// <p>The ARN of the CloudWatch Logs log group to receive access logs.</p>
    #[serde(rename = "DestinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    /// <p>A single line format of the access logs of data, as specified by selected $context
    /// variables. The format must include at least $context.requestId.</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// <p>Represents an API.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Api {
    /// <p>The URI of the API, of the form {api-id}.execute-api.{region}.amazonaws.com. The
    /// stage name is typically appended to this URI to form a complete path to a deployed
    /// API stage.</p>
    #[serde(rename = "ApiEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    /// <p>The API ID.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>An API key selection expression. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>The timestamp when the API was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The API protocol: Currently only WEBSOCKET is supported.</p>
    #[serde(rename = "ProtocolType")]
    pub protocol_type: String,
    /// <p>The route selection expression for the API.</p>
    #[serde(rename = "RouteSelectionExpression")]
    pub route_selection_expression: String,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The warning messages reported when failonwarnings is turned on during
    /// API import.</p>
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p>Represents an API mapping.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApiMapping {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The API mapping identifier.</p>
    #[serde(rename = "ApiMappingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_id: Option<String>,
    /// <p>The API mapping key.</p>
    #[serde(rename = "ApiMappingKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_key: Option<String>,
    /// <p>The API stage.</p>
    #[serde(rename = "Stage")]
    pub stage: String,
}

/// <p>Represents an authorizer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Authorizer {
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the
    /// authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon
    /// Resource Name (ARN). To use resource-based permissions on the Lambda function,
    /// specify null.</p>
    #[serde(rename = "AuthorizerCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The time to live (TTL), in seconds, of cached authorizer results. If it equals 0,
    /// authorization caching is disabled. If it is greater than 0, API Gateway will cache
    /// authorizer responses. If this field is not set, the default value is 300. The maximum
    /// value is 3600, or 1 hour.</p>
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>The authorizer type. Currently the only valid value is REQUEST, for a
    /// Lambda function using incoming request parameters.</p>
    #[serde(rename = "AuthorizerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    /// <p>The authorizer's Uniform Resource Identifier (URI).
    /// ForREQUEST authorizers, this must be a
    /// well-formed Lambda function URI, for example,
    /// arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account_id}:function:{lambda_function_name}/invocations.
    /// In general, the URI has this form:
    /// arn:aws:apigateway:{region}:lambda:path/{service_api}
    /// , where {region} is the same as the region hosting the Lambda
    /// function, path indicates that the remaining substring in the URI should be treated as
    /// the path to the resource, including the initial /. For Lambda functions,
    /// this is usually of the form
    /// /2015-03-31/functions/[FunctionARN]/invocations.</p>
    #[serde(rename = "AuthorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identity source for which authorization is requested.</p><p>For the REQUEST authorizer, this is required when authorization
    /// caching is enabled. The value is a comma-separated string of one or more mapping
    /// expressions of the specified request parameters. For example, if an Auth
    /// header and a Name query string parameters are defined as identity
    /// sources, this value is method.request.header.Auth,
    /// method.request.querystring.Name. These parameters will be used to
    /// derive the authorization caching key and to perform runtime validation of the
    /// REQUEST authorizer by verifying all of the identity-related request
    /// parameters are present, not null, and non-empty. Only when this is true does the
    /// authorizer invoke the authorizer Lambda function, otherwise, it returns a 401
    /// Unauthorized response without calling the Lambda function. The valid value
    /// is a string of comma-separated mapping expressions of the specified request
    /// parameters. When the authorization caching is not enabled, this property is
    /// optional.</p>
    #[serde(rename = "IdentitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    /// <p>The
    /// validation expression does not apply to the REQUEST authorizer.</p>
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>The name of the authorizer.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>For
    /// REQUEST authorizer, this is not
    /// defined.</p>
    #[serde(rename = "ProviderArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateApiMappingRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,

    #[serde(rename = "ApiMappingKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_key: Option<String>,
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The API stage.</p>
    #[serde(rename = "Stage")]
    pub stage: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateApiMappingResponse {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>The API mapping identifier.</p>
    #[serde(rename = "ApiMappingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_id: Option<String>,
    /// <p>The API mapping key.</p>
    #[serde(rename = "ApiMappingKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_key: Option<String>,
    /// <p>The API stage.</p>
    #[serde(rename = "Stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateApiRequest {
    /// <p>An API key selection expression. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The API protocol: Currently only WEBSOCKET is supported.</p>
    #[serde(rename = "ProtocolType")]
    pub protocol_type: String,
    /// <p>The route selection expression for the API.</p>
    #[serde(rename = "RouteSelectionExpression")]
    pub route_selection_expression: String,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateApiResponse {
    /// <p>The URI of the API, of the form {api-id}.execute-api.{region}.amazonaws.com. The
    /// stage name is typically appended to this URI to form a complete path to a deployed
    /// API stage.</p>
    #[serde(rename = "ApiEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    /// <p>The API ID.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>An API key selection expression. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>The timestamp when the API was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The API protocol: Currently only WEBSOCKET is supported.</p>
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// <p>The route selection expression for the API.</p>
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The warning messages reported when failonwarnings is turned on during
    /// API import.</p>
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAuthorizerRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the
    /// authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon
    /// Resource Name (ARN). To use resource-based permissions on the Lambda function,
    /// specify null.</p>
    #[serde(rename = "AuthorizerCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    /// <p>The time to live (TTL), in seconds, of cached authorizer results. If it equals 0,
    /// authorization caching is disabled. If it is greater than 0, API Gateway will cache
    /// authorizer responses. If this field is not set, the default value is 300. The maximum
    /// value is 3600, or 1 hour.</p>
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>The authorizer type. Currently the only valid value is REQUEST, for a
    /// Lambda function using incoming request parameters.</p>
    #[serde(rename = "AuthorizerType")]
    pub authorizer_type: String,
    /// <p>The authorizer's Uniform Resource Identifier (URI). For
    /// REQUEST authorizers, this must be a
    /// well-formed Lambda function URI, for example,
    /// arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account_id}:function:{lambda_function_name}/invocations.
    /// In general, the URI has this form:
    /// arn:aws:apigateway:{region}:lambda:path/{service_api}
    /// , where {region} is the same as the region hosting the Lambda
    /// function, path indicates that the remaining substring in the URI should be treated as
    /// the path to the resource, including the initial /. For Lambda functions,
    /// this is usually of the form
    /// /2015-03-31/functions/[FunctionARN]/invocations.</p>
    #[serde(rename = "AuthorizerUri")]
    pub authorizer_uri: String,
    /// <p>The identity source for which authorization is requested.</p><p>For the REQUEST authorizer, this is required when authorization
    /// caching is enabled. The value is a comma-separated string of one or more mapping
    /// expressions of the specified request parameters. For example, if an Auth
    /// header and a Name query string parameters are defined as identity
    /// sources, this value is method.request.header.Auth,
    /// method.request.querystring.Name. These parameters will be used to
    /// derive the authorization caching key and to perform runtime validation of the
    /// REQUEST authorizer by verifying all of the identity-related request
    /// parameters are present, not null, and non-empty. Only when this is true does the
    /// authorizer invoke the authorizer Lambda function, otherwise, it returns a 401
    /// Unauthorized response without calling the Lambda function. The valid value
    /// is a string of comma-separated mapping expressions of the specified request
    /// parameters. When the authorization caching is not enabled, this property is
    /// optional.</p>
    #[serde(rename = "IdentitySource")]
    pub identity_source: Vec<String>,
    /// <p>The
    /// validation expression does not apply to the REQUEST authorizer.</p>
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>The name of the authorizer.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>For
    /// REQUEST authorizer, this is not
    /// defined.</p>
    #[serde(rename = "ProviderArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAuthorizerResponse {
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the
    /// authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon
    /// Resource Name (ARN). To use resource-based permissions on the Lambda function,
    /// specify null.</p>
    #[serde(rename = "AuthorizerCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The time to live (TTL), in seconds, of cached authorizer results. If it equals 0,
    /// authorization caching is disabled. If it is greater than 0, API Gateway will cache
    /// authorizer responses. If this field is not set, the default value is 300. The maximum
    /// value is 3600, or 1 hour.</p>
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>The authorizer type. Currently the only valid value is REQUEST, for a
    /// Lambda function using incoming request parameters.</p>
    #[serde(rename = "AuthorizerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    /// <p>The authorizer's Uniform Resource Identifier (URI).
    /// ForREQUEST authorizers, this must be a
    /// well-formed Lambda function URI, for example,
    /// arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account_id}:function:{lambda_function_name}/invocations.
    /// In general, the URI has this form:
    /// arn:aws:apigateway:{region}:lambda:path/{service_api}
    /// , where {region} is the same as the region hosting the Lambda
    /// function, path indicates that the remaining substring in the URI should be treated as
    /// the path to the resource, including the initial /. For Lambda functions,
    /// this is usually of the form
    /// /2015-03-31/functions/[FunctionARN]/invocations.</p>
    #[serde(rename = "AuthorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identity source for which authorization is requested.</p><p>For the REQUEST authorizer, this is required when authorization
    /// caching is enabled. The value is a comma-separated string of one or more mapping
    /// expressions of the specified request parameters. For example, if an Auth
    /// header and a Name query string parameters are defined as identity
    /// sources, this value is method.request.header.Auth,
    /// method.request.querystring.Name. These parameters will be used to
    /// derive the authorization caching key and to perform runtime validation of the
    /// REQUEST authorizer by verifying all of the identity-related request
    /// parameters are present, not null, and non-empty. Only when this is true does the
    /// authorizer invoke the authorizer Lambda function, otherwise, it returns a 401
    /// Unauthorized response without calling the Lambda function. The valid value
    /// is a string of comma-separated mapping expressions of the specified request
    /// parameters. When the authorization caching is not enabled, this property is
    /// optional.</p>
    #[serde(rename = "IdentitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    /// <p>The
    /// validation expression does not apply to the REQUEST authorizer.</p>
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>The name of the authorizer.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>For
    /// REQUEST authorizer, this is not
    /// defined.</p>
    #[serde(rename = "ProviderArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeploymentRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The description for the deployment resource.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the Stage resource for the Deployment
    /// resource to create.</p>
    #[serde(rename = "StageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDeploymentResponse {
    /// <p>The date and time when the Deployment resource was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The identifier for the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The status of the deployment: PENDING, FAILED, or
    /// SUCCEEDED.</p>
    #[serde(rename = "DeploymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    /// <p>May contain additional feedback on the status of an API deployment.</p>
    #[serde(rename = "DeploymentStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status_message: Option<String>,
    /// <p>The description for the deployment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDomainNameRequest {
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The domain name configurations.</p>
    #[serde(rename = "DomainNameConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDomainNameResponse {
    /// <p>The API mapping selection expression.</p>
    #[serde(rename = "ApiMappingSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_selection_expression: Option<String>,
    /// <p>The name of the DomainName resource.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The domain name configurations.</p>
    #[serde(rename = "DomainNameConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateIntegrationRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The connection ID.</p>
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. Currently the only
    /// valid value is INTERNET, for connections through the public routable
    /// internet.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Specifies how to handle response payload content type conversions. Supported
    /// values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the
    /// following behaviors:</p><p>
    /// CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded
    /// string to the corresponding binary blob.</p><p>
    /// CONVERT_TO_TEXT: Converts a response payload from a binary blob to a
    /// Base64-encoded string.</p><p>If this property is not defined, the response payload will be passed through from
    /// the integration response to the route response or method response without
    /// modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS
    /// integrations, three options are available. To specify an IAM Role for API Gateway to
    /// assume, use the role's Amazon Resource Name (ARN). To require that the caller's
    /// identity be passed through from the request, specify the string
    /// arn:aws:iam::*:user/*. To use resource-based permissions on supported
    /// AWS services, specify null.</p>
    #[serde(rename = "CredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    /// <p>The description of the integration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Specifies the integration's HTTP method type.</p>
    #[serde(rename = "IntegrationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_method: Option<String>,
    /// <p>The integration type of an integration. One of the following:</p><p>
    /// AWS: for integrating the route or method request with an AWS service
    /// action, including the Lambda function-invoking action. With the Lambda
    /// function-invoking action, this is referred to as the Lambda custom integration. With
    /// any other AWS service action, this is known as AWS integration.</p><p>
    /// AWS_PROXY: for integrating the route or method request with the Lambda
    /// function-invoking action with the client request passed through as-is. This
    /// integration is also referred to as Lambda proxy integration.</p><p>
    /// HTTP: for integrating the route or method request with an HTTP
    /// endpoint. This
    /// integration is also referred to as HTTP custom integration.</p><p>
    /// HTTP_PROXY: for integrating route or method request with an HTTP
    /// endpoint, with the client
    /// request passed through as-is. This is also referred to as HTTP proxy
    /// integration.</p><p>
    /// MOCK: for integrating the route or method request with API Gateway as a
    /// "loopback" endpoint without invoking any backend.</p>
    #[serde(rename = "IntegrationType")]
    pub integration_type: String,
    /// <p>For a Lambda proxy integration, this is the URI of the Lambda function.</p>
    #[serde(rename = "IntegrationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    /// <p>Specifies the pass-through behavior for incoming requests based on the
    /// Content-Type header in the request, and the available mapping
    /// templates specified as the requestTemplates property on the
    /// Integration resource. There are three valid values:
    /// WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and
    /// NEVER.</p><p>
    /// WHEN_NO_MATCH passes the request body for unmapped content types through
    /// to the integration backend without transformation.</p><p>
    /// NEVER rejects unmapped content types with an HTTP 415 Unsupported
    /// Media Type response.</p><p>
    /// WHEN_NO_TEMPLATES allows pass-through when the integration has no
    /// content types mapped to templates. However, if there is at least one content type
    /// defined, unmapped content types will be rejected with the same HTTP 415
    /// Unsupported Media Type response.</p>
    #[serde(rename = "PassthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method
    /// request to the backend. The key is an integration request parameter name and the
    /// associated value is a method request parameter value or static value that must be
    /// enclosed within single quotes and pre-encoded as required by the backend. The method
    /// request parameter value must match the pattern of
    /// method.request.{location}.{name}
    /// , where
    /// {location}
    /// is querystring, path, or header; and
    /// {name}
    /// must be a valid and unique method request parameter name.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload
    /// based on the value of the Content-Type header sent by the client. The content type
    /// value is the key in this map, and the template (as a String) is the value.</p>
    #[serde(rename = "RequestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000
    /// milliseconds or 29 seconds.</p>
    #[serde(rename = "TimeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateIntegrationResponse {
    /// <p>The connection ID.</p>
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. Currently the only
    /// valid value is INTERNET, for connections through the public routable
    /// internet.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Specifies how to handle response payload content type conversions. Supported
    /// values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the
    /// following behaviors:</p><p>
    /// CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded
    /// string to the corresponding binary blob.</p><p>
    /// CONVERT_TO_TEXT: Converts a response payload from a binary blob to a
    /// Base64-encoded string.</p><p>If this property is not defined, the response payload will be passed through from
    /// the integration response to the route response or method response without
    /// modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS
    /// integrations, three options are available. To specify an IAM Role for API Gateway to
    /// assume, use the role's Amazon Resource Name (ARN). To require that the caller's
    /// identity be passed through from the request, specify the string
    /// arn:aws:iam::*:user/*. To use resource-based permissions on supported
    /// AWS services, specify null.</p>
    #[serde(rename = "CredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    /// <p>Represents the description of an integration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Represents the identifier of an integration.</p>
    #[serde(rename = "IntegrationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    /// <p>Specifies the integration's HTTP method type.</p>
    #[serde(rename = "IntegrationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_method: Option<String>,
    /// <p>The integration response selection expression for the integration. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-integration-response-selection-expressions">Integration Response Selection Expressions</a>.</p>
    #[serde(rename = "IntegrationResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_selection_expression: Option<String>,
    /// <p>The integration type of an integration. One of the following:</p><p>
    /// AWS: for integrating the route or method request with an AWS service
    /// action, including the Lambda function-invoking action. With the Lambda
    /// function-invoking action, this is referred to as the Lambda custom integration. With
    /// any other AWS service action, this is known as AWS integration.</p><p>
    /// AWS_PROXY: for integrating the route or method request with the Lambda
    /// function-invoking action with the client request passed through as-is. This
    /// integration is also referred to as Lambda proxy integration.</p><p>
    /// HTTP: for integrating the route or method request with an HTTP
    /// endpoint. This
    /// integration is also referred to as the HTTP custom integration.</p><p>
    /// HTTP_PROXY: for integrating route or method request with an HTTP
    /// endpoint, with the client
    /// request passed through as-is. This is also referred to as HTTP proxy
    /// integration.</p><p>
    /// MOCK: for integrating the route or method request with API Gateway as a
    /// "loopback" endpoint without invoking any backend.</p>
    #[serde(rename = "IntegrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    /// <p>For a Lambda proxy integration, this is the URI of the Lambda function.</p>
    #[serde(rename = "IntegrationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    /// <p>Specifies the pass-through behavior for incoming requests based on the
    /// Content-Type header in the request, and the available mapping
    /// templates specified as the requestTemplates property on the
    /// Integration resource. There are three valid values:
    /// WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and
    /// NEVER.</p><p>
    /// WHEN_NO_MATCH passes the request body for unmapped content types through
    /// to the integration backend without transformation.</p><p>
    /// NEVER rejects unmapped content types with an HTTP 415 Unsupported
    /// Media Type response.</p><p>
    /// WHEN_NO_TEMPLATES allows pass-through when the integration has no
    /// content types mapped to templates. However, if there is at least one content type
    /// defined, unmapped content types will be rejected with the same HTTP 415
    /// Unsupported Media Type response.</p>
    #[serde(rename = "PassthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method
    /// request to the backend. The key is an integration request parameter name and the
    /// associated value is a method request parameter value or static value that must be
    /// enclosed within single quotes and pre-encoded as required by the backend. The method
    /// request parameter value must match the pattern of
    /// method.request.{location}.{name}
    /// , where
    /// {location}
    /// is querystring, path, or header; and
    /// {name}
    /// must be a valid and unique method request parameter name.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload
    /// based on the value of the Content-Type header sent by the client. The content type
    /// value is the key in this map, and the template (as a String) is the value.</p>
    #[serde(rename = "RequestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000
    /// milliseconds or 29 seconds.</p>
    #[serde(rename = "TimeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateIntegrationResponseRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Specifies how to handle response payload content type conversions. Supported
    /// values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the
    /// following behaviors:</p><p>
    /// CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded
    /// string to the corresponding binary blob.</p><p>
    /// CONVERT_TO_TEXT: Converts a response payload from a binary blob to a
    /// Base64-encoded string.</p><p>If this property is not defined, the response payload will be passed through from
    /// the integration response to the route response or method response without
    /// modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>The integration ID.</p>
    #[serde(rename = "IntegrationId")]
    pub integration_id: String,
    /// <p>The integration response key.</p>
    #[serde(rename = "IntegrationResponseKey")]
    pub integration_response_key: String,
    /// <p>A key-value map specifying response parameters that are passed to the method
    /// response from the backend. The key is a method response header parameter name and the
    /// mapped value is an integration response header value, a static value enclosed within
    /// a pair of single quotes, or a JSON expression from the integration response body. The
    /// mapping key must match the pattern of method.response.header.{name},
    /// where {name} is a valid and unique header name. The mapped non-static
    /// value must match the pattern of integration.response.header.{name} or
    /// integration.response.body.{JSON-expression}, where
    /// {name} is a valid and unique response header name and
    /// {JSON-expression} is a valid JSON expression without the $
    /// prefix.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of response templates for the integration response as a
    /// string-to-string map of key-value pairs. Response templates are represented as a
    /// key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration response.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateIntegrationResponseResponse {
    /// <p>Specifies how to handle response payload content type conversions. Supported
    /// values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the
    /// following behaviors:</p><p>
    /// CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded
    /// string to the corresponding binary blob.</p><p>
    /// CONVERT_TO_TEXT: Converts a response payload from a binary blob to a
    /// Base64-encoded string.</p><p>If this property is not defined, the response payload will be passed through from
    /// the integration response to the route response or method response without
    /// modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>The integration response ID.</p>
    #[serde(rename = "IntegrationResponseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_id: Option<String>,
    /// <p>The integration response key.</p>
    #[serde(rename = "IntegrationResponseKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_key: Option<String>,
    /// <p>A key-value map specifying response parameters that are passed to the method
    /// response from the backend. The key is a method response header parameter name and the
    /// mapped value is an integration response header value, a static value enclosed within
    /// a pair of single quotes, or a JSON expression from the integration response body. The
    /// mapping key must match the pattern of method.response.header.{name}, where name is a
    /// valid and unique header name. The mapped non-static value must match the pattern of
    /// integration.response.header.{name} or integration.response.body.{JSON-expression},
    /// where name is a valid and unique response header name and JSON-expression is a valid
    /// JSON expression without the $ prefix.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of response templates for the integration response as a
    /// string-to-string map of key-value pairs. Response templates are represented as a
    /// key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expressions for the integration response.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateModelRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The content-type for the model, for example, "application/json".</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The description of the model.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the model. Must be alphanumeric.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The schema for the model. For application/json models, this should be JSON schema
    /// draft 4 model.</p>
    #[serde(rename = "Schema")]
    pub schema: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateModelResponse {
    /// <p>The content-type for the model, for example, "application/json".</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The description of the model.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The model identifier.</p>
    #[serde(rename = "ModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The name of the model. Must be alphanumeric.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The schema for the model. For application/json models, this should be JSON schema
    /// draft 4 model.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRouteRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Specifies whether an API key is required for the route.</p>
    #[serde(rename = "ApiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>The authorization scopes supported by this
    /// route.</p>
    #[serde(rename = "AuthorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The authorization type for the route. Valid values are NONE for open
    /// access, AWS_IAM for using AWS IAM permissions, and CUSTOM
    /// for using a Lambda
    /// authorizer.</p>
    #[serde(rename = "AuthorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of the Authorizer resource to be associated with this
    /// route, if the authorizationType is CUSTOM
    /// . The authorizer identifier is generated by API Gateway
    /// when you created the authorizer.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The model selection expression for the route.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The operation name for the route.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The request models for the route.</p>
    #[serde(rename = "RequestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The request parameters for the route.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, ParameterConstraints>>,
    /// <p>The route key for the route.</p>
    #[serde(rename = "RouteKey")]
    pub route_key: String,
    /// <p>The route response selection expression for the route.</p>
    #[serde(rename = "RouteResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    /// <p>The target for the route.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateRouteResponse {
    /// <p>Specifies whether an API key is required for this route.</p>
    #[serde(rename = "ApiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>A list of authorization scopes configured on a route. The scopes are used with a
    /// COGNITO_USER_POOLS authorizer to authorize the method invocation. The authorization
    /// works by matching the route scopes against the scopes parsed from the access token in
    /// the incoming request. The method invocation is authorized if any route scope matches
    /// a claimed scope in the access token. Otherwise, the invocation is not authorized.
    /// When the route scope is configured, the client must provide an access token instead
    /// of an identity token for authorization purposes.</p>
    #[serde(rename = "AuthorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The authorization type for the route. Valid values are NONE for open
    /// access, AWS_IAM for using AWS IAM permissions, and CUSTOM
    /// for using a Lambda
    /// authorizer</p>
    #[serde(rename = "AuthorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of the Authorizer resource to be associated with this
    /// route, if the authorizationType is CUSTOM
    /// . The authorizer identifier is generated by API Gateway
    /// when you created the authorizer.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The model selection expression for the route.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The operation name for the route.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The request models for the route.</p>
    #[serde(rename = "RequestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The request parameters for the route.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, ParameterConstraints>>,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    /// <p>The route key for the route.</p>
    #[serde(rename = "RouteKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<String>,
    /// <p>The route response selection expression for the route.</p>
    #[serde(rename = "RouteResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    /// <p>The target for the route.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRouteResponseRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The model selection expression for the route response.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The response models for the route response.</p>
    #[serde(rename = "ResponseModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The route response parameters.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, ParameterConstraints>>,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    pub route_id: String,
    /// <p>The route response key.</p>
    #[serde(rename = "RouteResponseKey")]
    pub route_response_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateRouteResponseResponse {
    /// <p>Represents the model selection expression of a route response.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>Represents the response models of a route response.</p>
    #[serde(rename = "ResponseModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents the response parameters of a route response.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, ParameterConstraints>>,
    /// <p>Represents the identifier of a route response.</p>
    #[serde(rename = "RouteResponseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_id: Option<String>,
    /// <p>Represents the route response key of a route response.</p>
    #[serde(rename = "RouteResponseKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateStageRequest {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The identifier of a client certificate for a Stage.</p>
    #[serde(rename = "ClientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>The default route settings for the stage.</p>
    #[serde(rename = "DefaultRouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<RouteSettings>,
    /// <p>The deployment identifier of the API stage.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The description for the API stage.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Route settings for the stage.</p>
    #[serde(rename = "RouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<::std::collections::HashMap<String, RouteSettings>>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "StageName")]
    pub stage_name: String,
    /// <p>A map that defines the stage variables for a Stage. Variable names
    /// can have alphanumeric and underscore characters, and the values must match
    /// [A-Za-z0-9-._~:/?#&=,]+.</p>
    #[serde(rename = "StageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateStageResponse {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>The identifier of a client certificate for a Stage.</p>
    #[serde(rename = "ClientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>The timestamp when the stage was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>Default route settings for the stage.</p>
    #[serde(rename = "DefaultRouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<RouteSettings>,
    /// <p>The identifier of the Deployment that the Stage is
    /// associated with.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The description of the stage.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The timestamp when the stage was last updated.</p>
    #[serde(rename = "LastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>Route settings for the stage.</p>
    #[serde(rename = "RouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<::std::collections::HashMap<String, RouteSettings>>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "StageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    /// <p>A map that defines the stage variables for a stage resource. Variable names can
    /// have alphanumeric and underscore characters, and the values must match
    /// [A-Za-z0-9-._~:/?#&=,]+.</p>
    #[serde(rename = "StageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApiMappingRequest {
    /// <p>The API mapping identifier.</p>
    #[serde(rename = "ApiMappingId")]
    pub api_mapping_id: String,
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApiRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAuthorizerRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    pub authorizer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDeploymentRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The deployment ID.</p>
    #[serde(rename = "DeploymentId")]
    pub deployment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDomainNameRequest {
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIntegrationRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The integration ID.</p>
    #[serde(rename = "IntegrationId")]
    pub integration_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIntegrationResponseRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The integration ID.</p>
    #[serde(rename = "IntegrationId")]
    pub integration_id: String,
    /// <p>The integration response ID.</p>
    #[serde(rename = "IntegrationResponseId")]
    pub integration_response_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteModelRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The model ID.</p>
    #[serde(rename = "ModelId")]
    pub model_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRouteRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    pub route_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRouteResponseRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    pub route_id: String,
    /// <p>The route response ID.</p>
    #[serde(rename = "RouteResponseId")]
    pub route_response_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteStageRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The stage name.</p>
    #[serde(rename = "StageName")]
    pub stage_name: String,
}

/// <p>An immutable representation of an API that can be called by users. A
/// Deployment must be associated with a Stage for it to be
/// callable over the internet.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Deployment {
    /// <p>The date and time when the Deployment resource was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The identifier for the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The status of the deployment: PENDING, FAILED, or
    /// SUCCEEDED.</p>
    #[serde(rename = "DeploymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    /// <p>May contain additional feedback on the status of an API deployment.</p>
    #[serde(rename = "DeploymentStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status_message: Option<String>,
    /// <p>The description for the deployment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// <p>Represents a domain name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DomainName {
    /// <p>The API mapping selection expression.</p>
    #[serde(rename = "ApiMappingSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_selection_expression: Option<String>,
    /// <p>The name of the DomainName resource.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The domain name configurations.</p>
    #[serde(rename = "DomainNameConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
}

/// <p>The domain name configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainNameConfiguration {
    /// <p>A domain name for the WebSocket API.</p>
    #[serde(rename = "ApiGatewayDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_domain_name: Option<String>,
    /// <p>An AWS-managed certificate that will be used by the edge-optimized endpoint for
    /// this domain name. AWS Certificate Manager is the only supported source.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The user-friendly name of the certificate that will be used by the edge-optimized
    /// endpoint for this domain name.</p>
    #[serde(rename = "CertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    /// <p>The timestamp when the certificate that was used by edge-optimized endpoint for
    /// this domain name was uploaded.</p>
    #[serde(rename = "CertificateUploadDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_upload_date: Option<f64>,
    /// <p>The endpoint type.</p>
    #[serde(rename = "EndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// <p>The Amazon Route 53 Hosted Zone ID of the endpoint.</p>
    #[serde(rename = "HostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApiMappingRequest {
    /// <p>The API mapping identifier.</p>
    #[serde(rename = "ApiMappingId")]
    pub api_mapping_id: String,
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApiMappingResponse {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>The API mapping identifier.</p>
    #[serde(rename = "ApiMappingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_id: Option<String>,
    /// <p>The API mapping key.</p>
    #[serde(rename = "ApiMappingKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_key: Option<String>,
    /// <p>The API stage.</p>
    #[serde(rename = "Stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApiMappingsRequest {
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApiMappingsResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ApiMapping>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApiRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApiResponse {
    /// <p>The URI of the API, of the form {api-id}.execute-api.{region}.amazonaws.com. The
    /// stage name is typically appended to this URI to form a complete path to a deployed
    /// API stage.</p>
    #[serde(rename = "ApiEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    /// <p>The API ID.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>An API key selection expression. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>The timestamp when the API was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The API protocol: Currently only WEBSOCKET is supported.</p>
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// <p>The route selection expression for the API.</p>
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The warning messages reported when failonwarnings is turned on during
    /// API import.</p>
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApisRequest {
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApisResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Api>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAuthorizerRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    pub authorizer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAuthorizerResponse {
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the
    /// authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon
    /// Resource Name (ARN). To use resource-based permissions on the Lambda function,
    /// specify null.</p>
    #[serde(rename = "AuthorizerCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The time to live (TTL), in seconds, of cached authorizer results. If it equals 0,
    /// authorization caching is disabled. If it is greater than 0, API Gateway will cache
    /// authorizer responses. If this field is not set, the default value is 300. The maximum
    /// value is 3600, or 1 hour.</p>
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>The authorizer type. Currently the only valid value is REQUEST, for a
    /// Lambda function using incoming request parameters.</p>
    #[serde(rename = "AuthorizerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    /// <p>The authorizer's Uniform Resource Identifier (URI).
    /// ForREQUEST authorizers, this must be a
    /// well-formed Lambda function URI, for example,
    /// arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account_id}:function:{lambda_function_name}/invocations.
    /// In general, the URI has this form:
    /// arn:aws:apigateway:{region}:lambda:path/{service_api}
    /// , where {region} is the same as the region hosting the Lambda
    /// function, path indicates that the remaining substring in the URI should be treated as
    /// the path to the resource, including the initial /. For Lambda functions,
    /// this is usually of the form
    /// /2015-03-31/functions/[FunctionARN]/invocations.</p>
    #[serde(rename = "AuthorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identity source for which authorization is requested.</p><p>For the REQUEST authorizer, this is required when authorization
    /// caching is enabled. The value is a comma-separated string of one or more mapping
    /// expressions of the specified request parameters. For example, if an Auth
    /// header and a Name query string parameters are defined as identity
    /// sources, this value is method.request.header.Auth,
    /// method.request.querystring.Name. These parameters will be used to
    /// derive the authorization caching key and to perform runtime validation of the
    /// REQUEST authorizer by verifying all of the identity-related request
    /// parameters are present, not null, and non-empty. Only when this is true does the
    /// authorizer invoke the authorizer Lambda function, otherwise, it returns a 401
    /// Unauthorized response without calling the Lambda function. The valid value
    /// is a string of comma-separated mapping expressions of the specified request
    /// parameters. When the authorization caching is not enabled, this property is
    /// optional.</p>
    #[serde(rename = "IdentitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    /// <p>The
    /// validation expression does not apply to the REQUEST authorizer.</p>
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>The name of the authorizer.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>For
    /// REQUEST authorizer, this is not
    /// defined.</p>
    #[serde(rename = "ProviderArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAuthorizersRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAuthorizersResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Authorizer>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeploymentRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The deployment ID.</p>
    #[serde(rename = "DeploymentId")]
    pub deployment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDeploymentResponse {
    /// <p>The date and time when the Deployment resource was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The identifier for the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The status of the deployment: PENDING, FAILED, or
    /// SUCCEEDED.</p>
    #[serde(rename = "DeploymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    /// <p>May contain additional feedback on the status of an API deployment.</p>
    #[serde(rename = "DeploymentStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status_message: Option<String>,
    /// <p>The description for the deployment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeploymentsRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDeploymentsResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Deployment>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDomainNameRequest {
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDomainNameResponse {
    /// <p>The API mapping selection expression.</p>
    #[serde(rename = "ApiMappingSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_selection_expression: Option<String>,
    /// <p>The name of the DomainName resource.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The domain name configurations.</p>
    #[serde(rename = "DomainNameConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDomainNamesRequest {
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDomainNamesResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DomainName>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntegrationRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The integration ID.</p>
    #[serde(rename = "IntegrationId")]
    pub integration_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetIntegrationResponse {
    /// <p>The connection ID.</p>
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. Currently the only
    /// valid value is INTERNET, for connections through the public routable
    /// internet.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Specifies how to handle response payload content type conversions. Supported
    /// values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the
    /// following behaviors:</p><p>
    /// CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded
    /// string to the corresponding binary blob.</p><p>
    /// CONVERT_TO_TEXT: Converts a response payload from a binary blob to a
    /// Base64-encoded string.</p><p>If this property is not defined, the response payload will be passed through from
    /// the integration response to the route response or method response without
    /// modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS
    /// integrations, three options are available. To specify an IAM Role for API Gateway to
    /// assume, use the role's Amazon Resource Name (ARN). To require that the caller's
    /// identity be passed through from the request, specify the string
    /// arn:aws:iam::*:user/*. To use resource-based permissions on supported
    /// AWS services, specify null.</p>
    #[serde(rename = "CredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    /// <p>Represents the description of an integration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Represents the identifier of an integration.</p>
    #[serde(rename = "IntegrationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    /// <p>Specifies the integration's HTTP method type.</p>
    #[serde(rename = "IntegrationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_method: Option<String>,
    /// <p>The integration response selection expression for the integration. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-integration-response-selection-expressions">Integration Response Selection Expressions</a>.</p>
    #[serde(rename = "IntegrationResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_selection_expression: Option<String>,
    /// <p>The integration type of an integration. One of the following:</p><p>
    /// AWS: for integrating the route or method request with an AWS service
    /// action, including the Lambda function-invoking action. With the Lambda
    /// function-invoking action, this is referred to as the Lambda custom integration. With
    /// any other AWS service action, this is known as AWS integration.</p><p>
    /// AWS_PROXY: for integrating the route or method request with the Lambda
    /// function-invoking action with the client request passed through as-is. This
    /// integration is also referred to as Lambda proxy integration.</p><p>
    /// HTTP: for integrating the route or method request with an HTTP
    /// endpoint. This
    /// integration is also referred to as the HTTP custom integration.</p><p>
    /// HTTP_PROXY: for integrating route or method request with an HTTP
    /// endpoint, with the client
    /// request passed through as-is. This is also referred to as HTTP proxy
    /// integration.</p><p>
    /// MOCK: for integrating the route or method request with API Gateway as a
    /// "loopback" endpoint without invoking any backend.</p>
    #[serde(rename = "IntegrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    /// <p>For a Lambda proxy integration, this is the URI of the Lambda function.</p>
    #[serde(rename = "IntegrationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    /// <p>Specifies the pass-through behavior for incoming requests based on the
    /// Content-Type header in the request, and the available mapping
    /// templates specified as the requestTemplates property on the
    /// Integration resource. There are three valid values:
    /// WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and
    /// NEVER.</p><p>
    /// WHEN_NO_MATCH passes the request body for unmapped content types through
    /// to the integration backend without transformation.</p><p>
    /// NEVER rejects unmapped content types with an HTTP 415 Unsupported
    /// Media Type response.</p><p>
    /// WHEN_NO_TEMPLATES allows pass-through when the integration has no
    /// content types mapped to templates. However, if there is at least one content type
    /// defined, unmapped content types will be rejected with the same HTTP 415
    /// Unsupported Media Type response.</p>
    #[serde(rename = "PassthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method
    /// request to the backend. The key is an integration request parameter name and the
    /// associated value is a method request parameter value or static value that must be
    /// enclosed within single quotes and pre-encoded as required by the backend. The method
    /// request parameter value must match the pattern of
    /// method.request.{location}.{name}
    /// , where
    /// {location}
    /// is querystring, path, or header; and
    /// {name}
    /// must be a valid and unique method request parameter name.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload
    /// based on the value of the Content-Type header sent by the client. The content type
    /// value is the key in this map, and the template (as a String) is the value.</p>
    #[serde(rename = "RequestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000
    /// milliseconds or 29 seconds.</p>
    #[serde(rename = "TimeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntegrationResponseRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The integration ID.</p>
    #[serde(rename = "IntegrationId")]
    pub integration_id: String,
    /// <p>The integration response ID.</p>
    #[serde(rename = "IntegrationResponseId")]
    pub integration_response_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetIntegrationResponseResponse {
    /// <p>Specifies how to handle response payload content type conversions. Supported
    /// values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the
    /// following behaviors:</p><p>
    /// CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded
    /// string to the corresponding binary blob.</p><p>
    /// CONVERT_TO_TEXT: Converts a response payload from a binary blob to a
    /// Base64-encoded string.</p><p>If this property is not defined, the response payload will be passed through from
    /// the integration response to the route response or method response without
    /// modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>The integration response ID.</p>
    #[serde(rename = "IntegrationResponseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_id: Option<String>,
    /// <p>The integration response key.</p>
    #[serde(rename = "IntegrationResponseKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_key: Option<String>,
    /// <p>A key-value map specifying response parameters that are passed to the method
    /// response from the backend. The key is a method response header parameter name and the
    /// mapped value is an integration response header value, a static value enclosed within
    /// a pair of single quotes, or a JSON expression from the integration response body. The
    /// mapping key must match the pattern of method.response.header.{name}, where name is a
    /// valid and unique header name. The mapped non-static value must match the pattern of
    /// integration.response.header.{name} or integration.response.body.{JSON-expression},
    /// where name is a valid and unique response header name and JSON-expression is a valid
    /// JSON expression without the $ prefix.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of response templates for the integration response as a
    /// string-to-string map of key-value pairs. Response templates are represented as a
    /// key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expressions for the integration response.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntegrationResponsesRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The integration ID.</p>
    #[serde(rename = "IntegrationId")]
    pub integration_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetIntegrationResponsesResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<IntegrationResponse>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntegrationsRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetIntegrationsResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Integration>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetModelRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The model ID.</p>
    #[serde(rename = "ModelId")]
    pub model_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetModelResponse {
    /// <p>The content-type for the model, for example, "application/json".</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The description of the model.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The model identifier.</p>
    #[serde(rename = "ModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The name of the model. Must be alphanumeric.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The schema for the model. For application/json models, this should be JSON schema
    /// draft 4 model.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetModelTemplateRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The model ID.</p>
    #[serde(rename = "ModelId")]
    pub model_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetModelTemplateResponse {
    /// <p>The template value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetModelsRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetModelsResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Model>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRouteRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    pub route_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRouteResponse {
    /// <p>Specifies whether an API key is required for this route.</p>
    #[serde(rename = "ApiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>A list of authorization scopes configured on a route. The scopes are used with a
    /// COGNITO_USER_POOLS authorizer to authorize the method invocation. The authorization
    /// works by matching the route scopes against the scopes parsed from the access token in
    /// the incoming request. The method invocation is authorized if any route scope matches
    /// a claimed scope in the access token. Otherwise, the invocation is not authorized.
    /// When the route scope is configured, the client must provide an access token instead
    /// of an identity token for authorization purposes.</p>
    #[serde(rename = "AuthorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The authorization type for the route. Valid values are NONE for open
    /// access, AWS_IAM for using AWS IAM permissions, and CUSTOM
    /// for using a Lambda
    /// authorizer</p>
    #[serde(rename = "AuthorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of the Authorizer resource to be associated with this
    /// route, if the authorizationType is CUSTOM
    /// . The authorizer identifier is generated by API Gateway
    /// when you created the authorizer.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The model selection expression for the route.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The operation name for the route.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The request models for the route.</p>
    #[serde(rename = "RequestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The request parameters for the route.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, ParameterConstraints>>,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    /// <p>The route key for the route.</p>
    #[serde(rename = "RouteKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<String>,
    /// <p>The route response selection expression for the route.</p>
    #[serde(rename = "RouteResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    /// <p>The target for the route.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRouteResponseRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    pub route_id: String,
    /// <p>The route response ID.</p>
    #[serde(rename = "RouteResponseId")]
    pub route_response_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRouteResponseResponse {
    /// <p>Represents the model selection expression of a route response.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>Represents the response models of a route response.</p>
    #[serde(rename = "ResponseModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents the response parameters of a route response.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, ParameterConstraints>>,
    /// <p>Represents the identifier of a route response.</p>
    #[serde(rename = "RouteResponseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_id: Option<String>,
    /// <p>Represents the route response key of a route response.</p>
    #[serde(rename = "RouteResponseKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRouteResponsesRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    pub route_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRouteResponsesResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RouteResponse>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRoutesRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRoutesResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Route>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetStageRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The stage name.</p>
    #[serde(rename = "StageName")]
    pub stage_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetStageResponse {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>The identifier of a client certificate for a Stage.</p>
    #[serde(rename = "ClientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>The timestamp when the stage was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>Default route settings for the stage.</p>
    #[serde(rename = "DefaultRouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<RouteSettings>,
    /// <p>The identifier of the Deployment that the Stage is
    /// associated with.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The description of the stage.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The timestamp when the stage was last updated.</p>
    #[serde(rename = "LastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>Route settings for the stage.</p>
    #[serde(rename = "RouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<::std::collections::HashMap<String, RouteSettings>>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "StageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    /// <p>A map that defines the stage variables for a stage resource. Variable names can
    /// have alphanumeric and underscore characters, and the values must match
    /// [A-Za-z0-9-._~:/?#&=,]+.</p>
    #[serde(rename = "StageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetStagesRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetStagesResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Stage>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of
    /// the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents an integration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Integration {
    /// <p>The connection ID.</p>
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. Currently the only
    /// valid value is INTERNET, for connections through the public routable
    /// internet.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Specifies how to handle response payload content type conversions. Supported
    /// values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the
    /// following behaviors:</p><p>
    /// CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded
    /// string to the corresponding binary blob.</p><p>
    /// CONVERT_TO_TEXT: Converts a response payload from a binary blob to a
    /// Base64-encoded string.</p><p>If this property is not defined, the response payload will be passed through from
    /// the integration response to the route response or method response without
    /// modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS
    /// integrations, three options are available. To specify an IAM Role for API Gateway to
    /// assume, use the role's Amazon Resource Name (ARN). To require that the caller's
    /// identity be passed through from the request, specify the string
    /// arn:aws:iam::*:user/*. To use resource-based permissions on supported
    /// AWS services, specify null.</p>
    #[serde(rename = "CredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    /// <p>Represents the description of an integration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Represents the identifier of an integration.</p>
    #[serde(rename = "IntegrationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    /// <p>Specifies the integration's HTTP method type.</p>
    #[serde(rename = "IntegrationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_method: Option<String>,
    /// <p>The integration response selection expression for the integration. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-integration-response-selection-expressions">Integration Response Selection Expressions</a>.</p>
    #[serde(rename = "IntegrationResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_selection_expression: Option<String>,
    /// <p>The integration type of an integration. One of the following:</p><p>
    /// AWS: for integrating the route or method request with an AWS service
    /// action, including the Lambda function-invoking action. With the Lambda
    /// function-invoking action, this is referred to as the Lambda custom integration. With
    /// any other AWS service action, this is known as AWS integration.</p><p>
    /// AWS_PROXY: for integrating the route or method request with the Lambda
    /// function-invoking action with the client request passed through as-is. This
    /// integration is also referred to as Lambda proxy integration.</p><p>
    /// HTTP: for integrating the route or method request with an HTTP
    /// endpoint. This
    /// integration is also referred to as the HTTP custom integration.</p><p>
    /// HTTP_PROXY: for integrating route or method request with an HTTP
    /// endpoint, with the client
    /// request passed through as-is. This is also referred to as HTTP proxy
    /// integration.</p><p>
    /// MOCK: for integrating the route or method request with API Gateway as a
    /// "loopback" endpoint without invoking any backend.</p>
    #[serde(rename = "IntegrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    /// <p>For a Lambda proxy integration, this is the URI of the Lambda function.</p>
    #[serde(rename = "IntegrationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    /// <p>Specifies the pass-through behavior for incoming requests based on the
    /// Content-Type header in the request, and the available mapping
    /// templates specified as the requestTemplates property on the
    /// Integration resource. There are three valid values:
    /// WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and
    /// NEVER.</p><p>
    /// WHEN_NO_MATCH passes the request body for unmapped content types through
    /// to the integration backend without transformation.</p><p>
    /// NEVER rejects unmapped content types with an HTTP 415 Unsupported
    /// Media Type response.</p><p>
    /// WHEN_NO_TEMPLATES allows pass-through when the integration has no
    /// content types mapped to templates. However, if there is at least one content type
    /// defined, unmapped content types will be rejected with the same HTTP 415
    /// Unsupported Media Type response.</p>
    #[serde(rename = "PassthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method
    /// request to the backend. The key is an integration request parameter name and the
    /// associated value is a method request parameter value or static value that must be
    /// enclosed within single quotes and pre-encoded as required by the backend. The method
    /// request parameter value must match the pattern of
    /// method.request.{location}.{name}
    /// , where
    /// {location}
    /// is querystring, path, or header; and
    /// {name}
    /// must be a valid and unique method request parameter name.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload
    /// based on the value of the Content-Type header sent by the client. The content type
    /// value is the key in this map, and the template (as a String) is the value.</p>
    #[serde(rename = "RequestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000
    /// milliseconds or 29 seconds.</p>
    #[serde(rename = "TimeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
}

/// <p>Represents an integration response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IntegrationResponse {
    /// <p>Specifies how to handle response payload content type conversions. Supported
    /// values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the
    /// following behaviors:</p><p>
    /// CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded
    /// string to the corresponding binary blob.</p><p>
    /// CONVERT_TO_TEXT: Converts a response payload from a binary blob to a
    /// Base64-encoded string.</p><p>If this property is not defined, the response payload will be passed through from
    /// the integration response to the route response or method response without
    /// modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>The integration response ID.</p>
    #[serde(rename = "IntegrationResponseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_id: Option<String>,
    /// <p>The integration response key.</p>
    #[serde(rename = "IntegrationResponseKey")]
    pub integration_response_key: String,
    /// <p>A key-value map specifying response parameters that are passed to the method
    /// response from the backend. The key is a method response header parameter name and the
    /// mapped value is an integration response header value, a static value enclosed within
    /// a pair of single quotes, or a JSON expression from the integration response body. The
    /// mapping key must match the pattern of method.response.header.{name}, where name is a
    /// valid and unique header name. The mapped non-static value must match the pattern of
    /// integration.response.header.{name} or integration.response.body.{JSON-expression},
    /// where name is a valid and unique response header name and JSON-expression is a valid
    /// JSON expression without the $ prefix.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of response templates for the integration response as a
    /// string-to-string map of key-value pairs. Response templates are represented as a
    /// key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expressions for the integration response.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

/// <p>Represents a data model for an API. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/models-mappings.html">Create Models and Mapping Templates for Request and Response
/// Mappings</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Model {
    /// <p>The content-type for the model, for example, "application/json".</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The description of the model.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The model identifier.</p>
    #[serde(rename = "ModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The name of the model. Must be alphanumeric.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The schema for the model. For application/json models, this should be JSON schema
    /// draft 4 model.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

/// <p>Validation constraints imposed on parameters of a request (path, query string,
/// headers).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterConstraints {
    /// <p>Whether or not the parameter is required.</p>
    #[serde(rename = "Required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// <p>Represents a route.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Route {
    /// <p>Specifies whether an API key is required for this route.</p>
    #[serde(rename = "ApiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>A list of authorization scopes configured on a route. The scopes are used with a
    /// COGNITO_USER_POOLS authorizer to authorize the method invocation. The authorization
    /// works by matching the route scopes against the scopes parsed from the access token in
    /// the incoming request. The method invocation is authorized if any route scope matches
    /// a claimed scope in the access token. Otherwise, the invocation is not authorized.
    /// When the route scope is configured, the client must provide an access token instead
    /// of an identity token for authorization purposes.</p>
    #[serde(rename = "AuthorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The authorization type for the route. Valid values are NONE for open
    /// access, AWS_IAM for using AWS IAM permissions, and CUSTOM
    /// for using a Lambda
    /// authorizer</p>
    #[serde(rename = "AuthorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of the Authorizer resource to be associated with this
    /// route, if the authorizationType is CUSTOM
    /// . The authorizer identifier is generated by API Gateway
    /// when you created the authorizer.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The model selection expression for the route.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The operation name for the route.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The request models for the route.</p>
    #[serde(rename = "RequestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The request parameters for the route.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, ParameterConstraints>>,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    /// <p>The route key for the route.</p>
    #[serde(rename = "RouteKey")]
    pub route_key: String,
    /// <p>The route response selection expression for the route.</p>
    #[serde(rename = "RouteResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    /// <p>The target for the route.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// <p>Represents a route response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RouteResponse {
    /// <p>Represents the model selection expression of a route response.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>Represents the response models of a route response.</p>
    #[serde(rename = "ResponseModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents the response parameters of a route response.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, ParameterConstraints>>,
    /// <p>Represents the identifier of a route response.</p>
    #[serde(rename = "RouteResponseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_id: Option<String>,
    /// <p>Represents the route response key of a route response.</p>
    #[serde(rename = "RouteResponseKey")]
    pub route_response_key: String,
}

/// <p>Represents a collection of route settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteSettings {
    /// <p>Specifies whether (true) or not (false) data trace
    /// logging is enabled for this route. This property affects the log entries pushed to
    /// Amazon CloudWatch Logs.</p>
    #[serde(rename = "DataTraceEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_trace_enabled: Option<bool>,
    /// <p>Specifies whether detailed metrics are enabled.</p>
    #[serde(rename = "DetailedMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_metrics_enabled: Option<bool>,
    /// <p>Specifies the logging level for this route: DEBUG, INFO,
    /// or WARN. This property affects the log entries pushed to Amazon
    /// CloudWatch Logs.</p>
    #[serde(rename = "LoggingLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    /// <p>Specifies the throttling burst limit.</p>
    #[serde(rename = "ThrottlingBurstLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_burst_limit: Option<i64>,
    /// <p>Specifies the throttling rate limit.</p>
    #[serde(rename = "ThrottlingRateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_rate_limit: Option<f64>,
}

/// <p>Represents an API stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Stage {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>The identifier of a client certificate for a Stage.</p>
    #[serde(rename = "ClientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>The timestamp when the stage was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>Default route settings for the stage.</p>
    #[serde(rename = "DefaultRouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<RouteSettings>,
    /// <p>The identifier of the Deployment that the Stage is
    /// associated with.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The description of the stage.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The timestamp when the stage was last updated.</p>
    #[serde(rename = "LastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>Route settings for the stage.</p>
    #[serde(rename = "RouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<::std::collections::HashMap<String, RouteSettings>>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "StageName")]
    pub stage_name: String,
    /// <p>A map that defines the stage variables for a stage resource. Variable names can
    /// have alphanumeric and underscore characters, and the values must match
    /// [A-Za-z0-9-._~:/?#&=,]+.</p>
    #[serde(rename = "StageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApiMappingRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The API mapping identifier.</p>
    #[serde(rename = "ApiMappingId")]
    pub api_mapping_id: String,
    /// <p>The API mapping key.</p>
    #[serde(rename = "ApiMappingKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_key: Option<String>,
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The API stage.</p>
    #[serde(rename = "Stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApiMappingResponse {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>The API mapping identifier.</p>
    #[serde(rename = "ApiMappingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_id: Option<String>,
    /// <p>The API mapping key.</p>
    #[serde(rename = "ApiMappingKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_key: Option<String>,
    /// <p>The API stage.</p>
    #[serde(rename = "Stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApiRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>An API key selection expression. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The route selection expression for the API.</p>
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApiResponse {
    /// <p>The URI of the API, of the form {api-id}.execute-api.{region}.amazonaws.com. The
    /// stage name is typically appended to this URI to form a complete path to a deployed
    /// API stage.</p>
    #[serde(rename = "ApiEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    /// <p>The API ID.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>An API key selection expression. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>The timestamp when the API was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The API protocol: Currently only WEBSOCKET is supported.</p>
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// <p>The route selection expression for the API.</p>
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The warning messages reported when failonwarnings is turned on during
    /// API import.</p>
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAuthorizerRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the
    /// authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon
    /// Resource Name (ARN). To use resource-based permissions on the Lambda function,
    /// specify null.</p>
    #[serde(rename = "AuthorizerCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    pub authorizer_id: String,
    /// <p>The time to live (TTL), in seconds, of cached authorizer results. If it is zero,
    /// authorization caching is disabled. If it is greater than zero, API Gateway will cache
    /// authorizer responses. If this field is not set, the default value is 300. The maximum
    /// value is 3600, or 1 hour.</p>
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>The authorizer type. Currently the only valid value is REQUEST, for a
    /// Lambda function using incoming request parameters.</p>
    #[serde(rename = "AuthorizerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    /// <p>The authorizer's Uniform Resource Identifier (URI). For
    /// REQUEST authorizers, this must be a
    /// well-formed Lambda function URI, for example,
    /// arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account_id}:function:{lambda_function_name}/invocations.
    /// In general, the URI has this form:
    /// arn:aws:apigateway:{region}:lambda:path/{service_api}
    /// , where {region} is the same as the region hosting the Lambda
    /// function, path indicates that the remaining substring in the URI should be treated as
    /// the path to the resource, including the initial /. For Lambda functions,
    /// this is usually of the form
    /// /2015-03-31/functions/[FunctionARN]/invocations.</p>
    #[serde(rename = "AuthorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identity source for which authorization is requested.</p><p>For the REQUEST authorizer, this is required when authorization
    /// caching is enabled. The value is a comma-separated string of one or more mapping
    /// expressions of the specified request parameters. For example, if an Auth header, a
    /// Name query string parameter are defined as identity sources, this value is
    /// $method.request.header.Auth, $method.request.querystring.Name. These
    /// parameters will be used to derive the authorization caching key and to perform
    /// runtime validation of the REQUEST authorizer by verifying all of the
    /// identity-related request parameters are present, not null and non-empty. Only when
    /// this is true does the authorizer invoke the authorizer Lambda function, otherwise, it
    /// returns a 401 Unauthorized response without calling the Lambda function.
    /// The valid value is a string of comma-separated mapping expressions of the specified
    /// request parameters. When the authorization caching is not enabled, this property is
    /// optional.</p>
    #[serde(rename = "IdentitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    /// <p>The
    /// validation expression does not apply to the REQUEST authorizer.</p>
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>The name of the authorizer.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>For
    /// REQUEST authorizer, this is not
    /// defined.</p>
    #[serde(rename = "ProviderArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateAuthorizerResponse {
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the
    /// authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon
    /// Resource Name (ARN). To use resource-based permissions on the Lambda function,
    /// specify null.</p>
    #[serde(rename = "AuthorizerCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The time to live (TTL), in seconds, of cached authorizer results. If it equals 0,
    /// authorization caching is disabled. If it is greater than 0, API Gateway will cache
    /// authorizer responses. If this field is not set, the default value is 300. The maximum
    /// value is 3600, or 1 hour.</p>
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>The authorizer type. Currently the only valid value is REQUEST, for a
    /// Lambda function using incoming request parameters.</p>
    #[serde(rename = "AuthorizerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    /// <p>The authorizer's Uniform Resource Identifier (URI).
    /// ForREQUEST authorizers, this must be a
    /// well-formed Lambda function URI, for example,
    /// arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account_id}:function:{lambda_function_name}/invocations.
    /// In general, the URI has this form:
    /// arn:aws:apigateway:{region}:lambda:path/{service_api}
    /// , where {region} is the same as the region hosting the Lambda
    /// function, path indicates that the remaining substring in the URI should be treated as
    /// the path to the resource, including the initial /. For Lambda functions,
    /// this is usually of the form
    /// /2015-03-31/functions/[FunctionARN]/invocations.</p>
    #[serde(rename = "AuthorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identity source for which authorization is requested.</p><p>For the REQUEST authorizer, this is required when authorization
    /// caching is enabled. The value is a comma-separated string of one or more mapping
    /// expressions of the specified request parameters. For example, if an Auth
    /// header and a Name query string parameters are defined as identity
    /// sources, this value is method.request.header.Auth,
    /// method.request.querystring.Name. These parameters will be used to
    /// derive the authorization caching key and to perform runtime validation of the
    /// REQUEST authorizer by verifying all of the identity-related request
    /// parameters are present, not null, and non-empty. Only when this is true does the
    /// authorizer invoke the authorizer Lambda function, otherwise, it returns a 401
    /// Unauthorized response without calling the Lambda function. The valid value
    /// is a string of comma-separated mapping expressions of the specified request
    /// parameters. When the authorization caching is not enabled, this property is
    /// optional.</p>
    #[serde(rename = "IdentitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    /// <p>The
    /// validation expression does not apply to the REQUEST authorizer.</p>
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>The name of the authorizer.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>For
    /// REQUEST authorizer, this is not
    /// defined.</p>
    #[serde(rename = "ProviderArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDeploymentRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The deployment ID.</p>
    #[serde(rename = "DeploymentId")]
    pub deployment_id: String,
    /// <p>The description for the deployment resource.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateDeploymentResponse {
    /// <p>The date and time when the Deployment resource was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The identifier for the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The status of the deployment: PENDING, FAILED, or
    /// SUCCEEDED.</p>
    #[serde(rename = "DeploymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    /// <p>May contain additional feedback on the status of an API deployment.</p>
    #[serde(rename = "DeploymentStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status_message: Option<String>,
    /// <p>The description for the deployment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDomainNameRequest {
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The domain name configurations.</p>
    #[serde(rename = "DomainNameConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateDomainNameResponse {
    /// <p>The API mapping selection expression.</p>
    #[serde(rename = "ApiMappingSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_selection_expression: Option<String>,
    /// <p>The name of the DomainName resource.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The domain name configurations.</p>
    #[serde(rename = "DomainNameConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateIntegrationRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The connection ID.</p>
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. Currently the only
    /// valid value is INTERNET, for connections through the public routable
    /// internet.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Specifies how to handle response payload content type conversions. Supported
    /// values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the
    /// following behaviors:</p><p>
    /// CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded
    /// string to the corresponding binary blob.</p><p>
    /// CONVERT_TO_TEXT: Converts a response payload from a binary blob to a
    /// Base64-encoded string.</p><p>If this property is not defined, the response payload will be passed through from
    /// the integration response to the route response or method response without
    /// modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS
    /// integrations, three options are available. To specify an IAM Role for API Gateway to
    /// assume, use the role's Amazon Resource Name (ARN). To require that the caller's
    /// identity be passed through from the request, specify the string
    /// arn:aws:iam::*:user/*. To use resource-based permissions on supported
    /// AWS services, specify null.</p>
    #[serde(rename = "CredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    /// <p>The description of the integration</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The integration ID.</p>
    #[serde(rename = "IntegrationId")]
    pub integration_id: String,
    /// <p>Specifies the integration's HTTP method type.</p>
    #[serde(rename = "IntegrationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_method: Option<String>,
    /// <p>The integration type of an integration. One of the following:</p><p>
    /// AWS: for integrating the route or method request with an AWS service
    /// action, including the Lambda function-invoking action. With the Lambda
    /// function-invoking action, this is referred to as the Lambda custom integration. With
    /// any other AWS service action, this is known as AWS integration.</p><p>
    /// AWS_PROXY: for integrating the route or method request with the Lambda
    /// function-invoking action with the client request passed through as-is. This
    /// integration is also referred to as Lambda proxy integration.</p><p>
    /// HTTP: for integrating the route or method request with an HTTP
    /// endpoint. This
    /// integration is also referred to as the HTTP custom integration.</p><p>
    /// HTTP_PROXY: for integrating route or method request with an HTTP
    /// endpoint, with the client
    /// request passed through as-is. This is also referred to as HTTP proxy
    /// integration.</p><p>
    /// MOCK: for integrating the route or method request with API Gateway as a
    /// "loopback" endpoint without invoking any backend.</p>
    #[serde(rename = "IntegrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    /// <p>For a Lambda proxy integration, this is the URI of the Lambda function.</p>
    #[serde(rename = "IntegrationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    /// <p>Specifies the pass-through behavior for incoming requests based on the
    /// Content-Type header in the request, and the available mapping
    /// templates specified as the requestTemplates property on the
    /// Integration resource. There are three valid values:
    /// WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and
    /// NEVER.</p><p>
    /// WHEN_NO_MATCH passes the request body for unmapped content types through
    /// to the integration backend without transformation.</p><p>
    /// NEVER rejects unmapped content types with an HTTP 415 Unsupported
    /// Media Type response.</p><p>
    /// WHEN_NO_TEMPLATES allows pass-through when the integration has no
    /// content types mapped to templates. However, if there is at least one content type
    /// defined, unmapped content types will be rejected with the same HTTP 415
    /// Unsupported Media Type response.</p>
    #[serde(rename = "PassthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method
    /// request to the backend. The key is an integration request parameter name and the
    /// associated value is a method request parameter value or static value that must be
    /// enclosed within single quotes and pre-encoded as required by the backend. The method
    /// request parameter value must match the pattern of
    /// method.request.{location}.{name}
    /// , where
    /// {location}
    /// is querystring, path, or header; and
    /// {name}
    /// must be a valid and unique method request parameter name.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload
    /// based on the value of the Content-Type header sent by the client. The content type
    /// value is the key in this map, and the template (as a String) is the value.</p>
    #[serde(rename = "RequestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000
    /// milliseconds or 29 seconds.</p>
    #[serde(rename = "TimeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateIntegrationResponse {
    /// <p>The connection ID.</p>
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. Currently the only
    /// valid value is INTERNET, for connections through the public routable
    /// internet.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Specifies how to handle response payload content type conversions. Supported
    /// values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the
    /// following behaviors:</p><p>
    /// CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded
    /// string to the corresponding binary blob.</p><p>
    /// CONVERT_TO_TEXT: Converts a response payload from a binary blob to a
    /// Base64-encoded string.</p><p>If this property is not defined, the response payload will be passed through from
    /// the integration response to the route response or method response without
    /// modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS
    /// integrations, three options are available. To specify an IAM Role for API Gateway to
    /// assume, use the role's Amazon Resource Name (ARN). To require that the caller's
    /// identity be passed through from the request, specify the string
    /// arn:aws:iam::*:user/*. To use resource-based permissions on supported
    /// AWS services, specify null.</p>
    #[serde(rename = "CredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    /// <p>Represents the description of an integration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Represents the identifier of an integration.</p>
    #[serde(rename = "IntegrationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    /// <p>Specifies the integration's HTTP method type.</p>
    #[serde(rename = "IntegrationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_method: Option<String>,
    /// <p>The integration response selection expression for the integration. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-integration-response-selection-expressions">Integration Response Selection Expressions</a>.</p>
    #[serde(rename = "IntegrationResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_selection_expression: Option<String>,
    /// <p>The integration type of an integration. One of the following:</p><p>
    /// AWS: for integrating the route or method request with an AWS service
    /// action, including the Lambda function-invoking action. With the Lambda
    /// function-invoking action, this is referred to as the Lambda custom integration. With
    /// any other AWS service action, this is known as AWS integration.</p><p>
    /// AWS_PROXY: for integrating the route or method request with the Lambda
    /// function-invoking action with the client request passed through as-is. This
    /// integration is also referred to as Lambda proxy integration.</p><p>
    /// HTTP: for integrating the route or method request with an HTTP
    /// endpoint. This
    /// integration is also referred to as the HTTP custom integration.</p><p>
    /// HTTP_PROXY: for integrating route or method request with an HTTP
    /// endpoint, with the client
    /// request passed through as-is. This is also referred to as HTTP proxy
    /// integration.</p><p>
    /// MOCK: for integrating the route or method request with API Gateway as a
    /// "loopback" endpoint without invoking any backend.</p>
    #[serde(rename = "IntegrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    /// <p>For a Lambda proxy integration, this is the URI of the Lambda function.</p>
    #[serde(rename = "IntegrationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    /// <p>Specifies the pass-through behavior for incoming requests based on the
    /// Content-Type header in the request, and the available mapping
    /// templates specified as the requestTemplates property on the
    /// Integration resource. There are three valid values:
    /// WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and
    /// NEVER.</p><p>
    /// WHEN_NO_MATCH passes the request body for unmapped content types through
    /// to the integration backend without transformation.</p><p>
    /// NEVER rejects unmapped content types with an HTTP 415 Unsupported
    /// Media Type response.</p><p>
    /// WHEN_NO_TEMPLATES allows pass-through when the integration has no
    /// content types mapped to templates. However, if there is at least one content type
    /// defined, unmapped content types will be rejected with the same HTTP 415
    /// Unsupported Media Type response.</p>
    #[serde(rename = "PassthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method
    /// request to the backend. The key is an integration request parameter name and the
    /// associated value is a method request parameter value or static value that must be
    /// enclosed within single quotes and pre-encoded as required by the backend. The method
    /// request parameter value must match the pattern of
    /// method.request.{location}.{name}
    /// , where
    /// {location}
    /// is querystring, path, or header; and
    /// {name}
    /// must be a valid and unique method request parameter name.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload
    /// based on the value of the Content-Type header sent by the client. The content type
    /// value is the key in this map, and the template (as a String) is the value.</p>
    #[serde(rename = "RequestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000
    /// milliseconds or 29 seconds.</p>
    #[serde(rename = "TimeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateIntegrationResponseRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Specifies how to handle response payload content type conversions. Supported
    /// values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the
    /// following behaviors:</p><p>
    /// CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded
    /// string to the corresponding binary blob.</p><p>
    /// CONVERT_TO_TEXT: Converts a response payload from a binary blob to a
    /// Base64-encoded string.</p><p>If this property is not defined, the response payload will be passed through from
    /// the integration response to the route response or method response without
    /// modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>The integration ID.</p>
    #[serde(rename = "IntegrationId")]
    pub integration_id: String,
    /// <p>The integration response ID.</p>
    #[serde(rename = "IntegrationResponseId")]
    pub integration_response_id: String,
    /// <p>The integration response key.</p>
    #[serde(rename = "IntegrationResponseKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_key: Option<String>,
    /// <p>A key-value map specifying response parameters that are passed to the method
    /// response from the backend. The key is a method response header parameter name and the
    /// mapped value is an integration response header value, a static value enclosed within
    /// a pair of single quotes, or a JSON expression from the integration response body. The
    /// mapping key must match the pattern of
    /// method.response.header.{name}
    /// , where name is a valid and unique header name. The mapped non-static value
    /// must match the pattern of
    /// integration.response.header.{name}
    /// or
    /// integration.response.body.{JSON-expression}
    /// , where
    /// {name}
    /// is a valid and unique response header name and
    /// {JSON-expression}
    /// is a valid JSON expression without the $ prefix.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of response templates for the integration response as a
    /// string-to-string map of key-value pairs. Response templates are represented as a
    /// key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration response.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateIntegrationResponseResponse {
    /// <p>Specifies how to handle response payload content type conversions. Supported
    /// values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the
    /// following behaviors:</p><p>
    /// CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded
    /// string to the corresponding binary blob.</p><p>
    /// CONVERT_TO_TEXT: Converts a response payload from a binary blob to a
    /// Base64-encoded string.</p><p>If this property is not defined, the response payload will be passed through from
    /// the integration response to the route response or method response without
    /// modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>The integration response ID.</p>
    #[serde(rename = "IntegrationResponseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_id: Option<String>,
    /// <p>The integration response key.</p>
    #[serde(rename = "IntegrationResponseKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_key: Option<String>,
    /// <p>A key-value map specifying response parameters that are passed to the method
    /// response from the backend. The key is a method response header parameter name and the
    /// mapped value is an integration response header value, a static value enclosed within
    /// a pair of single quotes, or a JSON expression from the integration response body. The
    /// mapping key must match the pattern of method.response.header.{name}, where name is a
    /// valid and unique header name. The mapped non-static value must match the pattern of
    /// integration.response.header.{name} or integration.response.body.{JSON-expression},
    /// where name is a valid and unique response header name and JSON-expression is a valid
    /// JSON expression without the $ prefix.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of response templates for the integration response as a
    /// string-to-string map of key-value pairs. Response templates are represented as a
    /// key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expressions for the integration response.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateModelRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The content-type for the model, for example, "application/json".</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The description of the model.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The model ID.</p>
    #[serde(rename = "ModelId")]
    pub model_id: String,
    /// <p>The name of the model.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The schema for the model. For application/json models, this should be JSON schema
    /// draft 4 model.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateModelResponse {
    /// <p>The content-type for the model, for example, "application/json".</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The description of the model.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The model identifier.</p>
    #[serde(rename = "ModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The name of the model. Must be alphanumeric.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The schema for the model. For application/json models, this should be JSON schema
    /// draft 4 model.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRouteRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Specifies whether an API key is required for the route.</p>
    #[serde(rename = "ApiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>The authorization scopes supported by this
    /// route.</p>
    #[serde(rename = "AuthorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The authorization type for the route. Valid values are NONE for open
    /// access, AWS_IAM for using AWS IAM permissions, and CUSTOM
    /// for using a Lambda
    /// authorizer.</p>
    #[serde(rename = "AuthorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of the Authorizer resource to be associated with this
    /// route, if the authorizationType is CUSTOM
    /// . The authorizer identifier is generated by API Gateway
    /// when you created the authorizer.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The model selection expression for the route.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The operation name for the route.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The request models for the route.</p>
    #[serde(rename = "RequestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The request parameters for the route.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, ParameterConstraints>>,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    pub route_id: String,
    /// <p>The route key for the route.</p>
    #[serde(rename = "RouteKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<String>,
    /// <p>The route response selection expression for the route.</p>
    #[serde(rename = "RouteResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    /// <p>The target for the route.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateRouteResponse {
    /// <p>Specifies whether an API key is required for this route.</p>
    #[serde(rename = "ApiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>A list of authorization scopes configured on a route. The scopes are used with a
    /// COGNITO_USER_POOLS authorizer to authorize the method invocation. The authorization
    /// works by matching the route scopes against the scopes parsed from the access token in
    /// the incoming request. The method invocation is authorized if any route scope matches
    /// a claimed scope in the access token. Otherwise, the invocation is not authorized.
    /// When the route scope is configured, the client must provide an access token instead
    /// of an identity token for authorization purposes.</p>
    #[serde(rename = "AuthorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The authorization type for the route. Valid values are NONE for open
    /// access, AWS_IAM for using AWS IAM permissions, and CUSTOM
    /// for using a Lambda
    /// authorizer</p>
    #[serde(rename = "AuthorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of the Authorizer resource to be associated with this
    /// route, if the authorizationType is CUSTOM
    /// . The authorizer identifier is generated by API Gateway
    /// when you created the authorizer.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The model selection expression for the route.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The operation name for the route.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The request models for the route.</p>
    #[serde(rename = "RequestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The request parameters for the route.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, ParameterConstraints>>,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    /// <p>The route key for the route.</p>
    #[serde(rename = "RouteKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<String>,
    /// <p>The route response selection expression for the route.</p>
    #[serde(rename = "RouteResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    /// <p>The target for the route.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRouteResponseRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The model selection expression for the route response.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The response models for the route response.</p>
    #[serde(rename = "ResponseModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The route response parameters.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, ParameterConstraints>>,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    pub route_id: String,
    /// <p>The route response ID.</p>
    #[serde(rename = "RouteResponseId")]
    pub route_response_id: String,
    /// <p>The route response key.</p>
    #[serde(rename = "RouteResponseKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateRouteResponseResponse {
    /// <p>Represents the model selection expression of a route response.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>Represents the response models of a route response.</p>
    #[serde(rename = "ResponseModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents the response parameters of a route response.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, ParameterConstraints>>,
    /// <p>Represents the identifier of a route response.</p>
    #[serde(rename = "RouteResponseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_id: Option<String>,
    /// <p>Represents the route response key of a route response.</p>
    #[serde(rename = "RouteResponseKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateStageRequest {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The identifier of a client certificate for a Stage.</p>
    #[serde(rename = "ClientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>The default route settings for the stage.</p>
    #[serde(rename = "DefaultRouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<RouteSettings>,
    /// <p>The deployment identifier for the API stage.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The description for the API stage.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Route settings for the stage.</p>
    #[serde(rename = "RouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<::std::collections::HashMap<String, RouteSettings>>,
    /// <p>The stage name.</p>
    #[serde(rename = "StageName")]
    pub stage_name: String,
    /// <p>A map that defines the stage variables for a Stage. Variable names
    /// can have alphanumeric and underscore characters, and the values must match
    /// [A-Za-z0-9-._~:/?#&=,]+.</p>
    #[serde(rename = "StageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateStageResponse {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>The identifier of a client certificate for a Stage.</p>
    #[serde(rename = "ClientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>The timestamp when the stage was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>Default route settings for the stage.</p>
    #[serde(rename = "DefaultRouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<RouteSettings>,
    /// <p>The identifier of the Deployment that the Stage is
    /// associated with.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The description of the stage.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The timestamp when the stage was last updated.</p>
    #[serde(rename = "LastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>Route settings for the stage.</p>
    #[serde(rename = "RouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<::std::collections::HashMap<String, RouteSettings>>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "StageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    /// <p>A map that defines the stage variables for a stage resource. Variable names can
    /// have alphanumeric and underscore characters, and the values must match
    /// [A-Za-z0-9-._~:/?#&=,]+.</p>
    #[serde(rename = "StageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
}

/// Errors returned by CreateApi
#[derive(Debug, PartialEq)]
pub enum CreateApiError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl CreateApiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateApiError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateApiError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateApiError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateApiError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApiError {
    fn description(&self) -> &str {
        match *self {
            CreateApiError::BadRequest(ref cause) => cause,
            CreateApiError::Conflict(ref cause) => cause,
            CreateApiError::NotFound(ref cause) => cause,
            CreateApiError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateApiMapping
#[derive(Debug, PartialEq)]
pub enum CreateApiMappingError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl CreateApiMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApiMappingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateApiMappingError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateApiMappingError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateApiMappingError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateApiMappingError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateApiMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApiMappingError {
    fn description(&self) -> &str {
        match *self {
            CreateApiMappingError::BadRequest(ref cause) => cause,
            CreateApiMappingError::Conflict(ref cause) => cause,
            CreateApiMappingError::NotFound(ref cause) => cause,
            CreateApiMappingError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAuthorizer
#[derive(Debug, PartialEq)]
pub enum CreateAuthorizerError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl CreateAuthorizerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAuthorizerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateAuthorizerError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateAuthorizerError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateAuthorizerError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateAuthorizerError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            CreateAuthorizerError::Conflict(ref cause) => cause,
            CreateAuthorizerError::NotFound(ref cause) => cause,
            CreateAuthorizerError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDeployment
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl CreateDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDeploymentError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateDeploymentError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDeploymentError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateDeploymentError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            CreateDeploymentError::NotFound(ref cause) => cause,
            CreateDeploymentError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDomainName
#[derive(Debug, PartialEq)]
pub enum CreateDomainNameError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl CreateDomainNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDomainNameError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDomainNameError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateDomainNameError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDomainNameError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateDomainNameError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            CreateDomainNameError::NotFound(ref cause) => cause,
            CreateDomainNameError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateIntegration
#[derive(Debug, PartialEq)]
pub enum CreateIntegrationError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl CreateIntegrationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIntegrationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateIntegrationError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateIntegrationError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateIntegrationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateIntegrationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateIntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateIntegrationError {
    fn description(&self) -> &str {
        match *self {
            CreateIntegrationError::BadRequest(ref cause) => cause,
            CreateIntegrationError::Conflict(ref cause) => cause,
            CreateIntegrationError::NotFound(ref cause) => cause,
            CreateIntegrationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum CreateIntegrationResponseError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl CreateIntegrationResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIntegrationResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateIntegrationResponseError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateIntegrationResponseError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateIntegrationResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateIntegrationResponseError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateIntegrationResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateIntegrationResponseError {
    fn description(&self) -> &str {
        match *self {
            CreateIntegrationResponseError::BadRequest(ref cause) => cause,
            CreateIntegrationResponseError::Conflict(ref cause) => cause,
            CreateIntegrationResponseError::NotFound(ref cause) => cause,
            CreateIntegrationResponseError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateModel
#[derive(Debug, PartialEq)]
pub enum CreateModelError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl CreateModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateModelError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateModelError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateModelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateModelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            CreateModelError::NotFound(ref cause) => cause,
            CreateModelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRoute
#[derive(Debug, PartialEq)]
pub enum CreateRouteError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl CreateRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateRouteError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateRouteError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateRouteError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateRouteError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateRouteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRouteError {
    fn description(&self) -> &str {
        match *self {
            CreateRouteError::BadRequest(ref cause) => cause,
            CreateRouteError::Conflict(ref cause) => cause,
            CreateRouteError::NotFound(ref cause) => cause,
            CreateRouteError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRouteResponse
#[derive(Debug, PartialEq)]
pub enum CreateRouteResponseError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl CreateRouteResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRouteResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateRouteResponseError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateRouteResponseError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateRouteResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateRouteResponseError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateRouteResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRouteResponseError {
    fn description(&self) -> &str {
        match *self {
            CreateRouteResponseError::BadRequest(ref cause) => cause,
            CreateRouteResponseError::Conflict(ref cause) => cause,
            CreateRouteResponseError::NotFound(ref cause) => cause,
            CreateRouteResponseError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStage
#[derive(Debug, PartialEq)]
pub enum CreateStageError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl CreateStageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateStageError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateStageError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateStageError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateStageError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            CreateStageError::NotFound(ref cause) => cause,
            CreateStageError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApi
#[derive(Debug, PartialEq)]
pub enum DeleteApiError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl DeleteApiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteApiError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteApiError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApiError {
    fn description(&self) -> &str {
        match *self {
            DeleteApiError::NotFound(ref cause) => cause,
            DeleteApiError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApiMapping
#[derive(Debug, PartialEq)]
pub enum DeleteApiMappingError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl DeleteApiMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApiMappingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteApiMappingError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteApiMappingError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteApiMappingError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteApiMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApiMappingError {
    fn description(&self) -> &str {
        match *self {
            DeleteApiMappingError::BadRequest(ref cause) => cause,
            DeleteApiMappingError::NotFound(ref cause) => cause,
            DeleteApiMappingError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAuthorizer
#[derive(Debug, PartialEq)]
pub enum DeleteAuthorizerError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl DeleteAuthorizerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAuthorizerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAuthorizerError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteAuthorizerError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteAuthorizerError::NotFound(ref cause) => cause,
            DeleteAuthorizerError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDeployment
#[derive(Debug, PartialEq)]
pub enum DeleteDeploymentError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl DeleteDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDeploymentError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteDeploymentError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteDeploymentError::NotFound(ref cause) => cause,
            DeleteDeploymentError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDomainName
#[derive(Debug, PartialEq)]
pub enum DeleteDomainNameError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl DeleteDomainNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDomainNameError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDomainNameError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteDomainNameError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteIntegration
#[derive(Debug, PartialEq)]
pub enum DeleteIntegrationError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl DeleteIntegrationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIntegrationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteIntegrationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteIntegrationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteIntegrationError::NotFound(ref cause) => cause,
            DeleteIntegrationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum DeleteIntegrationResponseError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl DeleteIntegrationResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIntegrationResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteIntegrationResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteIntegrationResponseError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteIntegrationResponseError::NotFound(ref cause) => cause,
            DeleteIntegrationResponseError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteModel
#[derive(Debug, PartialEq)]
pub enum DeleteModelError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl DeleteModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteModelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteModelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteModelError::NotFound(ref cause) => cause,
            DeleteModelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRoute
#[derive(Debug, PartialEq)]
pub enum DeleteRouteError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl DeleteRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRouteError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteRouteError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRouteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRouteError {
    fn description(&self) -> &str {
        match *self {
            DeleteRouteError::NotFound(ref cause) => cause,
            DeleteRouteError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRouteResponse
#[derive(Debug, PartialEq)]
pub enum DeleteRouteResponseError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl DeleteRouteResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRouteResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRouteResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteRouteResponseError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRouteResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRouteResponseError {
    fn description(&self) -> &str {
        match *self {
            DeleteRouteResponseError::NotFound(ref cause) => cause,
            DeleteRouteResponseError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteStage
#[derive(Debug, PartialEq)]
pub enum DeleteStageError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl DeleteStageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteStageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteStageError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteStageError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteStageError::NotFound(ref cause) => cause,
            DeleteStageError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApi
#[derive(Debug, PartialEq)]
pub enum GetApiError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetApiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => return RusotoError::Service(GetApiError::NotFound(err.msg)),
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApiError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApiError {
    fn description(&self) -> &str {
        match *self {
            GetApiError::NotFound(ref cause) => cause,
            GetApiError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApiMapping
#[derive(Debug, PartialEq)]
pub enum GetApiMappingError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetApiMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApiMappingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetApiMappingError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetApiMappingError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApiMappingError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetApiMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApiMappingError {
    fn description(&self) -> &str {
        match *self {
            GetApiMappingError::BadRequest(ref cause) => cause,
            GetApiMappingError::NotFound(ref cause) => cause,
            GetApiMappingError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApiMappings
#[derive(Debug, PartialEq)]
pub enum GetApiMappingsError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetApiMappingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApiMappingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetApiMappingsError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetApiMappingsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApiMappingsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetApiMappingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApiMappingsError {
    fn description(&self) -> &str {
        match *self {
            GetApiMappingsError::BadRequest(ref cause) => cause,
            GetApiMappingsError::NotFound(ref cause) => cause,
            GetApiMappingsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApis
#[derive(Debug, PartialEq)]
pub enum GetApisError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetApisError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApisError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetApisError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetApisError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApisError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetApisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApisError {
    fn description(&self) -> &str {
        match *self {
            GetApisError::BadRequest(ref cause) => cause,
            GetApisError::NotFound(ref cause) => cause,
            GetApisError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAuthorizer
#[derive(Debug, PartialEq)]
pub enum GetAuthorizerError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetAuthorizerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAuthorizerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetAuthorizerError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetAuthorizerError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetAuthorizers
#[derive(Debug, PartialEq)]
pub enum GetAuthorizersError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetAuthorizersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAuthorizersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetAuthorizersError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetAuthorizersError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetAuthorizersError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetDeployment
#[derive(Debug, PartialEq)]
pub enum GetDeploymentError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetDeploymentError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDeploymentError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            GetDeploymentError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeployments
#[derive(Debug, PartialEq)]
pub enum GetDeploymentsError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetDeploymentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeploymentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDeploymentsError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDeploymentsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDeploymentsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            GetDeploymentsError::NotFound(ref cause) => cause,
            GetDeploymentsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDomainName
#[derive(Debug, PartialEq)]
pub enum GetDomainNameError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetDomainNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainNameError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetDomainNameError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDomainNameError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            GetDomainNameError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDomainNames
#[derive(Debug, PartialEq)]
pub enum GetDomainNamesError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetDomainNamesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainNamesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDomainNamesError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDomainNamesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDomainNamesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            GetDomainNamesError::NotFound(ref cause) => cause,
            GetDomainNamesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIntegration
#[derive(Debug, PartialEq)]
pub enum GetIntegrationError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetIntegrationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIntegrationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetIntegrationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetIntegrationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum GetIntegrationResponseError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetIntegrationResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIntegrationResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetIntegrationResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetIntegrationResponseError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetIntegrationResponses
#[derive(Debug, PartialEq)]
pub enum GetIntegrationResponsesError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetIntegrationResponsesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIntegrationResponsesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetIntegrationResponsesError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetIntegrationResponsesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetIntegrationResponsesError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetIntegrationResponsesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIntegrationResponsesError {
    fn description(&self) -> &str {
        match *self {
            GetIntegrationResponsesError::BadRequest(ref cause) => cause,
            GetIntegrationResponsesError::NotFound(ref cause) => cause,
            GetIntegrationResponsesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIntegrations
#[derive(Debug, PartialEq)]
pub enum GetIntegrationsError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetIntegrationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIntegrationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetIntegrationsError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetIntegrationsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetIntegrationsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetIntegrationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIntegrationsError {
    fn description(&self) -> &str {
        match *self {
            GetIntegrationsError::BadRequest(ref cause) => cause,
            GetIntegrationsError::NotFound(ref cause) => cause,
            GetIntegrationsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetModel
#[derive(Debug, PartialEq)]
pub enum GetModelError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetModelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetModelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetModelTemplate
#[derive(Debug, PartialEq)]
pub enum GetModelTemplateError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetModelTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetModelTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetModelTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetModelTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            GetModelTemplateError::NotFound(ref cause) => cause,
            GetModelTemplateError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetModels
#[derive(Debug, PartialEq)]
pub enum GetModelsError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetModelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetModelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetModelsError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetModelsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetModelsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetRoute
#[derive(Debug, PartialEq)]
pub enum GetRouteError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetRouteError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetRouteError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRouteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRouteError {
    fn description(&self) -> &str {
        match *self {
            GetRouteError::NotFound(ref cause) => cause,
            GetRouteError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRouteResponse
#[derive(Debug, PartialEq)]
pub enum GetRouteResponseError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetRouteResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRouteResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetRouteResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetRouteResponseError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRouteResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRouteResponseError {
    fn description(&self) -> &str {
        match *self {
            GetRouteResponseError::NotFound(ref cause) => cause,
            GetRouteResponseError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRouteResponses
#[derive(Debug, PartialEq)]
pub enum GetRouteResponsesError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetRouteResponsesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRouteResponsesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetRouteResponsesError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRouteResponsesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetRouteResponsesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRouteResponsesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRouteResponsesError {
    fn description(&self) -> &str {
        match *self {
            GetRouteResponsesError::BadRequest(ref cause) => cause,
            GetRouteResponsesError::NotFound(ref cause) => cause,
            GetRouteResponsesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRoutes
#[derive(Debug, PartialEq)]
pub enum GetRoutesError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetRoutesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRoutesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetRoutesError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRoutesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetRoutesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRoutesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRoutesError {
    fn description(&self) -> &str {
        match *self {
            GetRoutesError::BadRequest(ref cause) => cause,
            GetRoutesError::NotFound(ref cause) => cause,
            GetRoutesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetStage
#[derive(Debug, PartialEq)]
pub enum GetStageError {
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetStageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetStageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetStageError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetStageError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetStages
#[derive(Debug, PartialEq)]
pub enum GetStagesError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetStagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetStagesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetStagesError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetStagesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetStagesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            GetStagesError::BadRequest(ref cause) => cause,
            GetStagesError::NotFound(ref cause) => cause,
            GetStagesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApi
#[derive(Debug, PartialEq)]
pub enum UpdateApiError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl UpdateApiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateApiError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateApiError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateApiError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateApiError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApiError {
    fn description(&self) -> &str {
        match *self {
            UpdateApiError::BadRequest(ref cause) => cause,
            UpdateApiError::Conflict(ref cause) => cause,
            UpdateApiError::NotFound(ref cause) => cause,
            UpdateApiError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApiMapping
#[derive(Debug, PartialEq)]
pub enum UpdateApiMappingError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl UpdateApiMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApiMappingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateApiMappingError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateApiMappingError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateApiMappingError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateApiMappingError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateApiMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApiMappingError {
    fn description(&self) -> &str {
        match *self {
            UpdateApiMappingError::BadRequest(ref cause) => cause,
            UpdateApiMappingError::Conflict(ref cause) => cause,
            UpdateApiMappingError::NotFound(ref cause) => cause,
            UpdateApiMappingError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAuthorizer
#[derive(Debug, PartialEq)]
pub enum UpdateAuthorizerError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl UpdateAuthorizerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAuthorizerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateAuthorizerError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateAuthorizerError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateAuthorizerError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateAuthorizerError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            UpdateAuthorizerError::Conflict(ref cause) => cause,
            UpdateAuthorizerError::NotFound(ref cause) => cause,
            UpdateAuthorizerError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDeployment
#[derive(Debug, PartialEq)]
pub enum UpdateDeploymentError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl UpdateDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateDeploymentError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateDeploymentError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDeploymentError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateDeploymentError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            UpdateDeploymentError::Conflict(ref cause) => cause,
            UpdateDeploymentError::NotFound(ref cause) => cause,
            UpdateDeploymentError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDomainName
#[derive(Debug, PartialEq)]
pub enum UpdateDomainNameError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl UpdateDomainNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDomainNameError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateDomainNameError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateDomainNameError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDomainNameError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateDomainNameError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateIntegration
#[derive(Debug, PartialEq)]
pub enum UpdateIntegrationError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl UpdateIntegrationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateIntegrationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateIntegrationError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateIntegrationError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateIntegrationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateIntegrationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum UpdateIntegrationResponseError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl UpdateIntegrationResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateIntegrationResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateIntegrationResponseError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateIntegrationResponseError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateIntegrationResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateIntegrationResponseError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateModel
#[derive(Debug, PartialEq)]
pub enum UpdateModelError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl UpdateModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateModelError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateModelError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateModelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateModelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateRoute
#[derive(Debug, PartialEq)]
pub enum UpdateRouteError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl UpdateRouteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRouteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateRouteError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateRouteError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRouteError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateRouteError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateRouteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRouteError {
    fn description(&self) -> &str {
        match *self {
            UpdateRouteError::BadRequest(ref cause) => cause,
            UpdateRouteError::Conflict(ref cause) => cause,
            UpdateRouteError::NotFound(ref cause) => cause,
            UpdateRouteError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRouteResponse
#[derive(Debug, PartialEq)]
pub enum UpdateRouteResponseError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl UpdateRouteResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRouteResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateRouteResponseError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateRouteResponseError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRouteResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateRouteResponseError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateRouteResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRouteResponseError {
    fn description(&self) -> &str {
        match *self {
            UpdateRouteResponseError::BadRequest(ref cause) => cause,
            UpdateRouteResponseError::Conflict(ref cause) => cause,
            UpdateRouteResponseError::NotFound(ref cause) => cause,
            UpdateRouteResponseError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateStage
#[derive(Debug, PartialEq)]
pub enum UpdateStageError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See
    /// the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service
    /// resource associated with the request. Resolve the conflict before retrying this
    /// request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message
    /// field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl UpdateStageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateStageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateStageError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateStageError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateStageError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateStageError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Trait representing the capabilities of the AmazonApiGatewayV2 API. AmazonApiGatewayV2 clients implement this trait.
pub trait ApiGatewayV2 {
    /// <p>Creates an Api resource.</p>
    fn create_api(
        &self,
        input: CreateApiRequest,
    ) -> RusotoFuture<CreateApiResponse, CreateApiError>;

    /// <p>Creates an API mapping.</p>
    fn create_api_mapping(
        &self,
        input: CreateApiMappingRequest,
    ) -> RusotoFuture<CreateApiMappingResponse, CreateApiMappingError>;

    /// <p>Creates an Authorizer for an API.</p>
    fn create_authorizer(
        &self,
        input: CreateAuthorizerRequest,
    ) -> RusotoFuture<CreateAuthorizerResponse, CreateAuthorizerError>;

    /// <p>Creates a Deployment for an API.</p>
    fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> RusotoFuture<CreateDeploymentResponse, CreateDeploymentError>;

    /// <p>Creates a domain name.</p>
    fn create_domain_name(
        &self,
        input: CreateDomainNameRequest,
    ) -> RusotoFuture<CreateDomainNameResponse, CreateDomainNameError>;

    /// <p>Creates an Integration.</p>
    fn create_integration(
        &self,
        input: CreateIntegrationRequest,
    ) -> RusotoFuture<CreateIntegrationResponse, CreateIntegrationError>;

    /// <p>Creates an IntegrationResponses.</p>
    fn create_integration_response(
        &self,
        input: CreateIntegrationResponseRequest,
    ) -> RusotoFuture<CreateIntegrationResponseResponse, CreateIntegrationResponseError>;

    /// <p>Creates a Model for an API.</p>
    fn create_model(
        &self,
        input: CreateModelRequest,
    ) -> RusotoFuture<CreateModelResponse, CreateModelError>;

    /// <p>Creates a Route for an API.</p>
    fn create_route(
        &self,
        input: CreateRouteRequest,
    ) -> RusotoFuture<CreateRouteResponse, CreateRouteError>;

    /// <p>Creates a RouteResponse for a Route.</p>
    fn create_route_response(
        &self,
        input: CreateRouteResponseRequest,
    ) -> RusotoFuture<CreateRouteResponseResponse, CreateRouteResponseError>;

    /// <p>Creates a Stage for an API.</p>
    fn create_stage(
        &self,
        input: CreateStageRequest,
    ) -> RusotoFuture<CreateStageResponse, CreateStageError>;

    /// <p>Deletes an Api resource.</p>
    fn delete_api(&self, input: DeleteApiRequest) -> RusotoFuture<(), DeleteApiError>;

    /// <p>Deletes an API mapping.</p>
    fn delete_api_mapping(
        &self,
        input: DeleteApiMappingRequest,
    ) -> RusotoFuture<(), DeleteApiMappingError>;

    /// <p>Deletes an Authorizer.</p>
    fn delete_authorizer(
        &self,
        input: DeleteAuthorizerRequest,
    ) -> RusotoFuture<(), DeleteAuthorizerError>;

    /// <p>Deletes a Deployment.</p>
    fn delete_deployment(
        &self,
        input: DeleteDeploymentRequest,
    ) -> RusotoFuture<(), DeleteDeploymentError>;

    /// <p>Deletes a domain name.</p>
    fn delete_domain_name(
        &self,
        input: DeleteDomainNameRequest,
    ) -> RusotoFuture<(), DeleteDomainNameError>;

    /// <p>Deletes an Integration.</p>
    fn delete_integration(
        &self,
        input: DeleteIntegrationRequest,
    ) -> RusotoFuture<(), DeleteIntegrationError>;

    /// <p>Deletes an IntegrationResponses.</p>
    fn delete_integration_response(
        &self,
        input: DeleteIntegrationResponseRequest,
    ) -> RusotoFuture<(), DeleteIntegrationResponseError>;

    /// <p>Deletes a Model.</p>
    fn delete_model(&self, input: DeleteModelRequest) -> RusotoFuture<(), DeleteModelError>;

    /// <p>Deletes a Route.</p>
    fn delete_route(&self, input: DeleteRouteRequest) -> RusotoFuture<(), DeleteRouteError>;

    /// <p>Deletes a RouteResponse.</p>
    fn delete_route_response(
        &self,
        input: DeleteRouteResponseRequest,
    ) -> RusotoFuture<(), DeleteRouteResponseError>;

    /// <p>Deletes a Stage.</p>
    fn delete_stage(&self, input: DeleteStageRequest) -> RusotoFuture<(), DeleteStageError>;

    /// <p>Gets an Api resource.</p>
    fn get_api(&self, input: GetApiRequest) -> RusotoFuture<GetApiResponse, GetApiError>;

    /// <p>The API mapping.</p>
    fn get_api_mapping(
        &self,
        input: GetApiMappingRequest,
    ) -> RusotoFuture<GetApiMappingResponse, GetApiMappingError>;

    /// <p>The API mappings.</p>
    fn get_api_mappings(
        &self,
        input: GetApiMappingsRequest,
    ) -> RusotoFuture<GetApiMappingsResponse, GetApiMappingsError>;

    /// <p>Gets a collection of Api resources.</p>
    fn get_apis(&self, input: GetApisRequest) -> RusotoFuture<GetApisResponse, GetApisError>;

    /// <p>Gets an Authorizer.</p>
    fn get_authorizer(
        &self,
        input: GetAuthorizerRequest,
    ) -> RusotoFuture<GetAuthorizerResponse, GetAuthorizerError>;

    /// <p>Gets the Authorizers for an API.</p>
    fn get_authorizers(
        &self,
        input: GetAuthorizersRequest,
    ) -> RusotoFuture<GetAuthorizersResponse, GetAuthorizersError>;

    /// <p>Gets a Deployment.</p>
    fn get_deployment(
        &self,
        input: GetDeploymentRequest,
    ) -> RusotoFuture<GetDeploymentResponse, GetDeploymentError>;

    /// <p>Gets the Deployments for an API.</p>
    fn get_deployments(
        &self,
        input: GetDeploymentsRequest,
    ) -> RusotoFuture<GetDeploymentsResponse, GetDeploymentsError>;

    /// <p>Gets a domain name.</p>
    fn get_domain_name(
        &self,
        input: GetDomainNameRequest,
    ) -> RusotoFuture<GetDomainNameResponse, GetDomainNameError>;

    /// <p>Gets the domain names for an AWS account.</p>
    fn get_domain_names(
        &self,
        input: GetDomainNamesRequest,
    ) -> RusotoFuture<GetDomainNamesResponse, GetDomainNamesError>;

    /// <p>Gets an Integration.</p>
    fn get_integration(
        &self,
        input: GetIntegrationRequest,
    ) -> RusotoFuture<GetIntegrationResponse, GetIntegrationError>;

    /// <p>Gets an IntegrationResponses.</p>
    fn get_integration_response(
        &self,
        input: GetIntegrationResponseRequest,
    ) -> RusotoFuture<GetIntegrationResponseResponse, GetIntegrationResponseError>;

    /// <p>Gets the IntegrationResponses for an Integration.</p>
    fn get_integration_responses(
        &self,
        input: GetIntegrationResponsesRequest,
    ) -> RusotoFuture<GetIntegrationResponsesResponse, GetIntegrationResponsesError>;

    /// <p>Gets the Integrations for an API.</p>
    fn get_integrations(
        &self,
        input: GetIntegrationsRequest,
    ) -> RusotoFuture<GetIntegrationsResponse, GetIntegrationsError>;

    /// <p>Gets a Model.</p>
    fn get_model(&self, input: GetModelRequest) -> RusotoFuture<GetModelResponse, GetModelError>;

    /// <p>Gets a model template.</p>
    fn get_model_template(
        &self,
        input: GetModelTemplateRequest,
    ) -> RusotoFuture<GetModelTemplateResponse, GetModelTemplateError>;

    /// <p>Gets the Models for an API.</p>
    fn get_models(
        &self,
        input: GetModelsRequest,
    ) -> RusotoFuture<GetModelsResponse, GetModelsError>;

    /// <p>Gets a Route.</p>
    fn get_route(&self, input: GetRouteRequest) -> RusotoFuture<GetRouteResponse, GetRouteError>;

    /// <p>Gets a RouteResponse.</p>
    fn get_route_response(
        &self,
        input: GetRouteResponseRequest,
    ) -> RusotoFuture<GetRouteResponseResponse, GetRouteResponseError>;

    /// <p>Gets the RouteResponses for a Route.</p>
    fn get_route_responses(
        &self,
        input: GetRouteResponsesRequest,
    ) -> RusotoFuture<GetRouteResponsesResponse, GetRouteResponsesError>;

    /// <p>Gets the Routes for an API.</p>
    fn get_routes(
        &self,
        input: GetRoutesRequest,
    ) -> RusotoFuture<GetRoutesResponse, GetRoutesError>;

    /// <p>Gets a Stage.</p>
    fn get_stage(&self, input: GetStageRequest) -> RusotoFuture<GetStageResponse, GetStageError>;

    /// <p>Gets the Stages for an API.</p>
    fn get_stages(
        &self,
        input: GetStagesRequest,
    ) -> RusotoFuture<GetStagesResponse, GetStagesError>;

    /// <p>Updates an Api resource.</p>
    fn update_api(
        &self,
        input: UpdateApiRequest,
    ) -> RusotoFuture<UpdateApiResponse, UpdateApiError>;

    /// <p>The API mapping.</p>
    fn update_api_mapping(
        &self,
        input: UpdateApiMappingRequest,
    ) -> RusotoFuture<UpdateApiMappingResponse, UpdateApiMappingError>;

    /// <p>Updates an Authorizer.</p>
    fn update_authorizer(
        &self,
        input: UpdateAuthorizerRequest,
    ) -> RusotoFuture<UpdateAuthorizerResponse, UpdateAuthorizerError>;

    /// <p>Updates a Deployment.</p>
    fn update_deployment(
        &self,
        input: UpdateDeploymentRequest,
    ) -> RusotoFuture<UpdateDeploymentResponse, UpdateDeploymentError>;

    /// <p>Updates a domain name.</p>
    fn update_domain_name(
        &self,
        input: UpdateDomainNameRequest,
    ) -> RusotoFuture<UpdateDomainNameResponse, UpdateDomainNameError>;

    /// <p>Updates an Integration.</p>
    fn update_integration(
        &self,
        input: UpdateIntegrationRequest,
    ) -> RusotoFuture<UpdateIntegrationResponse, UpdateIntegrationError>;

    /// <p>Updates an IntegrationResponses.</p>
    fn update_integration_response(
        &self,
        input: UpdateIntegrationResponseRequest,
    ) -> RusotoFuture<UpdateIntegrationResponseResponse, UpdateIntegrationResponseError>;

    /// <p>Updates a Model.</p>
    fn update_model(
        &self,
        input: UpdateModelRequest,
    ) -> RusotoFuture<UpdateModelResponse, UpdateModelError>;

    /// <p>Updates a Route.</p>
    fn update_route(
        &self,
        input: UpdateRouteRequest,
    ) -> RusotoFuture<UpdateRouteResponse, UpdateRouteError>;

    /// <p>Updates a RouteResponse.</p>
    fn update_route_response(
        &self,
        input: UpdateRouteResponseRequest,
    ) -> RusotoFuture<UpdateRouteResponseResponse, UpdateRouteResponseError>;

    /// <p>Updates a Stage.</p>
    fn update_stage(
        &self,
        input: UpdateStageRequest,
    ) -> RusotoFuture<UpdateStageResponse, UpdateStageError>;
}
/// A client for the AmazonApiGatewayV2 API.
#[derive(Clone)]
pub struct ApiGatewayV2Client {
    client: Client,
    region: region::Region,
}

impl ApiGatewayV2Client {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ApiGatewayV2Client {
        ApiGatewayV2Client {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ApiGatewayV2Client
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + CompressRequestPayload + Send + Sync + 'static,
        D::Future: Send,
    {
        ApiGatewayV2Client {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl ApiGatewayV2 for ApiGatewayV2Client {
    /// <p>Creates an Api resource.</p>
    fn create_api(
        &self,
        input: CreateApiRequest,
    ) -> RusotoFuture<CreateApiResponse, CreateApiError> {
        let request_uri = "/v2/apis";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateApiResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an API mapping.</p>
    fn create_api_mapping(
        &self,
        input: CreateApiMappingRequest,
    ) -> RusotoFuture<CreateApiMappingResponse, CreateApiMappingError> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}/apimappings",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateApiMappingResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateApiMappingError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an Authorizer for an API.</p>
    fn create_authorizer(
        &self,
        input: CreateAuthorizerRequest,
    ) -> RusotoFuture<CreateAuthorizerResponse, CreateAuthorizerError> {
        let request_uri = format!("/v2/apis/{api_id}/authorizers", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAuthorizerResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAuthorizerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a Deployment for an API.</p>
    fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> RusotoFuture<CreateDeploymentResponse, CreateDeploymentError> {
        let request_uri = format!("/v2/apis/{api_id}/deployments", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDeploymentResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDeploymentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a domain name.</p>
    fn create_domain_name(
        &self,
        input: CreateDomainNameRequest,
    ) -> RusotoFuture<CreateDomainNameResponse, CreateDomainNameError> {
        let request_uri = "/v2/domainnames";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDomainNameResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDomainNameError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an Integration.</p>
    fn create_integration(
        &self,
        input: CreateIntegrationRequest,
    ) -> RusotoFuture<CreateIntegrationResponse, CreateIntegrationError> {
        let request_uri = format!("/v2/apis/{api_id}/integrations", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateIntegrationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateIntegrationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an IntegrationResponses.</p>
    fn create_integration_response(
        &self,
        input: CreateIntegrationResponseRequest,
    ) -> RusotoFuture<CreateIntegrationResponseResponse, CreateIntegrationResponseError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/integrations/{integration_id}/integrationresponses",
            api_id = input.api_id,
            integration_id = input.integration_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateIntegrationResponseResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateIntegrationResponseError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a Model for an API.</p>
    fn create_model(
        &self,
        input: CreateModelRequest,
    ) -> RusotoFuture<CreateModelResponse, CreateModelError> {
        let request_uri = format!("/v2/apis/{api_id}/models", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateModelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateModelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a Route for an API.</p>
    fn create_route(
        &self,
        input: CreateRouteRequest,
    ) -> RusotoFuture<CreateRouteResponse, CreateRouteError> {
        let request_uri = format!("/v2/apis/{api_id}/routes", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateRouteResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateRouteError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a RouteResponse for a Route.</p>
    fn create_route_response(
        &self,
        input: CreateRouteResponseRequest,
    ) -> RusotoFuture<CreateRouteResponseResponse, CreateRouteResponseError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}/routeresponses",
            api_id = input.api_id,
            route_id = input.route_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateRouteResponseResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateRouteResponseError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a Stage for an API.</p>
    fn create_stage(
        &self,
        input: CreateStageRequest,
    ) -> RusotoFuture<CreateStageResponse, CreateStageError> {
        let request_uri = format!("/v2/apis/{api_id}/stages", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateStageResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateStageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an Api resource.</p>
    fn delete_api(&self, input: DeleteApiRequest) -> RusotoFuture<(), DeleteApiError> {
        let request_uri = format!("/v2/apis/{api_id}", api_id = input.api_id);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an API mapping.</p>
    fn delete_api_mapping(
        &self,
        input: DeleteApiMappingRequest,
    ) -> RusotoFuture<(), DeleteApiMappingError> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}/apimappings/{api_mapping_id}",
            api_mapping_id = input.api_mapping_id,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteApiMappingError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an Authorizer.</p>
    fn delete_authorizer(
        &self,
        input: DeleteAuthorizerRequest,
    ) -> RusotoFuture<(), DeleteAuthorizerError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/authorizers/{authorizer_id}",
            api_id = input.api_id,
            authorizer_id = input.authorizer_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAuthorizerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a Deployment.</p>
    fn delete_deployment(
        &self,
        input: DeleteDeploymentRequest,
    ) -> RusotoFuture<(), DeleteDeploymentError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/deployments/{deployment_id}",
            api_id = input.api_id,
            deployment_id = input.deployment_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDeploymentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a domain name.</p>
    fn delete_domain_name(
        &self,
        input: DeleteDomainNameRequest,
    ) -> RusotoFuture<(), DeleteDomainNameError> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDomainNameError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an Integration.</p>
    fn delete_integration(
        &self,
        input: DeleteIntegrationRequest,
    ) -> RusotoFuture<(), DeleteIntegrationError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/integrations/{integration_id}",
            api_id = input.api_id,
            integration_id = input.integration_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteIntegrationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an IntegrationResponses.</p>
    fn delete_integration_response(
        &self,
        input: DeleteIntegrationResponseRequest,
    ) -> RusotoFuture<(), DeleteIntegrationResponseError> {
        let request_uri = format!("/v2/apis/{api_id}/integrations/{integration_id}/integrationresponses/{integration_response_id}", api_id = input.api_id, integration_id = input.integration_id, integration_response_id = input.integration_response_id);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteIntegrationResponseError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a Model.</p>
    fn delete_model(&self, input: DeleteModelRequest) -> RusotoFuture<(), DeleteModelError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/models/{model_id}",
            api_id = input.api_id,
            model_id = input.model_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteModelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a Route.</p>
    fn delete_route(&self, input: DeleteRouteRequest) -> RusotoFuture<(), DeleteRouteError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}",
            api_id = input.api_id,
            route_id = input.route_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteRouteError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a RouteResponse.</p>
    fn delete_route_response(
        &self,
        input: DeleteRouteResponseRequest,
    ) -> RusotoFuture<(), DeleteRouteResponseError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}/routeresponses/{route_response_id}",
            api_id = input.api_id,
            route_id = input.route_id,
            route_response_id = input.route_response_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteRouteResponseError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a Stage.</p>
    fn delete_stage(&self, input: DeleteStageRequest) -> RusotoFuture<(), DeleteStageError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/stages/{stage_name}",
            api_id = input.api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteStageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets an Api resource.</p>
    fn get_api(&self, input: GetApiRequest) -> RusotoFuture<GetApiResponse, GetApiError> {
        let request_uri = format!("/v2/apis/{api_id}", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetApiResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>The API mapping.</p>
    fn get_api_mapping(
        &self,
        input: GetApiMappingRequest,
    ) -> RusotoFuture<GetApiMappingResponse, GetApiMappingError> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}/apimappings/{api_mapping_id}",
            api_mapping_id = input.api_mapping_id,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetApiMappingResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetApiMappingError::from_response(response))),
                )
            }
        })
    }

    /// <p>The API mappings.</p>
    fn get_api_mappings(
        &self,
        input: GetApiMappingsRequest,
    ) -> RusotoFuture<GetApiMappingsResponse, GetApiMappingsError> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}/apimappings",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetApiMappingsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetApiMappingsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a collection of Api resources.</p>
    fn get_apis(&self, input: GetApisRequest) -> RusotoFuture<GetApisResponse, GetApisError> {
        let request_uri = "/v2/apis";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetApisResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetApisError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets an Authorizer.</p>
    fn get_authorizer(
        &self,
        input: GetAuthorizerRequest,
    ) -> RusotoFuture<GetAuthorizerResponse, GetAuthorizerError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/authorizers/{authorizer_id}",
            api_id = input.api_id,
            authorizer_id = input.authorizer_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAuthorizerResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAuthorizerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the Authorizers for an API.</p>
    fn get_authorizers(
        &self,
        input: GetAuthorizersRequest,
    ) -> RusotoFuture<GetAuthorizersResponse, GetAuthorizersError> {
        let request_uri = format!("/v2/apis/{api_id}/authorizers", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAuthorizersResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAuthorizersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a Deployment.</p>
    fn get_deployment(
        &self,
        input: GetDeploymentRequest,
    ) -> RusotoFuture<GetDeploymentResponse, GetDeploymentError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/deployments/{deployment_id}",
            api_id = input.api_id,
            deployment_id = input.deployment_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDeploymentResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDeploymentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the Deployments for an API.</p>
    fn get_deployments(
        &self,
        input: GetDeploymentsRequest,
    ) -> RusotoFuture<GetDeploymentsResponse, GetDeploymentsError> {
        let request_uri = format!("/v2/apis/{api_id}/deployments", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDeploymentsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDeploymentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a domain name.</p>
    fn get_domain_name(
        &self,
        input: GetDomainNameRequest,
    ) -> RusotoFuture<GetDomainNameResponse, GetDomainNameError> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDomainNameResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDomainNameError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the domain names for an AWS account.</p>
    fn get_domain_names(
        &self,
        input: GetDomainNamesRequest,
    ) -> RusotoFuture<GetDomainNamesResponse, GetDomainNamesError> {
        let request_uri = "/v2/domainnames";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDomainNamesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDomainNamesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets an Integration.</p>
    fn get_integration(
        &self,
        input: GetIntegrationRequest,
    ) -> RusotoFuture<GetIntegrationResponse, GetIntegrationError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/integrations/{integration_id}",
            api_id = input.api_id,
            integration_id = input.integration_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetIntegrationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetIntegrationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets an IntegrationResponses.</p>
    fn get_integration_response(
        &self,
        input: GetIntegrationResponseRequest,
    ) -> RusotoFuture<GetIntegrationResponseResponse, GetIntegrationResponseError> {
        let request_uri = format!("/v2/apis/{api_id}/integrations/{integration_id}/integrationresponses/{integration_response_id}", api_id = input.api_id, integration_id = input.integration_id, integration_response_id = input.integration_response_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetIntegrationResponseResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetIntegrationResponseError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets the IntegrationResponses for an Integration.</p>
    fn get_integration_responses(
        &self,
        input: GetIntegrationResponsesRequest,
    ) -> RusotoFuture<GetIntegrationResponsesResponse, GetIntegrationResponsesError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/integrations/{integration_id}/integrationresponses",
            api_id = input.api_id,
            integration_id = input.integration_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetIntegrationResponsesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetIntegrationResponsesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets the Integrations for an API.</p>
    fn get_integrations(
        &self,
        input: GetIntegrationsRequest,
    ) -> RusotoFuture<GetIntegrationsResponse, GetIntegrationsError> {
        let request_uri = format!("/v2/apis/{api_id}/integrations", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetIntegrationsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetIntegrationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a Model.</p>
    fn get_model(&self, input: GetModelRequest) -> RusotoFuture<GetModelResponse, GetModelError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/models/{model_id}",
            api_id = input.api_id,
            model_id = input.model_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetModelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetModelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a model template.</p>
    fn get_model_template(
        &self,
        input: GetModelTemplateRequest,
    ) -> RusotoFuture<GetModelTemplateResponse, GetModelTemplateError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/models/{model_id}/template",
            api_id = input.api_id,
            model_id = input.model_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetModelTemplateResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetModelTemplateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the Models for an API.</p>
    fn get_models(
        &self,
        input: GetModelsRequest,
    ) -> RusotoFuture<GetModelsResponse, GetModelsError> {
        let request_uri = format!("/v2/apis/{api_id}/models", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetModelsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetModelsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a Route.</p>
    fn get_route(&self, input: GetRouteRequest) -> RusotoFuture<GetRouteResponse, GetRouteError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}",
            api_id = input.api_id,
            route_id = input.route_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRouteResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRouteError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a RouteResponse.</p>
    fn get_route_response(
        &self,
        input: GetRouteResponseRequest,
    ) -> RusotoFuture<GetRouteResponseResponse, GetRouteResponseError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}/routeresponses/{route_response_id}",
            api_id = input.api_id,
            route_id = input.route_id,
            route_response_id = input.route_response_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRouteResponseResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRouteResponseError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the RouteResponses for a Route.</p>
    fn get_route_responses(
        &self,
        input: GetRouteResponsesRequest,
    ) -> RusotoFuture<GetRouteResponsesResponse, GetRouteResponsesError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}/routeresponses",
            api_id = input.api_id,
            route_id = input.route_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRouteResponsesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRouteResponsesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the Routes for an API.</p>
    fn get_routes(
        &self,
        input: GetRoutesRequest,
    ) -> RusotoFuture<GetRoutesResponse, GetRoutesError> {
        let request_uri = format!("/v2/apis/{api_id}/routes", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRoutesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRoutesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a Stage.</p>
    fn get_stage(&self, input: GetStageRequest) -> RusotoFuture<GetStageResponse, GetStageError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/stages/{stage_name}",
            api_id = input.api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetStageResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetStageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the Stages for an API.</p>
    fn get_stages(
        &self,
        input: GetStagesRequest,
    ) -> RusotoFuture<GetStagesResponse, GetStagesError> {
        let request_uri = format!("/v2/apis/{api_id}/stages", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetStagesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetStagesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an Api resource.</p>
    fn update_api(
        &self,
        input: UpdateApiRequest,
    ) -> RusotoFuture<UpdateApiResponse, UpdateApiError> {
        let request_uri = format!("/v2/apis/{api_id}", api_id = input.api_id);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateApiResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>The API mapping.</p>
    fn update_api_mapping(
        &self,
        input: UpdateApiMappingRequest,
    ) -> RusotoFuture<UpdateApiMappingResponse, UpdateApiMappingError> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}/apimappings/{api_mapping_id}",
            api_mapping_id = input.api_mapping_id,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateApiMappingResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateApiMappingError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an Authorizer.</p>
    fn update_authorizer(
        &self,
        input: UpdateAuthorizerRequest,
    ) -> RusotoFuture<UpdateAuthorizerResponse, UpdateAuthorizerError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/authorizers/{authorizer_id}",
            api_id = input.api_id,
            authorizer_id = input.authorizer_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateAuthorizerResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateAuthorizerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a Deployment.</p>
    fn update_deployment(
        &self,
        input: UpdateDeploymentRequest,
    ) -> RusotoFuture<UpdateDeploymentResponse, UpdateDeploymentError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/deployments/{deployment_id}",
            api_id = input.api_id,
            deployment_id = input.deployment_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDeploymentResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDeploymentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a domain name.</p>
    fn update_domain_name(
        &self,
        input: UpdateDomainNameRequest,
    ) -> RusotoFuture<UpdateDomainNameResponse, UpdateDomainNameError> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDomainNameResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDomainNameError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an Integration.</p>
    fn update_integration(
        &self,
        input: UpdateIntegrationRequest,
    ) -> RusotoFuture<UpdateIntegrationResponse, UpdateIntegrationError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/integrations/{integration_id}",
            api_id = input.api_id,
            integration_id = input.integration_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateIntegrationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateIntegrationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an IntegrationResponses.</p>
    fn update_integration_response(
        &self,
        input: UpdateIntegrationResponseRequest,
    ) -> RusotoFuture<UpdateIntegrationResponseResponse, UpdateIntegrationResponseError> {
        let request_uri = format!("/v2/apis/{api_id}/integrations/{integration_id}/integrationresponses/{integration_response_id}", api_id = input.api_id, integration_id = input.integration_id, integration_response_id = input.integration_response_id);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateIntegrationResponseResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateIntegrationResponseError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates a Model.</p>
    fn update_model(
        &self,
        input: UpdateModelRequest,
    ) -> RusotoFuture<UpdateModelResponse, UpdateModelError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/models/{model_id}",
            api_id = input.api_id,
            model_id = input.model_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateModelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateModelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a Route.</p>
    fn update_route(
        &self,
        input: UpdateRouteRequest,
    ) -> RusotoFuture<UpdateRouteResponse, UpdateRouteError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}",
            api_id = input.api_id,
            route_id = input.route_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateRouteResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateRouteError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a RouteResponse.</p>
    fn update_route_response(
        &self,
        input: UpdateRouteResponseRequest,
    ) -> RusotoFuture<UpdateRouteResponseResponse, UpdateRouteResponseError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}/routeresponses/{route_response_id}",
            api_id = input.api_id,
            route_id = input.route_id,
            route_response_id = input.route_response_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateRouteResponseResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateRouteResponseError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates a Stage.</p>
    fn update_stage(
        &self,
        input: UpdateStageRequest,
    ) -> RusotoFuture<UpdateStageResponse, UpdateStageError> {
        let request_uri = format!(
            "/v2/apis/{api_id}/stages/{stage_name}",
            api_id = input.api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateStageResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateStageError::from_response(response))),
                )
            }
        })
    }
}
