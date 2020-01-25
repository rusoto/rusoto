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
use rusoto_core::proto::xml::error::*;
use rusoto_core::proto::xml::util::{
    characters, deserialize_elements, end_element, find_start_element, peek_at_name, skip_tree,
    start_element,
};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::signature::SignedRequest;
use serde_urlencoded;
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::EventReader;

/// <p>Information about an action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Action {
    /// <p>[HTTPS listeners] Information for using Amazon Cognito to authenticate users. Specify only when <code>Type</code> is <code>authenticate-cognito</code>.</p>
    pub authenticate_cognito_config: Option<AuthenticateCognitoActionConfig>,
    /// <p>[HTTPS listeners] Information about an identity provider that is compliant with OpenID Connect (OIDC). Specify only when <code>Type</code> is <code>authenticate-oidc</code>.</p>
    pub authenticate_oidc_config: Option<AuthenticateOidcActionConfig>,
    /// <p>[Application Load Balancer] Information for creating an action that returns a custom HTTP response. Specify only when <code>Type</code> is <code>fixed-response</code>.</p>
    pub fixed_response_config: Option<FixedResponseActionConfig>,
    /// <p>Information for creating an action that distributes requests among one or more target groups. For Network Load Balancers, you can specify a single target group. Specify only when <code>Type</code> is <code>forward</code>. If you specify both <code>ForwardConfig</code> and <code>TargetGroupArn</code>, you can specify only one target group using <code>ForwardConfig</code> and it must be the same target group specified in <code>TargetGroupArn</code>.</p>
    pub forward_config: Option<ForwardActionConfig>,
    /// <p>The order for the action. This value is required for rules with multiple actions. The action with the lowest value for order is performed first. The last action to be performed must be one of the following types of actions: a <code>forward</code>, <code>fixed-response</code>, or <code>redirect</code>.</p>
    pub order: Option<i64>,
    /// <p>[Application Load Balancer] Information for creating a redirect action. Specify only when <code>Type</code> is <code>redirect</code>.</p>
    pub redirect_config: Option<RedirectActionConfig>,
    /// <p>The Amazon Resource Name (ARN) of the target group. Specify only when <code>Type</code> is <code>forward</code> and you want to route to a single target group. To route to one or more target groups, use <code>ForwardConfig</code> instead.</p>
    pub target_group_arn: Option<String>,
    /// <p>The type of action.</p>
    pub type_: String,
}

struct ActionDeserializer;
impl ActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Action, XmlParseError> {
        deserialize_elements::<_, Action, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AuthenticateCognitoConfig" => {
                    obj.authenticate_cognito_config =
                        Some(AuthenticateCognitoActionConfigDeserializer::deserialize(
                            "AuthenticateCognitoConfig",
                            stack,
                        )?);
                }
                "AuthenticateOidcConfig" => {
                    obj.authenticate_oidc_config =
                        Some(AuthenticateOidcActionConfigDeserializer::deserialize(
                            "AuthenticateOidcConfig",
                            stack,
                        )?);
                }
                "FixedResponseConfig" => {
                    obj.fixed_response_config =
                        Some(FixedResponseActionConfigDeserializer::deserialize(
                            "FixedResponseConfig",
                            stack,
                        )?);
                }
                "ForwardConfig" => {
                    obj.forward_config = Some(ForwardActionConfigDeserializer::deserialize(
                        "ForwardConfig",
                        stack,
                    )?);
                }
                "Order" => {
                    obj.order = Some(ActionOrderDeserializer::deserialize("Order", stack)?);
                }
                "RedirectConfig" => {
                    obj.redirect_config = Some(RedirectActionConfigDeserializer::deserialize(
                        "RedirectConfig",
                        stack,
                    )?);
                }
                "TargetGroupArn" => {
                    obj.target_group_arn = Some(TargetGroupArnDeserializer::deserialize(
                        "TargetGroupArn",
                        stack,
                    )?);
                }
                "Type" => {
                    obj.type_ = ActionTypeEnumDeserializer::deserialize("Type", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `Action` contents to a `SignedRequest`.
struct ActionSerializer;
impl ActionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Action) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.authenticate_cognito_config {
            AuthenticateCognitoActionConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AuthenticateCognitoConfig"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.authenticate_oidc_config {
            AuthenticateOidcActionConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AuthenticateOidcConfig"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.fixed_response_config {
            FixedResponseActionConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "FixedResponseConfig"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.forward_config {
            ForwardActionConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ForwardConfig"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.order {
            params.put(&format!("{}{}", prefix, "Order"), &field_value);
        }
        if let Some(ref field_value) = obj.redirect_config {
            RedirectActionConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RedirectConfig"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.target_group_arn {
            params.put(&format!("{}{}", prefix, "TargetGroupArn"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Type"), &obj.type_);
    }
}

struct ActionOrderDeserializer;
impl ActionOrderDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ActionTypeEnumDeserializer;
impl ActionTypeEnumDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ActionsDeserializer;
impl ActionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Action>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ActionDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `Actions` contents to a `SignedRequest`.
struct ActionsSerializer;
impl ActionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Action>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ActionSerializer::serialize(params, &key, obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddListenerCertificatesInput {
    /// <p>The certificate to add. You can specify one certificate per call. Set <code>CertificateArn</code> to the certificate ARN but do not set <code>IsDefault</code>.</p>
    pub certificates: Vec<Certificate>,
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: String,
}

/// Serialize `AddListenerCertificatesInput` contents to a `SignedRequest`.
struct AddListenerCertificatesInputSerializer;
impl AddListenerCertificatesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddListenerCertificatesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        CertificateListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Certificates"),
            &obj.certificates,
        );
        params.put(&format!("{}{}", prefix, "ListenerArn"), &obj.listener_arn);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AddListenerCertificatesOutput {
    /// <p>Information about the certificates in the certificate list.</p>
    pub certificates: Option<Vec<Certificate>>,
}

struct AddListenerCertificatesOutputDeserializer;
impl AddListenerCertificatesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AddListenerCertificatesOutput, XmlParseError> {
        deserialize_elements::<_, AddListenerCertificatesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Certificates" => {
                        obj.certificates.get_or_insert(vec![]).extend(
                            CertificateListDeserializer::deserialize("Certificates", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsInput {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub resource_arns: Vec<String>,
    /// <p>The tags. Each resource can have a maximum of 10 tags.</p>
    pub tags: Vec<Tag>,
}

/// Serialize `AddTagsInput` contents to a `SignedRequest`.
struct AddTagsInputSerializer;
impl AddTagsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddTagsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ResourceArnsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ResourceArns"),
            &obj.resource_arns,
        );
        TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), &obj.tags);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AddTagsOutput {}

struct AddTagsOutputDeserializer;
impl AddTagsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AddTagsOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = AddTagsOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AllocationIdDeserializer;
impl AllocationIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateCognitoActionAuthenticationRequestExtraParamsDeserializer;
impl AuthenticateCognitoActionAuthenticationRequestExtraParamsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        start_element(tag_name, stack)?;

        let mut obj = ::std::collections::HashMap::new();

        while peek_at_name(stack)? == "entry" {
            start_element("entry", stack)?;
            let key =
                AuthenticateCognitoActionAuthenticationRequestParamNameDeserializer::deserialize(
                    "key", stack,
                )?;
            let value =
                AuthenticateCognitoActionAuthenticationRequestParamValueDeserializer::deserialize(
                    "value", stack,
                )?;
            obj.insert(key, value);
            end_element("entry", stack)?;
        }

        end_element(tag_name, stack)?;
        Ok(obj)
    }
}

/// Serialize `AuthenticateCognitoActionAuthenticationRequestExtraParams` contents to a `SignedRequest`.
struct AuthenticateCognitoActionAuthenticationRequestExtraParamsSerializer;
impl AuthenticateCognitoActionAuthenticationRequestExtraParamsSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &::std::collections::HashMap<String, String>,
    ) {
        for (index, (key, value)) in obj.iter().enumerate() {
            let prefix = format!("{}.{}", name, index + 1);
            params.put(&format!("{}.{}", prefix, "key"), &key);
            params.put(&format!("{}.{}", prefix, "Value"), &value);
        }
    }
}

struct AuthenticateCognitoActionAuthenticationRequestParamNameDeserializer;
impl AuthenticateCognitoActionAuthenticationRequestParamNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateCognitoActionAuthenticationRequestParamValueDeserializer;
impl AuthenticateCognitoActionAuthenticationRequestParamValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateCognitoActionConditionalBehaviorEnumDeserializer;
impl AuthenticateCognitoActionConditionalBehaviorEnumDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Request parameters to use when integrating with Amazon Cognito to authenticate users.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AuthenticateCognitoActionConfig {
    /// <p>The query parameters (up to 10) to include in the redirect request to the authorization endpoint.</p>
    pub authentication_request_extra_params: Option<::std::collections::HashMap<String, String>>,
    /// <p><p>The behavior if the user is not authenticated. The following are possible values:</p> <ul> <li> <p>deny<code/> - Return an HTTP 401 Unauthorized error.</p> </li> <li> <p>allow<code/> - Allow the request to be forwarded to the target.</p> </li> <li> <p>authenticate<code/> - Redirect the request to the IdP authorization endpoint. This is the default value.</p> </li> </ul></p>
    pub on_unauthenticated_request: Option<String>,
    /// <p>The set of user claims to be requested from the IdP. The default is <code>openid</code>.</p> <p>To verify which scope values your IdP supports and how to separate multiple values, see the documentation for your IdP.</p>
    pub scope: Option<String>,
    /// <p>The name of the cookie used to maintain session information. The default is AWSELBAuthSessionCookie.</p>
    pub session_cookie_name: Option<String>,
    /// <p>The maximum duration of the authentication session, in seconds. The default is 604800 seconds (7 days).</p>
    pub session_timeout: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Cognito user pool.</p>
    pub user_pool_arn: String,
    /// <p>The ID of the Amazon Cognito user pool client.</p>
    pub user_pool_client_id: String,
    /// <p>The domain prefix or fully-qualified domain name of the Amazon Cognito user pool.</p>
    pub user_pool_domain: String,
}

struct AuthenticateCognitoActionConfigDeserializer;
impl AuthenticateCognitoActionConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AuthenticateCognitoActionConfig, XmlParseError> {
        deserialize_elements::<_, AuthenticateCognitoActionConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AuthenticationRequestExtraParams" => {
                        obj.authentication_request_extra_params = Some(AuthenticateCognitoActionAuthenticationRequestExtraParamsDeserializer::deserialize("AuthenticationRequestExtraParams", stack)?);
                    }
                    "OnUnauthenticatedRequest" => {
                        obj.on_unauthenticated_request = Some(AuthenticateCognitoActionConditionalBehaviorEnumDeserializer::deserialize("OnUnauthenticatedRequest", stack)?);
                    }
                    "Scope" => {
                        obj.scope = Some(AuthenticateCognitoActionScopeDeserializer::deserialize(
                            "Scope", stack,
                        )?);
                    }
                    "SessionCookieName" => {
                        obj.session_cookie_name = Some(
                            AuthenticateCognitoActionSessionCookieNameDeserializer::deserialize(
                                "SessionCookieName",
                                stack,
                            )?,
                        );
                    }
                    "SessionTimeout" => {
                        obj.session_timeout = Some(
                            AuthenticateCognitoActionSessionTimeoutDeserializer::deserialize(
                                "SessionTimeout",
                                stack,
                            )?,
                        );
                    }
                    "UserPoolArn" => {
                        obj.user_pool_arn =
                            AuthenticateCognitoActionUserPoolArnDeserializer::deserialize(
                                "UserPoolArn",
                                stack,
                            )?;
                    }
                    "UserPoolClientId" => {
                        obj.user_pool_client_id =
                            AuthenticateCognitoActionUserPoolClientIdDeserializer::deserialize(
                                "UserPoolClientId",
                                stack,
                            )?;
                    }
                    "UserPoolDomain" => {
                        obj.user_pool_domain =
                            AuthenticateCognitoActionUserPoolDomainDeserializer::deserialize(
                                "UserPoolDomain",
                                stack,
                            )?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

/// Serialize `AuthenticateCognitoActionConfig` contents to a `SignedRequest`.
struct AuthenticateCognitoActionConfigSerializer;
impl AuthenticateCognitoActionConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AuthenticateCognitoActionConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.authentication_request_extra_params {
            AuthenticateCognitoActionAuthenticationRequestExtraParamsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AuthenticationRequestExtraParams"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.on_unauthenticated_request {
            params.put(
                &format!("{}{}", prefix, "OnUnauthenticatedRequest"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.scope {
            params.put(&format!("{}{}", prefix, "Scope"), &field_value);
        }
        if let Some(ref field_value) = obj.session_cookie_name {
            params.put(&format!("{}{}", prefix, "SessionCookieName"), &field_value);
        }
        if let Some(ref field_value) = obj.session_timeout {
            params.put(&format!("{}{}", prefix, "SessionTimeout"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "UserPoolArn"), &obj.user_pool_arn);
        params.put(
            &format!("{}{}", prefix, "UserPoolClientId"),
            &obj.user_pool_client_id,
        );
        params.put(
            &format!("{}{}", prefix, "UserPoolDomain"),
            &obj.user_pool_domain,
        );
    }
}

struct AuthenticateCognitoActionScopeDeserializer;
impl AuthenticateCognitoActionScopeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateCognitoActionSessionCookieNameDeserializer;
impl AuthenticateCognitoActionSessionCookieNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateCognitoActionSessionTimeoutDeserializer;
impl AuthenticateCognitoActionSessionTimeoutDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateCognitoActionUserPoolArnDeserializer;
impl AuthenticateCognitoActionUserPoolArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateCognitoActionUserPoolClientIdDeserializer;
impl AuthenticateCognitoActionUserPoolClientIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateCognitoActionUserPoolDomainDeserializer;
impl AuthenticateCognitoActionUserPoolDomainDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateOidcActionAuthenticationRequestExtraParamsDeserializer;
impl AuthenticateOidcActionAuthenticationRequestExtraParamsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        start_element(tag_name, stack)?;

        let mut obj = ::std::collections::HashMap::new();

        while peek_at_name(stack)? == "entry" {
            start_element("entry", stack)?;
            let key =
                AuthenticateOidcActionAuthenticationRequestParamNameDeserializer::deserialize(
                    "key", stack,
                )?;
            let value =
                AuthenticateOidcActionAuthenticationRequestParamValueDeserializer::deserialize(
                    "value", stack,
                )?;
            obj.insert(key, value);
            end_element("entry", stack)?;
        }

        end_element(tag_name, stack)?;
        Ok(obj)
    }
}

/// Serialize `AuthenticateOidcActionAuthenticationRequestExtraParams` contents to a `SignedRequest`.
struct AuthenticateOidcActionAuthenticationRequestExtraParamsSerializer;
impl AuthenticateOidcActionAuthenticationRequestExtraParamsSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &::std::collections::HashMap<String, String>,
    ) {
        for (index, (key, value)) in obj.iter().enumerate() {
            let prefix = format!("{}.{}", name, index + 1);
            params.put(&format!("{}.{}", prefix, "key"), &key);
            params.put(&format!("{}.{}", prefix, "Value"), &value);
        }
    }
}

struct AuthenticateOidcActionAuthenticationRequestParamNameDeserializer;
impl AuthenticateOidcActionAuthenticationRequestParamNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateOidcActionAuthenticationRequestParamValueDeserializer;
impl AuthenticateOidcActionAuthenticationRequestParamValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateOidcActionAuthorizationEndpointDeserializer;
impl AuthenticateOidcActionAuthorizationEndpointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateOidcActionClientIdDeserializer;
impl AuthenticateOidcActionClientIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateOidcActionClientSecretDeserializer;
impl AuthenticateOidcActionClientSecretDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateOidcActionConditionalBehaviorEnumDeserializer;
impl AuthenticateOidcActionConditionalBehaviorEnumDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Request parameters when using an identity provider (IdP) that is compliant with OpenID Connect (OIDC) to authenticate users.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AuthenticateOidcActionConfig {
    /// <p>The query parameters (up to 10) to include in the redirect request to the authorization endpoint.</p>
    pub authentication_request_extra_params: Option<::std::collections::HashMap<String, String>>,
    /// <p>The authorization endpoint of the IdP. This must be a full URL, including the HTTPS protocol, the domain, and the path.</p>
    pub authorization_endpoint: String,
    /// <p>The OAuth 2.0 client identifier.</p>
    pub client_id: String,
    /// <p>The OAuth 2.0 client secret. This parameter is required if you are creating a rule. If you are modifying a rule, you can omit this parameter if you set <code>UseExistingClientSecret</code> to true.</p>
    pub client_secret: Option<String>,
    /// <p>The OIDC issuer identifier of the IdP. This must be a full URL, including the HTTPS protocol, the domain, and the path.</p>
    pub issuer: String,
    /// <p><p>The behavior if the user is not authenticated. The following are possible values:</p> <ul> <li> <p>deny<code/> - Return an HTTP 401 Unauthorized error.</p> </li> <li> <p>allow<code/> - Allow the request to be forwarded to the target.</p> </li> <li> <p>authenticate<code/> - Redirect the request to the IdP authorization endpoint. This is the default value.</p> </li> </ul></p>
    pub on_unauthenticated_request: Option<String>,
    /// <p>The set of user claims to be requested from the IdP. The default is <code>openid</code>.</p> <p>To verify which scope values your IdP supports and how to separate multiple values, see the documentation for your IdP.</p>
    pub scope: Option<String>,
    /// <p>The name of the cookie used to maintain session information. The default is AWSELBAuthSessionCookie.</p>
    pub session_cookie_name: Option<String>,
    /// <p>The maximum duration of the authentication session, in seconds. The default is 604800 seconds (7 days).</p>
    pub session_timeout: Option<i64>,
    /// <p>The token endpoint of the IdP. This must be a full URL, including the HTTPS protocol, the domain, and the path.</p>
    pub token_endpoint: String,
    /// <p>Indicates whether to use the existing client secret when modifying a rule. If you are creating a rule, you can omit this parameter or set it to false.</p>
    pub use_existing_client_secret: Option<bool>,
    /// <p>The user info endpoint of the IdP. This must be a full URL, including the HTTPS protocol, the domain, and the path.</p>
    pub user_info_endpoint: String,
}

struct AuthenticateOidcActionConfigDeserializer;
impl AuthenticateOidcActionConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AuthenticateOidcActionConfig, XmlParseError> {
        deserialize_elements::<_, AuthenticateOidcActionConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AuthenticationRequestExtraParams" => {
                        obj.authentication_request_extra_params = Some(AuthenticateOidcActionAuthenticationRequestExtraParamsDeserializer::deserialize("AuthenticationRequestExtraParams", stack)?);
                    }
                    "AuthorizationEndpoint" => {
                        obj.authorization_endpoint =
                            AuthenticateOidcActionAuthorizationEndpointDeserializer::deserialize(
                                "AuthorizationEndpoint",
                                stack,
                            )?;
                    }
                    "ClientId" => {
                        obj.client_id = AuthenticateOidcActionClientIdDeserializer::deserialize(
                            "ClientId", stack,
                        )?;
                    }
                    "ClientSecret" => {
                        obj.client_secret =
                            Some(AuthenticateOidcActionClientSecretDeserializer::deserialize(
                                "ClientSecret",
                                stack,
                            )?);
                    }
                    "Issuer" => {
                        obj.issuer =
                            AuthenticateOidcActionIssuerDeserializer::deserialize("Issuer", stack)?;
                    }
                    "OnUnauthenticatedRequest" => {
                        obj.on_unauthenticated_request = Some(
                            AuthenticateOidcActionConditionalBehaviorEnumDeserializer::deserialize(
                                "OnUnauthenticatedRequest",
                                stack,
                            )?,
                        );
                    }
                    "Scope" => {
                        obj.scope = Some(AuthenticateOidcActionScopeDeserializer::deserialize(
                            "Scope", stack,
                        )?);
                    }
                    "SessionCookieName" => {
                        obj.session_cookie_name = Some(
                            AuthenticateOidcActionSessionCookieNameDeserializer::deserialize(
                                "SessionCookieName",
                                stack,
                            )?,
                        );
                    }
                    "SessionTimeout" => {
                        obj.session_timeout = Some(
                            AuthenticateOidcActionSessionTimeoutDeserializer::deserialize(
                                "SessionTimeout",
                                stack,
                            )?,
                        );
                    }
                    "TokenEndpoint" => {
                        obj.token_endpoint =
                            AuthenticateOidcActionTokenEndpointDeserializer::deserialize(
                                "TokenEndpoint",
                                stack,
                            )?;
                    }
                    "UseExistingClientSecret" => {
                        obj.use_existing_client_secret = Some(
                            AuthenticateOidcActionUseExistingClientSecretDeserializer::deserialize(
                                "UseExistingClientSecret",
                                stack,
                            )?,
                        );
                    }
                    "UserInfoEndpoint" => {
                        obj.user_info_endpoint =
                            AuthenticateOidcActionUserInfoEndpointDeserializer::deserialize(
                                "UserInfoEndpoint",
                                stack,
                            )?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

/// Serialize `AuthenticateOidcActionConfig` contents to a `SignedRequest`.
struct AuthenticateOidcActionConfigSerializer;
impl AuthenticateOidcActionConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AuthenticateOidcActionConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.authentication_request_extra_params {
            AuthenticateOidcActionAuthenticationRequestExtraParamsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AuthenticationRequestExtraParams"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "AuthorizationEndpoint"),
            &obj.authorization_endpoint,
        );
        params.put(&format!("{}{}", prefix, "ClientId"), &obj.client_id);
        if let Some(ref field_value) = obj.client_secret {
            params.put(&format!("{}{}", prefix, "ClientSecret"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Issuer"), &obj.issuer);
        if let Some(ref field_value) = obj.on_unauthenticated_request {
            params.put(
                &format!("{}{}", prefix, "OnUnauthenticatedRequest"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.scope {
            params.put(&format!("{}{}", prefix, "Scope"), &field_value);
        }
        if let Some(ref field_value) = obj.session_cookie_name {
            params.put(&format!("{}{}", prefix, "SessionCookieName"), &field_value);
        }
        if let Some(ref field_value) = obj.session_timeout {
            params.put(&format!("{}{}", prefix, "SessionTimeout"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "TokenEndpoint"),
            &obj.token_endpoint,
        );
        if let Some(ref field_value) = obj.use_existing_client_secret {
            params.put(
                &format!("{}{}", prefix, "UseExistingClientSecret"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "UserInfoEndpoint"),
            &obj.user_info_endpoint,
        );
    }
}

struct AuthenticateOidcActionIssuerDeserializer;
impl AuthenticateOidcActionIssuerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateOidcActionScopeDeserializer;
impl AuthenticateOidcActionScopeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateOidcActionSessionCookieNameDeserializer;
impl AuthenticateOidcActionSessionCookieNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateOidcActionSessionTimeoutDeserializer;
impl AuthenticateOidcActionSessionTimeoutDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateOidcActionTokenEndpointDeserializer;
impl AuthenticateOidcActionTokenEndpointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateOidcActionUseExistingClientSecretDeserializer;
impl AuthenticateOidcActionUseExistingClientSecretDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AuthenticateOidcActionUserInfoEndpointDeserializer;
impl AuthenticateOidcActionUserInfoEndpointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about an Availability Zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AvailabilityZone {
    /// <p>[Network Load Balancers] If you need static IP addresses for your load balancer, you can specify one Elastic IP address per Availability Zone when you create an internal-facing load balancer. For internal load balancers, you can specify a private IP address from the IPv4 range of the subnet.</p>
    pub load_balancer_addresses: Option<Vec<LoadBalancerAddress>>,
    /// <p>The ID of the subnet. You can specify one subnet per Availability Zone.</p>
    pub subnet_id: Option<String>,
    /// <p>The name of the Availability Zone.</p>
    pub zone_name: Option<String>,
}

struct AvailabilityZoneDeserializer;
impl AvailabilityZoneDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AvailabilityZone, XmlParseError> {
        deserialize_elements::<_, AvailabilityZone, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "LoadBalancerAddresses" => {
                    obj.load_balancer_addresses.get_or_insert(vec![]).extend(
                        LoadBalancerAddressesDeserializer::deserialize(
                            "LoadBalancerAddresses",
                            stack,
                        )?,
                    );
                }
                "SubnetId" => {
                    obj.subnet_id = Some(SubnetIdDeserializer::deserialize("SubnetId", stack)?);
                }
                "ZoneName" => {
                    obj.zone_name = Some(ZoneNameDeserializer::deserialize("ZoneName", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct AvailabilityZonesDeserializer;
impl AvailabilityZonesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AvailabilityZone>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(AvailabilityZoneDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct CanonicalHostedZoneIdDeserializer;
impl CanonicalHostedZoneIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about an SSL server certificate.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Certificate {
    /// <p>The Amazon Resource Name (ARN) of the certificate.</p>
    pub certificate_arn: Option<String>,
    /// <p>Indicates whether the certificate is the default certificate. Do not set this value when specifying a certificate as an input. This value is not included in the output when describing a listener, but is included when describing listener certificates.</p>
    pub is_default: Option<bool>,
}

struct CertificateDeserializer;
impl CertificateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Certificate, XmlParseError> {
        deserialize_elements::<_, Certificate, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CertificateArn" => {
                    obj.certificate_arn = Some(CertificateArnDeserializer::deserialize(
                        "CertificateArn",
                        stack,
                    )?);
                }
                "IsDefault" => {
                    obj.is_default = Some(DefaultDeserializer::deserialize("IsDefault", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `Certificate` contents to a `SignedRequest`.
struct CertificateSerializer;
impl CertificateSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Certificate) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.certificate_arn {
            params.put(&format!("{}{}", prefix, "CertificateArn"), &field_value);
        }
        if let Some(ref field_value) = obj.is_default {
            params.put(&format!("{}{}", prefix, "IsDefault"), &field_value);
        }
    }
}

struct CertificateArnDeserializer;
impl CertificateArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct CertificateListDeserializer;
impl CertificateListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Certificate>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(CertificateDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `CertificateList` contents to a `SignedRequest`.
struct CertificateListSerializer;
impl CertificateListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Certificate>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            CertificateSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>Information about a cipher used in a policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Cipher {
    /// <p>The name of the cipher.</p>
    pub name: Option<String>,
    /// <p>The priority of the cipher.</p>
    pub priority: Option<i64>,
}

struct CipherDeserializer;
impl CipherDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Cipher, XmlParseError> {
        deserialize_elements::<_, Cipher, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Name" => {
                    obj.name = Some(CipherNameDeserializer::deserialize("Name", stack)?);
                }
                "Priority" => {
                    obj.priority =
                        Some(CipherPriorityDeserializer::deserialize("Priority", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct CipherNameDeserializer;
impl CipherNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct CipherPriorityDeserializer;
impl CipherPriorityDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct CiphersDeserializer;
impl CiphersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Cipher>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(CipherDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ConditionFieldNameDeserializer;
impl ConditionFieldNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateListenerInput {
    /// <p>[HTTPS and TLS listeners] The default certificate for the listener. You must provide exactly one certificate. Set <code>CertificateArn</code> to the certificate ARN but do not set <code>IsDefault</code>.</p> <p>To create a certificate list for the listener, use <a>AddListenerCertificates</a>.</p>
    pub certificates: Option<Vec<Certificate>>,
    /// <p>The actions for the default rule. The rule must include one forward action or one or more fixed-response actions.</p> <p>If the action type is <code>forward</code>, you specify one or more target groups. The protocol of the target group must be HTTP or HTTPS for an Application Load Balancer. The protocol of the target group must be TCP, TLS, UDP, or TCP_UDP for a Network Load Balancer.</p> <p>[HTTPS listeners] If the action type is <code>authenticate-oidc</code>, you authenticate users through an identity provider that is OpenID Connect (OIDC) compliant.</p> <p>[HTTPS listeners] If the action type is <code>authenticate-cognito</code>, you authenticate users through the user pools supported by Amazon Cognito.</p> <p>[Application Load Balancer] If the action type is <code>redirect</code>, you redirect specified client requests from one URL to another.</p> <p>[Application Load Balancer] If the action type is <code>fixed-response</code>, you drop specified client requests and return a custom HTTP response.</p>
    pub default_actions: Vec<Action>,
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
    /// <p>The port on which the load balancer is listening.</p>
    pub port: i64,
    /// <p>The protocol for connections from clients to the load balancer. For Application Load Balancers, the supported protocols are HTTP and HTTPS. For Network Load Balancers, the supported protocols are TCP, TLS, UDP, and TCP_UDP.</p>
    pub protocol: String,
    /// <p>[HTTPS and TLS listeners] The security policy that defines which ciphers and protocols are supported. The default is the current predefined security policy.</p>
    pub ssl_policy: Option<String>,
}

/// Serialize `CreateListenerInput` contents to a `SignedRequest`.
struct CreateListenerInputSerializer;
impl CreateListenerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateListenerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.certificates {
            CertificateListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Certificates"),
                field_value,
            );
        }
        ActionsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "DefaultActions"),
            &obj.default_actions,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn,
        );
        params.put(&format!("{}{}", prefix, "Port"), &obj.port);
        params.put(&format!("{}{}", prefix, "Protocol"), &obj.protocol);
        if let Some(ref field_value) = obj.ssl_policy {
            params.put(&format!("{}{}", prefix, "SslPolicy"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateListenerOutput {
    /// <p>Information about the listener.</p>
    pub listeners: Option<Vec<Listener>>,
}

struct CreateListenerOutputDeserializer;
impl CreateListenerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateListenerOutput, XmlParseError> {
        deserialize_elements::<_, CreateListenerOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Listeners" => {
                    obj.listeners
                        .get_or_insert(vec![])
                        .extend(ListenersDeserializer::deserialize("Listeners", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLoadBalancerInput {
    /// <p>[Application Load Balancers] The type of IP addresses used by the subnets for your load balancer. The possible values are <code>ipv4</code> (for IPv4 addresses) and <code>dualstack</code> (for IPv4 and IPv6 addresses). Internal load balancers must use <code>ipv4</code>.</p>
    pub ip_address_type: Option<String>,
    /// <p>The name of the load balancer.</p> <p>This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, must not begin or end with a hyphen, and must not begin with "internal-".</p>
    pub name: String,
    /// <p>The nodes of an Internet-facing load balancer have public IP addresses. The DNS name of an Internet-facing load balancer is publicly resolvable to the public IP addresses of the nodes. Therefore, Internet-facing load balancers can route requests from clients over the internet.</p> <p>The nodes of an internal load balancer have only private IP addresses. The DNS name of an internal load balancer is publicly resolvable to the private IP addresses of the nodes. Therefore, internal load balancers can route requests only from clients with access to the VPC for the load balancer.</p> <p>The default is an Internet-facing load balancer.</p>
    pub scheme: Option<String>,
    /// <p>[Application Load Balancers] The IDs of the security groups for the load balancer.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p>The IDs of the public subnets. You can specify only one subnet per Availability Zone. You must specify either subnets or subnet mappings.</p> <p>[Application Load Balancers] You must specify subnets from at least two Availability Zones. You cannot specify Elastic IP addresses for your subnets.</p> <p>[Network Load Balancers] You can specify subnets from one or more Availability Zones. You can specify one Elastic IP address per subnet if you need static IP addresses for your internet-facing load balancer. For internal load balancers, you can specify one private IP address per subnet from the IPv4 range of the subnet.</p>
    pub subnet_mappings: Option<Vec<SubnetMapping>>,
    /// <p>The IDs of the public subnets. You can specify only one subnet per Availability Zone. You must specify either subnets or subnet mappings.</p> <p>[Application Load Balancers] You must specify subnets from at least two Availability Zones.</p> <p>[Network Load Balancers] You can specify subnets from one or more Availability Zones.</p>
    pub subnets: Option<Vec<String>>,
    /// <p>One or more tags to assign to the load balancer.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The type of load balancer. The default is <code>application</code>.</p>
    pub type_: Option<String>,
}

/// Serialize `CreateLoadBalancerInput` contents to a `SignedRequest`.
struct CreateLoadBalancerInputSerializer;
impl CreateLoadBalancerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateLoadBalancerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.ip_address_type {
            params.put(&format!("{}{}", prefix, "IpAddressType"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        if let Some(ref field_value) = obj.scheme {
            params.put(&format!("{}{}", prefix, "Scheme"), &field_value);
        }
        if let Some(ref field_value) = obj.security_groups {
            SecurityGroupsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SecurityGroups"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.subnet_mappings {
            SubnetMappingsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SubnetMappings"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.subnets {
            SubnetsSerializer::serialize(params, &format!("{}{}", prefix, "Subnets"), field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        if let Some(ref field_value) = obj.type_ {
            params.put(&format!("{}{}", prefix, "Type"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateLoadBalancerOutput {
    /// <p>Information about the load balancer.</p>
    pub load_balancers: Option<Vec<LoadBalancer>>,
}

struct CreateLoadBalancerOutputDeserializer;
impl CreateLoadBalancerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateLoadBalancerOutput, XmlParseError> {
        deserialize_elements::<_, CreateLoadBalancerOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LoadBalancers" => {
                        obj.load_balancers.get_or_insert(vec![]).extend(
                            LoadBalancersDeserializer::deserialize("LoadBalancers", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRuleInput {
    /// <p>The actions. Each rule must include exactly one of the following types of actions: <code>forward</code>, <code>fixed-response</code>, or <code>redirect</code>, and it must be the last action to be performed.</p> <p>If the action type is <code>forward</code>, you specify one or more target groups. The protocol of the target group must be HTTP or HTTPS for an Application Load Balancer. The protocol of the target group must be TCP, TLS, UDP, or TCP_UDP for a Network Load Balancer.</p> <p>[HTTPS listeners] If the action type is <code>authenticate-oidc</code>, you authenticate users through an identity provider that is OpenID Connect (OIDC) compliant.</p> <p>[HTTPS listeners] If the action type is <code>authenticate-cognito</code>, you authenticate users through the user pools supported by Amazon Cognito.</p> <p>[Application Load Balancer] If the action type is <code>redirect</code>, you redirect specified client requests from one URL to another.</p> <p>[Application Load Balancer] If the action type is <code>fixed-response</code>, you drop specified client requests and return a custom HTTP response.</p>
    pub actions: Vec<Action>,
    /// <p>The conditions. Each rule can include zero or one of the following conditions: <code>http-request-method</code>, <code>host-header</code>, <code>path-pattern</code>, and <code>source-ip</code>, and zero or more of the following conditions: <code>http-header</code> and <code>query-string</code>.</p>
    pub conditions: Vec<RuleCondition>,
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: String,
    /// <p>The rule priority. A listener can't have multiple rules with the same priority.</p>
    pub priority: i64,
}

/// Serialize `CreateRuleInput` contents to a `SignedRequest`.
struct CreateRuleInputSerializer;
impl CreateRuleInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateRuleInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ActionsSerializer::serialize(params, &format!("{}{}", prefix, "Actions"), &obj.actions);
        RuleConditionListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Conditions"),
            &obj.conditions,
        );
        params.put(&format!("{}{}", prefix, "ListenerArn"), &obj.listener_arn);
        params.put(&format!("{}{}", prefix, "Priority"), &obj.priority);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateRuleOutput {
    /// <p>Information about the rule.</p>
    pub rules: Option<Vec<Rule>>,
}

struct CreateRuleOutputDeserializer;
impl CreateRuleOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateRuleOutput, XmlParseError> {
        deserialize_elements::<_, CreateRuleOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Rules" => {
                    obj.rules
                        .get_or_insert(vec![])
                        .extend(RulesDeserializer::deserialize("Rules", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTargetGroupInput {
    /// <p>Indicates whether health checks are enabled. If the target type is <code>lambda</code>, health checks are disabled by default but can be enabled. If the target type is <code>instance</code> or <code>ip</code>, health checks are always enabled and cannot be disabled.</p>
    pub health_check_enabled: Option<bool>,
    /// <p>The approximate amount of time, in seconds, between health checks of an individual target. For HTTP and HTTPS health checks, the range is 5–300 seconds. For TCP health checks, the supported values are 10 and 30 seconds. If the target type is <code>instance</code> or <code>ip</code>, the default is 30 seconds. If the target type is <code>lambda</code>, the default is 35 seconds.</p>
    pub health_check_interval_seconds: Option<i64>,
    /// <p>[HTTP/HTTPS health checks] The ping path that is the destination on the targets for health checks. The default is /.</p>
    pub health_check_path: Option<String>,
    /// <p>The port the load balancer uses when performing health checks on targets. The default is <code>traffic-port</code>, which is the port on which each target receives traffic from the load balancer.</p>
    pub health_check_port: Option<String>,
    /// <p>The protocol the load balancer uses when performing health checks on targets. For Application Load Balancers, the default is HTTP. For Network Load Balancers, the default is TCP. The TCP protocol is supported for health checks only if the protocol of the target group is TCP, TLS, UDP, or TCP_UDP. The TLS, UDP, and TCP_UDP protocols are not supported for health checks.</p>
    pub health_check_protocol: Option<String>,
    /// <p>The amount of time, in seconds, during which no response from a target means a failed health check. For target groups with a protocol of HTTP or HTTPS, the default is 5 seconds. For target groups with a protocol of TCP or TLS, this value must be 6 seconds for HTTP health checks and 10 seconds for TCP and HTTPS health checks. If the target type is <code>lambda</code>, the default is 30 seconds.</p>
    pub health_check_timeout_seconds: Option<i64>,
    /// <p>The number of consecutive health checks successes required before considering an unhealthy target healthy. For target groups with a protocol of HTTP or HTTPS, the default is 5. For target groups with a protocol of TCP or TLS, the default is 3. If the target type is <code>lambda</code>, the default is 5.</p>
    pub healthy_threshold_count: Option<i64>,
    /// <p>[HTTP/HTTPS health checks] The HTTP codes to use when checking for a successful response from a target.</p>
    pub matcher: Option<Matcher>,
    /// <p>The name of the target group.</p> <p>This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.</p>
    pub name: String,
    /// <p>The port on which the targets receive traffic. This port is used unless you specify a port override when registering the target. If the target is a Lambda function, this parameter does not apply.</p>
    pub port: Option<i64>,
    /// <p>The protocol to use for routing traffic to the targets. For Application Load Balancers, the supported protocols are HTTP and HTTPS. For Network Load Balancers, the supported protocols are TCP, TLS, UDP, or TCP_UDP. A TCP_UDP listener must be associated with a TCP_UDP target group. If the target is a Lambda function, this parameter does not apply.</p>
    pub protocol: Option<String>,
    /// <p><p>The type of target that you must specify when registering targets with this target group. You can&#39;t specify targets for a target group using more than one target type.</p> <ul> <li> <p> <code>instance</code> - Targets are specified by instance ID. This is the default value. If the target group protocol is UDP or TCP_UDP, the target type must be <code>instance</code>.</p> </li> <li> <p> <code>ip</code> - Targets are specified by IP address. You can specify IP addresses from the subnets of the virtual private cloud (VPC) for the target group, the RFC 1918 range (10.0.0.0/8, 172.16.0.0/12, and 192.168.0.0/16), and the RFC 6598 range (100.64.0.0/10). You can&#39;t specify publicly routable IP addresses.</p> </li> <li> <p> <code>lambda</code> - The target groups contains a single Lambda function.</p> </li> </ul></p>
    pub target_type: Option<String>,
    /// <p>The number of consecutive health check failures required before considering a target unhealthy. For target groups with a protocol of HTTP or HTTPS, the default is 2. For target groups with a protocol of TCP or TLS, this value must be the same as the healthy threshold count. If the target type is <code>lambda</code>, the default is 2.</p>
    pub unhealthy_threshold_count: Option<i64>,
    /// <p>The identifier of the virtual private cloud (VPC). If the target is a Lambda function, this parameter does not apply. Otherwise, this parameter is required.</p>
    pub vpc_id: Option<String>,
}

/// Serialize `CreateTargetGroupInput` contents to a `SignedRequest`.
struct CreateTargetGroupInputSerializer;
impl CreateTargetGroupInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateTargetGroupInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.health_check_enabled {
            params.put(&format!("{}{}", prefix, "HealthCheckEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.health_check_interval_seconds {
            params.put(
                &format!("{}{}", prefix, "HealthCheckIntervalSeconds"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.health_check_path {
            params.put(&format!("{}{}", prefix, "HealthCheckPath"), &field_value);
        }
        if let Some(ref field_value) = obj.health_check_port {
            params.put(&format!("{}{}", prefix, "HealthCheckPort"), &field_value);
        }
        if let Some(ref field_value) = obj.health_check_protocol {
            params.put(
                &format!("{}{}", prefix, "HealthCheckProtocol"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.health_check_timeout_seconds {
            params.put(
                &format!("{}{}", prefix, "HealthCheckTimeoutSeconds"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.healthy_threshold_count {
            params.put(
                &format!("{}{}", prefix, "HealthyThresholdCount"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.matcher {
            MatcherSerializer::serialize(params, &format!("{}{}", prefix, "Matcher"), field_value);
        }
        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value);
        }
        if let Some(ref field_value) = obj.protocol {
            params.put(&format!("{}{}", prefix, "Protocol"), &field_value);
        }
        if let Some(ref field_value) = obj.target_type {
            params.put(&format!("{}{}", prefix, "TargetType"), &field_value);
        }
        if let Some(ref field_value) = obj.unhealthy_threshold_count {
            params.put(
                &format!("{}{}", prefix, "UnhealthyThresholdCount"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.vpc_id {
            params.put(&format!("{}{}", prefix, "VpcId"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateTargetGroupOutput {
    /// <p>Information about the target group.</p>
    pub target_groups: Option<Vec<TargetGroup>>,
}

struct CreateTargetGroupOutputDeserializer;
impl CreateTargetGroupOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateTargetGroupOutput, XmlParseError> {
        deserialize_elements::<_, CreateTargetGroupOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TargetGroups" => {
                        obj.target_groups.get_or_insert(vec![]).extend(
                            TargetGroupsDeserializer::deserialize("TargetGroups", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct CreatedTimeDeserializer;
impl CreatedTimeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DNSNameDeserializer;
impl DNSNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DefaultDeserializer;
impl DefaultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteListenerInput {
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: String,
}

/// Serialize `DeleteListenerInput` contents to a `SignedRequest`.
struct DeleteListenerInputSerializer;
impl DeleteListenerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteListenerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ListenerArn"), &obj.listener_arn);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteListenerOutput {}

struct DeleteListenerOutputDeserializer;
impl DeleteListenerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteListenerOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteListenerOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLoadBalancerInput {
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
}

/// Serialize `DeleteLoadBalancerInput` contents to a `SignedRequest`.
struct DeleteLoadBalancerInputSerializer;
impl DeleteLoadBalancerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteLoadBalancerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteLoadBalancerOutput {}

struct DeleteLoadBalancerOutputDeserializer;
impl DeleteLoadBalancerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteLoadBalancerOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteLoadBalancerOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRuleInput {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    pub rule_arn: String,
}

/// Serialize `DeleteRuleInput` contents to a `SignedRequest`.
struct DeleteRuleInputSerializer;
impl DeleteRuleInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteRuleInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "RuleArn"), &obj.rule_arn);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteRuleOutput {}

struct DeleteRuleOutputDeserializer;
impl DeleteRuleOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteRuleOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteRuleOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTargetGroupInput {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
}

/// Serialize `DeleteTargetGroupInput` contents to a `SignedRequest`.
struct DeleteTargetGroupInputSerializer;
impl DeleteTargetGroupInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteTargetGroupInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteTargetGroupOutput {}

struct DeleteTargetGroupOutputDeserializer;
impl DeleteTargetGroupOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteTargetGroupOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteTargetGroupOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterTargetsInput {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
    /// <p>The targets. If you specified a port override when you registered a target, you must specify both the target ID and the port when you deregister it.</p>
    pub targets: Vec<TargetDescription>,
}

/// Serialize `DeregisterTargetsInput` contents to a `SignedRequest`.
struct DeregisterTargetsInputSerializer;
impl DeregisterTargetsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeregisterTargetsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn,
        );
        TargetDescriptionsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Targets"),
            &obj.targets,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeregisterTargetsOutput {}

struct DeregisterTargetsOutputDeserializer;
impl DeregisterTargetsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeregisterTargetsOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeregisterTargetsOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAccountLimitsInput {
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
}

/// Serialize `DescribeAccountLimitsInput` contents to a `SignedRequest`.
struct DescribeAccountLimitsInputSerializer;
impl DescribeAccountLimitsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeAccountLimitsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.page_size {
            params.put(&format!("{}{}", prefix, "PageSize"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeAccountLimitsOutput {
    /// <p>Information about the limits.</p>
    pub limits: Option<Vec<Limit>>,
    /// <p>If there are additional results, this is the marker for the next set of results. Otherwise, this is null.</p>
    pub next_marker: Option<String>,
}

struct DescribeAccountLimitsOutputDeserializer;
impl DescribeAccountLimitsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAccountLimitsOutput, XmlParseError> {
        deserialize_elements::<_, DescribeAccountLimitsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Limits" => {
                        obj.limits
                            .get_or_insert(vec![])
                            .extend(LimitsDeserializer::deserialize("Limits", stack)?);
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(MarkerDeserializer::deserialize("NextMarker", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeListenerCertificatesInput {
    /// <p>The Amazon Resource Names (ARN) of the listener.</p>
    pub listener_arn: String,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
}

/// Serialize `DescribeListenerCertificatesInput` contents to a `SignedRequest`.
struct DescribeListenerCertificatesInputSerializer;
impl DescribeListenerCertificatesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeListenerCertificatesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ListenerArn"), &obj.listener_arn);
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.page_size {
            params.put(&format!("{}{}", prefix, "PageSize"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeListenerCertificatesOutput {
    /// <p>Information about the certificates.</p>
    pub certificates: Option<Vec<Certificate>>,
    /// <p>If there are additional results, this is the marker for the next set of results. Otherwise, this is null.</p>
    pub next_marker: Option<String>,
}

struct DescribeListenerCertificatesOutputDeserializer;
impl DescribeListenerCertificatesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeListenerCertificatesOutput, XmlParseError> {
        deserialize_elements::<_, DescribeListenerCertificatesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Certificates" => {
                        obj.certificates.get_or_insert(vec![]).extend(
                            CertificateListDeserializer::deserialize("Certificates", stack)?,
                        );
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(MarkerDeserializer::deserialize("NextMarker", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeListenersInput {
    /// <p>The Amazon Resource Names (ARN) of the listeners.</p>
    pub listener_arns: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: Option<String>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
}

/// Serialize `DescribeListenersInput` contents to a `SignedRequest`.
struct DescribeListenersInputSerializer;
impl DescribeListenersInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeListenersInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.listener_arns {
            ListenerArnsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ListenerArns"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.load_balancer_arn {
            params.put(&format!("{}{}", prefix, "LoadBalancerArn"), &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.page_size {
            params.put(&format!("{}{}", prefix, "PageSize"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeListenersOutput {
    /// <p>Information about the listeners.</p>
    pub listeners: Option<Vec<Listener>>,
    /// <p>If there are additional results, this is the marker for the next set of results. Otherwise, this is null.</p>
    pub next_marker: Option<String>,
}

struct DescribeListenersOutputDeserializer;
impl DescribeListenersOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeListenersOutput, XmlParseError> {
        deserialize_elements::<_, DescribeListenersOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Listeners" => {
                        obj.listeners
                            .get_or_insert(vec![])
                            .extend(ListenersDeserializer::deserialize("Listeners", stack)?);
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(MarkerDeserializer::deserialize("NextMarker", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLoadBalancerAttributesInput {
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
}

/// Serialize `DescribeLoadBalancerAttributesInput` contents to a `SignedRequest`.
struct DescribeLoadBalancerAttributesInputSerializer;
impl DescribeLoadBalancerAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeLoadBalancerAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeLoadBalancerAttributesOutput {
    /// <p>Information about the load balancer attributes.</p>
    pub attributes: Option<Vec<LoadBalancerAttribute>>,
}

struct DescribeLoadBalancerAttributesOutputDeserializer;
impl DescribeLoadBalancerAttributesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLoadBalancerAttributesOutput, XmlParseError> {
        deserialize_elements::<_, DescribeLoadBalancerAttributesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Attributes" => {
                        obj.attributes.get_or_insert(vec![]).extend(
                            LoadBalancerAttributesDeserializer::deserialize("Attributes", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLoadBalancersInput {
    /// <p>The Amazon Resource Names (ARN) of the load balancers. You can specify up to 20 load balancers in a single call.</p>
    pub load_balancer_arns: Option<Vec<String>>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The names of the load balancers.</p>
    pub names: Option<Vec<String>>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
}

/// Serialize `DescribeLoadBalancersInput` contents to a `SignedRequest`.
struct DescribeLoadBalancersInputSerializer;
impl DescribeLoadBalancersInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeLoadBalancersInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.load_balancer_arns {
            LoadBalancerArnsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LoadBalancerArns"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.names {
            LoadBalancerNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Names"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.page_size {
            params.put(&format!("{}{}", prefix, "PageSize"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeLoadBalancersOutput {
    /// <p>Information about the load balancers.</p>
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// <p>If there are additional results, this is the marker for the next set of results. Otherwise, this is null.</p>
    pub next_marker: Option<String>,
}

struct DescribeLoadBalancersOutputDeserializer;
impl DescribeLoadBalancersOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeLoadBalancersOutput, XmlParseError> {
        deserialize_elements::<_, DescribeLoadBalancersOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LoadBalancers" => {
                        obj.load_balancers.get_or_insert(vec![]).extend(
                            LoadBalancersDeserializer::deserialize("LoadBalancers", stack)?,
                        );
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(MarkerDeserializer::deserialize("NextMarker", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRulesInput {
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: Option<String>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
    /// <p>The Amazon Resource Names (ARN) of the rules.</p>
    pub rule_arns: Option<Vec<String>>,
}

/// Serialize `DescribeRulesInput` contents to a `SignedRequest`.
struct DescribeRulesInputSerializer;
impl DescribeRulesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeRulesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.listener_arn {
            params.put(&format!("{}{}", prefix, "ListenerArn"), &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.page_size {
            params.put(&format!("{}{}", prefix, "PageSize"), &field_value);
        }
        if let Some(ref field_value) = obj.rule_arns {
            RuleArnsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RuleArns"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeRulesOutput {
    /// <p>If there are additional results, this is the marker for the next set of results. Otherwise, this is null.</p>
    pub next_marker: Option<String>,
    /// <p>Information about the rules.</p>
    pub rules: Option<Vec<Rule>>,
}

struct DescribeRulesOutputDeserializer;
impl DescribeRulesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeRulesOutput, XmlParseError> {
        deserialize_elements::<_, DescribeRulesOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "NextMarker" => {
                    obj.next_marker = Some(MarkerDeserializer::deserialize("NextMarker", stack)?);
                }
                "Rules" => {
                    obj.rules
                        .get_or_insert(vec![])
                        .extend(RulesDeserializer::deserialize("Rules", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSSLPoliciesInput {
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The names of the policies.</p>
    pub names: Option<Vec<String>>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
}

/// Serialize `DescribeSSLPoliciesInput` contents to a `SignedRequest`.
struct DescribeSSLPoliciesInputSerializer;
impl DescribeSSLPoliciesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeSSLPoliciesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.names {
            SslPolicyNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Names"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.page_size {
            params.put(&format!("{}{}", prefix, "PageSize"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeSSLPoliciesOutput {
    /// <p>If there are additional results, this is the marker for the next set of results. Otherwise, this is null.</p>
    pub next_marker: Option<String>,
    /// <p>Information about the policies.</p>
    pub ssl_policies: Option<Vec<SslPolicy>>,
}

struct DescribeSSLPoliciesOutputDeserializer;
impl DescribeSSLPoliciesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeSSLPoliciesOutput, XmlParseError> {
        deserialize_elements::<_, DescribeSSLPoliciesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextMarker" => {
                        obj.next_marker =
                            Some(MarkerDeserializer::deserialize("NextMarker", stack)?);
                    }
                    "SslPolicies" => {
                        obj.ssl_policies
                            .get_or_insert(vec![])
                            .extend(SslPoliciesDeserializer::deserialize("SslPolicies", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTagsInput {
    /// <p>The Amazon Resource Names (ARN) of the resources.</p>
    pub resource_arns: Vec<String>,
}

/// Serialize `DescribeTagsInput` contents to a `SignedRequest`.
struct DescribeTagsInputSerializer;
impl DescribeTagsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeTagsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ResourceArnsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ResourceArns"),
            &obj.resource_arns,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeTagsOutput {
    /// <p>Information about the tags.</p>
    pub tag_descriptions: Option<Vec<TagDescription>>,
}

struct DescribeTagsOutputDeserializer;
impl DescribeTagsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeTagsOutput, XmlParseError> {
        deserialize_elements::<_, DescribeTagsOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "TagDescriptions" => {
                    obj.tag_descriptions.get_or_insert(vec![]).extend(
                        TagDescriptionsDeserializer::deserialize("TagDescriptions", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTargetGroupAttributesInput {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
}

/// Serialize `DescribeTargetGroupAttributesInput` contents to a `SignedRequest`.
struct DescribeTargetGroupAttributesInputSerializer;
impl DescribeTargetGroupAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeTargetGroupAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeTargetGroupAttributesOutput {
    /// <p>Information about the target group attributes</p>
    pub attributes: Option<Vec<TargetGroupAttribute>>,
}

struct DescribeTargetGroupAttributesOutputDeserializer;
impl DescribeTargetGroupAttributesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeTargetGroupAttributesOutput, XmlParseError> {
        deserialize_elements::<_, DescribeTargetGroupAttributesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Attributes" => {
                        obj.attributes.get_or_insert(vec![]).extend(
                            TargetGroupAttributesDeserializer::deserialize("Attributes", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTargetGroupsInput {
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: Option<String>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub marker: Option<String>,
    /// <p>The names of the target groups.</p>
    pub names: Option<Vec<String>>,
    /// <p>The maximum number of results to return with this call.</p>
    pub page_size: Option<i64>,
    /// <p>The Amazon Resource Names (ARN) of the target groups.</p>
    pub target_group_arns: Option<Vec<String>>,
}

/// Serialize `DescribeTargetGroupsInput` contents to a `SignedRequest`.
struct DescribeTargetGroupsInputSerializer;
impl DescribeTargetGroupsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeTargetGroupsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.load_balancer_arn {
            params.put(&format!("{}{}", prefix, "LoadBalancerArn"), &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.names {
            TargetGroupNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Names"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.page_size {
            params.put(&format!("{}{}", prefix, "PageSize"), &field_value);
        }
        if let Some(ref field_value) = obj.target_group_arns {
            TargetGroupArnsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TargetGroupArns"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeTargetGroupsOutput {
    /// <p>If there are additional results, this is the marker for the next set of results. Otherwise, this is null.</p>
    pub next_marker: Option<String>,
    /// <p>Information about the target groups.</p>
    pub target_groups: Option<Vec<TargetGroup>>,
}

struct DescribeTargetGroupsOutputDeserializer;
impl DescribeTargetGroupsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeTargetGroupsOutput, XmlParseError> {
        deserialize_elements::<_, DescribeTargetGroupsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextMarker" => {
                        obj.next_marker =
                            Some(MarkerDeserializer::deserialize("NextMarker", stack)?);
                    }
                    "TargetGroups" => {
                        obj.target_groups.get_or_insert(vec![]).extend(
                            TargetGroupsDeserializer::deserialize("TargetGroups", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTargetHealthInput {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
    /// <p>The targets.</p>
    pub targets: Option<Vec<TargetDescription>>,
}

/// Serialize `DescribeTargetHealthInput` contents to a `SignedRequest`.
struct DescribeTargetHealthInputSerializer;
impl DescribeTargetHealthInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeTargetHealthInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn,
        );
        if let Some(ref field_value) = obj.targets {
            TargetDescriptionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Targets"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeTargetHealthOutput {
    /// <p>Information about the health of the targets.</p>
    pub target_health_descriptions: Option<Vec<TargetHealthDescription>>,
}

struct DescribeTargetHealthOutputDeserializer;
impl DescribeTargetHealthOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeTargetHealthOutput, XmlParseError> {
        deserialize_elements::<_, DescribeTargetHealthOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TargetHealthDescriptions" => {
                        obj.target_health_descriptions.get_or_insert(vec![]).extend(
                            TargetHealthDescriptionsDeserializer::deserialize(
                                "TargetHealthDescriptions",
                                stack,
                            )?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct DescriptionDeserializer;
impl DescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about an action that returns a custom HTTP response.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FixedResponseActionConfig {
    /// <p>The content type.</p> <p>Valid Values: text/plain | text/css | text/html | application/javascript | application/json</p>
    pub content_type: Option<String>,
    /// <p>The message.</p>
    pub message_body: Option<String>,
    /// <p>The HTTP response code (2XX, 4XX, or 5XX).</p>
    pub status_code: String,
}

struct FixedResponseActionConfigDeserializer;
impl FixedResponseActionConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FixedResponseActionConfig, XmlParseError> {
        deserialize_elements::<_, FixedResponseActionConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ContentType" => {
                        obj.content_type =
                            Some(FixedResponseActionContentTypeDeserializer::deserialize(
                                "ContentType",
                                stack,
                            )?);
                    }
                    "MessageBody" => {
                        obj.message_body =
                            Some(FixedResponseActionMessageDeserializer::deserialize(
                                "MessageBody",
                                stack,
                            )?);
                    }
                    "StatusCode" => {
                        obj.status_code = FixedResponseActionStatusCodeDeserializer::deserialize(
                            "StatusCode",
                            stack,
                        )?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

/// Serialize `FixedResponseActionConfig` contents to a `SignedRequest`.
struct FixedResponseActionConfigSerializer;
impl FixedResponseActionConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &FixedResponseActionConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.content_type {
            params.put(&format!("{}{}", prefix, "ContentType"), &field_value);
        }
        if let Some(ref field_value) = obj.message_body {
            params.put(&format!("{}{}", prefix, "MessageBody"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StatusCode"), &obj.status_code);
    }
}

struct FixedResponseActionContentTypeDeserializer;
impl FixedResponseActionContentTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct FixedResponseActionMessageDeserializer;
impl FixedResponseActionMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct FixedResponseActionStatusCodeDeserializer;
impl FixedResponseActionStatusCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about a forward action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ForwardActionConfig {
    /// <p>The target group stickiness for the rule.</p>
    pub target_group_stickiness_config: Option<TargetGroupStickinessConfig>,
    /// <p>One or more target groups. For Network Load Balancers, you can specify a single target group.</p>
    pub target_groups: Option<Vec<TargetGroupTuple>>,
}

struct ForwardActionConfigDeserializer;
impl ForwardActionConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ForwardActionConfig, XmlParseError> {
        deserialize_elements::<_, ForwardActionConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "TargetGroupStickinessConfig" => {
                    obj.target_group_stickiness_config =
                        Some(TargetGroupStickinessConfigDeserializer::deserialize(
                            "TargetGroupStickinessConfig",
                            stack,
                        )?);
                }
                "TargetGroups" => {
                    obj.target_groups.get_or_insert(vec![]).extend(
                        TargetGroupListDeserializer::deserialize("TargetGroups", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `ForwardActionConfig` contents to a `SignedRequest`.
struct ForwardActionConfigSerializer;
impl ForwardActionConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ForwardActionConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.target_group_stickiness_config {
            TargetGroupStickinessConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TargetGroupStickinessConfig"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.target_groups {
            TargetGroupListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TargetGroups"),
                field_value,
            );
        }
    }
}

struct HealthCheckEnabledDeserializer;
impl HealthCheckEnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct HealthCheckIntervalSecondsDeserializer;
impl HealthCheckIntervalSecondsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct HealthCheckPortDeserializer;
impl HealthCheckPortDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct HealthCheckThresholdCountDeserializer;
impl HealthCheckThresholdCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct HealthCheckTimeoutSecondsDeserializer;
impl HealthCheckTimeoutSecondsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about a host header condition.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HostHeaderConditionConfig {
    /// <p>One or more host names. The maximum size of each name is 128 characters. The comparison is case insensitive. The following wildcard characters are supported: * (matches 0 or more characters) and ? (matches exactly 1 character).</p> <p>If you specify multiple strings, the condition is satisfied if one of the strings matches the host name.</p>
    pub values: Option<Vec<String>>,
}

struct HostHeaderConditionConfigDeserializer;
impl HostHeaderConditionConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HostHeaderConditionConfig, XmlParseError> {
        deserialize_elements::<_, HostHeaderConditionConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Values" => {
                        obj.values
                            .get_or_insert(vec![])
                            .extend(ListOfStringDeserializer::deserialize("Values", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

/// Serialize `HostHeaderConditionConfig` contents to a `SignedRequest`.
struct HostHeaderConditionConfigSerializer;
impl HostHeaderConditionConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &HostHeaderConditionConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.values {
            ListOfStringSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Values"),
                field_value,
            );
        }
    }
}

struct HttpCodeDeserializer;
impl HttpCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about an HTTP header condition.</p> <p>There is a set of standard HTTP header fields. You can also define custom HTTP header fields.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HttpHeaderConditionConfig {
    /// <p>The name of the HTTP header field. The maximum size is 40 characters. The header name is case insensitive. The allowed characters are specified by RFC 7230. Wildcards are not supported.</p> <p>You can't use an HTTP header condition to specify the host header. Use <a>HostHeaderConditionConfig</a> to specify a host header condition.</p>
    pub http_header_name: Option<String>,
    /// <p>One or more strings to compare against the value of the HTTP header. The maximum size of each string is 128 characters. The comparison strings are case insensitive. The following wildcard characters are supported: * (matches 0 or more characters) and ? (matches exactly 1 character).</p> <p>If the same header appears multiple times in the request, we search them in order until a match is found.</p> <p>If you specify multiple strings, the condition is satisfied if one of the strings matches the value of the HTTP header. To require that all of the strings are a match, create one condition per string.</p>
    pub values: Option<Vec<String>>,
}

struct HttpHeaderConditionConfigDeserializer;
impl HttpHeaderConditionConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HttpHeaderConditionConfig, XmlParseError> {
        deserialize_elements::<_, HttpHeaderConditionConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HttpHeaderName" => {
                        obj.http_header_name =
                            Some(HttpHeaderConditionNameDeserializer::deserialize(
                                "HttpHeaderName",
                                stack,
                            )?);
                    }
                    "Values" => {
                        obj.values
                            .get_or_insert(vec![])
                            .extend(ListOfStringDeserializer::deserialize("Values", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

/// Serialize `HttpHeaderConditionConfig` contents to a `SignedRequest`.
struct HttpHeaderConditionConfigSerializer;
impl HttpHeaderConditionConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &HttpHeaderConditionConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.http_header_name {
            params.put(&format!("{}{}", prefix, "HttpHeaderName"), &field_value);
        }
        if let Some(ref field_value) = obj.values {
            ListOfStringSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Values"),
                field_value,
            );
        }
    }
}

struct HttpHeaderConditionNameDeserializer;
impl HttpHeaderConditionNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about an HTTP method condition.</p> <p>HTTP defines a set of request methods, also referred to as HTTP verbs. For more information, see the <a href="https://www.iana.org/assignments/http-methods/http-methods.xhtml">HTTP Method Registry</a>. You can also define custom HTTP methods.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HttpRequestMethodConditionConfig {
    /// <p>The name of the request method. The maximum size is 40 characters. The allowed characters are A-Z, hyphen (-), and underscore (_). The comparison is case sensitive. Wildcards are not supported; therefore, the method name must be an exact match.</p> <p>If you specify multiple strings, the condition is satisfied if one of the strings matches the HTTP request method. We recommend that you route GET and HEAD requests in the same way, because the response to a HEAD request may be cached.</p>
    pub values: Option<Vec<String>>,
}

struct HttpRequestMethodConditionConfigDeserializer;
impl HttpRequestMethodConditionConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HttpRequestMethodConditionConfig, XmlParseError> {
        deserialize_elements::<_, HttpRequestMethodConditionConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Values" => {
                        obj.values
                            .get_or_insert(vec![])
                            .extend(ListOfStringDeserializer::deserialize("Values", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

/// Serialize `HttpRequestMethodConditionConfig` contents to a `SignedRequest`.
struct HttpRequestMethodConditionConfigSerializer;
impl HttpRequestMethodConditionConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &HttpRequestMethodConditionConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.values {
            ListOfStringSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Values"),
                field_value,
            );
        }
    }
}

struct IpAddressDeserializer;
impl IpAddressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct IpAddressTypeDeserializer;
impl IpAddressTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct IsDefaultDeserializer;
impl IsDefaultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about an Elastic Load Balancing resource limit for your AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Limit {
    /// <p>The maximum value of the limit.</p>
    pub max: Option<String>,
    /// <p><p>The name of the limit. The possible values are:</p> <ul> <li> <p>application-load-balancers</p> </li> <li> <p>listeners-per-application-load-balancer</p> </li> <li> <p>listeners-per-network-load-balancer</p> </li> <li> <p>network-load-balancers</p> </li> <li> <p>rules-per-application-load-balancer</p> </li> <li> <p>target-groups</p> </li> <li> <p>target-groups-per-action-on-application-load-balancer</p> </li> <li> <p>target-groups-per-action-on-network-load-balancer</p> </li> <li> <p>target-groups-per-application-load-balancer</p> </li> <li> <p>targets-per-application-load-balancer</p> </li> <li> <p>targets-per-availability-zone-per-network-load-balancer</p> </li> <li> <p>targets-per-network-load-balancer</p> </li> </ul></p>
    pub name: Option<String>,
}

struct LimitDeserializer;
impl LimitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Limit, XmlParseError> {
        deserialize_elements::<_, Limit, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Max" => {
                    obj.max = Some(MaxDeserializer::deserialize("Max", stack)?);
                }
                "Name" => {
                    obj.name = Some(NameDeserializer::deserialize("Name", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct LimitsDeserializer;
impl LimitsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Limit>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(LimitDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ListOfStringDeserializer;
impl ListOfStringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StringValueDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `ListOfString` contents to a `SignedRequest`.
struct ListOfStringSerializer;
impl ListOfStringSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a listener.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Listener {
    /// <p>[HTTPS or TLS listener] The default certificate for the listener.</p>
    pub certificates: Option<Vec<Certificate>>,
    /// <p>The default actions for the listener.</p>
    pub default_actions: Option<Vec<Action>>,
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: Option<String>,
    /// <p>The port on which the load balancer is listening.</p>
    pub port: Option<i64>,
    /// <p>The protocol for connections from clients to the load balancer.</p>
    pub protocol: Option<String>,
    /// <p>[HTTPS or TLS listener] The security policy that defines which ciphers and protocols are supported. The default is the current predefined security policy.</p>
    pub ssl_policy: Option<String>,
}

struct ListenerDeserializer;
impl ListenerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Listener, XmlParseError> {
        deserialize_elements::<_, Listener, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Certificates" => {
                    obj.certificates.get_or_insert(vec![]).extend(
                        CertificateListDeserializer::deserialize("Certificates", stack)?,
                    );
                }
                "DefaultActions" => {
                    obj.default_actions
                        .get_or_insert(vec![])
                        .extend(ActionsDeserializer::deserialize("DefaultActions", stack)?);
                }
                "ListenerArn" => {
                    obj.listener_arn =
                        Some(ListenerArnDeserializer::deserialize("ListenerArn", stack)?);
                }
                "LoadBalancerArn" => {
                    obj.load_balancer_arn = Some(LoadBalancerArnDeserializer::deserialize(
                        "LoadBalancerArn",
                        stack,
                    )?);
                }
                "Port" => {
                    obj.port = Some(PortDeserializer::deserialize("Port", stack)?);
                }
                "Protocol" => {
                    obj.protocol = Some(ProtocolEnumDeserializer::deserialize("Protocol", stack)?);
                }
                "SslPolicy" => {
                    obj.ssl_policy =
                        Some(SslPolicyNameDeserializer::deserialize("SslPolicy", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ListenerArnDeserializer;
impl ListenerArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `ListenerArns` contents to a `SignedRequest`.
struct ListenerArnsSerializer;
impl ListenerArnsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct ListenersDeserializer;
impl ListenersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Listener>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ListenerDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Information about a load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LoadBalancer {
    /// <p>The Availability Zones for the load balancer.</p>
    pub availability_zones: Option<Vec<AvailabilityZone>>,
    /// <p>The ID of the Amazon Route 53 hosted zone associated with the load balancer.</p>
    pub canonical_hosted_zone_id: Option<String>,
    /// <p>The date and time the load balancer was created.</p>
    pub created_time: Option<String>,
    /// <p>The public DNS name of the load balancer.</p>
    pub dns_name: Option<String>,
    /// <p>The type of IP addresses used by the subnets for your load balancer. The possible values are <code>ipv4</code> (for IPv4 addresses) and <code>dualstack</code> (for IPv4 and IPv6 addresses).</p>
    pub ip_address_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: Option<String>,
    /// <p>The name of the load balancer.</p>
    pub load_balancer_name: Option<String>,
    /// <p>The nodes of an Internet-facing load balancer have public IP addresses. The DNS name of an Internet-facing load balancer is publicly resolvable to the public IP addresses of the nodes. Therefore, Internet-facing load balancers can route requests from clients over the internet.</p> <p>The nodes of an internal load balancer have only private IP addresses. The DNS name of an internal load balancer is publicly resolvable to the private IP addresses of the nodes. Therefore, internal load balancers can route requests only from clients with access to the VPC for the load balancer.</p>
    pub scheme: Option<String>,
    /// <p>The IDs of the security groups for the load balancer.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p>The state of the load balancer.</p>
    pub state: Option<LoadBalancerState>,
    /// <p>The type of load balancer.</p>
    pub type_: Option<String>,
    /// <p>The ID of the VPC for the load balancer.</p>
    pub vpc_id: Option<String>,
}

struct LoadBalancerDeserializer;
impl LoadBalancerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancer, XmlParseError> {
        deserialize_elements::<_, LoadBalancer, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AvailabilityZones" => {
                    obj.availability_zones.get_or_insert(vec![]).extend(
                        AvailabilityZonesDeserializer::deserialize("AvailabilityZones", stack)?,
                    );
                }
                "CanonicalHostedZoneId" => {
                    obj.canonical_hosted_zone_id =
                        Some(CanonicalHostedZoneIdDeserializer::deserialize(
                            "CanonicalHostedZoneId",
                            stack,
                        )?);
                }
                "CreatedTime" => {
                    obj.created_time =
                        Some(CreatedTimeDeserializer::deserialize("CreatedTime", stack)?);
                }
                "DNSName" => {
                    obj.dns_name = Some(DNSNameDeserializer::deserialize("DNSName", stack)?);
                }
                "IpAddressType" => {
                    obj.ip_address_type = Some(IpAddressTypeDeserializer::deserialize(
                        "IpAddressType",
                        stack,
                    )?);
                }
                "LoadBalancerArn" => {
                    obj.load_balancer_arn = Some(LoadBalancerArnDeserializer::deserialize(
                        "LoadBalancerArn",
                        stack,
                    )?);
                }
                "LoadBalancerName" => {
                    obj.load_balancer_name = Some(LoadBalancerNameDeserializer::deserialize(
                        "LoadBalancerName",
                        stack,
                    )?);
                }
                "Scheme" => {
                    obj.scheme = Some(LoadBalancerSchemeEnumDeserializer::deserialize(
                        "Scheme", stack,
                    )?);
                }
                "SecurityGroups" => {
                    obj.security_groups.get_or_insert(vec![]).extend(
                        SecurityGroupsDeserializer::deserialize("SecurityGroups", stack)?,
                    );
                }
                "State" => {
                    obj.state = Some(LoadBalancerStateDeserializer::deserialize("State", stack)?);
                }
                "Type" => {
                    obj.type_ = Some(LoadBalancerTypeEnumDeserializer::deserialize(
                        "Type", stack,
                    )?);
                }
                "VpcId" => {
                    obj.vpc_id = Some(VpcIdDeserializer::deserialize("VpcId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Information about a static IP address for a load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LoadBalancerAddress {
    /// <p>[Network Load Balancers] The allocation ID of the Elastic IP address for an internal-facing load balancer.</p>
    pub allocation_id: Option<String>,
    /// <p>The static IP address.</p>
    pub ip_address: Option<String>,
    /// <p>[Network Load Balancers] The private IPv4 address for an internal load balancer.</p>
    pub private_i_pv_4_address: Option<String>,
}

struct LoadBalancerAddressDeserializer;
impl LoadBalancerAddressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerAddress, XmlParseError> {
        deserialize_elements::<_, LoadBalancerAddress, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AllocationId" => {
                    obj.allocation_id = Some(AllocationIdDeserializer::deserialize(
                        "AllocationId",
                        stack,
                    )?);
                }
                "IpAddress" => {
                    obj.ip_address = Some(IpAddressDeserializer::deserialize("IpAddress", stack)?);
                }
                "PrivateIPv4Address" => {
                    obj.private_i_pv_4_address = Some(PrivateIPv4AddressDeserializer::deserialize(
                        "PrivateIPv4Address",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct LoadBalancerAddressesDeserializer;
impl LoadBalancerAddressesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LoadBalancerAddress>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(LoadBalancerAddressDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct LoadBalancerArnDeserializer;
impl LoadBalancerArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct LoadBalancerArnsDeserializer;
impl LoadBalancerArnsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(LoadBalancerArnDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `LoadBalancerArns` contents to a `SignedRequest`.
struct LoadBalancerArnsSerializer;
impl LoadBalancerArnsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a load balancer attribute.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LoadBalancerAttribute {
    /// <p><p>The name of the attribute.</p> <p>The following attributes are supported by both Application Load Balancers and Network Load Balancers:</p> <ul> <li> <p> <code>access<em>logs.s3.enabled</code> - Indicates whether access logs are enabled. The value is <code>true</code> or <code>false</code>. The default is <code>false</code>.</p> </li> <li> <p> <code>access</em>logs.s3.bucket</code> - The name of the S3 bucket for the access logs. This attribute is required if access logs are enabled. The bucket must exist in the same region as the load balancer and have a bucket policy that grants Elastic Load Balancing permissions to write to the bucket.</p> </li> <li> <p> <code>access<em>logs.s3.prefix</code> - The prefix for the location in the S3 bucket for the access logs.</p> </li> <li> <p> <code>deletion</em>protection.enabled</code> - Indicates whether deletion protection is enabled. The value is <code>true</code> or <code>false</code>. The default is <code>false</code>.</p> </li> </ul> <p>The following attributes are supported by only Application Load Balancers:</p> <ul> <li> <p> <code>idle<em>timeout.timeout</em>seconds</code> - The idle timeout value, in seconds. The valid range is 1-4000 seconds. The default is 60 seconds.</p> </li> <li> <p> <code>routing.http.drop<em>invalid</em>header<em>fields.enabled</code> - Indicates whether HTTP headers with invalid header fields are removed by the load balancer (<code>true</code>) or routed to targets (<code>false</code>). The default is <code>false</code>.</p> </li> <li> <p> <code>routing.http2.enabled</code> - Indicates whether HTTP/2 is enabled. The value is <code>true</code> or <code>false</code>. The default is <code>true</code>.</p> </li> </ul> <p>The following attributes are supported by only Network Load Balancers:</p> <ul> <li> <p> <code>load</em>balancing.cross_zone.enabled</code> - Indicates whether cross-zone load balancing is enabled. The value is <code>true</code> or <code>false</code>. The default is <code>false</code>.</p> </li> </ul></p>
    pub key: Option<String>,
    /// <p>The value of the attribute.</p>
    pub value: Option<String>,
}

struct LoadBalancerAttributeDeserializer;
impl LoadBalancerAttributeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerAttribute, XmlParseError> {
        deserialize_elements::<_, LoadBalancerAttribute, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Key" => {
                    obj.key = Some(LoadBalancerAttributeKeyDeserializer::deserialize(
                        "Key", stack,
                    )?);
                }
                "Value" => {
                    obj.value = Some(LoadBalancerAttributeValueDeserializer::deserialize(
                        "Value", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `LoadBalancerAttribute` contents to a `SignedRequest`.
struct LoadBalancerAttributeSerializer;
impl LoadBalancerAttributeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &LoadBalancerAttribute) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.key {
            params.put(&format!("{}{}", prefix, "Key"), &field_value);
        }
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }
    }
}

struct LoadBalancerAttributeKeyDeserializer;
impl LoadBalancerAttributeKeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct LoadBalancerAttributeValueDeserializer;
impl LoadBalancerAttributeValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct LoadBalancerAttributesDeserializer;
impl LoadBalancerAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LoadBalancerAttribute>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(LoadBalancerAttributeDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `LoadBalancerAttributes` contents to a `SignedRequest`.
struct LoadBalancerAttributesSerializer;
impl LoadBalancerAttributesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<LoadBalancerAttribute>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            LoadBalancerAttributeSerializer::serialize(params, &key, obj);
        }
    }
}

struct LoadBalancerNameDeserializer;
impl LoadBalancerNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `LoadBalancerNames` contents to a `SignedRequest`.
struct LoadBalancerNamesSerializer;
impl LoadBalancerNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct LoadBalancerSchemeEnumDeserializer;
impl LoadBalancerSchemeEnumDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about the state of the load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LoadBalancerState {
    /// <p>The state code. The initial state of the load balancer is <code>provisioning</code>. After the load balancer is fully set up and ready to route traffic, its state is <code>active</code>. If the load balancer could not be set up, its state is <code>failed</code>.</p>
    pub code: Option<String>,
    /// <p>A description of the state.</p>
    pub reason: Option<String>,
}

struct LoadBalancerStateDeserializer;
impl LoadBalancerStateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerState, XmlParseError> {
        deserialize_elements::<_, LoadBalancerState, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Code" => {
                    obj.code = Some(LoadBalancerStateEnumDeserializer::deserialize(
                        "Code", stack,
                    )?);
                }
                "Reason" => {
                    obj.reason = Some(StateReasonDeserializer::deserialize("Reason", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct LoadBalancerStateEnumDeserializer;
impl LoadBalancerStateEnumDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct LoadBalancerTypeEnumDeserializer;
impl LoadBalancerTypeEnumDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct LoadBalancersDeserializer;
impl LoadBalancersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LoadBalancer>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(LoadBalancerDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct MarkerDeserializer;
impl MarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information to use when checking for a successful response from a target.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Matcher {
    /// <p>The HTTP codes.</p> <p>For Application Load Balancers, you can specify values between 200 and 499, and the default value is 200. You can specify multiple values (for example, "200,202") or a range of values (for example, "200-299").</p> <p>For Network Load Balancers, this is 200–399.</p>
    pub http_code: String,
}

struct MatcherDeserializer;
impl MatcherDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Matcher, XmlParseError> {
        deserialize_elements::<_, Matcher, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "HttpCode" => {
                    obj.http_code = HttpCodeDeserializer::deserialize("HttpCode", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `Matcher` contents to a `SignedRequest`.
struct MatcherSerializer;
impl MatcherSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Matcher) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "HttpCode"), &obj.http_code);
    }
}

struct MaxDeserializer;
impl MaxDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyListenerInput {
    /// <p>[HTTPS and TLS listeners] The default certificate for the listener. You must provide exactly one certificate. Set <code>CertificateArn</code> to the certificate ARN but do not set <code>IsDefault</code>.</p> <p>To create a certificate list, use <a>AddListenerCertificates</a>.</p>
    pub certificates: Option<Vec<Certificate>>,
    /// <p>The actions for the default rule. The rule must include one forward action or one or more fixed-response actions.</p> <p>If the action type is <code>forward</code>, you specify one or more target groups. The protocol of the target group must be HTTP or HTTPS for an Application Load Balancer. The protocol of the target group must be TCP, TLS, UDP, or TCP_UDP for a Network Load Balancer.</p> <p>[HTTPS listeners] If the action type is <code>authenticate-oidc</code>, you authenticate users through an identity provider that is OpenID Connect (OIDC) compliant.</p> <p>[HTTPS listeners] If the action type is <code>authenticate-cognito</code>, you authenticate users through the user pools supported by Amazon Cognito.</p> <p>[Application Load Balancer] If the action type is <code>redirect</code>, you redirect specified client requests from one URL to another.</p> <p>[Application Load Balancer] If the action type is <code>fixed-response</code>, you drop specified client requests and return a custom HTTP response.</p>
    pub default_actions: Option<Vec<Action>>,
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: String,
    /// <p>The port for connections from clients to the load balancer.</p>
    pub port: Option<i64>,
    /// <p>The protocol for connections from clients to the load balancer. Application Load Balancers support the HTTP and HTTPS protocols. Network Load Balancers support the TCP, TLS, UDP, and TCP_UDP protocols.</p>
    pub protocol: Option<String>,
    /// <p>[HTTPS and TLS listeners] The security policy that defines which protocols and ciphers are supported. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/create-https-listener.html#describe-ssl-policies">Security Policies</a> in the <i>Application Load Balancers Guide</i>.</p>
    pub ssl_policy: Option<String>,
}

/// Serialize `ModifyListenerInput` contents to a `SignedRequest`.
struct ModifyListenerInputSerializer;
impl ModifyListenerInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyListenerInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.certificates {
            CertificateListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Certificates"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.default_actions {
            ActionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "DefaultActions"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "ListenerArn"), &obj.listener_arn);
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value);
        }
        if let Some(ref field_value) = obj.protocol {
            params.put(&format!("{}{}", prefix, "Protocol"), &field_value);
        }
        if let Some(ref field_value) = obj.ssl_policy {
            params.put(&format!("{}{}", prefix, "SslPolicy"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyListenerOutput {
    /// <p>Information about the modified listener.</p>
    pub listeners: Option<Vec<Listener>>,
}

struct ModifyListenerOutputDeserializer;
impl ModifyListenerOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyListenerOutput, XmlParseError> {
        deserialize_elements::<_, ModifyListenerOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Listeners" => {
                    obj.listeners
                        .get_or_insert(vec![])
                        .extend(ListenersDeserializer::deserialize("Listeners", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyLoadBalancerAttributesInput {
    /// <p>The load balancer attributes.</p>
    pub attributes: Vec<LoadBalancerAttribute>,
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
}

/// Serialize `ModifyLoadBalancerAttributesInput` contents to a `SignedRequest`.
struct ModifyLoadBalancerAttributesInputSerializer;
impl ModifyLoadBalancerAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyLoadBalancerAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        LoadBalancerAttributesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Attributes"),
            &obj.attributes,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyLoadBalancerAttributesOutput {
    /// <p>Information about the load balancer attributes.</p>
    pub attributes: Option<Vec<LoadBalancerAttribute>>,
}

struct ModifyLoadBalancerAttributesOutputDeserializer;
impl ModifyLoadBalancerAttributesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyLoadBalancerAttributesOutput, XmlParseError> {
        deserialize_elements::<_, ModifyLoadBalancerAttributesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Attributes" => {
                        obj.attributes.get_or_insert(vec![]).extend(
                            LoadBalancerAttributesDeserializer::deserialize("Attributes", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyRuleInput {
    /// <p>The actions. Each rule must include exactly one of the following types of actions: <code>forward</code>, <code>fixed-response</code>, or <code>redirect</code>, and it must be the last action to be performed.</p> <p>If the action type is <code>forward</code>, you specify one or more target groups. The protocol of the target group must be HTTP or HTTPS for an Application Load Balancer. The protocol of the target group must be TCP, TLS, UDP, or TCP_UDP for a Network Load Balancer.</p> <p>[HTTPS listeners] If the action type is <code>authenticate-oidc</code>, you authenticate users through an identity provider that is OpenID Connect (OIDC) compliant.</p> <p>[HTTPS listeners] If the action type is <code>authenticate-cognito</code>, you authenticate users through the user pools supported by Amazon Cognito.</p> <p>[Application Load Balancer] If the action type is <code>redirect</code>, you redirect specified client requests from one URL to another.</p> <p>[Application Load Balancer] If the action type is <code>fixed-response</code>, you drop specified client requests and return a custom HTTP response.</p>
    pub actions: Option<Vec<Action>>,
    /// <p>The conditions. Each rule can include zero or one of the following conditions: <code>http-request-method</code>, <code>host-header</code>, <code>path-pattern</code>, and <code>source-ip</code>, and zero or more of the following conditions: <code>http-header</code> and <code>query-string</code>.</p>
    pub conditions: Option<Vec<RuleCondition>>,
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    pub rule_arn: String,
}

/// Serialize `ModifyRuleInput` contents to a `SignedRequest`.
struct ModifyRuleInputSerializer;
impl ModifyRuleInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyRuleInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.actions {
            ActionsSerializer::serialize(params, &format!("{}{}", prefix, "Actions"), field_value);
        }
        if let Some(ref field_value) = obj.conditions {
            RuleConditionListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Conditions"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "RuleArn"), &obj.rule_arn);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyRuleOutput {
    /// <p>Information about the modified rule.</p>
    pub rules: Option<Vec<Rule>>,
}

struct ModifyRuleOutputDeserializer;
impl ModifyRuleOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyRuleOutput, XmlParseError> {
        deserialize_elements::<_, ModifyRuleOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Rules" => {
                    obj.rules
                        .get_or_insert(vec![])
                        .extend(RulesDeserializer::deserialize("Rules", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyTargetGroupAttributesInput {
    /// <p>The attributes.</p>
    pub attributes: Vec<TargetGroupAttribute>,
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
}

/// Serialize `ModifyTargetGroupAttributesInput` contents to a `SignedRequest`.
struct ModifyTargetGroupAttributesInputSerializer;
impl ModifyTargetGroupAttributesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyTargetGroupAttributesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        TargetGroupAttributesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Attributes"),
            &obj.attributes,
        );
        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyTargetGroupAttributesOutput {
    /// <p>Information about the attributes.</p>
    pub attributes: Option<Vec<TargetGroupAttribute>>,
}

struct ModifyTargetGroupAttributesOutputDeserializer;
impl ModifyTargetGroupAttributesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyTargetGroupAttributesOutput, XmlParseError> {
        deserialize_elements::<_, ModifyTargetGroupAttributesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Attributes" => {
                        obj.attributes.get_or_insert(vec![]).extend(
                            TargetGroupAttributesDeserializer::deserialize("Attributes", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyTargetGroupInput {
    /// <p>Indicates whether health checks are enabled.</p>
    pub health_check_enabled: Option<bool>,
    /// <p>The approximate amount of time, in seconds, between health checks of an individual target. For Application Load Balancers, the range is 5 to 300 seconds. For Network Load Balancers, the supported values are 10 or 30 seconds.</p> <p>With Network Load Balancers, you can't modify this setting.</p>
    pub health_check_interval_seconds: Option<i64>,
    /// <p>[HTTP/HTTPS health checks] The ping path that is the destination for the health check request.</p>
    pub health_check_path: Option<String>,
    /// <p>The port the load balancer uses when performing health checks on targets.</p>
    pub health_check_port: Option<String>,
    /// <p>The protocol the load balancer uses when performing health checks on targets. The TCP protocol is supported for health checks only if the protocol of the target group is TCP, TLS, UDP, or TCP_UDP. The TLS, UDP, and TCP_UDP protocols are not supported for health checks.</p> <p>With Network Load Balancers, you can't modify this setting.</p>
    pub health_check_protocol: Option<String>,
    /// <p>[HTTP/HTTPS health checks] The amount of time, in seconds, during which no response means a failed health check.</p> <p>With Network Load Balancers, you can't modify this setting.</p>
    pub health_check_timeout_seconds: Option<i64>,
    /// <p>The number of consecutive health checks successes required before considering an unhealthy target healthy.</p>
    pub healthy_threshold_count: Option<i64>,
    /// <p>[HTTP/HTTPS health checks] The HTTP codes to use when checking for a successful response from a target.</p> <p>With Network Load Balancers, you can't modify this setting.</p>
    pub matcher: Option<Matcher>,
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
    /// <p>The number of consecutive health check failures required before considering the target unhealthy. For Network Load Balancers, this value must be the same as the healthy threshold count.</p>
    pub unhealthy_threshold_count: Option<i64>,
}

/// Serialize `ModifyTargetGroupInput` contents to a `SignedRequest`.
struct ModifyTargetGroupInputSerializer;
impl ModifyTargetGroupInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyTargetGroupInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.health_check_enabled {
            params.put(&format!("{}{}", prefix, "HealthCheckEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.health_check_interval_seconds {
            params.put(
                &format!("{}{}", prefix, "HealthCheckIntervalSeconds"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.health_check_path {
            params.put(&format!("{}{}", prefix, "HealthCheckPath"), &field_value);
        }
        if let Some(ref field_value) = obj.health_check_port {
            params.put(&format!("{}{}", prefix, "HealthCheckPort"), &field_value);
        }
        if let Some(ref field_value) = obj.health_check_protocol {
            params.put(
                &format!("{}{}", prefix, "HealthCheckProtocol"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.health_check_timeout_seconds {
            params.put(
                &format!("{}{}", prefix, "HealthCheckTimeoutSeconds"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.healthy_threshold_count {
            params.put(
                &format!("{}{}", prefix, "HealthyThresholdCount"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.matcher {
            MatcherSerializer::serialize(params, &format!("{}{}", prefix, "Matcher"), field_value);
        }
        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn,
        );
        if let Some(ref field_value) = obj.unhealthy_threshold_count {
            params.put(
                &format!("{}{}", prefix, "UnhealthyThresholdCount"),
                &field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyTargetGroupOutput {
    /// <p>Information about the modified target group.</p>
    pub target_groups: Option<Vec<TargetGroup>>,
}

struct ModifyTargetGroupOutputDeserializer;
impl ModifyTargetGroupOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyTargetGroupOutput, XmlParseError> {
        deserialize_elements::<_, ModifyTargetGroupOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TargetGroups" => {
                        obj.target_groups.get_or_insert(vec![]).extend(
                            TargetGroupsDeserializer::deserialize("TargetGroups", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct NameDeserializer;
impl NameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PathDeserializer;
impl PathDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about a path pattern condition.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PathPatternConditionConfig {
    /// <p>One or more path patterns to compare against the request URL. The maximum size of each string is 128 characters. The comparison is case sensitive. The following wildcard characters are supported: * (matches 0 or more characters) and ? (matches exactly 1 character).</p> <p>If you specify multiple strings, the condition is satisfied if one of them matches the request URL. The path pattern is compared only to the path of the URL, not to its query string. To compare against the query string, use <a>QueryStringConditionConfig</a>.</p>
    pub values: Option<Vec<String>>,
}

struct PathPatternConditionConfigDeserializer;
impl PathPatternConditionConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PathPatternConditionConfig, XmlParseError> {
        deserialize_elements::<_, PathPatternConditionConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Values" => {
                        obj.values
                            .get_or_insert(vec![])
                            .extend(ListOfStringDeserializer::deserialize("Values", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

/// Serialize `PathPatternConditionConfig` contents to a `SignedRequest`.
struct PathPatternConditionConfigSerializer;
impl PathPatternConditionConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PathPatternConditionConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.values {
            ListOfStringSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Values"),
                field_value,
            );
        }
    }
}

struct PortDeserializer;
impl PortDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PrivateIPv4AddressDeserializer;
impl PrivateIPv4AddressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ProtocolEnumDeserializer;
impl ProtocolEnumDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about a query string condition.</p> <p>The query string component of a URI starts after the first '?' character and is terminated by either a '#' character or the end of the URI. A typical query string contains key/value pairs separated by '&amp;' characters. The allowed characters are specified by RFC 3986. Any character can be percentage encoded.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct QueryStringConditionConfig {
    /// <p>One or more key/value pairs or values to find in the query string. The maximum size of each string is 128 characters. The comparison is case insensitive. The following wildcard characters are supported: * (matches 0 or more characters) and ? (matches exactly 1 character). To search for a literal '*' or '?' character in a query string, you must escape these characters in <code>Values</code> using a '\' character.</p> <p>If you specify multiple key/value pairs or values, the condition is satisfied if one of them is found in the query string.</p>
    pub values: Option<Vec<QueryStringKeyValuePair>>,
}

struct QueryStringConditionConfigDeserializer;
impl QueryStringConditionConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueryStringConditionConfig, XmlParseError> {
        deserialize_elements::<_, QueryStringConditionConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Values" => {
                        obj.values.get_or_insert(vec![]).extend(
                            QueryStringKeyValuePairListDeserializer::deserialize("Values", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

/// Serialize `QueryStringConditionConfig` contents to a `SignedRequest`.
struct QueryStringConditionConfigSerializer;
impl QueryStringConditionConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &QueryStringConditionConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.values {
            QueryStringKeyValuePairListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Values"),
                field_value,
            );
        }
    }
}

/// <p>Information about a key/value pair.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct QueryStringKeyValuePair {
    /// <p>The key. You can omit the key.</p>
    pub key: Option<String>,
    /// <p>The value.</p>
    pub value: Option<String>,
}

struct QueryStringKeyValuePairDeserializer;
impl QueryStringKeyValuePairDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueryStringKeyValuePair, XmlParseError> {
        deserialize_elements::<_, QueryStringKeyValuePair, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Key" => {
                        obj.key = Some(StringValueDeserializer::deserialize("Key", stack)?);
                    }
                    "Value" => {
                        obj.value = Some(StringValueDeserializer::deserialize("Value", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

/// Serialize `QueryStringKeyValuePair` contents to a `SignedRequest`.
struct QueryStringKeyValuePairSerializer;
impl QueryStringKeyValuePairSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &QueryStringKeyValuePair) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.key {
            params.put(&format!("{}{}", prefix, "Key"), &field_value);
        }
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }
    }
}

struct QueryStringKeyValuePairListDeserializer;
impl QueryStringKeyValuePairListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<QueryStringKeyValuePair>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(QueryStringKeyValuePairDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `QueryStringKeyValuePairList` contents to a `SignedRequest`.
struct QueryStringKeyValuePairListSerializer;
impl QueryStringKeyValuePairListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<QueryStringKeyValuePair>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            QueryStringKeyValuePairSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>Information about a redirect action.</p> <p>A URI consists of the following components: protocol://hostname:port/path?query. You must modify at least one of the following components to avoid a redirect loop: protocol, hostname, port, or path. Any components that you do not modify retain their original values.</p> <p>You can reuse URI components using the following reserved keywords:</p> <ul> <li> <p>#{protocol}</p> </li> <li> <p>#{host}</p> </li> <li> <p>#{port}</p> </li> <li> <p>#{path} (the leading "/" is removed)</p> </li> <li> <p>#{query}</p> </li> </ul> <p>For example, you can change the path to "/new/#{path}", the hostname to "example.#{host}", or the query to "#{query}&amp;value=xyz".</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RedirectActionConfig {
    /// <p>The hostname. This component is not percent-encoded. The hostname can contain #{host}.</p>
    pub host: Option<String>,
    /// <p>The absolute path, starting with the leading "/". This component is not percent-encoded. The path can contain #{host}, #{path}, and #{port}.</p>
    pub path: Option<String>,
    /// <p>The port. You can specify a value from 1 to 65535 or #{port}.</p>
    pub port: Option<String>,
    /// <p>The protocol. You can specify HTTP, HTTPS, or #{protocol}. You can redirect HTTP to HTTP, HTTP to HTTPS, and HTTPS to HTTPS. You cannot redirect HTTPS to HTTP.</p>
    pub protocol: Option<String>,
    /// <p>The query parameters, URL-encoded when necessary, but not percent-encoded. Do not include the leading "?", as it is automatically added. You can specify any of the reserved keywords.</p>
    pub query: Option<String>,
    /// <p>The HTTP redirect code. The redirect is either permanent (HTTP 301) or temporary (HTTP 302).</p>
    pub status_code: String,
}

struct RedirectActionConfigDeserializer;
impl RedirectActionConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RedirectActionConfig, XmlParseError> {
        deserialize_elements::<_, RedirectActionConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Host" => {
                    obj.host = Some(RedirectActionHostDeserializer::deserialize("Host", stack)?);
                }
                "Path" => {
                    obj.path = Some(RedirectActionPathDeserializer::deserialize("Path", stack)?);
                }
                "Port" => {
                    obj.port = Some(RedirectActionPortDeserializer::deserialize("Port", stack)?);
                }
                "Protocol" => {
                    obj.protocol = Some(RedirectActionProtocolDeserializer::deserialize(
                        "Protocol", stack,
                    )?);
                }
                "Query" => {
                    obj.query = Some(RedirectActionQueryDeserializer::deserialize(
                        "Query", stack,
                    )?);
                }
                "StatusCode" => {
                    obj.status_code =
                        RedirectActionStatusCodeEnumDeserializer::deserialize("StatusCode", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `RedirectActionConfig` contents to a `SignedRequest`.
struct RedirectActionConfigSerializer;
impl RedirectActionConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RedirectActionConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.host {
            params.put(&format!("{}{}", prefix, "Host"), &field_value);
        }
        if let Some(ref field_value) = obj.path {
            params.put(&format!("{}{}", prefix, "Path"), &field_value);
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value);
        }
        if let Some(ref field_value) = obj.protocol {
            params.put(&format!("{}{}", prefix, "Protocol"), &field_value);
        }
        if let Some(ref field_value) = obj.query {
            params.put(&format!("{}{}", prefix, "Query"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StatusCode"), &obj.status_code);
    }
}

struct RedirectActionHostDeserializer;
impl RedirectActionHostDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct RedirectActionPathDeserializer;
impl RedirectActionPathDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct RedirectActionPortDeserializer;
impl RedirectActionPortDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct RedirectActionProtocolDeserializer;
impl RedirectActionProtocolDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct RedirectActionQueryDeserializer;
impl RedirectActionQueryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct RedirectActionStatusCodeEnumDeserializer;
impl RedirectActionStatusCodeEnumDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterTargetsInput {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: String,
    /// <p>The targets.</p> <p>To register a target by instance ID, specify the instance ID. To register a target by IP address, specify the IP address. To register a Lambda function, specify the ARN of the Lambda function.</p>
    pub targets: Vec<TargetDescription>,
}

/// Serialize `RegisterTargetsInput` contents to a `SignedRequest`.
struct RegisterTargetsInputSerializer;
impl RegisterTargetsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RegisterTargetsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "TargetGroupArn"),
            &obj.target_group_arn,
        );
        TargetDescriptionsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Targets"),
            &obj.targets,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RegisterTargetsOutput {}

struct RegisterTargetsOutputDeserializer;
impl RegisterTargetsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RegisterTargetsOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = RegisterTargetsOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveListenerCertificatesInput {
    /// <p>The certificate to remove. You can specify one certificate per call. Set <code>CertificateArn</code> to the certificate ARN but do not set <code>IsDefault</code>.</p>
    pub certificates: Vec<Certificate>,
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub listener_arn: String,
}

/// Serialize `RemoveListenerCertificatesInput` contents to a `SignedRequest`.
struct RemoveListenerCertificatesInputSerializer;
impl RemoveListenerCertificatesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RemoveListenerCertificatesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        CertificateListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Certificates"),
            &obj.certificates,
        );
        params.put(&format!("{}{}", prefix, "ListenerArn"), &obj.listener_arn);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RemoveListenerCertificatesOutput {}

struct RemoveListenerCertificatesOutputDeserializer;
impl RemoveListenerCertificatesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RemoveListenerCertificatesOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = RemoveListenerCertificatesOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsInput {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub resource_arns: Vec<String>,
    /// <p>The tag keys for the tags to remove.</p>
    pub tag_keys: Vec<String>,
}

/// Serialize `RemoveTagsInput` contents to a `SignedRequest`.
struct RemoveTagsInputSerializer;
impl RemoveTagsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RemoveTagsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ResourceArnsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ResourceArns"),
            &obj.resource_arns,
        );
        TagKeysSerializer::serialize(params, &format!("{}{}", prefix, "TagKeys"), &obj.tag_keys);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RemoveTagsOutput {}

struct RemoveTagsOutputDeserializer;
impl RemoveTagsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RemoveTagsOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = RemoveTagsOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ResourceArnDeserializer;
impl ResourceArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `ResourceArns` contents to a `SignedRequest`.
struct ResourceArnsSerializer;
impl ResourceArnsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Rule {
    /// <p>The actions. Each rule must include exactly one of the following types of actions: <code>forward</code>, <code>redirect</code>, or <code>fixed-response</code>, and it must be the last action to be performed.</p>
    pub actions: Option<Vec<Action>>,
    /// <p>The conditions. Each rule can include zero or one of the following conditions: <code>http-request-method</code>, <code>host-header</code>, <code>path-pattern</code>, and <code>source-ip</code>, and zero or more of the following conditions: <code>http-header</code> and <code>query-string</code>.</p>
    pub conditions: Option<Vec<RuleCondition>>,
    /// <p>Indicates whether this is the default rule.</p>
    pub is_default: Option<bool>,
    /// <p>The priority.</p>
    pub priority: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    pub rule_arn: Option<String>,
}

struct RuleDeserializer;
impl RuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Rule, XmlParseError> {
        deserialize_elements::<_, Rule, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Actions" => {
                    obj.actions
                        .get_or_insert(vec![])
                        .extend(ActionsDeserializer::deserialize("Actions", stack)?);
                }
                "Conditions" => {
                    obj.conditions.get_or_insert(vec![]).extend(
                        RuleConditionListDeserializer::deserialize("Conditions", stack)?,
                    );
                }
                "IsDefault" => {
                    obj.is_default = Some(IsDefaultDeserializer::deserialize("IsDefault", stack)?);
                }
                "Priority" => {
                    obj.priority = Some(StringDeserializer::deserialize("Priority", stack)?);
                }
                "RuleArn" => {
                    obj.rule_arn = Some(RuleArnDeserializer::deserialize("RuleArn", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct RuleArnDeserializer;
impl RuleArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `RuleArns` contents to a `SignedRequest`.
struct RuleArnsSerializer;
impl RuleArnsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a condition for a rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RuleCondition {
    /// <p><p>The field in the HTTP request. The following are the possible values:</p> <ul> <li> <p> <code>http-header</code> </p> </li> <li> <p> <code>http-request-method</code> </p> </li> <li> <p> <code>host-header</code> </p> </li> <li> <p> <code>path-pattern</code> </p> </li> <li> <p> <code>query-string</code> </p> </li> <li> <p> <code>source-ip</code> </p> </li> </ul></p>
    pub field: Option<String>,
    /// <p>Information for a host header condition. Specify only when <code>Field</code> is <code>host-header</code>.</p>
    pub host_header_config: Option<HostHeaderConditionConfig>,
    /// <p>Information for an HTTP header condition. Specify only when <code>Field</code> is <code>http-header</code>.</p>
    pub http_header_config: Option<HttpHeaderConditionConfig>,
    /// <p>Information for an HTTP method condition. Specify only when <code>Field</code> is <code>http-request-method</code>.</p>
    pub http_request_method_config: Option<HttpRequestMethodConditionConfig>,
    /// <p>Information for a path pattern condition. Specify only when <code>Field</code> is <code>path-pattern</code>.</p>
    pub path_pattern_config: Option<PathPatternConditionConfig>,
    /// <p>Information for a query string condition. Specify only when <code>Field</code> is <code>query-string</code>.</p>
    pub query_string_config: Option<QueryStringConditionConfig>,
    /// <p>Information for a source IP condition. Specify only when <code>Field</code> is <code>source-ip</code>.</p>
    pub source_ip_config: Option<SourceIpConditionConfig>,
    /// <p><p>The condition value. You can use <code>Values</code> if the rule contains only <code>host-header</code> and <code>path-pattern</code> conditions. Otherwise, you can use <code>HostHeaderConfig</code> for <code>host-header</code> conditions and <code>PathPatternConfig</code> for <code>path-pattern</code> conditions.</p> <p>If <code>Field</code> is <code>host-header</code>, you can specify a single host name (for example, my.example.com). A host name is case insensitive, can be up to 128 characters in length, and can contain any of the following characters.</p> <ul> <li> <p>A-Z, a-z, 0-9</p> </li> <li> <p>- .</p> </li> <li> <p>* (matches 0 or more characters)</p> </li> <li> <p>? (matches exactly 1 character)</p> </li> </ul> <p>If <code>Field</code> is <code>path-pattern</code>, you can specify a single path pattern (for example, /img/<em>). A path pattern is case-sensitive, can be up to 128 characters in length, and can contain any of the following characters.</p> <ul> <li> <p>A-Z, a-z, 0-9</p> </li> <li> <p>_ - . $ / ~ &quot; &#39; @ : +</p> </li> <li> <p>&amp; (using &amp;amp;)</p> </li> <li> <p></em> (matches 0 or more characters)</p> </li> <li> <p>? (matches exactly 1 character)</p> </li> </ul></p>
    pub values: Option<Vec<String>>,
}

struct RuleConditionDeserializer;
impl RuleConditionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RuleCondition, XmlParseError> {
        deserialize_elements::<_, RuleCondition, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Field" => {
                    obj.field = Some(ConditionFieldNameDeserializer::deserialize("Field", stack)?);
                }
                "HostHeaderConfig" => {
                    obj.host_header_config =
                        Some(HostHeaderConditionConfigDeserializer::deserialize(
                            "HostHeaderConfig",
                            stack,
                        )?);
                }
                "HttpHeaderConfig" => {
                    obj.http_header_config =
                        Some(HttpHeaderConditionConfigDeserializer::deserialize(
                            "HttpHeaderConfig",
                            stack,
                        )?);
                }
                "HttpRequestMethodConfig" => {
                    obj.http_request_method_config =
                        Some(HttpRequestMethodConditionConfigDeserializer::deserialize(
                            "HttpRequestMethodConfig",
                            stack,
                        )?);
                }
                "PathPatternConfig" => {
                    obj.path_pattern_config =
                        Some(PathPatternConditionConfigDeserializer::deserialize(
                            "PathPatternConfig",
                            stack,
                        )?);
                }
                "QueryStringConfig" => {
                    obj.query_string_config =
                        Some(QueryStringConditionConfigDeserializer::deserialize(
                            "QueryStringConfig",
                            stack,
                        )?);
                }
                "SourceIpConfig" => {
                    obj.source_ip_config = Some(SourceIpConditionConfigDeserializer::deserialize(
                        "SourceIpConfig",
                        stack,
                    )?);
                }
                "Values" => {
                    obj.values
                        .get_or_insert(vec![])
                        .extend(ListOfStringDeserializer::deserialize("Values", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `RuleCondition` contents to a `SignedRequest`.
struct RuleConditionSerializer;
impl RuleConditionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RuleCondition) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.field {
            params.put(&format!("{}{}", prefix, "Field"), &field_value);
        }
        if let Some(ref field_value) = obj.host_header_config {
            HostHeaderConditionConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "HostHeaderConfig"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.http_header_config {
            HttpHeaderConditionConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "HttpHeaderConfig"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.http_request_method_config {
            HttpRequestMethodConditionConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "HttpRequestMethodConfig"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.path_pattern_config {
            PathPatternConditionConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "PathPatternConfig"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.query_string_config {
            QueryStringConditionConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "QueryStringConfig"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.source_ip_config {
            SourceIpConditionConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SourceIpConfig"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.values {
            ListOfStringSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Values"),
                field_value,
            );
        }
    }
}

struct RuleConditionListDeserializer;
impl RuleConditionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<RuleCondition>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(RuleConditionDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `RuleConditionList` contents to a `SignedRequest`.
struct RuleConditionListSerializer;
impl RuleConditionListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<RuleCondition>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            RuleConditionSerializer::serialize(params, &key, obj);
        }
    }
}

/// Serialize `RulePriorityList` contents to a `SignedRequest`.
struct RulePriorityListSerializer;
impl RulePriorityListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<RulePriorityPair>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            RulePriorityPairSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>Information about the priorities for the rules for a listener.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RulePriorityPair {
    /// <p>The rule priority.</p>
    pub priority: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    pub rule_arn: Option<String>,
}

/// Serialize `RulePriorityPair` contents to a `SignedRequest`.
struct RulePriorityPairSerializer;
impl RulePriorityPairSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RulePriorityPair) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.priority {
            params.put(&format!("{}{}", prefix, "Priority"), &field_value);
        }
        if let Some(ref field_value) = obj.rule_arn {
            params.put(&format!("{}{}", prefix, "RuleArn"), &field_value);
        }
    }
}

struct RulesDeserializer;
impl RulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Rule>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(RuleDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct SecurityGroupIdDeserializer;
impl SecurityGroupIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct SecurityGroupsDeserializer;
impl SecurityGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(SecurityGroupIdDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `SecurityGroups` contents to a `SignedRequest`.
struct SecurityGroupsSerializer;
impl SecurityGroupsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetIpAddressTypeInput {
    /// <p>The IP address type. The possible values are <code>ipv4</code> (for IPv4 addresses) and <code>dualstack</code> (for IPv4 and IPv6 addresses). Internal load balancers must use <code>ipv4</code>. Network Load Balancers must use <code>ipv4</code>.</p>
    pub ip_address_type: String,
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
}

/// Serialize `SetIpAddressTypeInput` contents to a `SignedRequest`.
struct SetIpAddressTypeInputSerializer;
impl SetIpAddressTypeInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetIpAddressTypeInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "IpAddressType"),
            &obj.ip_address_type,
        );
        params.put(
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetIpAddressTypeOutput {
    /// <p>The IP address type.</p>
    pub ip_address_type: Option<String>,
}

struct SetIpAddressTypeOutputDeserializer;
impl SetIpAddressTypeOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetIpAddressTypeOutput, XmlParseError> {
        deserialize_elements::<_, SetIpAddressTypeOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "IpAddressType" => {
                    obj.ip_address_type = Some(IpAddressTypeDeserializer::deserialize(
                        "IpAddressType",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetRulePrioritiesInput {
    /// <p>The rule priorities.</p>
    pub rule_priorities: Vec<RulePriorityPair>,
}

/// Serialize `SetRulePrioritiesInput` contents to a `SignedRequest`.
struct SetRulePrioritiesInputSerializer;
impl SetRulePrioritiesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetRulePrioritiesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        RulePriorityListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "RulePriorities"),
            &obj.rule_priorities,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetRulePrioritiesOutput {
    /// <p>Information about the rules.</p>
    pub rules: Option<Vec<Rule>>,
}

struct SetRulePrioritiesOutputDeserializer;
impl SetRulePrioritiesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetRulePrioritiesOutput, XmlParseError> {
        deserialize_elements::<_, SetRulePrioritiesOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Rules" => {
                        obj.rules
                            .get_or_insert(vec![])
                            .extend(RulesDeserializer::deserialize("Rules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetSecurityGroupsInput {
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
    /// <p>The IDs of the security groups.</p>
    pub security_groups: Vec<String>,
}

/// Serialize `SetSecurityGroupsInput` contents to a `SignedRequest`.
struct SetSecurityGroupsInputSerializer;
impl SetSecurityGroupsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetSecurityGroupsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn,
        );
        SecurityGroupsSerializer::serialize(
            params,
            &format!("{}{}", prefix, "SecurityGroups"),
            &obj.security_groups,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetSecurityGroupsOutput {
    /// <p>The IDs of the security groups associated with the load balancer.</p>
    pub security_group_ids: Option<Vec<String>>,
}

struct SetSecurityGroupsOutputDeserializer;
impl SetSecurityGroupsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetSecurityGroupsOutput, XmlParseError> {
        deserialize_elements::<_, SetSecurityGroupsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "SecurityGroupIds" => {
                        obj.security_group_ids.get_or_insert(vec![]).extend(
                            SecurityGroupsDeserializer::deserialize("SecurityGroupIds", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetSubnetsInput {
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub load_balancer_arn: String,
    /// <p>The IDs of the public subnets. You can specify only one subnet per Availability Zone. You must specify either subnets or subnet mappings.</p> <p>[Application Load Balancers] You must specify subnets from at least two Availability Zones. You cannot specify Elastic IP addresses for your subnets.</p> <p>[Network Load Balancers] You can specify subnets from one or more Availability Zones. If you need static IP addresses for your internet-facing load balancer, you can specify one Elastic IP address per subnet. For internal load balancers, you can specify one private IP address per subnet from the IPv4 range of the subnet.</p>
    pub subnet_mappings: Option<Vec<SubnetMapping>>,
    /// <p>The IDs of the public subnets. You must specify subnets from at least two Availability Zones. You can specify only one subnet per Availability Zone. You must specify either subnets or subnet mappings.</p>
    pub subnets: Option<Vec<String>>,
}

/// Serialize `SetSubnetsInput` contents to a `SignedRequest`.
struct SetSubnetsInputSerializer;
impl SetSubnetsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetSubnetsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LoadBalancerArn"),
            &obj.load_balancer_arn,
        );
        if let Some(ref field_value) = obj.subnet_mappings {
            SubnetMappingsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SubnetMappings"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.subnets {
            SubnetsSerializer::serialize(params, &format!("{}{}", prefix, "Subnets"), field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SetSubnetsOutput {
    /// <p>Information about the subnet and Availability Zone.</p>
    pub availability_zones: Option<Vec<AvailabilityZone>>,
}

struct SetSubnetsOutputDeserializer;
impl SetSubnetsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetSubnetsOutput, XmlParseError> {
        deserialize_elements::<_, SetSubnetsOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AvailabilityZones" => {
                    obj.availability_zones.get_or_insert(vec![]).extend(
                        AvailabilityZonesDeserializer::deserialize("AvailabilityZones", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Information about a source IP condition.</p> <p>You can use this condition to route based on the IP address of the source that connects to the load balancer. If a client is behind a proxy, this is the IP address of the proxy not the IP address of the client.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SourceIpConditionConfig {
    /// <p>One or more source IP addresses, in CIDR format. You can use both IPv4 and IPv6 addresses. Wildcards are not supported.</p> <p>If you specify multiple addresses, the condition is satisfied if the source IP address of the request matches one of the CIDR blocks. This condition is not satisfied by the addresses in the X-Forwarded-For header. To search for addresses in the X-Forwarded-For header, use <a>HttpHeaderConditionConfig</a>.</p>
    pub values: Option<Vec<String>>,
}

struct SourceIpConditionConfigDeserializer;
impl SourceIpConditionConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SourceIpConditionConfig, XmlParseError> {
        deserialize_elements::<_, SourceIpConditionConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Values" => {
                        obj.values
                            .get_or_insert(vec![])
                            .extend(ListOfStringDeserializer::deserialize("Values", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

/// Serialize `SourceIpConditionConfig` contents to a `SignedRequest`.
struct SourceIpConditionConfigSerializer;
impl SourceIpConditionConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SourceIpConditionConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.values {
            ListOfStringSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Values"),
                field_value,
            );
        }
    }
}

struct SslPoliciesDeserializer;
impl SslPoliciesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SslPolicy>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(SslPolicyDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Information about a policy used for SSL negotiation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SslPolicy {
    /// <p>The ciphers.</p>
    pub ciphers: Option<Vec<Cipher>>,
    /// <p>The name of the policy.</p>
    pub name: Option<String>,
    /// <p>The protocols.</p>
    pub ssl_protocols: Option<Vec<String>>,
}

struct SslPolicyDeserializer;
impl SslPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SslPolicy, XmlParseError> {
        deserialize_elements::<_, SslPolicy, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Ciphers" => {
                    obj.ciphers
                        .get_or_insert(vec![])
                        .extend(CiphersDeserializer::deserialize("Ciphers", stack)?);
                }
                "Name" => {
                    obj.name = Some(SslPolicyNameDeserializer::deserialize("Name", stack)?);
                }
                "SslProtocols" => {
                    obj.ssl_protocols.get_or_insert(vec![]).extend(
                        SslProtocolsDeserializer::deserialize("SslProtocols", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct SslPolicyNameDeserializer;
impl SslPolicyNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `SslPolicyNames` contents to a `SignedRequest`.
struct SslPolicyNamesSerializer;
impl SslPolicyNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct SslProtocolDeserializer;
impl SslProtocolDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct SslProtocolsDeserializer;
impl SslProtocolsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(SslProtocolDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct StateReasonDeserializer;
impl StateReasonDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StringDeserializer;
impl StringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StringValueDeserializer;
impl StringValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct SubnetIdDeserializer;
impl SubnetIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about a subnet mapping.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SubnetMapping {
    /// <p>[Network Load Balancers] The allocation ID of the Elastic IP address for an internet-facing load balancer.</p>
    pub allocation_id: Option<String>,
    /// <p>[Network Load Balancers] The private IPv4 address for an internal load balancer.</p>
    pub private_i_pv_4_address: Option<String>,
    /// <p>The ID of the subnet.</p>
    pub subnet_id: Option<String>,
}

/// Serialize `SubnetMapping` contents to a `SignedRequest`.
struct SubnetMappingSerializer;
impl SubnetMappingSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SubnetMapping) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.allocation_id {
            params.put(&format!("{}{}", prefix, "AllocationId"), &field_value);
        }
        if let Some(ref field_value) = obj.private_i_pv_4_address {
            params.put(&format!("{}{}", prefix, "PrivateIPv4Address"), &field_value);
        }
        if let Some(ref field_value) = obj.subnet_id {
            params.put(&format!("{}{}", prefix, "SubnetId"), &field_value);
        }
    }
}

/// Serialize `SubnetMappings` contents to a `SignedRequest`.
struct SubnetMappingsSerializer;
impl SubnetMappingsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<SubnetMapping>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            SubnetMappingSerializer::serialize(params, &key, obj);
        }
    }
}

/// Serialize `Subnets` contents to a `SignedRequest`.
struct SubnetsSerializer;
impl SubnetsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a tag.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tag {
    /// <p>The key of the tag.</p>
    pub key: String,
    /// <p>The value of the tag.</p>
    pub value: Option<String>,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Tag, XmlParseError> {
        deserialize_elements::<_, Tag, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Key" => {
                    obj.key = TagKeyDeserializer::deserialize("Key", stack)?;
                }
                "Value" => {
                    obj.value = Some(TagValueDeserializer::deserialize("Value", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `Tag` contents to a `SignedRequest`.
struct TagSerializer;
impl TagSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Tag) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Key"), &obj.key);
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }
    }
}

/// <p>The tags associated with a resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TagDescription {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub resource_arn: Option<String>,
    /// <p>Information about the tags.</p>
    pub tags: Option<Vec<Tag>>,
}

struct TagDescriptionDeserializer;
impl TagDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TagDescription, XmlParseError> {
        deserialize_elements::<_, TagDescription, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ResourceArn" => {
                    obj.resource_arn =
                        Some(ResourceArnDeserializer::deserialize("ResourceArn", stack)?);
                }
                "Tags" => {
                    obj.tags
                        .get_or_insert(vec![])
                        .extend(TagListDeserializer::deserialize("Tags", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct TagDescriptionsDeserializer;
impl TagDescriptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TagDescription>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TagDescriptionDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct TagKeyDeserializer;
impl TagKeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `TagKeys` contents to a `SignedRequest`.
struct TagKeysSerializer;
impl TagKeysSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct TagListDeserializer;
impl TagListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TagDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `TagList` contents to a `SignedRequest`.
struct TagListSerializer;
impl TagListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Tag>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TagSerializer::serialize(params, &key, obj);
        }
    }
}

struct TagValueDeserializer;
impl TagValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about a target.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TargetDescription {
    /// <p>An Availability Zone or <code>all</code>. This determines whether the target receives traffic from the load balancer nodes in the specified Availability Zone or from all enabled Availability Zones for the load balancer.</p> <p>This parameter is not supported if the target type of the target group is <code>instance</code>.</p> <p>If the target type is <code>ip</code> and the IP address is in a subnet of the VPC for the target group, the Availability Zone is automatically detected and this parameter is optional. If the IP address is outside the VPC, this parameter is required.</p> <p>With an Application Load Balancer, if the target type is <code>ip</code> and the IP address is outside the VPC for the target group, the only supported value is <code>all</code>.</p> <p>If the target type is <code>lambda</code>, this parameter is optional and the only supported value is <code>all</code>.</p>
    pub availability_zone: Option<String>,
    /// <p>The ID of the target. If the target type of the target group is <code>instance</code>, specify an instance ID. If the target type is <code>ip</code>, specify an IP address. If the target type is <code>lambda</code>, specify the ARN of the Lambda function.</p>
    pub id: String,
    /// <p>The port on which the target is listening. Not used if the target is a Lambda function.</p>
    pub port: Option<i64>,
}

struct TargetDescriptionDeserializer;
impl TargetDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetDescription, XmlParseError> {
        deserialize_elements::<_, TargetDescription, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AvailabilityZone" => {
                    obj.availability_zone = Some(ZoneNameDeserializer::deserialize(
                        "AvailabilityZone",
                        stack,
                    )?);
                }
                "Id" => {
                    obj.id = TargetIdDeserializer::deserialize("Id", stack)?;
                }
                "Port" => {
                    obj.port = Some(PortDeserializer::deserialize("Port", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `TargetDescription` contents to a `SignedRequest`.
struct TargetDescriptionSerializer;
impl TargetDescriptionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TargetDescription) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.availability_zone {
            params.put(&format!("{}{}", prefix, "AvailabilityZone"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Id"), &obj.id);
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value);
        }
    }
}

/// Serialize `TargetDescriptions` contents to a `SignedRequest`.
struct TargetDescriptionsSerializer;
impl TargetDescriptionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<TargetDescription>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TargetDescriptionSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>Information about a target group.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TargetGroup {
    /// <p>Indicates whether health checks are enabled.</p>
    pub health_check_enabled: Option<bool>,
    /// <p>The approximate amount of time, in seconds, between health checks of an individual target.</p>
    pub health_check_interval_seconds: Option<i64>,
    /// <p>The destination for the health check request.</p>
    pub health_check_path: Option<String>,
    /// <p>The port to use to connect with the target.</p>
    pub health_check_port: Option<String>,
    /// <p>The protocol to use to connect with the target.</p>
    pub health_check_protocol: Option<String>,
    /// <p>The amount of time, in seconds, during which no response means a failed health check.</p>
    pub health_check_timeout_seconds: Option<i64>,
    /// <p>The number of consecutive health checks successes required before considering an unhealthy target healthy.</p>
    pub healthy_threshold_count: Option<i64>,
    /// <p>The Amazon Resource Names (ARN) of the load balancers that route traffic to this target group.</p>
    pub load_balancer_arns: Option<Vec<String>>,
    /// <p>The HTTP codes to use when checking for a successful response from a target.</p>
    pub matcher: Option<Matcher>,
    /// <p>The port on which the targets are listening. Not used if the target is a Lambda function.</p>
    pub port: Option<i64>,
    /// <p>The protocol to use for routing traffic to the targets.</p>
    pub protocol: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: Option<String>,
    /// <p>The name of the target group.</p>
    pub target_group_name: Option<String>,
    /// <p>The type of target that you must specify when registering targets with this target group. The possible values are <code>instance</code> (targets are specified by instance ID) or <code>ip</code> (targets are specified by IP address).</p>
    pub target_type: Option<String>,
    /// <p>The number of consecutive health check failures required before considering the target unhealthy.</p>
    pub unhealthy_threshold_count: Option<i64>,
    /// <p>The ID of the VPC for the targets.</p>
    pub vpc_id: Option<String>,
}

struct TargetGroupDeserializer;
impl TargetGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetGroup, XmlParseError> {
        deserialize_elements::<_, TargetGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "HealthCheckEnabled" => {
                    obj.health_check_enabled = Some(HealthCheckEnabledDeserializer::deserialize(
                        "HealthCheckEnabled",
                        stack,
                    )?);
                }
                "HealthCheckIntervalSeconds" => {
                    obj.health_check_interval_seconds =
                        Some(HealthCheckIntervalSecondsDeserializer::deserialize(
                            "HealthCheckIntervalSeconds",
                            stack,
                        )?);
                }
                "HealthCheckPath" => {
                    obj.health_check_path =
                        Some(PathDeserializer::deserialize("HealthCheckPath", stack)?);
                }
                "HealthCheckPort" => {
                    obj.health_check_port = Some(HealthCheckPortDeserializer::deserialize(
                        "HealthCheckPort",
                        stack,
                    )?);
                }
                "HealthCheckProtocol" => {
                    obj.health_check_protocol = Some(ProtocolEnumDeserializer::deserialize(
                        "HealthCheckProtocol",
                        stack,
                    )?);
                }
                "HealthCheckTimeoutSeconds" => {
                    obj.health_check_timeout_seconds =
                        Some(HealthCheckTimeoutSecondsDeserializer::deserialize(
                            "HealthCheckTimeoutSeconds",
                            stack,
                        )?);
                }
                "HealthyThresholdCount" => {
                    obj.healthy_threshold_count =
                        Some(HealthCheckThresholdCountDeserializer::deserialize(
                            "HealthyThresholdCount",
                            stack,
                        )?);
                }
                "LoadBalancerArns" => {
                    obj.load_balancer_arns.get_or_insert(vec![]).extend(
                        LoadBalancerArnsDeserializer::deserialize("LoadBalancerArns", stack)?,
                    );
                }
                "Matcher" => {
                    obj.matcher = Some(MatcherDeserializer::deserialize("Matcher", stack)?);
                }
                "Port" => {
                    obj.port = Some(PortDeserializer::deserialize("Port", stack)?);
                }
                "Protocol" => {
                    obj.protocol = Some(ProtocolEnumDeserializer::deserialize("Protocol", stack)?);
                }
                "TargetGroupArn" => {
                    obj.target_group_arn = Some(TargetGroupArnDeserializer::deserialize(
                        "TargetGroupArn",
                        stack,
                    )?);
                }
                "TargetGroupName" => {
                    obj.target_group_name = Some(TargetGroupNameDeserializer::deserialize(
                        "TargetGroupName",
                        stack,
                    )?);
                }
                "TargetType" => {
                    obj.target_type = Some(TargetTypeEnumDeserializer::deserialize(
                        "TargetType",
                        stack,
                    )?);
                }
                "UnhealthyThresholdCount" => {
                    obj.unhealthy_threshold_count =
                        Some(HealthCheckThresholdCountDeserializer::deserialize(
                            "UnhealthyThresholdCount",
                            stack,
                        )?);
                }
                "VpcId" => {
                    obj.vpc_id = Some(VpcIdDeserializer::deserialize("VpcId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct TargetGroupArnDeserializer;
impl TargetGroupArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `TargetGroupArns` contents to a `SignedRequest`.
struct TargetGroupArnsSerializer;
impl TargetGroupArnsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about a target group attribute.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TargetGroupAttribute {
    /// <p><p>The name of the attribute.</p> <p>The following attribute is supported by both Application Load Balancers and Network Load Balancers:</p> <ul> <li> <p> <code>deregistration<em>delay.timeout</em>seconds</code> - The amount of time, in seconds, for Elastic Load Balancing to wait before changing the state of a deregistering target from <code>draining</code> to <code>unused</code>. The range is 0-3600 seconds. The default value is 300 seconds. If the target is a Lambda function, this attribute is not supported.</p> </li> </ul> <p>The following attributes are supported by Application Load Balancers if the target is not a Lambda function:</p> <ul> <li> <p> <code>load<em>balancing.algorithm.type</code> - The load balancing algorithm determines how the load balancer selects targets when routing requests. The value is <code>round</em>robin</code> or <code>least<em>outstanding</em>requests</code>. The default is <code>round<em>robin</code>.</p> </li> <li> <p> <code>slow</em>start.duration<em>seconds</code> - The time period, in seconds, during which a newly registered target receives a linearly increasing share of the traffic to the target group. After this time period ends, the target receives its full share of traffic. The range is 30-900 seconds (15 minutes). Slow start mode is disabled by default.</p> </li> <li> <p> <code>stickiness.enabled</code> - Indicates whether sticky sessions are enabled. The value is <code>true</code> or <code>false</code>. The default is <code>false</code>.</p> </li> <li> <p> <code>stickiness.type</code> - The type of sticky sessions. The possible value is <code>lb</em>cookie</code>.</p> </li> <li> <p> <code>stickiness.lb<em>cookie.duration</em>seconds</code> - The time period, in seconds, during which requests from a client should be routed to the same target. After this time period expires, the load balancer-generated cookie is considered stale. The range is 1 second to 1 week (604800 seconds). The default value is 1 day (86400 seconds).</p> </li> </ul> <p>The following attribute is supported only if the target is a Lambda function.</p> <ul> <li> <p> <code>lambda.multi<em>value</em>headers.enabled</code> - Indicates whether the request and response headers exchanged between the load balancer and the Lambda function include arrays of values or strings. The value is <code>true</code> or <code>false</code>. The default is <code>false</code>. If the value is <code>false</code> and the request contains a duplicate header field name or query parameter key, the load balancer uses the last value sent by the client.</p> </li> </ul> <p>The following attribute is supported only by Network Load Balancers:</p> <ul> <li> <p> <code>proxy<em>protocol</em>v2.enabled</code> - Indicates whether Proxy Protocol version 2 is enabled. The value is <code>true</code> or <code>false</code>. The default is <code>false</code>.</p> </li> </ul></p>
    pub key: Option<String>,
    /// <p>The value of the attribute.</p>
    pub value: Option<String>,
}

struct TargetGroupAttributeDeserializer;
impl TargetGroupAttributeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetGroupAttribute, XmlParseError> {
        deserialize_elements::<_, TargetGroupAttribute, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Key" => {
                    obj.key = Some(TargetGroupAttributeKeyDeserializer::deserialize(
                        "Key", stack,
                    )?);
                }
                "Value" => {
                    obj.value = Some(TargetGroupAttributeValueDeserializer::deserialize(
                        "Value", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `TargetGroupAttribute` contents to a `SignedRequest`.
struct TargetGroupAttributeSerializer;
impl TargetGroupAttributeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TargetGroupAttribute) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.key {
            params.put(&format!("{}{}", prefix, "Key"), &field_value);
        }
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }
    }
}

struct TargetGroupAttributeKeyDeserializer;
impl TargetGroupAttributeKeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TargetGroupAttributeValueDeserializer;
impl TargetGroupAttributeValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TargetGroupAttributesDeserializer;
impl TargetGroupAttributesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TargetGroupAttribute>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TargetGroupAttributeDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `TargetGroupAttributes` contents to a `SignedRequest`.
struct TargetGroupAttributesSerializer;
impl TargetGroupAttributesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<TargetGroupAttribute>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TargetGroupAttributeSerializer::serialize(params, &key, obj);
        }
    }
}

struct TargetGroupListDeserializer;
impl TargetGroupListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TargetGroupTuple>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TargetGroupTupleDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `TargetGroupList` contents to a `SignedRequest`.
struct TargetGroupListSerializer;
impl TargetGroupListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<TargetGroupTuple>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TargetGroupTupleSerializer::serialize(params, &key, obj);
        }
    }
}

struct TargetGroupNameDeserializer;
impl TargetGroupNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `TargetGroupNames` contents to a `SignedRequest`.
struct TargetGroupNamesSerializer;
impl TargetGroupNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Information about the target group stickiness for a rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TargetGroupStickinessConfig {
    /// <p>The time period, in seconds, during which requests from a client should be routed to the same target group. The range is 1-604800 seconds (7 days).</p>
    pub duration_seconds: Option<i64>,
    /// <p>Indicates whether target group stickiness is enabled.</p>
    pub enabled: Option<bool>,
}

struct TargetGroupStickinessConfigDeserializer;
impl TargetGroupStickinessConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetGroupStickinessConfig, XmlParseError> {
        deserialize_elements::<_, TargetGroupStickinessConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DurationSeconds" => {
                        obj.duration_seconds = Some(
                            TargetGroupStickinessDurationSecondsDeserializer::deserialize(
                                "DurationSeconds",
                                stack,
                            )?,
                        );
                    }
                    "Enabled" => {
                        obj.enabled = Some(TargetGroupStickinessEnabledDeserializer::deserialize(
                            "Enabled", stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

/// Serialize `TargetGroupStickinessConfig` contents to a `SignedRequest`.
struct TargetGroupStickinessConfigSerializer;
impl TargetGroupStickinessConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TargetGroupStickinessConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.duration_seconds {
            params.put(&format!("{}{}", prefix, "DurationSeconds"), &field_value);
        }
        if let Some(ref field_value) = obj.enabled {
            params.put(&format!("{}{}", prefix, "Enabled"), &field_value);
        }
    }
}

struct TargetGroupStickinessDurationSecondsDeserializer;
impl TargetGroupStickinessDurationSecondsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TargetGroupStickinessEnabledDeserializer;
impl TargetGroupStickinessEnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about how traffic will be distributed between multiple target groups in a forward rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TargetGroupTuple {
    /// <p>The Amazon Resource Name (ARN) of the target group.</p>
    pub target_group_arn: Option<String>,
    /// <p>The weight. The range is 0 to 999.</p>
    pub weight: Option<i64>,
}

struct TargetGroupTupleDeserializer;
impl TargetGroupTupleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetGroupTuple, XmlParseError> {
        deserialize_elements::<_, TargetGroupTuple, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "TargetGroupArn" => {
                    obj.target_group_arn = Some(TargetGroupArnDeserializer::deserialize(
                        "TargetGroupArn",
                        stack,
                    )?);
                }
                "Weight" => {
                    obj.weight = Some(TargetGroupWeightDeserializer::deserialize("Weight", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `TargetGroupTuple` contents to a `SignedRequest`.
struct TargetGroupTupleSerializer;
impl TargetGroupTupleSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TargetGroupTuple) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.target_group_arn {
            params.put(&format!("{}{}", prefix, "TargetGroupArn"), &field_value);
        }
        if let Some(ref field_value) = obj.weight {
            params.put(&format!("{}{}", prefix, "Weight"), &field_value);
        }
    }
}

struct TargetGroupWeightDeserializer;
impl TargetGroupWeightDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TargetGroupsDeserializer;
impl TargetGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TargetGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TargetGroupDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Information about the current health of a target.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TargetHealth {
    /// <p>A description of the target health that provides additional details. If the state is <code>healthy</code>, a description is not provided.</p>
    pub description: Option<String>,
    /// <p><p>The reason code.</p> <p>If the target state is <code>healthy</code>, a reason code is not provided.</p> <p>If the target state is <code>initial</code>, the reason code can be one of the following values:</p> <ul> <li> <p> <code>Elb.RegistrationInProgress</code> - The target is in the process of being registered with the load balancer.</p> </li> <li> <p> <code>Elb.InitialHealthChecking</code> - The load balancer is still sending the target the minimum number of health checks required to determine its health status.</p> </li> </ul> <p>If the target state is <code>unhealthy</code>, the reason code can be one of the following values:</p> <ul> <li> <p> <code>Target.ResponseCodeMismatch</code> - The health checks did not return an expected HTTP code. Applies only to Application Load Balancers.</p> </li> <li> <p> <code>Target.Timeout</code> - The health check requests timed out. Applies only to Application Load Balancers.</p> </li> <li> <p> <code>Target.FailedHealthChecks</code> - The load balancer received an error while establishing a connection to the target or the target response was malformed.</p> </li> <li> <p> <code>Elb.InternalError</code> - The health checks failed due to an internal error. Applies only to Application Load Balancers.</p> </li> </ul> <p>If the target state is <code>unused</code>, the reason code can be one of the following values:</p> <ul> <li> <p> <code>Target.NotRegistered</code> - The target is not registered with the target group.</p> </li> <li> <p> <code>Target.NotInUse</code> - The target group is not used by any load balancer or the target is in an Availability Zone that is not enabled for its load balancer.</p> </li> <li> <p> <code>Target.InvalidState</code> - The target is in the stopped or terminated state.</p> </li> <li> <p> <code>Target.IpUnusable</code> - The target IP address is reserved for use by a load balancer.</p> </li> </ul> <p>If the target state is <code>draining</code>, the reason code can be the following value:</p> <ul> <li> <p> <code>Target.DeregistrationInProgress</code> - The target is in the process of being deregistered and the deregistration delay period has not expired.</p> </li> </ul> <p>If the target state is <code>unavailable</code>, the reason code can be the following value:</p> <ul> <li> <p> <code>Target.HealthCheckDisabled</code> - Health checks are disabled for the target group. Applies only to Application Load Balancers.</p> </li> <li> <p> <code>Elb.InternalError</code> - Target health is unavailable due to an internal error. Applies only to Network Load Balancers.</p> </li> </ul></p>
    pub reason: Option<String>,
    /// <p>The state of the target.</p>
    pub state: Option<String>,
}

struct TargetHealthDeserializer;
impl TargetHealthDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetHealth, XmlParseError> {
        deserialize_elements::<_, TargetHealth, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Description" => {
                    obj.description =
                        Some(DescriptionDeserializer::deserialize("Description", stack)?);
                }
                "Reason" => {
                    obj.reason = Some(TargetHealthReasonEnumDeserializer::deserialize(
                        "Reason", stack,
                    )?);
                }
                "State" => {
                    obj.state = Some(TargetHealthStateEnumDeserializer::deserialize(
                        "State", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Information about the health of a target.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TargetHealthDescription {
    /// <p>The port to use to connect with the target.</p>
    pub health_check_port: Option<String>,
    /// <p>The description of the target.</p>
    pub target: Option<TargetDescription>,
    /// <p>The health information for the target.</p>
    pub target_health: Option<TargetHealth>,
}

struct TargetHealthDescriptionDeserializer;
impl TargetHealthDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetHealthDescription, XmlParseError> {
        deserialize_elements::<_, TargetHealthDescription, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HealthCheckPort" => {
                        obj.health_check_port = Some(HealthCheckPortDeserializer::deserialize(
                            "HealthCheckPort",
                            stack,
                        )?);
                    }
                    "Target" => {
                        obj.target =
                            Some(TargetDescriptionDeserializer::deserialize("Target", stack)?);
                    }
                    "TargetHealth" => {
                        obj.target_health = Some(TargetHealthDeserializer::deserialize(
                            "TargetHealth",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct TargetHealthDescriptionsDeserializer;
impl TargetHealthDescriptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TargetHealthDescription>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TargetHealthDescriptionDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct TargetHealthReasonEnumDeserializer;
impl TargetHealthReasonEnumDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TargetHealthStateEnumDeserializer;
impl TargetHealthStateEnumDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TargetIdDeserializer;
impl TargetIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TargetTypeEnumDeserializer;
impl TargetTypeEnumDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct VpcIdDeserializer;
impl VpcIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ZoneNameDeserializer;
impl ZoneNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// Errors returned by AddListenerCertificates
#[derive(Debug, PartialEq)]
pub enum AddListenerCertificatesError {
    /// <p>The specified certificate does not exist.</p>
    CertificateNotFound(String),
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>You've reached the limit on the number of certificates per load balancer.</p>
    TooManyCertificates(String),
}

impl AddListenerCertificatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddListenerCertificatesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CertificateNotFound" => {
                        return RusotoError::Service(
                            AddListenerCertificatesError::CertificateNotFound(parsed_error.message),
                        )
                    }
                    "ListenerNotFound" => {
                        return RusotoError::Service(
                            AddListenerCertificatesError::ListenerNotFound(parsed_error.message),
                        )
                    }
                    "TooManyCertificates" => {
                        return RusotoError::Service(
                            AddListenerCertificatesError::TooManyCertificates(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for AddListenerCertificatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddListenerCertificatesError::CertificateNotFound(ref cause) => write!(f, "{}", cause),
            AddListenerCertificatesError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
            AddListenerCertificatesError::TooManyCertificates(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddListenerCertificatesError {}
/// Errors returned by AddTags
#[derive(Debug, PartialEq)]
pub enum AddTagsError {
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of tags per load balancer.</p>
    TooManyTags(String),
}

impl AddTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DuplicateTagKeys" => {
                        return RusotoError::Service(AddTagsError::DuplicateTagKeys(
                            parsed_error.message,
                        ))
                    }
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(AddTagsError::LoadBalancerNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(AddTagsError::TargetGroupNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TooManyTags" => {
                        return RusotoError::Service(AddTagsError::TooManyTags(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for AddTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsError::DuplicateTagKeys(ref cause) => write!(f, "{}", cause),
            AddTagsError::LoadBalancerNotFound(ref cause) => write!(f, "{}", cause),
            AddTagsError::TargetGroupNotFound(ref cause) => write!(f, "{}", cause),
            AddTagsError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsError {}
/// Errors returned by CreateListener
#[derive(Debug, PartialEq)]
pub enum CreateListenerError {
    /// <p>The specified certificate does not exist.</p>
    CertificateNotFound(String),
    /// <p>A listener with the specified port already exists.</p>
    DuplicateListener(String),
    /// <p>The specified configuration is not valid with this protocol.</p>
    IncompatibleProtocols(String),
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The requested action is not valid.</p>
    InvalidLoadBalancerAction(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>The specified SSL policy does not exist.</p>
    SSLPolicyNotFound(String),
    /// <p>You've reached the limit on the number of load balancers per target group.</p>
    TargetGroupAssociationLimit(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of actions per rule.</p>
    TooManyActions(String),
    /// <p>You've reached the limit on the number of certificates per load balancer.</p>
    TooManyCertificates(String),
    /// <p>You've reached the limit on the number of listeners per load balancer.</p>
    TooManyListeners(String),
    /// <p>You've reached the limit on the number of times a target can be registered with a load balancer.</p>
    TooManyRegistrationsForTargetId(String),
    /// <p>You've reached the limit on the number of targets.</p>
    TooManyTargets(String),
    /// <p>You've reached the limit on the number of unique target groups per load balancer across all listeners. If a target group is used by multiple actions for a load balancer, it is counted as only one use.</p>
    TooManyUniqueTargetGroupsPerLoadBalancer(String),
    /// <p>The specified protocol is not supported.</p>
    UnsupportedProtocol(String),
}

impl CreateListenerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateListenerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CertificateNotFound" => {
                        return RusotoError::Service(CreateListenerError::CertificateNotFound(
                            parsed_error.message,
                        ))
                    }
                    "DuplicateListener" => {
                        return RusotoError::Service(CreateListenerError::DuplicateListener(
                            parsed_error.message,
                        ))
                    }
                    "IncompatibleProtocols" => {
                        return RusotoError::Service(CreateListenerError::IncompatibleProtocols(
                            parsed_error.message,
                        ))
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            CreateListenerError::InvalidConfigurationRequest(parsed_error.message),
                        )
                    }
                    "InvalidLoadBalancerAction" => {
                        return RusotoError::Service(
                            CreateListenerError::InvalidLoadBalancerAction(parsed_error.message),
                        )
                    }
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(CreateListenerError::LoadBalancerNotFound(
                            parsed_error.message,
                        ))
                    }
                    "SSLPolicyNotFound" => {
                        return RusotoError::Service(CreateListenerError::SSLPolicyNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TargetGroupAssociationLimit" => {
                        return RusotoError::Service(
                            CreateListenerError::TargetGroupAssociationLimit(parsed_error.message),
                        )
                    }
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(CreateListenerError::TargetGroupNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TooManyActions" => {
                        return RusotoError::Service(CreateListenerError::TooManyActions(
                            parsed_error.message,
                        ))
                    }
                    "TooManyCertificates" => {
                        return RusotoError::Service(CreateListenerError::TooManyCertificates(
                            parsed_error.message,
                        ))
                    }
                    "TooManyListeners" => {
                        return RusotoError::Service(CreateListenerError::TooManyListeners(
                            parsed_error.message,
                        ))
                    }
                    "TooManyRegistrationsForTargetId" => {
                        return RusotoError::Service(
                            CreateListenerError::TooManyRegistrationsForTargetId(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyTargets" => {
                        return RusotoError::Service(CreateListenerError::TooManyTargets(
                            parsed_error.message,
                        ))
                    }
                    "TooManyUniqueTargetGroupsPerLoadBalancer" => {
                        return RusotoError::Service(
                            CreateListenerError::TooManyUniqueTargetGroupsPerLoadBalancer(
                                parsed_error.message,
                            ),
                        )
                    }
                    "UnsupportedProtocol" => {
                        return RusotoError::Service(CreateListenerError::UnsupportedProtocol(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateListenerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateListenerError::CertificateNotFound(ref cause) => write!(f, "{}", cause),
            CreateListenerError::DuplicateListener(ref cause) => write!(f, "{}", cause),
            CreateListenerError::IncompatibleProtocols(ref cause) => write!(f, "{}", cause),
            CreateListenerError::InvalidConfigurationRequest(ref cause) => write!(f, "{}", cause),
            CreateListenerError::InvalidLoadBalancerAction(ref cause) => write!(f, "{}", cause),
            CreateListenerError::LoadBalancerNotFound(ref cause) => write!(f, "{}", cause),
            CreateListenerError::SSLPolicyNotFound(ref cause) => write!(f, "{}", cause),
            CreateListenerError::TargetGroupAssociationLimit(ref cause) => write!(f, "{}", cause),
            CreateListenerError::TargetGroupNotFound(ref cause) => write!(f, "{}", cause),
            CreateListenerError::TooManyActions(ref cause) => write!(f, "{}", cause),
            CreateListenerError::TooManyCertificates(ref cause) => write!(f, "{}", cause),
            CreateListenerError::TooManyListeners(ref cause) => write!(f, "{}", cause),
            CreateListenerError::TooManyRegistrationsForTargetId(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateListenerError::TooManyTargets(ref cause) => write!(f, "{}", cause),
            CreateListenerError::TooManyUniqueTargetGroupsPerLoadBalancer(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateListenerError::UnsupportedProtocol(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateListenerError {}
/// Errors returned by CreateLoadBalancer
#[derive(Debug, PartialEq)]
pub enum CreateLoadBalancerError {
    /// <p>The specified allocation ID does not exist.</p>
    AllocationIdNotFound(String),
    /// <p>The specified Availability Zone is not supported.</p>
    AvailabilityZoneNotSupported(String),
    /// <p>A load balancer with the specified name already exists.</p>
    DuplicateLoadBalancerName(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The requested scheme is not valid.</p>
    InvalidScheme(String),
    /// <p>The specified security group does not exist.</p>
    InvalidSecurityGroup(String),
    /// <p>The specified subnet is out of available addresses.</p>
    InvalidSubnet(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>A specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified subnet does not exist.</p>
    SubnetNotFound(String),
    /// <p>You've reached the limit on the number of load balancers for your AWS account.</p>
    TooManyLoadBalancers(String),
    /// <p>You've reached the limit on the number of tags per load balancer.</p>
    TooManyTags(String),
}

impl CreateLoadBalancerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLoadBalancerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AllocationIdNotFound" => {
                        return RusotoError::Service(CreateLoadBalancerError::AllocationIdNotFound(
                            parsed_error.message,
                        ))
                    }
                    "AvailabilityZoneNotSupported" => {
                        return RusotoError::Service(
                            CreateLoadBalancerError::AvailabilityZoneNotSupported(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DuplicateLoadBalancerName" => {
                        return RusotoError::Service(
                            CreateLoadBalancerError::DuplicateLoadBalancerName(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DuplicateTagKeys" => {
                        return RusotoError::Service(CreateLoadBalancerError::DuplicateTagKeys(
                            parsed_error.message,
                        ))
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            CreateLoadBalancerError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidScheme" => {
                        return RusotoError::Service(CreateLoadBalancerError::InvalidScheme(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSecurityGroup" => {
                        return RusotoError::Service(CreateLoadBalancerError::InvalidSecurityGroup(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(CreateLoadBalancerError::InvalidSubnet(
                            parsed_error.message,
                        ))
                    }
                    "OperationNotPermitted" => {
                        return RusotoError::Service(
                            CreateLoadBalancerError::OperationNotPermitted(parsed_error.message),
                        )
                    }
                    "ResourceInUse" => {
                        return RusotoError::Service(CreateLoadBalancerError::ResourceInUse(
                            parsed_error.message,
                        ))
                    }
                    "SubnetNotFound" => {
                        return RusotoError::Service(CreateLoadBalancerError::SubnetNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TooManyLoadBalancers" => {
                        return RusotoError::Service(CreateLoadBalancerError::TooManyLoadBalancers(
                            parsed_error.message,
                        ))
                    }
                    "TooManyTags" => {
                        return RusotoError::Service(CreateLoadBalancerError::TooManyTags(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLoadBalancerError::AllocationIdNotFound(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::AvailabilityZoneNotSupported(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLoadBalancerError::DuplicateLoadBalancerName(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::DuplicateTagKeys(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLoadBalancerError::InvalidScheme(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::InvalidSecurityGroup(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::SubnetNotFound(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::TooManyLoadBalancers(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLoadBalancerError {}
/// Errors returned by CreateRule
#[derive(Debug, PartialEq)]
pub enum CreateRuleError {
    /// <p>The specified configuration is not valid with this protocol.</p>
    IncompatibleProtocols(String),
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The requested action is not valid.</p>
    InvalidLoadBalancerAction(String),
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>The specified priority is in use.</p>
    PriorityInUse(String),
    /// <p>You've reached the limit on the number of load balancers per target group.</p>
    TargetGroupAssociationLimit(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of actions per rule.</p>
    TooManyActions(String),
    /// <p>You've reached the limit on the number of times a target can be registered with a load balancer.</p>
    TooManyRegistrationsForTargetId(String),
    /// <p>You've reached the limit on the number of rules per load balancer.</p>
    TooManyRules(String),
    /// <p>You've reached the limit on the number of target groups for your AWS account.</p>
    TooManyTargetGroups(String),
    /// <p>You've reached the limit on the number of targets.</p>
    TooManyTargets(String),
    /// <p>You've reached the limit on the number of unique target groups per load balancer across all listeners. If a target group is used by multiple actions for a load balancer, it is counted as only one use.</p>
    TooManyUniqueTargetGroupsPerLoadBalancer(String),
    /// <p>The specified protocol is not supported.</p>
    UnsupportedProtocol(String),
}

impl CreateRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRuleError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "IncompatibleProtocols" => {
                        return RusotoError::Service(CreateRuleError::IncompatibleProtocols(
                            parsed_error.message,
                        ))
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(CreateRuleError::InvalidConfigurationRequest(
                            parsed_error.message,
                        ))
                    }
                    "InvalidLoadBalancerAction" => {
                        return RusotoError::Service(CreateRuleError::InvalidLoadBalancerAction(
                            parsed_error.message,
                        ))
                    }
                    "ListenerNotFound" => {
                        return RusotoError::Service(CreateRuleError::ListenerNotFound(
                            parsed_error.message,
                        ))
                    }
                    "PriorityInUse" => {
                        return RusotoError::Service(CreateRuleError::PriorityInUse(
                            parsed_error.message,
                        ))
                    }
                    "TargetGroupAssociationLimit" => {
                        return RusotoError::Service(CreateRuleError::TargetGroupAssociationLimit(
                            parsed_error.message,
                        ))
                    }
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(CreateRuleError::TargetGroupNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TooManyActions" => {
                        return RusotoError::Service(CreateRuleError::TooManyActions(
                            parsed_error.message,
                        ))
                    }
                    "TooManyRegistrationsForTargetId" => {
                        return RusotoError::Service(
                            CreateRuleError::TooManyRegistrationsForTargetId(parsed_error.message),
                        )
                    }
                    "TooManyRules" => {
                        return RusotoError::Service(CreateRuleError::TooManyRules(
                            parsed_error.message,
                        ))
                    }
                    "TooManyTargetGroups" => {
                        return RusotoError::Service(CreateRuleError::TooManyTargetGroups(
                            parsed_error.message,
                        ))
                    }
                    "TooManyTargets" => {
                        return RusotoError::Service(CreateRuleError::TooManyTargets(
                            parsed_error.message,
                        ))
                    }
                    "TooManyUniqueTargetGroupsPerLoadBalancer" => {
                        return RusotoError::Service(
                            CreateRuleError::TooManyUniqueTargetGroupsPerLoadBalancer(
                                parsed_error.message,
                            ),
                        )
                    }
                    "UnsupportedProtocol" => {
                        return RusotoError::Service(CreateRuleError::UnsupportedProtocol(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRuleError::IncompatibleProtocols(ref cause) => write!(f, "{}", cause),
            CreateRuleError::InvalidConfigurationRequest(ref cause) => write!(f, "{}", cause),
            CreateRuleError::InvalidLoadBalancerAction(ref cause) => write!(f, "{}", cause),
            CreateRuleError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
            CreateRuleError::PriorityInUse(ref cause) => write!(f, "{}", cause),
            CreateRuleError::TargetGroupAssociationLimit(ref cause) => write!(f, "{}", cause),
            CreateRuleError::TargetGroupNotFound(ref cause) => write!(f, "{}", cause),
            CreateRuleError::TooManyActions(ref cause) => write!(f, "{}", cause),
            CreateRuleError::TooManyRegistrationsForTargetId(ref cause) => write!(f, "{}", cause),
            CreateRuleError::TooManyRules(ref cause) => write!(f, "{}", cause),
            CreateRuleError::TooManyTargetGroups(ref cause) => write!(f, "{}", cause),
            CreateRuleError::TooManyTargets(ref cause) => write!(f, "{}", cause),
            CreateRuleError::TooManyUniqueTargetGroupsPerLoadBalancer(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRuleError::UnsupportedProtocol(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRuleError {}
/// Errors returned by CreateTargetGroup
#[derive(Debug, PartialEq)]
pub enum CreateTargetGroupError {
    /// <p>A target group with the specified name already exists.</p>
    DuplicateTargetGroupName(String),
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>You've reached the limit on the number of target groups for your AWS account.</p>
    TooManyTargetGroups(String),
}

impl CreateTargetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTargetGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DuplicateTargetGroupName" => {
                        return RusotoError::Service(
                            CreateTargetGroupError::DuplicateTargetGroupName(parsed_error.message),
                        )
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            CreateTargetGroupError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyTargetGroups" => {
                        return RusotoError::Service(CreateTargetGroupError::TooManyTargetGroups(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateTargetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTargetGroupError::DuplicateTargetGroupName(ref cause) => write!(f, "{}", cause),
            CreateTargetGroupError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateTargetGroupError::TooManyTargetGroups(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTargetGroupError {}
/// Errors returned by DeleteListener
#[derive(Debug, PartialEq)]
pub enum DeleteListenerError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
}

impl DeleteListenerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteListenerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ListenerNotFound" => {
                        return RusotoError::Service(DeleteListenerError::ListenerNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteListenerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteListenerError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteListenerError {}
/// Errors returned by DeleteLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DeleteLoadBalancerError {
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>A specified resource is in use.</p>
    ResourceInUse(String),
}

impl DeleteLoadBalancerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLoadBalancerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(DeleteLoadBalancerError::LoadBalancerNotFound(
                            parsed_error.message,
                        ))
                    }
                    "OperationNotPermitted" => {
                        return RusotoError::Service(
                            DeleteLoadBalancerError::OperationNotPermitted(parsed_error.message),
                        )
                    }
                    "ResourceInUse" => {
                        return RusotoError::Service(DeleteLoadBalancerError::ResourceInUse(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLoadBalancerError::LoadBalancerNotFound(ref cause) => write!(f, "{}", cause),
            DeleteLoadBalancerError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            DeleteLoadBalancerError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLoadBalancerError {}
/// Errors returned by DeleteRule
#[derive(Debug, PartialEq)]
pub enum DeleteRuleError {
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified rule does not exist.</p>
    RuleNotFound(String),
}

impl DeleteRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRuleError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "OperationNotPermitted" => {
                        return RusotoError::Service(DeleteRuleError::OperationNotPermitted(
                            parsed_error.message,
                        ))
                    }
                    "RuleNotFound" => {
                        return RusotoError::Service(DeleteRuleError::RuleNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRuleError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::RuleNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRuleError {}
/// Errors returned by DeleteTargetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteTargetGroupError {
    /// <p>A specified resource is in use.</p>
    ResourceInUse(String),
}

impl DeleteTargetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTargetGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceInUse" => {
                        return RusotoError::Service(DeleteTargetGroupError::ResourceInUse(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteTargetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTargetGroupError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTargetGroupError {}
/// Errors returned by DeregisterTargets
#[derive(Debug, PartialEq)]
pub enum DeregisterTargetsError {
    /// <p>The specified target does not exist, is not in the same VPC as the target group, or has an unsupported instance type.</p>
    InvalidTarget(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
}

impl DeregisterTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterTargetsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidTarget" => {
                        return RusotoError::Service(DeregisterTargetsError::InvalidTarget(
                            parsed_error.message,
                        ))
                    }
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(DeregisterTargetsError::TargetGroupNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeregisterTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterTargetsError::InvalidTarget(ref cause) => write!(f, "{}", cause),
            DeregisterTargetsError::TargetGroupNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterTargetsError {}
/// Errors returned by DescribeAccountLimits
#[derive(Debug, PartialEq)]
pub enum DescribeAccountLimitsError {}

impl DescribeAccountLimitsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccountLimitsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeAccountLimitsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeAccountLimitsError {}
/// Errors returned by DescribeListenerCertificates
#[derive(Debug, PartialEq)]
pub enum DescribeListenerCertificatesError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
}

impl DescribeListenerCertificatesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeListenerCertificatesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ListenerNotFound" => {
                        return RusotoError::Service(
                            DescribeListenerCertificatesError::ListenerNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeListenerCertificatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeListenerCertificatesError::ListenerNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeListenerCertificatesError {}
/// Errors returned by DescribeListeners
#[derive(Debug, PartialEq)]
pub enum DescribeListenersError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>The specified protocol is not supported.</p>
    UnsupportedProtocol(String),
}

impl DescribeListenersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeListenersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ListenerNotFound" => {
                        return RusotoError::Service(DescribeListenersError::ListenerNotFound(
                            parsed_error.message,
                        ))
                    }
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(DescribeListenersError::LoadBalancerNotFound(
                            parsed_error.message,
                        ))
                    }
                    "UnsupportedProtocol" => {
                        return RusotoError::Service(DescribeListenersError::UnsupportedProtocol(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeListenersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeListenersError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
            DescribeListenersError::LoadBalancerNotFound(ref cause) => write!(f, "{}", cause),
            DescribeListenersError::UnsupportedProtocol(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeListenersError {}
/// Errors returned by DescribeLoadBalancerAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancerAttributesError {
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
}

impl DescribeLoadBalancerAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeLoadBalancerAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            DescribeLoadBalancerAttributesError::LoadBalancerNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeLoadBalancerAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLoadBalancerAttributesError::LoadBalancerNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeLoadBalancerAttributesError {}
/// Errors returned by DescribeLoadBalancers
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBalancersError {
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
}

impl DescribeLoadBalancersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLoadBalancersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            DescribeLoadBalancersError::LoadBalancerNotFound(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeLoadBalancersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLoadBalancersError::LoadBalancerNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLoadBalancersError {}
/// Errors returned by DescribeRules
#[derive(Debug, PartialEq)]
pub enum DescribeRulesError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>The specified rule does not exist.</p>
    RuleNotFound(String),
    /// <p>The specified protocol is not supported.</p>
    UnsupportedProtocol(String),
}

impl DescribeRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRulesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ListenerNotFound" => {
                        return RusotoError::Service(DescribeRulesError::ListenerNotFound(
                            parsed_error.message,
                        ))
                    }
                    "RuleNotFound" => {
                        return RusotoError::Service(DescribeRulesError::RuleNotFound(
                            parsed_error.message,
                        ))
                    }
                    "UnsupportedProtocol" => {
                        return RusotoError::Service(DescribeRulesError::UnsupportedProtocol(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRulesError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
            DescribeRulesError::RuleNotFound(ref cause) => write!(f, "{}", cause),
            DescribeRulesError::UnsupportedProtocol(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRulesError {}
/// Errors returned by DescribeSSLPolicies
#[derive(Debug, PartialEq)]
pub enum DescribeSSLPoliciesError {
    /// <p>The specified SSL policy does not exist.</p>
    SSLPolicyNotFound(String),
}

impl DescribeSSLPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSSLPoliciesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "SSLPolicyNotFound" => {
                        return RusotoError::Service(DescribeSSLPoliciesError::SSLPolicyNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeSSLPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSSLPoliciesError::SSLPolicyNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSSLPoliciesError {}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>The specified rule does not exist.</p>
    RuleNotFound(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
}

impl DescribeTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTagsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ListenerNotFound" => {
                        return RusotoError::Service(DescribeTagsError::ListenerNotFound(
                            parsed_error.message,
                        ))
                    }
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(DescribeTagsError::LoadBalancerNotFound(
                            parsed_error.message,
                        ))
                    }
                    "RuleNotFound" => {
                        return RusotoError::Service(DescribeTagsError::RuleNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(DescribeTagsError::TargetGroupNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTagsError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::LoadBalancerNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::RuleNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::TargetGroupNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTagsError {}
/// Errors returned by DescribeTargetGroupAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeTargetGroupAttributesError {
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
}

impl DescribeTargetGroupAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeTargetGroupAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(
                            DescribeTargetGroupAttributesError::TargetGroupNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeTargetGroupAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTargetGroupAttributesError::TargetGroupNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeTargetGroupAttributesError {}
/// Errors returned by DescribeTargetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeTargetGroupsError {
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
}

impl DescribeTargetGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTargetGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            DescribeTargetGroupsError::LoadBalancerNotFound(parsed_error.message),
                        )
                    }
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(
                            DescribeTargetGroupsError::TargetGroupNotFound(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeTargetGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTargetGroupsError::LoadBalancerNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTargetGroupsError::TargetGroupNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTargetGroupsError {}
/// Errors returned by DescribeTargetHealth
#[derive(Debug, PartialEq)]
pub enum DescribeTargetHealthError {
    /// <p>The health of the specified targets could not be retrieved due to an internal error.</p>
    HealthUnavailable(String),
    /// <p>The specified target does not exist, is not in the same VPC as the target group, or has an unsupported instance type.</p>
    InvalidTarget(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
}

impl DescribeTargetHealthError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTargetHealthError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "HealthUnavailable" => {
                        return RusotoError::Service(DescribeTargetHealthError::HealthUnavailable(
                            parsed_error.message,
                        ))
                    }
                    "InvalidTarget" => {
                        return RusotoError::Service(DescribeTargetHealthError::InvalidTarget(
                            parsed_error.message,
                        ))
                    }
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(
                            DescribeTargetHealthError::TargetGroupNotFound(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeTargetHealthError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTargetHealthError::HealthUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeTargetHealthError::InvalidTarget(ref cause) => write!(f, "{}", cause),
            DescribeTargetHealthError::TargetGroupNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTargetHealthError {}
/// Errors returned by ModifyListener
#[derive(Debug, PartialEq)]
pub enum ModifyListenerError {
    /// <p>The specified certificate does not exist.</p>
    CertificateNotFound(String),
    /// <p>A listener with the specified port already exists.</p>
    DuplicateListener(String),
    /// <p>The specified configuration is not valid with this protocol.</p>
    IncompatibleProtocols(String),
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The requested action is not valid.</p>
    InvalidLoadBalancerAction(String),
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>The specified SSL policy does not exist.</p>
    SSLPolicyNotFound(String),
    /// <p>You've reached the limit on the number of load balancers per target group.</p>
    TargetGroupAssociationLimit(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of actions per rule.</p>
    TooManyActions(String),
    /// <p>You've reached the limit on the number of certificates per load balancer.</p>
    TooManyCertificates(String),
    /// <p>You've reached the limit on the number of listeners per load balancer.</p>
    TooManyListeners(String),
    /// <p>You've reached the limit on the number of times a target can be registered with a load balancer.</p>
    TooManyRegistrationsForTargetId(String),
    /// <p>You've reached the limit on the number of targets.</p>
    TooManyTargets(String),
    /// <p>You've reached the limit on the number of unique target groups per load balancer across all listeners. If a target group is used by multiple actions for a load balancer, it is counted as only one use.</p>
    TooManyUniqueTargetGroupsPerLoadBalancer(String),
    /// <p>The specified protocol is not supported.</p>
    UnsupportedProtocol(String),
}

impl ModifyListenerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyListenerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CertificateNotFound" => {
                        return RusotoError::Service(ModifyListenerError::CertificateNotFound(
                            parsed_error.message,
                        ))
                    }
                    "DuplicateListener" => {
                        return RusotoError::Service(ModifyListenerError::DuplicateListener(
                            parsed_error.message,
                        ))
                    }
                    "IncompatibleProtocols" => {
                        return RusotoError::Service(ModifyListenerError::IncompatibleProtocols(
                            parsed_error.message,
                        ))
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            ModifyListenerError::InvalidConfigurationRequest(parsed_error.message),
                        )
                    }
                    "InvalidLoadBalancerAction" => {
                        return RusotoError::Service(
                            ModifyListenerError::InvalidLoadBalancerAction(parsed_error.message),
                        )
                    }
                    "ListenerNotFound" => {
                        return RusotoError::Service(ModifyListenerError::ListenerNotFound(
                            parsed_error.message,
                        ))
                    }
                    "SSLPolicyNotFound" => {
                        return RusotoError::Service(ModifyListenerError::SSLPolicyNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TargetGroupAssociationLimit" => {
                        return RusotoError::Service(
                            ModifyListenerError::TargetGroupAssociationLimit(parsed_error.message),
                        )
                    }
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(ModifyListenerError::TargetGroupNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TooManyActions" => {
                        return RusotoError::Service(ModifyListenerError::TooManyActions(
                            parsed_error.message,
                        ))
                    }
                    "TooManyCertificates" => {
                        return RusotoError::Service(ModifyListenerError::TooManyCertificates(
                            parsed_error.message,
                        ))
                    }
                    "TooManyListeners" => {
                        return RusotoError::Service(ModifyListenerError::TooManyListeners(
                            parsed_error.message,
                        ))
                    }
                    "TooManyRegistrationsForTargetId" => {
                        return RusotoError::Service(
                            ModifyListenerError::TooManyRegistrationsForTargetId(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyTargets" => {
                        return RusotoError::Service(ModifyListenerError::TooManyTargets(
                            parsed_error.message,
                        ))
                    }
                    "TooManyUniqueTargetGroupsPerLoadBalancer" => {
                        return RusotoError::Service(
                            ModifyListenerError::TooManyUniqueTargetGroupsPerLoadBalancer(
                                parsed_error.message,
                            ),
                        )
                    }
                    "UnsupportedProtocol" => {
                        return RusotoError::Service(ModifyListenerError::UnsupportedProtocol(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyListenerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyListenerError::CertificateNotFound(ref cause) => write!(f, "{}", cause),
            ModifyListenerError::DuplicateListener(ref cause) => write!(f, "{}", cause),
            ModifyListenerError::IncompatibleProtocols(ref cause) => write!(f, "{}", cause),
            ModifyListenerError::InvalidConfigurationRequest(ref cause) => write!(f, "{}", cause),
            ModifyListenerError::InvalidLoadBalancerAction(ref cause) => write!(f, "{}", cause),
            ModifyListenerError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
            ModifyListenerError::SSLPolicyNotFound(ref cause) => write!(f, "{}", cause),
            ModifyListenerError::TargetGroupAssociationLimit(ref cause) => write!(f, "{}", cause),
            ModifyListenerError::TargetGroupNotFound(ref cause) => write!(f, "{}", cause),
            ModifyListenerError::TooManyActions(ref cause) => write!(f, "{}", cause),
            ModifyListenerError::TooManyCertificates(ref cause) => write!(f, "{}", cause),
            ModifyListenerError::TooManyListeners(ref cause) => write!(f, "{}", cause),
            ModifyListenerError::TooManyRegistrationsForTargetId(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyListenerError::TooManyTargets(ref cause) => write!(f, "{}", cause),
            ModifyListenerError::TooManyUniqueTargetGroupsPerLoadBalancer(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyListenerError::UnsupportedProtocol(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyListenerError {}
/// Errors returned by ModifyLoadBalancerAttributes
#[derive(Debug, PartialEq)]
pub enum ModifyLoadBalancerAttributesError {
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
}

impl ModifyLoadBalancerAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ModifyLoadBalancerAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            ModifyLoadBalancerAttributesError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(
                            ModifyLoadBalancerAttributesError::LoadBalancerNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyLoadBalancerAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyLoadBalancerAttributesError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyLoadBalancerAttributesError::LoadBalancerNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyLoadBalancerAttributesError {}
/// Errors returned by ModifyRule
#[derive(Debug, PartialEq)]
pub enum ModifyRuleError {
    /// <p>The specified configuration is not valid with this protocol.</p>
    IncompatibleProtocols(String),
    /// <p>The requested action is not valid.</p>
    InvalidLoadBalancerAction(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified rule does not exist.</p>
    RuleNotFound(String),
    /// <p>You've reached the limit on the number of load balancers per target group.</p>
    TargetGroupAssociationLimit(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of actions per rule.</p>
    TooManyActions(String),
    /// <p>You've reached the limit on the number of times a target can be registered with a load balancer.</p>
    TooManyRegistrationsForTargetId(String),
    /// <p>You've reached the limit on the number of targets.</p>
    TooManyTargets(String),
    /// <p>You've reached the limit on the number of unique target groups per load balancer across all listeners. If a target group is used by multiple actions for a load balancer, it is counted as only one use.</p>
    TooManyUniqueTargetGroupsPerLoadBalancer(String),
    /// <p>The specified protocol is not supported.</p>
    UnsupportedProtocol(String),
}

impl ModifyRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyRuleError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "IncompatibleProtocols" => {
                        return RusotoError::Service(ModifyRuleError::IncompatibleProtocols(
                            parsed_error.message,
                        ))
                    }
                    "InvalidLoadBalancerAction" => {
                        return RusotoError::Service(ModifyRuleError::InvalidLoadBalancerAction(
                            parsed_error.message,
                        ))
                    }
                    "OperationNotPermitted" => {
                        return RusotoError::Service(ModifyRuleError::OperationNotPermitted(
                            parsed_error.message,
                        ))
                    }
                    "RuleNotFound" => {
                        return RusotoError::Service(ModifyRuleError::RuleNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TargetGroupAssociationLimit" => {
                        return RusotoError::Service(ModifyRuleError::TargetGroupAssociationLimit(
                            parsed_error.message,
                        ))
                    }
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(ModifyRuleError::TargetGroupNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TooManyActions" => {
                        return RusotoError::Service(ModifyRuleError::TooManyActions(
                            parsed_error.message,
                        ))
                    }
                    "TooManyRegistrationsForTargetId" => {
                        return RusotoError::Service(
                            ModifyRuleError::TooManyRegistrationsForTargetId(parsed_error.message),
                        )
                    }
                    "TooManyTargets" => {
                        return RusotoError::Service(ModifyRuleError::TooManyTargets(
                            parsed_error.message,
                        ))
                    }
                    "TooManyUniqueTargetGroupsPerLoadBalancer" => {
                        return RusotoError::Service(
                            ModifyRuleError::TooManyUniqueTargetGroupsPerLoadBalancer(
                                parsed_error.message,
                            ),
                        )
                    }
                    "UnsupportedProtocol" => {
                        return RusotoError::Service(ModifyRuleError::UnsupportedProtocol(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyRuleError::IncompatibleProtocols(ref cause) => write!(f, "{}", cause),
            ModifyRuleError::InvalidLoadBalancerAction(ref cause) => write!(f, "{}", cause),
            ModifyRuleError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            ModifyRuleError::RuleNotFound(ref cause) => write!(f, "{}", cause),
            ModifyRuleError::TargetGroupAssociationLimit(ref cause) => write!(f, "{}", cause),
            ModifyRuleError::TargetGroupNotFound(ref cause) => write!(f, "{}", cause),
            ModifyRuleError::TooManyActions(ref cause) => write!(f, "{}", cause),
            ModifyRuleError::TooManyRegistrationsForTargetId(ref cause) => write!(f, "{}", cause),
            ModifyRuleError::TooManyTargets(ref cause) => write!(f, "{}", cause),
            ModifyRuleError::TooManyUniqueTargetGroupsPerLoadBalancer(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyRuleError::UnsupportedProtocol(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyRuleError {}
/// Errors returned by ModifyTargetGroup
#[derive(Debug, PartialEq)]
pub enum ModifyTargetGroupError {
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
}

impl ModifyTargetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyTargetGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            ModifyTargetGroupError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(ModifyTargetGroupError::TargetGroupNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyTargetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyTargetGroupError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyTargetGroupError::TargetGroupNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyTargetGroupError {}
/// Errors returned by ModifyTargetGroupAttributes
#[derive(Debug, PartialEq)]
pub enum ModifyTargetGroupAttributesError {
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
}

impl ModifyTargetGroupAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ModifyTargetGroupAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            ModifyTargetGroupAttributesError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(
                            ModifyTargetGroupAttributesError::TargetGroupNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyTargetGroupAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyTargetGroupAttributesError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyTargetGroupAttributesError::TargetGroupNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyTargetGroupAttributesError {}
/// Errors returned by RegisterTargets
#[derive(Debug, PartialEq)]
pub enum RegisterTargetsError {
    /// <p>The specified target does not exist, is not in the same VPC as the target group, or has an unsupported instance type.</p>
    InvalidTarget(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of times a target can be registered with a load balancer.</p>
    TooManyRegistrationsForTargetId(String),
    /// <p>You've reached the limit on the number of targets.</p>
    TooManyTargets(String),
}

impl RegisterTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterTargetsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidTarget" => {
                        return RusotoError::Service(RegisterTargetsError::InvalidTarget(
                            parsed_error.message,
                        ))
                    }
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(RegisterTargetsError::TargetGroupNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TooManyRegistrationsForTargetId" => {
                        return RusotoError::Service(
                            RegisterTargetsError::TooManyRegistrationsForTargetId(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyTargets" => {
                        return RusotoError::Service(RegisterTargetsError::TooManyTargets(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RegisterTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterTargetsError::InvalidTarget(ref cause) => write!(f, "{}", cause),
            RegisterTargetsError::TargetGroupNotFound(ref cause) => write!(f, "{}", cause),
            RegisterTargetsError::TooManyRegistrationsForTargetId(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterTargetsError::TooManyTargets(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterTargetsError {}
/// Errors returned by RemoveListenerCertificates
#[derive(Debug, PartialEq)]
pub enum RemoveListenerCertificatesError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
}

impl RemoveListenerCertificatesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RemoveListenerCertificatesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ListenerNotFound" => {
                        return RusotoError::Service(
                            RemoveListenerCertificatesError::ListenerNotFound(parsed_error.message),
                        )
                    }
                    "OperationNotPermitted" => {
                        return RusotoError::Service(
                            RemoveListenerCertificatesError::OperationNotPermitted(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RemoveListenerCertificatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveListenerCertificatesError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
            RemoveListenerCertificatesError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RemoveListenerCertificatesError {}
/// Errors returned by RemoveTags
#[derive(Debug, PartialEq)]
pub enum RemoveTagsError {
    /// <p>The specified listener does not exist.</p>
    ListenerNotFound(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>The specified rule does not exist.</p>
    RuleNotFound(String),
    /// <p>The specified target group does not exist.</p>
    TargetGroupNotFound(String),
    /// <p>You've reached the limit on the number of tags per load balancer.</p>
    TooManyTags(String),
}

impl RemoveTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ListenerNotFound" => {
                        return RusotoError::Service(RemoveTagsError::ListenerNotFound(
                            parsed_error.message,
                        ))
                    }
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(RemoveTagsError::LoadBalancerNotFound(
                            parsed_error.message,
                        ))
                    }
                    "RuleNotFound" => {
                        return RusotoError::Service(RemoveTagsError::RuleNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TargetGroupNotFound" => {
                        return RusotoError::Service(RemoveTagsError::TargetGroupNotFound(
                            parsed_error.message,
                        ))
                    }
                    "TooManyTags" => {
                        return RusotoError::Service(RemoveTagsError::TooManyTags(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RemoveTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsError::ListenerNotFound(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::LoadBalancerNotFound(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::RuleNotFound(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::TargetGroupNotFound(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTagsError {}
/// Errors returned by SetIpAddressType
#[derive(Debug, PartialEq)]
pub enum SetIpAddressTypeError {
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified subnet is out of available addresses.</p>
    InvalidSubnet(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
}

impl SetIpAddressTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetIpAddressTypeError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            SetIpAddressTypeError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(SetIpAddressTypeError::InvalidSubnet(
                            parsed_error.message,
                        ))
                    }
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(SetIpAddressTypeError::LoadBalancerNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetIpAddressTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetIpAddressTypeError::InvalidConfigurationRequest(ref cause) => write!(f, "{}", cause),
            SetIpAddressTypeError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            SetIpAddressTypeError::LoadBalancerNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetIpAddressTypeError {}
/// Errors returned by SetRulePriorities
#[derive(Debug, PartialEq)]
pub enum SetRulePrioritiesError {
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified priority is in use.</p>
    PriorityInUse(String),
    /// <p>The specified rule does not exist.</p>
    RuleNotFound(String),
}

impl SetRulePrioritiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetRulePrioritiesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "OperationNotPermitted" => {
                        return RusotoError::Service(SetRulePrioritiesError::OperationNotPermitted(
                            parsed_error.message,
                        ))
                    }
                    "PriorityInUse" => {
                        return RusotoError::Service(SetRulePrioritiesError::PriorityInUse(
                            parsed_error.message,
                        ))
                    }
                    "RuleNotFound" => {
                        return RusotoError::Service(SetRulePrioritiesError::RuleNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetRulePrioritiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetRulePrioritiesError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            SetRulePrioritiesError::PriorityInUse(ref cause) => write!(f, "{}", cause),
            SetRulePrioritiesError::RuleNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetRulePrioritiesError {}
/// Errors returned by SetSecurityGroups
#[derive(Debug, PartialEq)]
pub enum SetSecurityGroupsError {
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified security group does not exist.</p>
    InvalidSecurityGroup(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
}

impl SetSecurityGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetSecurityGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(
                            SetSecurityGroupsError::InvalidConfigurationRequest(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSecurityGroup" => {
                        return RusotoError::Service(SetSecurityGroupsError::InvalidSecurityGroup(
                            parsed_error.message,
                        ))
                    }
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(SetSecurityGroupsError::LoadBalancerNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetSecurityGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetSecurityGroupsError::InvalidConfigurationRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            SetSecurityGroupsError::InvalidSecurityGroup(ref cause) => write!(f, "{}", cause),
            SetSecurityGroupsError::LoadBalancerNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetSecurityGroupsError {}
/// Errors returned by SetSubnets
#[derive(Debug, PartialEq)]
pub enum SetSubnetsError {
    /// <p>The specified allocation ID does not exist.</p>
    AllocationIdNotFound(String),
    /// <p>The specified Availability Zone is not supported.</p>
    AvailabilityZoneNotSupported(String),
    /// <p>The requested configuration is not valid.</p>
    InvalidConfigurationRequest(String),
    /// <p>The specified subnet is out of available addresses.</p>
    InvalidSubnet(String),
    /// <p>The specified load balancer does not exist.</p>
    LoadBalancerNotFound(String),
    /// <p>The specified subnet does not exist.</p>
    SubnetNotFound(String),
}

impl SetSubnetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetSubnetsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AllocationIdNotFound" => {
                        return RusotoError::Service(SetSubnetsError::AllocationIdNotFound(
                            parsed_error.message,
                        ))
                    }
                    "AvailabilityZoneNotSupported" => {
                        return RusotoError::Service(SetSubnetsError::AvailabilityZoneNotSupported(
                            parsed_error.message,
                        ))
                    }
                    "InvalidConfigurationRequest" => {
                        return RusotoError::Service(SetSubnetsError::InvalidConfigurationRequest(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(SetSubnetsError::InvalidSubnet(
                            parsed_error.message,
                        ))
                    }
                    "LoadBalancerNotFound" => {
                        return RusotoError::Service(SetSubnetsError::LoadBalancerNotFound(
                            parsed_error.message,
                        ))
                    }
                    "SubnetNotFound" => {
                        return RusotoError::Service(SetSubnetsError::SubnetNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetSubnetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetSubnetsError::AllocationIdNotFound(ref cause) => write!(f, "{}", cause),
            SetSubnetsError::AvailabilityZoneNotSupported(ref cause) => write!(f, "{}", cause),
            SetSubnetsError::InvalidConfigurationRequest(ref cause) => write!(f, "{}", cause),
            SetSubnetsError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            SetSubnetsError::LoadBalancerNotFound(ref cause) => write!(f, "{}", cause),
            SetSubnetsError::SubnetNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetSubnetsError {}
/// Trait representing the capabilities of the Elastic Load Balancing v2 API. Elastic Load Balancing v2 clients implement this trait.
#[async_trait]
pub trait Elb {
    /// <p>Adds the specified SSL server certificate to the certificate list for the specified HTTPS or TLS listener.</p> <p>If the certificate in already in the certificate list, the call is successful but the certificate is not added again.</p> <p>To get the certificate list for a listener, use <a>DescribeListenerCertificates</a>. To remove certificates from the certificate list for a listener, use <a>RemoveListenerCertificates</a>. To replace the default certificate for a listener, use <a>ModifyListener</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/create-https-listener.html#https-listener-certificates">SSL Certificates</a> in the <i>Application Load Balancers Guide</i>.</p>
    async fn add_listener_certificates(
        &self,
        input: AddListenerCertificatesInput,
    ) -> Result<AddListenerCertificatesOutput, RusotoError<AddListenerCertificatesError>>;

    /// <p>Adds the specified tags to the specified Elastic Load Balancing resource. You can tag your Application Load Balancers, Network Load Balancers, and your target groups.</p> <p>Each tag consists of a key and an optional value. If a resource already has a tag with the same key, <code>AddTags</code> updates its value.</p> <p>To list the current tags for your resources, use <a>DescribeTags</a>. To remove tags from your resources, use <a>RemoveTags</a>.</p>
    async fn add_tags(
        &self,
        input: AddTagsInput,
    ) -> Result<AddTagsOutput, RusotoError<AddTagsError>>;

    /// <p>Creates a listener for the specified Application Load Balancer or Network Load Balancer.</p> <p>To update a listener, use <a>ModifyListener</a>. When you are finished with a listener, you can delete it using <a>DeleteListener</a>. If you are finished with both the listener and the load balancer, you can delete them both using <a>DeleteLoadBalancer</a>.</p> <p>This operation is idempotent, which means that it completes at most one time. If you attempt to create multiple listeners with the same settings, each call succeeds.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-listeners.html">Listeners for Your Application Load Balancers</a> in the <i>Application Load Balancers Guide</i> and <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-listeners.html">Listeners for Your Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    async fn create_listener(
        &self,
        input: CreateListenerInput,
    ) -> Result<CreateListenerOutput, RusotoError<CreateListenerError>>;

    /// <p>Creates an Application Load Balancer or a Network Load Balancer.</p> <p>When you create a load balancer, you can specify security groups, public subnets, IP address type, and tags. Otherwise, you could do so later using <a>SetSecurityGroups</a>, <a>SetSubnets</a>, <a>SetIpAddressType</a>, and <a>AddTags</a>.</p> <p>To create listeners for your load balancer, use <a>CreateListener</a>. To describe your current load balancers, see <a>DescribeLoadBalancers</a>. When you are finished with a load balancer, you can delete it using <a>DeleteLoadBalancer</a>.</p> <p>For limit information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-limits.html">Limits for Your Application Load Balancer</a> in the <i>Application Load Balancers Guide</i> and <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-limits.html">Limits for Your Network Load Balancer</a> in the <i>Network Load Balancers Guide</i>.</p> <p>This operation is idempotent, which means that it completes at most one time. If you attempt to create multiple load balancers with the same settings, each call succeeds.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/application-load-balancers.html">Application Load Balancers</a> in the <i>Application Load Balancers Guide</i> and <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/network-load-balancers.html">Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    async fn create_load_balancer(
        &self,
        input: CreateLoadBalancerInput,
    ) -> Result<CreateLoadBalancerOutput, RusotoError<CreateLoadBalancerError>>;

    /// <p>Creates a rule for the specified listener. The listener must be associated with an Application Load Balancer.</p> <p>Rules are evaluated in priority order, from the lowest value to the highest value. When the conditions for a rule are met, its actions are performed. If the conditions for no rules are met, the actions for the default rule are performed. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-listeners.html#listener-rules">Listener Rules</a> in the <i>Application Load Balancers Guide</i>.</p> <p>To view your current rules, use <a>DescribeRules</a>. To update a rule, use <a>ModifyRule</a>. To set the priorities of your rules, use <a>SetRulePriorities</a>. To delete a rule, use <a>DeleteRule</a>.</p>
    async fn create_rule(
        &self,
        input: CreateRuleInput,
    ) -> Result<CreateRuleOutput, RusotoError<CreateRuleError>>;

    /// <p>Creates a target group.</p> <p>To register targets with the target group, use <a>RegisterTargets</a>. To update the health check settings for the target group, use <a>ModifyTargetGroup</a>. To monitor the health of targets in the target group, use <a>DescribeTargetHealth</a>.</p> <p>To route traffic to the targets in a target group, specify the target group in an action using <a>CreateListener</a> or <a>CreateRule</a>.</p> <p>To delete a target group, use <a>DeleteTargetGroup</a>.</p> <p>This operation is idempotent, which means that it completes at most one time. If you attempt to create multiple target groups with the same settings, each call succeeds.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-target-groups.html">Target Groups for Your Application Load Balancers</a> in the <i>Application Load Balancers Guide</i> or <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html">Target Groups for Your Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    async fn create_target_group(
        &self,
        input: CreateTargetGroupInput,
    ) -> Result<CreateTargetGroupOutput, RusotoError<CreateTargetGroupError>>;

    /// <p>Deletes the specified listener.</p> <p>Alternatively, your listener is deleted when you delete the load balancer to which it is attached, using <a>DeleteLoadBalancer</a>.</p>
    async fn delete_listener(
        &self,
        input: DeleteListenerInput,
    ) -> Result<DeleteListenerOutput, RusotoError<DeleteListenerError>>;

    /// <p>Deletes the specified Application Load Balancer or Network Load Balancer and its attached listeners.</p> <p>You can't delete a load balancer if deletion protection is enabled. If the load balancer does not exist or has already been deleted, the call succeeds.</p> <p>Deleting a load balancer does not affect its registered targets. For example, your EC2 instances continue to run and are still registered to their target groups. If you no longer need these EC2 instances, you can stop or terminate them.</p>
    async fn delete_load_balancer(
        &self,
        input: DeleteLoadBalancerInput,
    ) -> Result<DeleteLoadBalancerOutput, RusotoError<DeleteLoadBalancerError>>;

    /// <p>Deletes the specified rule.</p>
    async fn delete_rule(
        &self,
        input: DeleteRuleInput,
    ) -> Result<DeleteRuleOutput, RusotoError<DeleteRuleError>>;

    /// <p>Deletes the specified target group.</p> <p>You can delete a target group if it is not referenced by any actions. Deleting a target group also deletes any associated health checks.</p>
    async fn delete_target_group(
        &self,
        input: DeleteTargetGroupInput,
    ) -> Result<DeleteTargetGroupOutput, RusotoError<DeleteTargetGroupError>>;

    /// <p>Deregisters the specified targets from the specified target group. After the targets are deregistered, they no longer receive traffic from the load balancer.</p>
    async fn deregister_targets(
        &self,
        input: DeregisterTargetsInput,
    ) -> Result<DeregisterTargetsOutput, RusotoError<DeregisterTargetsError>>;

    /// <p>Describes the current Elastic Load Balancing resource limits for your AWS account.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-limits.html">Limits for Your Application Load Balancers</a> in the <i>Application Load Balancer Guide</i> or <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-limits.html">Limits for Your Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    async fn describe_account_limits(
        &self,
        input: DescribeAccountLimitsInput,
    ) -> Result<DescribeAccountLimitsOutput, RusotoError<DescribeAccountLimitsError>>;

    /// <p>Describes the default certificate and the certificate list for the specified HTTPS or TLS listener.</p> <p>If the default certificate is also in the certificate list, it appears twice in the results (once with <code>IsDefault</code> set to true and once with <code>IsDefault</code> set to false).</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/create-https-listener.html#https-listener-certificates">SSL Certificates</a> in the <i>Application Load Balancers Guide</i>.</p>
    async fn describe_listener_certificates(
        &self,
        input: DescribeListenerCertificatesInput,
    ) -> Result<DescribeListenerCertificatesOutput, RusotoError<DescribeListenerCertificatesError>>;

    /// <p>Describes the specified listeners or the listeners for the specified Application Load Balancer or Network Load Balancer. You must specify either a load balancer or one or more listeners.</p> <p>For an HTTPS or TLS listener, the output includes the default certificate for the listener. To describe the certificate list for the listener, use <a>DescribeListenerCertificates</a>.</p>
    async fn describe_listeners(
        &self,
        input: DescribeListenersInput,
    ) -> Result<DescribeListenersOutput, RusotoError<DescribeListenersError>>;

    /// <p>Describes the attributes for the specified Application Load Balancer or Network Load Balancer.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/application-load-balancers.html#load-balancer-attributes">Load Balancer Attributes</a> in the <i>Application Load Balancers Guide</i> or <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/network-load-balancers.html#load-balancer-attributes">Load Balancer Attributes</a> in the <i>Network Load Balancers Guide</i>.</p>
    async fn describe_load_balancer_attributes(
        &self,
        input: DescribeLoadBalancerAttributesInput,
    ) -> Result<
        DescribeLoadBalancerAttributesOutput,
        RusotoError<DescribeLoadBalancerAttributesError>,
    >;

    /// <p>Describes the specified load balancers or all of your load balancers.</p> <p>To describe the listeners for a load balancer, use <a>DescribeListeners</a>. To describe the attributes for a load balancer, use <a>DescribeLoadBalancerAttributes</a>.</p>
    async fn describe_load_balancers(
        &self,
        input: DescribeLoadBalancersInput,
    ) -> Result<DescribeLoadBalancersOutput, RusotoError<DescribeLoadBalancersError>>;

    /// <p>Describes the specified rules or the rules for the specified listener. You must specify either a listener or one or more rules.</p>
    async fn describe_rules(
        &self,
        input: DescribeRulesInput,
    ) -> Result<DescribeRulesOutput, RusotoError<DescribeRulesError>>;

    /// <p>Describes the specified policies or all policies used for SSL negotiation.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/create-https-listener.html#describe-ssl-policies">Security Policies</a> in the <i>Application Load Balancers Guide</i>.</p>
    async fn describe_ssl_policies(
        &self,
        input: DescribeSSLPoliciesInput,
    ) -> Result<DescribeSSLPoliciesOutput, RusotoError<DescribeSSLPoliciesError>>;

    /// <p>Describes the tags for the specified resources. You can describe the tags for one or more Application Load Balancers, Network Load Balancers, and target groups.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsInput,
    ) -> Result<DescribeTagsOutput, RusotoError<DescribeTagsError>>;

    /// <p>Describes the attributes for the specified target group.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-target-groups.html#target-group-attributes">Target Group Attributes</a> in the <i>Application Load Balancers Guide</i> or <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html#target-group-attributes">Target Group Attributes</a> in the <i>Network Load Balancers Guide</i>.</p>
    async fn describe_target_group_attributes(
        &self,
        input: DescribeTargetGroupAttributesInput,
    ) -> Result<DescribeTargetGroupAttributesOutput, RusotoError<DescribeTargetGroupAttributesError>>;

    /// <p>Describes the specified target groups or all of your target groups. By default, all target groups are described. Alternatively, you can specify one of the following to filter the results: the ARN of the load balancer, the names of one or more target groups, or the ARNs of one or more target groups.</p> <p>To describe the targets for a target group, use <a>DescribeTargetHealth</a>. To describe the attributes of a target group, use <a>DescribeTargetGroupAttributes</a>.</p>
    async fn describe_target_groups(
        &self,
        input: DescribeTargetGroupsInput,
    ) -> Result<DescribeTargetGroupsOutput, RusotoError<DescribeTargetGroupsError>>;

    /// <p>Describes the health of the specified targets or all of your targets.</p>
    async fn describe_target_health(
        &self,
        input: DescribeTargetHealthInput,
    ) -> Result<DescribeTargetHealthOutput, RusotoError<DescribeTargetHealthError>>;

    /// <p>Replaces the specified properties of the specified listener. Any properties that you do not specify remain unchanged.</p> <p>Changing the protocol from HTTPS to HTTP, or from TLS to TCP, removes the security policy and default certificate properties. If you change the protocol from HTTP to HTTPS, or from TCP to TLS, you must add the security policy and default certificate properties.</p> <p>To add an item to a list, remove an item from a list, or update an item in a list, you must provide the entire list. For example, to add an action, specify a list with the current actions plus the new action.</p>
    async fn modify_listener(
        &self,
        input: ModifyListenerInput,
    ) -> Result<ModifyListenerOutput, RusotoError<ModifyListenerError>>;

    /// <p>Modifies the specified attributes of the specified Application Load Balancer or Network Load Balancer.</p> <p>If any of the specified attributes can't be modified as requested, the call fails. Any existing attributes that you do not modify retain their current values.</p>
    async fn modify_load_balancer_attributes(
        &self,
        input: ModifyLoadBalancerAttributesInput,
    ) -> Result<ModifyLoadBalancerAttributesOutput, RusotoError<ModifyLoadBalancerAttributesError>>;

    /// <p>Replaces the specified properties of the specified rule. Any properties that you do not specify are unchanged.</p> <p>To add an item to a list, remove an item from a list, or update an item in a list, you must provide the entire list. For example, to add an action, specify a list with the current actions plus the new action.</p> <p>To modify the actions for the default rule, use <a>ModifyListener</a>.</p>
    async fn modify_rule(
        &self,
        input: ModifyRuleInput,
    ) -> Result<ModifyRuleOutput, RusotoError<ModifyRuleError>>;

    /// <p>Modifies the health checks used when evaluating the health state of the targets in the specified target group.</p> <p>To monitor the health of the targets, use <a>DescribeTargetHealth</a>.</p>
    async fn modify_target_group(
        &self,
        input: ModifyTargetGroupInput,
    ) -> Result<ModifyTargetGroupOutput, RusotoError<ModifyTargetGroupError>>;

    /// <p>Modifies the specified attributes of the specified target group.</p>
    async fn modify_target_group_attributes(
        &self,
        input: ModifyTargetGroupAttributesInput,
    ) -> Result<ModifyTargetGroupAttributesOutput, RusotoError<ModifyTargetGroupAttributesError>>;

    /// <p>Registers the specified targets with the specified target group.</p> <p>If the target is an EC2 instance, it must be in the <code>running</code> state when you register it.</p> <p>By default, the load balancer routes requests to registered targets using the protocol and port for the target group. Alternatively, you can override the port for a target when you register it. You can register each EC2 instance or IP address with the same target group multiple times using different ports.</p> <p>With a Network Load Balancer, you cannot register instances by instance ID if they have the following instance types: C1, CC1, CC2, CG1, CG2, CR1, CS1, G1, G2, HI1, HS1, M1, M2, M3, and T1. You can register instances of these types by IP address.</p> <p>To remove a target from a target group, use <a>DeregisterTargets</a>.</p>
    async fn register_targets(
        &self,
        input: RegisterTargetsInput,
    ) -> Result<RegisterTargetsOutput, RusotoError<RegisterTargetsError>>;

    /// <p>Removes the specified certificate from the certificate list for the specified HTTPS or TLS listener.</p> <p>You can't remove the default certificate for a listener. To replace the default certificate, call <a>ModifyListener</a>.</p> <p>To list the certificates for your listener, use <a>DescribeListenerCertificates</a>.</p>
    async fn remove_listener_certificates(
        &self,
        input: RemoveListenerCertificatesInput,
    ) -> Result<RemoveListenerCertificatesOutput, RusotoError<RemoveListenerCertificatesError>>;

    /// <p>Removes the specified tags from the specified Elastic Load Balancing resource.</p> <p>To list the current tags for your resources, use <a>DescribeTags</a>.</p>
    async fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> Result<RemoveTagsOutput, RusotoError<RemoveTagsError>>;

    /// <p>Sets the type of IP addresses used by the subnets of the specified Application Load Balancer or Network Load Balancer.</p>
    async fn set_ip_address_type(
        &self,
        input: SetIpAddressTypeInput,
    ) -> Result<SetIpAddressTypeOutput, RusotoError<SetIpAddressTypeError>>;

    /// <p>Sets the priorities of the specified rules.</p> <p>You can reorder the rules as long as there are no priority conflicts in the new order. Any existing rules that you do not specify retain their current priority.</p>
    async fn set_rule_priorities(
        &self,
        input: SetRulePrioritiesInput,
    ) -> Result<SetRulePrioritiesOutput, RusotoError<SetRulePrioritiesError>>;

    /// <p>Associates the specified security groups with the specified Application Load Balancer. The specified security groups override the previously associated security groups.</p> <p>You can't specify a security group for a Network Load Balancer.</p>
    async fn set_security_groups(
        &self,
        input: SetSecurityGroupsInput,
    ) -> Result<SetSecurityGroupsOutput, RusotoError<SetSecurityGroupsError>>;

    /// <p>Enables the Availability Zones for the specified public subnets for the specified load balancer. The specified subnets replace the previously enabled subnets.</p> <p>When you specify subnets for a Network Load Balancer, you must include all subnets that were enabled previously, with their existing configurations, plus any additional subnets.</p>
    async fn set_subnets(
        &self,
        input: SetSubnetsInput,
    ) -> Result<SetSubnetsOutput, RusotoError<SetSubnetsError>>;
}
/// A client for the Elastic Load Balancing v2 API.
#[derive(Clone)]
pub struct ElbClient {
    client: Client,
    region: region::Region,
}

impl ElbClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ElbClient {
        ElbClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ElbClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ElbClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ElbClient {
        ElbClient { client, region }
    }
}

#[async_trait]
impl Elb for ElbClient {
    /// <p>Adds the specified SSL server certificate to the certificate list for the specified HTTPS or TLS listener.</p> <p>If the certificate in already in the certificate list, the call is successful but the certificate is not added again.</p> <p>To get the certificate list for a listener, use <a>DescribeListenerCertificates</a>. To remove certificates from the certificate list for a listener, use <a>RemoveListenerCertificates</a>. To replace the default certificate for a listener, use <a>ModifyListener</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/create-https-listener.html#https-listener-certificates">SSL Certificates</a> in the <i>Application Load Balancers Guide</i>.</p>
    async fn add_listener_certificates(
        &self,
        input: AddListenerCertificatesInput,
    ) -> Result<AddListenerCertificatesOutput, RusotoError<AddListenerCertificatesError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddListenerCertificates");
        params.put("Version", "2015-12-01");
        AddListenerCertificatesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(AddListenerCertificatesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = AddListenerCertificatesOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = AddListenerCertificatesOutputDeserializer::deserialize(
                "AddListenerCertificatesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Adds the specified tags to the specified Elastic Load Balancing resource. You can tag your Application Load Balancers, Network Load Balancers, and your target groups.</p> <p>Each tag consists of a key and an optional value. If a resource already has a tag with the same key, <code>AddTags</code> updates its value.</p> <p>To list the current tags for your resources, use <a>DescribeTags</a>. To remove tags from your resources, use <a>RemoveTags</a>.</p>
    async fn add_tags(
        &self,
        input: AddTagsInput,
    ) -> Result<AddTagsOutput, RusotoError<AddTagsError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddTags");
        params.put("Version", "2015-12-01");
        AddTagsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(AddTagsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = AddTagsOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = AddTagsOutputDeserializer::deserialize("AddTagsResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a listener for the specified Application Load Balancer or Network Load Balancer.</p> <p>To update a listener, use <a>ModifyListener</a>. When you are finished with a listener, you can delete it using <a>DeleteListener</a>. If you are finished with both the listener and the load balancer, you can delete them both using <a>DeleteLoadBalancer</a>.</p> <p>This operation is idempotent, which means that it completes at most one time. If you attempt to create multiple listeners with the same settings, each call succeeds.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-listeners.html">Listeners for Your Application Load Balancers</a> in the <i>Application Load Balancers Guide</i> and <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-listeners.html">Listeners for Your Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    async fn create_listener(
        &self,
        input: CreateListenerInput,
    ) -> Result<CreateListenerOutput, RusotoError<CreateListenerError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateListener");
        params.put("Version", "2015-12-01");
        CreateListenerInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateListenerError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateListenerOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result =
                CreateListenerOutputDeserializer::deserialize("CreateListenerResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates an Application Load Balancer or a Network Load Balancer.</p> <p>When you create a load balancer, you can specify security groups, public subnets, IP address type, and tags. Otherwise, you could do so later using <a>SetSecurityGroups</a>, <a>SetSubnets</a>, <a>SetIpAddressType</a>, and <a>AddTags</a>.</p> <p>To create listeners for your load balancer, use <a>CreateListener</a>. To describe your current load balancers, see <a>DescribeLoadBalancers</a>. When you are finished with a load balancer, you can delete it using <a>DeleteLoadBalancer</a>.</p> <p>For limit information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-limits.html">Limits for Your Application Load Balancer</a> in the <i>Application Load Balancers Guide</i> and <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-limits.html">Limits for Your Network Load Balancer</a> in the <i>Network Load Balancers Guide</i>.</p> <p>This operation is idempotent, which means that it completes at most one time. If you attempt to create multiple load balancers with the same settings, each call succeeds.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/application-load-balancers.html">Application Load Balancers</a> in the <i>Application Load Balancers Guide</i> and <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/network-load-balancers.html">Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    async fn create_load_balancer(
        &self,
        input: CreateLoadBalancerInput,
    ) -> Result<CreateLoadBalancerOutput, RusotoError<CreateLoadBalancerError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateLoadBalancer");
        params.put("Version", "2015-12-01");
        CreateLoadBalancerInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateLoadBalancerError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateLoadBalancerOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateLoadBalancerOutputDeserializer::deserialize(
                "CreateLoadBalancerResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a rule for the specified listener. The listener must be associated with an Application Load Balancer.</p> <p>Rules are evaluated in priority order, from the lowest value to the highest value. When the conditions for a rule are met, its actions are performed. If the conditions for no rules are met, the actions for the default rule are performed. For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-listeners.html#listener-rules">Listener Rules</a> in the <i>Application Load Balancers Guide</i>.</p> <p>To view your current rules, use <a>DescribeRules</a>. To update a rule, use <a>ModifyRule</a>. To set the priorities of your rules, use <a>SetRulePriorities</a>. To delete a rule, use <a>DeleteRule</a>.</p>
    async fn create_rule(
        &self,
        input: CreateRuleInput,
    ) -> Result<CreateRuleOutput, RusotoError<CreateRuleError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateRule");
        params.put("Version", "2015-12-01");
        CreateRuleInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateRuleError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateRuleOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateRuleOutputDeserializer::deserialize("CreateRuleResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates a target group.</p> <p>To register targets with the target group, use <a>RegisterTargets</a>. To update the health check settings for the target group, use <a>ModifyTargetGroup</a>. To monitor the health of targets in the target group, use <a>DescribeTargetHealth</a>.</p> <p>To route traffic to the targets in a target group, specify the target group in an action using <a>CreateListener</a> or <a>CreateRule</a>.</p> <p>To delete a target group, use <a>DeleteTargetGroup</a>.</p> <p>This operation is idempotent, which means that it completes at most one time. If you attempt to create multiple target groups with the same settings, each call succeeds.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-target-groups.html">Target Groups for Your Application Load Balancers</a> in the <i>Application Load Balancers Guide</i> or <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html">Target Groups for Your Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    async fn create_target_group(
        &self,
        input: CreateTargetGroupInput,
    ) -> Result<CreateTargetGroupOutput, RusotoError<CreateTargetGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateTargetGroup");
        params.put("Version", "2015-12-01");
        CreateTargetGroupInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateTargetGroupError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = CreateTargetGroupOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = CreateTargetGroupOutputDeserializer::deserialize(
                "CreateTargetGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified listener.</p> <p>Alternatively, your listener is deleted when you delete the load balancer to which it is attached, using <a>DeleteLoadBalancer</a>.</p>
    async fn delete_listener(
        &self,
        input: DeleteListenerInput,
    ) -> Result<DeleteListenerOutput, RusotoError<DeleteListenerError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteListener");
        params.put("Version", "2015-12-01");
        DeleteListenerInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteListenerError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteListenerOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result =
                DeleteListenerOutputDeserializer::deserialize("DeleteListenerResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified Application Load Balancer or Network Load Balancer and its attached listeners.</p> <p>You can't delete a load balancer if deletion protection is enabled. If the load balancer does not exist or has already been deleted, the call succeeds.</p> <p>Deleting a load balancer does not affect its registered targets. For example, your EC2 instances continue to run and are still registered to their target groups. If you no longer need these EC2 instances, you can stop or terminate them.</p>
    async fn delete_load_balancer(
        &self,
        input: DeleteLoadBalancerInput,
    ) -> Result<DeleteLoadBalancerOutput, RusotoError<DeleteLoadBalancerError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteLoadBalancer");
        params.put("Version", "2015-12-01");
        DeleteLoadBalancerInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteLoadBalancerError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteLoadBalancerOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteLoadBalancerOutputDeserializer::deserialize(
                "DeleteLoadBalancerResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified rule.</p>
    async fn delete_rule(
        &self,
        input: DeleteRuleInput,
    ) -> Result<DeleteRuleOutput, RusotoError<DeleteRuleError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteRule");
        params.put("Version", "2015-12-01");
        DeleteRuleInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteRuleError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteRuleOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteRuleOutputDeserializer::deserialize("DeleteRuleResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified target group.</p> <p>You can delete a target group if it is not referenced by any actions. Deleting a target group also deletes any associated health checks.</p>
    async fn delete_target_group(
        &self,
        input: DeleteTargetGroupInput,
    ) -> Result<DeleteTargetGroupOutput, RusotoError<DeleteTargetGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteTargetGroup");
        params.put("Version", "2015-12-01");
        DeleteTargetGroupInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteTargetGroupError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeleteTargetGroupOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeleteTargetGroupOutputDeserializer::deserialize(
                "DeleteTargetGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deregisters the specified targets from the specified target group. After the targets are deregistered, they no longer receive traffic from the load balancer.</p>
    async fn deregister_targets(
        &self,
        input: DeregisterTargetsInput,
    ) -> Result<DeregisterTargetsOutput, RusotoError<DeregisterTargetsError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeregisterTargets");
        params.put("Version", "2015-12-01");
        DeregisterTargetsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeregisterTargetsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeregisterTargetsOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DeregisterTargetsOutputDeserializer::deserialize(
                "DeregisterTargetsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the current Elastic Load Balancing resource limits for your AWS account.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-limits.html">Limits for Your Application Load Balancers</a> in the <i>Application Load Balancer Guide</i> or <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-limits.html">Limits for Your Network Load Balancers</a> in the <i>Network Load Balancers Guide</i>.</p>
    async fn describe_account_limits(
        &self,
        input: DescribeAccountLimitsInput,
    ) -> Result<DescribeAccountLimitsOutput, RusotoError<DescribeAccountLimitsError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAccountLimits");
        params.put("Version", "2015-12-01");
        DescribeAccountLimitsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeAccountLimitsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeAccountLimitsOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeAccountLimitsOutputDeserializer::deserialize(
                "DescribeAccountLimitsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the default certificate and the certificate list for the specified HTTPS or TLS listener.</p> <p>If the default certificate is also in the certificate list, it appears twice in the results (once with <code>IsDefault</code> set to true and once with <code>IsDefault</code> set to false).</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/create-https-listener.html#https-listener-certificates">SSL Certificates</a> in the <i>Application Load Balancers Guide</i>.</p>
    async fn describe_listener_certificates(
        &self,
        input: DescribeListenerCertificatesInput,
    ) -> Result<DescribeListenerCertificatesOutput, RusotoError<DescribeListenerCertificatesError>>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeListenerCertificates");
        params.put("Version", "2015-12-01");
        DescribeListenerCertificatesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeListenerCertificatesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeListenerCertificatesOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeListenerCertificatesOutputDeserializer::deserialize(
                "DescribeListenerCertificatesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the specified listeners or the listeners for the specified Application Load Balancer or Network Load Balancer. You must specify either a load balancer or one or more listeners.</p> <p>For an HTTPS or TLS listener, the output includes the default certificate for the listener. To describe the certificate list for the listener, use <a>DescribeListenerCertificates</a>.</p>
    async fn describe_listeners(
        &self,
        input: DescribeListenersInput,
    ) -> Result<DescribeListenersOutput, RusotoError<DescribeListenersError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeListeners");
        params.put("Version", "2015-12-01");
        DescribeListenersInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeListenersError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeListenersOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeListenersOutputDeserializer::deserialize(
                "DescribeListenersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the attributes for the specified Application Load Balancer or Network Load Balancer.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/application-load-balancers.html#load-balancer-attributes">Load Balancer Attributes</a> in the <i>Application Load Balancers Guide</i> or <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/network-load-balancers.html#load-balancer-attributes">Load Balancer Attributes</a> in the <i>Network Load Balancers Guide</i>.</p>
    async fn describe_load_balancer_attributes(
        &self,
        input: DescribeLoadBalancerAttributesInput,
    ) -> Result<
        DescribeLoadBalancerAttributesOutput,
        RusotoError<DescribeLoadBalancerAttributesError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLoadBalancerAttributes");
        params.put("Version", "2015-12-01");
        DescribeLoadBalancerAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeLoadBalancerAttributesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeLoadBalancerAttributesOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeLoadBalancerAttributesOutputDeserializer::deserialize(
                "DescribeLoadBalancerAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the specified load balancers or all of your load balancers.</p> <p>To describe the listeners for a load balancer, use <a>DescribeListeners</a>. To describe the attributes for a load balancer, use <a>DescribeLoadBalancerAttributes</a>.</p>
    async fn describe_load_balancers(
        &self,
        input: DescribeLoadBalancersInput,
    ) -> Result<DescribeLoadBalancersOutput, RusotoError<DescribeLoadBalancersError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeLoadBalancers");
        params.put("Version", "2015-12-01");
        DescribeLoadBalancersInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeLoadBalancersError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeLoadBalancersOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeLoadBalancersOutputDeserializer::deserialize(
                "DescribeLoadBalancersResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the specified rules or the rules for the specified listener. You must specify either a listener or one or more rules.</p>
    async fn describe_rules(
        &self,
        input: DescribeRulesInput,
    ) -> Result<DescribeRulesOutput, RusotoError<DescribeRulesError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeRules");
        params.put("Version", "2015-12-01");
        DescribeRulesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeRulesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeRulesOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result =
                DescribeRulesOutputDeserializer::deserialize("DescribeRulesResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the specified policies or all policies used for SSL negotiation.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/create-https-listener.html#describe-ssl-policies">Security Policies</a> in the <i>Application Load Balancers Guide</i>.</p>
    async fn describe_ssl_policies(
        &self,
        input: DescribeSSLPoliciesInput,
    ) -> Result<DescribeSSLPoliciesOutput, RusotoError<DescribeSSLPoliciesError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeSSLPolicies");
        params.put("Version", "2015-12-01");
        DescribeSSLPoliciesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeSSLPoliciesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeSSLPoliciesOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeSSLPoliciesOutputDeserializer::deserialize(
                "DescribeSSLPoliciesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the tags for the specified resources. You can describe the tags for one or more Application Load Balancers, Network Load Balancers, and target groups.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsInput,
    ) -> Result<DescribeTagsOutput, RusotoError<DescribeTagsError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTags");
        params.put("Version", "2015-12-01");
        DescribeTagsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeTagsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeTagsOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeTagsOutputDeserializer::deserialize("DescribeTagsResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the attributes for the specified target group.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-target-groups.html#target-group-attributes">Target Group Attributes</a> in the <i>Application Load Balancers Guide</i> or <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html#target-group-attributes">Target Group Attributes</a> in the <i>Network Load Balancers Guide</i>.</p>
    async fn describe_target_group_attributes(
        &self,
        input: DescribeTargetGroupAttributesInput,
    ) -> Result<DescribeTargetGroupAttributesOutput, RusotoError<DescribeTargetGroupAttributesError>>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTargetGroupAttributes");
        params.put("Version", "2015-12-01");
        DescribeTargetGroupAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeTargetGroupAttributesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeTargetGroupAttributesOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeTargetGroupAttributesOutputDeserializer::deserialize(
                "DescribeTargetGroupAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the specified target groups or all of your target groups. By default, all target groups are described. Alternatively, you can specify one of the following to filter the results: the ARN of the load balancer, the names of one or more target groups, or the ARNs of one or more target groups.</p> <p>To describe the targets for a target group, use <a>DescribeTargetHealth</a>. To describe the attributes of a target group, use <a>DescribeTargetGroupAttributes</a>.</p>
    async fn describe_target_groups(
        &self,
        input: DescribeTargetGroupsInput,
    ) -> Result<DescribeTargetGroupsOutput, RusotoError<DescribeTargetGroupsError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTargetGroups");
        params.put("Version", "2015-12-01");
        DescribeTargetGroupsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeTargetGroupsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeTargetGroupsOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeTargetGroupsOutputDeserializer::deserialize(
                "DescribeTargetGroupsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Describes the health of the specified targets or all of your targets.</p>
    async fn describe_target_health(
        &self,
        input: DescribeTargetHealthInput,
    ) -> Result<DescribeTargetHealthOutput, RusotoError<DescribeTargetHealthError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeTargetHealth");
        params.put("Version", "2015-12-01");
        DescribeTargetHealthInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeTargetHealthError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeTargetHealthOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = DescribeTargetHealthOutputDeserializer::deserialize(
                "DescribeTargetHealthResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Replaces the specified properties of the specified listener. Any properties that you do not specify remain unchanged.</p> <p>Changing the protocol from HTTPS to HTTP, or from TLS to TCP, removes the security policy and default certificate properties. If you change the protocol from HTTP to HTTPS, or from TCP to TLS, you must add the security policy and default certificate properties.</p> <p>To add an item to a list, remove an item from a list, or update an item in a list, you must provide the entire list. For example, to add an action, specify a list with the current actions plus the new action.</p>
    async fn modify_listener(
        &self,
        input: ModifyListenerInput,
    ) -> Result<ModifyListenerOutput, RusotoError<ModifyListenerError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyListener");
        params.put("Version", "2015-12-01");
        ModifyListenerInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyListenerError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ModifyListenerOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result =
                ModifyListenerOutputDeserializer::deserialize("ModifyListenerResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Modifies the specified attributes of the specified Application Load Balancer or Network Load Balancer.</p> <p>If any of the specified attributes can't be modified as requested, the call fails. Any existing attributes that you do not modify retain their current values.</p>
    async fn modify_load_balancer_attributes(
        &self,
        input: ModifyLoadBalancerAttributesInput,
    ) -> Result<ModifyLoadBalancerAttributesOutput, RusotoError<ModifyLoadBalancerAttributesError>>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyLoadBalancerAttributes");
        params.put("Version", "2015-12-01");
        ModifyLoadBalancerAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyLoadBalancerAttributesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ModifyLoadBalancerAttributesOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ModifyLoadBalancerAttributesOutputDeserializer::deserialize(
                "ModifyLoadBalancerAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Replaces the specified properties of the specified rule. Any properties that you do not specify are unchanged.</p> <p>To add an item to a list, remove an item from a list, or update an item in a list, you must provide the entire list. For example, to add an action, specify a list with the current actions plus the new action.</p> <p>To modify the actions for the default rule, use <a>ModifyListener</a>.</p>
    async fn modify_rule(
        &self,
        input: ModifyRuleInput,
    ) -> Result<ModifyRuleOutput, RusotoError<ModifyRuleError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyRule");
        params.put("Version", "2015-12-01");
        ModifyRuleInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyRuleError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ModifyRuleOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ModifyRuleOutputDeserializer::deserialize("ModifyRuleResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Modifies the health checks used when evaluating the health state of the targets in the specified target group.</p> <p>To monitor the health of the targets, use <a>DescribeTargetHealth</a>.</p>
    async fn modify_target_group(
        &self,
        input: ModifyTargetGroupInput,
    ) -> Result<ModifyTargetGroupOutput, RusotoError<ModifyTargetGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyTargetGroup");
        params.put("Version", "2015-12-01");
        ModifyTargetGroupInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyTargetGroupError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ModifyTargetGroupOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ModifyTargetGroupOutputDeserializer::deserialize(
                "ModifyTargetGroupResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Modifies the specified attributes of the specified target group.</p>
    async fn modify_target_group_attributes(
        &self,
        input: ModifyTargetGroupAttributesInput,
    ) -> Result<ModifyTargetGroupAttributesOutput, RusotoError<ModifyTargetGroupAttributesError>>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyTargetGroupAttributes");
        params.put("Version", "2015-12-01");
        ModifyTargetGroupAttributesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyTargetGroupAttributesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ModifyTargetGroupAttributesOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ModifyTargetGroupAttributesOutputDeserializer::deserialize(
                "ModifyTargetGroupAttributesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Registers the specified targets with the specified target group.</p> <p>If the target is an EC2 instance, it must be in the <code>running</code> state when you register it.</p> <p>By default, the load balancer routes requests to registered targets using the protocol and port for the target group. Alternatively, you can override the port for a target when you register it. You can register each EC2 instance or IP address with the same target group multiple times using different ports.</p> <p>With a Network Load Balancer, you cannot register instances by instance ID if they have the following instance types: C1, CC1, CC2, CG1, CG2, CR1, CS1, G1, G2, HI1, HS1, M1, M2, M3, and T1. You can register instances of these types by IP address.</p> <p>To remove a target from a target group, use <a>DeregisterTargets</a>.</p>
    async fn register_targets(
        &self,
        input: RegisterTargetsInput,
    ) -> Result<RegisterTargetsOutput, RusotoError<RegisterTargetsError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RegisterTargets");
        params.put("Version", "2015-12-01");
        RegisterTargetsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(RegisterTargetsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = RegisterTargetsOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = RegisterTargetsOutputDeserializer::deserialize(
                "RegisterTargetsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Removes the specified certificate from the certificate list for the specified HTTPS or TLS listener.</p> <p>You can't remove the default certificate for a listener. To replace the default certificate, call <a>ModifyListener</a>.</p> <p>To list the certificates for your listener, use <a>DescribeListenerCertificates</a>.</p>
    async fn remove_listener_certificates(
        &self,
        input: RemoveListenerCertificatesInput,
    ) -> Result<RemoveListenerCertificatesOutput, RusotoError<RemoveListenerCertificatesError>>
    {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemoveListenerCertificates");
        params.put("Version", "2015-12-01");
        RemoveListenerCertificatesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(RemoveListenerCertificatesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = RemoveListenerCertificatesOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = RemoveListenerCertificatesOutputDeserializer::deserialize(
                "RemoveListenerCertificatesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Removes the specified tags from the specified Elastic Load Balancing resource.</p> <p>To list the current tags for your resources, use <a>DescribeTags</a>.</p>
    async fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> Result<RemoveTagsOutput, RusotoError<RemoveTagsError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemoveTags");
        params.put("Version", "2015-12-01");
        RemoveTagsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(RemoveTagsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = RemoveTagsOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = RemoveTagsOutputDeserializer::deserialize("RemoveTagsResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Sets the type of IP addresses used by the subnets of the specified Application Load Balancer or Network Load Balancer.</p>
    async fn set_ip_address_type(
        &self,
        input: SetIpAddressTypeInput,
    ) -> Result<SetIpAddressTypeOutput, RusotoError<SetIpAddressTypeError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetIpAddressType");
        params.put("Version", "2015-12-01");
        SetIpAddressTypeInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetIpAddressTypeError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SetIpAddressTypeOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SetIpAddressTypeOutputDeserializer::deserialize(
                "SetIpAddressTypeResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Sets the priorities of the specified rules.</p> <p>You can reorder the rules as long as there are no priority conflicts in the new order. Any existing rules that you do not specify retain their current priority.</p>
    async fn set_rule_priorities(
        &self,
        input: SetRulePrioritiesInput,
    ) -> Result<SetRulePrioritiesOutput, RusotoError<SetRulePrioritiesError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetRulePriorities");
        params.put("Version", "2015-12-01");
        SetRulePrioritiesInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetRulePrioritiesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SetRulePrioritiesOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SetRulePrioritiesOutputDeserializer::deserialize(
                "SetRulePrioritiesResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Associates the specified security groups with the specified Application Load Balancer. The specified security groups override the previously associated security groups.</p> <p>You can't specify a security group for a Network Load Balancer.</p>
    async fn set_security_groups(
        &self,
        input: SetSecurityGroupsInput,
    ) -> Result<SetSecurityGroupsOutput, RusotoError<SetSecurityGroupsError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetSecurityGroups");
        params.put("Version", "2015-12-01");
        SetSecurityGroupsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetSecurityGroupsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SetSecurityGroupsOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SetSecurityGroupsOutputDeserializer::deserialize(
                "SetSecurityGroupsResult",
                &mut stack,
            )?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Enables the Availability Zones for the specified public subnets for the specified load balancer. The specified subnets replace the previously enabled subnets.</p> <p>When you specify subnets for a Network Load Balancer, you must include all subnets that were enabled previously, with their existing configurations, plus any additional subnets.</p>
    async fn set_subnets(
        &self,
        input: SetSubnetsInput,
    ) -> Result<SetSubnetsOutput, RusotoError<SetSubnetsError>> {
        let mut request = SignedRequest::new("POST", "elasticloadbalancing", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetSubnets");
        params.put("Version", "2015-12-01");
        SetSubnetsInputSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SetSubnetsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SetSubnetsOutput::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SetSubnetsOutputDeserializer::deserialize("SetSubnetsResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[tokio::test]
    async fn test_parse_error_elb_describe_load_balancers() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "elb-describe-load-balancers.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = ElbClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeLoadBalancersInput::default();
        let result = client.describe_load_balancers(request).await;
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_elb_describe_load_balancers() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elb-describe-load-balancers.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = ElbClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeLoadBalancersInput::default();
        let result = client.describe_load_balancers(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
