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
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Settings for logging access in a stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessLogSettings {
    /// <p>The ARN of the CloudWatch Logs log group to receive access logs.</p>
    #[serde(rename = "DestinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    /// <p>A single line format of the access logs of data, as specified by selected $context variables. The format must include at least $context.requestId.</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// <p>Represents an API.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Api {
    /// <p>The URI of the API, of the form {api-id}.execute-api.{region}.amazonaws.com. The stage name is typically appended to this URI to form a complete path to a deployed API stage.</p>
    #[serde(rename = "ApiEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    /// <p>The API ID.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>An API key selection expression. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>A CORS configuration. Supported only for HTTP APIs.</p>
    #[serde(rename = "CorsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    /// <p>The timestamp when the API was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment. Supported only for WebSocket APIs.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The validation information during API import. This may include particular properties of your OpenAPI definition which are ignored during import. Supported only for HTTP APIs.</p>
    #[serde(rename = "ImportInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_info: Option<Vec<String>>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The API protocol.</p>
    #[serde(rename = "ProtocolType")]
    pub protocol_type: String,
    /// <p>The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.</p>
    #[serde(rename = "RouteSelectionExpression")]
    pub route_selection_expression: String,
    /// <p>A collection of tags associated with the API.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The warning messages reported when failonwarnings is turned on during API import.</p>
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p>Represents an API mapping.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Authorizer {
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null. Supported only for REQUEST authorizers.</p>
    #[serde(rename = "AuthorizerCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>Authorizer caching is not currently supported. Don't specify this value for authorizers.</p>
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>The authorizer type. For WebSocket APIs, specify REQUEST for a Lambda function using incoming request parameters. For HTTP APIs, specify JWT to use JSON Web Tokens.</p>
    #[serde(rename = "AuthorizerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    /// <p>The authorizer's Uniform Resource Identifier (URI). ForREQUEST authorizers, this must be a well-formed Lambda function URI, for example, arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:<replaceable>{account_id}</replaceable>:function:<replaceable>{lambda_function_name}</replaceable>/invocations. In general, the URI has this form: arn:aws:apigateway:<replaceable>{region}</replaceable>:lambda:path/<replaceable>{service_api}</replaceable>
    /// , where <replaceable></replaceable>{region} is the same as the region hosting the Lambda function, path indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial /. For Lambda functions, this is usually of the form /2015-03-31/functions/[FunctionARN]/invocations. Supported only for REQUEST authorizers.</p>
    #[serde(rename = "AuthorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identity source for which authorization is requested.</p> <p>For a REQUEST authorizer, this is optional. The value is a set of one or more mapping expressions of the specified request parameters. Currently, the identity source can be headers, query string parameters, stage variables, and context parameters. For example, if an Auth header and a Name query string parameter are defined as identity sources, this value is route.request.header.Auth, route.request.querystring.Name. These parameters will be used to perform runtime validation for Lambda-based authorizers by verifying all of the identity-related request parameters are present in the request, not null, and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda function. Otherwise, it returns a 401 Unauthorized response without calling the Lambda function.</p> <p>For JWT, a single entry that specifies where to extract the JSON Web Token (JWT) from inbound requests. Currently only header-based and query parameter-based selections are supported, for example "$request.header.Authorization".</p>
    #[serde(rename = "IdentitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    /// <p>The validation expression does not apply to the REQUEST authorizer.</p>
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>Represents the configuration of a JWT authorizer. Required for the JWT authorizer type. Supported only for HTTP APIs.</p>
    #[serde(rename = "JwtConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<JWTConfiguration>,
    /// <p>The name of the authorizer.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Represents a CORS configuration. Supported only for HTTP APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html">Configuring CORS</a> for more information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cors {
    /// <p>Specifies whether credentials are included in the CORS request. Supported only for HTTP APIs.</p>
    #[serde(rename = "AllowCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
    /// <p>Represents a collection of allowed headers. Supported only for HTTP APIs.</p>
    #[serde(rename = "AllowHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_headers: Option<Vec<String>>,
    /// <p>Represents a collection of allowed HTTP methods. Supported only for HTTP APIs.</p>
    #[serde(rename = "AllowMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_methods: Option<Vec<String>>,
    /// <p>Represents a collection of allowed origins. Supported only for HTTP APIs.</p>
    #[serde(rename = "AllowOrigins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_origins: Option<Vec<String>>,
    /// <p>Represents a collection of exposed headers. Supported only for HTTP APIs.</p>
    #[serde(rename = "ExposeHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,
    /// <p>The number of seconds that the browser should cache preflight request results. Supported only for HTTP APIs.</p>
    #[serde(rename = "MaxAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i64>,
}

/// <p>Creates a new ApiMapping resource to represent an API mapping.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApiMappingRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The API mapping key.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Creates a new Api resource to represent an API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApiRequest {
    /// <p>An API key selection expression. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>A CORS configuration. Supported only for HTTP APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html">Configuring CORS</a> for more information.</p>
    #[serde(rename = "CorsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    /// <p>This property is part of quick create. It specifies the credentials required for the integration, if any. For a Lambda integration, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null. Currently, this property is not used for HTTP integrations. Supported only for HTTP APIs.</p>
    #[serde(rename = "CredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment. Supported only for WebSocket APIs.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The API protocol.</p>
    #[serde(rename = "ProtocolType")]
    pub protocol_type: String,
    /// <p>This property is part of quick create. If you don't specify a routeKey, a default route of $default is created. The $default route acts as a catch-all for any request made to your API, for a particular stage. The $default route key can't be modified. You can add routes after creating the API, and you can update the route keys of additional routes. Supported only for HTTP APIs.</p>
    #[serde(rename = "RouteKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<String>,
    /// <p>The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.</p>
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>This property is part of quick create. Quick create produces an API with an integration, a default catch-all route, and a default stage which is configured to automatically deploy changes. For HTTP integrations, specify a fully qualified URL. For Lambda integrations, specify a function ARN. The type of the integration will be HTTP_PROXY or AWS_PROXY, respectively. Supported only for HTTP APIs.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateApiResponse {
    /// <p>The URI of the API, of the form {api-id}.execute-api.{region}.amazonaws.com. The stage name is typically appended to this URI to form a complete path to a deployed API stage.</p>
    #[serde(rename = "ApiEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    /// <p>The API ID.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>An API key selection expression. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>A CORS configuration. Supported only for HTTP APIs.</p>
    #[serde(rename = "CorsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    /// <p>The timestamp when the API was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment. Supported only for WebSocket APIs.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The validation information during API import. This may include particular properties of your OpenAPI definition which are ignored during import. Supported only for HTTP APIs.</p>
    #[serde(rename = "ImportInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_info: Option<Vec<String>>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The API protocol.</p>
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// <p>The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.</p>
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    /// <p>A collection of tags associated with the API.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The warning messages reported when failonwarnings is turned on during API import.</p>
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p>Creates a new Authorizer resource to represent an authorizer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAuthorizerRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null. Supported only for REQUEST authorizers.</p>
    #[serde(rename = "AuthorizerCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    /// <p>Authorizer caching is not currently supported. Don't specify this value for authorizers.</p>
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>The authorizer type. For WebSocket APIs, specify REQUEST for a Lambda function using incoming request parameters. For HTTP APIs, specify JWT to use JSON Web Tokens.</p>
    #[serde(rename = "AuthorizerType")]
    pub authorizer_type: String,
    /// <p>The authorizer's Uniform Resource Identifier (URI). For REQUEST authorizers, this must be a well-formed Lambda function URI, for example, arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:<replaceable>{account_id}</replaceable>:function:<replaceable>{lambda_function_name}</replaceable>/invocations. In general, the URI has this form: arn:aws:apigateway:<replaceable>{region}</replaceable>:lambda:path/<replaceable>{service_api}</replaceable>
    /// , where <replaceable></replaceable>{region} is the same as the region hosting the Lambda function, path indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial /. For Lambda functions, this is usually of the form /2015-03-31/functions/[FunctionARN]/invocations. Supported only for REQUEST authorizers.</p>
    #[serde(rename = "AuthorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identity source for which authorization is requested.</p> <p>For a REQUEST authorizer, this is optional. The value is a set of one or more mapping expressions of the specified request parameters. Currently, the identity source can be headers, query string parameters, stage variables, and context parameters. For example, if an Auth header and a Name query string parameter are defined as identity sources, this value is route.request.header.Auth, route.request.querystring.Name. These parameters will be used to perform runtime validation for Lambda-based authorizers by verifying all of the identity-related request parameters are present in the request, not null, and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda function. Otherwise, it returns a 401 Unauthorized response without calling the Lambda function.</p> <p>For JWT, a single entry that specifies where to extract the JSON Web Token (JWT )from inbound requests. Currently only header-based and query parameter-based selections are supported, for example "$request.header.Authorization".</p>
    #[serde(rename = "IdentitySource")]
    pub identity_source: Vec<String>,
    /// <p>This parameter is not used.</p>
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>Represents the configuration of a JWT authorizer. Required for the JWT authorizer type. Supported only for HTTP APIs.</p>
    #[serde(rename = "JwtConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<JWTConfiguration>,
    /// <p>The name of the authorizer.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAuthorizerResponse {
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null. Supported only for REQUEST authorizers.</p>
    #[serde(rename = "AuthorizerCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>Authorizer caching is not currently supported. Don't specify this value for authorizers.</p>
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>The authorizer type. For WebSocket APIs, specify REQUEST for a Lambda function using incoming request parameters. For HTTP APIs, specify JWT to use JSON Web Tokens.</p>
    #[serde(rename = "AuthorizerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    /// <p>The authorizer's Uniform Resource Identifier (URI). ForREQUEST authorizers, this must be a well-formed Lambda function URI, for example, arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:<replaceable>{account_id}</replaceable>:function:<replaceable>{lambda_function_name}</replaceable>/invocations. In general, the URI has this form: arn:aws:apigateway:<replaceable>{region}</replaceable>:lambda:path/<replaceable>{service_api}</replaceable>
    /// , where <replaceable></replaceable>{region} is the same as the region hosting the Lambda function, path indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial /. For Lambda functions, this is usually of the form /2015-03-31/functions/[FunctionARN]/invocations. Supported only for REQUEST authorizers.</p>
    #[serde(rename = "AuthorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identity source for which authorization is requested.</p> <p>For a REQUEST authorizer, this is optional. The value is a set of one or more mapping expressions of the specified request parameters. Currently, the identity source can be headers, query string parameters, stage variables, and context parameters. For example, if an Auth header and a Name query string parameter are defined as identity sources, this value is route.request.header.Auth, route.request.querystring.Name. These parameters will be used to perform runtime validation for Lambda-based authorizers by verifying all of the identity-related request parameters are present in the request, not null, and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda function. Otherwise, it returns a 401 Unauthorized response without calling the Lambda function.</p> <p>For JWT, a single entry that specifies where to extract the JSON Web Token (JWT) from inbound requests. Currently only header-based and query parameter-based selections are supported, for example "$request.header.Authorization".</p>
    #[serde(rename = "IdentitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    /// <p>The validation expression does not apply to the REQUEST authorizer.</p>
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>Represents the configuration of a JWT authorizer. Required for the JWT authorizer type. Supported only for HTTP APIs.</p>
    #[serde(rename = "JwtConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<JWTConfiguration>,
    /// <p>The name of the authorizer.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Creates a new Deployment resource to represent a deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDeploymentRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The description for the deployment resource.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the Stage resource for the Deployment resource to create.</p>
    #[serde(rename = "StageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeploymentResponse {
    /// <p>Specifies whether a deployment was automatically released.</p>
    #[serde(rename = "AutoDeployed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deployed: Option<bool>,
    /// <p>The date and time when the Deployment resource was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The identifier for the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The status of the deployment: PENDING, FAILED, or SUCCEEDED.</p>
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

/// <p>Creates a new DomainName resource to represent a domain name.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDomainNameRequest {
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The domain name configurations.</p>
    #[serde(rename = "DomainNameConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
    /// <p>The collection of tags associated with a domain name.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The collection of tags associated with a domain name.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Creates a new Integration resource to represent an integration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateIntegrationRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The connection ID.</p>
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. Currently the only valid value is INTERNET, for connections through the public routable internet.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null.</p>
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
    /// <p>The integration type of an integration. One of the following:</p> <p>AWS: for integrating the route or method request with an AWS service action, including the Lambda function-invoking action. With the Lambda function-invoking action, this is referred to as the Lambda custom integration. With any other AWS service action, this is known as AWS integration. Supported only for WebSocket APIs.</p> <p>AWS_PROXY: for integrating the route or method request with the Lambda function-invoking action with the client request passed through as-is. This integration is also referred to as Lambda proxy integration.</p> <p>HTTP: for integrating the route or method request with an HTTP endpoint. This integration is also referred to as the HTTP custom integration. Supported only for WebSocket APIs.</p> <p>HTTP_PROXY: for integrating route or method request with an HTTP endpoint, with the client request passed through as-is. This is also referred to as HTTP proxy integration.</p> <p>MOCK: for integrating the route or method request with API Gateway as a "loopback" endpoint without invoking any backend. Supported only for WebSocket APIs.</p>
    #[serde(rename = "IntegrationType")]
    pub integration_type: String,
    /// <p>For a Lambda proxy integration, this is the URI of the Lambda function.</p>
    #[serde(rename = "IntegrationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    /// <p>Specifies the pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the requestTemplates property on the Integration resource. There are three valid values: WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and NEVER. Supported only for WebSocket APIs.</p> <p>WHEN_NO_MATCH passes the request body for unmapped content types through to the integration backend without transformation.</p> <p>NEVER rejects unmapped content types with an HTTP 415 Unsupported Media Type response.</p> <p>WHEN_NO_TEMPLATES allows pass-through when the integration has no content types mapped to templates. However, if there is at least one content type defined, unmapped content types will be rejected with the same HTTP 415 Unsupported Media Type response.</p>
    #[serde(rename = "PassthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>Specifies the format of the payload sent to an integration. Required for HTTP APIs. Currently, the only supported value is 1.0.</p>
    #[serde(rename = "PayloadFormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_version: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method request to the backend. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the backend. The method request parameter value must match the pattern of method.request.<replaceable>{location}</replaceable>.<replaceable>{name}</replaceable>
    /// , where
    /// <replaceable>{location}</replaceable>
    /// is querystring, path, or header; and
    /// <replaceable>{name}</replaceable>
    /// must be a valid and unique method request parameter name. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds for WebSocket APIs. The default value is 5,000 milliseconds, or 5 seconds for HTTP APIs.</p>
    #[serde(rename = "TimeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
}

/// <p>Creates a new IntegrationResponse resource to represent an integration response.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateIntegrationResponseRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>The integration ID.</p>
    #[serde(rename = "IntegrationId")]
    pub integration_id: String,
    /// <p>The integration response key.</p>
    #[serde(rename = "IntegrationResponseKey")]
    pub integration_response_key: String,
    /// <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.{name}, where {name} is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.{name} or integration.response.body.{JSON-expression}, where {name} is a valid and unique response header name and {JSON-expression} is a valid JSON expression without the $ prefix.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration response. Supported only for WebSocket APIs.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIntegrationResponseResponse {
    /// <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
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
    /// <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.{name}, where name is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.{name} or integration.response.body.{JSON-expression}, where name is a valid and unique response header name and JSON-expression is a valid JSON expression without the $ prefix.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expressions for the integration response.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIntegrationResult {
    /// <p>Specifies whether an integration is managed by API Gateway. If you created an API using using quick create, the resulting integration is managed by API Gateway. You can update a managed integration, but you can't delete it.</p>
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    /// <p>The connection ID.</p>
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. Currently the only valid value is INTERNET, for connections through the public routable internet.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null.</p>
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
    /// <p>The integration response selection expression for the integration. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-integration-response-selection-expressions">Integration Response Selection Expressions</a>.</p>
    #[serde(rename = "IntegrationResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_selection_expression: Option<String>,
    /// <p>The integration type of an integration. One of the following:</p> <p>AWS: for integrating the route or method request with an AWS service action, including the Lambda function-invoking action. With the Lambda function-invoking action, this is referred to as the Lambda custom integration. With any other AWS service action, this is known as AWS integration. Supported only for WebSocket APIs.</p> <p>AWS_PROXY: for integrating the route or method request with the Lambda function-invoking action with the client request passed through as-is. This integration is also referred to as Lambda proxy integration.</p> <p>HTTP: for integrating the route or method request with an HTTP endpoint. This integration is also referred to as the HTTP custom integration. Supported only for WebSocket APIs.</p> <p>HTTP_PROXY: for integrating route or method request with an HTTP endpoint, with the client request passed through as-is. This is also referred to as HTTP proxy integration.</p> <p>MOCK: for integrating the route or method request with API Gateway as a "loopback" endpoint without invoking any backend. Supported only for WebSocket APIs.</p>
    #[serde(rename = "IntegrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    /// <p>For a Lambda proxy integration, this is the URI of the Lambda function.</p>
    #[serde(rename = "IntegrationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    /// <p>Specifies the pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the requestTemplates property on the Integration resource. There are three valid values: WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and NEVER. Supported only for WebSocket APIs.</p> <p>WHEN_NO_MATCH passes the request body for unmapped content types through to the integration backend without transformation.</p> <p>NEVER rejects unmapped content types with an HTTP 415 Unsupported Media Type response.</p> <p>WHEN_NO_TEMPLATES allows pass-through when the integration has no content types mapped to templates. However, if there is at least one content type defined, unmapped content types will be rejected with the same HTTP 415 Unsupported Media Type response.</p>
    #[serde(rename = "PassthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>Specifies the format of the payload sent to an integration. Required for HTTP APIs. Currently, the only supported value is 1.0.</p>
    #[serde(rename = "PayloadFormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_version: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method request to the backend. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the backend. The method request parameter value must match the pattern of method.request.<replaceable>{location}</replaceable>.<replaceable>{name}</replaceable>
    /// , where
    /// <replaceable>{location}</replaceable>
    /// is querystring, path, or header; and
    /// <replaceable>{name}</replaceable>
    /// must be a valid and unique method request parameter name. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration. Supported only for WebSocket APIs.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds for WebSocket APIs. The default value is 5,000 milliseconds, or 5 seconds for HTTP APIs.</p>
    #[serde(rename = "TimeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
}

/// <p>Creates a new Model.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The schema for the model. For application/json models, this should be JSON schema draft 4 model.</p>
    #[serde(rename = "Schema")]
    pub schema: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The schema for the model. For application/json models, this should be JSON schema draft 4 model.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

/// <p>Creates a new Route resource to represent a route.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRouteRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Specifies whether an API key is required for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ApiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>The authorization scopes supported by this route.</p>
    #[serde(rename = "AuthorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The authorization type for the route. For WebSocket APIs, valid values are NONE for open access, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer For HTTP APIs, valid values are NONE for open access, or JWT for using JSON Web Tokens.</p>
    #[serde(rename = "AuthorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of the Authorizer resource to be associated with this route. The authorizer identifier is generated by API Gateway when you created the authorizer.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The model selection expression for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The operation name for the route.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The request models for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The request parameters for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, ParameterConstraints>>,
    /// <p>The route key for the route.</p>
    #[serde(rename = "RouteKey")]
    pub route_key: String,
    /// <p>The route response selection expression for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RouteResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    /// <p>The target for the route.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// <p>Creates a new RouteResponse resource to represent a route response.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRouteResponseRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The model selection expression for the route response. Supported only for WebSocket APIs.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRouteResponseResponse {
    /// <p>Represents the model selection expression of a route response. Supported only for WebSocket APIs.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRouteResult {
    /// <p>Specifies whether a route is managed by API Gateway. If you created an API using quick create, the $default route is managed by API Gateway. You can't modify the $default route key.</p>
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    /// <p>Specifies whether an API key is required for this route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ApiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>A list of authorization scopes configured on a route. The scopes are used with a JWT authorizer to authorize the method invocation. The authorization works by matching the route scopes against the scopes parsed from the access token in the incoming request. The method invocation is authorized if any route scope matches a claimed scope in the access token. Otherwise, the invocation is not authorized. When the route scope is configured, the client must provide an access token instead of an identity token for authorization purposes.</p>
    #[serde(rename = "AuthorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The authorization type for the route. For WebSocket APIs, valid values are NONE for open access, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer For HTTP APIs, valid values are NONE for open access, or JWT for using JSON Web Tokens.</p>
    #[serde(rename = "AuthorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of the Authorizer resource to be associated with this route. The authorizer identifier is generated by API Gateway when you created the authorizer.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The model selection expression for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The operation name for the route.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The request models for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The request parameters for the route. Supported only for WebSocket APIs.</p>
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
    /// <p>The route response selection expression for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RouteResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    /// <p>The target for the route.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// <p>Creates a new Stage resource to represent a stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateStageRequest {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Specifies whether updates to an API automatically trigger a new deployment. The default value is false.</p>
    #[serde(rename = "AutoDeploy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    /// <p>The identifier of a client certificate for a Stage. Supported only for WebSocket APIs.</p>
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
    /// <p>Route settings for the stage, by routeKey.</p>
    #[serde(rename = "RouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<::std::collections::HashMap<String, RouteSettings>>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "StageName")]
    pub stage_name: String,
    /// <p>A map that defines the stage variables for a Stage. Variable names can have alphanumeric and underscore characters, and the values must match [A-Za-z0-9-._~:/?#&amp;=,]+. Supported only for WebSocket APIs.</p>
    #[serde(rename = "StageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateStageResponse {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>Specifies whether a stage is managed by API Gateway. If you created an API using quick create, the $default stage is managed by API Gateway. You can't modify the $default stage.</p>
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    /// <p>Specifies whether updates to an API automatically trigger a new deployment. The default value is false.</p>
    #[serde(rename = "AutoDeploy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    /// <p>The identifier of a client certificate for a Stage. Supported only for WebSocket APIs.</p>
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
    /// <p>The identifier of the Deployment that the Stage is associated with. Can't be updated if autoDeploy is enabled.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The description of the stage.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Describes the status of the last deployment of a stage. Supported only for stages with autoDeploy enabled.</p>
    #[serde(rename = "LastDeploymentStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_status_message: Option<String>,
    /// <p>The timestamp when the stage was last updated.</p>
    #[serde(rename = "LastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>Route settings for the stage, by routeKey.</p>
    #[serde(rename = "RouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<::std::collections::HashMap<String, RouteSettings>>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "StageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    /// <p>A map that defines the stage variables for a stage resource. Variable names can have alphanumeric and underscore characters, and the values must match [A-Za-z0-9-._~:/?#&amp;=,]+. Supported only for WebSocket APIs.</p>
    #[serde(rename = "StageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApiMappingRequest {
    /// <p>The API mapping identifier.</p>
    #[serde(rename = "ApiMappingId")]
    pub api_mapping_id: String,
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApiRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAuthorizerRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    pub authorizer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCorsConfigurationRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDeploymentRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The deployment ID.</p>
    #[serde(rename = "DeploymentId")]
    pub deployment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDomainNameRequest {
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIntegrationRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The integration ID.</p>
    #[serde(rename = "IntegrationId")]
    pub integration_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteModelRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The model ID.</p>
    #[serde(rename = "ModelId")]
    pub model_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRouteRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    pub route_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRouteSettingsRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The route key.</p>
    #[serde(rename = "RouteKey")]
    pub route_key: String,
    /// <p>The stage name. Stage names can only contain alphanumeric characters, hyphens, and underscores. Maximum length is 128 characters.</p>
    #[serde(rename = "StageName")]
    pub stage_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteStageRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The stage name. Stage names can only contain alphanumeric characters, hyphens, and underscores. Maximum length is 128 characters.</p>
    #[serde(rename = "StageName")]
    pub stage_name: String,
}

/// <p>An immutable representation of an API that can be called by users. A Deployment must be associated with a Stage for it to be callable over the internet.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Deployment {
    /// <p>Specifies whether a deployment was automatically released.</p>
    #[serde(rename = "AutoDeployed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deployed: Option<bool>,
    /// <p>The date and time when the Deployment resource was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The identifier for the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The status of the deployment: PENDING, FAILED, or SUCCEEDED.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The collection of tags associated with a domain name.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The domain name configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainNameConfiguration {
    /// <p>A domain name for the API.</p>
    #[serde(rename = "ApiGatewayDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_domain_name: Option<String>,
    /// <p>An AWS-managed certificate that will be used by the edge-optimized endpoint for this domain name. AWS Certificate Manager is the only supported source.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The user-friendly name of the certificate that will be used by the edge-optimized endpoint for this domain name.</p>
    #[serde(rename = "CertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    /// <p>The timestamp when the certificate that was used by edge-optimized endpoint for this domain name was uploaded.</p>
    #[serde(rename = "CertificateUploadDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_upload_date: Option<f64>,
    /// <p>The status of the domain name migration. The valid values are AVAILABLE and UPDATING. If the status is UPDATING, the domain cannot be modified further until the existing operation is complete. If it is AVAILABLE, the domain can be updated.</p>
    #[serde(rename = "DomainNameStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_status: Option<String>,
    /// <p>An optional text message containing detailed information about status of the domain name migration.</p>
    #[serde(rename = "DomainNameStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_status_message: Option<String>,
    /// <p>The endpoint type.</p>
    #[serde(rename = "EndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// <p>The Amazon Route 53 Hosted Zone ID of the endpoint.</p>
    #[serde(rename = "HostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    /// <p>The Transport Layer Security (TLS) version of the security policy for this domain name. The valid values are TLS_1_0 and TLS_1_2.</p>
    #[serde(rename = "SecurityPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApiMappingRequest {
    /// <p>The API mapping identifier.</p>
    #[serde(rename = "ApiMappingId")]
    pub api_mapping_id: String,
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApiMappingsRequest {
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetApiMappingsResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ApiMapping>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApiRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetApiResponse {
    /// <p>The URI of the API, of the form {api-id}.execute-api.{region}.amazonaws.com. The stage name is typically appended to this URI to form a complete path to a deployed API stage.</p>
    #[serde(rename = "ApiEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    /// <p>The API ID.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>An API key selection expression. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>A CORS configuration. Supported only for HTTP APIs.</p>
    #[serde(rename = "CorsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    /// <p>The timestamp when the API was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment. Supported only for WebSocket APIs.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The validation information during API import. This may include particular properties of your OpenAPI definition which are ignored during import. Supported only for HTTP APIs.</p>
    #[serde(rename = "ImportInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_info: Option<Vec<String>>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The API protocol.</p>
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// <p>The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.</p>
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    /// <p>A collection of tags associated with the API.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The warning messages reported when failonwarnings is turned on during API import.</p>
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApisRequest {
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetApisResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Api>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAuthorizerRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    pub authorizer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAuthorizerResponse {
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null. Supported only for REQUEST authorizers.</p>
    #[serde(rename = "AuthorizerCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>Authorizer caching is not currently supported. Don't specify this value for authorizers.</p>
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>The authorizer type. For WebSocket APIs, specify REQUEST for a Lambda function using incoming request parameters. For HTTP APIs, specify JWT to use JSON Web Tokens.</p>
    #[serde(rename = "AuthorizerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    /// <p>The authorizer's Uniform Resource Identifier (URI). ForREQUEST authorizers, this must be a well-formed Lambda function URI, for example, arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:<replaceable>{account_id}</replaceable>:function:<replaceable>{lambda_function_name}</replaceable>/invocations. In general, the URI has this form: arn:aws:apigateway:<replaceable>{region}</replaceable>:lambda:path/<replaceable>{service_api}</replaceable>
    /// , where <replaceable></replaceable>{region} is the same as the region hosting the Lambda function, path indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial /. For Lambda functions, this is usually of the form /2015-03-31/functions/[FunctionARN]/invocations. Supported only for REQUEST authorizers.</p>
    #[serde(rename = "AuthorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identity source for which authorization is requested.</p> <p>For a REQUEST authorizer, this is optional. The value is a set of one or more mapping expressions of the specified request parameters. Currently, the identity source can be headers, query string parameters, stage variables, and context parameters. For example, if an Auth header and a Name query string parameter are defined as identity sources, this value is route.request.header.Auth, route.request.querystring.Name. These parameters will be used to perform runtime validation for Lambda-based authorizers by verifying all of the identity-related request parameters are present in the request, not null, and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda function. Otherwise, it returns a 401 Unauthorized response without calling the Lambda function.</p> <p>For JWT, a single entry that specifies where to extract the JSON Web Token (JWT) from inbound requests. Currently only header-based and query parameter-based selections are supported, for example "$request.header.Authorization".</p>
    #[serde(rename = "IdentitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    /// <p>The validation expression does not apply to the REQUEST authorizer.</p>
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>Represents the configuration of a JWT authorizer. Required for the JWT authorizer type. Supported only for HTTP APIs.</p>
    #[serde(rename = "JwtConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<JWTConfiguration>,
    /// <p>The name of the authorizer.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAuthorizersRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAuthorizersResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Authorizer>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeploymentRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The deployment ID.</p>
    #[serde(rename = "DeploymentId")]
    pub deployment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeploymentResponse {
    /// <p>Specifies whether a deployment was automatically released.</p>
    #[serde(rename = "AutoDeployed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deployed: Option<bool>,
    /// <p>The date and time when the Deployment resource was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The identifier for the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The status of the deployment: PENDING, FAILED, or SUCCEEDED.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeploymentsRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeploymentsResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Deployment>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDomainNameRequest {
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The collection of tags associated with a domain name.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDomainNamesRequest {
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDomainNamesResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DomainName>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIntegrationRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The integration ID.</p>
    #[serde(rename = "IntegrationId")]
    pub integration_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetIntegrationResponseResponse {
    /// <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
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
    /// <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.{name}, where name is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.{name} or integration.response.body.{JSON-expression}, where name is a valid and unique response header name and JSON-expression is a valid JSON expression without the $ prefix.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expressions for the integration response.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetIntegrationResponsesResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<IntegrationResponse>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetIntegrationResult {
    /// <p>Specifies whether an integration is managed by API Gateway. If you created an API using using quick create, the resulting integration is managed by API Gateway. You can update a managed integration, but you can't delete it.</p>
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    /// <p>The connection ID.</p>
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. Currently the only valid value is INTERNET, for connections through the public routable internet.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null.</p>
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
    /// <p>The integration response selection expression for the integration. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-integration-response-selection-expressions">Integration Response Selection Expressions</a>.</p>
    #[serde(rename = "IntegrationResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_selection_expression: Option<String>,
    /// <p>The integration type of an integration. One of the following:</p> <p>AWS: for integrating the route or method request with an AWS service action, including the Lambda function-invoking action. With the Lambda function-invoking action, this is referred to as the Lambda custom integration. With any other AWS service action, this is known as AWS integration. Supported only for WebSocket APIs.</p> <p>AWS_PROXY: for integrating the route or method request with the Lambda function-invoking action with the client request passed through as-is. This integration is also referred to as Lambda proxy integration.</p> <p>HTTP: for integrating the route or method request with an HTTP endpoint. This integration is also referred to as the HTTP custom integration. Supported only for WebSocket APIs.</p> <p>HTTP_PROXY: for integrating route or method request with an HTTP endpoint, with the client request passed through as-is. This is also referred to as HTTP proxy integration.</p> <p>MOCK: for integrating the route or method request with API Gateway as a "loopback" endpoint without invoking any backend. Supported only for WebSocket APIs.</p>
    #[serde(rename = "IntegrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    /// <p>For a Lambda proxy integration, this is the URI of the Lambda function.</p>
    #[serde(rename = "IntegrationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    /// <p>Specifies the pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the requestTemplates property on the Integration resource. There are three valid values: WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and NEVER. Supported only for WebSocket APIs.</p> <p>WHEN_NO_MATCH passes the request body for unmapped content types through to the integration backend without transformation.</p> <p>NEVER rejects unmapped content types with an HTTP 415 Unsupported Media Type response.</p> <p>WHEN_NO_TEMPLATES allows pass-through when the integration has no content types mapped to templates. However, if there is at least one content type defined, unmapped content types will be rejected with the same HTTP 415 Unsupported Media Type response.</p>
    #[serde(rename = "PassthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>Specifies the format of the payload sent to an integration. Required for HTTP APIs. Currently, the only supported value is 1.0.</p>
    #[serde(rename = "PayloadFormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_version: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method request to the backend. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the backend. The method request parameter value must match the pattern of method.request.<replaceable>{location}</replaceable>.<replaceable>{name}</replaceable>
    /// , where
    /// <replaceable>{location}</replaceable>
    /// is querystring, path, or header; and
    /// <replaceable>{name}</replaceable>
    /// must be a valid and unique method request parameter name. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration. Supported only for WebSocket APIs.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds for WebSocket APIs. The default value is 5,000 milliseconds, or 5 seconds for HTTP APIs.</p>
    #[serde(rename = "TimeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIntegrationsRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetIntegrationsResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Integration>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetModelRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The model ID.</p>
    #[serde(rename = "ModelId")]
    pub model_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The schema for the model. For application/json models, this should be JSON schema draft 4 model.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetModelTemplateRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The model ID.</p>
    #[serde(rename = "ModelId")]
    pub model_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetModelTemplateResponse {
    /// <p>The template value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetModelsRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetModelsResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Model>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRouteRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    pub route_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRouteResponseResponse {
    /// <p>Represents the model selection expression of a route response. Supported only for WebSocket APIs.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRouteResponsesRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The route ID.</p>
    #[serde(rename = "RouteId")]
    pub route_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRouteResponsesResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RouteResponse>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRouteResult {
    /// <p>Specifies whether a route is managed by API Gateway. If you created an API using quick create, the $default route is managed by API Gateway. You can't modify the $default route key.</p>
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    /// <p>Specifies whether an API key is required for this route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ApiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>A list of authorization scopes configured on a route. The scopes are used with a JWT authorizer to authorize the method invocation. The authorization works by matching the route scopes against the scopes parsed from the access token in the incoming request. The method invocation is authorized if any route scope matches a claimed scope in the access token. Otherwise, the invocation is not authorized. When the route scope is configured, the client must provide an access token instead of an identity token for authorization purposes.</p>
    #[serde(rename = "AuthorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The authorization type for the route. For WebSocket APIs, valid values are NONE for open access, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer For HTTP APIs, valid values are NONE for open access, or JWT for using JSON Web Tokens.</p>
    #[serde(rename = "AuthorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of the Authorizer resource to be associated with this route. The authorizer identifier is generated by API Gateway when you created the authorizer.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The model selection expression for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The operation name for the route.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The request models for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The request parameters for the route. Supported only for WebSocket APIs.</p>
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
    /// <p>The route response selection expression for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RouteResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    /// <p>The target for the route.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRoutesRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRoutesResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Route>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetStageRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The stage name. Stage names can only contain alphanumeric characters, hyphens, and underscores. Maximum length is 128 characters.</p>
    #[serde(rename = "StageName")]
    pub stage_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetStageResponse {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>Specifies whether a stage is managed by API Gateway. If you created an API using quick create, the $default stage is managed by API Gateway. You can't modify the $default stage.</p>
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    /// <p>Specifies whether updates to an API automatically trigger a new deployment. The default value is false.</p>
    #[serde(rename = "AutoDeploy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    /// <p>The identifier of a client certificate for a Stage. Supported only for WebSocket APIs.</p>
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
    /// <p>The identifier of the Deployment that the Stage is associated with. Can't be updated if autoDeploy is enabled.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The description of the stage.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Describes the status of the last deployment of a stage. Supported only for stages with autoDeploy enabled.</p>
    #[serde(rename = "LastDeploymentStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_status_message: Option<String>,
    /// <p>The timestamp when the stage was last updated.</p>
    #[serde(rename = "LastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>Route settings for the stage, by routeKey.</p>
    #[serde(rename = "RouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<::std::collections::HashMap<String, RouteSettings>>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "StageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    /// <p>A map that defines the stage variables for a stage resource. Variable names can have alphanumeric and underscore characters, and the values must match [A-Za-z0-9-._~:/?#&amp;=,]+. Supported only for WebSocket APIs.</p>
    #[serde(rename = "StageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetStagesRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The maximum number of elements to be returned for this resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetStagesResponse {
    /// <p>The elements from this collection.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Stage>>,
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTagsRequest {
    /// <p>The resource ARN for the tag.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTagsResponse {
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportApiRequest {
    /// <p>Represents the base path of the imported API. Supported only for HTTP APIs.</p>
    #[serde(rename = "Basepath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basepath: Option<String>,
    /// <p>The OpenAPI definition. Supported only for HTTP APIs.</p>
    #[serde(rename = "Body")]
    pub body: String,
    /// <p>Specifies whether to rollback the API creation (true) or not (false) when a warning is encountered. The default value is false.</p>
    #[serde(rename = "FailOnWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportApiResponse {
    /// <p>The URI of the API, of the form {api-id}.execute-api.{region}.amazonaws.com. The stage name is typically appended to this URI to form a complete path to a deployed API stage.</p>
    #[serde(rename = "ApiEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    /// <p>The API ID.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>An API key selection expression. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>A CORS configuration. Supported only for HTTP APIs.</p>
    #[serde(rename = "CorsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    /// <p>The timestamp when the API was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment. Supported only for WebSocket APIs.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The validation information during API import. This may include particular properties of your OpenAPI definition which are ignored during import. Supported only for HTTP APIs.</p>
    #[serde(rename = "ImportInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_info: Option<Vec<String>>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The API protocol.</p>
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// <p>The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.</p>
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    /// <p>A collection of tags associated with the API.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The warning messages reported when failonwarnings is turned on during API import.</p>
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p>Represents an integration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Integration {
    /// <p>Specifies whether an integration is managed by API Gateway. If you created an API using using quick create, the resulting integration is managed by API Gateway. You can update a managed integration, but you can't delete it.</p>
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    /// <p>The connection ID.</p>
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. Currently the only valid value is INTERNET, for connections through the public routable internet.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null.</p>
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
    /// <p>The integration response selection expression for the integration. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-integration-response-selection-expressions">Integration Response Selection Expressions</a>.</p>
    #[serde(rename = "IntegrationResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_selection_expression: Option<String>,
    /// <p>The integration type of an integration. One of the following:</p> <p>AWS: for integrating the route or method request with an AWS service action, including the Lambda function-invoking action. With the Lambda function-invoking action, this is referred to as the Lambda custom integration. With any other AWS service action, this is known as AWS integration. Supported only for WebSocket APIs.</p> <p>AWS_PROXY: for integrating the route or method request with the Lambda function-invoking action with the client request passed through as-is. This integration is also referred to as Lambda proxy integration.</p> <p>HTTP: for integrating the route or method request with an HTTP endpoint. This integration is also referred to as the HTTP custom integration. Supported only for WebSocket APIs.</p> <p>HTTP_PROXY: for integrating route or method request with an HTTP endpoint, with the client request passed through as-is. This is also referred to as HTTP proxy integration.</p> <p>MOCK: for integrating the route or method request with API Gateway as a "loopback" endpoint without invoking any backend. Supported only for WebSocket APIs.</p>
    #[serde(rename = "IntegrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    /// <p>For a Lambda proxy integration, this is the URI of the Lambda function.</p>
    #[serde(rename = "IntegrationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    /// <p>Specifies the pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the requestTemplates property on the Integration resource. There are three valid values: WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and NEVER. Supported only for WebSocket APIs.</p> <p>WHEN_NO_MATCH passes the request body for unmapped content types through to the integration backend without transformation.</p> <p>NEVER rejects unmapped content types with an HTTP 415 Unsupported Media Type response.</p> <p>WHEN_NO_TEMPLATES allows pass-through when the integration has no content types mapped to templates. However, if there is at least one content type defined, unmapped content types will be rejected with the same HTTP 415 Unsupported Media Type response.</p>
    #[serde(rename = "PassthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>Specifies the format of the payload sent to an integration. Required for HTTP APIs. Currently, the only supported value is 1.0.</p>
    #[serde(rename = "PayloadFormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_version: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method request to the backend. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the backend. The method request parameter value must match the pattern of method.request.<replaceable>{location}</replaceable>.<replaceable>{name}</replaceable>
    /// , where
    /// <replaceable>{location}</replaceable>
    /// is querystring, path, or header; and
    /// <replaceable>{name}</replaceable>
    /// must be a valid and unique method request parameter name. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration. Supported only for WebSocket APIs.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds for WebSocket APIs. The default value is 5,000 milliseconds, or 5 seconds for HTTP APIs.</p>
    #[serde(rename = "TimeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
}

/// <p>Represents an integration response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IntegrationResponse {
    /// <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
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
    /// <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.{name}, where name is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.{name} or integration.response.body.{JSON-expression}, where name is a valid and unique response header name and JSON-expression is a valid JSON expression without the $ prefix.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expressions for the integration response.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

/// <p>Represents the configuration of a JWT authorizer. Required for the JWT authorizer type. Supported only for HTTP APIs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JWTConfiguration {
    /// <p>A list of the intended recipients of the JWT. A valid JWT must provide an aud that matches at least one entry in this list. See <a href="https://tools.ietf.org/html/rfc7519#section-4.1.3">RFC 7519</a>. Supported only for HTTP APIs.</p>
    #[serde(rename = "Audience")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Vec<String>>,
    /// <p>The base domain of the identity provider that issues JSON Web Tokens. For example, an Amazon Cognito user pool has the following format: https://cognito-idp.<replaceable>{region}</replaceable>.amazonaws.com/<replaceable>{userPoolId}</replaceable>
    /// . Required for the JWT authorizer type. Supported only for HTTP APIs.</p>
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

/// <p>Represents a data model for an API. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/models-mappings.html">Create Models and Mapping Templates for Request and Response Mappings</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The schema for the model. For application/json models, this should be JSON schema draft 4 model.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

/// <p>Validation constraints imposed on parameters of a request (path, query string, headers).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterConstraints {
    /// <p>Whether or not the parameter is required.</p>
    #[serde(rename = "Required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// <p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReimportApiRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Represents the base path of the imported API. Supported only for HTTP APIs.</p>
    #[serde(rename = "Basepath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basepath: Option<String>,
    /// <p>The OpenAPI definition. Supported only for HTTP APIs.</p>
    #[serde(rename = "Body")]
    pub body: String,
    /// <p>Specifies whether to rollback the API creation (true) or not (false) when a warning is encountered. The default value is false.</p>
    #[serde(rename = "FailOnWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReimportApiResponse {
    /// <p>The URI of the API, of the form {api-id}.execute-api.{region}.amazonaws.com. The stage name is typically appended to this URI to form a complete path to a deployed API stage.</p>
    #[serde(rename = "ApiEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    /// <p>The API ID.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>An API key selection expression. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>A CORS configuration. Supported only for HTTP APIs.</p>
    #[serde(rename = "CorsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    /// <p>The timestamp when the API was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment. Supported only for WebSocket APIs.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The validation information during API import. This may include particular properties of your OpenAPI definition which are ignored during import. Supported only for HTTP APIs.</p>
    #[serde(rename = "ImportInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_info: Option<Vec<String>>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The API protocol.</p>
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// <p>The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.</p>
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    /// <p>A collection of tags associated with the API.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The warning messages reported when failonwarnings is turned on during API import.</p>
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p>Represents a route.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Route {
    /// <p>Specifies whether a route is managed by API Gateway. If you created an API using quick create, the $default route is managed by API Gateway. You can't modify the $default route key.</p>
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    /// <p>Specifies whether an API key is required for this route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ApiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>A list of authorization scopes configured on a route. The scopes are used with a JWT authorizer to authorize the method invocation. The authorization works by matching the route scopes against the scopes parsed from the access token in the incoming request. The method invocation is authorized if any route scope matches a claimed scope in the access token. Otherwise, the invocation is not authorized. When the route scope is configured, the client must provide an access token instead of an identity token for authorization purposes.</p>
    #[serde(rename = "AuthorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The authorization type for the route. For WebSocket APIs, valid values are NONE for open access, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer For HTTP APIs, valid values are NONE for open access, or JWT for using JSON Web Tokens.</p>
    #[serde(rename = "AuthorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of the Authorizer resource to be associated with this route. The authorizer identifier is generated by API Gateway when you created the authorizer.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The model selection expression for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The operation name for the route.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The request models for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The request parameters for the route. Supported only for WebSocket APIs.</p>
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
    /// <p>The route response selection expression for the route. Supported only for WebSocket APIs.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RouteResponse {
    /// <p>Represents the model selection expression of a route response. Supported only for WebSocket APIs.</p>
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
    /// <p>Specifies whether (true) or not (false) data trace logging is enabled for this route. This property affects the log entries pushed to Amazon CloudWatch Logs. Supported only for WebSocket APIs.</p>
    #[serde(rename = "DataTraceEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_trace_enabled: Option<bool>,
    /// <p>Specifies whether detailed metrics are enabled.</p>
    #[serde(rename = "DetailedMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_metrics_enabled: Option<bool>,
    /// <p>Specifies the logging level for this route: INFO, ERROR, or OFF. This property affects the log entries pushed to Amazon CloudWatch Logs. Supported only for WebSocket APIs.</p>
    #[serde(rename = "LoggingLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    /// <p>Specifies the throttling burst limit. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ThrottlingBurstLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_burst_limit: Option<i64>,
    /// <p>Specifies the throttling rate limit. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ThrottlingRateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_rate_limit: Option<f64>,
}

/// <p>Represents an API stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Stage {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>Specifies whether a stage is managed by API Gateway. If you created an API using quick create, the $default stage is managed by API Gateway. You can't modify the $default stage.</p>
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    /// <p>Specifies whether updates to an API automatically trigger a new deployment. The default value is false.</p>
    #[serde(rename = "AutoDeploy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    /// <p>The identifier of a client certificate for a Stage. Supported only for WebSocket APIs.</p>
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
    /// <p>The identifier of the Deployment that the Stage is associated with. Can't be updated if autoDeploy is enabled.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The description of the stage.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Describes the status of the last deployment of a stage. Supported only for stages with autoDeploy enabled.</p>
    #[serde(rename = "LastDeploymentStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_status_message: Option<String>,
    /// <p>The timestamp when the stage was last updated.</p>
    #[serde(rename = "LastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>Route settings for the stage, by routeKey.</p>
    #[serde(rename = "RouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<::std::collections::HashMap<String, RouteSettings>>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "StageName")]
    pub stage_name: String,
    /// <p>A map that defines the stage variables for a stage resource. Variable names can have alphanumeric and underscore characters, and the values must match [A-Za-z0-9-._~:/?#&amp;=,]+. Supported only for WebSocket APIs.</p>
    #[serde(rename = "StageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Creates a new Tag resource to represent a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The resource ARN for the tag.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The resource ARN for the tag.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <pre><code>        &lt;p&gt;The Tag keys to delete.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>Updates an ApiMapping.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Updates an Api.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApiRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>An API key selection expression. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>A CORS configuration. Supported only for HTTP APIs.</p>
    #[serde(rename = "CorsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    /// <p>This property is part of quick create. It specifies the credentials required for the integration, if any. For a Lambda integration, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null. Currently, this property is not used for HTTP integrations. If provided, this value replaces the credentials associated with the quick create integration. Supported only for HTTP APIs.</p>
    #[serde(rename = "CredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment. Supported only for WebSocket APIs.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>This property is part of quick create. If not specified, the route created using quick create is kept. Otherwise, this value replaces the route key of the quick create route. Additional routes may still be added after the API is updated. Supported only for HTTP APIs.</p>
    #[serde(rename = "RouteKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<String>,
    /// <p>The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.</p>
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    /// <p>This property is part of quick create. For HTTP integrations, specify a fully qualified URL. For Lambda integrations, specify a function ARN. The type of the integration will be HTTP_PROXY or AWS_PROXY, respectively. The value provided updates the integration URI and integration type. You can update a quick-created target, but you can't remove it from an API. Supported only for HTTP APIs.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApiResponse {
    /// <p>The URI of the API, of the form {api-id}.execute-api.{region}.amazonaws.com. The stage name is typically appended to this URI to form a complete path to a deployed API stage.</p>
    #[serde(rename = "ApiEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    /// <p>The API ID.</p>
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>An API key selection expression. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p>
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    /// <p>A CORS configuration. Supported only for HTTP APIs.</p>
    #[serde(rename = "CorsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    /// <p>The timestamp when the API was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the API.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Avoid validating models when creating a deployment. Supported only for WebSocket APIs.</p>
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    /// <p>The validation information during API import. This may include particular properties of your OpenAPI definition which are ignored during import. Supported only for HTTP APIs.</p>
    #[serde(rename = "ImportInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_info: Option<Vec<String>>,
    /// <p>The name of the API.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The API protocol.</p>
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// <p>The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.</p>
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    /// <p>A collection of tags associated with the API.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The warning messages reported when failonwarnings is turned on during API import.</p>
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p>Updates an Authorizer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAuthorizerRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null.</p>
    #[serde(rename = "AuthorizerCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    pub authorizer_id: String,
    /// <p>Authorizer caching is not currently supported. Don't specify this value for authorizers.</p>
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>The authorizer type. For WebSocket APIs, specify REQUEST for a Lambda function using incoming request parameters. For HTTP APIs, specify JWT to use JSON Web Tokens.</p>
    #[serde(rename = "AuthorizerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    /// <p>The authorizer's Uniform Resource Identifier (URI). For REQUEST authorizers, this must be a well-formed Lambda function URI, for example, arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:<replaceable>{account_id}</replaceable>:function:<replaceable>{lambda_function_name}</replaceable>/invocations. In general, the URI has this form: arn:aws:apigateway:<replaceable>{region}</replaceable>:lambda:path/<replaceable>{service_api}</replaceable>
    /// , where <replaceable></replaceable>{region} is the same as the region hosting the Lambda function, path indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial /. For Lambda functions, this is usually of the form /2015-03-31/functions/[FunctionARN]/invocations. Supported only for REQUEST authorizers.</p>
    #[serde(rename = "AuthorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identity source for which authorization is requested.</p> <p>For a REQUEST authorizer, this is optional. The value is a set of one or more mapping expressions of the specified request parameters. Currently, the identity source can be headers, query string parameters, stage variables, and context parameters. For example, if an Auth header and a Name query string parameter are defined as identity sources, this value is route.request.header.Auth, route.request.querystring.Name. These parameters will be used to perform runtime validation for Lambda-based authorizers by verifying all of the identity-related request parameters are present in the request, not null, and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda function. Otherwise, it returns a 401 Unauthorized response without calling the Lambda function.</p> <p>For JWT, a single entry that specifies where to extract the JSON Web Token (JWT) from inbound requests. Currently only header-based and query parameter-based selections are supported, for example "$request.header.Authorization".</p>
    #[serde(rename = "IdentitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    /// <p>This parameter is not used.</p>
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>Represents the configuration of a JWT authorizer. Required for the JWT authorizer type. Supported only for HTTP APIs.</p>
    #[serde(rename = "JwtConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<JWTConfiguration>,
    /// <p>The name of the authorizer.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAuthorizerResponse {
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null. Supported only for REQUEST authorizers.</p>
    #[serde(rename = "AuthorizerCredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    /// <p>The authorizer identifier.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>Authorizer caching is not currently supported. Don't specify this value for authorizers.</p>
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>The authorizer type. For WebSocket APIs, specify REQUEST for a Lambda function using incoming request parameters. For HTTP APIs, specify JWT to use JSON Web Tokens.</p>
    #[serde(rename = "AuthorizerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    /// <p>The authorizer's Uniform Resource Identifier (URI). ForREQUEST authorizers, this must be a well-formed Lambda function URI, for example, arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:<replaceable>{account_id}</replaceable>:function:<replaceable>{lambda_function_name}</replaceable>/invocations. In general, the URI has this form: arn:aws:apigateway:<replaceable>{region}</replaceable>:lambda:path/<replaceable>{service_api}</replaceable>
    /// , where <replaceable></replaceable>{region} is the same as the region hosting the Lambda function, path indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial /. For Lambda functions, this is usually of the form /2015-03-31/functions/[FunctionARN]/invocations. Supported only for REQUEST authorizers.</p>
    #[serde(rename = "AuthorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identity source for which authorization is requested.</p> <p>For a REQUEST authorizer, this is optional. The value is a set of one or more mapping expressions of the specified request parameters. Currently, the identity source can be headers, query string parameters, stage variables, and context parameters. For example, if an Auth header and a Name query string parameter are defined as identity sources, this value is route.request.header.Auth, route.request.querystring.Name. These parameters will be used to perform runtime validation for Lambda-based authorizers by verifying all of the identity-related request parameters are present in the request, not null, and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda function. Otherwise, it returns a 401 Unauthorized response without calling the Lambda function.</p> <p>For JWT, a single entry that specifies where to extract the JSON Web Token (JWT) from inbound requests. Currently only header-based and query parameter-based selections are supported, for example "$request.header.Authorization".</p>
    #[serde(rename = "IdentitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    /// <p>The validation expression does not apply to the REQUEST authorizer.</p>
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>Represents the configuration of a JWT authorizer. Required for the JWT authorizer type. Supported only for HTTP APIs.</p>
    #[serde(rename = "JwtConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<JWTConfiguration>,
    /// <p>The name of the authorizer.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Updates a Deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDeploymentResponse {
    /// <p>Specifies whether a deployment was automatically released.</p>
    #[serde(rename = "AutoDeployed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deployed: Option<bool>,
    /// <p>The date and time when the Deployment resource was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The identifier for the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The status of the deployment: PENDING, FAILED, or SUCCEEDED.</p>
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

/// <p>Updates a DomainName.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The collection of tags associated with a domain name.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Updates an Integration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateIntegrationRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The connection ID.</p>
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. Currently the only valid value is INTERNET, for connections through the public routable internet.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null.</p>
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
    /// <p>The integration type of an integration. One of the following:</p> <p>AWS: for integrating the route or method request with an AWS service action, including the Lambda function-invoking action. With the Lambda function-invoking action, this is referred to as the Lambda custom integration. With any other AWS service action, this is known as AWS integration. Supported only for WebSocket APIs.</p> <p>AWS_PROXY: for integrating the route or method request with the Lambda function-invoking action with the client request passed through as-is. This integration is also referred to as Lambda proxy integration.</p> <p>HTTP: for integrating the route or method request with an HTTP endpoint. This integration is also referred to as the HTTP custom integration. Supported only for WebSocket APIs.</p> <p>HTTP_PROXY: for integrating route or method request with an HTTP endpoint, with the client request passed through as-is. This is also referred to as HTTP proxy integration.</p> <p>MOCK: for integrating the route or method request with API Gateway as a "loopback" endpoint without invoking any backend. Supported only for WebSocket APIs.</p>
    #[serde(rename = "IntegrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    /// <p>For a Lambda proxy integration, this is the URI of the Lambda function.</p>
    #[serde(rename = "IntegrationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    /// <p>Specifies the pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the requestTemplates property on the Integration resource. There are three valid values: WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and NEVER. Supported only for WebSocket APIs.</p> <p>WHEN_NO_MATCH passes the request body for unmapped content types through to the integration backend without transformation.</p> <p>NEVER rejects unmapped content types with an HTTP 415 Unsupported Media Type response.</p> <p>WHEN_NO_TEMPLATES allows pass-through when the integration has no content types mapped to templates. However, if there is at least one content type defined, unmapped content types will be rejected with the same HTTP 415 Unsupported Media Type response.</p>
    #[serde(rename = "PassthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>Specifies the format of the payload sent to an integration. Required for HTTP APIs. Currently, the only supported value is 1.0.</p>
    #[serde(rename = "PayloadFormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_version: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method request to the backend. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the backend. The method request parameter value must match the pattern of method.request.<replaceable>{location}</replaceable>.<replaceable>{name}</replaceable>
    /// , where
    /// <replaceable>{location}</replaceable>
    /// is querystring, path, or header; and
    /// <replaceable>{name}</replaceable>
    /// must be a valid and unique method request parameter name. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds for WebSocket APIs. The default value is 5,000 milliseconds, or 5 seconds for HTTP APIs.</p>
    #[serde(rename = "TimeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
}

/// <p>Updates an IntegrationResponses.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateIntegrationResponseRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
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
    /// <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.<replaceable>{name}</replaceable>
    /// , where name is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.<replaceable>{name}</replaceable>
    /// or integration.response.body.<replaceable>{JSON-expression}</replaceable>
    /// , where
    /// <replaceable>{name}</replaceable>
    /// is a valid and unique response header name and
    /// <replaceable>{JSON-expression}</replaceable>
    /// is a valid JSON expression without the $ prefix.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration response. Supported only for WebSocket APIs.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateIntegrationResponseResponse {
    /// <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
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
    /// <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.{name}, where name is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.{name} or integration.response.body.{JSON-expression}, where name is a valid and unique response header name and JSON-expression is a valid JSON expression without the $ prefix.</p>
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expressions for the integration response.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateIntegrationResult {
    /// <p>Specifies whether an integration is managed by API Gateway. If you created an API using using quick create, the resulting integration is managed by API Gateway. You can update a managed integration, but you can't delete it.</p>
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    /// <p>The connection ID.</p>
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. Currently the only valid value is INTERNET, for connections through the public routable internet.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    #[serde(rename = "ContentHandlingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null.</p>
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
    /// <p>The integration response selection expression for the integration. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-integration-response-selection-expressions">Integration Response Selection Expressions</a>.</p>
    #[serde(rename = "IntegrationResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_selection_expression: Option<String>,
    /// <p>The integration type of an integration. One of the following:</p> <p>AWS: for integrating the route or method request with an AWS service action, including the Lambda function-invoking action. With the Lambda function-invoking action, this is referred to as the Lambda custom integration. With any other AWS service action, this is known as AWS integration. Supported only for WebSocket APIs.</p> <p>AWS_PROXY: for integrating the route or method request with the Lambda function-invoking action with the client request passed through as-is. This integration is also referred to as Lambda proxy integration.</p> <p>HTTP: for integrating the route or method request with an HTTP endpoint. This integration is also referred to as the HTTP custom integration. Supported only for WebSocket APIs.</p> <p>HTTP_PROXY: for integrating route or method request with an HTTP endpoint, with the client request passed through as-is. This is also referred to as HTTP proxy integration.</p> <p>MOCK: for integrating the route or method request with API Gateway as a "loopback" endpoint without invoking any backend. Supported only for WebSocket APIs.</p>
    #[serde(rename = "IntegrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    /// <p>For a Lambda proxy integration, this is the URI of the Lambda function.</p>
    #[serde(rename = "IntegrationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    /// <p>Specifies the pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the requestTemplates property on the Integration resource. There are three valid values: WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and NEVER. Supported only for WebSocket APIs.</p> <p>WHEN_NO_MATCH passes the request body for unmapped content types through to the integration backend without transformation.</p> <p>NEVER rejects unmapped content types with an HTTP 415 Unsupported Media Type response.</p> <p>WHEN_NO_TEMPLATES allows pass-through when the integration has no content types mapped to templates. However, if there is at least one content type defined, unmapped content types will be rejected with the same HTTP 415 Unsupported Media Type response.</p>
    #[serde(rename = "PassthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>Specifies the format of the payload sent to an integration. Required for HTTP APIs. Currently, the only supported value is 1.0.</p>
    #[serde(rename = "PayloadFormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_version: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method request to the backend. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the backend. The method request parameter value must match the pattern of method.request.<replaceable>{location}</replaceable>.<replaceable>{name}</replaceable>
    /// , where
    /// <replaceable>{location}</replaceable>
    /// is querystring, path, or header; and
    /// <replaceable>{name}</replaceable>
    /// must be a valid and unique method request parameter name. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template selection expression for the integration. Supported only for WebSocket APIs.</p>
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds for WebSocket APIs. The default value is 5,000 milliseconds, or 5 seconds for HTTP APIs.</p>
    #[serde(rename = "TimeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
}

/// <p>Updates a Model.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The schema for the model. For application/json models, this should be JSON schema draft 4 model.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The schema for the model. For application/json models, this should be JSON schema draft 4 model.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

/// <p>Updates a Route.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRouteRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Specifies whether an API key is required for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ApiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>The authorization scopes supported by this route.</p>
    #[serde(rename = "AuthorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The authorization type for the route. For WebSocket APIs, valid values are NONE for open access, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer For HTTP APIs, valid values are NONE for open access, or JWT for using JSON Web Tokens.</p>
    #[serde(rename = "AuthorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of the Authorizer resource to be associated with this route. The authorizer identifier is generated by API Gateway when you created the authorizer.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The model selection expression for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The operation name for the route.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The request models for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The request parameters for the route. Supported only for WebSocket APIs.</p>
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
    /// <p>The route response selection expression for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RouteResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    /// <p>The target for the route.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// <p>Updates a RouteResponse.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRouteResponseRequest {
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>The model selection expression for the route response. Supported only for WebSocket APIs.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRouteResponseResponse {
    /// <p>Represents the model selection expression of a route response. Supported only for WebSocket APIs.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRouteResult {
    /// <p>Specifies whether a route is managed by API Gateway. If you created an API using quick create, the $default route is managed by API Gateway. You can't modify the $default route key.</p>
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    /// <p>Specifies whether an API key is required for this route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ApiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>A list of authorization scopes configured on a route. The scopes are used with a JWT authorizer to authorize the method invocation. The authorization works by matching the route scopes against the scopes parsed from the access token in the incoming request. The method invocation is authorized if any route scope matches a claimed scope in the access token. Otherwise, the invocation is not authorized. When the route scope is configured, the client must provide an access token instead of an identity token for authorization purposes.</p>
    #[serde(rename = "AuthorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The authorization type for the route. For WebSocket APIs, valid values are NONE for open access, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer For HTTP APIs, valid values are NONE for open access, or JWT for using JSON Web Tokens.</p>
    #[serde(rename = "AuthorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of the Authorizer resource to be associated with this route. The authorizer identifier is generated by API Gateway when you created the authorizer.</p>
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The model selection expression for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "ModelSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    /// <p>The operation name for the route.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The request models for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RequestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>The request parameters for the route. Supported only for WebSocket APIs.</p>
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
    /// <p>The route response selection expression for the route. Supported only for WebSocket APIs.</p>
    #[serde(rename = "RouteResponseSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    /// <p>The target for the route.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// <p>Updates a Stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateStageRequest {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>The API identifier.</p>
    #[serde(rename = "ApiId")]
    pub api_id: String,
    /// <p>Specifies whether updates to an API automatically trigger a new deployment. The default value is false.</p>
    #[serde(rename = "AutoDeploy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    /// <p>The identifier of a client certificate for a Stage.</p>
    #[serde(rename = "ClientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>The default route settings for the stage.</p>
    #[serde(rename = "DefaultRouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<RouteSettings>,
    /// <p>The deployment identifier for the API stage. Can't be updated if autoDeploy is enabled.</p>
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
    /// <p>The stage name. Stage names can only contain alphanumeric characters, hyphens, and underscores. Maximum length is 128 characters.</p>
    #[serde(rename = "StageName")]
    pub stage_name: String,
    /// <p>A map that defines the stage variables for a Stage. Variable names can have alphanumeric and underscore characters, and the values must match [A-Za-z0-9-._~:/?#&amp;=,]+. Supported only for WebSocket APIs.</p>
    #[serde(rename = "StageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateStageResponse {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "AccessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>Specifies whether a stage is managed by API Gateway. If you created an API using quick create, the $default stage is managed by API Gateway. You can't modify the $default stage.</p>
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    /// <p>Specifies whether updates to an API automatically trigger a new deployment. The default value is false.</p>
    #[serde(rename = "AutoDeploy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    /// <p>The identifier of a client certificate for a Stage. Supported only for WebSocket APIs.</p>
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
    /// <p>The identifier of the Deployment that the Stage is associated with. Can't be updated if autoDeploy is enabled.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The description of the stage.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Describes the status of the last deployment of a stage. Supported only for stages with autoDeploy enabled.</p>
    #[serde(rename = "LastDeploymentStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_status_message: Option<String>,
    /// <p>The timestamp when the stage was last updated.</p>
    #[serde(rename = "LastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>Route settings for the stage, by routeKey.</p>
    #[serde(rename = "RouteSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<::std::collections::HashMap<String, RouteSettings>>,
    /// <p>The name of the stage.</p>
    #[serde(rename = "StageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    /// <p>A map that defines the stage variables for a stage resource. Variable names can have alphanumeric and underscore characters, and the values must match [A-Za-z0-9-._~:/?#&amp;=,]+. Supported only for WebSocket APIs.</p>
    #[serde(rename = "StageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// Errors returned by CreateApi
#[derive(Debug, PartialEq)]
pub enum CreateApiError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateApiError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateApiError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateApiError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateApiError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateApiError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateApiError {}
/// Errors returned by CreateApiMapping
#[derive(Debug, PartialEq)]
pub enum CreateApiMappingError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateApiMappingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateApiMappingError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateApiMappingError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateApiMappingError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateApiMappingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateApiMappingError {}
/// Errors returned by CreateAuthorizer
#[derive(Debug, PartialEq)]
pub enum CreateAuthorizerError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAuthorizerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAuthorizerError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateAuthorizerError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateAuthorizerError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateAuthorizerError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAuthorizerError {}
/// Errors returned by CreateDeployment
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDeploymentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDeploymentError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDeploymentError {}
/// Errors returned by CreateDomainName
#[derive(Debug, PartialEq)]
pub enum CreateDomainNameError {
    AccessDenied(String),
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl CreateDomainNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDomainNameError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDomainNameError::AccessDenied(err.msg))
                }
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDomainNameError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDomainNameError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateDomainNameError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateDomainNameError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateDomainNameError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateDomainNameError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDomainNameError {}
/// Errors returned by CreateIntegration
#[derive(Debug, PartialEq)]
pub enum CreateIntegrationError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateIntegrationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateIntegrationError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateIntegrationError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateIntegrationError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateIntegrationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIntegrationError {}
/// Errors returned by CreateIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum CreateIntegrationResponseError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateIntegrationResponseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateIntegrationResponseError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateIntegrationResponseError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateIntegrationResponseError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateIntegrationResponseError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIntegrationResponseError {}
/// Errors returned by CreateModel
#[derive(Debug, PartialEq)]
pub enum CreateModelError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateModelError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateModelError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateModelError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateModelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateModelError {}
/// Errors returned by CreateRoute
#[derive(Debug, PartialEq)]
pub enum CreateRouteError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRouteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRouteError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateRouteError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateRouteError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateRouteError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRouteError {}
/// Errors returned by CreateRouteResponse
#[derive(Debug, PartialEq)]
pub enum CreateRouteResponseError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRouteResponseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRouteResponseError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateRouteResponseError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateRouteResponseError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateRouteResponseError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRouteResponseError {}
/// Errors returned by CreateStage
#[derive(Debug, PartialEq)]
pub enum CreateStageError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateStageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateStageError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateStageError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateStageError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateStageError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateStageError {}
/// Errors returned by DeleteApi
#[derive(Debug, PartialEq)]
pub enum DeleteApiError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApiError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApiError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteApiError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApiError {}
/// Errors returned by DeleteApiMapping
#[derive(Debug, PartialEq)]
pub enum DeleteApiMappingError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApiMappingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApiMappingError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteApiMappingError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteApiMappingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApiMappingError {}
/// Errors returned by DeleteAuthorizer
#[derive(Debug, PartialEq)]
pub enum DeleteAuthorizerError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAuthorizerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAuthorizerError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteAuthorizerError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAuthorizerError {}
/// Errors returned by DeleteCorsConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteCorsConfigurationError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl DeleteCorsConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCorsConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteCorsConfigurationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteCorsConfigurationError::TooManyRequests(
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
impl fmt::Display for DeleteCorsConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCorsConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteCorsConfigurationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCorsConfigurationError {}
/// Errors returned by DeleteDeployment
#[derive(Debug, PartialEq)]
pub enum DeleteDeploymentError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDeploymentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDeploymentError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteDeploymentError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDeploymentError {}
/// Errors returned by DeleteDomainName
#[derive(Debug, PartialEq)]
pub enum DeleteDomainNameError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDomainNameError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDomainNameError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteDomainNameError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDomainNameError {}
/// Errors returned by DeleteIntegration
#[derive(Debug, PartialEq)]
pub enum DeleteIntegrationError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteIntegrationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteIntegrationError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteIntegrationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteIntegrationError {}
/// Errors returned by DeleteIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum DeleteIntegrationResponseError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteIntegrationResponseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteIntegrationResponseError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteIntegrationResponseError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteIntegrationResponseError {}
/// Errors returned by DeleteModel
#[derive(Debug, PartialEq)]
pub enum DeleteModelError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteModelError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteModelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteModelError {}
/// Errors returned by DeleteRoute
#[derive(Debug, PartialEq)]
pub enum DeleteRouteError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRouteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRouteError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteRouteError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRouteError {}
/// Errors returned by DeleteRouteResponse
#[derive(Debug, PartialEq)]
pub enum DeleteRouteResponseError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRouteResponseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRouteResponseError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteRouteResponseError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRouteResponseError {}
/// Errors returned by DeleteRouteSettings
#[derive(Debug, PartialEq)]
pub enum DeleteRouteSettingsError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl DeleteRouteSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRouteSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRouteSettingsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteRouteSettingsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRouteSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRouteSettingsError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteRouteSettingsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRouteSettingsError {}
/// Errors returned by DeleteStage
#[derive(Debug, PartialEq)]
pub enum DeleteStageError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteStageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteStageError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteStageError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteStageError {}
/// Errors returned by GetApi
#[derive(Debug, PartialEq)]
pub enum GetApiError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetApiError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApiError::NotFound(ref cause) => write!(f, "{}", cause),
            GetApiError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApiError {}
/// Errors returned by GetApiMapping
#[derive(Debug, PartialEq)]
pub enum GetApiMappingError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetApiMappingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApiMappingError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetApiMappingError::NotFound(ref cause) => write!(f, "{}", cause),
            GetApiMappingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApiMappingError {}
/// Errors returned by GetApiMappings
#[derive(Debug, PartialEq)]
pub enum GetApiMappingsError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetApiMappingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApiMappingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetApiMappingsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetApiMappingsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApiMappingsError {}
/// Errors returned by GetApis
#[derive(Debug, PartialEq)]
pub enum GetApisError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetApisError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApisError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetApisError::NotFound(ref cause) => write!(f, "{}", cause),
            GetApisError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApisError {}
/// Errors returned by GetAuthorizer
#[derive(Debug, PartialEq)]
pub enum GetAuthorizerError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAuthorizerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAuthorizerError::NotFound(ref cause) => write!(f, "{}", cause),
            GetAuthorizerError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAuthorizerError {}
/// Errors returned by GetAuthorizers
#[derive(Debug, PartialEq)]
pub enum GetAuthorizersError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAuthorizersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAuthorizersError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetAuthorizersError::NotFound(ref cause) => write!(f, "{}", cause),
            GetAuthorizersError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAuthorizersError {}
/// Errors returned by GetDeployment
#[derive(Debug, PartialEq)]
pub enum GetDeploymentError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDeploymentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeploymentError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDeploymentError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDeploymentError {}
/// Errors returned by GetDeployments
#[derive(Debug, PartialEq)]
pub enum GetDeploymentsError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDeploymentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeploymentsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDeploymentsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDeploymentsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDeploymentsError {}
/// Errors returned by GetDomainName
#[derive(Debug, PartialEq)]
pub enum GetDomainNameError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDomainNameError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDomainNameError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDomainNameError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDomainNameError {}
/// Errors returned by GetDomainNames
#[derive(Debug, PartialEq)]
pub enum GetDomainNamesError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDomainNamesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDomainNamesError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDomainNamesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDomainNamesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDomainNamesError {}
/// Errors returned by GetIntegration
#[derive(Debug, PartialEq)]
pub enum GetIntegrationError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetIntegrationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetIntegrationError::NotFound(ref cause) => write!(f, "{}", cause),
            GetIntegrationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetIntegrationError {}
/// Errors returned by GetIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum GetIntegrationResponseError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetIntegrationResponseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetIntegrationResponseError::NotFound(ref cause) => write!(f, "{}", cause),
            GetIntegrationResponseError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetIntegrationResponseError {}
/// Errors returned by GetIntegrationResponses
#[derive(Debug, PartialEq)]
pub enum GetIntegrationResponsesError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetIntegrationResponsesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetIntegrationResponsesError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetIntegrationResponsesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetIntegrationResponsesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetIntegrationResponsesError {}
/// Errors returned by GetIntegrations
#[derive(Debug, PartialEq)]
pub enum GetIntegrationsError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetIntegrationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetIntegrationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetIntegrationsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetIntegrationsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetIntegrationsError {}
/// Errors returned by GetModel
#[derive(Debug, PartialEq)]
pub enum GetModelError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetModelError::NotFound(ref cause) => write!(f, "{}", cause),
            GetModelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetModelError {}
/// Errors returned by GetModelTemplate
#[derive(Debug, PartialEq)]
pub enum GetModelTemplateError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetModelTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetModelTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            GetModelTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetModelTemplateError {}
/// Errors returned by GetModels
#[derive(Debug, PartialEq)]
pub enum GetModelsError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetModelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetModelsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetModelsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetModelsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetModelsError {}
/// Errors returned by GetRoute
#[derive(Debug, PartialEq)]
pub enum GetRouteError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRouteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRouteError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRouteError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRouteError {}
/// Errors returned by GetRouteResponse
#[derive(Debug, PartialEq)]
pub enum GetRouteResponseError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRouteResponseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRouteResponseError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRouteResponseError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRouteResponseError {}
/// Errors returned by GetRouteResponses
#[derive(Debug, PartialEq)]
pub enum GetRouteResponsesError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRouteResponsesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRouteResponsesError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetRouteResponsesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRouteResponsesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRouteResponsesError {}
/// Errors returned by GetRoutes
#[derive(Debug, PartialEq)]
pub enum GetRoutesError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRoutesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRoutesError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetRoutesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRoutesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRoutesError {}
/// Errors returned by GetStage
#[derive(Debug, PartialEq)]
pub enum GetStageError {
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetStageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetStageError::NotFound(ref cause) => write!(f, "{}", cause),
            GetStageError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetStageError {}
/// Errors returned by GetStages
#[derive(Debug, PartialEq)]
pub enum GetStagesError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetStagesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetStagesError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetStagesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetStagesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetStagesError {}
/// Errors returned by GetTags
#[derive(Debug, PartialEq)]
pub enum GetTagsError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl GetTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetTagsError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(GetTagsError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetTagsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetTagsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTagsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetTagsError::Conflict(ref cause) => write!(f, "{}", cause),
            GetTagsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetTagsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTagsError {}
/// Errors returned by ImportApi
#[derive(Debug, PartialEq)]
pub enum ImportApiError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl ImportApiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportApiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ImportApiError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ImportApiError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ImportApiError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ImportApiError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ImportApiError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportApiError::BadRequest(ref cause) => write!(f, "{}", cause),
            ImportApiError::Conflict(ref cause) => write!(f, "{}", cause),
            ImportApiError::NotFound(ref cause) => write!(f, "{}", cause),
            ImportApiError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ImportApiError {}
/// Errors returned by ReimportApi
#[derive(Debug, PartialEq)]
pub enum ReimportApiError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl ReimportApiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ReimportApiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ReimportApiError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ReimportApiError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ReimportApiError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ReimportApiError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ReimportApiError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReimportApiError::BadRequest(ref cause) => write!(f, "{}", cause),
            ReimportApiError::Conflict(ref cause) => write!(f, "{}", cause),
            ReimportApiError::NotFound(ref cause) => write!(f, "{}", cause),
            ReimportApiError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ReimportApiError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(TagResourceError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TagResourceError::TooManyRequests(err.msg))
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
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::Conflict(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
    NotFound(String),
    /// <p>A limit has been exceeded. See the accompanying error message for details.</p>
    TooManyRequests(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UntagResourceError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyRequests(err.msg))
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
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Conflict(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateApi
#[derive(Debug, PartialEq)]
pub enum UpdateApiError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateApiError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApiError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateApiError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateApiError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateApiError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApiError {}
/// Errors returned by UpdateApiMapping
#[derive(Debug, PartialEq)]
pub enum UpdateApiMappingError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateApiMappingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApiMappingError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateApiMappingError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateApiMappingError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateApiMappingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApiMappingError {}
/// Errors returned by UpdateAuthorizer
#[derive(Debug, PartialEq)]
pub enum UpdateAuthorizerError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAuthorizerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAuthorizerError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateAuthorizerError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateAuthorizerError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateAuthorizerError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAuthorizerError {}
/// Errors returned by UpdateDeployment
#[derive(Debug, PartialEq)]
pub enum UpdateDeploymentError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDeploymentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDeploymentError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDeploymentError {}
/// Errors returned by UpdateDomainName
#[derive(Debug, PartialEq)]
pub enum UpdateDomainNameError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDomainNameError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDomainNameError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateDomainNameError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateDomainNameError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateDomainNameError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDomainNameError {}
/// Errors returned by UpdateIntegration
#[derive(Debug, PartialEq)]
pub enum UpdateIntegrationError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateIntegrationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateIntegrationError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateIntegrationError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateIntegrationError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateIntegrationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateIntegrationError {}
/// Errors returned by UpdateIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum UpdateIntegrationResponseError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateIntegrationResponseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateIntegrationResponseError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateIntegrationResponseError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateIntegrationResponseError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateIntegrationResponseError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateIntegrationResponseError {}
/// Errors returned by UpdateModel
#[derive(Debug, PartialEq)]
pub enum UpdateModelError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateModelError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateModelError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateModelError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateModelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateModelError {}
/// Errors returned by UpdateRoute
#[derive(Debug, PartialEq)]
pub enum UpdateRouteError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRouteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRouteError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateRouteError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateRouteError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateRouteError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRouteError {}
/// Errors returned by UpdateRouteResponse
#[derive(Debug, PartialEq)]
pub enum UpdateRouteResponseError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRouteResponseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRouteResponseError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateRouteResponseError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateRouteResponseError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateRouteResponseError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRouteResponseError {}
/// Errors returned by UpdateStage
#[derive(Debug, PartialEq)]
pub enum UpdateStageError {
    /// <p>The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. See the accompanying error message for details.</p>
    Conflict(String),
    /// <p>The resource specified in the request was not found. See the message field for more information.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateStageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateStageError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateStageError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateStageError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateStageError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateStageError {}
/// Trait representing the capabilities of the AmazonApiGatewayV2 API. AmazonApiGatewayV2 clients implement this trait.
#[async_trait]
pub trait ApiGatewayV2 {
    /// <p>Creates an Api resource.</p>
    async fn create_api(
        &self,
        input: CreateApiRequest,
    ) -> Result<CreateApiResponse, RusotoError<CreateApiError>>;

    /// <p>Creates an API mapping.</p>
    async fn create_api_mapping(
        &self,
        input: CreateApiMappingRequest,
    ) -> Result<CreateApiMappingResponse, RusotoError<CreateApiMappingError>>;

    /// <p>Creates an Authorizer for an API.</p>
    async fn create_authorizer(
        &self,
        input: CreateAuthorizerRequest,
    ) -> Result<CreateAuthorizerResponse, RusotoError<CreateAuthorizerError>>;

    /// <p>Creates a Deployment for an API.</p>
    async fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> Result<CreateDeploymentResponse, RusotoError<CreateDeploymentError>>;

    /// <p>Creates a domain name.</p>
    async fn create_domain_name(
        &self,
        input: CreateDomainNameRequest,
    ) -> Result<CreateDomainNameResponse, RusotoError<CreateDomainNameError>>;

    /// <p>Creates an Integration.</p>
    async fn create_integration(
        &self,
        input: CreateIntegrationRequest,
    ) -> Result<CreateIntegrationResult, RusotoError<CreateIntegrationError>>;

    /// <p>Creates an IntegrationResponses.</p>
    async fn create_integration_response(
        &self,
        input: CreateIntegrationResponseRequest,
    ) -> Result<CreateIntegrationResponseResponse, RusotoError<CreateIntegrationResponseError>>;

    /// <p>Creates a Model for an API.</p>
    async fn create_model(
        &self,
        input: CreateModelRequest,
    ) -> Result<CreateModelResponse, RusotoError<CreateModelError>>;

    /// <p>Creates a Route for an API.</p>
    async fn create_route(
        &self,
        input: CreateRouteRequest,
    ) -> Result<CreateRouteResult, RusotoError<CreateRouteError>>;

    /// <p>Creates a RouteResponse for a Route.</p>
    async fn create_route_response(
        &self,
        input: CreateRouteResponseRequest,
    ) -> Result<CreateRouteResponseResponse, RusotoError<CreateRouteResponseError>>;

    /// <p>Creates a Stage for an API.</p>
    async fn create_stage(
        &self,
        input: CreateStageRequest,
    ) -> Result<CreateStageResponse, RusotoError<CreateStageError>>;

    /// <p>Deletes an Api resource.</p>
    async fn delete_api(&self, input: DeleteApiRequest) -> Result<(), RusotoError<DeleteApiError>>;

    /// <p>Deletes an API mapping.</p>
    async fn delete_api_mapping(
        &self,
        input: DeleteApiMappingRequest,
    ) -> Result<(), RusotoError<DeleteApiMappingError>>;

    /// <p>Deletes an Authorizer.</p>
    async fn delete_authorizer(
        &self,
        input: DeleteAuthorizerRequest,
    ) -> Result<(), RusotoError<DeleteAuthorizerError>>;

    /// <p>Deletes a CORS configuration.</p>
    async fn delete_cors_configuration(
        &self,
        input: DeleteCorsConfigurationRequest,
    ) -> Result<(), RusotoError<DeleteCorsConfigurationError>>;

    /// <p>Deletes a Deployment.</p>
    async fn delete_deployment(
        &self,
        input: DeleteDeploymentRequest,
    ) -> Result<(), RusotoError<DeleteDeploymentError>>;

    /// <p>Deletes a domain name.</p>
    async fn delete_domain_name(
        &self,
        input: DeleteDomainNameRequest,
    ) -> Result<(), RusotoError<DeleteDomainNameError>>;

    /// <p>Deletes an Integration.</p>
    async fn delete_integration(
        &self,
        input: DeleteIntegrationRequest,
    ) -> Result<(), RusotoError<DeleteIntegrationError>>;

    /// <p>Deletes an IntegrationResponses.</p>
    async fn delete_integration_response(
        &self,
        input: DeleteIntegrationResponseRequest,
    ) -> Result<(), RusotoError<DeleteIntegrationResponseError>>;

    /// <p>Deletes a Model.</p>
    async fn delete_model(
        &self,
        input: DeleteModelRequest,
    ) -> Result<(), RusotoError<DeleteModelError>>;

    /// <p>Deletes a Route.</p>
    async fn delete_route(
        &self,
        input: DeleteRouteRequest,
    ) -> Result<(), RusotoError<DeleteRouteError>>;

    /// <p>Deletes a RouteResponse.</p>
    async fn delete_route_response(
        &self,
        input: DeleteRouteResponseRequest,
    ) -> Result<(), RusotoError<DeleteRouteResponseError>>;

    /// <p>Deletes the RouteSettings for a stage.</p>
    async fn delete_route_settings(
        &self,
        input: DeleteRouteSettingsRequest,
    ) -> Result<(), RusotoError<DeleteRouteSettingsError>>;

    /// <p>Deletes a Stage.</p>
    async fn delete_stage(
        &self,
        input: DeleteStageRequest,
    ) -> Result<(), RusotoError<DeleteStageError>>;

    /// <p>Gets an Api resource.</p>
    async fn get_api(
        &self,
        input: GetApiRequest,
    ) -> Result<GetApiResponse, RusotoError<GetApiError>>;

    /// <p>Gets an API mapping.</p>
    async fn get_api_mapping(
        &self,
        input: GetApiMappingRequest,
    ) -> Result<GetApiMappingResponse, RusotoError<GetApiMappingError>>;

    /// <p>Gets API mappings.</p>
    async fn get_api_mappings(
        &self,
        input: GetApiMappingsRequest,
    ) -> Result<GetApiMappingsResponse, RusotoError<GetApiMappingsError>>;

    /// <p>Gets a collection of Api resources.</p>
    async fn get_apis(
        &self,
        input: GetApisRequest,
    ) -> Result<GetApisResponse, RusotoError<GetApisError>>;

    /// <p>Gets an Authorizer.</p>
    async fn get_authorizer(
        &self,
        input: GetAuthorizerRequest,
    ) -> Result<GetAuthorizerResponse, RusotoError<GetAuthorizerError>>;

    /// <p>Gets the Authorizers for an API.</p>
    async fn get_authorizers(
        &self,
        input: GetAuthorizersRequest,
    ) -> Result<GetAuthorizersResponse, RusotoError<GetAuthorizersError>>;

    /// <p>Gets a Deployment.</p>
    async fn get_deployment(
        &self,
        input: GetDeploymentRequest,
    ) -> Result<GetDeploymentResponse, RusotoError<GetDeploymentError>>;

    /// <p>Gets the Deployments for an API.</p>
    async fn get_deployments(
        &self,
        input: GetDeploymentsRequest,
    ) -> Result<GetDeploymentsResponse, RusotoError<GetDeploymentsError>>;

    /// <p>Gets a domain name.</p>
    async fn get_domain_name(
        &self,
        input: GetDomainNameRequest,
    ) -> Result<GetDomainNameResponse, RusotoError<GetDomainNameError>>;

    /// <p>Gets the domain names for an AWS account.</p>
    async fn get_domain_names(
        &self,
        input: GetDomainNamesRequest,
    ) -> Result<GetDomainNamesResponse, RusotoError<GetDomainNamesError>>;

    /// <p>Gets an Integration.</p>
    async fn get_integration(
        &self,
        input: GetIntegrationRequest,
    ) -> Result<GetIntegrationResult, RusotoError<GetIntegrationError>>;

    /// <p>Gets an IntegrationResponses.</p>
    async fn get_integration_response(
        &self,
        input: GetIntegrationResponseRequest,
    ) -> Result<GetIntegrationResponseResponse, RusotoError<GetIntegrationResponseError>>;

    /// <p>Gets the IntegrationResponses for an Integration.</p>
    async fn get_integration_responses(
        &self,
        input: GetIntegrationResponsesRequest,
    ) -> Result<GetIntegrationResponsesResponse, RusotoError<GetIntegrationResponsesError>>;

    /// <p>Gets the Integrations for an API.</p>
    async fn get_integrations(
        &self,
        input: GetIntegrationsRequest,
    ) -> Result<GetIntegrationsResponse, RusotoError<GetIntegrationsError>>;

    /// <p>Gets a Model.</p>
    async fn get_model(
        &self,
        input: GetModelRequest,
    ) -> Result<GetModelResponse, RusotoError<GetModelError>>;

    /// <p>Gets a model template.</p>
    async fn get_model_template(
        &self,
        input: GetModelTemplateRequest,
    ) -> Result<GetModelTemplateResponse, RusotoError<GetModelTemplateError>>;

    /// <p>Gets the Models for an API.</p>
    async fn get_models(
        &self,
        input: GetModelsRequest,
    ) -> Result<GetModelsResponse, RusotoError<GetModelsError>>;

    /// <p>Gets a Route.</p>
    async fn get_route(
        &self,
        input: GetRouteRequest,
    ) -> Result<GetRouteResult, RusotoError<GetRouteError>>;

    /// <p>Gets a RouteResponse.</p>
    async fn get_route_response(
        &self,
        input: GetRouteResponseRequest,
    ) -> Result<GetRouteResponseResponse, RusotoError<GetRouteResponseError>>;

    /// <p>Gets the RouteResponses for a Route.</p>
    async fn get_route_responses(
        &self,
        input: GetRouteResponsesRequest,
    ) -> Result<GetRouteResponsesResponse, RusotoError<GetRouteResponsesError>>;

    /// <p>Gets the Routes for an API.</p>
    async fn get_routes(
        &self,
        input: GetRoutesRequest,
    ) -> Result<GetRoutesResponse, RusotoError<GetRoutesError>>;

    /// <p>Gets a Stage.</p>
    async fn get_stage(
        &self,
        input: GetStageRequest,
    ) -> Result<GetStageResponse, RusotoError<GetStageError>>;

    /// <p>Gets the Stages for an API.</p>
    async fn get_stages(
        &self,
        input: GetStagesRequest,
    ) -> Result<GetStagesResponse, RusotoError<GetStagesError>>;

    /// <p>Gets a collection of Tag resources.</p>
    async fn get_tags(
        &self,
        input: GetTagsRequest,
    ) -> Result<GetTagsResponse, RusotoError<GetTagsError>>;

    /// <p>Imports an API.</p>
    async fn import_api(
        &self,
        input: ImportApiRequest,
    ) -> Result<ImportApiResponse, RusotoError<ImportApiError>>;

    /// <p>Puts an Api resource.</p>
    async fn reimport_api(
        &self,
        input: ReimportApiRequest,
    ) -> Result<ReimportApiResponse, RusotoError<ReimportApiError>>;

    /// <p>Creates a new Tag resource to represent a tag.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Deletes a Tag.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Updates an Api resource.</p>
    async fn update_api(
        &self,
        input: UpdateApiRequest,
    ) -> Result<UpdateApiResponse, RusotoError<UpdateApiError>>;

    /// <p>The API mapping.</p>
    async fn update_api_mapping(
        &self,
        input: UpdateApiMappingRequest,
    ) -> Result<UpdateApiMappingResponse, RusotoError<UpdateApiMappingError>>;

    /// <p>Updates an Authorizer.</p>
    async fn update_authorizer(
        &self,
        input: UpdateAuthorizerRequest,
    ) -> Result<UpdateAuthorizerResponse, RusotoError<UpdateAuthorizerError>>;

    /// <p>Updates a Deployment.</p>
    async fn update_deployment(
        &self,
        input: UpdateDeploymentRequest,
    ) -> Result<UpdateDeploymentResponse, RusotoError<UpdateDeploymentError>>;

    /// <p>Updates a domain name.</p>
    async fn update_domain_name(
        &self,
        input: UpdateDomainNameRequest,
    ) -> Result<UpdateDomainNameResponse, RusotoError<UpdateDomainNameError>>;

    /// <p>Updates an Integration.</p>
    async fn update_integration(
        &self,
        input: UpdateIntegrationRequest,
    ) -> Result<UpdateIntegrationResult, RusotoError<UpdateIntegrationError>>;

    /// <p>Updates an IntegrationResponses.</p>
    async fn update_integration_response(
        &self,
        input: UpdateIntegrationResponseRequest,
    ) -> Result<UpdateIntegrationResponseResponse, RusotoError<UpdateIntegrationResponseError>>;

    /// <p>Updates a Model.</p>
    async fn update_model(
        &self,
        input: UpdateModelRequest,
    ) -> Result<UpdateModelResponse, RusotoError<UpdateModelError>>;

    /// <p>Updates a Route.</p>
    async fn update_route(
        &self,
        input: UpdateRouteRequest,
    ) -> Result<UpdateRouteResult, RusotoError<UpdateRouteError>>;

    /// <p>Updates a RouteResponse.</p>
    async fn update_route_response(
        &self,
        input: UpdateRouteResponseRequest,
    ) -> Result<UpdateRouteResponseResponse, RusotoError<UpdateRouteResponseError>>;

    /// <p>Updates a Stage.</p>
    async fn update_stage(
        &self,
        input: UpdateStageRequest,
    ) -> Result<UpdateStageResponse, RusotoError<UpdateStageError>>;
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
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ApiGatewayV2Client {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ApiGatewayV2Client {
        ApiGatewayV2Client { client, region }
    }
}

#[async_trait]
impl ApiGatewayV2 for ApiGatewayV2Client {
    /// <p>Creates an Api resource.</p>
    async fn create_api(
        &self,
        input: CreateApiRequest,
    ) -> Result<CreateApiResponse, RusotoError<CreateApiError>> {
        let request_uri = "/v2/apis";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateApiResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateApiError::from_response(response))
        }
    }

    /// <p>Creates an API mapping.</p>
    async fn create_api_mapping(
        &self,
        input: CreateApiMappingRequest,
    ) -> Result<CreateApiMappingResponse, RusotoError<CreateApiMappingError>> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}/apimappings",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateApiMappingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateApiMappingError::from_response(response))
        }
    }

    /// <p>Creates an Authorizer for an API.</p>
    async fn create_authorizer(
        &self,
        input: CreateAuthorizerRequest,
    ) -> Result<CreateAuthorizerResponse, RusotoError<CreateAuthorizerError>> {
        let request_uri = format!("/v2/apis/{api_id}/authorizers", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateAuthorizerResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAuthorizerError::from_response(response))
        }
    }

    /// <p>Creates a Deployment for an API.</p>
    async fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> Result<CreateDeploymentResponse, RusotoError<CreateDeploymentError>> {
        let request_uri = format!("/v2/apis/{api_id}/deployments", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDeploymentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDeploymentError::from_response(response))
        }
    }

    /// <p>Creates a domain name.</p>
    async fn create_domain_name(
        &self,
        input: CreateDomainNameRequest,
    ) -> Result<CreateDomainNameResponse, RusotoError<CreateDomainNameError>> {
        let request_uri = "/v2/domainnames";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDomainNameResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDomainNameError::from_response(response))
        }
    }

    /// <p>Creates an Integration.</p>
    async fn create_integration(
        &self,
        input: CreateIntegrationRequest,
    ) -> Result<CreateIntegrationResult, RusotoError<CreateIntegrationError>> {
        let request_uri = format!("/v2/apis/{api_id}/integrations", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateIntegrationResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIntegrationError::from_response(response))
        }
    }

    /// <p>Creates an IntegrationResponses.</p>
    async fn create_integration_response(
        &self,
        input: CreateIntegrationResponseRequest,
    ) -> Result<CreateIntegrationResponseResponse, RusotoError<CreateIntegrationResponseError>>
    {
        let request_uri = format!(
            "/v2/apis/{api_id}/integrations/{integration_id}/integrationresponses",
            api_id = input.api_id,
            integration_id = input.integration_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateIntegrationResponseResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIntegrationResponseError::from_response(response))
        }
    }

    /// <p>Creates a Model for an API.</p>
    async fn create_model(
        &self,
        input: CreateModelRequest,
    ) -> Result<CreateModelResponse, RusotoError<CreateModelError>> {
        let request_uri = format!("/v2/apis/{api_id}/models", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateModelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateModelError::from_response(response))
        }
    }

    /// <p>Creates a Route for an API.</p>
    async fn create_route(
        &self,
        input: CreateRouteRequest,
    ) -> Result<CreateRouteResult, RusotoError<CreateRouteError>> {
        let request_uri = format!("/v2/apis/{api_id}/routes", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateRouteResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRouteError::from_response(response))
        }
    }

    /// <p>Creates a RouteResponse for a Route.</p>
    async fn create_route_response(
        &self,
        input: CreateRouteResponseRequest,
    ) -> Result<CreateRouteResponseResponse, RusotoError<CreateRouteResponseError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}/routeresponses",
            api_id = input.api_id,
            route_id = input.route_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateRouteResponseResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRouteResponseError::from_response(response))
        }
    }

    /// <p>Creates a Stage for an API.</p>
    async fn create_stage(
        &self,
        input: CreateStageRequest,
    ) -> Result<CreateStageResponse, RusotoError<CreateStageError>> {
        let request_uri = format!("/v2/apis/{api_id}/stages", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateStageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateStageError::from_response(response))
        }
    }

    /// <p>Deletes an Api resource.</p>
    async fn delete_api(&self, input: DeleteApiRequest) -> Result<(), RusotoError<DeleteApiError>> {
        let request_uri = format!("/v2/apis/{api_id}", api_id = input.api_id);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteApiError::from_response(response))
        }
    }

    /// <p>Deletes an API mapping.</p>
    async fn delete_api_mapping(
        &self,
        input: DeleteApiMappingRequest,
    ) -> Result<(), RusotoError<DeleteApiMappingError>> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}/apimappings/{api_mapping_id}",
            api_mapping_id = input.api_mapping_id,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteApiMappingError::from_response(response))
        }
    }

    /// <p>Deletes an Authorizer.</p>
    async fn delete_authorizer(
        &self,
        input: DeleteAuthorizerRequest,
    ) -> Result<(), RusotoError<DeleteAuthorizerError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/authorizers/{authorizer_id}",
            api_id = input.api_id,
            authorizer_id = input.authorizer_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAuthorizerError::from_response(response))
        }
    }

    /// <p>Deletes a CORS configuration.</p>
    async fn delete_cors_configuration(
        &self,
        input: DeleteCorsConfigurationRequest,
    ) -> Result<(), RusotoError<DeleteCorsConfigurationError>> {
        let request_uri = format!("/v2/apis/{api_id}/cors", api_id = input.api_id);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteCorsConfigurationError::from_response(response))
        }
    }

    /// <p>Deletes a Deployment.</p>
    async fn delete_deployment(
        &self,
        input: DeleteDeploymentRequest,
    ) -> Result<(), RusotoError<DeleteDeploymentError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/deployments/{deployment_id}",
            api_id = input.api_id,
            deployment_id = input.deployment_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDeploymentError::from_response(response))
        }
    }

    /// <p>Deletes a domain name.</p>
    async fn delete_domain_name(
        &self,
        input: DeleteDomainNameRequest,
    ) -> Result<(), RusotoError<DeleteDomainNameError>> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDomainNameError::from_response(response))
        }
    }

    /// <p>Deletes an Integration.</p>
    async fn delete_integration(
        &self,
        input: DeleteIntegrationRequest,
    ) -> Result<(), RusotoError<DeleteIntegrationError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/integrations/{integration_id}",
            api_id = input.api_id,
            integration_id = input.integration_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteIntegrationError::from_response(response))
        }
    }

    /// <p>Deletes an IntegrationResponses.</p>
    async fn delete_integration_response(
        &self,
        input: DeleteIntegrationResponseRequest,
    ) -> Result<(), RusotoError<DeleteIntegrationResponseError>> {
        let request_uri = format!("/v2/apis/{api_id}/integrations/{integration_id}/integrationresponses/{integration_response_id}", api_id = input.api_id, integration_id = input.integration_id, integration_response_id = input.integration_response_id);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteIntegrationResponseError::from_response(response))
        }
    }

    /// <p>Deletes a Model.</p>
    async fn delete_model(
        &self,
        input: DeleteModelRequest,
    ) -> Result<(), RusotoError<DeleteModelError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/models/{model_id}",
            api_id = input.api_id,
            model_id = input.model_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteModelError::from_response(response))
        }
    }

    /// <p>Deletes a Route.</p>
    async fn delete_route(
        &self,
        input: DeleteRouteRequest,
    ) -> Result<(), RusotoError<DeleteRouteError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}",
            api_id = input.api_id,
            route_id = input.route_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRouteError::from_response(response))
        }
    }

    /// <p>Deletes a RouteResponse.</p>
    async fn delete_route_response(
        &self,
        input: DeleteRouteResponseRequest,
    ) -> Result<(), RusotoError<DeleteRouteResponseError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}/routeresponses/{route_response_id}",
            api_id = input.api_id,
            route_id = input.route_id,
            route_response_id = input.route_response_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRouteResponseError::from_response(response))
        }
    }

    /// <p>Deletes the RouteSettings for a stage.</p>
    async fn delete_route_settings(
        &self,
        input: DeleteRouteSettingsRequest,
    ) -> Result<(), RusotoError<DeleteRouteSettingsError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/stages/{stage_name}/routesettings/{route_key}",
            api_id = input.api_id,
            route_key = input.route_key,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRouteSettingsError::from_response(response))
        }
    }

    /// <p>Deletes a Stage.</p>
    async fn delete_stage(
        &self,
        input: DeleteStageRequest,
    ) -> Result<(), RusotoError<DeleteStageError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/stages/{stage_name}",
            api_id = input.api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteStageError::from_response(response))
        }
    }

    /// <p>Gets an Api resource.</p>
    async fn get_api(
        &self,
        input: GetApiRequest,
    ) -> Result<GetApiResponse, RusotoError<GetApiError>> {
        let request_uri = format!("/v2/apis/{api_id}", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetApiResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetApiError::from_response(response))
        }
    }

    /// <p>Gets an API mapping.</p>
    async fn get_api_mapping(
        &self,
        input: GetApiMappingRequest,
    ) -> Result<GetApiMappingResponse, RusotoError<GetApiMappingError>> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}/apimappings/{api_mapping_id}",
            api_mapping_id = input.api_mapping_id,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetApiMappingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetApiMappingError::from_response(response))
        }
    }

    /// <p>Gets API mappings.</p>
    async fn get_api_mappings(
        &self,
        input: GetApiMappingsRequest,
    ) -> Result<GetApiMappingsResponse, RusotoError<GetApiMappingsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetApiMappingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetApiMappingsError::from_response(response))
        }
    }

    /// <p>Gets a collection of Api resources.</p>
    async fn get_apis(
        &self,
        input: GetApisRequest,
    ) -> Result<GetApisResponse, RusotoError<GetApisError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetApisResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetApisError::from_response(response))
        }
    }

    /// <p>Gets an Authorizer.</p>
    async fn get_authorizer(
        &self,
        input: GetAuthorizerRequest,
    ) -> Result<GetAuthorizerResponse, RusotoError<GetAuthorizerError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/authorizers/{authorizer_id}",
            api_id = input.api_id,
            authorizer_id = input.authorizer_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAuthorizerResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAuthorizerError::from_response(response))
        }
    }

    /// <p>Gets the Authorizers for an API.</p>
    async fn get_authorizers(
        &self,
        input: GetAuthorizersRequest,
    ) -> Result<GetAuthorizersResponse, RusotoError<GetAuthorizersError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAuthorizersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAuthorizersError::from_response(response))
        }
    }

    /// <p>Gets a Deployment.</p>
    async fn get_deployment(
        &self,
        input: GetDeploymentRequest,
    ) -> Result<GetDeploymentResponse, RusotoError<GetDeploymentError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/deployments/{deployment_id}",
            api_id = input.api_id,
            deployment_id = input.deployment_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDeploymentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeploymentError::from_response(response))
        }
    }

    /// <p>Gets the Deployments for an API.</p>
    async fn get_deployments(
        &self,
        input: GetDeploymentsRequest,
    ) -> Result<GetDeploymentsResponse, RusotoError<GetDeploymentsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDeploymentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeploymentsError::from_response(response))
        }
    }

    /// <p>Gets a domain name.</p>
    async fn get_domain_name(
        &self,
        input: GetDomainNameRequest,
    ) -> Result<GetDomainNameResponse, RusotoError<GetDomainNameError>> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDomainNameResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDomainNameError::from_response(response))
        }
    }

    /// <p>Gets the domain names for an AWS account.</p>
    async fn get_domain_names(
        &self,
        input: GetDomainNamesRequest,
    ) -> Result<GetDomainNamesResponse, RusotoError<GetDomainNamesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDomainNamesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDomainNamesError::from_response(response))
        }
    }

    /// <p>Gets an Integration.</p>
    async fn get_integration(
        &self,
        input: GetIntegrationRequest,
    ) -> Result<GetIntegrationResult, RusotoError<GetIntegrationError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/integrations/{integration_id}",
            api_id = input.api_id,
            integration_id = input.integration_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetIntegrationResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetIntegrationError::from_response(response))
        }
    }

    /// <p>Gets an IntegrationResponses.</p>
    async fn get_integration_response(
        &self,
        input: GetIntegrationResponseRequest,
    ) -> Result<GetIntegrationResponseResponse, RusotoError<GetIntegrationResponseError>> {
        let request_uri = format!("/v2/apis/{api_id}/integrations/{integration_id}/integrationresponses/{integration_response_id}", api_id = input.api_id, integration_id = input.integration_id, integration_response_id = input.integration_response_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetIntegrationResponseResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetIntegrationResponseError::from_response(response))
        }
    }

    /// <p>Gets the IntegrationResponses for an Integration.</p>
    async fn get_integration_responses(
        &self,
        input: GetIntegrationResponsesRequest,
    ) -> Result<GetIntegrationResponsesResponse, RusotoError<GetIntegrationResponsesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetIntegrationResponsesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetIntegrationResponsesError::from_response(response))
        }
    }

    /// <p>Gets the Integrations for an API.</p>
    async fn get_integrations(
        &self,
        input: GetIntegrationsRequest,
    ) -> Result<GetIntegrationsResponse, RusotoError<GetIntegrationsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetIntegrationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetIntegrationsError::from_response(response))
        }
    }

    /// <p>Gets a Model.</p>
    async fn get_model(
        &self,
        input: GetModelRequest,
    ) -> Result<GetModelResponse, RusotoError<GetModelError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/models/{model_id}",
            api_id = input.api_id,
            model_id = input.model_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetModelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetModelError::from_response(response))
        }
    }

    /// <p>Gets a model template.</p>
    async fn get_model_template(
        &self,
        input: GetModelTemplateRequest,
    ) -> Result<GetModelTemplateResponse, RusotoError<GetModelTemplateError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/models/{model_id}/template",
            api_id = input.api_id,
            model_id = input.model_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetModelTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetModelTemplateError::from_response(response))
        }
    }

    /// <p>Gets the Models for an API.</p>
    async fn get_models(
        &self,
        input: GetModelsRequest,
    ) -> Result<GetModelsResponse, RusotoError<GetModelsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetModelsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetModelsError::from_response(response))
        }
    }

    /// <p>Gets a Route.</p>
    async fn get_route(
        &self,
        input: GetRouteRequest,
    ) -> Result<GetRouteResult, RusotoError<GetRouteError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}",
            api_id = input.api_id,
            route_id = input.route_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetRouteResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetRouteError::from_response(response))
        }
    }

    /// <p>Gets a RouteResponse.</p>
    async fn get_route_response(
        &self,
        input: GetRouteResponseRequest,
    ) -> Result<GetRouteResponseResponse, RusotoError<GetRouteResponseError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}/routeresponses/{route_response_id}",
            api_id = input.api_id,
            route_id = input.route_id,
            route_response_id = input.route_response_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRouteResponseResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetRouteResponseError::from_response(response))
        }
    }

    /// <p>Gets the RouteResponses for a Route.</p>
    async fn get_route_responses(
        &self,
        input: GetRouteResponsesRequest,
    ) -> Result<GetRouteResponsesResponse, RusotoError<GetRouteResponsesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRouteResponsesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetRouteResponsesError::from_response(response))
        }
    }

    /// <p>Gets the Routes for an API.</p>
    async fn get_routes(
        &self,
        input: GetRoutesRequest,
    ) -> Result<GetRoutesResponse, RusotoError<GetRoutesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRoutesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetRoutesError::from_response(response))
        }
    }

    /// <p>Gets a Stage.</p>
    async fn get_stage(
        &self,
        input: GetStageRequest,
    ) -> Result<GetStageResponse, RusotoError<GetStageError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/stages/{stage_name}",
            api_id = input.api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetStageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetStageError::from_response(response))
        }
    }

    /// <p>Gets the Stages for an API.</p>
    async fn get_stages(
        &self,
        input: GetStagesRequest,
    ) -> Result<GetStagesResponse, RusotoError<GetStagesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetStagesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetStagesError::from_response(response))
        }
    }

    /// <p>Gets a collection of Tag resources.</p>
    async fn get_tags(
        &self,
        input: GetTagsRequest,
    ) -> Result<GetTagsResponse, RusotoError<GetTagsError>> {
        let request_uri = format!("/v2/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetTagsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetTagsError::from_response(response))
        }
    }

    /// <p>Imports an API.</p>
    async fn import_api(
        &self,
        input: ImportApiRequest,
    ) -> Result<ImportApiResponse, RusotoError<ImportApiError>> {
        let request_uri = "/v2/apis";

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.basepath {
            params.put("basepath", x);
        }
        if let Some(ref x) = input.fail_on_warnings {
            params.put("failOnWarnings", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ImportApiResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ImportApiError::from_response(response))
        }
    }

    /// <p>Puts an Api resource.</p>
    async fn reimport_api(
        &self,
        input: ReimportApiRequest,
    ) -> Result<ReimportApiResponse, RusotoError<ReimportApiError>> {
        let request_uri = format!("/v2/apis/{api_id}", api_id = input.api_id);

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.basepath {
            params.put("basepath", x);
        }
        if let Some(ref x) = input.fail_on_warnings {
            params.put("failOnWarnings", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ReimportApiResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ReimportApiError::from_response(response))
        }
    }

    /// <p>Creates a new Tag resource to represent a tag.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!("/v2/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Deletes a Tag.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/v2/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates an Api resource.</p>
    async fn update_api(
        &self,
        input: UpdateApiRequest,
    ) -> Result<UpdateApiResponse, RusotoError<UpdateApiError>> {
        let request_uri = format!("/v2/apis/{api_id}", api_id = input.api_id);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateApiResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApiError::from_response(response))
        }
    }

    /// <p>The API mapping.</p>
    async fn update_api_mapping(
        &self,
        input: UpdateApiMappingRequest,
    ) -> Result<UpdateApiMappingResponse, RusotoError<UpdateApiMappingError>> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}/apimappings/{api_mapping_id}",
            api_mapping_id = input.api_mapping_id,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateApiMappingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApiMappingError::from_response(response))
        }
    }

    /// <p>Updates an Authorizer.</p>
    async fn update_authorizer(
        &self,
        input: UpdateAuthorizerRequest,
    ) -> Result<UpdateAuthorizerResponse, RusotoError<UpdateAuthorizerError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/authorizers/{authorizer_id}",
            api_id = input.api_id,
            authorizer_id = input.authorizer_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateAuthorizerResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAuthorizerError::from_response(response))
        }
    }

    /// <p>Updates a Deployment.</p>
    async fn update_deployment(
        &self,
        input: UpdateDeploymentRequest,
    ) -> Result<UpdateDeploymentResponse, RusotoError<UpdateDeploymentError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/deployments/{deployment_id}",
            api_id = input.api_id,
            deployment_id = input.deployment_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDeploymentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDeploymentError::from_response(response))
        }
    }

    /// <p>Updates a domain name.</p>
    async fn update_domain_name(
        &self,
        input: UpdateDomainNameRequest,
    ) -> Result<UpdateDomainNameResponse, RusotoError<UpdateDomainNameError>> {
        let request_uri = format!(
            "/v2/domainnames/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDomainNameResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDomainNameError::from_response(response))
        }
    }

    /// <p>Updates an Integration.</p>
    async fn update_integration(
        &self,
        input: UpdateIntegrationRequest,
    ) -> Result<UpdateIntegrationResult, RusotoError<UpdateIntegrationError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/integrations/{integration_id}",
            api_id = input.api_id,
            integration_id = input.integration_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateIntegrationResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateIntegrationError::from_response(response))
        }
    }

    /// <p>Updates an IntegrationResponses.</p>
    async fn update_integration_response(
        &self,
        input: UpdateIntegrationResponseRequest,
    ) -> Result<UpdateIntegrationResponseResponse, RusotoError<UpdateIntegrationResponseError>>
    {
        let request_uri = format!("/v2/apis/{api_id}/integrations/{integration_id}/integrationresponses/{integration_response_id}", api_id = input.api_id, integration_id = input.integration_id, integration_response_id = input.integration_response_id);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateIntegrationResponseResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateIntegrationResponseError::from_response(response))
        }
    }

    /// <p>Updates a Model.</p>
    async fn update_model(
        &self,
        input: UpdateModelRequest,
    ) -> Result<UpdateModelResponse, RusotoError<UpdateModelError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/models/{model_id}",
            api_id = input.api_id,
            model_id = input.model_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateModelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateModelError::from_response(response))
        }
    }

    /// <p>Updates a Route.</p>
    async fn update_route(
        &self,
        input: UpdateRouteRequest,
    ) -> Result<UpdateRouteResult, RusotoError<UpdateRouteError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/routes/{route_id}",
            api_id = input.api_id,
            route_id = input.route_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateRouteResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRouteError::from_response(response))
        }
    }

    /// <p>Updates a RouteResponse.</p>
    async fn update_route_response(
        &self,
        input: UpdateRouteResponseRequest,
    ) -> Result<UpdateRouteResponseResponse, RusotoError<UpdateRouteResponseError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateRouteResponseResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRouteResponseError::from_response(response))
        }
    }

    /// <p>Updates a Stage.</p>
    async fn update_stage(
        &self,
        input: UpdateStageRequest,
    ) -> Result<UpdateStageResponse, RusotoError<UpdateStageError>> {
        let request_uri = format!(
            "/v2/apis/{api_id}/stages/{stage_name}",
            api_id = input.api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateStageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateStageError::from_response(response))
        }
    }
}
