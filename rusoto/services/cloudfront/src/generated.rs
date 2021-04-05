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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto::xml::error::*;
use rusoto_core::proto::xml::util::{
    self as xml_util, deserialize_elements, find_start_element, skip_tree, write_characters_element,
};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[cfg(feature = "deserialize_structs")]
use serde::Deserialize;
#[cfg(feature = "serialize_structs")]
use serde::Serialize;
use std::io::Write;
use std::str::FromStr;
use xml;
use xml::EventReader;
use xml::EventWriter;

impl CloudFrontClient {
    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}
/// <p>A list of key groups, and the public keys in each key group, that CloudFront can use to verify the signatures of signed URLs and signed cookies.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ActiveTrustedKeyGroups {
    /// <p>This field is <code>true</code> if any of the key groups have public keys that CloudFront can use to verify the signatures of signed URLs and signed cookies. If not, this field is <code>false</code>.</p>
    pub enabled: bool,
    /// <p>A list of key groups, including the identifiers of the public keys in each key group that CloudFront can use to verify the signatures of signed URLs and signed cookies.</p>
    pub items: Option<Vec<KGKeyPairIds>>,
    /// <p>The number of key groups in the list.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct ActiveTrustedKeyGroupsDeserializer;
impl ActiveTrustedKeyGroupsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ActiveTrustedKeyGroups, XmlParseError> {
        deserialize_elements::<_, ActiveTrustedKeyGroups, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Enabled" => {
                    obj.enabled = BooleanDeserializer::deserialize("Enabled", stack)?;
                }
                "Items" => {
                    obj.items
                        .get_or_insert(vec![])
                        .extend(KGKeyPairIdsListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A list of AWS accounts and the active CloudFront key pairs in each account that CloudFront can use to verify the signatures of signed URLs and signed cookies.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ActiveTrustedSigners {
    /// <p>This field is <code>true</code> if any of the AWS accounts in the list have active CloudFront key pairs that CloudFront can use to verify the signatures of signed URLs and signed cookies. If not, this field is <code>false</code>.</p>
    pub enabled: bool,
    /// <p>A list of AWS accounts and the identifiers of active CloudFront key pairs in each account that CloudFront can use to verify the signatures of signed URLs and signed cookies.</p>
    pub items: Option<Vec<Signer>>,
    /// <p>The number of AWS accounts in the list.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct ActiveTrustedSignersDeserializer;
impl ActiveTrustedSignersDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ActiveTrustedSigners, XmlParseError> {
        deserialize_elements::<_, ActiveTrustedSigners, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Enabled" => {
                    obj.enabled = BooleanDeserializer::deserialize("Enabled", stack)?;
                }
                "Items" => {
                    obj.items
                        .get_or_insert(vec![])
                        .extend(SignerListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>AWS services in China customers must file for an Internet Content Provider (ICP) recordal if they want to serve content publicly on an alternate domain name, also known as a CNAME, that they've added to CloudFront. AliasICPRecordal provides the ICP recordal status for CNAMEs associated with distributions. The status is returned in the CloudFront response; you can't configure it yourself.</p> <p>For more information about ICP recordals, see <a href="https://docs.amazonaws.cn/en_us/aws/latest/userguide/accounts-and-credentials.html"> Signup, Accounts, and Credentials</a> in <i>Getting Started with AWS services in China</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AliasICPRecordal {
    /// <p>A domain name associated with a distribution. </p>
    pub cname: Option<String>,
    /// <p><p>The Internet Content Provider (ICP) recordal status for a CNAME. The ICPRecordalStatus is set to APPROVED for all CNAMEs (aliases) in regions outside of China. </p> <p>The status values returned are the following:</p> <ul> <li> <p> <b>APPROVED</b> indicates that the associated CNAME has a valid ICP recordal number. Multiple CNAMEs can be associated with a distribution, and CNAMEs can correspond to different ICP recordals. To be marked as APPROVED, that is, valid to use with China region, a CNAME must have one ICP recordal number associated with it.</p> </li> <li> <p> <b>SUSPENDED</b> indicates that the associated CNAME does not have a valid ICP recordal number.</p> </li> <li> <p> <b>PENDING</b> indicates that CloudFront can&#39;t determine the ICP recordal status of the CNAME associated with the distribution because there was an error in trying to determine the status. You can try again to see if the error is resolved in which case CloudFront returns an APPROVED or SUSPENDED status.</p> </li> </ul></p>
    pub icp_recordal_status: Option<String>,
}

#[allow(dead_code)]
struct AliasICPRecordalDeserializer;
impl AliasICPRecordalDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AliasICPRecordal, XmlParseError> {
        deserialize_elements::<_, AliasICPRecordal, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CNAME" => {
                    obj.cname = Some(StringDeserializer::deserialize("CNAME", stack)?);
                }
                "ICPRecordalStatus" => {
                    obj.icp_recordal_status = Some(ICPRecordalStatusDeserializer::deserialize(
                        "ICPRecordalStatus",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct AliasICPRecordalsDeserializer;
impl AliasICPRecordalsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AliasICPRecordal>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "AliasICPRecordal" {
                obj.push(AliasICPRecordalDeserializer::deserialize(
                    "AliasICPRecordal",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct AliasListDeserializer;
impl AliasListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CNAME" {
                obj.push(StringDeserializer::deserialize("CNAME", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct AliasListSerializer;
impl AliasListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            StringSerializer::serialize(writer, "CNAME", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A complex type that contains information about CNAMEs (alternate domain names), if any, for this distribution. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Aliases {
    /// <p>A complex type that contains the CNAME aliases, if any, that you want to associate with this distribution.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of CNAME aliases, if any, that you want to associate with this distribution.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct AliasesDeserializer;
impl AliasesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Aliases, XmlParseError> {
        deserialize_elements::<_, Aliases, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .get_or_insert(vec![])
                        .extend(AliasListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct AliasesSerializer;
impl AliasesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Aliases,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &AliasListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that controls which HTTP methods CloudFront processes and forwards to your Amazon S3 bucket or your custom origin. There are three choices:</p> <ul> <li> <p>CloudFront forwards only <code>GET</code> and <code>HEAD</code> requests.</p> </li> <li> <p>CloudFront forwards only <code>GET</code>, <code>HEAD</code>, and <code>OPTIONS</code> requests.</p> </li> <li> <p>CloudFront forwards <code>GET, HEAD, OPTIONS, PUT, PATCH, POST</code>, and <code>DELETE</code> requests.</p> </li> </ul> <p>If you pick the third choice, you may need to restrict access to your Amazon S3 bucket or to your custom origin so users can't perform operations that you don't want them to. For example, you might not want users to have permissions to delete objects from your origin.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AllowedMethods {
    pub cached_methods: Option<CachedMethods>,
    /// <p>A complex type that contains the HTTP methods that you want CloudFront to process and forward to your origin.</p>
    pub items: Vec<String>,
    /// <p>The number of HTTP methods that you want CloudFront to forward to your origin. Valid values are 2 (for <code>GET</code> and <code>HEAD</code> requests), 3 (for <code>GET</code>, <code>HEAD</code>, and <code>OPTIONS</code> requests) and 7 (for <code>GET, HEAD, OPTIONS, PUT, PATCH, POST</code>, and <code>DELETE</code> requests).</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct AllowedMethodsDeserializer;
impl AllowedMethodsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AllowedMethods, XmlParseError> {
        deserialize_elements::<_, AllowedMethods, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CachedMethods" => {
                    obj.cached_methods = Some(CachedMethodsDeserializer::deserialize(
                        "CachedMethods",
                        stack,
                    )?);
                }
                "Items" => {
                    obj.items
                        .extend(MethodsListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct AllowedMethodsSerializer;
impl AllowedMethodsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AllowedMethods,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.cached_methods {
            &CachedMethodsSerializer::serialize(&mut writer, "CachedMethods", value)?;
        }
        MethodsListSerializer::serialize(&mut writer, "Items", &obj.items)?;
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct AwsAccountNumberListDeserializer;
impl AwsAccountNumberListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "AwsAccountNumber" {
                obj.push(StringDeserializer::deserialize("AwsAccountNumber", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct AwsAccountNumberListSerializer;
impl AwsAccountNumberListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            StringSerializer::serialize(writer, "AwsAccountNumber", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[allow(dead_code)]
struct BooleanDeserializer;
impl BooleanDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(bool::from_str(&s).unwrap()))
    }
}

pub struct BooleanSerializer;
impl BooleanSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, &obj.to_string())
    }
}

/// <p>A complex type that describes how CloudFront processes requests.</p> <p>You must create at least as many cache behaviors (including the default cache behavior) as you have origins if you want CloudFront to serve objects from all of the origins. Each cache behavior specifies the one origin from which you want CloudFront to get objects. If you have two origins and only the default cache behavior, the default cache behavior will cause CloudFront to get objects from one of the origins, but the other origin is never used.</p> <p>For the current quota (formerly known as limit) on the number of cache behaviors that you can add to a distribution, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>If you don’t want to specify any cache behaviors, include only an empty <code>CacheBehaviors</code> element. Don’t include an empty <code>CacheBehavior</code> element because this is invalid.</p> <p>To delete all cache behaviors in an existing distribution, update the distribution configuration and include only an empty <code>CacheBehaviors</code> element.</p> <p>To add, change, or remove one or more cache behaviors, update the distribution configuration and specify all of the cache behaviors that you want to include in the updated distribution.</p> <p>For more information about cache behaviors, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesCacheBehavior">Cache Behavior Settings</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CacheBehavior {
    pub allowed_methods: Option<AllowedMethods>,
    /// <p>The unique identifier of the cache policy that is attached to this cache behavior. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html#cache-key-create-cache-policy">Creating cache policies</a> or <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-managed-cache-policies.html">Using the managed cache policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub cache_policy_id: Option<String>,
    /// <p>Whether you want CloudFront to automatically compress certain files for this cache behavior. If so, specify true; if not, specify false. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/ServingCompressedFiles.html">Serving Compressed Files</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub compress: Option<bool>,
    /// <p>The value of <code>ID</code> for the field-level encryption configuration that you want CloudFront to use for encrypting specific fields of data for this cache behavior.</p>
    pub field_level_encryption_id: Option<String>,
    /// <p>A complex type that contains zero or more Lambda function associations for a cache behavior.</p>
    pub lambda_function_associations: Option<LambdaFunctionAssociations>,
    /// <p>The unique identifier of the origin request policy that is attached to this cache behavior. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-origin-requests.html#origin-request-create-origin-request-policy">Creating origin request policies</a> or <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-managed-origin-request-policies.html">Using the managed origin request policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub origin_request_policy_id: Option<String>,
    /// <p>The pattern (for example, <code>images/*.jpg</code>) that specifies which requests to apply the behavior to. When CloudFront receives a viewer request, the requested path is compared with path patterns in the order in which cache behaviors are listed in the distribution.</p> <note> <p>You can optionally include a slash (<code>/</code>) at the beginning of the path pattern. For example, <code>/images/*.jpg</code>. CloudFront behavior is the same with or without the leading <code>/</code>.</p> </note> <p>The path pattern for the default cache behavior is <code>*</code> and cannot be changed. If the request for an object does not match the path pattern for any cache behaviors, CloudFront applies the behavior in the default cache behavior.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesPathPattern">Path Pattern</a> in the <i> Amazon CloudFront Developer Guide</i>.</p>
    pub path_pattern: String,
    /// <p>The Amazon Resource Name (ARN) of the real-time log configuration that is attached to this cache behavior. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html">Real-time logs</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub realtime_log_config_arn: Option<String>,
    /// <p>Indicates whether you want to distribute media files in the Microsoft Smooth Streaming format using the origin that is associated with this cache behavior. If so, specify <code>true</code>; if not, specify <code>false</code>. If you specify <code>true</code> for <code>SmoothStreaming</code>, you can still distribute other content using this cache behavior if the content matches the value of <code>PathPattern</code>. </p>
    pub smooth_streaming: Option<bool>,
    /// <p>The value of <code>ID</code> for the origin that you want CloudFront to route requests to when they match this cache behavior.</p>
    pub target_origin_id: String,
    /// <p>A list of key groups that CloudFront can use to validate signed URLs or signed cookies.</p> <p>When a cache behavior contains trusted key groups, CloudFront requires signed URLs or signed cookies for all requests that match the cache behavior. The URLs or cookies must be signed with a private key whose corresponding public key is in the key group. The signed URL or cookie contains information about which public key CloudFront should use to verify the signature. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving private content</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub trusted_key_groups: Option<TrustedKeyGroups>,
    /// <p><important> <p>We recommend using <code>TrustedKeyGroups</code> instead of <code>TrustedSigners</code>.</p> </important> <p>A list of AWS account IDs whose public keys CloudFront can use to validate signed URLs or signed cookies.</p> <p>When a cache behavior contains trusted signers, CloudFront requires signed URLs or signed cookies for all requests that match the cache behavior. The URLs or cookies must be signed with the private key of a CloudFront key pair in the trusted signer’s AWS account. The signed URL or cookie contains information about which public key CloudFront should use to verify the signature. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving private content</a> in the <i>Amazon CloudFront Developer Guide</i>.</p></p>
    pub trusted_signers: Option<TrustedSigners>,
    /// <p><p>The protocol that viewers can use to access the files in the origin specified by <code>TargetOriginId</code> when a request matches the path pattern in <code>PathPattern</code>. You can specify the following options:</p> <ul> <li> <p> <code>allow-all</code>: Viewers can use HTTP or HTTPS.</p> </li> <li> <p> <code>redirect-to-https</code>: If a viewer submits an HTTP request, CloudFront returns an HTTP status code of 301 (Moved Permanently) to the viewer along with the HTTPS URL. The viewer then resubmits the request using the new URL. </p> </li> <li> <p> <code>https-only</code>: If a viewer sends an HTTP request, CloudFront returns an HTTP status code of 403 (Forbidden). </p> </li> </ul> <p>For more information about requiring the HTTPS protocol, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-https-viewers-to-cloudfront.html">Requiring HTTPS Between Viewers and CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <note> <p>The only way to guarantee that viewers retrieve an object that was fetched from the origin using HTTPS is never to use any other protocol to fetch the object. If you have recently changed from HTTP to HTTPS, we recommend that you clear your objects’ cache because cached objects are protocol agnostic. That means that an edge location will return an object from the cache regardless of whether the current request protocol matches the protocol used previously. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Managing Cache Expiration</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </note></p>
    pub viewer_protocol_policy: String,
}

#[allow(dead_code)]
struct CacheBehaviorDeserializer;
impl CacheBehaviorDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheBehavior, XmlParseError> {
        deserialize_elements::<_, CacheBehavior, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AllowedMethods" => {
                    obj.allowed_methods = Some(AllowedMethodsDeserializer::deserialize(
                        "AllowedMethods",
                        stack,
                    )?);
                }
                "CachePolicyId" => {
                    obj.cache_policy_id =
                        Some(StringDeserializer::deserialize("CachePolicyId", stack)?);
                }
                "Compress" => {
                    obj.compress = Some(BooleanDeserializer::deserialize("Compress", stack)?);
                }
                "FieldLevelEncryptionId" => {
                    obj.field_level_encryption_id = Some(StringDeserializer::deserialize(
                        "FieldLevelEncryptionId",
                        stack,
                    )?);
                }
                "LambdaFunctionAssociations" => {
                    obj.lambda_function_associations =
                        Some(LambdaFunctionAssociationsDeserializer::deserialize(
                            "LambdaFunctionAssociations",
                            stack,
                        )?);
                }
                "OriginRequestPolicyId" => {
                    obj.origin_request_policy_id = Some(StringDeserializer::deserialize(
                        "OriginRequestPolicyId",
                        stack,
                    )?);
                }
                "PathPattern" => {
                    obj.path_pattern = StringDeserializer::deserialize("PathPattern", stack)?;
                }
                "RealtimeLogConfigArn" => {
                    obj.realtime_log_config_arn = Some(StringDeserializer::deserialize(
                        "RealtimeLogConfigArn",
                        stack,
                    )?);
                }
                "SmoothStreaming" => {
                    obj.smooth_streaming =
                        Some(BooleanDeserializer::deserialize("SmoothStreaming", stack)?);
                }
                "TargetOriginId" => {
                    obj.target_origin_id =
                        StringDeserializer::deserialize("TargetOriginId", stack)?;
                }
                "TrustedKeyGroups" => {
                    obj.trusted_key_groups = Some(TrustedKeyGroupsDeserializer::deserialize(
                        "TrustedKeyGroups",
                        stack,
                    )?);
                }
                "TrustedSigners" => {
                    obj.trusted_signers = Some(TrustedSignersDeserializer::deserialize(
                        "TrustedSigners",
                        stack,
                    )?);
                }
                "ViewerProtocolPolicy" => {
                    obj.viewer_protocol_policy = ViewerProtocolPolicyDeserializer::deserialize(
                        "ViewerProtocolPolicy",
                        stack,
                    )?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct CacheBehaviorSerializer;
impl CacheBehaviorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CacheBehavior,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.allowed_methods {
            &AllowedMethodsSerializer::serialize(&mut writer, "AllowedMethods", value)?;
        }
        if let Some(ref value) = obj.cache_policy_id {
            write_characters_element(writer, "CachePolicyId", &value.to_string())?;
        }
        if let Some(ref value) = obj.compress {
            write_characters_element(writer, "Compress", &value.to_string())?;
        }
        if let Some(ref value) = obj.field_level_encryption_id {
            write_characters_element(writer, "FieldLevelEncryptionId", &value.to_string())?;
        }
        if let Some(ref value) = obj.lambda_function_associations {
            &LambdaFunctionAssociationsSerializer::serialize(
                &mut writer,
                "LambdaFunctionAssociations",
                value,
            )?;
        }
        if let Some(ref value) = obj.origin_request_policy_id {
            write_characters_element(writer, "OriginRequestPolicyId", &value.to_string())?;
        }
        write_characters_element(writer, "PathPattern", &obj.path_pattern.to_string())?;
        if let Some(ref value) = obj.realtime_log_config_arn {
            write_characters_element(writer, "RealtimeLogConfigArn", &value.to_string())?;
        }
        if let Some(ref value) = obj.smooth_streaming {
            write_characters_element(writer, "SmoothStreaming", &value.to_string())?;
        }
        write_characters_element(writer, "TargetOriginId", &obj.target_origin_id.to_string())?;
        if let Some(ref value) = obj.trusted_key_groups {
            &TrustedKeyGroupsSerializer::serialize(&mut writer, "TrustedKeyGroups", value)?;
        }
        if let Some(ref value) = obj.trusted_signers {
            &TrustedSignersSerializer::serialize(&mut writer, "TrustedSigners", value)?;
        }
        write_characters_element(
            writer,
            "ViewerProtocolPolicy",
            &obj.viewer_protocol_policy.to_string(),
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct CacheBehaviorListDeserializer;
impl CacheBehaviorListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CacheBehavior>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CacheBehavior" {
                obj.push(CacheBehaviorDeserializer::deserialize(
                    "CacheBehavior",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct CacheBehaviorListSerializer;
impl CacheBehaviorListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<CacheBehavior>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            CacheBehaviorSerializer::serialize(writer, "CacheBehavior", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A complex type that contains zero or more <code>CacheBehavior</code> elements. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CacheBehaviors {
    /// <p>Optional: A complex type that contains cache behaviors for this distribution. If <code>Quantity</code> is <code>0</code>, you can omit <code>Items</code>.</p>
    pub items: Option<Vec<CacheBehavior>>,
    /// <p>The number of cache behaviors for this distribution. </p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct CacheBehaviorsDeserializer;
impl CacheBehaviorsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheBehaviors, XmlParseError> {
        deserialize_elements::<_, CacheBehaviors, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .get_or_insert(vec![])
                        .extend(CacheBehaviorListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct CacheBehaviorsSerializer;
impl CacheBehaviorsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CacheBehaviors,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &CacheBehaviorListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A cache policy.</p> <p>When it’s attached to a cache behavior, the cache policy determines the following:</p> <ul> <li> <p>The values that CloudFront includes in the cache key. These values can include HTTP headers, cookies, and URL query strings. CloudFront uses the cache key to find an object in its cache that it can return to the viewer.</p> </li> <li> <p>The default, minimum, and maximum time to live (TTL) values that you want objects to stay in the CloudFront cache.</p> </li> </ul> <p>The headers, cookies, and query strings that are included in the cache key are automatically included in requests that CloudFront sends to the origin. CloudFront sends a request when it can’t find a valid object in its cache that matches the request’s cache key. If you want to send values to the origin but <i>not</i> include them in the cache key, use <code>OriginRequestPolicy</code>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CachePolicy {
    /// <p>The cache policy configuration.</p>
    pub cache_policy_config: CachePolicyConfig,
    /// <p>The unique identifier for the cache policy.</p>
    pub id: String,
    /// <p>The date and time when the cache policy was last modified.</p>
    pub last_modified_time: String,
}

#[allow(dead_code)]
struct CachePolicyDeserializer;
impl CachePolicyDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CachePolicy, XmlParseError> {
        deserialize_elements::<_, CachePolicy, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CachePolicyConfig" => {
                    obj.cache_policy_config =
                        CachePolicyConfigDeserializer::deserialize("CachePolicyConfig", stack)?;
                }
                "Id" => {
                    obj.id = StringDeserializer::deserialize("Id", stack)?;
                }
                "LastModifiedTime" => {
                    obj.last_modified_time =
                        TimestampDeserializer::deserialize("LastModifiedTime", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A cache policy configuration.</p> <p>This configuration determines the following:</p> <ul> <li> <p>The values that CloudFront includes in the cache key. These values can include HTTP headers, cookies, and URL query strings. CloudFront uses the cache key to find an object in its cache that it can return to the viewer.</p> </li> <li> <p>The default, minimum, and maximum time to live (TTL) values that you want objects to stay in the CloudFront cache.</p> </li> </ul> <p>The headers, cookies, and query strings that are included in the cache key are automatically included in requests that CloudFront sends to the origin. CloudFront sends a request when it can’t find a valid object in its cache that matches the request’s cache key. If you want to send values to the origin but <i>not</i> include them in the cache key, use <code>OriginRequestPolicy</code>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CachePolicyConfig {
    /// <p>A comment to describe the cache policy.</p>
    pub comment: Option<String>,
    /// <p>The default amount of time, in seconds, that you want objects to stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated. CloudFront uses this value as the object’s time to live (TTL) only when the origin does <i>not</i> send <code>Cache-Control</code> or <code>Expires</code> headers with the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Managing How Long Content Stays in an Edge Cache (Expiration)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>The default value for this field is 86400 seconds (one day). If the value of <code>MinTTL</code> is more than 86400 seconds, then the default value for this field is the same as the value of <code>MinTTL</code>.</p>
    pub default_ttl: Option<i64>,
    /// <p>The maximum amount of time, in seconds, that objects stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated. CloudFront uses this value only when the origin sends <code>Cache-Control</code> or <code>Expires</code> headers with the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Managing How Long Content Stays in an Edge Cache (Expiration)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>The default value for this field is 31536000 seconds (one year). If the value of <code>MinTTL</code> or <code>DefaultTTL</code> is more than 31536000 seconds, then the default value for this field is the same as the value of <code>DefaultTTL</code>.</p>
    pub max_ttl: Option<i64>,
    /// <p>The minimum amount of time, in seconds, that you want objects to stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Managing How Long Content Stays in an Edge Cache (Expiration)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub min_ttl: i64,
    /// <p>A unique name to identify the cache policy.</p>
    pub name: String,
    /// <p>The HTTP headers, cookies, and URL query strings to include in the cache key. The values included in the cache key are automatically included in requests that CloudFront sends to the origin.</p>
    pub parameters_in_cache_key_and_forwarded_to_origin:
        Option<ParametersInCacheKeyAndForwardedToOrigin>,
}

#[allow(dead_code)]
struct CachePolicyConfigDeserializer;
impl CachePolicyConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CachePolicyConfig, XmlParseError> {
        deserialize_elements::<_, CachePolicyConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Comment" => {
                    obj.comment = Some(StringDeserializer::deserialize("Comment", stack)?);
                }
                "DefaultTTL" => {
                    obj.default_ttl = Some(LongDeserializer::deserialize("DefaultTTL", stack)?);
                }
                "MaxTTL" => {
                    obj.max_ttl = Some(LongDeserializer::deserialize("MaxTTL", stack)?);
                }
                "MinTTL" => {
                    obj.min_ttl = LongDeserializer::deserialize("MinTTL", stack)?;
                }
                "Name" => {
                    obj.name = StringDeserializer::deserialize("Name", stack)?;
                }
                "ParametersInCacheKeyAndForwardedToOrigin" => {
                    obj.parameters_in_cache_key_and_forwarded_to_origin = Some(
                        ParametersInCacheKeyAndForwardedToOriginDeserializer::deserialize(
                            "ParametersInCacheKeyAndForwardedToOrigin",
                            stack,
                        )?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct CachePolicyConfigSerializer;
impl CachePolicyConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CachePolicyConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.comment {
            write_characters_element(writer, "Comment", &value.to_string())?;
        }
        if let Some(ref value) = obj.default_ttl {
            write_characters_element(writer, "DefaultTTL", &value.to_string())?;
        }
        if let Some(ref value) = obj.max_ttl {
            write_characters_element(writer, "MaxTTL", &value.to_string())?;
        }
        write_characters_element(writer, "MinTTL", &obj.min_ttl.to_string())?;
        write_characters_element(writer, "Name", &obj.name.to_string())?;
        if let Some(ref value) = obj.parameters_in_cache_key_and_forwarded_to_origin {
            &ParametersInCacheKeyAndForwardedToOriginSerializer::serialize(
                &mut writer,
                "ParametersInCacheKeyAndForwardedToOrigin",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct CachePolicyCookieBehaviorDeserializer;
impl CachePolicyCookieBehaviorDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct CachePolicyCookieBehaviorSerializer;
impl CachePolicyCookieBehaviorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>An object that determines whether any cookies in viewer requests (and if so, which cookies) are included in the cache key and automatically included in requests that CloudFront sends to the origin.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CachePolicyCookiesConfig {
    /// <p><p>Determines whether any cookies in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values are:</p> <ul> <li> <p> <code>none</code> – Cookies in viewer requests are not included in the cache key and are not automatically included in requests that CloudFront sends to the origin. Even when this field is set to <code>none</code>, any cookies that are listed in an <code>OriginRequestPolicy</code> <i>are</i> included in origin requests.</p> </li> <li> <p> <code>whitelist</code> – The cookies in viewer requests that are listed in the <code>CookieNames</code> type are included in the cache key and automatically included in requests that CloudFront sends to the origin.</p> </li> <li> <p> <code>allExcept</code> – All cookies in viewer requests that are <i> <b>not</b> </i> listed in the <code>CookieNames</code> type are included in the cache key and automatically included in requests that CloudFront sends to the origin.</p> </li> <li> <p> <code>all</code> – All cookies in viewer requests are included in the cache key and are automatically included in requests that CloudFront sends to the origin.</p> </li> </ul></p>
    pub cookie_behavior: String,
    pub cookies: Option<CookieNames>,
}

#[allow(dead_code)]
struct CachePolicyCookiesConfigDeserializer;
impl CachePolicyCookiesConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CachePolicyCookiesConfig, XmlParseError> {
        deserialize_elements::<_, CachePolicyCookiesConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CookieBehavior" => {
                        obj.cookie_behavior = CachePolicyCookieBehaviorDeserializer::deserialize(
                            "CookieBehavior",
                            stack,
                        )?;
                    }
                    "Cookies" => {
                        obj.cookies = Some(CookieNamesDeserializer::deserialize("Cookies", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct CachePolicyCookiesConfigSerializer;
impl CachePolicyCookiesConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CachePolicyCookiesConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "CookieBehavior", &obj.cookie_behavior.to_string())?;
        if let Some(ref value) = obj.cookies {
            &CookieNamesSerializer::serialize(&mut writer, "Cookies", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct CachePolicyHeaderBehaviorDeserializer;
impl CachePolicyHeaderBehaviorDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct CachePolicyHeaderBehaviorSerializer;
impl CachePolicyHeaderBehaviorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>An object that determines whether any HTTP headers (and if so, which headers) are included in the cache key and automatically included in requests that CloudFront sends to the origin.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CachePolicyHeadersConfig {
    /// <p><p>Determines whether any HTTP headers are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values are:</p> <ul> <li> <p> <code>none</code> – HTTP headers are not included in the cache key and are not automatically included in requests that CloudFront sends to the origin. Even when this field is set to <code>none</code>, any headers that are listed in an <code>OriginRequestPolicy</code> <i>are</i> included in origin requests.</p> </li> <li> <p> <code>whitelist</code> – The HTTP headers that are listed in the <code>Headers</code> type are included in the cache key and are automatically included in requests that CloudFront sends to the origin.</p> </li> </ul></p>
    pub header_behavior: String,
    pub headers: Option<Headers>,
}

#[allow(dead_code)]
struct CachePolicyHeadersConfigDeserializer;
impl CachePolicyHeadersConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CachePolicyHeadersConfig, XmlParseError> {
        deserialize_elements::<_, CachePolicyHeadersConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HeaderBehavior" => {
                        obj.header_behavior = CachePolicyHeaderBehaviorDeserializer::deserialize(
                            "HeaderBehavior",
                            stack,
                        )?;
                    }
                    "Headers" => {
                        obj.headers = Some(HeadersDeserializer::deserialize("Headers", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct CachePolicyHeadersConfigSerializer;
impl CachePolicyHeadersConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CachePolicyHeadersConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "HeaderBehavior", &obj.header_behavior.to_string())?;
        if let Some(ref value) = obj.headers {
            &HeadersSerializer::serialize(&mut writer, "Headers", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A list of cache policies.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CachePolicyList {
    /// <p>Contains the cache policies in the list.</p>
    pub items: Option<Vec<CachePolicySummary>>,
    /// <p>The maximum number of cache policies requested.</p>
    pub max_items: i64,
    /// <p>If there are more items in the list than are in this response, this element is present. It contains the value that you should use in the <code>Marker</code> field of a subsequent request to continue listing cache policies where you left off.</p>
    pub next_marker: Option<String>,
    /// <p>The total number of cache policies returned in the response.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct CachePolicyListDeserializer;
impl CachePolicyListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CachePolicyList, XmlParseError> {
        deserialize_elements::<_, CachePolicyList, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        CachePolicySummaryListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "MaxItems" => {
                    obj.max_items = IntegerDeserializer::deserialize("MaxItems", stack)?;
                }
                "NextMarker" => {
                    obj.next_marker = Some(StringDeserializer::deserialize("NextMarker", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct CachePolicyQueryStringBehaviorDeserializer;
impl CachePolicyQueryStringBehaviorDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct CachePolicyQueryStringBehaviorSerializer;
impl CachePolicyQueryStringBehaviorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>An object that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the cache key and automatically included in requests that CloudFront sends to the origin.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CachePolicyQueryStringsConfig {
    /// <p><p>Determines whether any URL query strings in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values are:</p> <ul> <li> <p> <code>none</code> – Query strings in viewer requests are not included in the cache key and are not automatically included in requests that CloudFront sends to the origin. Even when this field is set to <code>none</code>, any query strings that are listed in an <code>OriginRequestPolicy</code> <i>are</i> included in origin requests.</p> </li> <li> <p> <code>whitelist</code> – The query strings in viewer requests that are listed in the <code>QueryStringNames</code> type are included in the cache key and automatically included in requests that CloudFront sends to the origin.</p> </li> <li> <p> <code>allExcept</code> – All query strings in viewer requests that are <i> <b>not</b> </i> listed in the <code>QueryStringNames</code> type are included in the cache key and automatically included in requests that CloudFront sends to the origin.</p> </li> <li> <p> <code>all</code> – All query strings in viewer requests are included in the cache key and are automatically included in requests that CloudFront sends to the origin.</p> </li> </ul></p>
    pub query_string_behavior: String,
    /// <p>Contains the specific query strings in viewer requests that either <i> <b>are</b> </i> or <i> <b>are not</b> </i> included in the cache key and automatically included in requests that CloudFront sends to the origin. The behavior depends on whether the <code>QueryStringBehavior</code> field in the <code>CachePolicyQueryStringsConfig</code> type is set to <code>whitelist</code> (the listed query strings <i> <b>are</b> </i> included) or <code>allExcept</code> (the listed query strings <i> <b>are not</b> </i> included, but all other query strings are).</p>
    pub query_strings: Option<QueryStringNames>,
}

#[allow(dead_code)]
struct CachePolicyQueryStringsConfigDeserializer;
impl CachePolicyQueryStringsConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CachePolicyQueryStringsConfig, XmlParseError> {
        deserialize_elements::<_, CachePolicyQueryStringsConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "QueryStringBehavior" => {
                        obj.query_string_behavior =
                            CachePolicyQueryStringBehaviorDeserializer::deserialize(
                                "QueryStringBehavior",
                                stack,
                            )?;
                    }
                    "QueryStrings" => {
                        obj.query_strings = Some(QueryStringNamesDeserializer::deserialize(
                            "QueryStrings",
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

pub struct CachePolicyQueryStringsConfigSerializer;
impl CachePolicyQueryStringsConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CachePolicyQueryStringsConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(
            writer,
            "QueryStringBehavior",
            &obj.query_string_behavior.to_string(),
        )?;
        if let Some(ref value) = obj.query_strings {
            &QueryStringNamesSerializer::serialize(&mut writer, "QueryStrings", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Contains a cache policy.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CachePolicySummary {
    /// <p>The cache policy.</p>
    pub cache_policy: CachePolicy,
    /// <p>The type of cache policy, either <code>managed</code> (created by AWS) or <code>custom</code> (created in this AWS account).</p>
    pub type_: String,
}

#[allow(dead_code)]
struct CachePolicySummaryDeserializer;
impl CachePolicySummaryDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CachePolicySummary, XmlParseError> {
        deserialize_elements::<_, CachePolicySummary, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CachePolicy" => {
                    obj.cache_policy = CachePolicyDeserializer::deserialize("CachePolicy", stack)?;
                }
                "Type" => {
                    obj.type_ = CachePolicyTypeDeserializer::deserialize("Type", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct CachePolicySummaryListDeserializer;
impl CachePolicySummaryListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CachePolicySummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CachePolicySummary" {
                obj.push(CachePolicySummaryDeserializer::deserialize(
                    "CachePolicySummary",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct CachePolicyTypeDeserializer;
impl CachePolicyTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct CachePolicyTypeSerializer;
impl CachePolicyTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>A complex type that controls whether CloudFront caches the response to requests using the specified HTTP methods. There are two choices:</p> <ul> <li> <p>CloudFront caches responses to <code>GET</code> and <code>HEAD</code> requests.</p> </li> <li> <p>CloudFront caches responses to <code>GET</code>, <code>HEAD</code>, and <code>OPTIONS</code> requests.</p> </li> </ul> <p>If you pick the second choice for your Amazon S3 Origin, you may need to forward Access-Control-Request-Method, Access-Control-Request-Headers, and Origin headers for the responses to be cached correctly. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CachedMethods {
    /// <p>A complex type that contains the HTTP methods that you want CloudFront to cache responses to.</p>
    pub items: Vec<String>,
    /// <p>The number of HTTP methods for which you want CloudFront to cache responses. Valid values are <code>2</code> (for caching responses to <code>GET</code> and <code>HEAD</code> requests) and <code>3</code> (for caching responses to <code>GET</code>, <code>HEAD</code>, and <code>OPTIONS</code> requests).</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct CachedMethodsDeserializer;
impl CachedMethodsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CachedMethods, XmlParseError> {
        deserialize_elements::<_, CachedMethods, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .extend(MethodsListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct CachedMethodsSerializer;
impl CachedMethodsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CachedMethods,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        MethodsListSerializer::serialize(&mut writer, "Items", &obj.items)?;
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>CloudFront origin access identity.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CloudFrontOriginAccessIdentity {
    /// <p>The current configuration information for the identity. </p>
    pub cloud_front_origin_access_identity_config: Option<CloudFrontOriginAccessIdentityConfig>,
    /// <p>The ID for the origin access identity, for example, <code>E74FTE3AJFJ256A</code>. </p>
    pub id: String,
    /// <p>The Amazon S3 canonical user ID for the origin access identity, used when giving the origin access identity read permission to an object in Amazon S3. </p>
    pub s3_canonical_user_id: String,
}

#[allow(dead_code)]
struct CloudFrontOriginAccessIdentityDeserializer;
impl CloudFrontOriginAccessIdentityDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudFrontOriginAccessIdentity, XmlParseError> {
        deserialize_elements::<_, CloudFrontOriginAccessIdentity, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CloudFrontOriginAccessIdentityConfig" => {
                        obj.cloud_front_origin_access_identity_config = Some(
                            CloudFrontOriginAccessIdentityConfigDeserializer::deserialize(
                                "CloudFrontOriginAccessIdentityConfig",
                                stack,
                            )?,
                        );
                    }
                    "Id" => {
                        obj.id = StringDeserializer::deserialize("Id", stack)?;
                    }
                    "S3CanonicalUserId" => {
                        obj.s3_canonical_user_id =
                            StringDeserializer::deserialize("S3CanonicalUserId", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Origin access identity configuration. Send a <code>GET</code> request to the <code>/<i>CloudFront API version</i>/CloudFront/identity ID/config</code> resource. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CloudFrontOriginAccessIdentityConfig {
    /// <p>A unique value (for example, a date-time stamp) that ensures that the request can't be replayed.</p> <p>If the value of <code>CallerReference</code> is new (regardless of the content of the <code>CloudFrontOriginAccessIdentityConfig</code> object), a new origin access identity is created.</p> <p>If the <code>CallerReference</code> is a value already sent in a previous identity request, and the content of the <code>CloudFrontOriginAccessIdentityConfig</code> is identical to the original request (ignoring white space), the response includes the same information returned to the original request. </p> <p>If the <code>CallerReference</code> is a value you already sent in a previous request to create an identity, but the content of the <code>CloudFrontOriginAccessIdentityConfig</code> is different from the original request, CloudFront returns a <code>CloudFrontOriginAccessIdentityAlreadyExists</code> error. </p>
    pub caller_reference: String,
    /// <p>Any comments you want to include about the origin access identity. </p>
    pub comment: String,
}

#[allow(dead_code)]
struct CloudFrontOriginAccessIdentityConfigDeserializer;
impl CloudFrontOriginAccessIdentityConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudFrontOriginAccessIdentityConfig, XmlParseError> {
        deserialize_elements::<_, CloudFrontOriginAccessIdentityConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CallerReference" => {
                        obj.caller_reference =
                            StringDeserializer::deserialize("CallerReference", stack)?;
                    }
                    "Comment" => {
                        obj.comment = StringDeserializer::deserialize("Comment", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct CloudFrontOriginAccessIdentityConfigSerializer;
impl CloudFrontOriginAccessIdentityConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CloudFrontOriginAccessIdentityConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "CallerReference", &obj.caller_reference.to_string())?;
        write_characters_element(writer, "Comment", &obj.comment.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Lists the origin access identities for CloudFront.Send a <code>GET</code> request to the <code>/<i>CloudFront API version</i>/origin-access-identity/cloudfront</code> resource. The response includes a <code>CloudFrontOriginAccessIdentityList</code> element with zero or more <code>CloudFrontOriginAccessIdentitySummary</code> child elements. By default, your entire list of origin access identities is returned in one single page. If the list is long, you can paginate it using the <code>MaxItems</code> and <code>Marker</code> parameters.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CloudFrontOriginAccessIdentityList {
    /// <p>A flag that indicates whether more origin access identities remain to be listed. If your results were truncated, you can make a follow-up pagination request using the <code>Marker</code> request parameter to retrieve more items in the list.</p>
    pub is_truncated: bool,
    /// <p>A complex type that contains one <code>CloudFrontOriginAccessIdentitySummary</code> element for each origin access identity that was created by the current AWS account.</p>
    pub items: Option<Vec<CloudFrontOriginAccessIdentitySummary>>,
    /// <p>Use this when paginating results to indicate where to begin in your list of origin access identities. The results include identities in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last identity on that page). </p>
    pub marker: String,
    /// <p>The maximum number of origin access identities you want in the response body. </p>
    pub max_items: i64,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value you can use for the <code>Marker</code> request parameter to continue listing your origin access identities where they left off. </p>
    pub next_marker: Option<String>,
    /// <p>The number of CloudFront origin access identities that were created by the current AWS account. </p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct CloudFrontOriginAccessIdentityListDeserializer;
impl CloudFrontOriginAccessIdentityListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudFrontOriginAccessIdentityList, XmlParseError> {
        deserialize_elements::<_, CloudFrontOriginAccessIdentityList, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IsTruncated" => {
                        obj.is_truncated = BooleanDeserializer::deserialize("IsTruncated", stack)?;
                    }
                    "Items" => {
                        obj.items.get_or_insert(vec![]).extend(
                            CloudFrontOriginAccessIdentitySummaryListDeserializer::deserialize(
                                "Items", stack,
                            )?,
                        );
                    }
                    "Marker" => {
                        obj.marker = StringDeserializer::deserialize("Marker", stack)?;
                    }
                    "MaxItems" => {
                        obj.max_items = IntegerDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(StringDeserializer::deserialize("NextMarker", stack)?);
                    }
                    "Quantity" => {
                        obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Summary of the information about a CloudFront origin access identity.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CloudFrontOriginAccessIdentitySummary {
    /// <p>The comment for this origin access identity, as originally specified when created.</p>
    pub comment: String,
    /// <p>The ID for the origin access identity. For example: <code>E74FTE3AJFJ256A</code>.</p>
    pub id: String,
    /// <p>The Amazon S3 canonical user ID for the origin access identity, which you use when giving the origin access identity read permission to an object in Amazon S3.</p>
    pub s3_canonical_user_id: String,
}

#[allow(dead_code)]
struct CloudFrontOriginAccessIdentitySummaryDeserializer;
impl CloudFrontOriginAccessIdentitySummaryDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudFrontOriginAccessIdentitySummary, XmlParseError> {
        deserialize_elements::<_, CloudFrontOriginAccessIdentitySummary, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Comment" => {
                        obj.comment = StringDeserializer::deserialize("Comment", stack)?;
                    }
                    "Id" => {
                        obj.id = StringDeserializer::deserialize("Id", stack)?;
                    }
                    "S3CanonicalUserId" => {
                        obj.s3_canonical_user_id =
                            StringDeserializer::deserialize("S3CanonicalUserId", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct CloudFrontOriginAccessIdentitySummaryListDeserializer;
impl CloudFrontOriginAccessIdentitySummaryListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CloudFrontOriginAccessIdentitySummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CloudFrontOriginAccessIdentitySummary" {
                obj.push(
                    CloudFrontOriginAccessIdentitySummaryDeserializer::deserialize(
                        "CloudFrontOriginAccessIdentitySummary",
                        stack,
                    )?,
                );
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct CommentTypeDeserializer;
impl CommentTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct CommentTypeSerializer;
impl CommentTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>A field-level encryption content type profile. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ContentTypeProfile {
    /// <p>The content type for a field-level encryption content type-profile mapping. </p>
    pub content_type: String,
    /// <p>The format for a field-level encryption content type-profile mapping. </p>
    pub format: String,
    /// <p>The profile ID for a field-level encryption content type-profile mapping. </p>
    pub profile_id: Option<String>,
}

#[allow(dead_code)]
struct ContentTypeProfileDeserializer;
impl ContentTypeProfileDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ContentTypeProfile, XmlParseError> {
        deserialize_elements::<_, ContentTypeProfile, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ContentType" => {
                    obj.content_type = StringDeserializer::deserialize("ContentType", stack)?;
                }
                "Format" => {
                    obj.format = FormatDeserializer::deserialize("Format", stack)?;
                }
                "ProfileId" => {
                    obj.profile_id = Some(StringDeserializer::deserialize("ProfileId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct ContentTypeProfileSerializer;
impl ContentTypeProfileSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ContentTypeProfile,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "ContentType", &obj.content_type.to_string())?;
        write_characters_element(writer, "Format", &obj.format.to_string())?;
        if let Some(ref value) = obj.profile_id {
            write_characters_element(writer, "ProfileId", &value.to_string())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>The configuration for a field-level encryption content type-profile mapping. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ContentTypeProfileConfig {
    /// <p>The configuration for a field-level encryption content type-profile. </p>
    pub content_type_profiles: Option<ContentTypeProfiles>,
    /// <p>The setting in a field-level encryption content type-profile mapping that specifies what to do when an unknown content type is provided for the profile. If true, content is forwarded without being encrypted when the content type is unknown. If false (the default), an error is returned when the content type is unknown. </p>
    pub forward_when_content_type_is_unknown: bool,
}

#[allow(dead_code)]
struct ContentTypeProfileConfigDeserializer;
impl ContentTypeProfileConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ContentTypeProfileConfig, XmlParseError> {
        deserialize_elements::<_, ContentTypeProfileConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ContentTypeProfiles" => {
                        obj.content_type_profiles =
                            Some(ContentTypeProfilesDeserializer::deserialize(
                                "ContentTypeProfiles",
                                stack,
                            )?);
                    }
                    "ForwardWhenContentTypeIsUnknown" => {
                        obj.forward_when_content_type_is_unknown =
                            BooleanDeserializer::deserialize(
                                "ForwardWhenContentTypeIsUnknown",
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

pub struct ContentTypeProfileConfigSerializer;
impl ContentTypeProfileConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ContentTypeProfileConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.content_type_profiles {
            &ContentTypeProfilesSerializer::serialize(&mut writer, "ContentTypeProfiles", value)?;
        }
        write_characters_element(
            writer,
            "ForwardWhenContentTypeIsUnknown",
            &obj.forward_when_content_type_is_unknown.to_string(),
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct ContentTypeProfileListDeserializer;
impl ContentTypeProfileListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ContentTypeProfile>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ContentTypeProfile" {
                obj.push(ContentTypeProfileDeserializer::deserialize(
                    "ContentTypeProfile",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct ContentTypeProfileListSerializer;
impl ContentTypeProfileListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<ContentTypeProfile>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            ContentTypeProfileSerializer::serialize(writer, "ContentTypeProfile", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>Field-level encryption content type-profile. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ContentTypeProfiles {
    /// <p>Items in a field-level encryption content type-profile mapping. </p>
    pub items: Option<Vec<ContentTypeProfile>>,
    /// <p>The number of field-level encryption content type-profile mappings. </p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct ContentTypeProfilesDeserializer;
impl ContentTypeProfilesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ContentTypeProfiles, XmlParseError> {
        deserialize_elements::<_, ContentTypeProfiles, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        ContentTypeProfileListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct ContentTypeProfilesSerializer;
impl ContentTypeProfilesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ContentTypeProfiles,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &ContentTypeProfileListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct CookieNameListDeserializer;
impl CookieNameListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Name" {
                obj.push(StringDeserializer::deserialize("Name", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct CookieNameListSerializer;
impl CookieNameListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            StringSerializer::serialize(writer, "Name", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>Contains a list of cookie names.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CookieNames {
    /// <p>A list of cookie names.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of cookie names in the <code>Items</code> list.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct CookieNamesDeserializer;
impl CookieNamesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CookieNames, XmlParseError> {
        deserialize_elements::<_, CookieNames, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .get_or_insert(vec![])
                        .extend(CookieNameListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct CookieNamesSerializer;
impl CookieNamesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CookieNames,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &CookieNameListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>This field is deprecated. We recommend that you use a cache policy or an origin request policy instead of this field.</p> <p>If you want to include cookies in the cache key, use <code>CookiesConfig</code> in a cache policy. See <code>CachePolicy</code>.</p> <p>If you want to send cookies to the origin but not include them in the cache key, use <code>CookiesConfig</code> in an origin request policy. See <code>OriginRequestPolicy</code>.</p> <p>A complex type that specifies whether you want CloudFront to forward cookies to the origin and, if so, which ones. For more information about forwarding cookies to the origin, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Cookies.html">Caching Content Based on Cookies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CookiePreference {
    /// <p>This field is deprecated. We recommend that you use a cache policy or an origin request policy instead of this field.</p> <p>If you want to include cookies in the cache key, use a cache policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html#cache-key-create-cache-policy">Creating cache policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>If you want to send cookies to the origin but not include them in the cache key, use origin request policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-origin-requests.html#origin-request-create-origin-request-policy">Creating origin request policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>Specifies which cookies to forward to the origin for this cache behavior: all, none, or the list of cookies specified in the <code>WhitelistedNames</code> complex type.</p> <p>Amazon S3 doesn't process cookies. When the cache behavior is forwarding requests to an Amazon S3 origin, specify none for the <code>Forward</code> element.</p>
    pub forward: String,
    /// <p>This field is deprecated. We recommend that you use a cache policy or an origin request policy instead of this field.</p> <p>If you want to include cookies in the cache key, use a cache policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html#cache-key-create-cache-policy">Creating cache policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>If you want to send cookies to the origin but not include them in the cache key, use an origin request policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-origin-requests.html#origin-request-create-origin-request-policy">Creating origin request policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>Required if you specify <code>whitelist</code> for the value of <code>Forward</code>. A complex type that specifies how many different cookies you want CloudFront to forward to the origin for this cache behavior and, if you want to forward selected cookies, the names of those cookies.</p> <p>If you specify <code>all</code> or <code>none</code> for the value of <code>Forward</code>, omit <code>WhitelistedNames</code>. If you change the value of <code>Forward</code> from <code>whitelist</code> to <code>all</code> or <code>none</code> and you don't delete the <code>WhitelistedNames</code> element and its child elements, CloudFront deletes them automatically.</p> <p>For the current limit on the number of cookie names that you can whitelist for each cache behavior, see <a href="https://docs.aws.amazon.com/general/latest/gr/xrefaws_service_limits.html#limits_cloudfront"> CloudFront Limits</a> in the <i>AWS General Reference</i>.</p>
    pub whitelisted_names: Option<CookieNames>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCachePolicyRequest {
    /// <p>A cache policy configuration.</p>
    pub cache_policy_config: CachePolicyConfig,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateCachePolicyResult {
    /// <p>A cache policy.</p>
    pub cache_policy: Option<CachePolicy>,
    /// <p>The current version of the cache policy.</p>
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the cache policy just created.</p>
    pub location: Option<String>,
}

#[allow(dead_code)]
struct CreateCachePolicyResultDeserializer;
impl CreateCachePolicyResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateCachePolicyResult, XmlParseError> {
        Ok(CreateCachePolicyResult {
            cache_policy: Some(CachePolicyDeserializer::deserialize("CachePolicy", stack)?),
            ..CreateCachePolicyResult::default()
        })
    }
}
/// <p>The request to create a new origin access identity (OAI). An origin access identity is a special CloudFront user that you can associate with Amazon S3 origins, so that you can secure all or just some of your Amazon S3 content. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-restricting-access-to-s3.html"> Restricting Access to Amazon S3 Content by Using an Origin Access Identity</a> in the <i>Amazon CloudFront Developer Guide</i>. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCloudFrontOriginAccessIdentityRequest {
    /// <p>The current configuration information for the identity.</p>
    pub cloud_front_origin_access_identity_config: CloudFrontOriginAccessIdentityConfig,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateCloudFrontOriginAccessIdentityResult {
    /// <p>The origin access identity's information.</p>
    pub cloud_front_origin_access_identity: Option<CloudFrontOriginAccessIdentity>,
    /// <p>The current version of the origin access identity created.</p>
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the new origin access identity just created.</p>
    pub location: Option<String>,
}

#[allow(dead_code)]
struct CreateCloudFrontOriginAccessIdentityResultDeserializer;
impl CreateCloudFrontOriginAccessIdentityResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateCloudFrontOriginAccessIdentityResult, XmlParseError> {
        Ok(CreateCloudFrontOriginAccessIdentityResult {
            cloud_front_origin_access_identity: Some(
                CloudFrontOriginAccessIdentityDeserializer::deserialize(
                    "CloudFrontOriginAccessIdentity",
                    stack,
                )?,
            ),
            ..CreateCloudFrontOriginAccessIdentityResult::default()
        })
    }
}
/// <p>The request to create a new distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDistributionRequest {
    /// <p>The distribution's configuration information.</p>
    pub distribution_config: DistributionConfig,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateDistributionResult {
    /// <p>The distribution's information.</p>
    pub distribution: Option<Distribution>,
    /// <p>The current version of the distribution created.</p>
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the new distribution resource just created.</p>
    pub location: Option<String>,
}

#[allow(dead_code)]
struct CreateDistributionResultDeserializer;
impl CreateDistributionResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDistributionResult, XmlParseError> {
        Ok(CreateDistributionResult {
            distribution: Some(DistributionDeserializer::deserialize(
                "Distribution",
                stack,
            )?),
            ..CreateDistributionResult::default()
        })
    }
}
/// <p>The request to create a new distribution with tags. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDistributionWithTagsRequest {
    /// <p>The distribution's configuration information. </p>
    pub distribution_config_with_tags: DistributionConfigWithTags,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateDistributionWithTagsResult {
    /// <p>The distribution's information. </p>
    pub distribution: Option<Distribution>,
    /// <p>The current version of the distribution created.</p>
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the new distribution resource just created.</p>
    pub location: Option<String>,
}

#[allow(dead_code)]
struct CreateDistributionWithTagsResultDeserializer;
impl CreateDistributionWithTagsResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDistributionWithTagsResult, XmlParseError> {
        Ok(CreateDistributionWithTagsResult {
            distribution: Some(DistributionDeserializer::deserialize(
                "Distribution",
                stack,
            )?),
            ..CreateDistributionWithTagsResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFieldLevelEncryptionConfigRequest {
    /// <p>The request to create a new field-level encryption configuration.</p>
    pub field_level_encryption_config: FieldLevelEncryptionConfig,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateFieldLevelEncryptionConfigResult {
    /// <p>The current version of the field level encryption configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Returned when you create a new field-level encryption configuration.</p>
    pub field_level_encryption: Option<FieldLevelEncryption>,
    /// <p>The fully qualified URI of the new configuration resource just created.</p>
    pub location: Option<String>,
}

#[allow(dead_code)]
struct CreateFieldLevelEncryptionConfigResultDeserializer;
impl CreateFieldLevelEncryptionConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateFieldLevelEncryptionConfigResult, XmlParseError> {
        Ok(CreateFieldLevelEncryptionConfigResult {
            field_level_encryption: Some(FieldLevelEncryptionDeserializer::deserialize(
                "FieldLevelEncryption",
                stack,
            )?),
            ..CreateFieldLevelEncryptionConfigResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFieldLevelEncryptionProfileRequest {
    /// <p>The request to create a field-level encryption profile.</p>
    pub field_level_encryption_profile_config: FieldLevelEncryptionProfileConfig,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateFieldLevelEncryptionProfileResult {
    /// <p>The current version of the field level encryption profile. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Returned when you create a new field-level encryption profile.</p>
    pub field_level_encryption_profile: Option<FieldLevelEncryptionProfile>,
    /// <p>The fully qualified URI of the new profile resource just created.</p>
    pub location: Option<String>,
}

#[allow(dead_code)]
struct CreateFieldLevelEncryptionProfileResultDeserializer;
impl CreateFieldLevelEncryptionProfileResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateFieldLevelEncryptionProfileResult, XmlParseError> {
        Ok(CreateFieldLevelEncryptionProfileResult {
            field_level_encryption_profile: Some(
                FieldLevelEncryptionProfileDeserializer::deserialize(
                    "FieldLevelEncryptionProfile",
                    stack,
                )?,
            ),
            ..CreateFieldLevelEncryptionProfileResult::default()
        })
    }
}
/// <p>The request to create an invalidation.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInvalidationRequest {
    /// <p>The distribution's id.</p>
    pub distribution_id: String,
    /// <p>The batch information for the invalidation.</p>
    pub invalidation_batch: InvalidationBatch,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateInvalidationResult {
    /// <p>The invalidation's information.</p>
    pub invalidation: Option<Invalidation>,
    /// <p>The fully qualified URI of the distribution and invalidation batch request, including the <code>Invalidation ID</code>.</p>
    pub location: Option<String>,
}

#[allow(dead_code)]
struct CreateInvalidationResultDeserializer;
impl CreateInvalidationResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateInvalidationResult, XmlParseError> {
        Ok(CreateInvalidationResult {
            invalidation: Some(InvalidationDeserializer::deserialize(
                "Invalidation",
                stack,
            )?),
            ..CreateInvalidationResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateKeyGroupRequest {
    /// <p>A key group configuration.</p>
    pub key_group_config: KeyGroupConfig,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateKeyGroupResult {
    /// <p>The identifier for this version of the key group.</p>
    pub e_tag: Option<String>,
    /// <p>The key group that was just created.</p>
    pub key_group: Option<KeyGroup>,
    /// <p>The URL of the key group.</p>
    pub location: Option<String>,
}

#[allow(dead_code)]
struct CreateKeyGroupResultDeserializer;
impl CreateKeyGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateKeyGroupResult, XmlParseError> {
        Ok(CreateKeyGroupResult {
            key_group: Some(KeyGroupDeserializer::deserialize("KeyGroup", stack)?),
            ..CreateKeyGroupResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMonitoringSubscriptionRequest {
    /// <p>The ID of the distribution that you are enabling metrics for.</p>
    pub distribution_id: String,
    /// <p>A monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.</p>
    pub monitoring_subscription: MonitoringSubscription,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateMonitoringSubscriptionResult {
    /// <p>A monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.</p>
    pub monitoring_subscription: Option<MonitoringSubscription>,
}

#[allow(dead_code)]
struct CreateMonitoringSubscriptionResultDeserializer;
impl CreateMonitoringSubscriptionResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateMonitoringSubscriptionResult, XmlParseError> {
        Ok(CreateMonitoringSubscriptionResult {
            monitoring_subscription: Some(MonitoringSubscriptionDeserializer::deserialize(
                "MonitoringSubscription",
                stack,
            )?),
            ..CreateMonitoringSubscriptionResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateOriginRequestPolicyRequest {
    /// <p>An origin request policy configuration.</p>
    pub origin_request_policy_config: OriginRequestPolicyConfig,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateOriginRequestPolicyResult {
    /// <p>The current version of the origin request policy.</p>
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the origin request policy just created.</p>
    pub location: Option<String>,
    /// <p>An origin request policy.</p>
    pub origin_request_policy: Option<OriginRequestPolicy>,
}

#[allow(dead_code)]
struct CreateOriginRequestPolicyResultDeserializer;
impl CreateOriginRequestPolicyResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateOriginRequestPolicyResult, XmlParseError> {
        Ok(CreateOriginRequestPolicyResult {
            origin_request_policy: Some(OriginRequestPolicyDeserializer::deserialize(
                "OriginRequestPolicy",
                stack,
            )?),
            ..CreateOriginRequestPolicyResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePublicKeyRequest {
    /// <p>A CloudFront public key configuration.</p>
    pub public_key_config: PublicKeyConfig,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreatePublicKeyResult {
    /// <p>The identifier for this version of the public key.</p>
    pub e_tag: Option<String>,
    /// <p>The URL of the public key.</p>
    pub location: Option<String>,
    /// <p>The public key.</p>
    pub public_key: Option<PublicKey>,
}

#[allow(dead_code)]
struct CreatePublicKeyResultDeserializer;
impl CreatePublicKeyResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreatePublicKeyResult, XmlParseError> {
        Ok(CreatePublicKeyResult {
            public_key: Some(PublicKeyDeserializer::deserialize("PublicKey", stack)?),
            ..CreatePublicKeyResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRealtimeLogConfigRequest {
    /// <p>Contains information about the Amazon Kinesis data stream where you are sending real-time log data.</p>
    pub end_points: Vec<EndPoint>,
    /// <p>A list of fields to include in each real-time log record.</p> <p>For more information about fields, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields">Real-time log configuration fields</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub fields: Vec<String>,
    /// <p>A unique name to identify this real-time log configuration.</p>
    pub name: String,
    /// <p>The sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. You must provide an integer between 1 and 100, inclusive.</p>
    pub sampling_rate: i64,
}

pub struct CreateRealtimeLogConfigRequestSerializer;
impl CreateRealtimeLogConfigRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CreateRealtimeLogConfigRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        EndPointListSerializer::serialize(&mut writer, "EndPoints", &obj.end_points)?;
        FieldListSerializer::serialize(&mut writer, "Fields", &obj.fields)?;
        StringSerializer::serialize(&mut writer, "Name", &obj.name)?;
        LongSerializer::serialize(&mut writer, "SamplingRate", &obj.sampling_rate)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateRealtimeLogConfigResult {
    /// <p>A real-time log configuration.</p>
    pub realtime_log_config: Option<RealtimeLogConfig>,
}

#[allow(dead_code)]
struct CreateRealtimeLogConfigResultDeserializer;
impl CreateRealtimeLogConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateRealtimeLogConfigResult, XmlParseError> {
        deserialize_elements::<_, CreateRealtimeLogConfigResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "RealtimeLogConfig" => {
                        obj.realtime_log_config = Some(RealtimeLogConfigDeserializer::deserialize(
                            "RealtimeLogConfig",
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
/// <p>The request to create a new streaming distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateStreamingDistributionRequest {
    /// <p>The streaming distribution's configuration information.</p>
    pub streaming_distribution_config: StreamingDistributionConfig,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateStreamingDistributionResult {
    /// <p>The current version of the streaming distribution created.</p>
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the new streaming distribution resource just created.</p>
    pub location: Option<String>,
    /// <p>The streaming distribution's information.</p>
    pub streaming_distribution: Option<StreamingDistribution>,
}

#[allow(dead_code)]
struct CreateStreamingDistributionResultDeserializer;
impl CreateStreamingDistributionResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateStreamingDistributionResult, XmlParseError> {
        Ok(CreateStreamingDistributionResult {
            streaming_distribution: Some(StreamingDistributionDeserializer::deserialize(
                "StreamingDistribution",
                stack,
            )?),
            ..CreateStreamingDistributionResult::default()
        })
    }
}
/// <p>The request to create a new streaming distribution with tags.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateStreamingDistributionWithTagsRequest {
    /// <p> The streaming distribution's configuration information. </p>
    pub streaming_distribution_config_with_tags: StreamingDistributionConfigWithTags,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateStreamingDistributionWithTagsResult {
    /// <p>The current version of the distribution created.</p>
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the new streaming distribution resource just created.</p>
    pub location: Option<String>,
    /// <p>The streaming distribution's information. </p>
    pub streaming_distribution: Option<StreamingDistribution>,
}

#[allow(dead_code)]
struct CreateStreamingDistributionWithTagsResultDeserializer;
impl CreateStreamingDistributionWithTagsResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateStreamingDistributionWithTagsResult, XmlParseError> {
        Ok(CreateStreamingDistributionWithTagsResult {
            streaming_distribution: Some(StreamingDistributionDeserializer::deserialize(
                "StreamingDistribution",
                stack,
            )?),
            ..CreateStreamingDistributionWithTagsResult::default()
        })
    }
}
/// <p>A complex type that controls:</p> <ul> <li> <p>Whether CloudFront replaces HTTP status codes in the 4xx and 5xx range with custom error messages before returning the response to the viewer. </p> </li> <li> <p>How long CloudFront caches HTTP status codes in the 4xx and 5xx range.</p> </li> </ul> <p>For more information about custom error pages, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/custom-error-pages.html">Customizing Error Responses</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CustomErrorResponse {
    /// <p>The minimum amount of time, in seconds, that you want CloudFront to cache the HTTP status code specified in <code>ErrorCode</code>. When this time period has elapsed, CloudFront queries your origin to see whether the problem that caused the error has been resolved and the requested object is now available.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/custom-error-pages.html">Customizing Error Responses</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub error_caching_min_ttl: Option<i64>,
    /// <p>The HTTP status code for which you want to specify a custom error page and/or a caching duration.</p>
    pub error_code: i64,
    /// <p>The HTTP status code that you want CloudFront to return to the viewer along with the custom error page. There are a variety of reasons that you might want CloudFront to return a status code different from the status code that your origin returned to CloudFront, for example:</p> <ul> <li> <p>Some Internet devices (some firewalls and corporate proxies, for example) intercept HTTP 4xx and 5xx and prevent the response from being returned to the viewer. If you substitute <code>200</code>, the response typically won't be intercepted.</p> </li> <li> <p>If you don't care about distinguishing among different client errors or server errors, you can specify <code>400</code> or <code>500</code> as the <code>ResponseCode</code> for all 4xx or 5xx errors.</p> </li> <li> <p>You might want to return a <code>200</code> status code (OK) and static website so your customers don't know that your website is down.</p> </li> </ul> <p>If you specify a value for <code>ResponseCode</code>, you must also specify a value for <code>ResponsePagePath</code>.</p>
    pub response_code: Option<String>,
    /// <p>The path to the custom error page that you want CloudFront to return to a viewer when your origin returns the HTTP status code specified by <code>ErrorCode</code>, for example, <code>/4xx-errors/403-forbidden.html</code>. If you want to store your objects and your custom error pages in different locations, your distribution must include a cache behavior for which the following is true:</p> <ul> <li> <p>The value of <code>PathPattern</code> matches the path to your custom error messages. For example, suppose you saved custom error pages for 4xx errors in an Amazon S3 bucket in a directory named <code>/4xx-errors</code>. Your distribution must include a cache behavior for which the path pattern routes requests for your custom error pages to that location, for example, <code>/4xx-errors/*</code>. </p> </li> <li> <p>The value of <code>TargetOriginId</code> specifies the value of the <code>ID</code> element for the origin that contains your custom error pages.</p> </li> </ul> <p>If you specify a value for <code>ResponsePagePath</code>, you must also specify a value for <code>ResponseCode</code>.</p> <p>We recommend that you store custom error pages in an Amazon S3 bucket. If you store custom error pages on an HTTP server and the server starts to return 5xx errors, CloudFront can't get the files that you want to return to viewers because the origin server is unavailable.</p>
    pub response_page_path: Option<String>,
}

#[allow(dead_code)]
struct CustomErrorResponseDeserializer;
impl CustomErrorResponseDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CustomErrorResponse, XmlParseError> {
        deserialize_elements::<_, CustomErrorResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ErrorCachingMinTTL" => {
                    obj.error_caching_min_ttl =
                        Some(LongDeserializer::deserialize("ErrorCachingMinTTL", stack)?);
                }
                "ErrorCode" => {
                    obj.error_code = IntegerDeserializer::deserialize("ErrorCode", stack)?;
                }
                "ResponseCode" => {
                    obj.response_code =
                        Some(StringDeserializer::deserialize("ResponseCode", stack)?);
                }
                "ResponsePagePath" => {
                    obj.response_page_path =
                        Some(StringDeserializer::deserialize("ResponsePagePath", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct CustomErrorResponseSerializer;
impl CustomErrorResponseSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CustomErrorResponse,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.error_caching_min_ttl {
            write_characters_element(writer, "ErrorCachingMinTTL", &value.to_string())?;
        }
        write_characters_element(writer, "ErrorCode", &obj.error_code.to_string())?;
        if let Some(ref value) = obj.response_code {
            write_characters_element(writer, "ResponseCode", &value.to_string())?;
        }
        if let Some(ref value) = obj.response_page_path {
            write_characters_element(writer, "ResponsePagePath", &value.to_string())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct CustomErrorResponseListDeserializer;
impl CustomErrorResponseListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CustomErrorResponse>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CustomErrorResponse" {
                obj.push(CustomErrorResponseDeserializer::deserialize(
                    "CustomErrorResponse",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct CustomErrorResponseListSerializer;
impl CustomErrorResponseListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<CustomErrorResponse>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            CustomErrorResponseSerializer::serialize(writer, "CustomErrorResponse", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A complex type that controls:</p> <ul> <li> <p>Whether CloudFront replaces HTTP status codes in the 4xx and 5xx range with custom error messages before returning the response to the viewer.</p> </li> <li> <p>How long CloudFront caches HTTP status codes in the 4xx and 5xx range.</p> </li> </ul> <p>For more information about custom error pages, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/custom-error-pages.html">Customizing Error Responses</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CustomErrorResponses {
    /// <p>A complex type that contains a <code>CustomErrorResponse</code> element for each HTTP status code for which you want to specify a custom error page and/or a caching duration. </p>
    pub items: Option<Vec<CustomErrorResponse>>,
    /// <p>The number of HTTP status codes for which you want to specify a custom error page and/or a caching duration. If <code>Quantity</code> is <code>0</code>, you can omit <code>Items</code>.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct CustomErrorResponsesDeserializer;
impl CustomErrorResponsesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CustomErrorResponses, XmlParseError> {
        deserialize_elements::<_, CustomErrorResponses, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        CustomErrorResponseListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct CustomErrorResponsesSerializer;
impl CustomErrorResponsesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CustomErrorResponses,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &CustomErrorResponseListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that contains the list of Custom Headers for each origin. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CustomHeaders {
    /// <p> <b>Optional</b>: A list that contains one <code>OriginCustomHeader</code> element for each custom header that you want CloudFront to forward to the origin. If Quantity is <code>0</code>, omit <code>Items</code>.</p>
    pub items: Option<Vec<OriginCustomHeader>>,
    /// <p>The number of custom headers, if any, for this distribution.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct CustomHeadersDeserializer;
impl CustomHeadersDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CustomHeaders, XmlParseError> {
        deserialize_elements::<_, CustomHeaders, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        OriginCustomHeadersListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct CustomHeadersSerializer;
impl CustomHeadersSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CustomHeaders,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &OriginCustomHeadersListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A custom origin. A custom origin is any origin that is <i>not</i> an Amazon S3 bucket, with one exception. An Amazon S3 bucket that is <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/WebsiteHosting.html">configured with static website hosting</a> <i>is</i> a custom origin.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CustomOriginConfig {
    /// <p>The HTTP port that CloudFront uses to connect to the origin. Specify the HTTP port that the origin listens on.</p>
    pub http_port: i64,
    /// <p>The HTTPS port that CloudFront uses to connect to the origin. Specify the HTTPS port that the origin listens on.</p>
    pub https_port: i64,
    /// <p>Specifies how long, in seconds, CloudFront persists its connection to the origin. The minimum timeout is 1 second, the maximum is 60 seconds, and the default (if you don’t specify otherwise) is 5 seconds.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesOriginKeepaliveTimeout">Origin Keep-alive Timeout</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub origin_keepalive_timeout: Option<i64>,
    /// <p><p>Specifies the protocol (HTTP or HTTPS) that CloudFront uses to connect to the origin. Valid values are:</p> <ul> <li> <p> <code>http-only</code> – CloudFront always uses HTTP to connect to the origin.</p> </li> <li> <p> <code>match-viewer</code> – CloudFront connects to the origin using the same protocol that the viewer used to connect to CloudFront.</p> </li> <li> <p> <code>https-only</code> – CloudFront always uses HTTPS to connect to the origin.</p> </li> </ul></p>
    pub origin_protocol_policy: String,
    /// <p>Specifies how long, in seconds, CloudFront waits for a response from the origin. This is also known as the <i>origin response timeout</i>. The minimum timeout is 1 second, the maximum is 60 seconds, and the default (if you don’t specify otherwise) is 30 seconds.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesOriginResponseTimeout">Origin Response Timeout</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub origin_read_timeout: Option<i64>,
    /// <p>Specifies the minimum SSL/TLS protocol that CloudFront uses when connecting to your origin over HTTPS. Valid values include <code>SSLv3</code>, <code>TLSv1</code>, <code>TLSv1.1</code>, and <code>TLSv1.2</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesOriginSSLProtocols">Minimum Origin SSL Protocol</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub origin_ssl_protocols: Option<OriginSslProtocols>,
}

#[allow(dead_code)]
struct CustomOriginConfigDeserializer;
impl CustomOriginConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CustomOriginConfig, XmlParseError> {
        deserialize_elements::<_, CustomOriginConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "HTTPPort" => {
                    obj.http_port = IntegerDeserializer::deserialize("HTTPPort", stack)?;
                }
                "HTTPSPort" => {
                    obj.https_port = IntegerDeserializer::deserialize("HTTPSPort", stack)?;
                }
                "OriginKeepaliveTimeout" => {
                    obj.origin_keepalive_timeout = Some(IntegerDeserializer::deserialize(
                        "OriginKeepaliveTimeout",
                        stack,
                    )?);
                }
                "OriginProtocolPolicy" => {
                    obj.origin_protocol_policy = OriginProtocolPolicyDeserializer::deserialize(
                        "OriginProtocolPolicy",
                        stack,
                    )?;
                }
                "OriginReadTimeout" => {
                    obj.origin_read_timeout = Some(IntegerDeserializer::deserialize(
                        "OriginReadTimeout",
                        stack,
                    )?);
                }
                "OriginSslProtocols" => {
                    obj.origin_ssl_protocols = Some(OriginSslProtocolsDeserializer::deserialize(
                        "OriginSslProtocols",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct CustomOriginConfigSerializer;
impl CustomOriginConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CustomOriginConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "HTTPPort", &obj.http_port.to_string())?;
        write_characters_element(writer, "HTTPSPort", &obj.https_port.to_string())?;
        if let Some(ref value) = obj.origin_keepalive_timeout {
            write_characters_element(writer, "OriginKeepaliveTimeout", &value.to_string())?;
        }
        write_characters_element(
            writer,
            "OriginProtocolPolicy",
            &obj.origin_protocol_policy.to_string(),
        )?;
        if let Some(ref value) = obj.origin_read_timeout {
            write_characters_element(writer, "OriginReadTimeout", &value.to_string())?;
        }
        if let Some(ref value) = obj.origin_ssl_protocols {
            &OriginSslProtocolsSerializer::serialize(&mut writer, "OriginSslProtocols", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that describes the default cache behavior if you don’t specify a <code>CacheBehavior</code> element or if request URLs don’t match any of the values of <code>PathPattern</code> in <code>CacheBehavior</code> elements. You must create exactly one default cache behavior.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DefaultCacheBehavior {
    pub allowed_methods: Option<AllowedMethods>,
    /// <p>The unique identifier of the cache policy that is attached to the default cache behavior. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html#cache-key-create-cache-policy">Creating cache policies</a> or <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-managed-cache-policies.html">Using the managed cache policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub cache_policy_id: Option<String>,
    /// <p>Whether you want CloudFront to automatically compress certain files for this cache behavior. If so, specify <code>true</code>; if not, specify <code>false</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/ServingCompressedFiles.html">Serving Compressed Files</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub compress: Option<bool>,
    /// <p>The value of <code>ID</code> for the field-level encryption configuration that you want CloudFront to use for encrypting specific fields of data for the default cache behavior.</p>
    pub field_level_encryption_id: Option<String>,
    /// <p>A complex type that contains zero or more Lambda function associations for a cache behavior.</p>
    pub lambda_function_associations: Option<LambdaFunctionAssociations>,
    /// <p>The unique identifier of the origin request policy that is attached to the default cache behavior. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-origin-requests.html#origin-request-create-origin-request-policy">Creating origin request policies</a> or <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-managed-origin-request-policies.html">Using the managed origin request policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub origin_request_policy_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the real-time log configuration that is attached to this cache behavior. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html">Real-time logs</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub realtime_log_config_arn: Option<String>,
    /// <p>Indicates whether you want to distribute media files in the Microsoft Smooth Streaming format using the origin that is associated with this cache behavior. If so, specify <code>true</code>; if not, specify <code>false</code>. If you specify <code>true</code> for <code>SmoothStreaming</code>, you can still distribute other content using this cache behavior if the content matches the value of <code>PathPattern</code>. </p>
    pub smooth_streaming: Option<bool>,
    /// <p>The value of <code>ID</code> for the origin that you want CloudFront to route requests to when they use the default cache behavior.</p>
    pub target_origin_id: String,
    /// <p>A list of key groups that CloudFront can use to validate signed URLs or signed cookies.</p> <p>When a cache behavior contains trusted key groups, CloudFront requires signed URLs or signed cookies for all requests that match the cache behavior. The URLs or cookies must be signed with a private key whose corresponding public key is in the key group. The signed URL or cookie contains information about which public key CloudFront should use to verify the signature. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving private content</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub trusted_key_groups: Option<TrustedKeyGroups>,
    /// <p><important> <p>We recommend using <code>TrustedKeyGroups</code> instead of <code>TrustedSigners</code>.</p> </important> <p>A list of AWS account IDs whose public keys CloudFront can use to validate signed URLs or signed cookies.</p> <p>When a cache behavior contains trusted signers, CloudFront requires signed URLs or signed cookies for all requests that match the cache behavior. The URLs or cookies must be signed with the private key of a CloudFront key pair in a trusted signer’s AWS account. The signed URL or cookie contains information about which public key CloudFront should use to verify the signature. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving private content</a> in the <i>Amazon CloudFront Developer Guide</i>.</p></p>
    pub trusted_signers: Option<TrustedSigners>,
    /// <p><p>The protocol that viewers can use to access the files in the origin specified by <code>TargetOriginId</code> when a request matches the path pattern in <code>PathPattern</code>. You can specify the following options:</p> <ul> <li> <p> <code>allow-all</code>: Viewers can use HTTP or HTTPS.</p> </li> <li> <p> <code>redirect-to-https</code>: If a viewer submits an HTTP request, CloudFront returns an HTTP status code of 301 (Moved Permanently) to the viewer along with the HTTPS URL. The viewer then resubmits the request using the new URL.</p> </li> <li> <p> <code>https-only</code>: If a viewer sends an HTTP request, CloudFront returns an HTTP status code of 403 (Forbidden).</p> </li> </ul> <p>For more information about requiring the HTTPS protocol, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-https-viewers-to-cloudfront.html">Requiring HTTPS Between Viewers and CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <note> <p>The only way to guarantee that viewers retrieve an object that was fetched from the origin using HTTPS is never to use any other protocol to fetch the object. If you have recently changed from HTTP to HTTPS, we recommend that you clear your objects’ cache because cached objects are protocol agnostic. That means that an edge location will return an object from the cache regardless of whether the current request protocol matches the protocol used previously. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Managing Cache Expiration</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </note></p>
    pub viewer_protocol_policy: String,
}

#[allow(dead_code)]
struct DefaultCacheBehaviorDeserializer;
impl DefaultCacheBehaviorDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DefaultCacheBehavior, XmlParseError> {
        deserialize_elements::<_, DefaultCacheBehavior, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AllowedMethods" => {
                    obj.allowed_methods = Some(AllowedMethodsDeserializer::deserialize(
                        "AllowedMethods",
                        stack,
                    )?);
                }
                "CachePolicyId" => {
                    obj.cache_policy_id =
                        Some(StringDeserializer::deserialize("CachePolicyId", stack)?);
                }
                "Compress" => {
                    obj.compress = Some(BooleanDeserializer::deserialize("Compress", stack)?);
                }
                "FieldLevelEncryptionId" => {
                    obj.field_level_encryption_id = Some(StringDeserializer::deserialize(
                        "FieldLevelEncryptionId",
                        stack,
                    )?);
                }
                "LambdaFunctionAssociations" => {
                    obj.lambda_function_associations =
                        Some(LambdaFunctionAssociationsDeserializer::deserialize(
                            "LambdaFunctionAssociations",
                            stack,
                        )?);
                }
                "OriginRequestPolicyId" => {
                    obj.origin_request_policy_id = Some(StringDeserializer::deserialize(
                        "OriginRequestPolicyId",
                        stack,
                    )?);
                }
                "RealtimeLogConfigArn" => {
                    obj.realtime_log_config_arn = Some(StringDeserializer::deserialize(
                        "RealtimeLogConfigArn",
                        stack,
                    )?);
                }
                "SmoothStreaming" => {
                    obj.smooth_streaming =
                        Some(BooleanDeserializer::deserialize("SmoothStreaming", stack)?);
                }
                "TargetOriginId" => {
                    obj.target_origin_id =
                        StringDeserializer::deserialize("TargetOriginId", stack)?;
                }
                "TrustedKeyGroups" => {
                    obj.trusted_key_groups = Some(TrustedKeyGroupsDeserializer::deserialize(
                        "TrustedKeyGroups",
                        stack,
                    )?);
                }
                "TrustedSigners" => {
                    obj.trusted_signers = Some(TrustedSignersDeserializer::deserialize(
                        "TrustedSigners",
                        stack,
                    )?);
                }
                "ViewerProtocolPolicy" => {
                    obj.viewer_protocol_policy = ViewerProtocolPolicyDeserializer::deserialize(
                        "ViewerProtocolPolicy",
                        stack,
                    )?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct DefaultCacheBehaviorSerializer;
impl DefaultCacheBehaviorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &DefaultCacheBehavior,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.allowed_methods {
            &AllowedMethodsSerializer::serialize(&mut writer, "AllowedMethods", value)?;
        }
        if let Some(ref value) = obj.cache_policy_id {
            write_characters_element(writer, "CachePolicyId", &value.to_string())?;
        }
        if let Some(ref value) = obj.compress {
            write_characters_element(writer, "Compress", &value.to_string())?;
        }
        if let Some(ref value) = obj.field_level_encryption_id {
            write_characters_element(writer, "FieldLevelEncryptionId", &value.to_string())?;
        }
        if let Some(ref value) = obj.lambda_function_associations {
            &LambdaFunctionAssociationsSerializer::serialize(
                &mut writer,
                "LambdaFunctionAssociations",
                value,
            )?;
        }
        if let Some(ref value) = obj.origin_request_policy_id {
            write_characters_element(writer, "OriginRequestPolicyId", &value.to_string())?;
        }
        if let Some(ref value) = obj.realtime_log_config_arn {
            write_characters_element(writer, "RealtimeLogConfigArn", &value.to_string())?;
        }
        if let Some(ref value) = obj.smooth_streaming {
            write_characters_element(writer, "SmoothStreaming", &value.to_string())?;
        }
        write_characters_element(writer, "TargetOriginId", &obj.target_origin_id.to_string())?;
        if let Some(ref value) = obj.trusted_key_groups {
            &TrustedKeyGroupsSerializer::serialize(&mut writer, "TrustedKeyGroups", value)?;
        }
        if let Some(ref value) = obj.trusted_signers {
            &TrustedSignersSerializer::serialize(&mut writer, "TrustedSigners", value)?;
        }
        write_characters_element(
            writer,
            "ViewerProtocolPolicy",
            &obj.viewer_protocol_policy.to_string(),
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCachePolicyRequest {
    /// <p>The unique identifier for the cache policy that you are deleting. To get the identifier, you can use <code>ListCachePolicies</code>.</p>
    pub id: String,
    /// <p>The version of the cache policy that you are deleting. The version is the cache policy’s <code>ETag</code> value, which you can get using <code>ListCachePolicies</code>, <code>GetCachePolicy</code>, or <code>GetCachePolicyConfig</code>.</p>
    pub if_match: Option<String>,
}

/// <p>Deletes a origin access identity.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCloudFrontOriginAccessIdentityRequest {
    /// <p>The origin access identity's ID.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header you received from a previous <code>GET</code> or <code>PUT</code> request. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

/// <p>This action deletes a web distribution. To delete a web distribution using the CloudFront API, perform the following steps.</p> <p> <b>To delete a web distribution using the CloudFront API:</b> </p> <ol> <li> <p>Disable the web distribution </p> </li> <li> <p>Submit a <code>GET Distribution Config</code> request to get the current configuration and the <code>Etag</code> header for the distribution.</p> </li> <li> <p>Update the XML document that was returned in the response to your <code>GET Distribution Config</code> request to change the value of <code>Enabled</code> to <code>false</code>.</p> </li> <li> <p>Submit a <code>PUT Distribution Config</code> request to update the configuration for your distribution. In the request body, include the XML document that you updated in Step 3. Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to the <code>PUT Distribution Config</code> request to confirm that the distribution was successfully disabled.</p> </li> <li> <p>Submit a <code>GET Distribution</code> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> </li> <li> <p>Submit a <code>DELETE Distribution</code> request. Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Distribution Config</code> request in Step 6.</p> </li> <li> <p>Review the response to your <code>DELETE Distribution</code> request to confirm that the distribution was successfully deleted.</p> </li> </ol> <p>For information about deleting a distribution using the CloudFront console, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/HowToDeleteDistribution.html">Deleting a Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDistributionRequest {
    /// <p>The distribution ID. </p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when you disabled the distribution. For example: <code>E2QWRUHAPOMQZL</code>. </p>
    pub if_match: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFieldLevelEncryptionConfigRequest {
    /// <p>The ID of the configuration you want to delete from CloudFront.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the configuration identity to delete. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFieldLevelEncryptionProfileRequest {
    /// <p>Request the ID of the profile you want to delete from CloudFront.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the profile to delete. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteKeyGroupRequest {
    /// <p>The identifier of the key group that you are deleting. To get the identifier, use <code>ListKeyGroups</code>.</p>
    pub id: String,
    /// <p>The version of the key group that you are deleting. The version is the key group’s <code>ETag</code> value. To get the <code>ETag</code>, use <code>GetKeyGroup</code> or <code>GetKeyGroupConfig</code>.</p>
    pub if_match: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMonitoringSubscriptionRequest {
    /// <p>The ID of the distribution that you are disabling metrics for.</p>
    pub distribution_id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteMonitoringSubscriptionResult {}

#[allow(dead_code)]
struct DeleteMonitoringSubscriptionResultDeserializer;
impl DeleteMonitoringSubscriptionResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteMonitoringSubscriptionResult, XmlParseError> {
        xml_util::start_element(tag_name, stack)?;

        let obj = DeleteMonitoringSubscriptionResult::default();

        xml_util::end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteOriginRequestPolicyRequest {
    /// <p>The unique identifier for the origin request policy that you are deleting. To get the identifier, you can use <code>ListOriginRequestPolicies</code>.</p>
    pub id: String,
    /// <p>The version of the origin request policy that you are deleting. The version is the origin request policy’s <code>ETag</code> value, which you can get using <code>ListOriginRequestPolicies</code>, <code>GetOriginRequestPolicy</code>, or <code>GetOriginRequestPolicyConfig</code>.</p>
    pub if_match: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePublicKeyRequest {
    /// <p>The ID of the public key you want to remove from CloudFront.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the public key identity to delete. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRealtimeLogConfigRequest {
    /// <p>The Amazon Resource Name (ARN) of the real-time log configuration to delete.</p>
    pub arn: Option<String>,
    /// <p>The name of the real-time log configuration to delete.</p>
    pub name: Option<String>,
}

pub struct DeleteRealtimeLogConfigRequestSerializer;
impl DeleteRealtimeLogConfigRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &DeleteRealtimeLogConfigRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        if let Some(ref value) = obj.arn {
            &StringSerializer::serialize(&mut writer, "ARN", value)?;
        }
        if let Some(ref value) = obj.name {
            &StringSerializer::serialize(&mut writer, "Name", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>The request to delete a streaming distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteStreamingDistributionRequest {
    /// <p>The distribution ID. </p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when you disabled the streaming distribution. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

/// <p>A distribution tells CloudFront where you want content to be delivered from, and the details about how to track and manage content delivery.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Distribution {
    /// <p>The ARN (Amazon Resource Name) for the distribution. For example: <code>arn:aws:cloudfront::123456789012:distribution/EDFDVBD632BHDS5</code>, where <code>123456789012</code> is your AWS account ID.</p>
    pub arn: String,
    /// <p>CloudFront automatically adds this field to the response if you’ve configured a cache behavior in this distribution to serve private content using key groups. This field contains a list of key groups and the public keys in each key group that CloudFront can use to verify the signatures of signed URLs or signed cookies.</p>
    pub active_trusted_key_groups: Option<ActiveTrustedKeyGroups>,
    /// <p><important> <p>We recommend using <code>TrustedKeyGroups</code> instead of <code>TrustedSigners</code>.</p> </important> <p>CloudFront automatically adds this field to the response if you’ve configured a cache behavior in this distribution to serve private content using trusted signers. This field contains a list of AWS account IDs and the active CloudFront key pairs in each account that CloudFront can use to verify the signatures of signed URLs or signed cookies.</p></p>
    pub active_trusted_signers: Option<ActiveTrustedSigners>,
    /// <p>AWS services in China customers must file for an Internet Content Provider (ICP) recordal if they want to serve content publicly on an alternate domain name, also known as a CNAME, that they've added to CloudFront. AliasICPRecordal provides the ICP recordal status for CNAMEs associated with distributions.</p> <p>For more information about ICP recordals, see <a href="https://docs.amazonaws.cn/en_us/aws/latest/userguide/accounts-and-credentials.html"> Signup, Accounts, and Credentials</a> in <i>Getting Started with AWS services in China</i>.</p>
    pub alias_icp_recordals: Option<Vec<AliasICPRecordal>>,
    /// <p>The current configuration information for the distribution. Send a <code>GET</code> request to the <code>/<i>CloudFront API version</i>/distribution ID/config</code> resource.</p>
    pub distribution_config: DistributionConfig,
    /// <p>The domain name corresponding to the distribution, for example, <code>d111111abcdef8.cloudfront.net</code>. </p>
    pub domain_name: String,
    /// <p>The identifier for the distribution. For example: <code>EDFDVBD632BHDS5</code>. </p>
    pub id: String,
    /// <p>The number of invalidation batches currently in progress. </p>
    pub in_progress_invalidation_batches: i64,
    /// <p>The date and time the distribution was last modified. </p>
    pub last_modified_time: String,
    /// <p>This response element indicates the current status of the distribution. When the status is <code>Deployed</code>, the distribution's information is fully propagated to all CloudFront edge locations. </p>
    pub status: String,
}

#[allow(dead_code)]
struct DistributionDeserializer;
impl DistributionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Distribution, XmlParseError> {
        deserialize_elements::<_, Distribution, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = StringDeserializer::deserialize("ARN", stack)?;
                }
                "ActiveTrustedKeyGroups" => {
                    obj.active_trusted_key_groups =
                        Some(ActiveTrustedKeyGroupsDeserializer::deserialize(
                            "ActiveTrustedKeyGroups",
                            stack,
                        )?);
                }
                "ActiveTrustedSigners" => {
                    obj.active_trusted_signers =
                        Some(ActiveTrustedSignersDeserializer::deserialize(
                            "ActiveTrustedSigners",
                            stack,
                        )?);
                }
                "AliasICPRecordals" => {
                    obj.alias_icp_recordals.get_or_insert(vec![]).extend(
                        AliasICPRecordalsDeserializer::deserialize("AliasICPRecordals", stack)?,
                    );
                }
                "DistributionConfig" => {
                    obj.distribution_config =
                        DistributionConfigDeserializer::deserialize("DistributionConfig", stack)?;
                }
                "DomainName" => {
                    obj.domain_name = StringDeserializer::deserialize("DomainName", stack)?;
                }
                "Id" => {
                    obj.id = StringDeserializer::deserialize("Id", stack)?;
                }
                "InProgressInvalidationBatches" => {
                    obj.in_progress_invalidation_batches =
                        IntegerDeserializer::deserialize("InProgressInvalidationBatches", stack)?;
                }
                "LastModifiedTime" => {
                    obj.last_modified_time =
                        TimestampDeserializer::deserialize("LastModifiedTime", stack)?;
                }
                "Status" => {
                    obj.status = StringDeserializer::deserialize("Status", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A distribution configuration.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DistributionConfig {
    /// <p>A complex type that contains information about CNAMEs (alternate domain names), if any, for this distribution.</p>
    pub aliases: Option<Aliases>,
    /// <p>A complex type that contains zero or more <code>CacheBehavior</code> elements. </p>
    pub cache_behaviors: Option<CacheBehaviors>,
    /// <p>A unique value (for example, a date-time stamp) that ensures that the request can't be replayed.</p> <p>If the value of <code>CallerReference</code> is new (regardless of the content of the <code>DistributionConfig</code> object), CloudFront creates a new distribution.</p> <p>If <code>CallerReference</code> is a value that you already sent in a previous request to create a distribution, CloudFront returns a <code>DistributionAlreadyExists</code> error.</p>
    pub caller_reference: String,
    /// <p>Any comments you want to include about the distribution.</p> <p>If you don't want to specify a comment, include an empty <code>Comment</code> element.</p> <p>To delete an existing comment, update the distribution configuration and include an empty <code>Comment</code> element.</p> <p>To add or change a comment, update the distribution configuration and specify the new comment.</p>
    pub comment: String,
    /// <p>A complex type that controls the following:</p> <ul> <li> <p>Whether CloudFront replaces HTTP status codes in the 4xx and 5xx range with custom error messages before returning the response to the viewer.</p> </li> <li> <p>How long CloudFront caches HTTP status codes in the 4xx and 5xx range.</p> </li> </ul> <p>For more information about custom error pages, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/custom-error-pages.html">Customizing Error Responses</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub custom_error_responses: Option<CustomErrorResponses>,
    /// <p>A complex type that describes the default cache behavior if you don't specify a <code>CacheBehavior</code> element or if files don't match any of the values of <code>PathPattern</code> in <code>CacheBehavior</code> elements. You must create exactly one default cache behavior.</p>
    pub default_cache_behavior: DefaultCacheBehavior,
    /// <p>The object that you want CloudFront to request from your origin (for example, <code>index.html</code>) when a viewer requests the root URL for your distribution (<code>http://www.example.com</code>) instead of an object in your distribution (<code>http://www.example.com/product-description.html</code>). Specifying a default root object avoids exposing the contents of your distribution.</p> <p>Specify only the object name, for example, <code>index.html</code>. Don't add a <code>/</code> before the object name.</p> <p>If you don't want to specify a default root object when you create a distribution, include an empty <code>DefaultRootObject</code> element.</p> <p>To delete the default root object from an existing distribution, update the distribution configuration and include an empty <code>DefaultRootObject</code> element.</p> <p>To replace the default root object, update the distribution configuration and specify the new object.</p> <p>For more information about the default root object, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/DefaultRootObject.html">Creating a Default Root Object</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub default_root_object: Option<String>,
    /// <p>From this field, you can enable or disable the selected distribution.</p>
    pub enabled: bool,
    /// <p>(Optional) Specify the maximum HTTP version that you want viewers to use to communicate with CloudFront. The default value for new web distributions is http2. Viewers that don't support HTTP/2 automatically use an earlier HTTP version.</p> <p>For viewers and CloudFront to use HTTP/2, viewers must support TLS 1.2 or later, and must support Server Name Identification (SNI).</p> <p>In general, configuring CloudFront to communicate with viewers using HTTP/2 reduces latency. You can improve performance by optimizing for HTTP/2. For more information, do an Internet search for "http/2 optimization." </p>
    pub http_version: Option<String>,
    /// <p>If you want CloudFront to respond to IPv6 DNS requests with an IPv6 address for your distribution, specify <code>true</code>. If you specify <code>false</code>, CloudFront responds to IPv6 DNS requests with the DNS response code <code>NOERROR</code> and with no IP addresses. This allows viewers to submit a second request, for an IPv4 address for your distribution. </p> <p>In general, you should enable IPv6 if you have users on IPv6 networks who want to access your content. However, if you're using signed URLs or signed cookies to restrict access to your content, and if you're using a custom policy that includes the <code>IpAddress</code> parameter to restrict the IP addresses that can access your content, don't enable IPv6. If you want to restrict access to some content by IP address and not restrict access to other content (or restrict access but not by IP address), you can create two distributions. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-creating-signed-url-custom-policy.html">Creating a Signed URL Using a Custom Policy</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>If you're using an Amazon Route 53 alias resource record set to route traffic to your CloudFront distribution, you need to create a second alias resource record set when both of the following are true:</p> <ul> <li> <p>You enable IPv6 for the distribution</p> </li> <li> <p>You're using alternate domain names in the URLs for your objects</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-to-cloudfront-distribution.html">Routing Traffic to an Amazon CloudFront Web Distribution by Using Your Domain Name</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If you created a CNAME resource record set, either with Amazon Route 53 or with another DNS service, you don't need to make any changes. A CNAME record will route traffic to your distribution regardless of the IP address format of the viewer request.</p>
    pub is_ipv6_enabled: Option<bool>,
    /// <p>A complex type that controls whether access logs are written for the distribution.</p> <p>For more information about logging, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/AccessLogs.html">Access Logs</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub logging: Option<LoggingConfig>,
    /// <p> A complex type that contains information about origin groups for this distribution.</p>
    pub origin_groups: Option<OriginGroups>,
    /// <p>A complex type that contains information about origins for this distribution. </p>
    pub origins: Origins,
    /// <p>The price class that corresponds with the maximum price that you want to pay for CloudFront service. If you specify <code>PriceClass_All</code>, CloudFront responds to requests for your objects from all CloudFront edge locations.</p> <p>If you specify a price class other than <code>PriceClass_All</code>, CloudFront serves your objects from the CloudFront edge location that has the lowest latency among the edge locations in your price class. Viewers who are in or near regions that are excluded from your specified price class may encounter slower performance.</p> <p>For more information about price classes, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PriceClass.html">Choosing the Price Class for a CloudFront Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>. For information about CloudFront pricing, including how price classes (such as Price Class 100) map to CloudFront regions, see <a href="http://aws.amazon.com/cloudfront/pricing/">Amazon CloudFront Pricing</a>.</p>
    pub price_class: Option<String>,
    /// <p>A complex type that identifies ways in which you want to restrict distribution of your content.</p>
    pub restrictions: Option<Restrictions>,
    /// <p>A complex type that determines the distribution’s SSL/TLS configuration for communicating with viewers.</p>
    pub viewer_certificate: Option<ViewerCertificate>,
    /// <p>A unique identifier that specifies the AWS WAF web ACL, if any, to associate with this distribution. To specify a web ACL created using the latest version of AWS WAF, use the ACL ARN, for example <code>arn:aws:wafv2:us-east-1:123456789012:global/webacl/ExampleWebACL/473e64fd-f30b-4765-81a0-62ad96dd167a</code>. To specify a web ACL created using AWS WAF Classic, use the ACL ID, for example <code>473e64fd-f30b-4765-81a0-62ad96dd167a</code>.</p> <p>AWS WAF is a web application firewall that lets you monitor the HTTP and HTTPS requests that are forwarded to CloudFront, and lets you control access to your content. Based on conditions that you specify, such as the IP addresses that requests originate from or the values of query strings, CloudFront responds to requests either with the requested content or with an HTTP 403 status code (Forbidden). You can also configure CloudFront to return a custom error page when a request is blocked. For more information about AWS WAF, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/what-is-aws-waf.html">AWS WAF Developer Guide</a>. </p>
    pub web_acl_id: Option<String>,
}

#[allow(dead_code)]
struct DistributionConfigDeserializer;
impl DistributionConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DistributionConfig, XmlParseError> {
        deserialize_elements::<_, DistributionConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Aliases" => {
                    obj.aliases = Some(AliasesDeserializer::deserialize("Aliases", stack)?);
                }
                "CacheBehaviors" => {
                    obj.cache_behaviors = Some(CacheBehaviorsDeserializer::deserialize(
                        "CacheBehaviors",
                        stack,
                    )?);
                }
                "CallerReference" => {
                    obj.caller_reference =
                        StringDeserializer::deserialize("CallerReference", stack)?;
                }
                "Comment" => {
                    obj.comment = CommentTypeDeserializer::deserialize("Comment", stack)?;
                }
                "CustomErrorResponses" => {
                    obj.custom_error_responses =
                        Some(CustomErrorResponsesDeserializer::deserialize(
                            "CustomErrorResponses",
                            stack,
                        )?);
                }
                "DefaultCacheBehavior" => {
                    obj.default_cache_behavior = DefaultCacheBehaviorDeserializer::deserialize(
                        "DefaultCacheBehavior",
                        stack,
                    )?;
                }
                "DefaultRootObject" => {
                    obj.default_root_object =
                        Some(StringDeserializer::deserialize("DefaultRootObject", stack)?);
                }
                "Enabled" => {
                    obj.enabled = BooleanDeserializer::deserialize("Enabled", stack)?;
                }
                "HttpVersion" => {
                    obj.http_version =
                        Some(HttpVersionDeserializer::deserialize("HttpVersion", stack)?);
                }
                "IsIPV6Enabled" => {
                    obj.is_ipv6_enabled =
                        Some(BooleanDeserializer::deserialize("IsIPV6Enabled", stack)?);
                }
                "Logging" => {
                    obj.logging = Some(LoggingConfigDeserializer::deserialize("Logging", stack)?);
                }
                "OriginGroups" => {
                    obj.origin_groups = Some(OriginGroupsDeserializer::deserialize(
                        "OriginGroups",
                        stack,
                    )?);
                }
                "Origins" => {
                    obj.origins = OriginsDeserializer::deserialize("Origins", stack)?;
                }
                "PriceClass" => {
                    obj.price_class =
                        Some(PriceClassDeserializer::deserialize("PriceClass", stack)?);
                }
                "Restrictions" => {
                    obj.restrictions = Some(RestrictionsDeserializer::deserialize(
                        "Restrictions",
                        stack,
                    )?);
                }
                "ViewerCertificate" => {
                    obj.viewer_certificate = Some(ViewerCertificateDeserializer::deserialize(
                        "ViewerCertificate",
                        stack,
                    )?);
                }
                "WebACLId" => {
                    obj.web_acl_id = Some(StringDeserializer::deserialize("WebACLId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct DistributionConfigSerializer;
impl DistributionConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &DistributionConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.aliases {
            &AliasesSerializer::serialize(&mut writer, "Aliases", value)?;
        }
        if let Some(ref value) = obj.cache_behaviors {
            &CacheBehaviorsSerializer::serialize(&mut writer, "CacheBehaviors", value)?;
        }
        write_characters_element(writer, "CallerReference", &obj.caller_reference.to_string())?;
        write_characters_element(writer, "Comment", &obj.comment.to_string())?;
        if let Some(ref value) = obj.custom_error_responses {
            &CustomErrorResponsesSerializer::serialize(&mut writer, "CustomErrorResponses", value)?;
        }
        DefaultCacheBehaviorSerializer::serialize(
            &mut writer,
            "DefaultCacheBehavior",
            &obj.default_cache_behavior,
        )?;
        if let Some(ref value) = obj.default_root_object {
            write_characters_element(writer, "DefaultRootObject", &value.to_string())?;
        }
        write_characters_element(writer, "Enabled", &obj.enabled.to_string())?;
        if let Some(ref value) = obj.http_version {
            write_characters_element(writer, "HttpVersion", &value.to_string())?;
        }
        if let Some(ref value) = obj.is_ipv6_enabled {
            write_characters_element(writer, "IsIPV6Enabled", &value.to_string())?;
        }
        if let Some(ref value) = obj.logging {
            &LoggingConfigSerializer::serialize(&mut writer, "Logging", value)?;
        }
        if let Some(ref value) = obj.origin_groups {
            &OriginGroupsSerializer::serialize(&mut writer, "OriginGroups", value)?;
        }
        OriginsSerializer::serialize(&mut writer, "Origins", &obj.origins)?;
        if let Some(ref value) = obj.price_class {
            write_characters_element(writer, "PriceClass", &value.to_string())?;
        }
        if let Some(ref value) = obj.restrictions {
            &RestrictionsSerializer::serialize(&mut writer, "Restrictions", value)?;
        }
        if let Some(ref value) = obj.viewer_certificate {
            &ViewerCertificateSerializer::serialize(&mut writer, "ViewerCertificate", value)?;
        }
        if let Some(ref value) = obj.web_acl_id {
            write_characters_element(writer, "WebACLId", &value.to_string())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A distribution Configuration and a list of tags to be associated with the distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DistributionConfigWithTags {
    /// <p>A distribution configuration.</p>
    pub distribution_config: DistributionConfig,
    /// <p>A complex type that contains zero or more <code>Tag</code> elements.</p>
    pub tags: Tags,
}

pub struct DistributionConfigWithTagsSerializer;
impl DistributionConfigWithTagsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &DistributionConfigWithTags,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        DistributionConfigSerializer::serialize(
            &mut writer,
            "DistributionConfig",
            &obj.distribution_config,
        )?;
        TagsSerializer::serialize(&mut writer, "Tags", &obj.tags)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A list of distribution IDs.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DistributionIdList {
    /// <p>A flag that indicates whether more distribution IDs remain to be listed. If your results were truncated, you can make a subsequent request using the <code>Marker</code> request field to retrieve more distribution IDs in the list.</p>
    pub is_truncated: bool,
    /// <p>Contains the distribution IDs in the list.</p>
    pub items: Option<Vec<String>>,
    /// <p>The value provided in the <code>Marker</code> request field.</p>
    pub marker: String,
    /// <p>The maximum number of distribution IDs requested.</p>
    pub max_items: i64,
    /// <p>Contains the value that you should use in the <code>Marker</code> field of a subsequent request to continue listing distribution IDs where you left off.</p>
    pub next_marker: Option<String>,
    /// <p>The total number of distribution IDs returned in the response.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct DistributionIdListDeserializer;
impl DistributionIdListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DistributionIdList, XmlParseError> {
        deserialize_elements::<_, DistributionIdList, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "IsTruncated" => {
                    obj.is_truncated = BooleanDeserializer::deserialize("IsTruncated", stack)?;
                }
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        DistributionIdListSummaryDeserializer::deserialize("Items", stack)?,
                    );
                }
                "Marker" => {
                    obj.marker = StringDeserializer::deserialize("Marker", stack)?;
                }
                "MaxItems" => {
                    obj.max_items = IntegerDeserializer::deserialize("MaxItems", stack)?;
                }
                "NextMarker" => {
                    obj.next_marker = Some(StringDeserializer::deserialize("NextMarker", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct DistributionIdListSummaryDeserializer;
impl DistributionIdListSummaryDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DistributionId" {
                obj.push(StringDeserializer::deserialize("DistributionId", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A distribution list.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DistributionList {
    /// <p>A flag that indicates whether more distributions remain to be listed. If your results were truncated, you can make a follow-up pagination request using the <code>Marker</code> request parameter to retrieve more distributions in the list.</p>
    pub is_truncated: bool,
    /// <p>A complex type that contains one <code>DistributionSummary</code> element for each distribution that was created by the current AWS account.</p>
    pub items: Option<Vec<DistributionSummary>>,
    /// <p>The value you provided for the <code>Marker</code> request parameter.</p>
    pub marker: String,
    /// <p>The value you provided for the <code>MaxItems</code> request parameter.</p>
    pub max_items: i64,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value you can use for the <code>Marker</code> request parameter to continue listing your distributions where they left off. </p>
    pub next_marker: Option<String>,
    /// <p>The number of distributions that were created by the current AWS account. </p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct DistributionListDeserializer;
impl DistributionListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DistributionList, XmlParseError> {
        deserialize_elements::<_, DistributionList, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "IsTruncated" => {
                    obj.is_truncated = BooleanDeserializer::deserialize("IsTruncated", stack)?;
                }
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        DistributionSummaryListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "Marker" => {
                    obj.marker = StringDeserializer::deserialize("Marker", stack)?;
                }
                "MaxItems" => {
                    obj.max_items = IntegerDeserializer::deserialize("MaxItems", stack)?;
                }
                "NextMarker" => {
                    obj.next_marker = Some(StringDeserializer::deserialize("NextMarker", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A summary of the information about a CloudFront distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DistributionSummary {
    /// <p>The ARN (Amazon Resource Name) for the distribution. For example: <code>arn:aws:cloudfront::123456789012:distribution/EDFDVBD632BHDS5</code>, where <code>123456789012</code> is your AWS account ID.</p>
    pub arn: String,
    /// <p>AWS services in China customers must file for an Internet Content Provider (ICP) recordal if they want to serve content publicly on an alternate domain name, also known as a CNAME, that they've added to CloudFront. AliasICPRecordal provides the ICP recordal status for CNAMEs associated with distributions.</p> <p>For more information about ICP recordals, see <a href="https://docs.amazonaws.cn/en_us/aws/latest/userguide/accounts-and-credentials.html"> Signup, Accounts, and Credentials</a> in <i>Getting Started with AWS services in China</i>.</p>
    pub alias_icp_recordals: Option<Vec<AliasICPRecordal>>,
    /// <p>A complex type that contains information about CNAMEs (alternate domain names), if any, for this distribution.</p>
    pub aliases: Aliases,
    /// <p>A complex type that contains zero or more <code>CacheBehavior</code> elements.</p>
    pub cache_behaviors: CacheBehaviors,
    /// <p>The comment originally specified when this distribution was created.</p>
    pub comment: String,
    /// <p>A complex type that contains zero or more <code>CustomErrorResponses</code> elements.</p>
    pub custom_error_responses: CustomErrorResponses,
    /// <p>A complex type that describes the default cache behavior if you don't specify a <code>CacheBehavior</code> element or if files don't match any of the values of <code>PathPattern</code> in <code>CacheBehavior</code> elements. You must create exactly one default cache behavior.</p>
    pub default_cache_behavior: DefaultCacheBehavior,
    /// <p>The domain name that corresponds to the distribution, for example, <code>d111111abcdef8.cloudfront.net</code>.</p>
    pub domain_name: String,
    /// <p>Whether the distribution is enabled to accept user requests for content.</p>
    pub enabled: bool,
    /// <p> Specify the maximum HTTP version that you want viewers to use to communicate with CloudFront. The default value for new web distributions is <code>http2</code>. Viewers that don't support <code>HTTP/2</code> will automatically use an earlier version.</p>
    pub http_version: String,
    /// <p>The identifier for the distribution. For example: <code>EDFDVBD632BHDS5</code>.</p>
    pub id: String,
    /// <p>Whether CloudFront responds to IPv6 DNS requests with an IPv6 address for your distribution.</p>
    pub is_ipv6_enabled: bool,
    /// <p>The date and time the distribution was last modified.</p>
    pub last_modified_time: String,
    /// <p> A complex type that contains information about origin groups for this distribution.</p>
    pub origin_groups: Option<OriginGroups>,
    /// <p>A complex type that contains information about origins for this distribution.</p>
    pub origins: Origins,
    /// <p>A complex type that contains information about price class for this streaming distribution. </p>
    pub price_class: String,
    /// <p>A complex type that identifies ways in which you want to restrict distribution of your content.</p>
    pub restrictions: Restrictions,
    /// <p>The current status of the distribution. When the status is <code>Deployed</code>, the distribution's information is propagated to all CloudFront edge locations.</p>
    pub status: String,
    /// <p>A complex type that determines the distribution’s SSL/TLS configuration for communicating with viewers.</p>
    pub viewer_certificate: ViewerCertificate,
    /// <p>The Web ACL Id (if any) associated with the distribution.</p>
    pub web_acl_id: String,
}

#[allow(dead_code)]
struct DistributionSummaryDeserializer;
impl DistributionSummaryDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DistributionSummary, XmlParseError> {
        deserialize_elements::<_, DistributionSummary, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = StringDeserializer::deserialize("ARN", stack)?;
                }
                "AliasICPRecordals" => {
                    obj.alias_icp_recordals.get_or_insert(vec![]).extend(
                        AliasICPRecordalsDeserializer::deserialize("AliasICPRecordals", stack)?,
                    );
                }
                "Aliases" => {
                    obj.aliases = AliasesDeserializer::deserialize("Aliases", stack)?;
                }
                "CacheBehaviors" => {
                    obj.cache_behaviors =
                        CacheBehaviorsDeserializer::deserialize("CacheBehaviors", stack)?;
                }
                "Comment" => {
                    obj.comment = StringDeserializer::deserialize("Comment", stack)?;
                }
                "CustomErrorResponses" => {
                    obj.custom_error_responses = CustomErrorResponsesDeserializer::deserialize(
                        "CustomErrorResponses",
                        stack,
                    )?;
                }
                "DefaultCacheBehavior" => {
                    obj.default_cache_behavior = DefaultCacheBehaviorDeserializer::deserialize(
                        "DefaultCacheBehavior",
                        stack,
                    )?;
                }
                "DomainName" => {
                    obj.domain_name = StringDeserializer::deserialize("DomainName", stack)?;
                }
                "Enabled" => {
                    obj.enabled = BooleanDeserializer::deserialize("Enabled", stack)?;
                }
                "HttpVersion" => {
                    obj.http_version = HttpVersionDeserializer::deserialize("HttpVersion", stack)?;
                }
                "Id" => {
                    obj.id = StringDeserializer::deserialize("Id", stack)?;
                }
                "IsIPV6Enabled" => {
                    obj.is_ipv6_enabled = BooleanDeserializer::deserialize("IsIPV6Enabled", stack)?;
                }
                "LastModifiedTime" => {
                    obj.last_modified_time =
                        TimestampDeserializer::deserialize("LastModifiedTime", stack)?;
                }
                "OriginGroups" => {
                    obj.origin_groups = Some(OriginGroupsDeserializer::deserialize(
                        "OriginGroups",
                        stack,
                    )?);
                }
                "Origins" => {
                    obj.origins = OriginsDeserializer::deserialize("Origins", stack)?;
                }
                "PriceClass" => {
                    obj.price_class = PriceClassDeserializer::deserialize("PriceClass", stack)?;
                }
                "Restrictions" => {
                    obj.restrictions =
                        RestrictionsDeserializer::deserialize("Restrictions", stack)?;
                }
                "Status" => {
                    obj.status = StringDeserializer::deserialize("Status", stack)?;
                }
                "ViewerCertificate" => {
                    obj.viewer_certificate =
                        ViewerCertificateDeserializer::deserialize("ViewerCertificate", stack)?;
                }
                "WebACLId" => {
                    obj.web_acl_id = StringDeserializer::deserialize("WebACLId", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct DistributionSummaryListDeserializer;
impl DistributionSummaryListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DistributionSummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DistributionSummary" {
                obj.push(DistributionSummaryDeserializer::deserialize(
                    "DistributionSummary",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Complex data type for field-level encryption profiles that includes all of the encryption entities. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EncryptionEntities {
    /// <p>An array of field patterns in a field-level encryption content type-profile mapping. </p>
    pub items: Option<Vec<EncryptionEntity>>,
    /// <p>Number of field pattern items in a field-level encryption content type-profile mapping. </p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct EncryptionEntitiesDeserializer;
impl EncryptionEntitiesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EncryptionEntities, XmlParseError> {
        deserialize_elements::<_, EncryptionEntities, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        EncryptionEntityListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct EncryptionEntitiesSerializer;
impl EncryptionEntitiesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &EncryptionEntities,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &EncryptionEntityListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Complex data type for field-level encryption profiles that includes the encryption key and field pattern specifications. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EncryptionEntity {
    /// <p>Field patterns in a field-level encryption content type profile specify the fields that you want to be encrypted. You can provide the full field name, or any beginning characters followed by a wildcard (*). You can't overlap field patterns. For example, you can't have both ABC* and AB*. Note that field patterns are case-sensitive. </p>
    pub field_patterns: FieldPatterns,
    /// <p>The provider associated with the public key being used for encryption. This value must also be provided with the private key for applications to be able to decrypt data.</p>
    pub provider_id: String,
    /// <p>The public key associated with a set of field-level encryption patterns, to be used when encrypting the fields that match the patterns. </p>
    pub public_key_id: String,
}

#[allow(dead_code)]
struct EncryptionEntityDeserializer;
impl EncryptionEntityDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EncryptionEntity, XmlParseError> {
        deserialize_elements::<_, EncryptionEntity, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "FieldPatterns" => {
                    obj.field_patterns =
                        FieldPatternsDeserializer::deserialize("FieldPatterns", stack)?;
                }
                "ProviderId" => {
                    obj.provider_id = StringDeserializer::deserialize("ProviderId", stack)?;
                }
                "PublicKeyId" => {
                    obj.public_key_id = StringDeserializer::deserialize("PublicKeyId", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct EncryptionEntitySerializer;
impl EncryptionEntitySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &EncryptionEntity,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        FieldPatternsSerializer::serialize(&mut writer, "FieldPatterns", &obj.field_patterns)?;
        write_characters_element(writer, "ProviderId", &obj.provider_id.to_string())?;
        write_characters_element(writer, "PublicKeyId", &obj.public_key_id.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct EncryptionEntityListDeserializer;
impl EncryptionEntityListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EncryptionEntity>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "EncryptionEntity" {
                obj.push(EncryptionEntityDeserializer::deserialize(
                    "EncryptionEntity",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct EncryptionEntityListSerializer;
impl EncryptionEntityListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<EncryptionEntity>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            EncryptionEntitySerializer::serialize(writer, "EncryptionEntity", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>Contains information about the Amazon Kinesis data stream where you are sending real-time log data in a real-time log configuration.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EndPoint {
    /// <p>Contains information about the Amazon Kinesis data stream where you are sending real-time log data.</p>
    pub kinesis_stream_config: Option<KinesisStreamConfig>,
    /// <p>The type of data stream where you are sending real-time log data. The only valid value is <code>Kinesis</code>.</p>
    pub stream_type: String,
}

#[allow(dead_code)]
struct EndPointDeserializer;
impl EndPointDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EndPoint, XmlParseError> {
        deserialize_elements::<_, EndPoint, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "KinesisStreamConfig" => {
                    obj.kinesis_stream_config = Some(KinesisStreamConfigDeserializer::deserialize(
                        "KinesisStreamConfig",
                        stack,
                    )?);
                }
                "StreamType" => {
                    obj.stream_type = StringDeserializer::deserialize("StreamType", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct EndPointSerializer;
impl EndPointSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &EndPoint,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.kinesis_stream_config {
            &KinesisStreamConfigSerializer::serialize(&mut writer, "KinesisStreamConfig", value)?;
        }
        write_characters_element(writer, "StreamType", &obj.stream_type.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct EndPointListDeserializer;
impl EndPointListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EndPoint>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(EndPointDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct EndPointListSerializer;
impl EndPointListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<EndPoint>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            EndPointSerializer::serialize(writer, "EndPoint", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[allow(dead_code)]
struct EventTypeDeserializer;
impl EventTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct EventTypeSerializer;
impl EventTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>A complex data type that includes the profile configurations and other options specified for field-level encryption. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct FieldLevelEncryption {
    /// <p>A complex data type that includes the profile configurations specified for field-level encryption. </p>
    pub field_level_encryption_config: FieldLevelEncryptionConfig,
    /// <p>The configuration ID for a field-level encryption configuration which includes a set of profiles that specify certain selected data fields to be encrypted by specific public keys.</p>
    pub id: String,
    /// <p>The last time the field-level encryption configuration was changed. </p>
    pub last_modified_time: String,
}

#[allow(dead_code)]
struct FieldLevelEncryptionDeserializer;
impl FieldLevelEncryptionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FieldLevelEncryption, XmlParseError> {
        deserialize_elements::<_, FieldLevelEncryption, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "FieldLevelEncryptionConfig" => {
                    obj.field_level_encryption_config =
                        FieldLevelEncryptionConfigDeserializer::deserialize(
                            "FieldLevelEncryptionConfig",
                            stack,
                        )?;
                }
                "Id" => {
                    obj.id = StringDeserializer::deserialize("Id", stack)?;
                }
                "LastModifiedTime" => {
                    obj.last_modified_time =
                        TimestampDeserializer::deserialize("LastModifiedTime", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A complex data type that includes the profile configurations specified for field-level encryption. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FieldLevelEncryptionConfig {
    /// <p>A unique number that ensures the request can't be replayed.</p>
    pub caller_reference: String,
    /// <p>An optional comment about the configuration.</p>
    pub comment: Option<String>,
    /// <p>A complex data type that specifies when to forward content if a content type isn't recognized and profiles to use as by default in a request if a query argument doesn't specify a profile to use.</p>
    pub content_type_profile_config: Option<ContentTypeProfileConfig>,
    /// <p>A complex data type that specifies when to forward content if a profile isn't found and the profile that can be provided as a query argument in a request.</p>
    pub query_arg_profile_config: Option<QueryArgProfileConfig>,
}

#[allow(dead_code)]
struct FieldLevelEncryptionConfigDeserializer;
impl FieldLevelEncryptionConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FieldLevelEncryptionConfig, XmlParseError> {
        deserialize_elements::<_, FieldLevelEncryptionConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CallerReference" => {
                        obj.caller_reference =
                            StringDeserializer::deserialize("CallerReference", stack)?;
                    }
                    "Comment" => {
                        obj.comment = Some(StringDeserializer::deserialize("Comment", stack)?);
                    }
                    "ContentTypeProfileConfig" => {
                        obj.content_type_profile_config =
                            Some(ContentTypeProfileConfigDeserializer::deserialize(
                                "ContentTypeProfileConfig",
                                stack,
                            )?);
                    }
                    "QueryArgProfileConfig" => {
                        obj.query_arg_profile_config =
                            Some(QueryArgProfileConfigDeserializer::deserialize(
                                "QueryArgProfileConfig",
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

pub struct FieldLevelEncryptionConfigSerializer;
impl FieldLevelEncryptionConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &FieldLevelEncryptionConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "CallerReference", &obj.caller_reference.to_string())?;
        if let Some(ref value) = obj.comment {
            write_characters_element(writer, "Comment", &value.to_string())?;
        }
        if let Some(ref value) = obj.content_type_profile_config {
            &ContentTypeProfileConfigSerializer::serialize(
                &mut writer,
                "ContentTypeProfileConfig",
                value,
            )?;
        }
        if let Some(ref value) = obj.query_arg_profile_config {
            &QueryArgProfileConfigSerializer::serialize(
                &mut writer,
                "QueryArgProfileConfig",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>List of field-level encrpytion configurations.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct FieldLevelEncryptionList {
    /// <p>An array of field-level encryption items.</p>
    pub items: Option<Vec<FieldLevelEncryptionSummary>>,
    /// <p>The maximum number of elements you want in the response body. </p>
    pub max_items: i64,
    /// <p>If there are more elements to be listed, this element is present and contains the value that you can use for the <code>Marker</code> request parameter to continue listing your configurations where you left off.</p>
    pub next_marker: Option<String>,
    /// <p>The number of field-level encryption items.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct FieldLevelEncryptionListDeserializer;
impl FieldLevelEncryptionListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FieldLevelEncryptionList, XmlParseError> {
        deserialize_elements::<_, FieldLevelEncryptionList, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Items" => {
                        obj.items.get_or_insert(vec![]).extend(
                            FieldLevelEncryptionSummaryListDeserializer::deserialize(
                                "Items", stack,
                            )?,
                        );
                    }
                    "MaxItems" => {
                        obj.max_items = IntegerDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(StringDeserializer::deserialize("NextMarker", stack)?);
                    }
                    "Quantity" => {
                        obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A complex data type for field-level encryption profiles.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct FieldLevelEncryptionProfile {
    /// <p>A complex data type that includes the profile name and the encryption entities for the field-level encryption profile.</p>
    pub field_level_encryption_profile_config: FieldLevelEncryptionProfileConfig,
    /// <p>The ID for a field-level encryption profile configuration which includes a set of profiles that specify certain selected data fields to be encrypted by specific public keys.</p>
    pub id: String,
    /// <p>The last time the field-level encryption profile was updated.</p>
    pub last_modified_time: String,
}

#[allow(dead_code)]
struct FieldLevelEncryptionProfileDeserializer;
impl FieldLevelEncryptionProfileDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FieldLevelEncryptionProfile, XmlParseError> {
        deserialize_elements::<_, FieldLevelEncryptionProfile, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "FieldLevelEncryptionProfileConfig" => {
                        obj.field_level_encryption_profile_config =
                            FieldLevelEncryptionProfileConfigDeserializer::deserialize(
                                "FieldLevelEncryptionProfileConfig",
                                stack,
                            )?;
                    }
                    "Id" => {
                        obj.id = StringDeserializer::deserialize("Id", stack)?;
                    }
                    "LastModifiedTime" => {
                        obj.last_modified_time =
                            TimestampDeserializer::deserialize("LastModifiedTime", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A complex data type of profiles for the field-level encryption.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FieldLevelEncryptionProfileConfig {
    /// <p>A unique number that ensures that the request can't be replayed.</p>
    pub caller_reference: String,
    /// <p>An optional comment for the field-level encryption profile.</p>
    pub comment: Option<String>,
    /// <p>A complex data type of encryption entities for the field-level encryption profile that include the public key ID, provider, and field patterns for specifying which fields to encrypt with this key.</p>
    pub encryption_entities: EncryptionEntities,
    /// <p>Profile name for the field-level encryption profile.</p>
    pub name: String,
}

#[allow(dead_code)]
struct FieldLevelEncryptionProfileConfigDeserializer;
impl FieldLevelEncryptionProfileConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FieldLevelEncryptionProfileConfig, XmlParseError> {
        deserialize_elements::<_, FieldLevelEncryptionProfileConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CallerReference" => {
                        obj.caller_reference =
                            StringDeserializer::deserialize("CallerReference", stack)?;
                    }
                    "Comment" => {
                        obj.comment = Some(StringDeserializer::deserialize("Comment", stack)?);
                    }
                    "EncryptionEntities" => {
                        obj.encryption_entities = EncryptionEntitiesDeserializer::deserialize(
                            "EncryptionEntities",
                            stack,
                        )?;
                    }
                    "Name" => {
                        obj.name = StringDeserializer::deserialize("Name", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct FieldLevelEncryptionProfileConfigSerializer;
impl FieldLevelEncryptionProfileConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &FieldLevelEncryptionProfileConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "CallerReference", &obj.caller_reference.to_string())?;
        if let Some(ref value) = obj.comment {
            write_characters_element(writer, "Comment", &value.to_string())?;
        }
        EncryptionEntitiesSerializer::serialize(
            &mut writer,
            "EncryptionEntities",
            &obj.encryption_entities,
        )?;
        write_characters_element(writer, "Name", &obj.name.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>List of field-level encryption profiles.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct FieldLevelEncryptionProfileList {
    /// <p>The field-level encryption profile items.</p>
    pub items: Option<Vec<FieldLevelEncryptionProfileSummary>>,
    /// <p>The maximum number of field-level encryption profiles you want in the response body. </p>
    pub max_items: i64,
    /// <p>If there are more elements to be listed, this element is present and contains the value that you can use for the <code>Marker</code> request parameter to continue listing your profiles where you left off.</p>
    pub next_marker: Option<String>,
    /// <p>The number of field-level encryption profiles.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct FieldLevelEncryptionProfileListDeserializer;
impl FieldLevelEncryptionProfileListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FieldLevelEncryptionProfileList, XmlParseError> {
        deserialize_elements::<_, FieldLevelEncryptionProfileList, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Items" => {
                        obj.items.get_or_insert(vec![]).extend(
                            FieldLevelEncryptionProfileSummaryListDeserializer::deserialize(
                                "Items", stack,
                            )?,
                        );
                    }
                    "MaxItems" => {
                        obj.max_items = IntegerDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(StringDeserializer::deserialize("NextMarker", stack)?);
                    }
                    "Quantity" => {
                        obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>The field-level encryption profile summary.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct FieldLevelEncryptionProfileSummary {
    /// <p>An optional comment for the field-level encryption profile summary.</p>
    pub comment: Option<String>,
    /// <p>A complex data type of encryption entities for the field-level encryption profile that include the public key ID, provider, and field patterns for specifying which fields to encrypt with this key.</p>
    pub encryption_entities: EncryptionEntities,
    /// <p>ID for the field-level encryption profile summary.</p>
    pub id: String,
    /// <p>The time when the the field-level encryption profile summary was last updated.</p>
    pub last_modified_time: String,
    /// <p>Name for the field-level encryption profile summary.</p>
    pub name: String,
}

#[allow(dead_code)]
struct FieldLevelEncryptionProfileSummaryDeserializer;
impl FieldLevelEncryptionProfileSummaryDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FieldLevelEncryptionProfileSummary, XmlParseError> {
        deserialize_elements::<_, FieldLevelEncryptionProfileSummary, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Comment" => {
                        obj.comment = Some(StringDeserializer::deserialize("Comment", stack)?);
                    }
                    "EncryptionEntities" => {
                        obj.encryption_entities = EncryptionEntitiesDeserializer::deserialize(
                            "EncryptionEntities",
                            stack,
                        )?;
                    }
                    "Id" => {
                        obj.id = StringDeserializer::deserialize("Id", stack)?;
                    }
                    "LastModifiedTime" => {
                        obj.last_modified_time =
                            TimestampDeserializer::deserialize("LastModifiedTime", stack)?;
                    }
                    "Name" => {
                        obj.name = StringDeserializer::deserialize("Name", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct FieldLevelEncryptionProfileSummaryListDeserializer;
impl FieldLevelEncryptionProfileSummaryListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<FieldLevelEncryptionProfileSummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "FieldLevelEncryptionProfileSummary" {
                obj.push(FieldLevelEncryptionProfileSummaryDeserializer::deserialize(
                    "FieldLevelEncryptionProfileSummary",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A summary of a field-level encryption item.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct FieldLevelEncryptionSummary {
    /// <p>An optional comment about the field-level encryption item.</p>
    pub comment: Option<String>,
    /// <p> A summary of a content type-profile mapping. </p>
    pub content_type_profile_config: Option<ContentTypeProfileConfig>,
    /// <p>The unique ID of a field-level encryption item.</p>
    pub id: String,
    /// <p>The last time that the summary of field-level encryption items was modified.</p>
    pub last_modified_time: String,
    /// <p> A summary of a query argument-profile mapping. </p>
    pub query_arg_profile_config: Option<QueryArgProfileConfig>,
}

#[allow(dead_code)]
struct FieldLevelEncryptionSummaryDeserializer;
impl FieldLevelEncryptionSummaryDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FieldLevelEncryptionSummary, XmlParseError> {
        deserialize_elements::<_, FieldLevelEncryptionSummary, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Comment" => {
                        obj.comment = Some(StringDeserializer::deserialize("Comment", stack)?);
                    }
                    "ContentTypeProfileConfig" => {
                        obj.content_type_profile_config =
                            Some(ContentTypeProfileConfigDeserializer::deserialize(
                                "ContentTypeProfileConfig",
                                stack,
                            )?);
                    }
                    "Id" => {
                        obj.id = StringDeserializer::deserialize("Id", stack)?;
                    }
                    "LastModifiedTime" => {
                        obj.last_modified_time =
                            TimestampDeserializer::deserialize("LastModifiedTime", stack)?;
                    }
                    "QueryArgProfileConfig" => {
                        obj.query_arg_profile_config =
                            Some(QueryArgProfileConfigDeserializer::deserialize(
                                "QueryArgProfileConfig",
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
#[allow(dead_code)]
struct FieldLevelEncryptionSummaryListDeserializer;
impl FieldLevelEncryptionSummaryListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<FieldLevelEncryptionSummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "FieldLevelEncryptionSummary" {
                obj.push(FieldLevelEncryptionSummaryDeserializer::deserialize(
                    "FieldLevelEncryptionSummary",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct FieldListDeserializer;
impl FieldListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Field" {
                obj.push(StringDeserializer::deserialize("Field", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct FieldListSerializer;
impl FieldListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            StringSerializer::serialize(writer, "Field", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[allow(dead_code)]
struct FieldPatternListDeserializer;
impl FieldPatternListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "FieldPattern" {
                obj.push(StringDeserializer::deserialize("FieldPattern", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct FieldPatternListSerializer;
impl FieldPatternListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            StringSerializer::serialize(writer, "FieldPattern", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A complex data type that includes the field patterns to match for field-level encryption.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FieldPatterns {
    /// <p>An array of the field-level encryption field patterns.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of field-level encryption field patterns.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct FieldPatternsDeserializer;
impl FieldPatternsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FieldPatterns, XmlParseError> {
        deserialize_elements::<_, FieldPatterns, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .get_or_insert(vec![])
                        .extend(FieldPatternListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct FieldPatternsSerializer;
impl FieldPatternsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &FieldPatterns,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &FieldPatternListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct FormatDeserializer;
impl FormatDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct FormatSerializer;
impl FormatSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>This field is deprecated. We recommend that you use a cache policy or an origin request policy instead of this field.</p> <p>If you want to include values in the cache key, use a cache policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html#cache-key-create-cache-policy">Creating cache policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>If you want to send values to the origin but not include them in the cache key, use an origin request policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-origin-requests.html#origin-request-create-origin-request-policy">Creating origin request policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>A complex type that specifies how CloudFront handles query strings, cookies, and HTTP headers.</p>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ForwardedValues {
    /// <p>This field is deprecated. We recommend that you use a cache policy or an origin request policy instead of this field.</p> <p>If you want to include cookies in the cache key, use a cache policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html#cache-key-create-cache-policy">Creating cache policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>If you want to send cookies to the origin but not include them in the cache key, use an origin request policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-origin-requests.html#origin-request-create-origin-request-policy">Creating origin request policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>A complex type that specifies whether you want CloudFront to forward cookies to the origin and, if so, which ones. For more information about forwarding cookies to the origin, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Cookies.html">How CloudFront Forwards, Caches, and Logs Cookies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub cookies: CookiePreference,
    /// <p>This field is deprecated. We recommend that you use a cache policy or an origin request policy instead of this field.</p> <p>If you want to include headers in the cache key, use a cache policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html#cache-key-create-cache-policy">Creating cache policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>If you want to send headers to the origin but not include them in the cache key, use an origin request policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-origin-requests.html#origin-request-create-origin-request-policy">Creating origin request policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>A complex type that specifies the <code>Headers</code>, if any, that you want CloudFront to forward to the origin for this cache behavior (whitelisted headers). For the headers that you specify, CloudFront also caches separate versions of a specified object that is based on the header values in viewer requests.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/header-caching.html"> Caching Content Based on Request Headers</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub headers: Option<Headers>,
    /// <p>This field is deprecated. We recommend that you use a cache policy or an origin request policy instead of this field.</p> <p>If you want to include query strings in the cache key, use a cache policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html#cache-key-create-cache-policy">Creating cache policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>If you want to send query strings to the origin but not include them in the cache key, use an origin request policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-origin-requests.html#origin-request-create-origin-request-policy">Creating origin request policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>Indicates whether you want CloudFront to forward query strings to the origin that is associated with this cache behavior and cache based on the query string parameters. CloudFront behavior depends on the value of <code>QueryString</code> and on the values that you specify for <code>QueryStringCacheKeys</code>, if any:</p> <p>If you specify true for <code>QueryString</code> and you don't specify any values for <code>QueryStringCacheKeys</code>, CloudFront forwards all query string parameters to the origin and caches based on all query string parameters. Depending on how many query string parameters and values you have, this can adversely affect performance because CloudFront must forward more requests to the origin.</p> <p>If you specify true for <code>QueryString</code> and you specify one or more values for <code>QueryStringCacheKeys</code>, CloudFront forwards all query string parameters to the origin, but it only caches based on the query string parameters that you specify.</p> <p>If you specify false for <code>QueryString</code>, CloudFront doesn't forward any query string parameters to the origin, and doesn't cache based on query string parameters.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/QueryStringParameters.html">Configuring CloudFront to Cache Based on Query String Parameters</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub query_string: bool,
    /// <p>This field is deprecated. We recommend that you use a cache policy or an origin request policy instead of this field.</p> <p>If you want to include query strings in the cache key, use a cache policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html#cache-key-create-cache-policy">Creating cache policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>If you want to send query strings to the origin but not include them in the cache key, use an origin request policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-origin-requests.html#origin-request-create-origin-request-policy">Creating origin request policies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>A complex type that contains information about the query string parameters that you want CloudFront to use for caching for this cache behavior.</p>
    pub query_string_cache_keys: Option<QueryStringCacheKeys>,
}

/// <p>A complex type that controls the countries in which your content is distributed. CloudFront determines the location of your users using <code>MaxMind</code> GeoIP databases. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GeoRestriction {
    /// <p> A complex type that contains a <code>Location</code> element for each country in which you want CloudFront either to distribute your content (<code>whitelist</code>) or not distribute your content (<code>blacklist</code>).</p> <p>The <code>Location</code> element is a two-letter, uppercase country code for a country that you want to include in your <code>blacklist</code> or <code>whitelist</code>. Include one <code>Location</code> element for each country.</p> <p>CloudFront and <code>MaxMind</code> both use <code>ISO 3166</code> country codes. For the current list of countries and the corresponding codes, see <code>ISO 3166-1-alpha-2</code> code on the <i>International Organization for Standardization</i> website. You can also refer to the country list on the CloudFront console, which includes both country names and codes.</p>
    pub items: Option<Vec<String>>,
    /// <p>When geo restriction is <code>enabled</code>, this is the number of countries in your <code>whitelist</code> or <code>blacklist</code>. Otherwise, when it is not enabled, <code>Quantity</code> is <code>0</code>, and you can omit <code>Items</code>.</p>
    pub quantity: i64,
    /// <p><p>The method that you want to use to restrict distribution of your content by country:</p> <ul> <li> <p> <code>none</code>: No geo restriction is enabled, meaning access to content is not restricted by client geo location.</p> </li> <li> <p> <code>blacklist</code>: The <code>Location</code> elements specify the countries in which you don&#39;t want CloudFront to distribute your content.</p> </li> <li> <p> <code>whitelist</code>: The <code>Location</code> elements specify the countries in which you want CloudFront to distribute your content.</p> </li> </ul></p>
    pub restriction_type: String,
}

#[allow(dead_code)]
struct GeoRestrictionDeserializer;
impl GeoRestrictionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GeoRestriction, XmlParseError> {
        deserialize_elements::<_, GeoRestriction, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .get_or_insert(vec![])
                        .extend(LocationListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                "RestrictionType" => {
                    obj.restriction_type =
                        GeoRestrictionTypeDeserializer::deserialize("RestrictionType", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct GeoRestrictionSerializer;
impl GeoRestrictionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &GeoRestriction,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &LocationListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        write_characters_element(writer, "RestrictionType", &obj.restriction_type.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct GeoRestrictionTypeDeserializer;
impl GeoRestrictionTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct GeoRestrictionTypeSerializer;
impl GeoRestrictionTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCachePolicyConfigRequest {
    /// <p>The unique identifier for the cache policy. If the cache policy is attached to a distribution’s cache behavior, you can get the policy’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the cache policy is not attached to a cache behavior, you can get the identifier using <code>ListCachePolicies</code>.</p>
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetCachePolicyConfigResult {
    /// <p>The cache policy configuration.</p>
    pub cache_policy_config: Option<CachePolicyConfig>,
    /// <p>The current version of the cache policy.</p>
    pub e_tag: Option<String>,
}

#[allow(dead_code)]
struct GetCachePolicyConfigResultDeserializer;
impl GetCachePolicyConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetCachePolicyConfigResult, XmlParseError> {
        Ok(GetCachePolicyConfigResult {
            cache_policy_config: Some(CachePolicyConfigDeserializer::deserialize(
                "CachePolicyConfig",
                stack,
            )?),
            ..GetCachePolicyConfigResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCachePolicyRequest {
    /// <p>The unique identifier for the cache policy. If the cache policy is attached to a distribution’s cache behavior, you can get the policy’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the cache policy is not attached to a cache behavior, you can get the identifier using <code>ListCachePolicies</code>.</p>
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetCachePolicyResult {
    /// <p>The cache policy.</p>
    pub cache_policy: Option<CachePolicy>,
    /// <p>The current version of the cache policy.</p>
    pub e_tag: Option<String>,
}

#[allow(dead_code)]
struct GetCachePolicyResultDeserializer;
impl GetCachePolicyResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetCachePolicyResult, XmlParseError> {
        Ok(GetCachePolicyResult {
            cache_policy: Some(CachePolicyDeserializer::deserialize("CachePolicy", stack)?),
            ..GetCachePolicyResult::default()
        })
    }
}
/// <p>The origin access identity's configuration information. For more information, see <a href="https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_CloudFrontOriginAccessIdentityConfig.html">CloudFrontOriginAccessIdentityConfig</a>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCloudFrontOriginAccessIdentityConfigRequest {
    /// <p>The identity's ID. </p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetCloudFrontOriginAccessIdentityConfigResult {
    /// <p>The origin access identity's configuration information. </p>
    pub cloud_front_origin_access_identity_config: Option<CloudFrontOriginAccessIdentityConfig>,
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

#[allow(dead_code)]
struct GetCloudFrontOriginAccessIdentityConfigResultDeserializer;
impl GetCloudFrontOriginAccessIdentityConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetCloudFrontOriginAccessIdentityConfigResult, XmlParseError> {
        Ok(GetCloudFrontOriginAccessIdentityConfigResult {
            cloud_front_origin_access_identity_config: Some(
                CloudFrontOriginAccessIdentityConfigDeserializer::deserialize(
                    "CloudFrontOriginAccessIdentityConfig",
                    stack,
                )?,
            ),
            ..GetCloudFrontOriginAccessIdentityConfigResult::default()
        })
    }
}
/// <p>The request to get an origin access identity's information.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCloudFrontOriginAccessIdentityRequest {
    /// <p>The identity's ID.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetCloudFrontOriginAccessIdentityResult {
    /// <p>The origin access identity's information.</p>
    pub cloud_front_origin_access_identity: Option<CloudFrontOriginAccessIdentity>,
    /// <p>The current version of the origin access identity's information. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

#[allow(dead_code)]
struct GetCloudFrontOriginAccessIdentityResultDeserializer;
impl GetCloudFrontOriginAccessIdentityResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetCloudFrontOriginAccessIdentityResult, XmlParseError> {
        Ok(GetCloudFrontOriginAccessIdentityResult {
            cloud_front_origin_access_identity: Some(
                CloudFrontOriginAccessIdentityDeserializer::deserialize(
                    "CloudFrontOriginAccessIdentity",
                    stack,
                )?,
            ),
            ..GetCloudFrontOriginAccessIdentityResult::default()
        })
    }
}
/// <p>The request to get a distribution configuration.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDistributionConfigRequest {
    /// <p>The distribution's ID. If the ID is empty, an empty distribution configuration is returned.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetDistributionConfigResult {
    /// <p>The distribution's configuration information.</p>
    pub distribution_config: Option<DistributionConfig>,
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

#[allow(dead_code)]
struct GetDistributionConfigResultDeserializer;
impl GetDistributionConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetDistributionConfigResult, XmlParseError> {
        Ok(GetDistributionConfigResult {
            distribution_config: Some(DistributionConfigDeserializer::deserialize(
                "DistributionConfig",
                stack,
            )?),
            ..GetDistributionConfigResult::default()
        })
    }
}
/// <p>The request to get a distribution's information.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDistributionRequest {
    /// <p>The distribution's ID. If the ID is empty, an empty distribution configuration is returned.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetDistributionResult {
    /// <p>The distribution's information.</p>
    pub distribution: Option<Distribution>,
    /// <p>The current version of the distribution's information. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

#[allow(dead_code)]
struct GetDistributionResultDeserializer;
impl GetDistributionResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetDistributionResult, XmlParseError> {
        Ok(GetDistributionResult {
            distribution: Some(DistributionDeserializer::deserialize(
                "Distribution",
                stack,
            )?),
            ..GetDistributionResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFieldLevelEncryptionConfigRequest {
    /// <p>Request the ID for the field-level encryption configuration information.</p>
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetFieldLevelEncryptionConfigResult {
    /// <p>The current version of the field level encryption configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Return the field-level encryption configuration information.</p>
    pub field_level_encryption_config: Option<FieldLevelEncryptionConfig>,
}

#[allow(dead_code)]
struct GetFieldLevelEncryptionConfigResultDeserializer;
impl GetFieldLevelEncryptionConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetFieldLevelEncryptionConfigResult, XmlParseError> {
        Ok(GetFieldLevelEncryptionConfigResult {
            field_level_encryption_config: Some(
                FieldLevelEncryptionConfigDeserializer::deserialize(
                    "FieldLevelEncryptionConfig",
                    stack,
                )?,
            ),
            ..GetFieldLevelEncryptionConfigResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFieldLevelEncryptionProfileConfigRequest {
    /// <p>Get the ID for the field-level encryption profile configuration information.</p>
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetFieldLevelEncryptionProfileConfigResult {
    /// <p>The current version of the field-level encryption profile configuration result. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Return the field-level encryption profile configuration information.</p>
    pub field_level_encryption_profile_config: Option<FieldLevelEncryptionProfileConfig>,
}

#[allow(dead_code)]
struct GetFieldLevelEncryptionProfileConfigResultDeserializer;
impl GetFieldLevelEncryptionProfileConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetFieldLevelEncryptionProfileConfigResult, XmlParseError> {
        Ok(GetFieldLevelEncryptionProfileConfigResult {
            field_level_encryption_profile_config: Some(
                FieldLevelEncryptionProfileConfigDeserializer::deserialize(
                    "FieldLevelEncryptionProfileConfig",
                    stack,
                )?,
            ),
            ..GetFieldLevelEncryptionProfileConfigResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFieldLevelEncryptionProfileRequest {
    /// <p>Get the ID for the field-level encryption profile information.</p>
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetFieldLevelEncryptionProfileResult {
    /// <p>The current version of the field level encryption profile. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Return the field-level encryption profile information.</p>
    pub field_level_encryption_profile: Option<FieldLevelEncryptionProfile>,
}

#[allow(dead_code)]
struct GetFieldLevelEncryptionProfileResultDeserializer;
impl GetFieldLevelEncryptionProfileResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetFieldLevelEncryptionProfileResult, XmlParseError> {
        Ok(GetFieldLevelEncryptionProfileResult {
            field_level_encryption_profile: Some(
                FieldLevelEncryptionProfileDeserializer::deserialize(
                    "FieldLevelEncryptionProfile",
                    stack,
                )?,
            ),
            ..GetFieldLevelEncryptionProfileResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFieldLevelEncryptionRequest {
    /// <p>Request the ID for the field-level encryption configuration information.</p>
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetFieldLevelEncryptionResult {
    /// <p>The current version of the field level encryption configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Return the field-level encryption configuration information.</p>
    pub field_level_encryption: Option<FieldLevelEncryption>,
}

#[allow(dead_code)]
struct GetFieldLevelEncryptionResultDeserializer;
impl GetFieldLevelEncryptionResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetFieldLevelEncryptionResult, XmlParseError> {
        Ok(GetFieldLevelEncryptionResult {
            field_level_encryption: Some(FieldLevelEncryptionDeserializer::deserialize(
                "FieldLevelEncryption",
                stack,
            )?),
            ..GetFieldLevelEncryptionResult::default()
        })
    }
}
/// <p>The request to get an invalidation's information. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInvalidationRequest {
    /// <p>The distribution's ID.</p>
    pub distribution_id: String,
    /// <p>The identifier for the invalidation request, for example, <code>IDFDVBD632BHDS5</code>.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetInvalidationResult {
    /// <p>The invalidation's information. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/InvalidationDatatype.html">Invalidation Complex Type</a>. </p>
    pub invalidation: Option<Invalidation>,
}

#[allow(dead_code)]
struct GetInvalidationResultDeserializer;
impl GetInvalidationResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetInvalidationResult, XmlParseError> {
        Ok(GetInvalidationResult {
            invalidation: Some(InvalidationDeserializer::deserialize(
                "Invalidation",
                stack,
            )?),
            ..GetInvalidationResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetKeyGroupConfigRequest {
    /// <p>The identifier of the key group whose configuration you are getting. To get the identifier, use <code>ListKeyGroups</code>.</p>
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetKeyGroupConfigResult {
    /// <p>The identifier for this version of the key group.</p>
    pub e_tag: Option<String>,
    /// <p>The key group configuration.</p>
    pub key_group_config: Option<KeyGroupConfig>,
}

#[allow(dead_code)]
struct GetKeyGroupConfigResultDeserializer;
impl GetKeyGroupConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetKeyGroupConfigResult, XmlParseError> {
        Ok(GetKeyGroupConfigResult {
            key_group_config: Some(KeyGroupConfigDeserializer::deserialize(
                "KeyGroupConfig",
                stack,
            )?),
            ..GetKeyGroupConfigResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetKeyGroupRequest {
    /// <p>The identifier of the key group that you are getting. To get the identifier, use <code>ListKeyGroups</code>.</p>
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetKeyGroupResult {
    /// <p>The identifier for this version of the key group.</p>
    pub e_tag: Option<String>,
    /// <p>The key group.</p>
    pub key_group: Option<KeyGroup>,
}

#[allow(dead_code)]
struct GetKeyGroupResultDeserializer;
impl GetKeyGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetKeyGroupResult, XmlParseError> {
        Ok(GetKeyGroupResult {
            key_group: Some(KeyGroupDeserializer::deserialize("KeyGroup", stack)?),
            ..GetKeyGroupResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMonitoringSubscriptionRequest {
    /// <p>The ID of the distribution that you are getting metrics information for.</p>
    pub distribution_id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetMonitoringSubscriptionResult {
    /// <p>A monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.</p>
    pub monitoring_subscription: Option<MonitoringSubscription>,
}

#[allow(dead_code)]
struct GetMonitoringSubscriptionResultDeserializer;
impl GetMonitoringSubscriptionResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetMonitoringSubscriptionResult, XmlParseError> {
        Ok(GetMonitoringSubscriptionResult {
            monitoring_subscription: Some(MonitoringSubscriptionDeserializer::deserialize(
                "MonitoringSubscription",
                stack,
            )?),
            ..GetMonitoringSubscriptionResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOriginRequestPolicyConfigRequest {
    /// <p>The unique identifier for the origin request policy. If the origin request policy is attached to a distribution’s cache behavior, you can get the policy’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the origin request policy is not attached to a cache behavior, you can get the identifier using <code>ListOriginRequestPolicies</code>.</p>
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetOriginRequestPolicyConfigResult {
    /// <p>The current version of the origin request policy.</p>
    pub e_tag: Option<String>,
    /// <p>The origin request policy configuration.</p>
    pub origin_request_policy_config: Option<OriginRequestPolicyConfig>,
}

#[allow(dead_code)]
struct GetOriginRequestPolicyConfigResultDeserializer;
impl GetOriginRequestPolicyConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetOriginRequestPolicyConfigResult, XmlParseError> {
        Ok(GetOriginRequestPolicyConfigResult {
            origin_request_policy_config: Some(OriginRequestPolicyConfigDeserializer::deserialize(
                "OriginRequestPolicyConfig",
                stack,
            )?),
            ..GetOriginRequestPolicyConfigResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOriginRequestPolicyRequest {
    /// <p>The unique identifier for the origin request policy. If the origin request policy is attached to a distribution’s cache behavior, you can get the policy’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the origin request policy is not attached to a cache behavior, you can get the identifier using <code>ListOriginRequestPolicies</code>.</p>
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetOriginRequestPolicyResult {
    /// <p>The current version of the origin request policy.</p>
    pub e_tag: Option<String>,
    /// <p>The origin request policy.</p>
    pub origin_request_policy: Option<OriginRequestPolicy>,
}

#[allow(dead_code)]
struct GetOriginRequestPolicyResultDeserializer;
impl GetOriginRequestPolicyResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetOriginRequestPolicyResult, XmlParseError> {
        Ok(GetOriginRequestPolicyResult {
            origin_request_policy: Some(OriginRequestPolicyDeserializer::deserialize(
                "OriginRequestPolicy",
                stack,
            )?),
            ..GetOriginRequestPolicyResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPublicKeyConfigRequest {
    /// <p>The identifier of the public key whose configuration you are getting.</p>
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetPublicKeyConfigResult {
    /// <p>The identifier for this version of the public key configuration.</p>
    pub e_tag: Option<String>,
    /// <p>A public key configuration.</p>
    pub public_key_config: Option<PublicKeyConfig>,
}

#[allow(dead_code)]
struct GetPublicKeyConfigResultDeserializer;
impl GetPublicKeyConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetPublicKeyConfigResult, XmlParseError> {
        Ok(GetPublicKeyConfigResult {
            public_key_config: Some(PublicKeyConfigDeserializer::deserialize(
                "PublicKeyConfig",
                stack,
            )?),
            ..GetPublicKeyConfigResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPublicKeyRequest {
    /// <p>The identifier of the public key you are getting.</p>
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetPublicKeyResult {
    /// <p>The identifier for this version of the public key.</p>
    pub e_tag: Option<String>,
    /// <p>The public key.</p>
    pub public_key: Option<PublicKey>,
}

#[allow(dead_code)]
struct GetPublicKeyResultDeserializer;
impl GetPublicKeyResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetPublicKeyResult, XmlParseError> {
        Ok(GetPublicKeyResult {
            public_key: Some(PublicKeyDeserializer::deserialize("PublicKey", stack)?),
            ..GetPublicKeyResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRealtimeLogConfigRequest {
    /// <p>The Amazon Resource Name (ARN) of the real-time log configuration to get.</p>
    pub arn: Option<String>,
    /// <p>The name of the real-time log configuration to get.</p>
    pub name: Option<String>,
}

pub struct GetRealtimeLogConfigRequestSerializer;
impl GetRealtimeLogConfigRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &GetRealtimeLogConfigRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        if let Some(ref value) = obj.arn {
            &StringSerializer::serialize(&mut writer, "ARN", value)?;
        }
        if let Some(ref value) = obj.name {
            &StringSerializer::serialize(&mut writer, "Name", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetRealtimeLogConfigResult {
    /// <p>A real-time log configuration.</p>
    pub realtime_log_config: Option<RealtimeLogConfig>,
}

#[allow(dead_code)]
struct GetRealtimeLogConfigResultDeserializer;
impl GetRealtimeLogConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetRealtimeLogConfigResult, XmlParseError> {
        deserialize_elements::<_, GetRealtimeLogConfigResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "RealtimeLogConfig" => {
                        obj.realtime_log_config = Some(RealtimeLogConfigDeserializer::deserialize(
                            "RealtimeLogConfig",
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
/// <p>To request to get a streaming distribution configuration.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetStreamingDistributionConfigRequest {
    /// <p>The streaming distribution's ID.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetStreamingDistributionConfigResult {
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>. </p>
    pub e_tag: Option<String>,
    /// <p>The streaming distribution's configuration information.</p>
    pub streaming_distribution_config: Option<StreamingDistributionConfig>,
}

#[allow(dead_code)]
struct GetStreamingDistributionConfigResultDeserializer;
impl GetStreamingDistributionConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetStreamingDistributionConfigResult, XmlParseError> {
        Ok(GetStreamingDistributionConfigResult {
            streaming_distribution_config: Some(
                StreamingDistributionConfigDeserializer::deserialize(
                    "StreamingDistributionConfig",
                    stack,
                )?,
            ),
            ..GetStreamingDistributionConfigResult::default()
        })
    }
}
/// <p>The request to get a streaming distribution's information.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetStreamingDistributionRequest {
    /// <p>The streaming distribution's ID.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetStreamingDistributionResult {
    /// <p>The current version of the streaming distribution's information. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>The streaming distribution's information.</p>
    pub streaming_distribution: Option<StreamingDistribution>,
}

#[allow(dead_code)]
struct GetStreamingDistributionResultDeserializer;
impl GetStreamingDistributionResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetStreamingDistributionResult, XmlParseError> {
        Ok(GetStreamingDistributionResult {
            streaming_distribution: Some(StreamingDistributionDeserializer::deserialize(
                "StreamingDistribution",
                stack,
            )?),
            ..GetStreamingDistributionResult::default()
        })
    }
}
#[allow(dead_code)]
struct HeaderListDeserializer;
impl HeaderListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Name" {
                obj.push(StringDeserializer::deserialize("Name", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct HeaderListSerializer;
impl HeaderListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            StringSerializer::serialize(writer, "Name", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>Contains a list of HTTP header names.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Headers {
    /// <p>A list of HTTP header names.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of header names in the <code>Items</code> list.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct HeadersDeserializer;
impl HeadersDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Headers, XmlParseError> {
        deserialize_elements::<_, Headers, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .get_or_insert(vec![])
                        .extend(HeaderListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct HeadersSerializer;
impl HeadersSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Headers,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &HeaderListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct HttpVersionDeserializer;
impl HttpVersionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct HttpVersionSerializer;
impl HttpVersionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

#[allow(dead_code)]
struct ICPRecordalStatusDeserializer;
impl ICPRecordalStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct IntegerDeserializer;
impl IntegerDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}

pub struct IntegerSerializer;
impl IntegerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, &obj.to_string())
    }
}

/// <p>An invalidation. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Invalidation {
    /// <p>The date and time the invalidation request was first made. </p>
    pub create_time: String,
    /// <p>The identifier for the invalidation request. For example: <code>IDFDVBD632BHDS5</code>.</p>
    pub id: String,
    /// <p>The current invalidation information for the batch request. </p>
    pub invalidation_batch: InvalidationBatch,
    /// <p>The status of the invalidation request. When the invalidation batch is finished, the status is <code>Completed</code>.</p>
    pub status: String,
}

#[allow(dead_code)]
struct InvalidationDeserializer;
impl InvalidationDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Invalidation, XmlParseError> {
        deserialize_elements::<_, Invalidation, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CreateTime" => {
                    obj.create_time = TimestampDeserializer::deserialize("CreateTime", stack)?;
                }
                "Id" => {
                    obj.id = StringDeserializer::deserialize("Id", stack)?;
                }
                "InvalidationBatch" => {
                    obj.invalidation_batch =
                        InvalidationBatchDeserializer::deserialize("InvalidationBatch", stack)?;
                }
                "Status" => {
                    obj.status = StringDeserializer::deserialize("Status", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>An invalidation batch.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InvalidationBatch {
    /// <p>A value that you specify to uniquely identify an invalidation request. CloudFront uses the value to prevent you from accidentally resubmitting an identical request. Whenever you create a new invalidation request, you must specify a new value for <code>CallerReference</code> and change other values in the request as applicable. One way to ensure that the value of <code>CallerReference</code> is unique is to use a <code>timestamp</code>, for example, <code>20120301090000</code>.</p> <p>If you make a second invalidation request with the same value for <code>CallerReference</code>, and if the rest of the request is the same, CloudFront doesn't create a new invalidation request. Instead, CloudFront returns information about the invalidation request that you previously created with the same <code>CallerReference</code>.</p> <p>If <code>CallerReference</code> is a value you already sent in a previous invalidation batch request but the content of any <code>Path</code> is different from the original request, CloudFront returns an <code>InvalidationBatchAlreadyExists</code> error.</p>
    pub caller_reference: String,
    /// <p>A complex type that contains information about the objects that you want to invalidate. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Invalidation.html#invalidation-specifying-objects">Specifying the Objects to Invalidate</a> in the <i>Amazon CloudFront Developer Guide</i>. </p>
    pub paths: Paths,
}

#[allow(dead_code)]
struct InvalidationBatchDeserializer;
impl InvalidationBatchDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InvalidationBatch, XmlParseError> {
        deserialize_elements::<_, InvalidationBatch, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CallerReference" => {
                    obj.caller_reference =
                        StringDeserializer::deserialize("CallerReference", stack)?;
                }
                "Paths" => {
                    obj.paths = PathsDeserializer::deserialize("Paths", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct InvalidationBatchSerializer;
impl InvalidationBatchSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InvalidationBatch,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "CallerReference", &obj.caller_reference.to_string())?;
        PathsSerializer::serialize(&mut writer, "Paths", &obj.paths)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>The <code>InvalidationList</code> complex type describes the list of invalidation objects. For more information about invalidation, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Invalidation.html">Invalidating Objects (Web Distributions Only)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct InvalidationList {
    /// <p>A flag that indicates whether more invalidation batch requests remain to be listed. If your results were truncated, you can make a follow-up pagination request using the <code>Marker</code> request parameter to retrieve more invalidation batches in the list.</p>
    pub is_truncated: bool,
    /// <p>A complex type that contains one <code>InvalidationSummary</code> element for each invalidation batch created by the current AWS account.</p>
    pub items: Option<Vec<InvalidationSummary>>,
    /// <p>The value that you provided for the <code>Marker</code> request parameter.</p>
    pub marker: String,
    /// <p>The value that you provided for the <code>MaxItems</code> request parameter.</p>
    pub max_items: i64,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value that you can use for the <code>Marker</code> request parameter to continue listing your invalidation batches where they left off.</p>
    pub next_marker: Option<String>,
    /// <p>The number of invalidation batches that were created by the current AWS account. </p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct InvalidationListDeserializer;
impl InvalidationListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InvalidationList, XmlParseError> {
        deserialize_elements::<_, InvalidationList, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "IsTruncated" => {
                    obj.is_truncated = BooleanDeserializer::deserialize("IsTruncated", stack)?;
                }
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        InvalidationSummaryListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "Marker" => {
                    obj.marker = StringDeserializer::deserialize("Marker", stack)?;
                }
                "MaxItems" => {
                    obj.max_items = IntegerDeserializer::deserialize("MaxItems", stack)?;
                }
                "NextMarker" => {
                    obj.next_marker = Some(StringDeserializer::deserialize("NextMarker", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A summary of an invalidation request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct InvalidationSummary {
    /// <p>The time that an invalidation request was created.</p>
    pub create_time: String,
    /// <p>The unique ID for an invalidation request.</p>
    pub id: String,
    /// <p>The status of an invalidation request.</p>
    pub status: String,
}

#[allow(dead_code)]
struct InvalidationSummaryDeserializer;
impl InvalidationSummaryDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InvalidationSummary, XmlParseError> {
        deserialize_elements::<_, InvalidationSummary, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CreateTime" => {
                    obj.create_time = TimestampDeserializer::deserialize("CreateTime", stack)?;
                }
                "Id" => {
                    obj.id = StringDeserializer::deserialize("Id", stack)?;
                }
                "Status" => {
                    obj.status = StringDeserializer::deserialize("Status", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct InvalidationSummaryListDeserializer;
impl InvalidationSummaryListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<InvalidationSummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "InvalidationSummary" {
                obj.push(InvalidationSummaryDeserializer::deserialize(
                    "InvalidationSummary",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A list of identifiers for the public keys that CloudFront can use to verify the signatures of signed URLs and signed cookies.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct KGKeyPairIds {
    /// <p>The identifier of the key group that contains the public keys.</p>
    pub key_group_id: Option<String>,
    pub key_pair_ids: Option<KeyPairIds>,
}

#[allow(dead_code)]
struct KGKeyPairIdsDeserializer;
impl KGKeyPairIdsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<KGKeyPairIds, XmlParseError> {
        deserialize_elements::<_, KGKeyPairIds, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "KeyGroupId" => {
                    obj.key_group_id = Some(StringDeserializer::deserialize("KeyGroupId", stack)?);
                }
                "KeyPairIds" => {
                    obj.key_pair_ids =
                        Some(KeyPairIdsDeserializer::deserialize("KeyPairIds", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct KGKeyPairIdsListDeserializer;
impl KGKeyPairIdsListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<KGKeyPairIds>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "KeyGroup" {
                obj.push(KGKeyPairIdsDeserializer::deserialize("KeyGroup", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A key group.</p> <p>A key group contains a list of public keys that you can use with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">CloudFront signed URLs and signed cookies</a>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct KeyGroup {
    /// <p>The identifier for the key group.</p>
    pub id: String,
    /// <p>The key group configuration.</p>
    pub key_group_config: KeyGroupConfig,
    /// <p>The date and time when the key group was last modified.</p>
    pub last_modified_time: String,
}

#[allow(dead_code)]
struct KeyGroupDeserializer;
impl KeyGroupDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<KeyGroup, XmlParseError> {
        deserialize_elements::<_, KeyGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Id" => {
                    obj.id = StringDeserializer::deserialize("Id", stack)?;
                }
                "KeyGroupConfig" => {
                    obj.key_group_config =
                        KeyGroupConfigDeserializer::deserialize("KeyGroupConfig", stack)?;
                }
                "LastModifiedTime" => {
                    obj.last_modified_time =
                        TimestampDeserializer::deserialize("LastModifiedTime", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A key group configuration.</p> <p>A key group contains a list of public keys that you can use with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">CloudFront signed URLs and signed cookies</a>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct KeyGroupConfig {
    /// <p>A comment to describe the key group.</p>
    pub comment: Option<String>,
    /// <p>A list of the identifiers of the public keys in the key group.</p>
    pub items: Vec<String>,
    /// <p>A name to identify the key group.</p>
    pub name: String,
}

#[allow(dead_code)]
struct KeyGroupConfigDeserializer;
impl KeyGroupConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<KeyGroupConfig, XmlParseError> {
        deserialize_elements::<_, KeyGroupConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Comment" => {
                    obj.comment = Some(StringDeserializer::deserialize("Comment", stack)?);
                }
                "Items" => {
                    obj.items
                        .extend(PublicKeyIdListDeserializer::deserialize("Items", stack)?);
                }
                "Name" => {
                    obj.name = StringDeserializer::deserialize("Name", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct KeyGroupConfigSerializer;
impl KeyGroupConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &KeyGroupConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.comment {
            write_characters_element(writer, "Comment", &value.to_string())?;
        }
        PublicKeyIdListSerializer::serialize(&mut writer, "Items", &obj.items)?;
        write_characters_element(writer, "Name", &obj.name.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A list of key groups.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct KeyGroupList {
    /// <p>A list of key groups.</p>
    pub items: Option<Vec<KeyGroupSummary>>,
    /// <p>The maximum number of key groups requested.</p>
    pub max_items: i64,
    /// <p>If there are more items in the list than are in this response, this element is present. It contains the value that you should use in the <code>Marker</code> field of a subsequent request to continue listing key groups.</p>
    pub next_marker: Option<String>,
    /// <p>The number of key groups returned in the response.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct KeyGroupListDeserializer;
impl KeyGroupListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<KeyGroupList, XmlParseError> {
        deserialize_elements::<_, KeyGroupList, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        KeyGroupSummaryListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "MaxItems" => {
                    obj.max_items = IntegerDeserializer::deserialize("MaxItems", stack)?;
                }
                "NextMarker" => {
                    obj.next_marker = Some(StringDeserializer::deserialize("NextMarker", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Contains information about a key group.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct KeyGroupSummary {
    /// <p>A key group.</p>
    pub key_group: KeyGroup,
}

#[allow(dead_code)]
struct KeyGroupSummaryDeserializer;
impl KeyGroupSummaryDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<KeyGroupSummary, XmlParseError> {
        deserialize_elements::<_, KeyGroupSummary, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "KeyGroup" => {
                    obj.key_group = KeyGroupDeserializer::deserialize("KeyGroup", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct KeyGroupSummaryListDeserializer;
impl KeyGroupSummaryListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<KeyGroupSummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "KeyGroupSummary" {
                obj.push(KeyGroupSummaryDeserializer::deserialize(
                    "KeyGroupSummary",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct KeyPairIdListDeserializer;
impl KeyPairIdListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "KeyPairId" {
                obj.push(StringDeserializer::deserialize("KeyPairId", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A list of CloudFront key pair identifiers.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct KeyPairIds {
    /// <p>A list of CloudFront key pair identifiers.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of key pair identifiers in the list.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct KeyPairIdsDeserializer;
impl KeyPairIdsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<KeyPairIds, XmlParseError> {
        deserialize_elements::<_, KeyPairIds, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .get_or_insert(vec![])
                        .extend(KeyPairIdListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Contains information about the Amazon Kinesis data stream where you are sending real-time log data.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct KinesisStreamConfig {
    /// <p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that CloudFront can use to send real-time log data to your Kinesis data stream.</p> <p>For more information the IAM role, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-iam-role">Real-time log configuration IAM role</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub role_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the Kinesis data stream where you are sending real-time log data.</p>
    pub stream_arn: String,
}

#[allow(dead_code)]
struct KinesisStreamConfigDeserializer;
impl KinesisStreamConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<KinesisStreamConfig, XmlParseError> {
        deserialize_elements::<_, KinesisStreamConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "RoleARN" => {
                    obj.role_arn = StringDeserializer::deserialize("RoleARN", stack)?;
                }
                "StreamARN" => {
                    obj.stream_arn = StringDeserializer::deserialize("StreamARN", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct KinesisStreamConfigSerializer;
impl KinesisStreamConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &KinesisStreamConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "RoleARN", &obj.role_arn.to_string())?;
        write_characters_element(writer, "StreamARN", &obj.stream_arn.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct LambdaFunctionARNDeserializer;
impl LambdaFunctionARNDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct LambdaFunctionARNSerializer;
impl LambdaFunctionARNSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>A complex type that contains a Lambda function association.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LambdaFunctionAssociation {
    /// <p><p>Specifies the event type that triggers a Lambda function invocation. You can specify the following values:</p> <ul> <li> <p> <code>viewer-request</code>: The function executes when CloudFront receives a request from a viewer and before it checks to see whether the requested object is in the edge cache. </p> </li> <li> <p> <code>origin-request</code>: The function executes only when CloudFront sends a request to your origin. When the requested object is in the edge cache, the function doesn&#39;t execute.</p> </li> <li> <p> <code>origin-response</code>: The function executes after CloudFront receives a response from the origin and before it caches the object in the response. When the requested object is in the edge cache, the function doesn&#39;t execute.</p> </li> <li> <p> <code>viewer-response</code>: The function executes before CloudFront returns the requested object to the viewer. The function executes regardless of whether the object was already in the edge cache.</p> <p>If the origin returns an HTTP status code other than HTTP 200 (OK), the function doesn&#39;t execute.</p> </li> </ul></p>
    pub event_type: String,
    /// <p>A flag that allows a Lambda function to have read access to the body content. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/lambda-include-body-access.html">Accessing the Request Body by Choosing the Include Body Option</a> in the Amazon CloudFront Developer Guide.</p>
    pub include_body: Option<bool>,
    /// <p>The ARN of the Lambda function. You must specify the ARN of a function version; you can't specify a Lambda alias or $LATEST.</p>
    pub lambda_function_arn: String,
}

#[allow(dead_code)]
struct LambdaFunctionAssociationDeserializer;
impl LambdaFunctionAssociationDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LambdaFunctionAssociation, XmlParseError> {
        deserialize_elements::<_, LambdaFunctionAssociation, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "EventType" => {
                        obj.event_type = EventTypeDeserializer::deserialize("EventType", stack)?;
                    }
                    "IncludeBody" => {
                        obj.include_body =
                            Some(BooleanDeserializer::deserialize("IncludeBody", stack)?);
                    }
                    "LambdaFunctionARN" => {
                        obj.lambda_function_arn =
                            LambdaFunctionARNDeserializer::deserialize("LambdaFunctionARN", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct LambdaFunctionAssociationSerializer;
impl LambdaFunctionAssociationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LambdaFunctionAssociation,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "EventType", &obj.event_type.to_string())?;
        if let Some(ref value) = obj.include_body {
            write_characters_element(writer, "IncludeBody", &value.to_string())?;
        }
        write_characters_element(
            writer,
            "LambdaFunctionARN",
            &obj.lambda_function_arn.to_string(),
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct LambdaFunctionAssociationListDeserializer;
impl LambdaFunctionAssociationListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LambdaFunctionAssociation>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "LambdaFunctionAssociation" {
                obj.push(LambdaFunctionAssociationDeserializer::deserialize(
                    "LambdaFunctionAssociation",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct LambdaFunctionAssociationListSerializer;
impl LambdaFunctionAssociationListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<LambdaFunctionAssociation>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            LambdaFunctionAssociationSerializer::serialize(
                writer,
                "LambdaFunctionAssociation",
                element,
            )?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A complex type that specifies a list of Lambda functions associations for a cache behavior.</p> <p>If you want to invoke one or more Lambda functions triggered by requests that match the <code>PathPattern</code> of the cache behavior, specify the applicable values for <code>Quantity</code> and <code>Items</code>. Note that there can be up to 4 <code>LambdaFunctionAssociation</code> items in this list (one for each possible value of <code>EventType</code>) and each <code>EventType</code> can be associated with the Lambda function only once.</p> <p>If you don't want to invoke any Lambda functions for the requests that match <code>PathPattern</code>, specify <code>0</code> for <code>Quantity</code> and omit <code>Items</code>. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LambdaFunctionAssociations {
    /// <p> <b>Optional</b>: A complex type that contains <code>LambdaFunctionAssociation</code> items for this cache behavior. If <code>Quantity</code> is <code>0</code>, you can omit <code>Items</code>.</p>
    pub items: Option<Vec<LambdaFunctionAssociation>>,
    /// <p>The number of Lambda function associations for this cache behavior.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct LambdaFunctionAssociationsDeserializer;
impl LambdaFunctionAssociationsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LambdaFunctionAssociations, XmlParseError> {
        deserialize_elements::<_, LambdaFunctionAssociations, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Items" => {
                        obj.items.get_or_insert(vec![]).extend(
                            LambdaFunctionAssociationListDeserializer::deserialize("Items", stack)?,
                        );
                    }
                    "Quantity" => {
                        obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct LambdaFunctionAssociationsSerializer;
impl LambdaFunctionAssociationsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LambdaFunctionAssociations,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &LambdaFunctionAssociationListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCachePoliciesRequest {
    /// <p>Use this field when paginating results to indicate where to begin in your list of cache policies. The response includes cache policies in the list that occur after the marker. To get the next page of the list, set this field’s value to the value of <code>NextMarker</code> from the current page’s response.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of cache policies that you want in the response.</p>
    pub max_items: Option<String>,
    /// <p><p>A filter to return only the specified kinds of cache policies. Valid values are:</p> <ul> <li> <p> <code>managed</code> – Returns only the managed policies created by AWS.</p> </li> <li> <p> <code>custom</code> – Returns only the custom policies created in your AWS account.</p> </li> </ul></p>
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListCachePoliciesResult {
    /// <p>A list of cache policies.</p>
    pub cache_policy_list: Option<CachePolicyList>,
}

#[allow(dead_code)]
struct ListCachePoliciesResultDeserializer;
impl ListCachePoliciesResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListCachePoliciesResult, XmlParseError> {
        Ok(ListCachePoliciesResult {
            cache_policy_list: Some(CachePolicyListDeserializer::deserialize(
                "CachePolicyList",
                stack,
            )?),
            ..ListCachePoliciesResult::default()
        })
    }
}
/// <p>The request to list origin access identities. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCloudFrontOriginAccessIdentitiesRequest {
    /// <p>Use this when paginating results to indicate where to begin in your list of origin access identities. The results include identities in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last identity on that page).</p>
    pub marker: Option<String>,
    /// <p>The maximum number of origin access identities you want in the response body. </p>
    pub max_items: Option<String>,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListCloudFrontOriginAccessIdentitiesResult {
    /// <p>The <code>CloudFrontOriginAccessIdentityList</code> type. </p>
    pub cloud_front_origin_access_identity_list: Option<CloudFrontOriginAccessIdentityList>,
}

#[allow(dead_code)]
struct ListCloudFrontOriginAccessIdentitiesResultDeserializer;
impl ListCloudFrontOriginAccessIdentitiesResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListCloudFrontOriginAccessIdentitiesResult, XmlParseError> {
        Ok(ListCloudFrontOriginAccessIdentitiesResult {
            cloud_front_origin_access_identity_list: Some(
                CloudFrontOriginAccessIdentityListDeserializer::deserialize(
                    "CloudFrontOriginAccessIdentityList",
                    stack,
                )?,
            ),
            ..ListCloudFrontOriginAccessIdentitiesResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDistributionsByCachePolicyIdRequest {
    /// <p>The ID of the cache policy whose associated distribution IDs you want to list.</p>
    pub cache_policy_id: String,
    /// <p>Use this field when paginating results to indicate where to begin in your list of distribution IDs. The response includes distribution IDs in the list that occur after the marker. To get the next page of the list, set this field’s value to the value of <code>NextMarker</code> from the current page’s response.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of distribution IDs that you want in the response.</p>
    pub max_items: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListDistributionsByCachePolicyIdResult {
    /// <p>A list of distribution IDs.</p>
    pub distribution_id_list: Option<DistributionIdList>,
}

#[allow(dead_code)]
struct ListDistributionsByCachePolicyIdResultDeserializer;
impl ListDistributionsByCachePolicyIdResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDistributionsByCachePolicyIdResult, XmlParseError> {
        Ok(ListDistributionsByCachePolicyIdResult {
            distribution_id_list: Some(DistributionIdListDeserializer::deserialize(
                "DistributionIdList",
                stack,
            )?),
            ..ListDistributionsByCachePolicyIdResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDistributionsByKeyGroupRequest {
    /// <p>The ID of the key group whose associated distribution IDs you are listing.</p>
    pub key_group_id: String,
    /// <p>Use this field when paginating results to indicate where to begin in your list of distribution IDs. The response includes distribution IDs in the list that occur after the marker. To get the next page of the list, set this field’s value to the value of <code>NextMarker</code> from the current page’s response.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of distribution IDs that you want in the response.</p>
    pub max_items: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListDistributionsByKeyGroupResult {
    pub distribution_id_list: Option<DistributionIdList>,
}

#[allow(dead_code)]
struct ListDistributionsByKeyGroupResultDeserializer;
impl ListDistributionsByKeyGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDistributionsByKeyGroupResult, XmlParseError> {
        Ok(ListDistributionsByKeyGroupResult {
            distribution_id_list: Some(DistributionIdListDeserializer::deserialize(
                "DistributionIdList",
                stack,
            )?),
            ..ListDistributionsByKeyGroupResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDistributionsByOriginRequestPolicyIdRequest {
    /// <p>Use this field when paginating results to indicate where to begin in your list of distribution IDs. The response includes distribution IDs in the list that occur after the marker. To get the next page of the list, set this field’s value to the value of <code>NextMarker</code> from the current page’s response.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of distribution IDs that you want in the response.</p>
    pub max_items: Option<String>,
    /// <p>The ID of the origin request policy whose associated distribution IDs you want to list.</p>
    pub origin_request_policy_id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListDistributionsByOriginRequestPolicyIdResult {
    /// <p>A list of distribution IDs.</p>
    pub distribution_id_list: Option<DistributionIdList>,
}

#[allow(dead_code)]
struct ListDistributionsByOriginRequestPolicyIdResultDeserializer;
impl ListDistributionsByOriginRequestPolicyIdResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDistributionsByOriginRequestPolicyIdResult, XmlParseError> {
        Ok(ListDistributionsByOriginRequestPolicyIdResult {
            distribution_id_list: Some(DistributionIdListDeserializer::deserialize(
                "DistributionIdList",
                stack,
            )?),
            ..ListDistributionsByOriginRequestPolicyIdResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDistributionsByRealtimeLogConfigRequest {
    /// <p>Use this field when paginating results to indicate where to begin in your list of distributions. The response includes distributions in the list that occur after the marker. To get the next page of the list, set this field’s value to the value of <code>NextMarker</code> from the current page’s response.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of distributions that you want in the response.</p>
    pub max_items: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the real-time log configuration whose associated distributions you want to list.</p>
    pub realtime_log_config_arn: Option<String>,
    /// <p>The name of the real-time log configuration whose associated distributions you want to list.</p>
    pub realtime_log_config_name: Option<String>,
}

pub struct ListDistributionsByRealtimeLogConfigRequestSerializer;
impl ListDistributionsByRealtimeLogConfigRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ListDistributionsByRealtimeLogConfigRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        if let Some(ref value) = obj.marker {
            &StringSerializer::serialize(&mut writer, "Marker", value)?;
        }
        if let Some(ref value) = obj.max_items {
            &StringSerializer::serialize(&mut writer, "MaxItems", value)?;
        }
        if let Some(ref value) = obj.realtime_log_config_arn {
            &StringSerializer::serialize(&mut writer, "RealtimeLogConfigArn", value)?;
        }
        if let Some(ref value) = obj.realtime_log_config_name {
            &StringSerializer::serialize(&mut writer, "RealtimeLogConfigName", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListDistributionsByRealtimeLogConfigResult {
    pub distribution_list: Option<DistributionList>,
}

#[allow(dead_code)]
struct ListDistributionsByRealtimeLogConfigResultDeserializer;
impl ListDistributionsByRealtimeLogConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDistributionsByRealtimeLogConfigResult, XmlParseError> {
        Ok(ListDistributionsByRealtimeLogConfigResult {
            distribution_list: Some(DistributionListDeserializer::deserialize(
                "DistributionList",
                stack,
            )?),
            ..ListDistributionsByRealtimeLogConfigResult::default()
        })
    }
}
/// <p>The request to list distributions that are associated with a specified AWS WAF web ACL. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDistributionsByWebACLIdRequest {
    /// <p>Use <code>Marker</code> and <code>MaxItems</code> to control pagination of results. If you have more than <code>MaxItems</code> distributions that satisfy the request, the response includes a <code>NextMarker</code> element. To get the next page of results, submit another request. For the value of <code>Marker</code>, specify the value of <code>NextMarker</code> from the last response. (For the first request, omit <code>Marker</code>.) </p>
    pub marker: Option<String>,
    /// <p>The maximum number of distributions that you want CloudFront to return in the response body. The maximum and default values are both 100.</p>
    pub max_items: Option<String>,
    /// <p>The ID of the AWS WAF web ACL that you want to list the associated distributions. If you specify "null" for the ID, the request returns a list of the distributions that aren't associated with a web ACL. </p>
    pub web_acl_id: String,
}

/// <p>The response to a request to list the distributions that are associated with a specified AWS WAF web ACL. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListDistributionsByWebACLIdResult {
    /// <p>The <code>DistributionList</code> type. </p>
    pub distribution_list: Option<DistributionList>,
}

#[allow(dead_code)]
struct ListDistributionsByWebACLIdResultDeserializer;
impl ListDistributionsByWebACLIdResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDistributionsByWebACLIdResult, XmlParseError> {
        Ok(ListDistributionsByWebACLIdResult {
            distribution_list: Some(DistributionListDeserializer::deserialize(
                "DistributionList",
                stack,
            )?),
            ..ListDistributionsByWebACLIdResult::default()
        })
    }
}
/// <p>The request to list your distributions. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDistributionsRequest {
    /// <p>Use this when paginating results to indicate where to begin in your list of distributions. The results include distributions in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last distribution on that page).</p>
    pub marker: Option<String>,
    /// <p>The maximum number of distributions you want in the response body.</p>
    pub max_items: Option<String>,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListDistributionsResult {
    /// <p>The <code>DistributionList</code> type. </p>
    pub distribution_list: Option<DistributionList>,
}

#[allow(dead_code)]
struct ListDistributionsResultDeserializer;
impl ListDistributionsResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDistributionsResult, XmlParseError> {
        Ok(ListDistributionsResult {
            distribution_list: Some(DistributionListDeserializer::deserialize(
                "DistributionList",
                stack,
            )?),
            ..ListDistributionsResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFieldLevelEncryptionConfigsRequest {
    /// <p>Use this when paginating results to indicate where to begin in your list of configurations. The results include configurations in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last configuration on that page). </p>
    pub marker: Option<String>,
    /// <p>The maximum number of field-level encryption configurations you want in the response body. </p>
    pub max_items: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListFieldLevelEncryptionConfigsResult {
    /// <p>Returns a list of all field-level encryption configurations that have been created in CloudFront for this account.</p>
    pub field_level_encryption_list: Option<FieldLevelEncryptionList>,
}

#[allow(dead_code)]
struct ListFieldLevelEncryptionConfigsResultDeserializer;
impl ListFieldLevelEncryptionConfigsResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListFieldLevelEncryptionConfigsResult, XmlParseError> {
        Ok(ListFieldLevelEncryptionConfigsResult {
            field_level_encryption_list: Some(FieldLevelEncryptionListDeserializer::deserialize(
                "FieldLevelEncryptionList",
                stack,
            )?),
            ..ListFieldLevelEncryptionConfigsResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFieldLevelEncryptionProfilesRequest {
    /// <p>Use this when paginating results to indicate where to begin in your list of profiles. The results include profiles in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last profile on that page). </p>
    pub marker: Option<String>,
    /// <p>The maximum number of field-level encryption profiles you want in the response body. </p>
    pub max_items: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListFieldLevelEncryptionProfilesResult {
    /// <p>Returns a list of the field-level encryption profiles that have been created in CloudFront for this account.</p>
    pub field_level_encryption_profile_list: Option<FieldLevelEncryptionProfileList>,
}

#[allow(dead_code)]
struct ListFieldLevelEncryptionProfilesResultDeserializer;
impl ListFieldLevelEncryptionProfilesResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListFieldLevelEncryptionProfilesResult, XmlParseError> {
        Ok(ListFieldLevelEncryptionProfilesResult {
            field_level_encryption_profile_list: Some(
                FieldLevelEncryptionProfileListDeserializer::deserialize(
                    "FieldLevelEncryptionProfileList",
                    stack,
                )?,
            ),
            ..ListFieldLevelEncryptionProfilesResult::default()
        })
    }
}
/// <p>The request to list invalidations. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInvalidationsRequest {
    /// <p>The distribution's ID.</p>
    pub distribution_id: String,
    /// <p>Use this parameter when paginating results to indicate where to begin in your list of invalidation batches. Because the results are returned in decreasing order from most recent to oldest, the most recent results are on the first page, the second page will contain earlier results, and so on. To get the next page of results, set <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response. This value is the same as the ID of the last invalidation batch on that page. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of invalidation batches that you want in the response body.</p>
    pub max_items: Option<String>,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListInvalidationsResult {
    /// <p>Information about invalidation batches. </p>
    pub invalidation_list: Option<InvalidationList>,
}

#[allow(dead_code)]
struct ListInvalidationsResultDeserializer;
impl ListInvalidationsResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListInvalidationsResult, XmlParseError> {
        Ok(ListInvalidationsResult {
            invalidation_list: Some(InvalidationListDeserializer::deserialize(
                "InvalidationList",
                stack,
            )?),
            ..ListInvalidationsResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListKeyGroupsRequest {
    /// <p>Use this field when paginating results to indicate where to begin in your list of key groups. The response includes key groups in the list that occur after the marker. To get the next page of the list, set this field’s value to the value of <code>NextMarker</code> from the current page’s response.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of key groups that you want in the response.</p>
    pub max_items: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListKeyGroupsResult {
    /// <p>A list of key groups.</p>
    pub key_group_list: Option<KeyGroupList>,
}

#[allow(dead_code)]
struct ListKeyGroupsResultDeserializer;
impl ListKeyGroupsResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListKeyGroupsResult, XmlParseError> {
        Ok(ListKeyGroupsResult {
            key_group_list: Some(KeyGroupListDeserializer::deserialize(
                "KeyGroupList",
                stack,
            )?),
            ..ListKeyGroupsResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOriginRequestPoliciesRequest {
    /// <p>Use this field when paginating results to indicate where to begin in your list of origin request policies. The response includes origin request policies in the list that occur after the marker. To get the next page of the list, set this field’s value to the value of <code>NextMarker</code> from the current page’s response.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of origin request policies that you want in the response.</p>
    pub max_items: Option<String>,
    /// <p><p>A filter to return only the specified kinds of origin request policies. Valid values are:</p> <ul> <li> <p> <code>managed</code> – Returns only the managed policies created by AWS.</p> </li> <li> <p> <code>custom</code> – Returns only the custom policies created in your AWS account.</p> </li> </ul></p>
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListOriginRequestPoliciesResult {
    /// <p>A list of origin request policies.</p>
    pub origin_request_policy_list: Option<OriginRequestPolicyList>,
}

#[allow(dead_code)]
struct ListOriginRequestPoliciesResultDeserializer;
impl ListOriginRequestPoliciesResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListOriginRequestPoliciesResult, XmlParseError> {
        Ok(ListOriginRequestPoliciesResult {
            origin_request_policy_list: Some(OriginRequestPolicyListDeserializer::deserialize(
                "OriginRequestPolicyList",
                stack,
            )?),
            ..ListOriginRequestPoliciesResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPublicKeysRequest {
    /// <p>Use this when paginating results to indicate where to begin in your list of public keys. The results include public keys in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last public key on that page). </p>
    pub marker: Option<String>,
    /// <p>The maximum number of public keys you want in the response body. </p>
    pub max_items: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListPublicKeysResult {
    /// <p>Returns a list of all public keys that have been added to CloudFront for this account.</p>
    pub public_key_list: Option<PublicKeyList>,
}

#[allow(dead_code)]
struct ListPublicKeysResultDeserializer;
impl ListPublicKeysResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListPublicKeysResult, XmlParseError> {
        Ok(ListPublicKeysResult {
            public_key_list: Some(PublicKeyListDeserializer::deserialize(
                "PublicKeyList",
                stack,
            )?),
            ..ListPublicKeysResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRealtimeLogConfigsRequest {
    /// <p>Use this field when paginating results to indicate where to begin in your list of real-time log configurations. The response includes real-time log configurations in the list that occur after the marker. To get the next page of the list, set this field’s value to the value of <code>NextMarker</code> from the current page’s response.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of real-time log configurations that you want in the response.</p>
    pub max_items: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListRealtimeLogConfigsResult {
    /// <p>A list of real-time log configurations.</p>
    pub realtime_log_configs: Option<RealtimeLogConfigs>,
}

#[allow(dead_code)]
struct ListRealtimeLogConfigsResultDeserializer;
impl ListRealtimeLogConfigsResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListRealtimeLogConfigsResult, XmlParseError> {
        Ok(ListRealtimeLogConfigsResult {
            realtime_log_configs: Some(RealtimeLogConfigsDeserializer::deserialize(
                "RealtimeLogConfigs",
                stack,
            )?),
            ..ListRealtimeLogConfigsResult::default()
        })
    }
}
/// <p>The request to list your streaming distributions. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListStreamingDistributionsRequest {
    /// <p>The value that you provided for the <code>Marker</code> request parameter.</p>
    pub marker: Option<String>,
    /// <p>The value that you provided for the <code>MaxItems</code> request parameter.</p>
    pub max_items: Option<String>,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListStreamingDistributionsResult {
    /// <p>The <code>StreamingDistributionList</code> type. </p>
    pub streaming_distribution_list: Option<StreamingDistributionList>,
}

#[allow(dead_code)]
struct ListStreamingDistributionsResultDeserializer;
impl ListStreamingDistributionsResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStreamingDistributionsResult, XmlParseError> {
        Ok(ListStreamingDistributionsResult {
            streaming_distribution_list: Some(StreamingDistributionListDeserializer::deserialize(
                "StreamingDistributionList",
                stack,
            )?),
            ..ListStreamingDistributionsResult::default()
        })
    }
}
/// <p> The request to list tags for a CloudFront resource.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p> An ARN of a CloudFront resource.</p>
    pub resource: String,
}

/// <p> The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListTagsForResourceResult {
    /// <p> A complex type that contains zero or more <code>Tag</code> elements.</p>
    pub tags: Tags,
}

#[allow(dead_code)]
struct ListTagsForResourceResultDeserializer;
impl ListTagsForResourceResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTagsForResourceResult, XmlParseError> {
        Ok(ListTagsForResourceResult {
            tags: TagsDeserializer::deserialize("Tags", stack)?,
            ..ListTagsForResourceResult::default()
        })
    }
}
#[allow(dead_code)]
struct LocationListDeserializer;
impl LocationListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Location" {
                obj.push(StringDeserializer::deserialize("Location", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct LocationListSerializer;
impl LocationListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            StringSerializer::serialize(writer, "Location", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A complex type that controls whether access logs are written for the distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LoggingConfig {
    /// <p>The Amazon S3 bucket to store the access logs in, for example, <code>myawslogbucket.s3.amazonaws.com</code>.</p>
    pub bucket: String,
    /// <p>Specifies whether you want CloudFront to save access logs to an Amazon S3 bucket. If you don't want to enable logging when you create a distribution or if you want to disable logging for an existing distribution, specify <code>false</code> for <code>Enabled</code>, and specify empty <code>Bucket</code> and <code>Prefix</code> elements. If you specify <code>false</code> for <code>Enabled</code> but you specify values for <code>Bucket</code>, <code>prefix</code>, and <code>IncludeCookies</code>, the values are automatically deleted.</p>
    pub enabled: bool,
    /// <p>Specifies whether you want CloudFront to include cookies in access logs, specify <code>true</code> for <code>IncludeCookies</code>. If you choose to include cookies in logs, CloudFront logs all cookies regardless of how you configure the cache behaviors for this distribution. If you don't want to include cookies when you create a distribution or if you want to disable include cookies for an existing distribution, specify <code>false</code> for <code>IncludeCookies</code>.</p>
    pub include_cookies: bool,
    /// <p>An optional string that you want CloudFront to prefix to the access log <code>filenames</code> for this distribution, for example, <code>myprefix/</code>. If you want to enable logging, but you don't want to specify a prefix, you still must include an empty <code>Prefix</code> element in the <code>Logging</code> element.</p>
    pub prefix: String,
}

#[allow(dead_code)]
struct LoggingConfigDeserializer;
impl LoggingConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoggingConfig, XmlParseError> {
        deserialize_elements::<_, LoggingConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Bucket" => {
                    obj.bucket = StringDeserializer::deserialize("Bucket", stack)?;
                }
                "Enabled" => {
                    obj.enabled = BooleanDeserializer::deserialize("Enabled", stack)?;
                }
                "IncludeCookies" => {
                    obj.include_cookies =
                        BooleanDeserializer::deserialize("IncludeCookies", stack)?;
                }
                "Prefix" => {
                    obj.prefix = StringDeserializer::deserialize("Prefix", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct LoggingConfigSerializer;
impl LoggingConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LoggingConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "Bucket", &obj.bucket.to_string())?;
        write_characters_element(writer, "Enabled", &obj.enabled.to_string())?;
        write_characters_element(writer, "IncludeCookies", &obj.include_cookies.to_string())?;
        write_characters_element(writer, "Prefix", &obj.prefix.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct LongDeserializer;
impl LongDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}

pub struct LongSerializer;
impl LongSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, &obj.to_string())
    }
}

#[allow(dead_code)]
struct MethodDeserializer;
impl MethodDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct MethodSerializer;
impl MethodSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

#[allow(dead_code)]
struct MethodsListDeserializer;
impl MethodsListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Method" {
                obj.push(MethodDeserializer::deserialize("Method", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct MethodsListSerializer;
impl MethodsListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            MethodSerializer::serialize(writer, "Method", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[allow(dead_code)]
struct MinimumProtocolVersionDeserializer;
impl MinimumProtocolVersionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct MinimumProtocolVersionSerializer;
impl MinimumProtocolVersionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>A monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MonitoringSubscription {
    /// <p>A subscription configuration for additional CloudWatch metrics.</p>
    pub realtime_metrics_subscription_config: Option<RealtimeMetricsSubscriptionConfig>,
}

#[allow(dead_code)]
struct MonitoringSubscriptionDeserializer;
impl MonitoringSubscriptionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MonitoringSubscription, XmlParseError> {
        deserialize_elements::<_, MonitoringSubscription, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "RealtimeMetricsSubscriptionConfig" => {
                    obj.realtime_metrics_subscription_config =
                        Some(RealtimeMetricsSubscriptionConfigDeserializer::deserialize(
                            "RealtimeMetricsSubscriptionConfig",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct MonitoringSubscriptionSerializer;
impl MonitoringSubscriptionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &MonitoringSubscription,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.realtime_metrics_subscription_config {
            &RealtimeMetricsSubscriptionConfigSerializer::serialize(
                &mut writer,
                "RealtimeMetricsSubscriptionConfig",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>An origin.</p> <p>An origin is the location where content is stored, and from which CloudFront gets content to serve to viewers. To specify an origin:</p> <ul> <li> <p>Use <code>S3OriginConfig</code> to specify an Amazon S3 bucket that is not configured with static website hosting.</p> </li> <li> <p>Use <code>CustomOriginConfig</code> to specify all other kinds of origins, including:</p> <ul> <li> <p>An Amazon S3 bucket that is configured with static website hosting</p> </li> <li> <p>An Elastic Load Balancing load balancer</p> </li> <li> <p>An AWS Elemental MediaPackage endpoint</p> </li> <li> <p>An AWS Elemental MediaStore container</p> </li> <li> <p>Any other HTTP server, running on an Amazon EC2 instance or any other kind of host</p> </li> </ul> </li> </ul> <p>For the current maximum number of origins that you can specify per distribution, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html#limits-web-distributions">General Quotas on Web Distributions</a> in the <i>Amazon CloudFront Developer Guide</i> (quotas were formerly referred to as limits).</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Origin {
    /// <p>The number of times that CloudFront attempts to connect to the origin. The minimum number is 1, the maximum is 3, and the default (if you don’t specify otherwise) is 3.</p> <p>For a custom origin (including an Amazon S3 bucket that’s configured with static website hosting), this value also specifies the number of times that CloudFront attempts to get a response from the origin, in the case of an <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesOriginResponseTimeout">Origin Response Timeout</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#origin-connection-attempts">Origin Connection Attempts</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub connection_attempts: Option<i64>,
    /// <p>The number of seconds that CloudFront waits when trying to establish a connection to the origin. The minimum timeout is 1 second, the maximum is 10 seconds, and the default (if you don’t specify otherwise) is 10 seconds.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#origin-connection-timeout">Origin Connection Timeout</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub connection_timeout: Option<i64>,
    /// <p>A list of HTTP header names and values that CloudFront adds to the requests that it sends to the origin.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/add-origin-custom-headers.html">Adding Custom Headers to Origin Requests</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub custom_headers: Option<CustomHeaders>,
    /// <p>Use this type to specify an origin that is not an Amazon S3 bucket, with one exception. If the Amazon S3 bucket is configured with static website hosting, use this type. If the Amazon S3 bucket is not configured with static website hosting, use the <code>S3OriginConfig</code> type instead.</p>
    pub custom_origin_config: Option<CustomOriginConfig>,
    /// <p>The domain name for the origin.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesDomainName">Origin Domain Name</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub domain_name: String,
    /// <p>A unique identifier for the origin. This value must be unique within the distribution.</p> <p>Use this value to specify the <code>TargetOriginId</code> in a <code>CacheBehavior</code> or <code>DefaultCacheBehavior</code>.</p>
    pub id: String,
    /// <p>An optional path that CloudFront appends to the origin domain name when CloudFront requests content from the origin.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesOriginPath">Origin Path</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub origin_path: Option<String>,
    /// <p>CloudFront Origin Shield. Using Origin Shield can help reduce the load on your origin.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/origin-shield.html">Using Origin Shield</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub origin_shield: Option<OriginShield>,
    /// <p>Use this type to specify an origin that is an Amazon S3 bucket that is not configured with static website hosting. To specify any other type of origin, including an Amazon S3 bucket that is configured with static website hosting, use the <code>CustomOriginConfig</code> type instead.</p>
    pub s3_origin_config: Option<S3OriginConfig>,
}

#[allow(dead_code)]
struct OriginDeserializer;
impl OriginDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Origin, XmlParseError> {
        deserialize_elements::<_, Origin, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ConnectionAttempts" => {
                    obj.connection_attempts = Some(IntegerDeserializer::deserialize(
                        "ConnectionAttempts",
                        stack,
                    )?);
                }
                "ConnectionTimeout" => {
                    obj.connection_timeout = Some(IntegerDeserializer::deserialize(
                        "ConnectionTimeout",
                        stack,
                    )?);
                }
                "CustomHeaders" => {
                    obj.custom_headers = Some(CustomHeadersDeserializer::deserialize(
                        "CustomHeaders",
                        stack,
                    )?);
                }
                "CustomOriginConfig" => {
                    obj.custom_origin_config = Some(CustomOriginConfigDeserializer::deserialize(
                        "CustomOriginConfig",
                        stack,
                    )?);
                }
                "DomainName" => {
                    obj.domain_name = StringDeserializer::deserialize("DomainName", stack)?;
                }
                "Id" => {
                    obj.id = StringDeserializer::deserialize("Id", stack)?;
                }
                "OriginPath" => {
                    obj.origin_path = Some(StringDeserializer::deserialize("OriginPath", stack)?);
                }
                "OriginShield" => {
                    obj.origin_shield = Some(OriginShieldDeserializer::deserialize(
                        "OriginShield",
                        stack,
                    )?);
                }
                "S3OriginConfig" => {
                    obj.s3_origin_config = Some(S3OriginConfigDeserializer::deserialize(
                        "S3OriginConfig",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct OriginSerializer;
impl OriginSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Origin,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.connection_attempts {
            write_characters_element(writer, "ConnectionAttempts", &value.to_string())?;
        }
        if let Some(ref value) = obj.connection_timeout {
            write_characters_element(writer, "ConnectionTimeout", &value.to_string())?;
        }
        if let Some(ref value) = obj.custom_headers {
            &CustomHeadersSerializer::serialize(&mut writer, "CustomHeaders", value)?;
        }
        if let Some(ref value) = obj.custom_origin_config {
            &CustomOriginConfigSerializer::serialize(&mut writer, "CustomOriginConfig", value)?;
        }
        write_characters_element(writer, "DomainName", &obj.domain_name.to_string())?;
        write_characters_element(writer, "Id", &obj.id.to_string())?;
        if let Some(ref value) = obj.origin_path {
            write_characters_element(writer, "OriginPath", &value.to_string())?;
        }
        if let Some(ref value) = obj.origin_shield {
            &OriginShieldSerializer::serialize(&mut writer, "OriginShield", value)?;
        }
        if let Some(ref value) = obj.s3_origin_config {
            &S3OriginConfigSerializer::serialize(&mut writer, "S3OriginConfig", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that contains <code>HeaderName</code> and <code>HeaderValue</code> elements, if any, for this distribution. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginCustomHeader {
    /// <p>The name of a header that you want CloudFront to send to your origin. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/forward-custom-headers.html">Adding Custom Headers to Origin Requests</a> in the <i> Amazon CloudFront Developer Guide</i>.</p>
    pub header_name: String,
    /// <p>The value for the header that you specified in the <code>HeaderName</code> field.</p>
    pub header_value: String,
}

#[allow(dead_code)]
struct OriginCustomHeaderDeserializer;
impl OriginCustomHeaderDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginCustomHeader, XmlParseError> {
        deserialize_elements::<_, OriginCustomHeader, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "HeaderName" => {
                    obj.header_name = StringDeserializer::deserialize("HeaderName", stack)?;
                }
                "HeaderValue" => {
                    obj.header_value = StringDeserializer::deserialize("HeaderValue", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct OriginCustomHeaderSerializer;
impl OriginCustomHeaderSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OriginCustomHeader,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "HeaderName", &obj.header_name.to_string())?;
        write_characters_element(writer, "HeaderValue", &obj.header_value.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct OriginCustomHeadersListDeserializer;
impl OriginCustomHeadersListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<OriginCustomHeader>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "OriginCustomHeader" {
                obj.push(OriginCustomHeaderDeserializer::deserialize(
                    "OriginCustomHeader",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct OriginCustomHeadersListSerializer;
impl OriginCustomHeadersListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<OriginCustomHeader>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            OriginCustomHeaderSerializer::serialize(writer, "OriginCustomHeader", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>An origin group includes two origins (a primary origin and a second origin to failover to) and a failover criteria that you specify. You create an origin group to support origin failover in CloudFront. When you create or update a distribution, you can specifiy the origin group instead of a single origin, and CloudFront will failover from the primary origin to the second origin under the failover conditions that you've chosen.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginGroup {
    /// <p>A complex type that contains information about the failover criteria for an origin group.</p>
    pub failover_criteria: OriginGroupFailoverCriteria,
    /// <p>The origin group's ID.</p>
    pub id: String,
    /// <p>A complex type that contains information about the origins in an origin group.</p>
    pub members: OriginGroupMembers,
}

#[allow(dead_code)]
struct OriginGroupDeserializer;
impl OriginGroupDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginGroup, XmlParseError> {
        deserialize_elements::<_, OriginGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "FailoverCriteria" => {
                    obj.failover_criteria = OriginGroupFailoverCriteriaDeserializer::deserialize(
                        "FailoverCriteria",
                        stack,
                    )?;
                }
                "Id" => {
                    obj.id = StringDeserializer::deserialize("Id", stack)?;
                }
                "Members" => {
                    obj.members = OriginGroupMembersDeserializer::deserialize("Members", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct OriginGroupSerializer;
impl OriginGroupSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OriginGroup,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        OriginGroupFailoverCriteriaSerializer::serialize(
            &mut writer,
            "FailoverCriteria",
            &obj.failover_criteria,
        )?;
        write_characters_element(writer, "Id", &obj.id.to_string())?;
        OriginGroupMembersSerializer::serialize(&mut writer, "Members", &obj.members)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex data type that includes information about the failover criteria for an origin group, including the status codes for which CloudFront will failover from the primary origin to the second origin.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginGroupFailoverCriteria {
    /// <p>The status codes that, when returned from the primary origin, will trigger CloudFront to failover to the second origin.</p>
    pub status_codes: StatusCodes,
}

#[allow(dead_code)]
struct OriginGroupFailoverCriteriaDeserializer;
impl OriginGroupFailoverCriteriaDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginGroupFailoverCriteria, XmlParseError> {
        deserialize_elements::<_, OriginGroupFailoverCriteria, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "StatusCodes" => {
                        obj.status_codes =
                            StatusCodesDeserializer::deserialize("StatusCodes", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct OriginGroupFailoverCriteriaSerializer;
impl OriginGroupFailoverCriteriaSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OriginGroupFailoverCriteria,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        StatusCodesSerializer::serialize(&mut writer, "StatusCodes", &obj.status_codes)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct OriginGroupListDeserializer;
impl OriginGroupListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<OriginGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "OriginGroup" {
                obj.push(OriginGroupDeserializer::deserialize("OriginGroup", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct OriginGroupListSerializer;
impl OriginGroupListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<OriginGroup>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            OriginGroupSerializer::serialize(writer, "OriginGroup", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>An origin in an origin group.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginGroupMember {
    /// <p>The ID for an origin in an origin group.</p>
    pub origin_id: String,
}

#[allow(dead_code)]
struct OriginGroupMemberDeserializer;
impl OriginGroupMemberDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginGroupMember, XmlParseError> {
        deserialize_elements::<_, OriginGroupMember, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "OriginId" => {
                    obj.origin_id = StringDeserializer::deserialize("OriginId", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct OriginGroupMemberSerializer;
impl OriginGroupMemberSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OriginGroupMember,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "OriginId", &obj.origin_id.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct OriginGroupMemberListDeserializer;
impl OriginGroupMemberListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<OriginGroupMember>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "OriginGroupMember" {
                obj.push(OriginGroupMemberDeserializer::deserialize(
                    "OriginGroupMember",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct OriginGroupMemberListSerializer;
impl OriginGroupMemberListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<OriginGroupMember>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            OriginGroupMemberSerializer::serialize(writer, "OriginGroupMember", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A complex data type for the origins included in an origin group.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginGroupMembers {
    /// <p>Items (origins) in an origin group.</p>
    pub items: Vec<OriginGroupMember>,
    /// <p>The number of origins in an origin group.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct OriginGroupMembersDeserializer;
impl OriginGroupMembersDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginGroupMembers, XmlParseError> {
        deserialize_elements::<_, OriginGroupMembers, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .extend(OriginGroupMemberListDeserializer::deserialize(
                            "Items", stack,
                        )?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct OriginGroupMembersSerializer;
impl OriginGroupMembersSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OriginGroupMembers,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        OriginGroupMemberListSerializer::serialize(&mut writer, "Items", &obj.items)?;
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex data type for the origin groups specified for a distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginGroups {
    /// <p>The items (origin groups) in a distribution.</p>
    pub items: Option<Vec<OriginGroup>>,
    /// <p>The number of origin groups.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct OriginGroupsDeserializer;
impl OriginGroupsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginGroups, XmlParseError> {
        deserialize_elements::<_, OriginGroups, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .get_or_insert(vec![])
                        .extend(OriginGroupListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct OriginGroupsSerializer;
impl OriginGroupsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OriginGroups,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &OriginGroupListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct OriginListDeserializer;
impl OriginListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Origin>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Origin" {
                obj.push(OriginDeserializer::deserialize("Origin", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct OriginListSerializer;
impl OriginListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<Origin>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            OriginSerializer::serialize(writer, "Origin", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[allow(dead_code)]
struct OriginProtocolPolicyDeserializer;
impl OriginProtocolPolicyDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct OriginProtocolPolicySerializer;
impl OriginProtocolPolicySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>An origin request policy.</p> <p>When it’s attached to a cache behavior, the origin request policy determines the values that CloudFront includes in requests that it sends to the origin. Each request that CloudFront sends to the origin includes the following:</p> <ul> <li> <p>The request body and the URL path (without the domain name) from the viewer request.</p> </li> <li> <p>The headers that CloudFront automatically includes in every origin request, including <code>Host</code>, <code>User-Agent</code>, and <code>X-Amz-Cf-Id</code>.</p> </li> <li> <p>All HTTP headers, cookies, and URL query strings that are specified in the cache policy or the origin request policy. These can include items from the viewer request and, in the case of headers, additional ones that are added by CloudFront.</p> </li> </ul> <p>CloudFront sends a request when it can’t find an object in its cache that matches the request. If you want to send values to the origin and also include them in the cache key, use <code>CachePolicy</code>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct OriginRequestPolicy {
    /// <p>The unique identifier for the origin request policy.</p>
    pub id: String,
    /// <p>The date and time when the origin request policy was last modified.</p>
    pub last_modified_time: String,
    /// <p>The origin request policy configuration.</p>
    pub origin_request_policy_config: OriginRequestPolicyConfig,
}

#[allow(dead_code)]
struct OriginRequestPolicyDeserializer;
impl OriginRequestPolicyDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginRequestPolicy, XmlParseError> {
        deserialize_elements::<_, OriginRequestPolicy, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Id" => {
                    obj.id = StringDeserializer::deserialize("Id", stack)?;
                }
                "LastModifiedTime" => {
                    obj.last_modified_time =
                        TimestampDeserializer::deserialize("LastModifiedTime", stack)?;
                }
                "OriginRequestPolicyConfig" => {
                    obj.origin_request_policy_config =
                        OriginRequestPolicyConfigDeserializer::deserialize(
                            "OriginRequestPolicyConfig",
                            stack,
                        )?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>An origin request policy configuration.</p> <p>This configuration determines the values that CloudFront includes in requests that it sends to the origin. Each request that CloudFront sends to the origin includes the following:</p> <ul> <li> <p>The request body and the URL path (without the domain name) from the viewer request.</p> </li> <li> <p>The headers that CloudFront automatically includes in every origin request, including <code>Host</code>, <code>User-Agent</code>, and <code>X-Amz-Cf-Id</code>.</p> </li> <li> <p>All HTTP headers, cookies, and URL query strings that are specified in the cache policy or the origin request policy. These can include items from the viewer request and, in the case of headers, additional ones that are added by CloudFront.</p> </li> </ul> <p>CloudFront sends a request when it can’t find an object in its cache that matches the request. If you want to send values to the origin and also include them in the cache key, use <code>CachePolicy</code>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginRequestPolicyConfig {
    /// <p>A comment to describe the origin request policy.</p>
    pub comment: Option<String>,
    /// <p>The cookies from viewer requests to include in origin requests.</p>
    pub cookies_config: OriginRequestPolicyCookiesConfig,
    /// <p>The HTTP headers to include in origin requests. These can include headers from viewer requests and additional headers added by CloudFront.</p>
    pub headers_config: OriginRequestPolicyHeadersConfig,
    /// <p>A unique name to identify the origin request policy.</p>
    pub name: String,
    /// <p>The URL query strings from viewer requests to include in origin requests.</p>
    pub query_strings_config: OriginRequestPolicyQueryStringsConfig,
}

#[allow(dead_code)]
struct OriginRequestPolicyConfigDeserializer;
impl OriginRequestPolicyConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginRequestPolicyConfig, XmlParseError> {
        deserialize_elements::<_, OriginRequestPolicyConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Comment" => {
                        obj.comment = Some(StringDeserializer::deserialize("Comment", stack)?);
                    }
                    "CookiesConfig" => {
                        obj.cookies_config =
                            OriginRequestPolicyCookiesConfigDeserializer::deserialize(
                                "CookiesConfig",
                                stack,
                            )?;
                    }
                    "HeadersConfig" => {
                        obj.headers_config =
                            OriginRequestPolicyHeadersConfigDeserializer::deserialize(
                                "HeadersConfig",
                                stack,
                            )?;
                    }
                    "Name" => {
                        obj.name = StringDeserializer::deserialize("Name", stack)?;
                    }
                    "QueryStringsConfig" => {
                        obj.query_strings_config =
                            OriginRequestPolicyQueryStringsConfigDeserializer::deserialize(
                                "QueryStringsConfig",
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

pub struct OriginRequestPolicyConfigSerializer;
impl OriginRequestPolicyConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OriginRequestPolicyConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.comment {
            write_characters_element(writer, "Comment", &value.to_string())?;
        }
        OriginRequestPolicyCookiesConfigSerializer::serialize(
            &mut writer,
            "CookiesConfig",
            &obj.cookies_config,
        )?;
        OriginRequestPolicyHeadersConfigSerializer::serialize(
            &mut writer,
            "HeadersConfig",
            &obj.headers_config,
        )?;
        write_characters_element(writer, "Name", &obj.name.to_string())?;
        OriginRequestPolicyQueryStringsConfigSerializer::serialize(
            &mut writer,
            "QueryStringsConfig",
            &obj.query_strings_config,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct OriginRequestPolicyCookieBehaviorDeserializer;
impl OriginRequestPolicyCookieBehaviorDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct OriginRequestPolicyCookieBehaviorSerializer;
impl OriginRequestPolicyCookieBehaviorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>An object that determines whether any cookies in viewer requests (and if so, which cookies) are included in requests that CloudFront sends to the origin.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginRequestPolicyCookiesConfig {
    /// <p><p>Determines whether cookies in viewer requests are included in requests that CloudFront sends to the origin. Valid values are:</p> <ul> <li> <p> <code>none</code> – Cookies in viewer requests are not included in requests that CloudFront sends to the origin. Even when this field is set to <code>none</code>, any cookies that are listed in a <code>CachePolicy</code> <i>are</i> included in origin requests.</p> </li> <li> <p> <code>whitelist</code> – The cookies in viewer requests that are listed in the <code>CookieNames</code> type are included in requests that CloudFront sends to the origin.</p> </li> <li> <p> <code>all</code> – All cookies in viewer requests are included in requests that CloudFront sends to the origin.</p> </li> </ul></p>
    pub cookie_behavior: String,
    pub cookies: Option<CookieNames>,
}

#[allow(dead_code)]
struct OriginRequestPolicyCookiesConfigDeserializer;
impl OriginRequestPolicyCookiesConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginRequestPolicyCookiesConfig, XmlParseError> {
        deserialize_elements::<_, OriginRequestPolicyCookiesConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CookieBehavior" => {
                        obj.cookie_behavior =
                            OriginRequestPolicyCookieBehaviorDeserializer::deserialize(
                                "CookieBehavior",
                                stack,
                            )?;
                    }
                    "Cookies" => {
                        obj.cookies = Some(CookieNamesDeserializer::deserialize("Cookies", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct OriginRequestPolicyCookiesConfigSerializer;
impl OriginRequestPolicyCookiesConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OriginRequestPolicyCookiesConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "CookieBehavior", &obj.cookie_behavior.to_string())?;
        if let Some(ref value) = obj.cookies {
            &CookieNamesSerializer::serialize(&mut writer, "Cookies", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct OriginRequestPolicyHeaderBehaviorDeserializer;
impl OriginRequestPolicyHeaderBehaviorDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct OriginRequestPolicyHeaderBehaviorSerializer;
impl OriginRequestPolicyHeaderBehaviorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>An object that determines whether any HTTP headers (and if so, which headers) are included in requests that CloudFront sends to the origin.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginRequestPolicyHeadersConfig {
    /// <p><p>Determines whether any HTTP headers are included in requests that CloudFront sends to the origin. Valid values are:</p> <ul> <li> <p> <code>none</code> – HTTP headers are not included in requests that CloudFront sends to the origin. Even when this field is set to <code>none</code>, any headers that are listed in a <code>CachePolicy</code> <i>are</i> included in origin requests.</p> </li> <li> <p> <code>whitelist</code> – The HTTP headers that are listed in the <code>Headers</code> type are included in requests that CloudFront sends to the origin.</p> </li> <li> <p> <code>allViewer</code> – All HTTP headers in viewer requests are included in requests that CloudFront sends to the origin.</p> </li> <li> <p> <code>allViewerAndWhitelistCloudFront</code> – All HTTP headers in viewer requests and the additional CloudFront headers that are listed in the <code>Headers</code> type are included in requests that CloudFront sends to the origin. The additional headers are added by CloudFront.</p> </li> </ul></p>
    pub header_behavior: String,
    pub headers: Option<Headers>,
}

#[allow(dead_code)]
struct OriginRequestPolicyHeadersConfigDeserializer;
impl OriginRequestPolicyHeadersConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginRequestPolicyHeadersConfig, XmlParseError> {
        deserialize_elements::<_, OriginRequestPolicyHeadersConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HeaderBehavior" => {
                        obj.header_behavior =
                            OriginRequestPolicyHeaderBehaviorDeserializer::deserialize(
                                "HeaderBehavior",
                                stack,
                            )?;
                    }
                    "Headers" => {
                        obj.headers = Some(HeadersDeserializer::deserialize("Headers", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct OriginRequestPolicyHeadersConfigSerializer;
impl OriginRequestPolicyHeadersConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OriginRequestPolicyHeadersConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "HeaderBehavior", &obj.header_behavior.to_string())?;
        if let Some(ref value) = obj.headers {
            &HeadersSerializer::serialize(&mut writer, "Headers", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A list of origin request policies.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct OriginRequestPolicyList {
    /// <p>Contains the origin request policies in the list.</p>
    pub items: Option<Vec<OriginRequestPolicySummary>>,
    /// <p>The maximum number of origin request policies requested.</p>
    pub max_items: i64,
    /// <p>If there are more items in the list than are in this response, this element is present. It contains the value that you should use in the <code>Marker</code> field of a subsequent request to continue listing origin request policies where you left off.</p>
    pub next_marker: Option<String>,
    /// <p>The total number of origin request policies returned in the response.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct OriginRequestPolicyListDeserializer;
impl OriginRequestPolicyListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginRequestPolicyList, XmlParseError> {
        deserialize_elements::<_, OriginRequestPolicyList, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Items" => {
                        obj.items.get_or_insert(vec![]).extend(
                            OriginRequestPolicySummaryListDeserializer::deserialize(
                                "Items", stack,
                            )?,
                        );
                    }
                    "MaxItems" => {
                        obj.max_items = IntegerDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(StringDeserializer::deserialize("NextMarker", stack)?);
                    }
                    "Quantity" => {
                        obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct OriginRequestPolicyQueryStringBehaviorDeserializer;
impl OriginRequestPolicyQueryStringBehaviorDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct OriginRequestPolicyQueryStringBehaviorSerializer;
impl OriginRequestPolicyQueryStringBehaviorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>An object that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in requests that CloudFront sends to the origin.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginRequestPolicyQueryStringsConfig {
    /// <p><p>Determines whether any URL query strings in viewer requests are included in requests that CloudFront sends to the origin. Valid values are:</p> <ul> <li> <p> <code>none</code> – Query strings in viewer requests are not included in requests that CloudFront sends to the origin. Even when this field is set to <code>none</code>, any query strings that are listed in a <code>CachePolicy</code> <i>are</i> included in origin requests.</p> </li> <li> <p> <code>whitelist</code> – The query strings in viewer requests that are listed in the <code>QueryStringNames</code> type are included in requests that CloudFront sends to the origin.</p> </li> <li> <p> <code>all</code> – All query strings in viewer requests are included in requests that CloudFront sends to the origin.</p> </li> </ul></p>
    pub query_string_behavior: String,
    /// <p>Contains a list of the query strings in viewer requests that are included in requests that CloudFront sends to the origin.</p>
    pub query_strings: Option<QueryStringNames>,
}

#[allow(dead_code)]
struct OriginRequestPolicyQueryStringsConfigDeserializer;
impl OriginRequestPolicyQueryStringsConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginRequestPolicyQueryStringsConfig, XmlParseError> {
        deserialize_elements::<_, OriginRequestPolicyQueryStringsConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "QueryStringBehavior" => {
                        obj.query_string_behavior =
                            OriginRequestPolicyQueryStringBehaviorDeserializer::deserialize(
                                "QueryStringBehavior",
                                stack,
                            )?;
                    }
                    "QueryStrings" => {
                        obj.query_strings = Some(QueryStringNamesDeserializer::deserialize(
                            "QueryStrings",
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

pub struct OriginRequestPolicyQueryStringsConfigSerializer;
impl OriginRequestPolicyQueryStringsConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OriginRequestPolicyQueryStringsConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(
            writer,
            "QueryStringBehavior",
            &obj.query_string_behavior.to_string(),
        )?;
        if let Some(ref value) = obj.query_strings {
            &QueryStringNamesSerializer::serialize(&mut writer, "QueryStrings", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Contains an origin request policy.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct OriginRequestPolicySummary {
    /// <p>The origin request policy.</p>
    pub origin_request_policy: OriginRequestPolicy,
    /// <p>The type of origin request policy, either <code>managed</code> (created by AWS) or <code>custom</code> (created in this AWS account).</p>
    pub type_: String,
}

#[allow(dead_code)]
struct OriginRequestPolicySummaryDeserializer;
impl OriginRequestPolicySummaryDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginRequestPolicySummary, XmlParseError> {
        deserialize_elements::<_, OriginRequestPolicySummary, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "OriginRequestPolicy" => {
                        obj.origin_request_policy = OriginRequestPolicyDeserializer::deserialize(
                            "OriginRequestPolicy",
                            stack,
                        )?;
                    }
                    "Type" => {
                        obj.type_ =
                            OriginRequestPolicyTypeDeserializer::deserialize("Type", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct OriginRequestPolicySummaryListDeserializer;
impl OriginRequestPolicySummaryListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<OriginRequestPolicySummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "OriginRequestPolicySummary" {
                obj.push(OriginRequestPolicySummaryDeserializer::deserialize(
                    "OriginRequestPolicySummary",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct OriginRequestPolicyTypeDeserializer;
impl OriginRequestPolicyTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct OriginRequestPolicyTypeSerializer;
impl OriginRequestPolicyTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>CloudFront Origin Shield.</p> <p>Using Origin Shield can help reduce the load on your origin. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/origin-shield.html">Using Origin Shield</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginShield {
    /// <p>A flag that specifies whether Origin Shield is enabled.</p> <p>When it’s enabled, CloudFront routes all requests through Origin Shield, which can help protect your origin. When it’s disabled, CloudFront might send requests directly to your origin from multiple edge locations or regional edge caches.</p>
    pub enabled: bool,
    /// <p>The AWS Region for Origin Shield.</p> <p>Specify the AWS Region that has the lowest latency to your origin. To specify a region, use the region code, not the region name. For example, specify the US East (Ohio) region as <code>us-east-2</code>.</p> <p>When you enable CloudFront Origin Shield, you must specify the AWS Region for Origin Shield. For the list of AWS Regions that you can specify, and for help choosing the best Region for your origin, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/origin-shield.html#choose-origin-shield-region">Choosing the AWS Region for Origin Shield</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub origin_shield_region: Option<String>,
}

#[allow(dead_code)]
struct OriginShieldDeserializer;
impl OriginShieldDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginShield, XmlParseError> {
        deserialize_elements::<_, OriginShield, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Enabled" => {
                    obj.enabled = BooleanDeserializer::deserialize("Enabled", stack)?;
                }
                "OriginShieldRegion" => {
                    obj.origin_shield_region = Some(OriginShieldRegionDeserializer::deserialize(
                        "OriginShieldRegion",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct OriginShieldSerializer;
impl OriginShieldSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OriginShield,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "Enabled", &obj.enabled.to_string())?;
        if let Some(ref value) = obj.origin_shield_region {
            write_characters_element(writer, "OriginShieldRegion", &value.to_string())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct OriginShieldRegionDeserializer;
impl OriginShieldRegionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct OriginShieldRegionSerializer;
impl OriginShieldRegionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>A complex type that contains information about the SSL/TLS protocols that CloudFront can use when establishing an HTTPS connection with your origin. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginSslProtocols {
    /// <p>A list that contains allowed SSL/TLS protocols for this distribution.</p>
    pub items: Vec<String>,
    /// <p>The number of SSL/TLS protocols that you want to allow CloudFront to use when establishing an HTTPS connection with this origin. </p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct OriginSslProtocolsDeserializer;
impl OriginSslProtocolsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginSslProtocols, XmlParseError> {
        deserialize_elements::<_, OriginSslProtocols, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .extend(SslProtocolsListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct OriginSslProtocolsSerializer;
impl OriginSslProtocolsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OriginSslProtocols,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        SslProtocolsListSerializer::serialize(&mut writer, "Items", &obj.items)?;
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Contains information about the origins for this distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Origins {
    /// <p>A list of origins.</p>
    pub items: Vec<Origin>,
    /// <p>The number of origins for this distribution.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct OriginsDeserializer;
impl OriginsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Origins, XmlParseError> {
        deserialize_elements::<_, Origins, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .extend(OriginListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct OriginsSerializer;
impl OriginsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Origins,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        OriginListSerializer::serialize(&mut writer, "Items", &obj.items)?;
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>This object determines the values that CloudFront includes in the cache key. These values can include HTTP headers, cookies, and URL query strings. CloudFront uses the cache key to find an object in its cache that it can return to the viewer.</p> <p>The headers, cookies, and query strings that are included in the cache key are automatically included in requests that CloudFront sends to the origin. CloudFront sends a request when it can’t find an object in its cache that matches the request’s cache key. If you want to send values to the origin but <i>not</i> include them in the cache key, use <code>OriginRequestPolicy</code>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ParametersInCacheKeyAndForwardedToOrigin {
    /// <p>An object that determines whether any cookies in viewer requests (and if so, which cookies) are included in the cache key and automatically included in requests that CloudFront sends to the origin.</p>
    pub cookies_config: CachePolicyCookiesConfig,
    /// <p>A flag that can affect whether the <code>Accept-Encoding</code> HTTP header is included in the cache key and included in requests that CloudFront sends to the origin.</p> <p>This field is related to the <code>EnableAcceptEncodingGzip</code> field. If one or both of these fields is <code>true</code> <i>and</i> the viewer request includes the <code>Accept-Encoding</code> header, then CloudFront does the following:</p> <ul> <li> <p>Normalizes the value of the viewer’s <code>Accept-Encoding</code> header</p> </li> <li> <p>Includes the normalized header in the cache key</p> </li> <li> <p>Includes the normalized header in the request to the origin, if a request is necessary</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html#cache-policy-compressed-objects">Compression support</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>If you set this value to <code>true</code>, and this cache behavior also has an origin request policy attached, do not include the <code>Accept-Encoding</code> header in the origin request policy. CloudFront always includes the <code>Accept-Encoding</code> header in origin requests when the value of this field is <code>true</code>, so including this header in an origin request policy has no effect.</p> <p>If both of these fields are <code>false</code>, then CloudFront treats the <code>Accept-Encoding</code> header the same as any other HTTP header in the viewer request. By default, it’s not included in the cache key and it’s not included in origin requests. In this case, you can manually add <code>Accept-Encoding</code> to the headers whitelist like any other HTTP header.</p>
    pub enable_accept_encoding_brotli: Option<bool>,
    /// <p>A flag that can affect whether the <code>Accept-Encoding</code> HTTP header is included in the cache key and included in requests that CloudFront sends to the origin.</p> <p>This field is related to the <code>EnableAcceptEncodingBrotli</code> field. If one or both of these fields is <code>true</code> <i>and</i> the viewer request includes the <code>Accept-Encoding</code> header, then CloudFront does the following:</p> <ul> <li> <p>Normalizes the value of the viewer’s <code>Accept-Encoding</code> header</p> </li> <li> <p>Includes the normalized header in the cache key</p> </li> <li> <p>Includes the normalized header in the request to the origin, if a request is necessary</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html#cache-policy-compressed-objects">Compression support</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>If you set this value to <code>true</code>, and this cache behavior also has an origin request policy attached, do not include the <code>Accept-Encoding</code> header in the origin request policy. CloudFront always includes the <code>Accept-Encoding</code> header in origin requests when the value of this field is <code>true</code>, so including this header in an origin request policy has no effect.</p> <p>If both of these fields are <code>false</code>, then CloudFront treats the <code>Accept-Encoding</code> header the same as any other HTTP header in the viewer request. By default, it’s not included in the cache key and it’s not included in origin requests. In this case, you can manually add <code>Accept-Encoding</code> to the headers whitelist like any other HTTP header.</p>
    pub enable_accept_encoding_gzip: bool,
    /// <p>An object that determines whether any HTTP headers (and if so, which headers) are included in the cache key and automatically included in requests that CloudFront sends to the origin.</p>
    pub headers_config: CachePolicyHeadersConfig,
    /// <p>An object that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the cache key and automatically included in requests that CloudFront sends to the origin.</p>
    pub query_strings_config: CachePolicyQueryStringsConfig,
}

#[allow(dead_code)]
struct ParametersInCacheKeyAndForwardedToOriginDeserializer;
impl ParametersInCacheKeyAndForwardedToOriginDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ParametersInCacheKeyAndForwardedToOrigin, XmlParseError> {
        deserialize_elements::<_, ParametersInCacheKeyAndForwardedToOrigin, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CookiesConfig" => {
                        obj.cookies_config = CachePolicyCookiesConfigDeserializer::deserialize(
                            "CookiesConfig",
                            stack,
                        )?;
                    }
                    "EnableAcceptEncodingBrotli" => {
                        obj.enable_accept_encoding_brotli = Some(BooleanDeserializer::deserialize(
                            "EnableAcceptEncodingBrotli",
                            stack,
                        )?);
                    }
                    "EnableAcceptEncodingGzip" => {
                        obj.enable_accept_encoding_gzip =
                            BooleanDeserializer::deserialize("EnableAcceptEncodingGzip", stack)?;
                    }
                    "HeadersConfig" => {
                        obj.headers_config = CachePolicyHeadersConfigDeserializer::deserialize(
                            "HeadersConfig",
                            stack,
                        )?;
                    }
                    "QueryStringsConfig" => {
                        obj.query_strings_config =
                            CachePolicyQueryStringsConfigDeserializer::deserialize(
                                "QueryStringsConfig",
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

pub struct ParametersInCacheKeyAndForwardedToOriginSerializer;
impl ParametersInCacheKeyAndForwardedToOriginSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ParametersInCacheKeyAndForwardedToOrigin,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        CachePolicyCookiesConfigSerializer::serialize(
            &mut writer,
            "CookiesConfig",
            &obj.cookies_config,
        )?;
        if let Some(ref value) = obj.enable_accept_encoding_brotli {
            write_characters_element(writer, "EnableAcceptEncodingBrotli", &value.to_string())?;
        }
        write_characters_element(
            writer,
            "EnableAcceptEncodingGzip",
            &obj.enable_accept_encoding_gzip.to_string(),
        )?;
        CachePolicyHeadersConfigSerializer::serialize(
            &mut writer,
            "HeadersConfig",
            &obj.headers_config,
        )?;
        CachePolicyQueryStringsConfigSerializer::serialize(
            &mut writer,
            "QueryStringsConfig",
            &obj.query_strings_config,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct PathListDeserializer;
impl PathListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Path" {
                obj.push(StringDeserializer::deserialize("Path", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct PathListSerializer;
impl PathListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            StringSerializer::serialize(writer, "Path", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A complex type that contains information about the objects that you want to invalidate. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Invalidation.html#invalidation-specifying-objects">Specifying the Objects to Invalidate</a> in the <i>Amazon CloudFront Developer Guide</i>. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Paths {
    /// <p>A complex type that contains a list of the paths that you want to invalidate.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of invalidation paths specified for the objects that you want to invalidate.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct PathsDeserializer;
impl PathsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Paths, XmlParseError> {
        deserialize_elements::<_, Paths, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .get_or_insert(vec![])
                        .extend(PathListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct PathsSerializer;
impl PathsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Paths,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &PathListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct PriceClassDeserializer;
impl PriceClassDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct PriceClassSerializer;
impl PriceClassSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>A public key that you can use with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">signed URLs and signed cookies</a>, or with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/field-level-encryption.html">field-level encryption</a>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PublicKey {
    /// <p>The date and time when the public key was uploaded.</p>
    pub created_time: String,
    /// <p>The identifier of the public key.</p>
    pub id: String,
    /// <p>Configuration information about a public key that you can use with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">signed URLs and signed cookies</a>, or with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/field-level-encryption.html">field-level encryption</a>.</p>
    pub public_key_config: PublicKeyConfig,
}

#[allow(dead_code)]
struct PublicKeyDeserializer;
impl PublicKeyDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PublicKey, XmlParseError> {
        deserialize_elements::<_, PublicKey, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CreatedTime" => {
                    obj.created_time = TimestampDeserializer::deserialize("CreatedTime", stack)?;
                }
                "Id" => {
                    obj.id = StringDeserializer::deserialize("Id", stack)?;
                }
                "PublicKeyConfig" => {
                    obj.public_key_config =
                        PublicKeyConfigDeserializer::deserialize("PublicKeyConfig", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Configuration information about a public key that you can use with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">signed URLs and signed cookies</a>, or with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/field-level-encryption.html">field-level encryption</a>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PublicKeyConfig {
    /// <p>A string included in the request to help make sure that the request can’t be replayed.</p>
    pub caller_reference: String,
    /// <p>A comment to describe the public key.</p>
    pub comment: Option<String>,
    /// <p>The public key that you can use with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">signed URLs and signed cookies</a>, or with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/field-level-encryption.html">field-level encryption</a>.</p>
    pub encoded_key: String,
    /// <p>A name to help identify the public key.</p>
    pub name: String,
}

#[allow(dead_code)]
struct PublicKeyConfigDeserializer;
impl PublicKeyConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PublicKeyConfig, XmlParseError> {
        deserialize_elements::<_, PublicKeyConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CallerReference" => {
                    obj.caller_reference =
                        StringDeserializer::deserialize("CallerReference", stack)?;
                }
                "Comment" => {
                    obj.comment = Some(StringDeserializer::deserialize("Comment", stack)?);
                }
                "EncodedKey" => {
                    obj.encoded_key = StringDeserializer::deserialize("EncodedKey", stack)?;
                }
                "Name" => {
                    obj.name = StringDeserializer::deserialize("Name", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct PublicKeyConfigSerializer;
impl PublicKeyConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &PublicKeyConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "CallerReference", &obj.caller_reference.to_string())?;
        if let Some(ref value) = obj.comment {
            write_characters_element(writer, "Comment", &value.to_string())?;
        }
        write_characters_element(writer, "EncodedKey", &obj.encoded_key.to_string())?;
        write_characters_element(writer, "Name", &obj.name.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct PublicKeyIdListDeserializer;
impl PublicKeyIdListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "PublicKey" {
                obj.push(StringDeserializer::deserialize("PublicKey", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct PublicKeyIdListSerializer;
impl PublicKeyIdListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            StringSerializer::serialize(writer, "PublicKey", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A list of public keys that you can use with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">signed URLs and signed cookies</a>, or with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/field-level-encryption.html">field-level encryption</a>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PublicKeyList {
    /// <p>A list of public keys.</p>
    pub items: Option<Vec<PublicKeySummary>>,
    /// <p>The maximum number of public keys you want in the response.</p>
    pub max_items: i64,
    /// <p>If there are more elements to be listed, this element is present and contains the value that you can use for the <code>Marker</code> request parameter to continue listing your public keys where you left off.</p>
    pub next_marker: Option<String>,
    /// <p>The number of public keys in the list.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct PublicKeyListDeserializer;
impl PublicKeyListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PublicKeyList, XmlParseError> {
        deserialize_elements::<_, PublicKeyList, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        PublicKeySummaryListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "MaxItems" => {
                    obj.max_items = IntegerDeserializer::deserialize("MaxItems", stack)?;
                }
                "NextMarker" => {
                    obj.next_marker = Some(StringDeserializer::deserialize("NextMarker", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Contains information about a public key.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PublicKeySummary {
    /// <p>A comment to describe the public key.</p>
    pub comment: Option<String>,
    /// <p>The date and time when the public key was uploaded.</p>
    pub created_time: String,
    /// <p>The public key.</p>
    pub encoded_key: String,
    /// <p>The identifier of the public key.</p>
    pub id: String,
    /// <p>A name to help identify the public key.</p>
    pub name: String,
}

#[allow(dead_code)]
struct PublicKeySummaryDeserializer;
impl PublicKeySummaryDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PublicKeySummary, XmlParseError> {
        deserialize_elements::<_, PublicKeySummary, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Comment" => {
                    obj.comment = Some(StringDeserializer::deserialize("Comment", stack)?);
                }
                "CreatedTime" => {
                    obj.created_time = TimestampDeserializer::deserialize("CreatedTime", stack)?;
                }
                "EncodedKey" => {
                    obj.encoded_key = StringDeserializer::deserialize("EncodedKey", stack)?;
                }
                "Id" => {
                    obj.id = StringDeserializer::deserialize("Id", stack)?;
                }
                "Name" => {
                    obj.name = StringDeserializer::deserialize("Name", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct PublicKeySummaryListDeserializer;
impl PublicKeySummaryListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PublicKeySummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "PublicKeySummary" {
                obj.push(PublicKeySummaryDeserializer::deserialize(
                    "PublicKeySummary",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Query argument-profile mapping for field-level encryption.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct QueryArgProfile {
    /// <p>ID of profile to use for field-level encryption query argument-profile mapping</p>
    pub profile_id: String,
    /// <p>Query argument for field-level encryption query argument-profile mapping.</p>
    pub query_arg: String,
}

#[allow(dead_code)]
struct QueryArgProfileDeserializer;
impl QueryArgProfileDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueryArgProfile, XmlParseError> {
        deserialize_elements::<_, QueryArgProfile, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ProfileId" => {
                    obj.profile_id = StringDeserializer::deserialize("ProfileId", stack)?;
                }
                "QueryArg" => {
                    obj.query_arg = StringDeserializer::deserialize("QueryArg", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct QueryArgProfileSerializer;
impl QueryArgProfileSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &QueryArgProfile,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "ProfileId", &obj.profile_id.to_string())?;
        write_characters_element(writer, "QueryArg", &obj.query_arg.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Configuration for query argument-profile mapping for field-level encryption.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct QueryArgProfileConfig {
    /// <p>Flag to set if you want a request to be forwarded to the origin even if the profile specified by the field-level encryption query argument, fle-profile, is unknown.</p>
    pub forward_when_query_arg_profile_is_unknown: bool,
    /// <p>Profiles specified for query argument-profile mapping for field-level encryption.</p>
    pub query_arg_profiles: Option<QueryArgProfiles>,
}

#[allow(dead_code)]
struct QueryArgProfileConfigDeserializer;
impl QueryArgProfileConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueryArgProfileConfig, XmlParseError> {
        deserialize_elements::<_, QueryArgProfileConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ForwardWhenQueryArgProfileIsUnknown" => {
                    obj.forward_when_query_arg_profile_is_unknown =
                        BooleanDeserializer::deserialize(
                            "ForwardWhenQueryArgProfileIsUnknown",
                            stack,
                        )?;
                }
                "QueryArgProfiles" => {
                    obj.query_arg_profiles = Some(QueryArgProfilesDeserializer::deserialize(
                        "QueryArgProfiles",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct QueryArgProfileConfigSerializer;
impl QueryArgProfileConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &QueryArgProfileConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(
            writer,
            "ForwardWhenQueryArgProfileIsUnknown",
            &obj.forward_when_query_arg_profile_is_unknown.to_string(),
        )?;
        if let Some(ref value) = obj.query_arg_profiles {
            &QueryArgProfilesSerializer::serialize(&mut writer, "QueryArgProfiles", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct QueryArgProfileListDeserializer;
impl QueryArgProfileListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<QueryArgProfile>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "QueryArgProfile" {
                obj.push(QueryArgProfileDeserializer::deserialize(
                    "QueryArgProfile",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct QueryArgProfileListSerializer;
impl QueryArgProfileListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<QueryArgProfile>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            QueryArgProfileSerializer::serialize(writer, "QueryArgProfile", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>Query argument-profile mapping for field-level encryption.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct QueryArgProfiles {
    /// <p>Number of items for query argument-profile mapping for field-level encryption.</p>
    pub items: Option<Vec<QueryArgProfile>>,
    /// <p>Number of profiles for query argument-profile mapping for field-level encryption.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct QueryArgProfilesDeserializer;
impl QueryArgProfilesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueryArgProfiles, XmlParseError> {
        deserialize_elements::<_, QueryArgProfiles, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        QueryArgProfileListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct QueryArgProfilesSerializer;
impl QueryArgProfilesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &QueryArgProfiles,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &QueryArgProfileListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>This field is deprecated. We recommend that you use a cache policy or an origin request policy instead of this field.</p> <p>If you want to include query strings in the cache key, use <code>QueryStringsConfig</code> in a cache policy. See <code>CachePolicy</code>.</p> <p>If you want to send query strings to the origin but not include them in the cache key, use <code>QueryStringsConfig</code> in an origin request policy. See <code>OriginRequestPolicy</code>.</p> <p>A complex type that contains information about the query string parameters that you want CloudFront to use for caching for a cache behavior. </p>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct QueryStringCacheKeys {
    /// <p>A list that contains the query string parameters that you want CloudFront to use as a basis for caching for a cache behavior. If <code>Quantity</code> is 0, you can omit <code>Items</code>. </p>
    pub items: Option<Vec<String>>,
    /// <p>The number of <code>whitelisted</code> query string parameters for a cache behavior.</p>
    pub quantity: i64,
}

/// <p>Contains a list of query string names.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct QueryStringNames {
    /// <p>A list of query string names.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of query string names in the <code>Items</code> list.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct QueryStringNamesDeserializer;
impl QueryStringNamesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueryStringNames, XmlParseError> {
        deserialize_elements::<_, QueryStringNames, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        QueryStringNamesListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct QueryStringNamesSerializer;
impl QueryStringNamesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &QueryStringNames,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &QueryStringNamesListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct QueryStringNamesListDeserializer;
impl QueryStringNamesListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Name" {
                obj.push(StringDeserializer::deserialize("Name", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct QueryStringNamesListSerializer;
impl QueryStringNamesListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            StringSerializer::serialize(writer, "Name", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A real-time log configuration.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RealtimeLogConfig {
    /// <p>The Amazon Resource Name (ARN) of this real-time log configuration.</p>
    pub arn: String,
    /// <p>Contains information about the Amazon Kinesis data stream where you are sending real-time log data for this real-time log configuration.</p>
    pub end_points: Vec<EndPoint>,
    /// <p>A list of fields that are included in each real-time log record. In an API response, the fields are provided in the same order in which they are sent to the Amazon Kinesis data stream.</p> <p>For more information about fields, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields">Real-time log configuration fields</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub fields: Vec<String>,
    /// <p>The unique name of this real-time log configuration.</p>
    pub name: String,
    /// <p>The sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. The sampling rate is an integer between 1 and 100, inclusive.</p>
    pub sampling_rate: i64,
}

#[allow(dead_code)]
struct RealtimeLogConfigDeserializer;
impl RealtimeLogConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RealtimeLogConfig, XmlParseError> {
        deserialize_elements::<_, RealtimeLogConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = StringDeserializer::deserialize("ARN", stack)?;
                }
                "EndPoints" => {
                    obj.end_points
                        .extend(EndPointListDeserializer::deserialize("EndPoints", stack)?);
                }
                "Fields" => {
                    obj.fields
                        .extend(FieldListDeserializer::deserialize("Fields", stack)?);
                }
                "Name" => {
                    obj.name = StringDeserializer::deserialize("Name", stack)?;
                }
                "SamplingRate" => {
                    obj.sampling_rate = LongDeserializer::deserialize("SamplingRate", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct RealtimeLogConfigListDeserializer;
impl RealtimeLogConfigListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<RealtimeLogConfig>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(RealtimeLogConfigDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A list of real-time log configurations.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RealtimeLogConfigs {
    /// <p>A flag that indicates whether there are more real-time log configurations than are contained in this list.</p>
    pub is_truncated: bool,
    /// <p>Contains the list of real-time log configurations.</p>
    pub items: Option<Vec<RealtimeLogConfig>>,
    /// <p>This parameter indicates where this list of real-time log configurations begins. This list includes real-time log configurations that occur after the marker.</p>
    pub marker: String,
    /// <p>The maximum number of real-time log configurations requested.</p>
    pub max_items: i64,
    /// <p>If there are more items in the list than are in this response, this element is present. It contains the value that you should use in the <code>Marker</code> field of a subsequent request to continue listing real-time log configurations where you left off. </p>
    pub next_marker: Option<String>,
}

#[allow(dead_code)]
struct RealtimeLogConfigsDeserializer;
impl RealtimeLogConfigsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RealtimeLogConfigs, XmlParseError> {
        deserialize_elements::<_, RealtimeLogConfigs, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "IsTruncated" => {
                    obj.is_truncated = BooleanDeserializer::deserialize("IsTruncated", stack)?;
                }
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        RealtimeLogConfigListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "Marker" => {
                    obj.marker = StringDeserializer::deserialize("Marker", stack)?;
                }
                "MaxItems" => {
                    obj.max_items = IntegerDeserializer::deserialize("MaxItems", stack)?;
                }
                "NextMarker" => {
                    obj.next_marker = Some(StringDeserializer::deserialize("NextMarker", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A subscription configuration for additional CloudWatch metrics.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RealtimeMetricsSubscriptionConfig {
    /// <p>A flag that indicates whether additional CloudWatch metrics are enabled for a given CloudFront distribution.</p>
    pub realtime_metrics_subscription_status: String,
}

#[allow(dead_code)]
struct RealtimeMetricsSubscriptionConfigDeserializer;
impl RealtimeMetricsSubscriptionConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RealtimeMetricsSubscriptionConfig, XmlParseError> {
        deserialize_elements::<_, RealtimeMetricsSubscriptionConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "RealtimeMetricsSubscriptionStatus" => {
                        obj.realtime_metrics_subscription_status =
                            RealtimeMetricsSubscriptionStatusDeserializer::deserialize(
                                "RealtimeMetricsSubscriptionStatus",
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

pub struct RealtimeMetricsSubscriptionConfigSerializer;
impl RealtimeMetricsSubscriptionConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &RealtimeMetricsSubscriptionConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(
            writer,
            "RealtimeMetricsSubscriptionStatus",
            &obj.realtime_metrics_subscription_status.to_string(),
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct RealtimeMetricsSubscriptionStatusDeserializer;
impl RealtimeMetricsSubscriptionStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct RealtimeMetricsSubscriptionStatusSerializer;
impl RealtimeMetricsSubscriptionStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

pub struct ResourceARNSerializer;
impl ResourceARNSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>A complex type that identifies ways in which you want to restrict distribution of your content.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Restrictions {
    /// <p>A complex type that controls the countries in which your content is distributed. CloudFront determines the location of your users using <code>MaxMind</code> GeoIP databases.</p>
    pub geo_restriction: GeoRestriction,
}

#[allow(dead_code)]
struct RestrictionsDeserializer;
impl RestrictionsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Restrictions, XmlParseError> {
        deserialize_elements::<_, Restrictions, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "GeoRestriction" => {
                    obj.geo_restriction =
                        GeoRestrictionDeserializer::deserialize("GeoRestriction", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct RestrictionsSerializer;
impl RestrictionsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Restrictions,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        GeoRestrictionSerializer::serialize(&mut writer, "GeoRestriction", &obj.geo_restriction)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that contains information about the Amazon S3 bucket from which you want CloudFront to get your media files for distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3Origin {
    /// <p>The DNS name of the Amazon S3 origin. </p>
    pub domain_name: String,
    /// <p>The CloudFront origin access identity to associate with the distribution. Use an origin access identity to configure the distribution so that end users can only access objects in an Amazon S3 bucket through CloudFront.</p> <p>If you want end users to be able to access objects using either the CloudFront URL or the Amazon S3 URL, specify an empty <code>OriginAccessIdentity</code> element.</p> <p>To delete the origin access identity from an existing distribution, update the distribution configuration and include an empty <code>OriginAccessIdentity</code> element.</p> <p>To replace the origin access identity, update the distribution configuration and specify the new origin access identity.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-restricting-access-to-s3.html">Using an Origin Access Identity to Restrict Access to Your Amazon S3 Content</a> in the <i> Amazon CloudFront Developer Guide</i>.</p>
    pub origin_access_identity: String,
}

#[allow(dead_code)]
struct S3OriginDeserializer;
impl S3OriginDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<S3Origin, XmlParseError> {
        deserialize_elements::<_, S3Origin, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DomainName" => {
                    obj.domain_name = StringDeserializer::deserialize("DomainName", stack)?;
                }
                "OriginAccessIdentity" => {
                    obj.origin_access_identity =
                        StringDeserializer::deserialize("OriginAccessIdentity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct S3OriginSerializer;
impl S3OriginSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &S3Origin,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "DomainName", &obj.domain_name.to_string())?;
        write_characters_element(
            writer,
            "OriginAccessIdentity",
            &obj.origin_access_identity.to_string(),
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that contains information about the Amazon S3 origin. If the origin is a custom origin or an S3 bucket that is configured as a website endpoint, use the <code>CustomOriginConfig</code> element instead.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3OriginConfig {
    /// <p>The CloudFront origin access identity to associate with the origin. Use an origin access identity to configure the origin so that viewers can <i>only</i> access objects in an Amazon S3 bucket through CloudFront. The format of the value is:</p> <p>origin-access-identity/cloudfront/<i>ID-of-origin-access-identity</i> </p> <p>where <code> <i>ID-of-origin-access-identity</i> </code> is the value that CloudFront returned in the <code>ID</code> element when you created the origin access identity.</p> <p>If you want viewers to be able to access objects using either the CloudFront URL or the Amazon S3 URL, specify an empty <code>OriginAccessIdentity</code> element.</p> <p>To delete the origin access identity from an existing distribution, update the distribution configuration and include an empty <code>OriginAccessIdentity</code> element.</p> <p>To replace the origin access identity, update the distribution configuration and specify the new origin access identity.</p> <p>For more information about the origin access identity, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub origin_access_identity: String,
}

#[allow(dead_code)]
struct S3OriginConfigDeserializer;
impl S3OriginConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<S3OriginConfig, XmlParseError> {
        deserialize_elements::<_, S3OriginConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "OriginAccessIdentity" => {
                    obj.origin_access_identity =
                        StringDeserializer::deserialize("OriginAccessIdentity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct S3OriginConfigSerializer;
impl S3OriginConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &S3OriginConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(
            writer,
            "OriginAccessIdentity",
            &obj.origin_access_identity.to_string(),
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct SSLSupportMethodDeserializer;
impl SSLSupportMethodDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct SSLSupportMethodSerializer;
impl SSLSupportMethodSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p>A list of AWS accounts and the active CloudFront key pairs in each account that CloudFront can use to verify the signatures of signed URLs and signed cookies.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Signer {
    /// <p>An AWS account number that contains active CloudFront key pairs that CloudFront can use to verify the signatures of signed URLs and signed cookies. If the AWS account that owns the key pairs is the same account that owns the CloudFront distribution, the value of this field is <code>self</code>.</p>
    pub aws_account_number: Option<String>,
    /// <p>A list of CloudFront key pair identifiers.</p>
    pub key_pair_ids: Option<KeyPairIds>,
}

#[allow(dead_code)]
struct SignerDeserializer;
impl SignerDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Signer, XmlParseError> {
        deserialize_elements::<_, Signer, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AwsAccountNumber" => {
                    obj.aws_account_number =
                        Some(StringDeserializer::deserialize("AwsAccountNumber", stack)?);
                }
                "KeyPairIds" => {
                    obj.key_pair_ids =
                        Some(KeyPairIdsDeserializer::deserialize("KeyPairIds", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct SignerListDeserializer;
impl SignerListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Signer>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Signer" {
                obj.push(SignerDeserializer::deserialize("Signer", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct SslProtocolDeserializer;
impl SslProtocolDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct SslProtocolSerializer;
impl SslProtocolSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

#[allow(dead_code)]
struct SslProtocolsListDeserializer;
impl SslProtocolsListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "SslProtocol" {
                obj.push(SslProtocolDeserializer::deserialize("SslProtocol", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct SslProtocolsListSerializer;
impl SslProtocolsListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            SslProtocolSerializer::serialize(writer, "SslProtocol", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[allow(dead_code)]
struct StatusCodeListDeserializer;
impl StatusCodeListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<i64>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "StatusCode" {
                obj.push(IntegerDeserializer::deserialize("StatusCode", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct StatusCodeListSerializer;
impl StatusCodeListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<i64>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            IntegerSerializer::serialize(writer, "StatusCode", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A complex data type for the status codes that you specify that, when returned by a primary origin, trigger CloudFront to failover to a second origin.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StatusCodes {
    /// <p>The items (status codes) for an origin group.</p>
    pub items: Vec<i64>,
    /// <p>The number of status codes.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct StatusCodesDeserializer;
impl StatusCodesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StatusCodes, XmlParseError> {
        deserialize_elements::<_, StatusCodes, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .extend(StatusCodeListDeserializer::deserialize("Items", stack)?);
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct StatusCodesSerializer;
impl StatusCodesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &StatusCodes,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        StatusCodeListSerializer::serialize(&mut writer, "Items", &obj.items)?;
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A streaming distribution tells CloudFront where you want RTMP content to be delivered from, and the details about how to track and manage content delivery.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct StreamingDistribution {
    /// <p>The ARN (Amazon Resource Name) for the distribution. For example: <code>arn:aws:cloudfront::123456789012:distribution/EDFDVBD632BHDS5</code>, where <code>123456789012</code> is your AWS account ID.</p>
    pub arn: String,
    /// <p>A complex type that lists the AWS accounts, if any, that you included in the <code>TrustedSigners</code> complex type for this distribution. These are the accounts that you want to allow to create signed URLs for private content.</p> <p>The <code>Signer</code> complex type lists the AWS account number of the trusted signer or <code>self</code> if the signer is the AWS account that created the distribution. The <code>Signer</code> element also includes the IDs of any active CloudFront key pairs that are associated with the trusted signer's AWS account. If no <code>KeyPairId</code> element appears for a <code>Signer</code>, that signer can't create signed URLs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>. </p>
    pub active_trusted_signers: ActiveTrustedSigners,
    /// <p>The domain name that corresponds to the streaming distribution, for example, <code>s5c39gqb8ow64r.cloudfront.net</code>. </p>
    pub domain_name: String,
    /// <p>The identifier for the RTMP distribution. For example: <code>EGTXBD79EXAMPLE</code>.</p>
    pub id: String,
    /// <p>The date and time that the distribution was last modified. </p>
    pub last_modified_time: Option<String>,
    /// <p>The current status of the RTMP distribution. When the status is <code>Deployed</code>, the distribution's information is propagated to all CloudFront edge locations.</p>
    pub status: String,
    /// <p>The current configuration information for the RTMP distribution.</p>
    pub streaming_distribution_config: StreamingDistributionConfig,
}

#[allow(dead_code)]
struct StreamingDistributionDeserializer;
impl StreamingDistributionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StreamingDistribution, XmlParseError> {
        deserialize_elements::<_, StreamingDistribution, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = StringDeserializer::deserialize("ARN", stack)?;
                }
                "ActiveTrustedSigners" => {
                    obj.active_trusted_signers = ActiveTrustedSignersDeserializer::deserialize(
                        "ActiveTrustedSigners",
                        stack,
                    )?;
                }
                "DomainName" => {
                    obj.domain_name = StringDeserializer::deserialize("DomainName", stack)?;
                }
                "Id" => {
                    obj.id = StringDeserializer::deserialize("Id", stack)?;
                }
                "LastModifiedTime" => {
                    obj.last_modified_time = Some(TimestampDeserializer::deserialize(
                        "LastModifiedTime",
                        stack,
                    )?);
                }
                "Status" => {
                    obj.status = StringDeserializer::deserialize("Status", stack)?;
                }
                "StreamingDistributionConfig" => {
                    obj.streaming_distribution_config =
                        StreamingDistributionConfigDeserializer::deserialize(
                            "StreamingDistributionConfig",
                            stack,
                        )?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>The RTMP distribution's configuration information.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StreamingDistributionConfig {
    /// <p>A complex type that contains information about CNAMEs (alternate domain names), if any, for this streaming distribution. </p>
    pub aliases: Option<Aliases>,
    /// <p>A unique value (for example, a date-time stamp) that ensures that the request can't be replayed.</p> <p>If the value of <code>CallerReference</code> is new (regardless of the content of the <code>StreamingDistributionConfig</code> object), CloudFront creates a new distribution.</p> <p>If <code>CallerReference</code> is a value that you already sent in a previous request to create a distribution, CloudFront returns a <code>DistributionAlreadyExists</code> error.</p>
    pub caller_reference: String,
    /// <p>Any comments you want to include about the streaming distribution. </p>
    pub comment: String,
    /// <p>Whether the streaming distribution is enabled to accept user requests for content.</p>
    pub enabled: bool,
    /// <p>A complex type that controls whether access logs are written for the streaming distribution. </p>
    pub logging: Option<StreamingLoggingConfig>,
    /// <p>A complex type that contains information about price class for this streaming distribution. </p>
    pub price_class: Option<String>,
    /// <p>A complex type that contains information about the Amazon S3 bucket from which you want CloudFront to get your media files for distribution. </p>
    pub s3_origin: S3Origin,
    /// <p>A complex type that specifies any AWS accounts that you want to permit to create signed URLs for private content. If you want the distribution to use signed URLs, include this element; if you want the distribution to use public URLs, remove this element. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>. </p>
    pub trusted_signers: TrustedSigners,
}

#[allow(dead_code)]
struct StreamingDistributionConfigDeserializer;
impl StreamingDistributionConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StreamingDistributionConfig, XmlParseError> {
        deserialize_elements::<_, StreamingDistributionConfig, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Aliases" => {
                        obj.aliases = Some(AliasesDeserializer::deserialize("Aliases", stack)?);
                    }
                    "CallerReference" => {
                        obj.caller_reference =
                            StringDeserializer::deserialize("CallerReference", stack)?;
                    }
                    "Comment" => {
                        obj.comment = StringDeserializer::deserialize("Comment", stack)?;
                    }
                    "Enabled" => {
                        obj.enabled = BooleanDeserializer::deserialize("Enabled", stack)?;
                    }
                    "Logging" => {
                        obj.logging = Some(StreamingLoggingConfigDeserializer::deserialize(
                            "Logging", stack,
                        )?);
                    }
                    "PriceClass" => {
                        obj.price_class =
                            Some(PriceClassDeserializer::deserialize("PriceClass", stack)?);
                    }
                    "S3Origin" => {
                        obj.s3_origin = S3OriginDeserializer::deserialize("S3Origin", stack)?;
                    }
                    "TrustedSigners" => {
                        obj.trusted_signers =
                            TrustedSignersDeserializer::deserialize("TrustedSigners", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct StreamingDistributionConfigSerializer;
impl StreamingDistributionConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &StreamingDistributionConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.aliases {
            &AliasesSerializer::serialize(&mut writer, "Aliases", value)?;
        }
        write_characters_element(writer, "CallerReference", &obj.caller_reference.to_string())?;
        write_characters_element(writer, "Comment", &obj.comment.to_string())?;
        write_characters_element(writer, "Enabled", &obj.enabled.to_string())?;
        if let Some(ref value) = obj.logging {
            &StreamingLoggingConfigSerializer::serialize(&mut writer, "Logging", value)?;
        }
        if let Some(ref value) = obj.price_class {
            write_characters_element(writer, "PriceClass", &value.to_string())?;
        }
        S3OriginSerializer::serialize(&mut writer, "S3Origin", &obj.s3_origin)?;
        TrustedSignersSerializer::serialize(&mut writer, "TrustedSigners", &obj.trusted_signers)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A streaming distribution Configuration and a list of tags to be associated with the streaming distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StreamingDistributionConfigWithTags {
    /// <p>A streaming distribution Configuration.</p>
    pub streaming_distribution_config: StreamingDistributionConfig,
    /// <p>A complex type that contains zero or more <code>Tag</code> elements.</p>
    pub tags: Tags,
}

pub struct StreamingDistributionConfigWithTagsSerializer;
impl StreamingDistributionConfigWithTagsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &StreamingDistributionConfigWithTags,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        StreamingDistributionConfigSerializer::serialize(
            &mut writer,
            "StreamingDistributionConfig",
            &obj.streaming_distribution_config,
        )?;
        TagsSerializer::serialize(&mut writer, "Tags", &obj.tags)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A streaming distribution list. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct StreamingDistributionList {
    /// <p>A flag that indicates whether more streaming distributions remain to be listed. If your results were truncated, you can make a follow-up pagination request using the <code>Marker</code> request parameter to retrieve more distributions in the list. </p>
    pub is_truncated: bool,
    /// <p>A complex type that contains one <code>StreamingDistributionSummary</code> element for each distribution that was created by the current AWS account.</p>
    pub items: Option<Vec<StreamingDistributionSummary>>,
    /// <p>The value you provided for the <code>Marker</code> request parameter. </p>
    pub marker: String,
    /// <p>The value you provided for the <code>MaxItems</code> request parameter. </p>
    pub max_items: i64,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value you can use for the <code>Marker</code> request parameter to continue listing your RTMP distributions where they left off. </p>
    pub next_marker: Option<String>,
    /// <p>The number of streaming distributions that were created by the current AWS account. </p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct StreamingDistributionListDeserializer;
impl StreamingDistributionListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StreamingDistributionList, XmlParseError> {
        deserialize_elements::<_, StreamingDistributionList, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IsTruncated" => {
                        obj.is_truncated = BooleanDeserializer::deserialize("IsTruncated", stack)?;
                    }
                    "Items" => {
                        obj.items.get_or_insert(vec![]).extend(
                            StreamingDistributionSummaryListDeserializer::deserialize(
                                "Items", stack,
                            )?,
                        );
                    }
                    "Marker" => {
                        obj.marker = StringDeserializer::deserialize("Marker", stack)?;
                    }
                    "MaxItems" => {
                        obj.max_items = IntegerDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(StringDeserializer::deserialize("NextMarker", stack)?);
                    }
                    "Quantity" => {
                        obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p> A summary of the information for a CloudFront streaming distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct StreamingDistributionSummary {
    /// <p> The ARN (Amazon Resource Name) for the streaming distribution. For example: <code>arn:aws:cloudfront::123456789012:streaming-distribution/EDFDVBD632BHDS5</code>, where <code>123456789012</code> is your AWS account ID.</p>
    pub arn: String,
    /// <p>A complex type that contains information about CNAMEs (alternate domain names), if any, for this streaming distribution.</p>
    pub aliases: Aliases,
    /// <p>The comment originally specified when this distribution was created.</p>
    pub comment: String,
    /// <p>The domain name corresponding to the distribution, for example, <code>d111111abcdef8.cloudfront.net</code>.</p>
    pub domain_name: String,
    /// <p>Whether the distribution is enabled to accept end user requests for content.</p>
    pub enabled: bool,
    /// <p>The identifier for the distribution, for example, <code>EDFDVBD632BHDS5</code>.</p>
    pub id: String,
    /// <p>The date and time the distribution was last modified.</p>
    pub last_modified_time: String,
    /// <p>A complex type that contains information about price class for this streaming distribution. </p>
    pub price_class: String,
    /// <p>A complex type that contains information about the Amazon S3 bucket from which you want CloudFront to get your media files for distribution.</p>
    pub s3_origin: S3Origin,
    /// <p> Indicates the current status of the distribution. When the status is <code>Deployed</code>, the distribution's information is fully propagated throughout the Amazon CloudFront system.</p>
    pub status: String,
    /// <p>A complex type that specifies the AWS accounts, if any, that you want to allow to create signed URLs for private content. If you want to require signed URLs in requests for objects in the target origin that match the <code>PathPattern</code> for this cache behavior, specify <code>true</code> for <code>Enabled</code>, and specify the applicable values for <code>Quantity</code> and <code>Items</code>.If you don't want to require signed URLs in requests for objects that match <code>PathPattern</code>, specify <code>false</code> for <code>Enabled</code> and <code>0</code> for <code>Quantity</code>. Omit <code>Items</code>. To add, change, or remove one or more trusted signers, change <code>Enabled</code> to <code>true</code> (if it's currently <code>false</code>), change <code>Quantity</code> as applicable, and specify all of the trusted signers that you want to include in the updated distribution.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>. </p>
    pub trusted_signers: TrustedSigners,
}

#[allow(dead_code)]
struct StreamingDistributionSummaryDeserializer;
impl StreamingDistributionSummaryDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StreamingDistributionSummary, XmlParseError> {
        deserialize_elements::<_, StreamingDistributionSummary, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ARN" => {
                        obj.arn = StringDeserializer::deserialize("ARN", stack)?;
                    }
                    "Aliases" => {
                        obj.aliases = AliasesDeserializer::deserialize("Aliases", stack)?;
                    }
                    "Comment" => {
                        obj.comment = StringDeserializer::deserialize("Comment", stack)?;
                    }
                    "DomainName" => {
                        obj.domain_name = StringDeserializer::deserialize("DomainName", stack)?;
                    }
                    "Enabled" => {
                        obj.enabled = BooleanDeserializer::deserialize("Enabled", stack)?;
                    }
                    "Id" => {
                        obj.id = StringDeserializer::deserialize("Id", stack)?;
                    }
                    "LastModifiedTime" => {
                        obj.last_modified_time =
                            TimestampDeserializer::deserialize("LastModifiedTime", stack)?;
                    }
                    "PriceClass" => {
                        obj.price_class = PriceClassDeserializer::deserialize("PriceClass", stack)?;
                    }
                    "S3Origin" => {
                        obj.s3_origin = S3OriginDeserializer::deserialize("S3Origin", stack)?;
                    }
                    "Status" => {
                        obj.status = StringDeserializer::deserialize("Status", stack)?;
                    }
                    "TrustedSigners" => {
                        obj.trusted_signers =
                            TrustedSignersDeserializer::deserialize("TrustedSigners", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct StreamingDistributionSummaryListDeserializer;
impl StreamingDistributionSummaryListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StreamingDistributionSummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "StreamingDistributionSummary" {
                obj.push(StreamingDistributionSummaryDeserializer::deserialize(
                    "StreamingDistributionSummary",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A complex type that controls whether access logs are written for this streaming distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StreamingLoggingConfig {
    /// <p>The Amazon S3 bucket to store the access logs in, for example, <code>myawslogbucket.s3.amazonaws.com</code>.</p>
    pub bucket: String,
    /// <p>Specifies whether you want CloudFront to save access logs to an Amazon S3 bucket. If you don't want to enable logging when you create a streaming distribution or if you want to disable logging for an existing streaming distribution, specify <code>false</code> for <code>Enabled</code>, and specify <code>empty Bucket</code> and <code>Prefix</code> elements. If you specify <code>false</code> for <code>Enabled</code> but you specify values for <code>Bucket</code> and <code>Prefix</code>, the values are automatically deleted. </p>
    pub enabled: bool,
    /// <p>An optional string that you want CloudFront to prefix to the access log filenames for this streaming distribution, for example, <code>myprefix/</code>. If you want to enable logging, but you don't want to specify a prefix, you still must include an empty <code>Prefix</code> element in the <code>Logging</code> element.</p>
    pub prefix: String,
}

#[allow(dead_code)]
struct StreamingLoggingConfigDeserializer;
impl StreamingLoggingConfigDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StreamingLoggingConfig, XmlParseError> {
        deserialize_elements::<_, StreamingLoggingConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Bucket" => {
                    obj.bucket = StringDeserializer::deserialize("Bucket", stack)?;
                }
                "Enabled" => {
                    obj.enabled = BooleanDeserializer::deserialize("Enabled", stack)?;
                }
                "Prefix" => {
                    obj.prefix = StringDeserializer::deserialize("Prefix", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct StreamingLoggingConfigSerializer;
impl StreamingLoggingConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &StreamingLoggingConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "Bucket", &obj.bucket.to_string())?;
        write_characters_element(writer, "Enabled", &obj.enabled.to_string())?;
        write_characters_element(writer, "Prefix", &obj.prefix.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct StringDeserializer;
impl StringDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct StringSerializer;
impl StringSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p> A complex type that contains <code>Tag</code> key and <code>Tag</code> value.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tag {
    /// <p> A string that contains <code>Tag</code> key.</p> <p>The string length should be between 1 and 128 characters. Valid characters include <code>a-z</code>, <code>A-Z</code>, <code>0-9</code>, space, and the special characters <code>_ - . : / = + @</code>.</p>
    pub key: String,
    /// <p> A string that contains an optional <code>Tag</code> value.</p> <p>The string length should be between 0 and 256 characters. Valid characters include <code>a-z</code>, <code>A-Z</code>, <code>0-9</code>, space, and the special characters <code>_ - . : / = + @</code>.</p>
    pub value: Option<String>,
}

#[allow(dead_code)]
struct TagDeserializer;
impl TagDeserializer {
    #[allow(dead_code, unused_variables)]
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

pub struct TagSerializer;
impl TagSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Tag,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "Key", &obj.key.to_string())?;
        if let Some(ref value) = obj.value {
            write_characters_element(writer, "Value", &value.to_string())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct TagKeyDeserializer;
impl TagKeyDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct TagKeySerializer;
impl TagKeySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

pub struct TagKeyListSerializer;
impl TagKeyListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            TagKeySerializer::serialize(writer, "Key", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p> A complex type that contains zero or more <code>Tag</code> elements.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagKeys {
    /// <p> A complex type that contains <code>Tag</code> key elements.</p>
    pub items: Option<Vec<String>>,
}

pub struct TagKeysSerializer;
impl TagKeysSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &TagKeys,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &TagKeyListSerializer::serialize(&mut writer, "Items", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct TagListDeserializer;
impl TagListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Tag" {
                obj.push(TagDeserializer::deserialize("Tag", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct TagListSerializer;
impl TagListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<Tag>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            TagSerializer::serialize(writer, "Tag", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p> The request to add tags to a CloudFront resource.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p> An ARN of a CloudFront resource.</p>
    pub resource: String,
    /// <p> A complex type that contains zero or more <code>Tag</code> elements.</p>
    pub tags: Tags,
}

#[allow(dead_code)]
struct TagValueDeserializer;
impl TagValueDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct TagValueSerializer;
impl TagValueSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// <p> A complex type that contains zero or more <code>Tag</code> elements.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tags {
    /// <p> A complex type that contains <code>Tag</code> elements.</p>
    pub items: Option<Vec<Tag>>,
}

#[allow(dead_code)]
struct TagsDeserializer;
impl TagsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Tags, XmlParseError> {
        deserialize_elements::<_, Tags, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items
                        .get_or_insert(vec![])
                        .extend(TagListDeserializer::deserialize("Items", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct TagsSerializer;
impl TagsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Tags,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &TagListSerializer::serialize(&mut writer, "Items", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct TimestampDeserializer;
impl TimestampDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct TrustedKeyGroupIdListDeserializer;
impl TrustedKeyGroupIdListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "KeyGroup" {
                obj.push(StringDeserializer::deserialize("KeyGroup", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct TrustedKeyGroupIdListSerializer;
impl TrustedKeyGroupIdListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            StringSerializer::serialize(writer, "KeyGroup", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A list of key groups whose public keys CloudFront can use to verify the signatures of signed URLs and signed cookies.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TrustedKeyGroups {
    /// <p>This field is <code>true</code> if any of the key groups in the list have public keys that CloudFront can use to verify the signatures of signed URLs and signed cookies. If not, this field is <code>false</code>.</p>
    pub enabled: bool,
    /// <p>A list of key groups identifiers.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of key groups in the list.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct TrustedKeyGroupsDeserializer;
impl TrustedKeyGroupsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TrustedKeyGroups, XmlParseError> {
        deserialize_elements::<_, TrustedKeyGroups, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Enabled" => {
                    obj.enabled = BooleanDeserializer::deserialize("Enabled", stack)?;
                }
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        TrustedKeyGroupIdListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct TrustedKeyGroupsSerializer;
impl TrustedKeyGroupsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &TrustedKeyGroups,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "Enabled", &obj.enabled.to_string())?;
        if let Some(ref value) = obj.items {
            &TrustedKeyGroupIdListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A list of AWS accounts whose public keys CloudFront can use to verify the signatures of signed URLs and signed cookies.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TrustedSigners {
    /// <p>This field is <code>true</code> if any of the AWS accounts have public keys that CloudFront can use to verify the signatures of signed URLs and signed cookies. If not, this field is <code>false</code>.</p>
    pub enabled: bool,
    /// <p>A list of AWS account identifiers.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of AWS accounts in the list.</p>
    pub quantity: i64,
}

#[allow(dead_code)]
struct TrustedSignersDeserializer;
impl TrustedSignersDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TrustedSigners, XmlParseError> {
        deserialize_elements::<_, TrustedSigners, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Enabled" => {
                    obj.enabled = BooleanDeserializer::deserialize("Enabled", stack)?;
                }
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        AwsAccountNumberListDeserializer::deserialize("Items", stack)?,
                    );
                }
                "Quantity" => {
                    obj.quantity = IntegerDeserializer::deserialize("Quantity", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct TrustedSignersSerializer;
impl TrustedSignersSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &TrustedSigners,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        write_characters_element(writer, "Enabled", &obj.enabled.to_string())?;
        if let Some(ref value) = obj.items {
            &AwsAccountNumberListSerializer::serialize(&mut writer, "Items", value)?;
        }
        write_characters_element(writer, "Quantity", &obj.quantity.to_string())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p> The request to remove tags from a CloudFront resource.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p> An ARN of a CloudFront resource.</p>
    pub resource: String,
    /// <p> A complex type that contains zero or more <code>Tag</code> key elements.</p>
    pub tag_keys: TagKeys,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateCachePolicyRequest {
    /// <p>A cache policy configuration.</p>
    pub cache_policy_config: CachePolicyConfig,
    /// <p>The unique identifier for the cache policy that you are updating. The identifier is returned in a cache behavior’s <code>CachePolicyId</code> field in the response to <code>GetDistributionConfig</code>.</p>
    pub id: String,
    /// <p>The version of the cache policy that you are updating. The version is returned in the cache policy’s <code>ETag</code> field in the response to <code>GetCachePolicyConfig</code>.</p>
    pub if_match: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateCachePolicyResult {
    /// <p>A cache policy.</p>
    pub cache_policy: Option<CachePolicy>,
    /// <p>The current version of the cache policy.</p>
    pub e_tag: Option<String>,
}

#[allow(dead_code)]
struct UpdateCachePolicyResultDeserializer;
impl UpdateCachePolicyResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateCachePolicyResult, XmlParseError> {
        Ok(UpdateCachePolicyResult {
            cache_policy: Some(CachePolicyDeserializer::deserialize("CachePolicy", stack)?),
            ..UpdateCachePolicyResult::default()
        })
    }
}
/// <p>The request to update an origin access identity.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateCloudFrontOriginAccessIdentityRequest {
    /// <p>The identity's configuration information.</p>
    pub cloud_front_origin_access_identity_config: CloudFrontOriginAccessIdentityConfig,
    /// <p>The identity's id.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the identity's configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateCloudFrontOriginAccessIdentityResult {
    /// <p>The origin access identity's information.</p>
    pub cloud_front_origin_access_identity: Option<CloudFrontOriginAccessIdentity>,
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

#[allow(dead_code)]
struct UpdateCloudFrontOriginAccessIdentityResultDeserializer;
impl UpdateCloudFrontOriginAccessIdentityResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateCloudFrontOriginAccessIdentityResult, XmlParseError> {
        Ok(UpdateCloudFrontOriginAccessIdentityResult {
            cloud_front_origin_access_identity: Some(
                CloudFrontOriginAccessIdentityDeserializer::deserialize(
                    "CloudFrontOriginAccessIdentity",
                    stack,
                )?,
            ),
            ..UpdateCloudFrontOriginAccessIdentityResult::default()
        })
    }
}
/// <p>The request to update a distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDistributionRequest {
    /// <p>The distribution's configuration information.</p>
    pub distribution_config: DistributionConfig,
    /// <p>The distribution's id.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the distribution's configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateDistributionResult {
    /// <p>The distribution's information.</p>
    pub distribution: Option<Distribution>,
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

#[allow(dead_code)]
struct UpdateDistributionResultDeserializer;
impl UpdateDistributionResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateDistributionResult, XmlParseError> {
        Ok(UpdateDistributionResult {
            distribution: Some(DistributionDeserializer::deserialize(
                "Distribution",
                stack,
            )?),
            ..UpdateDistributionResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFieldLevelEncryptionConfigRequest {
    /// <p>Request to update a field-level encryption configuration. </p>
    pub field_level_encryption_config: FieldLevelEncryptionConfig,
    /// <p>The ID of the configuration you want to update.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the configuration identity to update. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateFieldLevelEncryptionConfigResult {
    /// <p>The value of the <code>ETag</code> header that you received when updating the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Return the results of updating the configuration.</p>
    pub field_level_encryption: Option<FieldLevelEncryption>,
}

#[allow(dead_code)]
struct UpdateFieldLevelEncryptionConfigResultDeserializer;
impl UpdateFieldLevelEncryptionConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateFieldLevelEncryptionConfigResult, XmlParseError> {
        Ok(UpdateFieldLevelEncryptionConfigResult {
            field_level_encryption: Some(FieldLevelEncryptionDeserializer::deserialize(
                "FieldLevelEncryption",
                stack,
            )?),
            ..UpdateFieldLevelEncryptionConfigResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFieldLevelEncryptionProfileRequest {
    /// <p>Request to update a field-level encryption profile. </p>
    pub field_level_encryption_profile_config: FieldLevelEncryptionProfileConfig,
    /// <p>The ID of the field-level encryption profile request. </p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the profile identity to update. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateFieldLevelEncryptionProfileResult {
    /// <p>The result of the field-level encryption profile request. </p>
    pub e_tag: Option<String>,
    /// <p>Return the results of updating the profile.</p>
    pub field_level_encryption_profile: Option<FieldLevelEncryptionProfile>,
}

#[allow(dead_code)]
struct UpdateFieldLevelEncryptionProfileResultDeserializer;
impl UpdateFieldLevelEncryptionProfileResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateFieldLevelEncryptionProfileResult, XmlParseError> {
        Ok(UpdateFieldLevelEncryptionProfileResult {
            field_level_encryption_profile: Some(
                FieldLevelEncryptionProfileDeserializer::deserialize(
                    "FieldLevelEncryptionProfile",
                    stack,
                )?,
            ),
            ..UpdateFieldLevelEncryptionProfileResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateKeyGroupRequest {
    /// <p>The identifier of the key group that you are updating.</p>
    pub id: String,
    /// <p>The version of the key group that you are updating. The version is the key group’s <code>ETag</code> value.</p>
    pub if_match: Option<String>,
    /// <p>The key group configuration.</p>
    pub key_group_config: KeyGroupConfig,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateKeyGroupResult {
    /// <p>The identifier for this version of the key group.</p>
    pub e_tag: Option<String>,
    /// <p>The key group that was just updated.</p>
    pub key_group: Option<KeyGroup>,
}

#[allow(dead_code)]
struct UpdateKeyGroupResultDeserializer;
impl UpdateKeyGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateKeyGroupResult, XmlParseError> {
        Ok(UpdateKeyGroupResult {
            key_group: Some(KeyGroupDeserializer::deserialize("KeyGroup", stack)?),
            ..UpdateKeyGroupResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateOriginRequestPolicyRequest {
    /// <p>The unique identifier for the origin request policy that you are updating. The identifier is returned in a cache behavior’s <code>OriginRequestPolicyId</code> field in the response to <code>GetDistributionConfig</code>.</p>
    pub id: String,
    /// <p>The version of the origin request policy that you are updating. The version is returned in the origin request policy’s <code>ETag</code> field in the response to <code>GetOriginRequestPolicyConfig</code>.</p>
    pub if_match: Option<String>,
    /// <p>An origin request policy configuration.</p>
    pub origin_request_policy_config: OriginRequestPolicyConfig,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateOriginRequestPolicyResult {
    /// <p>The current version of the origin request policy.</p>
    pub e_tag: Option<String>,
    /// <p>An origin request policy.</p>
    pub origin_request_policy: Option<OriginRequestPolicy>,
}

#[allow(dead_code)]
struct UpdateOriginRequestPolicyResultDeserializer;
impl UpdateOriginRequestPolicyResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateOriginRequestPolicyResult, XmlParseError> {
        Ok(UpdateOriginRequestPolicyResult {
            origin_request_policy: Some(OriginRequestPolicyDeserializer::deserialize(
                "OriginRequestPolicy",
                stack,
            )?),
            ..UpdateOriginRequestPolicyResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePublicKeyRequest {
    /// <p>The identifier of the public key that you are updating.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the public key to update. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
    /// <p>A public key configuration.</p>
    pub public_key_config: PublicKeyConfig,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdatePublicKeyResult {
    /// <p>The identifier of the current version of the public key.</p>
    pub e_tag: Option<String>,
    /// <p>The public key.</p>
    pub public_key: Option<PublicKey>,
}

#[allow(dead_code)]
struct UpdatePublicKeyResultDeserializer;
impl UpdatePublicKeyResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdatePublicKeyResult, XmlParseError> {
        Ok(UpdatePublicKeyResult {
            public_key: Some(PublicKeyDeserializer::deserialize("PublicKey", stack)?),
            ..UpdatePublicKeyResult::default()
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRealtimeLogConfigRequest {
    /// <p>The Amazon Resource Name (ARN) for this real-time log configuration.</p>
    pub arn: Option<String>,
    /// <p>Contains information about the Amazon Kinesis data stream where you are sending real-time log data.</p>
    pub end_points: Option<Vec<EndPoint>>,
    /// <p>A list of fields to include in each real-time log record.</p> <p>For more information about fields, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields">Real-time log configuration fields</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub fields: Option<Vec<String>>,
    /// <p>The name for this real-time log configuration.</p>
    pub name: Option<String>,
    /// <p>The sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. You must provide an integer between 1 and 100, inclusive.</p>
    pub sampling_rate: Option<i64>,
}

pub struct UpdateRealtimeLogConfigRequestSerializer;
impl UpdateRealtimeLogConfigRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &UpdateRealtimeLogConfigRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        if let Some(ref value) = obj.arn {
            &StringSerializer::serialize(&mut writer, "ARN", value)?;
        }
        if let Some(ref value) = obj.end_points {
            &EndPointListSerializer::serialize(&mut writer, "EndPoints", value)?;
        }
        if let Some(ref value) = obj.fields {
            &FieldListSerializer::serialize(&mut writer, "Fields", value)?;
        }
        if let Some(ref value) = obj.name {
            &StringSerializer::serialize(&mut writer, "Name", value)?;
        }
        if let Some(ref value) = obj.sampling_rate {
            &LongSerializer::serialize(&mut writer, "SamplingRate", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateRealtimeLogConfigResult {
    /// <p>A real-time log configuration.</p>
    pub realtime_log_config: Option<RealtimeLogConfig>,
}

#[allow(dead_code)]
struct UpdateRealtimeLogConfigResultDeserializer;
impl UpdateRealtimeLogConfigResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateRealtimeLogConfigResult, XmlParseError> {
        deserialize_elements::<_, UpdateRealtimeLogConfigResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "RealtimeLogConfig" => {
                        obj.realtime_log_config = Some(RealtimeLogConfigDeserializer::deserialize(
                            "RealtimeLogConfig",
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
/// <p>The request to update a streaming distribution.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateStreamingDistributionRequest {
    /// <p>The streaming distribution's id.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the streaming distribution's configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
    /// <p>The streaming distribution's configuration information.</p>
    pub streaming_distribution_config: StreamingDistributionConfig,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateStreamingDistributionResult {
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>The streaming distribution's information.</p>
    pub streaming_distribution: Option<StreamingDistribution>,
}

#[allow(dead_code)]
struct UpdateStreamingDistributionResultDeserializer;
impl UpdateStreamingDistributionResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateStreamingDistributionResult, XmlParseError> {
        Ok(UpdateStreamingDistributionResult {
            streaming_distribution: Some(StreamingDistributionDeserializer::deserialize(
                "StreamingDistribution",
                stack,
            )?),
            ..UpdateStreamingDistributionResult::default()
        })
    }
}
/// <p>A complex type that determines the distribution’s SSL/TLS configuration for communicating with viewers.</p> <p>If the distribution doesn’t use <code>Aliases</code> (also known as alternate domain names or CNAMEs)—that is, if the distribution uses the CloudFront domain name such as <code>d111111abcdef8.cloudfront.net</code>—set <code>CloudFrontDefaultCertificate</code> to <code>true</code> and leave all other fields empty.</p> <p>If the distribution uses <code>Aliases</code> (alternate domain names or CNAMEs), use the fields in this type to specify the following settings:</p> <ul> <li> <p>Which viewers the distribution accepts HTTPS connections from: only viewers that support <a href="https://en.wikipedia.org/wiki/Server_Name_Indication">server name indication (SNI)</a> (recommended), or all viewers including those that don’t support SNI.</p> <ul> <li> <p>To accept HTTPS connections from only viewers that support SNI, set <code>SSLSupportMethod</code> to <code>sni-only</code>. This is recommended. Most browsers and clients support SNI. </p> </li> <li> <p>To accept HTTPS connections from all viewers, including those that don’t support SNI, set <code>SSLSupportMethod</code> to <code>vip</code>. This is not recommended, and results in additional monthly charges from CloudFront. </p> </li> </ul> </li> <li> <p>The minimum SSL/TLS protocol version that the distribution can use to communicate with viewers. To specify a minimum version, choose a value for <code>MinimumProtocolVersion</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValues-security-policy">Security Policy</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </li> <li> <p>The location of the SSL/TLS certificate, <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-overview.html">AWS Certificate Manager (ACM)</a> (recommended) or <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_server-certs.html">AWS Identity and Access Management (AWS IAM)</a>. You specify the location by setting a value in one of the following fields (not both):</p> <ul> <li> <p> <code>ACMCertificateArn</code> </p> </li> <li> <p> <code>IAMCertificateId</code> </p> </li> </ul> </li> </ul> <p>All distributions support HTTPS connections from viewers. To require viewers to use HTTPS only, or to redirect them from HTTP to HTTPS, use <code>ViewerProtocolPolicy</code> in the <code>CacheBehavior</code> or <code>DefaultCacheBehavior</code>. To specify how CloudFront should use SSL/TLS to communicate with your custom origin, use <code>CustomOriginConfig</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-https.html">Using HTTPS with CloudFront</a> and <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-https-alternate-domain-names.html"> Using Alternate Domain Names and HTTPS</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ViewerCertificate {
    /// <p>If the distribution uses <code>Aliases</code> (alternate domain names or CNAMEs) and the SSL/TLS certificate is stored in <a href="https://docs.aws.amazon.com/acm/latest/userguide/acm-overview.html">AWS Certificate Manager (ACM)</a>, provide the Amazon Resource Name (ARN) of the ACM certificate. CloudFront only supports ACM certificates in the US East (N. Virginia) Region (<code>us-east-1</code>).</p> <p>If you specify an ACM certificate ARN, you must also specify values for <code>MinimumProtocolVerison</code> and <code>SSLSupportMethod</code>. </p>
    pub acm_certificate_arn: Option<String>,
    /// <p><p>If the distribution uses the CloudFront domain name such as <code>d111111abcdef8.cloudfront.net</code>, set this field to <code>true</code>.</p> <p>If the distribution uses <code>Aliases</code> (alternate domain names or CNAMEs), set this field to <code>false</code> and specify values for the following fields:</p> <ul> <li> <p> <code>ACMCertificateArn</code> or <code>IAMCertificateId</code> (specify a value for one, not both)</p> </li> <li> <p> <code>MinimumProtocolVersion</code> </p> </li> <li> <p> <code>SSLSupportMethod</code> </p> </li> </ul></p>
    pub cloud_front_default_certificate: Option<bool>,
    /// <p>If the distribution uses <code>Aliases</code> (alternate domain names or CNAMEs) and the SSL/TLS certificate is stored in <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_server-certs.html">AWS Identity and Access Management (AWS IAM)</a>, provide the ID of the IAM certificate.</p> <p>If you specify an IAM certificate ID, you must also specify values for <code>MinimumProtocolVerison</code> and <code>SSLSupportMethod</code>. </p>
    pub iam_certificate_id: Option<String>,
    /// <p>If the distribution uses <code>Aliases</code> (alternate domain names or CNAMEs), specify the security policy that you want CloudFront to use for HTTPS connections with viewers. The security policy determines two settings:</p> <ul> <li> <p>The minimum SSL/TLS protocol that CloudFront can use to communicate with viewers.</p> </li> <li> <p>The ciphers that CloudFront can use to encrypt the content that it returns to viewers.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValues-security-policy">Security Policy</a> and <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/secure-connections-supported-viewer-protocols-ciphers.html#secure-connections-supported-ciphers">Supported Protocols and Ciphers Between Viewers and CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <note> <p>On the CloudFront console, this setting is called <b>Security Policy</b>.</p> </note> <p>When you’re using SNI only (you set <code>SSLSupportMethod</code> to <code>sni-only</code>), you must specify <code>TLSv1</code> or higher. </p> <p>If the distribution uses the CloudFront domain name such as <code>d111111abcdef8.cloudfront.net</code> (you set <code>CloudFrontDefaultCertificate</code> to <code>true</code>), CloudFront automatically sets the security policy to <code>TLSv1</code> regardless of the value that you set here.</p>
    pub minimum_protocol_version: Option<String>,
    /// <p>If the distribution uses <code>Aliases</code> (alternate domain names or CNAMEs), specify which viewers the distribution accepts HTTPS connections from.</p> <ul> <li> <p> <code>sni-only</code> – The distribution accepts HTTPS connections from only viewers that support <a href="https://en.wikipedia.org/wiki/Server_Name_Indication">server name indication (SNI)</a>. This is recommended. Most browsers and clients support SNI.</p> </li> <li> <p> <code>vip</code> – The distribution accepts HTTPS connections from all viewers including those that don’t support SNI. This is not recommended, and results in additional monthly charges from CloudFront.</p> </li> <li> <p> <code>static-ip</code> - Do not specify this value unless your distribution has been enabled for this feature by the CloudFront team. If you have a use case that requires static IP addresses for a distribution, contact CloudFront through the <a href="https://console.aws.amazon.com/support/home">AWS Support Center</a>.</p> </li> </ul> <p>If the distribution uses the CloudFront domain name such as <code>d111111abcdef8.cloudfront.net</code>, don’t set a value for this field.</p>
    pub ssl_support_method: Option<String>,
}

#[allow(dead_code)]
struct ViewerCertificateDeserializer;
impl ViewerCertificateDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ViewerCertificate, XmlParseError> {
        deserialize_elements::<_, ViewerCertificate, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ACMCertificateArn" => {
                    obj.acm_certificate_arn =
                        Some(StringDeserializer::deserialize("ACMCertificateArn", stack)?);
                }
                "CloudFrontDefaultCertificate" => {
                    obj.cloud_front_default_certificate = Some(BooleanDeserializer::deserialize(
                        "CloudFrontDefaultCertificate",
                        stack,
                    )?);
                }
                "IAMCertificateId" => {
                    obj.iam_certificate_id =
                        Some(StringDeserializer::deserialize("IAMCertificateId", stack)?);
                }
                "MinimumProtocolVersion" => {
                    obj.minimum_protocol_version =
                        Some(MinimumProtocolVersionDeserializer::deserialize(
                            "MinimumProtocolVersion",
                            stack,
                        )?);
                }
                "SSLSupportMethod" => {
                    obj.ssl_support_method = Some(SSLSupportMethodDeserializer::deserialize(
                        "SSLSupportMethod",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct ViewerCertificateSerializer;
impl ViewerCertificateSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ViewerCertificate,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.acm_certificate_arn {
            write_characters_element(writer, "ACMCertificateArn", &value.to_string())?;
        }
        if let Some(ref value) = obj.cloud_front_default_certificate {
            write_characters_element(writer, "CloudFrontDefaultCertificate", &value.to_string())?;
        }
        if let Some(ref value) = obj.iam_certificate_id {
            write_characters_element(writer, "IAMCertificateId", &value.to_string())?;
        }
        if let Some(ref value) = obj.minimum_protocol_version {
            write_characters_element(writer, "MinimumProtocolVersion", &value.to_string())?;
        }
        if let Some(ref value) = obj.ssl_support_method {
            write_characters_element(writer, "SSLSupportMethod", &value.to_string())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[allow(dead_code)]
struct ViewerProtocolPolicyDeserializer;
impl ViewerProtocolPolicyDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

pub struct ViewerProtocolPolicySerializer;
impl ViewerProtocolPolicySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

/// Errors returned by CreateCachePolicy
#[derive(Debug, PartialEq)]
pub enum CreateCachePolicyError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>A cache policy with this name already exists. You must provide a unique name. To modify an existing cache policy, use <code>UpdateCachePolicy</code>.</p>
    CachePolicyAlreadyExists(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>You have reached the maximum number of cache policies for this AWS account. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyCachePolicies(String),
    /// <p>The number of cookies in the cache policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyCookiesInCachePolicy(String),
    /// <p>The number of headers in the cache policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyHeadersInCachePolicy(String),
    /// <p>The number of query strings in the cache policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyQueryStringsInCachePolicy(String),
}

impl CreateCachePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCachePolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(CreateCachePolicyError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "CachePolicyAlreadyExists" => {
                        return RusotoError::Service(
                            CreateCachePolicyError::CachePolicyAlreadyExists(parsed_error.message),
                        )
                    }
                    "InconsistentQuantities" => {
                        return RusotoError::Service(
                            CreateCachePolicyError::InconsistentQuantities(parsed_error.message),
                        )
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(CreateCachePolicyError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "TooManyCachePolicies" => {
                        return RusotoError::Service(CreateCachePolicyError::TooManyCachePolicies(
                            parsed_error.message,
                        ))
                    }
                    "TooManyCookiesInCachePolicy" => {
                        return RusotoError::Service(
                            CreateCachePolicyError::TooManyCookiesInCachePolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyHeadersInCachePolicy" => {
                        return RusotoError::Service(
                            CreateCachePolicyError::TooManyHeadersInCachePolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyQueryStringsInCachePolicy" => {
                        return RusotoError::Service(
                            CreateCachePolicyError::TooManyQueryStringsInCachePolicy(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateCachePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCachePolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateCachePolicyError::CachePolicyAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateCachePolicyError::InconsistentQuantities(ref cause) => write!(f, "{}", cause),
            CreateCachePolicyError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateCachePolicyError::TooManyCachePolicies(ref cause) => write!(f, "{}", cause),
            CreateCachePolicyError::TooManyCookiesInCachePolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCachePolicyError::TooManyHeadersInCachePolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCachePolicyError::TooManyQueryStringsInCachePolicy(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateCachePolicyError {}
/// Errors returned by CreateCloudFrontOriginAccessIdentity
#[derive(Debug, PartialEq)]
pub enum CreateCloudFrontOriginAccessIdentityError {
    /// <p>If the <code>CallerReference</code> is a value you already sent in a previous request to create an identity but the content of the <code>CloudFrontOriginAccessIdentityConfig</code> is different from the original request, CloudFront returns a <code>CloudFrontOriginAccessIdentityAlreadyExists</code> error. </p>
    CloudFrontOriginAccessIdentityAlreadyExists(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>Processing your request would cause you to exceed the maximum number of origin access identities allowed.</p>
    TooManyCloudFrontOriginAccessIdentities(String),
}

impl CreateCloudFrontOriginAccessIdentityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateCloudFrontOriginAccessIdentityError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "CloudFrontOriginAccessIdentityAlreadyExists" => return RusotoError::Service(CreateCloudFrontOriginAccessIdentityError::CloudFrontOriginAccessIdentityAlreadyExists(parsed_error.message)),"InconsistentQuantities" => return RusotoError::Service(CreateCloudFrontOriginAccessIdentityError::InconsistentQuantities(parsed_error.message)),"InvalidArgument" => return RusotoError::Service(CreateCloudFrontOriginAccessIdentityError::InvalidArgument(parsed_error.message)),"MissingBody" => return RusotoError::Service(CreateCloudFrontOriginAccessIdentityError::MissingBody(parsed_error.message)),"TooManyCloudFrontOriginAccessIdentities" => return RusotoError::Service(CreateCloudFrontOriginAccessIdentityError::TooManyCloudFrontOriginAccessIdentities(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateCloudFrontOriginAccessIdentityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            CreateCloudFrontOriginAccessIdentityError::CloudFrontOriginAccessIdentityAlreadyExists(ref cause) => write!(f, "{}", cause),
CreateCloudFrontOriginAccessIdentityError::InconsistentQuantities(ref cause) => write!(f, "{}", cause),
CreateCloudFrontOriginAccessIdentityError::InvalidArgument(ref cause) => write!(f, "{}", cause),
CreateCloudFrontOriginAccessIdentityError::MissingBody(ref cause) => write!(f, "{}", cause),
CreateCloudFrontOriginAccessIdentityError::TooManyCloudFrontOriginAccessIdentities(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for CreateCloudFrontOriginAccessIdentityError {}
/// Errors returned by CreateDistribution
#[derive(Debug, PartialEq)]
pub enum CreateDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The CNAME specified is already defined for CloudFront.</p>
    CNAMEAlreadyExists(String),
    /// <p>The caller reference you attempted to create the distribution with is associated with another distribution.</p>
    DistributionAlreadyExists(String),
    /// <p>The specified configuration for field-level encryption can't be associated with the specified cache behavior.</p>
    IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The default root object file name is too big or contains an invalid character.</p>
    InvalidDefaultRootObject(String),
    /// <p>An invalid error code was specified.</p>
    InvalidErrorCode(String),
    /// <p>Your request contains forward cookies option which doesn't match with the expectation for the <code>whitelisted</code> list of cookie names. Either list of cookie names has been specified when not allowed or list of cookie names is missing when expected.</p>
    InvalidForwardCookies(String),
    /// <p>The specified geo restriction parameter is not valid.</p>
    InvalidGeoRestrictionParameter(String),
    /// <p>The headers specified are not valid for an Amazon S3 origin.</p>
    InvalidHeadersForS3Origin(String),
    /// <p>The specified Lambda function association is invalid.</p>
    InvalidLambdaFunctionAssociation(String),
    /// <p>The location code specified is not valid.</p>
    InvalidLocationCode(String),
    /// <p>The minimum protocol version specified is not valid.</p>
    InvalidMinimumProtocolVersion(String),
    /// <p>The Amazon S3 origin server specified does not refer to a valid Amazon S3 bucket.</p>
    InvalidOrigin(String),
    /// <p>The origin access identity is not valid or doesn't exist.</p>
    InvalidOriginAccessIdentity(String),
    /// <p>The keep alive timeout specified for the origin is not valid.</p>
    InvalidOriginKeepaliveTimeout(String),
    /// <p>The read timeout specified for the origin is not valid.</p>
    InvalidOriginReadTimeout(String),
    /// <p>You cannot specify SSLv3 as the minimum protocol version if you only want to support only clients that support Server Name Indication (SNI).</p>
    InvalidProtocolSettings(String),
    /// <p>The query string parameters specified are not valid.</p>
    InvalidQueryStringParameters(String),
    /// <p>The relative path is too big, is not URL-encoded, or does not begin with a slash (/).</p>
    InvalidRelativePath(String),
    /// <p>This operation requires the HTTPS protocol. Ensure that you specify the HTTPS protocol in your request, or omit the <code>RequiredProtocols</code> element from your distribution configuration.</p>
    InvalidRequiredProtocol(String),
    /// <p>A response code is not valid.</p>
    InvalidResponseCode(String),
    /// <p>The TTL order specified is not valid.</p>
    InvalidTTLOrder(String),
    /// <p>A viewer certificate specified is not valid.</p>
    InvalidViewerCertificate(String),
    /// <p>A web ACL ID specified is not valid. To specify a web ACL created using the latest version of AWS WAF, use the ACL ARN, for example <code>arn:aws:wafv2:us-east-1:123456789012:global/webacl/ExampleWebACL/473e64fd-f30b-4765-81a0-62ad96dd167a</code>. To specify a web ACL created using AWS WAF Classic, use the ACL ID, for example <code>473e64fd-f30b-4765-81a0-62ad96dd167a</code>.</p>
    InvalidWebACLId(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>The cache policy does not exist.</p>
    NoSuchCachePolicy(String),
    /// <p>The specified configuration for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionConfig(String),
    /// <p>No origin exists with the specified <code>Origin Id</code>. </p>
    NoSuchOrigin(String),
    /// <p>The origin request policy does not exist.</p>
    NoSuchOriginRequestPolicy(String),
    /// <p>You cannot create more cache behaviors for the distribution.</p>
    TooManyCacheBehaviors(String),
    /// <p>You cannot create anymore custom SSL/TLS certificates.</p>
    TooManyCertificates(String),
    /// <p>Your request contains more cookie names in the whitelist than are allowed per cache behavior.</p>
    TooManyCookieNamesInWhiteList(String),
    /// <p>Your request contains more CNAMEs than are allowed per distribution.</p>
    TooManyDistributionCNAMEs(String),
    /// <p>Processing your request would cause you to exceed the maximum number of distributions allowed.</p>
    TooManyDistributions(String),
    /// <p>The maximum number of distributions have been associated with the specified cache policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyDistributionsAssociatedToCachePolicy(String),
    /// <p>The maximum number of distributions have been associated with the specified configuration for field-level encryption.</p>
    TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(String),
    /// <p>The number of distributions that reference this key group is more than the maximum allowed. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyDistributionsAssociatedToKeyGroup(String),
    /// <p>The maximum number of distributions have been associated with the specified origin request policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyDistributionsAssociatedToOriginRequestPolicy(String),
    /// <p>Processing your request would cause the maximum number of distributions with Lambda function associations per owner to be exceeded.</p>
    TooManyDistributionsWithLambdaAssociations(String),
    /// <p>The maximum number of distributions have been associated with the specified Lambda function.</p>
    TooManyDistributionsWithSingleFunctionARN(String),
    /// <p>Your request contains too many headers in forwarded values.</p>
    TooManyHeadersInForwardedValues(String),
    /// <p>The number of key groups referenced by this distribution is more than the maximum allowed. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyKeyGroupsAssociatedToDistribution(String),
    /// <p>Your request contains more Lambda function associations than are allowed per distribution.</p>
    TooManyLambdaFunctionAssociations(String),
    /// <p>Your request contains too many origin custom headers.</p>
    TooManyOriginCustomHeaders(String),
    /// <p>Processing your request would cause you to exceed the maximum number of origin groups allowed.</p>
    TooManyOriginGroupsPerDistribution(String),
    /// <p>You cannot create more origins for the distribution.</p>
    TooManyOrigins(String),
    /// <p>Your request contains too many query string parameters.</p>
    TooManyQueryStringParameters(String),
    /// <p>Your request contains more trusted signers than are allowed per distribution.</p>
    TooManyTrustedSigners(String),
    /// <p>The specified key group does not exist.</p>
    TrustedKeyGroupDoesNotExist(String),
    /// <p>One or more of your trusted signers don't exist.</p>
    TrustedSignerDoesNotExist(String),
}

impl CreateDistributionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDistributionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "AccessDenied" => return RusotoError::Service(CreateDistributionError::AccessDenied(parsed_error.message)),"CNAMEAlreadyExists" => return RusotoError::Service(CreateDistributionError::CNAMEAlreadyExists(parsed_error.message)),"DistributionAlreadyExists" => return RusotoError::Service(CreateDistributionError::DistributionAlreadyExists(parsed_error.message)),"IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior" => return RusotoError::Service(CreateDistributionError::IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(parsed_error.message)),"InconsistentQuantities" => return RusotoError::Service(CreateDistributionError::InconsistentQuantities(parsed_error.message)),"InvalidArgument" => return RusotoError::Service(CreateDistributionError::InvalidArgument(parsed_error.message)),"InvalidDefaultRootObject" => return RusotoError::Service(CreateDistributionError::InvalidDefaultRootObject(parsed_error.message)),"InvalidErrorCode" => return RusotoError::Service(CreateDistributionError::InvalidErrorCode(parsed_error.message)),"InvalidForwardCookies" => return RusotoError::Service(CreateDistributionError::InvalidForwardCookies(parsed_error.message)),"InvalidGeoRestrictionParameter" => return RusotoError::Service(CreateDistributionError::InvalidGeoRestrictionParameter(parsed_error.message)),"InvalidHeadersForS3Origin" => return RusotoError::Service(CreateDistributionError::InvalidHeadersForS3Origin(parsed_error.message)),"InvalidLambdaFunctionAssociation" => return RusotoError::Service(CreateDistributionError::InvalidLambdaFunctionAssociation(parsed_error.message)),"InvalidLocationCode" => return RusotoError::Service(CreateDistributionError::InvalidLocationCode(parsed_error.message)),"InvalidMinimumProtocolVersion" => return RusotoError::Service(CreateDistributionError::InvalidMinimumProtocolVersion(parsed_error.message)),"InvalidOrigin" => return RusotoError::Service(CreateDistributionError::InvalidOrigin(parsed_error.message)),"InvalidOriginAccessIdentity" => return RusotoError::Service(CreateDistributionError::InvalidOriginAccessIdentity(parsed_error.message)),"InvalidOriginKeepaliveTimeout" => return RusotoError::Service(CreateDistributionError::InvalidOriginKeepaliveTimeout(parsed_error.message)),"InvalidOriginReadTimeout" => return RusotoError::Service(CreateDistributionError::InvalidOriginReadTimeout(parsed_error.message)),"InvalidProtocolSettings" => return RusotoError::Service(CreateDistributionError::InvalidProtocolSettings(parsed_error.message)),"InvalidQueryStringParameters" => return RusotoError::Service(CreateDistributionError::InvalidQueryStringParameters(parsed_error.message)),"InvalidRelativePath" => return RusotoError::Service(CreateDistributionError::InvalidRelativePath(parsed_error.message)),"InvalidRequiredProtocol" => return RusotoError::Service(CreateDistributionError::InvalidRequiredProtocol(parsed_error.message)),"InvalidResponseCode" => return RusotoError::Service(CreateDistributionError::InvalidResponseCode(parsed_error.message)),"InvalidTTLOrder" => return RusotoError::Service(CreateDistributionError::InvalidTTLOrder(parsed_error.message)),"InvalidViewerCertificate" => return RusotoError::Service(CreateDistributionError::InvalidViewerCertificate(parsed_error.message)),"InvalidWebACLId" => return RusotoError::Service(CreateDistributionError::InvalidWebACLId(parsed_error.message)),"MissingBody" => return RusotoError::Service(CreateDistributionError::MissingBody(parsed_error.message)),"NoSuchCachePolicy" => return RusotoError::Service(CreateDistributionError::NoSuchCachePolicy(parsed_error.message)),"NoSuchFieldLevelEncryptionConfig" => return RusotoError::Service(CreateDistributionError::NoSuchFieldLevelEncryptionConfig(parsed_error.message)),"NoSuchOrigin" => return RusotoError::Service(CreateDistributionError::NoSuchOrigin(parsed_error.message)),"NoSuchOriginRequestPolicy" => return RusotoError::Service(CreateDistributionError::NoSuchOriginRequestPolicy(parsed_error.message)),"TooManyCacheBehaviors" => return RusotoError::Service(CreateDistributionError::TooManyCacheBehaviors(parsed_error.message)),"TooManyCertificates" => return RusotoError::Service(CreateDistributionError::TooManyCertificates(parsed_error.message)),"TooManyCookieNamesInWhiteList" => return RusotoError::Service(CreateDistributionError::TooManyCookieNamesInWhiteList(parsed_error.message)),"TooManyDistributionCNAMEs" => return RusotoError::Service(CreateDistributionError::TooManyDistributionCNAMEs(parsed_error.message)),"TooManyDistributions" => return RusotoError::Service(CreateDistributionError::TooManyDistributions(parsed_error.message)),"TooManyDistributionsAssociatedToCachePolicy" => return RusotoError::Service(CreateDistributionError::TooManyDistributionsAssociatedToCachePolicy(parsed_error.message)),"TooManyDistributionsAssociatedToFieldLevelEncryptionConfig" => return RusotoError::Service(CreateDistributionError::TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(parsed_error.message)),"TooManyDistributionsAssociatedToKeyGroup" => return RusotoError::Service(CreateDistributionError::TooManyDistributionsAssociatedToKeyGroup(parsed_error.message)),"TooManyDistributionsAssociatedToOriginRequestPolicy" => return RusotoError::Service(CreateDistributionError::TooManyDistributionsAssociatedToOriginRequestPolicy(parsed_error.message)),"TooManyDistributionsWithLambdaAssociations" => return RusotoError::Service(CreateDistributionError::TooManyDistributionsWithLambdaAssociations(parsed_error.message)),"TooManyDistributionsWithSingleFunctionARN" => return RusotoError::Service(CreateDistributionError::TooManyDistributionsWithSingleFunctionARN(parsed_error.message)),"TooManyHeadersInForwardedValues" => return RusotoError::Service(CreateDistributionError::TooManyHeadersInForwardedValues(parsed_error.message)),"TooManyKeyGroupsAssociatedToDistribution" => return RusotoError::Service(CreateDistributionError::TooManyKeyGroupsAssociatedToDistribution(parsed_error.message)),"TooManyLambdaFunctionAssociations" => return RusotoError::Service(CreateDistributionError::TooManyLambdaFunctionAssociations(parsed_error.message)),"TooManyOriginCustomHeaders" => return RusotoError::Service(CreateDistributionError::TooManyOriginCustomHeaders(parsed_error.message)),"TooManyOriginGroupsPerDistribution" => return RusotoError::Service(CreateDistributionError::TooManyOriginGroupsPerDistribution(parsed_error.message)),"TooManyOrigins" => return RusotoError::Service(CreateDistributionError::TooManyOrigins(parsed_error.message)),"TooManyQueryStringParameters" => return RusotoError::Service(CreateDistributionError::TooManyQueryStringParameters(parsed_error.message)),"TooManyTrustedSigners" => return RusotoError::Service(CreateDistributionError::TooManyTrustedSigners(parsed_error.message)),"TrustedKeyGroupDoesNotExist" => return RusotoError::Service(CreateDistributionError::TrustedKeyGroupDoesNotExist(parsed_error.message)),"TrustedSignerDoesNotExist" => return RusotoError::Service(CreateDistributionError::TrustedSignerDoesNotExist(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateDistributionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            CreateDistributionError::AccessDenied(ref cause) => write!(f, "{}", cause),
CreateDistributionError::CNAMEAlreadyExists(ref cause) => write!(f, "{}", cause),
CreateDistributionError::DistributionAlreadyExists(ref cause) => write!(f, "{}", cause),
CreateDistributionError::IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InconsistentQuantities(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidArgument(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidDefaultRootObject(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidErrorCode(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidForwardCookies(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidGeoRestrictionParameter(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidHeadersForS3Origin(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidLambdaFunctionAssociation(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidLocationCode(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidMinimumProtocolVersion(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidOrigin(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidOriginAccessIdentity(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidOriginKeepaliveTimeout(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidOriginReadTimeout(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidProtocolSettings(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidQueryStringParameters(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidRelativePath(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidRequiredProtocol(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidResponseCode(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidTTLOrder(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidViewerCertificate(ref cause) => write!(f, "{}", cause),
CreateDistributionError::InvalidWebACLId(ref cause) => write!(f, "{}", cause),
CreateDistributionError::MissingBody(ref cause) => write!(f, "{}", cause),
CreateDistributionError::NoSuchCachePolicy(ref cause) => write!(f, "{}", cause),
CreateDistributionError::NoSuchFieldLevelEncryptionConfig(ref cause) => write!(f, "{}", cause),
CreateDistributionError::NoSuchOrigin(ref cause) => write!(f, "{}", cause),
CreateDistributionError::NoSuchOriginRequestPolicy(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyCacheBehaviors(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyCertificates(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyCookieNamesInWhiteList(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyDistributionCNAMEs(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyDistributions(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyDistributionsAssociatedToCachePolicy(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyDistributionsAssociatedToKeyGroup(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyDistributionsAssociatedToOriginRequestPolicy(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyDistributionsWithLambdaAssociations(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyDistributionsWithSingleFunctionARN(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyHeadersInForwardedValues(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyKeyGroupsAssociatedToDistribution(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyLambdaFunctionAssociations(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyOriginCustomHeaders(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyOriginGroupsPerDistribution(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyOrigins(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyQueryStringParameters(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyTrustedSigners(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TrustedKeyGroupDoesNotExist(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TrustedSignerDoesNotExist(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for CreateDistributionError {}
/// Errors returned by CreateDistributionWithTags
#[derive(Debug, PartialEq)]
pub enum CreateDistributionWithTagsError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The CNAME specified is already defined for CloudFront.</p>
    CNAMEAlreadyExists(String),
    /// <p>The caller reference you attempted to create the distribution with is associated with another distribution.</p>
    DistributionAlreadyExists(String),
    /// <p>The specified configuration for field-level encryption can't be associated with the specified cache behavior.</p>
    IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The default root object file name is too big or contains an invalid character.</p>
    InvalidDefaultRootObject(String),
    /// <p>An invalid error code was specified.</p>
    InvalidErrorCode(String),
    /// <p>Your request contains forward cookies option which doesn't match with the expectation for the <code>whitelisted</code> list of cookie names. Either list of cookie names has been specified when not allowed or list of cookie names is missing when expected.</p>
    InvalidForwardCookies(String),
    /// <p>The specified geo restriction parameter is not valid.</p>
    InvalidGeoRestrictionParameter(String),
    /// <p>The headers specified are not valid for an Amazon S3 origin.</p>
    InvalidHeadersForS3Origin(String),
    /// <p>The specified Lambda function association is invalid.</p>
    InvalidLambdaFunctionAssociation(String),
    /// <p>The location code specified is not valid.</p>
    InvalidLocationCode(String),
    /// <p>The minimum protocol version specified is not valid.</p>
    InvalidMinimumProtocolVersion(String),
    /// <p>The Amazon S3 origin server specified does not refer to a valid Amazon S3 bucket.</p>
    InvalidOrigin(String),
    /// <p>The origin access identity is not valid or doesn't exist.</p>
    InvalidOriginAccessIdentity(String),
    /// <p>The keep alive timeout specified for the origin is not valid.</p>
    InvalidOriginKeepaliveTimeout(String),
    /// <p>The read timeout specified for the origin is not valid.</p>
    InvalidOriginReadTimeout(String),
    /// <p>You cannot specify SSLv3 as the minimum protocol version if you only want to support only clients that support Server Name Indication (SNI).</p>
    InvalidProtocolSettings(String),
    /// <p>The query string parameters specified are not valid.</p>
    InvalidQueryStringParameters(String),
    /// <p>The relative path is too big, is not URL-encoded, or does not begin with a slash (/).</p>
    InvalidRelativePath(String),
    /// <p>This operation requires the HTTPS protocol. Ensure that you specify the HTTPS protocol in your request, or omit the <code>RequiredProtocols</code> element from your distribution configuration.</p>
    InvalidRequiredProtocol(String),
    /// <p>A response code is not valid.</p>
    InvalidResponseCode(String),
    /// <p>The TTL order specified is not valid.</p>
    InvalidTTLOrder(String),
    /// <p>The tagging specified is not valid.</p>
    InvalidTagging(String),
    /// <p>A viewer certificate specified is not valid.</p>
    InvalidViewerCertificate(String),
    /// <p>A web ACL ID specified is not valid. To specify a web ACL created using the latest version of AWS WAF, use the ACL ARN, for example <code>arn:aws:wafv2:us-east-1:123456789012:global/webacl/ExampleWebACL/473e64fd-f30b-4765-81a0-62ad96dd167a</code>. To specify a web ACL created using AWS WAF Classic, use the ACL ID, for example <code>473e64fd-f30b-4765-81a0-62ad96dd167a</code>.</p>
    InvalidWebACLId(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>The cache policy does not exist.</p>
    NoSuchCachePolicy(String),
    /// <p>The specified configuration for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionConfig(String),
    /// <p>No origin exists with the specified <code>Origin Id</code>. </p>
    NoSuchOrigin(String),
    /// <p>The origin request policy does not exist.</p>
    NoSuchOriginRequestPolicy(String),
    /// <p>You cannot create more cache behaviors for the distribution.</p>
    TooManyCacheBehaviors(String),
    /// <p>You cannot create anymore custom SSL/TLS certificates.</p>
    TooManyCertificates(String),
    /// <p>Your request contains more cookie names in the whitelist than are allowed per cache behavior.</p>
    TooManyCookieNamesInWhiteList(String),
    /// <p>Your request contains more CNAMEs than are allowed per distribution.</p>
    TooManyDistributionCNAMEs(String),
    /// <p>Processing your request would cause you to exceed the maximum number of distributions allowed.</p>
    TooManyDistributions(String),
    /// <p>The maximum number of distributions have been associated with the specified cache policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyDistributionsAssociatedToCachePolicy(String),
    /// <p>The maximum number of distributions have been associated with the specified configuration for field-level encryption.</p>
    TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(String),
    /// <p>The number of distributions that reference this key group is more than the maximum allowed. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyDistributionsAssociatedToKeyGroup(String),
    /// <p>The maximum number of distributions have been associated with the specified origin request policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyDistributionsAssociatedToOriginRequestPolicy(String),
    /// <p>Processing your request would cause the maximum number of distributions with Lambda function associations per owner to be exceeded.</p>
    TooManyDistributionsWithLambdaAssociations(String),
    /// <p>The maximum number of distributions have been associated with the specified Lambda function.</p>
    TooManyDistributionsWithSingleFunctionARN(String),
    /// <p>Your request contains too many headers in forwarded values.</p>
    TooManyHeadersInForwardedValues(String),
    /// <p>The number of key groups referenced by this distribution is more than the maximum allowed. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyKeyGroupsAssociatedToDistribution(String),
    /// <p>Your request contains more Lambda function associations than are allowed per distribution.</p>
    TooManyLambdaFunctionAssociations(String),
    /// <p>Your request contains too many origin custom headers.</p>
    TooManyOriginCustomHeaders(String),
    /// <p>Processing your request would cause you to exceed the maximum number of origin groups allowed.</p>
    TooManyOriginGroupsPerDistribution(String),
    /// <p>You cannot create more origins for the distribution.</p>
    TooManyOrigins(String),
    /// <p>Your request contains too many query string parameters.</p>
    TooManyQueryStringParameters(String),
    /// <p>Your request contains more trusted signers than are allowed per distribution.</p>
    TooManyTrustedSigners(String),
    /// <p>The specified key group does not exist.</p>
    TrustedKeyGroupDoesNotExist(String),
    /// <p>One or more of your trusted signers don't exist.</p>
    TrustedSignerDoesNotExist(String),
}

impl CreateDistributionWithTagsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateDistributionWithTagsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "AccessDenied" => return RusotoError::Service(CreateDistributionWithTagsError::AccessDenied(parsed_error.message)),"CNAMEAlreadyExists" => return RusotoError::Service(CreateDistributionWithTagsError::CNAMEAlreadyExists(parsed_error.message)),"DistributionAlreadyExists" => return RusotoError::Service(CreateDistributionWithTagsError::DistributionAlreadyExists(parsed_error.message)),"IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior" => return RusotoError::Service(CreateDistributionWithTagsError::IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(parsed_error.message)),"InconsistentQuantities" => return RusotoError::Service(CreateDistributionWithTagsError::InconsistentQuantities(parsed_error.message)),"InvalidArgument" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidArgument(parsed_error.message)),"InvalidDefaultRootObject" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidDefaultRootObject(parsed_error.message)),"InvalidErrorCode" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidErrorCode(parsed_error.message)),"InvalidForwardCookies" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidForwardCookies(parsed_error.message)),"InvalidGeoRestrictionParameter" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidGeoRestrictionParameter(parsed_error.message)),"InvalidHeadersForS3Origin" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidHeadersForS3Origin(parsed_error.message)),"InvalidLambdaFunctionAssociation" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidLambdaFunctionAssociation(parsed_error.message)),"InvalidLocationCode" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidLocationCode(parsed_error.message)),"InvalidMinimumProtocolVersion" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidMinimumProtocolVersion(parsed_error.message)),"InvalidOrigin" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidOrigin(parsed_error.message)),"InvalidOriginAccessIdentity" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidOriginAccessIdentity(parsed_error.message)),"InvalidOriginKeepaliveTimeout" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidOriginKeepaliveTimeout(parsed_error.message)),"InvalidOriginReadTimeout" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidOriginReadTimeout(parsed_error.message)),"InvalidProtocolSettings" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidProtocolSettings(parsed_error.message)),"InvalidQueryStringParameters" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidQueryStringParameters(parsed_error.message)),"InvalidRelativePath" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidRelativePath(parsed_error.message)),"InvalidRequiredProtocol" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidRequiredProtocol(parsed_error.message)),"InvalidResponseCode" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidResponseCode(parsed_error.message)),"InvalidTTLOrder" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidTTLOrder(parsed_error.message)),"InvalidTagging" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidTagging(parsed_error.message)),"InvalidViewerCertificate" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidViewerCertificate(parsed_error.message)),"InvalidWebACLId" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidWebACLId(parsed_error.message)),"MissingBody" => return RusotoError::Service(CreateDistributionWithTagsError::MissingBody(parsed_error.message)),"NoSuchCachePolicy" => return RusotoError::Service(CreateDistributionWithTagsError::NoSuchCachePolicy(parsed_error.message)),"NoSuchFieldLevelEncryptionConfig" => return RusotoError::Service(CreateDistributionWithTagsError::NoSuchFieldLevelEncryptionConfig(parsed_error.message)),"NoSuchOrigin" => return RusotoError::Service(CreateDistributionWithTagsError::NoSuchOrigin(parsed_error.message)),"NoSuchOriginRequestPolicy" => return RusotoError::Service(CreateDistributionWithTagsError::NoSuchOriginRequestPolicy(parsed_error.message)),"TooManyCacheBehaviors" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyCacheBehaviors(parsed_error.message)),"TooManyCertificates" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyCertificates(parsed_error.message)),"TooManyCookieNamesInWhiteList" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyCookieNamesInWhiteList(parsed_error.message)),"TooManyDistributionCNAMEs" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyDistributionCNAMEs(parsed_error.message)),"TooManyDistributions" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyDistributions(parsed_error.message)),"TooManyDistributionsAssociatedToCachePolicy" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyDistributionsAssociatedToCachePolicy(parsed_error.message)),"TooManyDistributionsAssociatedToFieldLevelEncryptionConfig" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(parsed_error.message)),"TooManyDistributionsAssociatedToKeyGroup" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyDistributionsAssociatedToKeyGroup(parsed_error.message)),"TooManyDistributionsAssociatedToOriginRequestPolicy" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyDistributionsAssociatedToOriginRequestPolicy(parsed_error.message)),"TooManyDistributionsWithLambdaAssociations" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyDistributionsWithLambdaAssociations(parsed_error.message)),"TooManyDistributionsWithSingleFunctionARN" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyDistributionsWithSingleFunctionARN(parsed_error.message)),"TooManyHeadersInForwardedValues" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyHeadersInForwardedValues(parsed_error.message)),"TooManyKeyGroupsAssociatedToDistribution" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyKeyGroupsAssociatedToDistribution(parsed_error.message)),"TooManyLambdaFunctionAssociations" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyLambdaFunctionAssociations(parsed_error.message)),"TooManyOriginCustomHeaders" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyOriginCustomHeaders(parsed_error.message)),"TooManyOriginGroupsPerDistribution" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyOriginGroupsPerDistribution(parsed_error.message)),"TooManyOrigins" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyOrigins(parsed_error.message)),"TooManyQueryStringParameters" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyQueryStringParameters(parsed_error.message)),"TooManyTrustedSigners" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyTrustedSigners(parsed_error.message)),"TrustedKeyGroupDoesNotExist" => return RusotoError::Service(CreateDistributionWithTagsError::TrustedKeyGroupDoesNotExist(parsed_error.message)),"TrustedSignerDoesNotExist" => return RusotoError::Service(CreateDistributionWithTagsError::TrustedSignerDoesNotExist(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateDistributionWithTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            CreateDistributionWithTagsError::AccessDenied(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::CNAMEAlreadyExists(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::DistributionAlreadyExists(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InconsistentQuantities(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidDefaultRootObject(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidErrorCode(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidForwardCookies(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidGeoRestrictionParameter(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidHeadersForS3Origin(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidLambdaFunctionAssociation(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidLocationCode(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidMinimumProtocolVersion(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidOrigin(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidOriginAccessIdentity(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidOriginKeepaliveTimeout(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidOriginReadTimeout(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidProtocolSettings(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidQueryStringParameters(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidRelativePath(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidRequiredProtocol(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidResponseCode(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidTTLOrder(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidTagging(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidViewerCertificate(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::InvalidWebACLId(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::MissingBody(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::NoSuchCachePolicy(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::NoSuchFieldLevelEncryptionConfig(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::NoSuchOrigin(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::NoSuchOriginRequestPolicy(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyCacheBehaviors(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyCertificates(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyCookieNamesInWhiteList(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyDistributionCNAMEs(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyDistributions(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyDistributionsAssociatedToCachePolicy(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyDistributionsAssociatedToKeyGroup(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyDistributionsAssociatedToOriginRequestPolicy(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyDistributionsWithLambdaAssociations(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyDistributionsWithSingleFunctionARN(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyHeadersInForwardedValues(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyKeyGroupsAssociatedToDistribution(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyLambdaFunctionAssociations(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyOriginCustomHeaders(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyOriginGroupsPerDistribution(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyOrigins(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyQueryStringParameters(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyTrustedSigners(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TrustedKeyGroupDoesNotExist(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TrustedSignerDoesNotExist(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for CreateDistributionWithTagsError {}
/// Errors returned by CreateFieldLevelEncryptionConfig
#[derive(Debug, PartialEq)]
pub enum CreateFieldLevelEncryptionConfigError {
    /// <p>The specified configuration for field-level encryption already exists.</p>
    FieldLevelEncryptionConfigAlreadyExists(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The specified profile for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionProfile(String),
    /// <p>No profile specified for the field-level encryption query argument.</p>
    QueryArgProfileEmpty(String),
    /// <p>The maximum number of configurations for field-level encryption have been created.</p>
    TooManyFieldLevelEncryptionConfigs(String),
    /// <p>The maximum number of content type profiles for field-level encryption have been created.</p>
    TooManyFieldLevelEncryptionContentTypeProfiles(String),
    /// <p>The maximum number of query arg profiles for field-level encryption have been created.</p>
    TooManyFieldLevelEncryptionQueryArgProfiles(String),
}

impl CreateFieldLevelEncryptionConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateFieldLevelEncryptionConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "FieldLevelEncryptionConfigAlreadyExists" => return RusotoError::Service(CreateFieldLevelEncryptionConfigError::FieldLevelEncryptionConfigAlreadyExists(parsed_error.message)),"InconsistentQuantities" => return RusotoError::Service(CreateFieldLevelEncryptionConfigError::InconsistentQuantities(parsed_error.message)),"InvalidArgument" => return RusotoError::Service(CreateFieldLevelEncryptionConfigError::InvalidArgument(parsed_error.message)),"NoSuchFieldLevelEncryptionProfile" => return RusotoError::Service(CreateFieldLevelEncryptionConfigError::NoSuchFieldLevelEncryptionProfile(parsed_error.message)),"QueryArgProfileEmpty" => return RusotoError::Service(CreateFieldLevelEncryptionConfigError::QueryArgProfileEmpty(parsed_error.message)),"TooManyFieldLevelEncryptionConfigs" => return RusotoError::Service(CreateFieldLevelEncryptionConfigError::TooManyFieldLevelEncryptionConfigs(parsed_error.message)),"TooManyFieldLevelEncryptionContentTypeProfiles" => return RusotoError::Service(CreateFieldLevelEncryptionConfigError::TooManyFieldLevelEncryptionContentTypeProfiles(parsed_error.message)),"TooManyFieldLevelEncryptionQueryArgProfiles" => return RusotoError::Service(CreateFieldLevelEncryptionConfigError::TooManyFieldLevelEncryptionQueryArgProfiles(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateFieldLevelEncryptionConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            CreateFieldLevelEncryptionConfigError::FieldLevelEncryptionConfigAlreadyExists(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionConfigError::InconsistentQuantities(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionConfigError::InvalidArgument(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionConfigError::NoSuchFieldLevelEncryptionProfile(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionConfigError::QueryArgProfileEmpty(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionConfigError::TooManyFieldLevelEncryptionConfigs(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionConfigError::TooManyFieldLevelEncryptionContentTypeProfiles(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionConfigError::TooManyFieldLevelEncryptionQueryArgProfiles(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for CreateFieldLevelEncryptionConfigError {}
/// Errors returned by CreateFieldLevelEncryptionProfile
#[derive(Debug, PartialEq)]
pub enum CreateFieldLevelEncryptionProfileError {
    /// <p>The specified profile for field-level encryption already exists.</p>
    FieldLevelEncryptionProfileAlreadyExists(String),
    /// <p>The maximum size of a profile for field-level encryption was exceeded.</p>
    FieldLevelEncryptionProfileSizeExceeded(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The specified public key doesn't exist.</p>
    NoSuchPublicKey(String),
    /// <p>The maximum number of encryption entities for field-level encryption have been created.</p>
    TooManyFieldLevelEncryptionEncryptionEntities(String),
    /// <p>The maximum number of field patterns for field-level encryption have been created.</p>
    TooManyFieldLevelEncryptionFieldPatterns(String),
    /// <p>The maximum number of profiles for field-level encryption have been created.</p>
    TooManyFieldLevelEncryptionProfiles(String),
}

impl CreateFieldLevelEncryptionProfileError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateFieldLevelEncryptionProfileError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "FieldLevelEncryptionProfileAlreadyExists" => return RusotoError::Service(CreateFieldLevelEncryptionProfileError::FieldLevelEncryptionProfileAlreadyExists(parsed_error.message)),"FieldLevelEncryptionProfileSizeExceeded" => return RusotoError::Service(CreateFieldLevelEncryptionProfileError::FieldLevelEncryptionProfileSizeExceeded(parsed_error.message)),"InconsistentQuantities" => return RusotoError::Service(CreateFieldLevelEncryptionProfileError::InconsistentQuantities(parsed_error.message)),"InvalidArgument" => return RusotoError::Service(CreateFieldLevelEncryptionProfileError::InvalidArgument(parsed_error.message)),"NoSuchPublicKey" => return RusotoError::Service(CreateFieldLevelEncryptionProfileError::NoSuchPublicKey(parsed_error.message)),"TooManyFieldLevelEncryptionEncryptionEntities" => return RusotoError::Service(CreateFieldLevelEncryptionProfileError::TooManyFieldLevelEncryptionEncryptionEntities(parsed_error.message)),"TooManyFieldLevelEncryptionFieldPatterns" => return RusotoError::Service(CreateFieldLevelEncryptionProfileError::TooManyFieldLevelEncryptionFieldPatterns(parsed_error.message)),"TooManyFieldLevelEncryptionProfiles" => return RusotoError::Service(CreateFieldLevelEncryptionProfileError::TooManyFieldLevelEncryptionProfiles(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateFieldLevelEncryptionProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            CreateFieldLevelEncryptionProfileError::FieldLevelEncryptionProfileAlreadyExists(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionProfileError::FieldLevelEncryptionProfileSizeExceeded(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionProfileError::InconsistentQuantities(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionProfileError::InvalidArgument(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionProfileError::NoSuchPublicKey(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionProfileError::TooManyFieldLevelEncryptionEncryptionEntities(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionProfileError::TooManyFieldLevelEncryptionFieldPatterns(ref cause) => write!(f, "{}", cause),
CreateFieldLevelEncryptionProfileError::TooManyFieldLevelEncryptionProfiles(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for CreateFieldLevelEncryptionProfileError {}
/// Errors returned by CreateInvalidation
#[derive(Debug, PartialEq)]
pub enum CreateInvalidationError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>Invalidation batch specified is too large.</p>
    BatchTooLarge(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
    /// <p>You have exceeded the maximum number of allowable InProgress invalidation batch requests, or invalidation objects.</p>
    TooManyInvalidationsInProgress(String),
}

impl CreateInvalidationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInvalidationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(CreateInvalidationError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "BatchTooLarge" => {
                        return RusotoError::Service(CreateInvalidationError::BatchTooLarge(
                            parsed_error.message,
                        ))
                    }
                    "InconsistentQuantities" => {
                        return RusotoError::Service(
                            CreateInvalidationError::InconsistentQuantities(parsed_error.message),
                        )
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(CreateInvalidationError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "MissingBody" => {
                        return RusotoError::Service(CreateInvalidationError::MissingBody(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDistribution" => {
                        return RusotoError::Service(CreateInvalidationError::NoSuchDistribution(
                            parsed_error.message,
                        ))
                    }
                    "TooManyInvalidationsInProgress" => {
                        return RusotoError::Service(
                            CreateInvalidationError::TooManyInvalidationsInProgress(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateInvalidationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInvalidationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateInvalidationError::BatchTooLarge(ref cause) => write!(f, "{}", cause),
            CreateInvalidationError::InconsistentQuantities(ref cause) => write!(f, "{}", cause),
            CreateInvalidationError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateInvalidationError::MissingBody(ref cause) => write!(f, "{}", cause),
            CreateInvalidationError::NoSuchDistribution(ref cause) => write!(f, "{}", cause),
            CreateInvalidationError::TooManyInvalidationsInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateInvalidationError {}
/// Errors returned by CreateKeyGroup
#[derive(Debug, PartialEq)]
pub enum CreateKeyGroupError {
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>A key group with this name already exists. You must provide a unique name. To modify an existing key group, use <code>UpdateKeyGroup</code>.</p>
    KeyGroupAlreadyExists(String),
    /// <p>You have reached the maximum number of key groups for this AWS account. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyKeyGroups(String),
    /// <p>The number of public keys in this key group is more than the maximum allowed. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyPublicKeysInKeyGroup(String),
}

impl CreateKeyGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateKeyGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidArgument" => {
                        return RusotoError::Service(CreateKeyGroupError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "KeyGroupAlreadyExists" => {
                        return RusotoError::Service(CreateKeyGroupError::KeyGroupAlreadyExists(
                            parsed_error.message,
                        ))
                    }
                    "TooManyKeyGroups" => {
                        return RusotoError::Service(CreateKeyGroupError::TooManyKeyGroups(
                            parsed_error.message,
                        ))
                    }
                    "TooManyPublicKeysInKeyGroup" => {
                        return RusotoError::Service(
                            CreateKeyGroupError::TooManyPublicKeysInKeyGroup(parsed_error.message),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateKeyGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateKeyGroupError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateKeyGroupError::KeyGroupAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateKeyGroupError::TooManyKeyGroups(ref cause) => write!(f, "{}", cause),
            CreateKeyGroupError::TooManyPublicKeysInKeyGroup(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateKeyGroupError {}
/// Errors returned by CreateMonitoringSubscription
#[derive(Debug, PartialEq)]
pub enum CreateMonitoringSubscriptionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
}

impl CreateMonitoringSubscriptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateMonitoringSubscriptionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(
                            CreateMonitoringSubscriptionError::AccessDenied(parsed_error.message),
                        )
                    }
                    "NoSuchDistribution" => {
                        return RusotoError::Service(
                            CreateMonitoringSubscriptionError::NoSuchDistribution(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateMonitoringSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMonitoringSubscriptionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateMonitoringSubscriptionError::NoSuchDistribution(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateMonitoringSubscriptionError {}
/// Errors returned by CreateOriginRequestPolicy
#[derive(Debug, PartialEq)]
pub enum CreateOriginRequestPolicyError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>An origin request policy with this name already exists. You must provide a unique name. To modify an existing origin request policy, use <code>UpdateOriginRequestPolicy</code>.</p>
    OriginRequestPolicyAlreadyExists(String),
    /// <p>The number of cookies in the origin request policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyCookiesInOriginRequestPolicy(String),
    /// <p>The number of headers in the origin request policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyHeadersInOriginRequestPolicy(String),
    /// <p>You have reached the maximum number of origin request policies for this AWS account. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyOriginRequestPolicies(String),
    /// <p>The number of query strings in the origin request policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyQueryStringsInOriginRequestPolicy(String),
}

impl CreateOriginRequestPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateOriginRequestPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(CreateOriginRequestPolicyError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "InconsistentQuantities" => {
                        return RusotoError::Service(
                            CreateOriginRequestPolicyError::InconsistentQuantities(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            CreateOriginRequestPolicyError::InvalidArgument(parsed_error.message),
                        )
                    }
                    "OriginRequestPolicyAlreadyExists" => {
                        return RusotoError::Service(
                            CreateOriginRequestPolicyError::OriginRequestPolicyAlreadyExists(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyCookiesInOriginRequestPolicy" => {
                        return RusotoError::Service(
                            CreateOriginRequestPolicyError::TooManyCookiesInOriginRequestPolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyHeadersInOriginRequestPolicy" => {
                        return RusotoError::Service(
                            CreateOriginRequestPolicyError::TooManyHeadersInOriginRequestPolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyOriginRequestPolicies" => {
                        return RusotoError::Service(
                            CreateOriginRequestPolicyError::TooManyOriginRequestPolicies(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyQueryStringsInOriginRequestPolicy" => return RusotoError::Service(
                        CreateOriginRequestPolicyError::TooManyQueryStringsInOriginRequestPolicy(
                            parsed_error.message,
                        ),
                    ),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateOriginRequestPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateOriginRequestPolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateOriginRequestPolicyError::InconsistentQuantities(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateOriginRequestPolicyError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateOriginRequestPolicyError::OriginRequestPolicyAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateOriginRequestPolicyError::TooManyCookiesInOriginRequestPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateOriginRequestPolicyError::TooManyHeadersInOriginRequestPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateOriginRequestPolicyError::TooManyOriginRequestPolicies(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateOriginRequestPolicyError::TooManyQueryStringsInOriginRequestPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateOriginRequestPolicyError {}
/// Errors returned by CreatePublicKey
#[derive(Debug, PartialEq)]
pub enum CreatePublicKeyError {
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The specified public key already exists.</p>
    PublicKeyAlreadyExists(String),
    /// <p>The maximum number of public keys for field-level encryption have been created. To create a new public key, delete one of the existing keys.</p>
    TooManyPublicKeys(String),
}

impl CreatePublicKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePublicKeyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidArgument" => {
                        return RusotoError::Service(CreatePublicKeyError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "PublicKeyAlreadyExists" => {
                        return RusotoError::Service(CreatePublicKeyError::PublicKeyAlreadyExists(
                            parsed_error.message,
                        ))
                    }
                    "TooManyPublicKeys" => {
                        return RusotoError::Service(CreatePublicKeyError::TooManyPublicKeys(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreatePublicKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePublicKeyError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreatePublicKeyError::PublicKeyAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreatePublicKeyError::TooManyPublicKeys(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePublicKeyError {}
/// Errors returned by CreateRealtimeLogConfig
#[derive(Debug, PartialEq)]
pub enum CreateRealtimeLogConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>A real-time log configuration with this name already exists. You must provide a unique name. To modify an existing real-time log configuration, use <code>UpdateRealtimeLogConfig</code>.</p>
    RealtimeLogConfigAlreadyExists(String),
    /// <p>You have reached the maximum number of real-time log configurations for this AWS account. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyRealtimeLogConfigs(String),
}

impl CreateRealtimeLogConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRealtimeLogConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(CreateRealtimeLogConfigError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(CreateRealtimeLogConfigError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "RealtimeLogConfigAlreadyExists" => {
                        return RusotoError::Service(
                            CreateRealtimeLogConfigError::RealtimeLogConfigAlreadyExists(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyRealtimeLogConfigs" => {
                        return RusotoError::Service(
                            CreateRealtimeLogConfigError::TooManyRealtimeLogConfigs(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateRealtimeLogConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRealtimeLogConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateRealtimeLogConfigError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateRealtimeLogConfigError::RealtimeLogConfigAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRealtimeLogConfigError::TooManyRealtimeLogConfigs(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateRealtimeLogConfigError {}
/// Errors returned by CreateStreamingDistribution
#[derive(Debug, PartialEq)]
pub enum CreateStreamingDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The CNAME specified is already defined for CloudFront.</p>
    CNAMEAlreadyExists(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The Amazon S3 origin server specified does not refer to a valid Amazon S3 bucket.</p>
    InvalidOrigin(String),
    /// <p>The origin access identity is not valid or doesn't exist.</p>
    InvalidOriginAccessIdentity(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>The caller reference you attempted to create the streaming distribution with is associated with another distribution</p>
    StreamingDistributionAlreadyExists(String),
    /// <p>Your request contains more CNAMEs than are allowed per distribution.</p>
    TooManyStreamingDistributionCNAMEs(String),
    /// <p>Processing your request would cause you to exceed the maximum number of streaming distributions allowed.</p>
    TooManyStreamingDistributions(String),
    /// <p>Your request contains more trusted signers than are allowed per distribution.</p>
    TooManyTrustedSigners(String),
    /// <p>One or more of your trusted signers don't exist.</p>
    TrustedSignerDoesNotExist(String),
}

impl CreateStreamingDistributionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateStreamingDistributionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(
                            CreateStreamingDistributionError::AccessDenied(parsed_error.message),
                        )
                    }
                    "CNAMEAlreadyExists" => {
                        return RusotoError::Service(
                            CreateStreamingDistributionError::CNAMEAlreadyExists(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InconsistentQuantities" => {
                        return RusotoError::Service(
                            CreateStreamingDistributionError::InconsistentQuantities(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            CreateStreamingDistributionError::InvalidArgument(parsed_error.message),
                        )
                    }
                    "InvalidOrigin" => {
                        return RusotoError::Service(
                            CreateStreamingDistributionError::InvalidOrigin(parsed_error.message),
                        )
                    }
                    "InvalidOriginAccessIdentity" => {
                        return RusotoError::Service(
                            CreateStreamingDistributionError::InvalidOriginAccessIdentity(
                                parsed_error.message,
                            ),
                        )
                    }
                    "MissingBody" => {
                        return RusotoError::Service(CreateStreamingDistributionError::MissingBody(
                            parsed_error.message,
                        ))
                    }
                    "StreamingDistributionAlreadyExists" => {
                        return RusotoError::Service(
                            CreateStreamingDistributionError::StreamingDistributionAlreadyExists(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyStreamingDistributionCNAMEs" => {
                        return RusotoError::Service(
                            CreateStreamingDistributionError::TooManyStreamingDistributionCNAMEs(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyStreamingDistributions" => {
                        return RusotoError::Service(
                            CreateStreamingDistributionError::TooManyStreamingDistributions(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyTrustedSigners" => {
                        return RusotoError::Service(
                            CreateStreamingDistributionError::TooManyTrustedSigners(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TrustedSignerDoesNotExist" => {
                        return RusotoError::Service(
                            CreateStreamingDistributionError::TrustedSignerDoesNotExist(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateStreamingDistributionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateStreamingDistributionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateStreamingDistributionError::CNAMEAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionError::InconsistentQuantities(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateStreamingDistributionError::InvalidOrigin(ref cause) => write!(f, "{}", cause),
            CreateStreamingDistributionError::InvalidOriginAccessIdentity(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionError::MissingBody(ref cause) => write!(f, "{}", cause),
            CreateStreamingDistributionError::StreamingDistributionAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionError::TooManyStreamingDistributionCNAMEs(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionError::TooManyStreamingDistributions(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionError::TooManyTrustedSigners(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionError::TrustedSignerDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateStreamingDistributionError {}
/// Errors returned by CreateStreamingDistributionWithTags
#[derive(Debug, PartialEq)]
pub enum CreateStreamingDistributionWithTagsError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The CNAME specified is already defined for CloudFront.</p>
    CNAMEAlreadyExists(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The Amazon S3 origin server specified does not refer to a valid Amazon S3 bucket.</p>
    InvalidOrigin(String),
    /// <p>The origin access identity is not valid or doesn't exist.</p>
    InvalidOriginAccessIdentity(String),
    /// <p>The tagging specified is not valid.</p>
    InvalidTagging(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>The caller reference you attempted to create the streaming distribution with is associated with another distribution</p>
    StreamingDistributionAlreadyExists(String),
    /// <p>Your request contains more CNAMEs than are allowed per distribution.</p>
    TooManyStreamingDistributionCNAMEs(String),
    /// <p>Processing your request would cause you to exceed the maximum number of streaming distributions allowed.</p>
    TooManyStreamingDistributions(String),
    /// <p>Your request contains more trusted signers than are allowed per distribution.</p>
    TooManyTrustedSigners(String),
    /// <p>One or more of your trusted signers don't exist.</p>
    TrustedSignerDoesNotExist(String),
}

impl CreateStreamingDistributionWithTagsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateStreamingDistributionWithTagsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "AccessDenied" => return RusotoError::Service(CreateStreamingDistributionWithTagsError::AccessDenied(parsed_error.message)),"CNAMEAlreadyExists" => return RusotoError::Service(CreateStreamingDistributionWithTagsError::CNAMEAlreadyExists(parsed_error.message)),"InconsistentQuantities" => return RusotoError::Service(CreateStreamingDistributionWithTagsError::InconsistentQuantities(parsed_error.message)),"InvalidArgument" => return RusotoError::Service(CreateStreamingDistributionWithTagsError::InvalidArgument(parsed_error.message)),"InvalidOrigin" => return RusotoError::Service(CreateStreamingDistributionWithTagsError::InvalidOrigin(parsed_error.message)),"InvalidOriginAccessIdentity" => return RusotoError::Service(CreateStreamingDistributionWithTagsError::InvalidOriginAccessIdentity(parsed_error.message)),"InvalidTagging" => return RusotoError::Service(CreateStreamingDistributionWithTagsError::InvalidTagging(parsed_error.message)),"MissingBody" => return RusotoError::Service(CreateStreamingDistributionWithTagsError::MissingBody(parsed_error.message)),"StreamingDistributionAlreadyExists" => return RusotoError::Service(CreateStreamingDistributionWithTagsError::StreamingDistributionAlreadyExists(parsed_error.message)),"TooManyStreamingDistributionCNAMEs" => return RusotoError::Service(CreateStreamingDistributionWithTagsError::TooManyStreamingDistributionCNAMEs(parsed_error.message)),"TooManyStreamingDistributions" => return RusotoError::Service(CreateStreamingDistributionWithTagsError::TooManyStreamingDistributions(parsed_error.message)),"TooManyTrustedSigners" => return RusotoError::Service(CreateStreamingDistributionWithTagsError::TooManyTrustedSigners(parsed_error.message)),"TrustedSignerDoesNotExist" => return RusotoError::Service(CreateStreamingDistributionWithTagsError::TrustedSignerDoesNotExist(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateStreamingDistributionWithTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateStreamingDistributionWithTagsError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionWithTagsError::CNAMEAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionWithTagsError::InconsistentQuantities(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionWithTagsError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionWithTagsError::InvalidOrigin(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionWithTagsError::InvalidOriginAccessIdentity(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionWithTagsError::InvalidTagging(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionWithTagsError::MissingBody(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionWithTagsError::StreamingDistributionAlreadyExists(
                ref cause,
            ) => write!(f, "{}", cause),
            CreateStreamingDistributionWithTagsError::TooManyStreamingDistributionCNAMEs(
                ref cause,
            ) => write!(f, "{}", cause),
            CreateStreamingDistributionWithTagsError::TooManyStreamingDistributions(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionWithTagsError::TooManyTrustedSigners(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamingDistributionWithTagsError::TrustedSignerDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateStreamingDistributionWithTagsError {}
/// Errors returned by DeleteCachePolicy
#[derive(Debug, PartialEq)]
pub enum DeleteCachePolicyError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>Cannot delete the cache policy because it is attached to one or more cache behaviors.</p>
    CachePolicyInUse(String),
    /// <p>You cannot delete a managed policy.</p>
    IllegalDelete(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The cache policy does not exist.</p>
    NoSuchCachePolicy(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
}

impl DeleteCachePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCachePolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(DeleteCachePolicyError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "CachePolicyInUse" => {
                        return RusotoError::Service(DeleteCachePolicyError::CachePolicyInUse(
                            parsed_error.message,
                        ))
                    }
                    "IllegalDelete" => {
                        return RusotoError::Service(DeleteCachePolicyError::IllegalDelete(
                            parsed_error.message,
                        ))
                    }
                    "InvalidIfMatchVersion" => {
                        return RusotoError::Service(DeleteCachePolicyError::InvalidIfMatchVersion(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchCachePolicy" => {
                        return RusotoError::Service(DeleteCachePolicyError::NoSuchCachePolicy(
                            parsed_error.message,
                        ))
                    }
                    "PreconditionFailed" => {
                        return RusotoError::Service(DeleteCachePolicyError::PreconditionFailed(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteCachePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCachePolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteCachePolicyError::CachePolicyInUse(ref cause) => write!(f, "{}", cause),
            DeleteCachePolicyError::IllegalDelete(ref cause) => write!(f, "{}", cause),
            DeleteCachePolicyError::InvalidIfMatchVersion(ref cause) => write!(f, "{}", cause),
            DeleteCachePolicyError::NoSuchCachePolicy(ref cause) => write!(f, "{}", cause),
            DeleteCachePolicyError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCachePolicyError {}
/// Errors returned by DeleteCloudFrontOriginAccessIdentity
#[derive(Debug, PartialEq)]
pub enum DeleteCloudFrontOriginAccessIdentityError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The Origin Access Identity specified is already in use.</p>
    CloudFrontOriginAccessIdentityInUse(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified origin access identity does not exist.</p>
    NoSuchCloudFrontOriginAccessIdentity(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
}

impl DeleteCloudFrontOriginAccessIdentityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteCloudFrontOriginAccessIdentityError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "AccessDenied" => return RusotoError::Service(DeleteCloudFrontOriginAccessIdentityError::AccessDenied(parsed_error.message)),"CloudFrontOriginAccessIdentityInUse" => return RusotoError::Service(DeleteCloudFrontOriginAccessIdentityError::CloudFrontOriginAccessIdentityInUse(parsed_error.message)),"InvalidIfMatchVersion" => return RusotoError::Service(DeleteCloudFrontOriginAccessIdentityError::InvalidIfMatchVersion(parsed_error.message)),"NoSuchCloudFrontOriginAccessIdentity" => return RusotoError::Service(DeleteCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity(parsed_error.message)),"PreconditionFailed" => return RusotoError::Service(DeleteCloudFrontOriginAccessIdentityError::PreconditionFailed(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteCloudFrontOriginAccessIdentityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCloudFrontOriginAccessIdentityError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCloudFrontOriginAccessIdentityError::CloudFrontOriginAccessIdentityInUse(
                ref cause,
            ) => write!(f, "{}", cause),
            DeleteCloudFrontOriginAccessIdentityError::InvalidIfMatchVersion(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity(
                ref cause,
            ) => write!(f, "{}", cause),
            DeleteCloudFrontOriginAccessIdentityError::PreconditionFailed(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteCloudFrontOriginAccessIdentityError {}
/// Errors returned by DeleteDistribution
#[derive(Debug, PartialEq)]
pub enum DeleteDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified CloudFront distribution is not disabled. You must disable the distribution before you can delete it.</p>
    DistributionNotDisabled(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
}

impl DeleteDistributionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDistributionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(DeleteDistributionError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "DistributionNotDisabled" => {
                        return RusotoError::Service(
                            DeleteDistributionError::DistributionNotDisabled(parsed_error.message),
                        )
                    }
                    "InvalidIfMatchVersion" => {
                        return RusotoError::Service(
                            DeleteDistributionError::InvalidIfMatchVersion(parsed_error.message),
                        )
                    }
                    "NoSuchDistribution" => {
                        return RusotoError::Service(DeleteDistributionError::NoSuchDistribution(
                            parsed_error.message,
                        ))
                    }
                    "PreconditionFailed" => {
                        return RusotoError::Service(DeleteDistributionError::PreconditionFailed(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteDistributionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDistributionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteDistributionError::DistributionNotDisabled(ref cause) => write!(f, "{}", cause),
            DeleteDistributionError::InvalidIfMatchVersion(ref cause) => write!(f, "{}", cause),
            DeleteDistributionError::NoSuchDistribution(ref cause) => write!(f, "{}", cause),
            DeleteDistributionError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDistributionError {}
/// Errors returned by DeleteFieldLevelEncryptionConfig
#[derive(Debug, PartialEq)]
pub enum DeleteFieldLevelEncryptionConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified configuration for field-level encryption is in use.</p>
    FieldLevelEncryptionConfigInUse(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified configuration for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionConfig(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
}

impl DeleteFieldLevelEncryptionConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteFieldLevelEncryptionConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(
                            DeleteFieldLevelEncryptionConfigError::AccessDenied(
                                parsed_error.message,
                            ),
                        )
                    }
                    "FieldLevelEncryptionConfigInUse" => {
                        return RusotoError::Service(
                            DeleteFieldLevelEncryptionConfigError::FieldLevelEncryptionConfigInUse(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidIfMatchVersion" => {
                        return RusotoError::Service(
                            DeleteFieldLevelEncryptionConfigError::InvalidIfMatchVersion(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchFieldLevelEncryptionConfig" => {
                        return RusotoError::Service(
                            DeleteFieldLevelEncryptionConfigError::NoSuchFieldLevelEncryptionConfig(
                                parsed_error.message,
                            ),
                        )
                    }
                    "PreconditionFailed" => {
                        return RusotoError::Service(
                            DeleteFieldLevelEncryptionConfigError::PreconditionFailed(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteFieldLevelEncryptionConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFieldLevelEncryptionConfigError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFieldLevelEncryptionConfigError::FieldLevelEncryptionConfigInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFieldLevelEncryptionConfigError::InvalidIfMatchVersion(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFieldLevelEncryptionConfigError::NoSuchFieldLevelEncryptionConfig(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFieldLevelEncryptionConfigError::PreconditionFailed(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteFieldLevelEncryptionConfigError {}
/// Errors returned by DeleteFieldLevelEncryptionProfile
#[derive(Debug, PartialEq)]
pub enum DeleteFieldLevelEncryptionProfileError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified profile for field-level encryption is in use.</p>
    FieldLevelEncryptionProfileInUse(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified profile for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionProfile(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
}

impl DeleteFieldLevelEncryptionProfileError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteFieldLevelEncryptionProfileError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(
                            DeleteFieldLevelEncryptionProfileError::AccessDenied(
                                parsed_error.message,
                            ),
                        )
                    }
                    "FieldLevelEncryptionProfileInUse" => return RusotoError::Service(
                        DeleteFieldLevelEncryptionProfileError::FieldLevelEncryptionProfileInUse(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidIfMatchVersion" => {
                        return RusotoError::Service(
                            DeleteFieldLevelEncryptionProfileError::InvalidIfMatchVersion(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchFieldLevelEncryptionProfile" => return RusotoError::Service(
                        DeleteFieldLevelEncryptionProfileError::NoSuchFieldLevelEncryptionProfile(
                            parsed_error.message,
                        ),
                    ),
                    "PreconditionFailed" => {
                        return RusotoError::Service(
                            DeleteFieldLevelEncryptionProfileError::PreconditionFailed(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteFieldLevelEncryptionProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFieldLevelEncryptionProfileError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFieldLevelEncryptionProfileError::FieldLevelEncryptionProfileInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFieldLevelEncryptionProfileError::InvalidIfMatchVersion(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFieldLevelEncryptionProfileError::NoSuchFieldLevelEncryptionProfile(
                ref cause,
            ) => write!(f, "{}", cause),
            DeleteFieldLevelEncryptionProfileError::PreconditionFailed(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteFieldLevelEncryptionProfileError {}
/// Errors returned by DeleteKeyGroup
#[derive(Debug, PartialEq)]
pub enum DeleteKeyGroupError {
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>A resource that was specified is not valid.</p>
    NoSuchResource(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
    /// <p>Cannot delete this resource because it is in use.</p>
    ResourceInUse(String),
}

impl DeleteKeyGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteKeyGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidIfMatchVersion" => {
                        return RusotoError::Service(DeleteKeyGroupError::InvalidIfMatchVersion(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchResource" => {
                        return RusotoError::Service(DeleteKeyGroupError::NoSuchResource(
                            parsed_error.message,
                        ))
                    }
                    "PreconditionFailed" => {
                        return RusotoError::Service(DeleteKeyGroupError::PreconditionFailed(
                            parsed_error.message,
                        ))
                    }
                    "ResourceInUse" => {
                        return RusotoError::Service(DeleteKeyGroupError::ResourceInUse(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteKeyGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteKeyGroupError::InvalidIfMatchVersion(ref cause) => write!(f, "{}", cause),
            DeleteKeyGroupError::NoSuchResource(ref cause) => write!(f, "{}", cause),
            DeleteKeyGroupError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
            DeleteKeyGroupError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteKeyGroupError {}
/// Errors returned by DeleteMonitoringSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteMonitoringSubscriptionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
}

impl DeleteMonitoringSubscriptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteMonitoringSubscriptionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(
                            DeleteMonitoringSubscriptionError::AccessDenied(parsed_error.message),
                        )
                    }
                    "NoSuchDistribution" => {
                        return RusotoError::Service(
                            DeleteMonitoringSubscriptionError::NoSuchDistribution(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteMonitoringSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMonitoringSubscriptionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteMonitoringSubscriptionError::NoSuchDistribution(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteMonitoringSubscriptionError {}
/// Errors returned by DeleteOriginRequestPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteOriginRequestPolicyError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>You cannot delete a managed policy.</p>
    IllegalDelete(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The origin request policy does not exist.</p>
    NoSuchOriginRequestPolicy(String),
    /// <p>Cannot delete the origin request policy because it is attached to one or more cache behaviors.</p>
    OriginRequestPolicyInUse(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
}

impl DeleteOriginRequestPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteOriginRequestPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(DeleteOriginRequestPolicyError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "IllegalDelete" => {
                        return RusotoError::Service(DeleteOriginRequestPolicyError::IllegalDelete(
                            parsed_error.message,
                        ))
                    }
                    "InvalidIfMatchVersion" => {
                        return RusotoError::Service(
                            DeleteOriginRequestPolicyError::InvalidIfMatchVersion(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchOriginRequestPolicy" => {
                        return RusotoError::Service(
                            DeleteOriginRequestPolicyError::NoSuchOriginRequestPolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "OriginRequestPolicyInUse" => {
                        return RusotoError::Service(
                            DeleteOriginRequestPolicyError::OriginRequestPolicyInUse(
                                parsed_error.message,
                            ),
                        )
                    }
                    "PreconditionFailed" => {
                        return RusotoError::Service(
                            DeleteOriginRequestPolicyError::PreconditionFailed(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteOriginRequestPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteOriginRequestPolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteOriginRequestPolicyError::IllegalDelete(ref cause) => write!(f, "{}", cause),
            DeleteOriginRequestPolicyError::InvalidIfMatchVersion(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteOriginRequestPolicyError::NoSuchOriginRequestPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteOriginRequestPolicyError::OriginRequestPolicyInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteOriginRequestPolicyError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteOriginRequestPolicyError {}
/// Errors returned by DeletePublicKey
#[derive(Debug, PartialEq)]
pub enum DeletePublicKeyError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified public key doesn't exist.</p>
    NoSuchPublicKey(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
    /// <p>The specified public key is in use. </p>
    PublicKeyInUse(String),
}

impl DeletePublicKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePublicKeyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(DeletePublicKeyError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "InvalidIfMatchVersion" => {
                        return RusotoError::Service(DeletePublicKeyError::InvalidIfMatchVersion(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchPublicKey" => {
                        return RusotoError::Service(DeletePublicKeyError::NoSuchPublicKey(
                            parsed_error.message,
                        ))
                    }
                    "PreconditionFailed" => {
                        return RusotoError::Service(DeletePublicKeyError::PreconditionFailed(
                            parsed_error.message,
                        ))
                    }
                    "PublicKeyInUse" => {
                        return RusotoError::Service(DeletePublicKeyError::PublicKeyInUse(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeletePublicKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePublicKeyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeletePublicKeyError::InvalidIfMatchVersion(ref cause) => write!(f, "{}", cause),
            DeletePublicKeyError::NoSuchPublicKey(ref cause) => write!(f, "{}", cause),
            DeletePublicKeyError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
            DeletePublicKeyError::PublicKeyInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePublicKeyError {}
/// Errors returned by DeleteRealtimeLogConfig
#[derive(Debug, PartialEq)]
pub enum DeleteRealtimeLogConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The real-time log configuration does not exist.</p>
    NoSuchRealtimeLogConfig(String),
    /// <p>Cannot delete the real-time log configuration because it is attached to one or more cache behaviors.</p>
    RealtimeLogConfigInUse(String),
}

impl DeleteRealtimeLogConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRealtimeLogConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(DeleteRealtimeLogConfigError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(DeleteRealtimeLogConfigError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchRealtimeLogConfig" => {
                        return RusotoError::Service(
                            DeleteRealtimeLogConfigError::NoSuchRealtimeLogConfig(
                                parsed_error.message,
                            ),
                        )
                    }
                    "RealtimeLogConfigInUse" => {
                        return RusotoError::Service(
                            DeleteRealtimeLogConfigError::RealtimeLogConfigInUse(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteRealtimeLogConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRealtimeLogConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteRealtimeLogConfigError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DeleteRealtimeLogConfigError::NoSuchRealtimeLogConfig(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteRealtimeLogConfigError::RealtimeLogConfigInUse(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteRealtimeLogConfigError {}
/// Errors returned by DeleteStreamingDistribution
#[derive(Debug, PartialEq)]
pub enum DeleteStreamingDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified streaming distribution does not exist.</p>
    NoSuchStreamingDistribution(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
    /// <p>The specified CloudFront distribution is not disabled. You must disable the distribution before you can delete it.</p>
    StreamingDistributionNotDisabled(String),
}

impl DeleteStreamingDistributionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteStreamingDistributionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(
                            DeleteStreamingDistributionError::AccessDenied(parsed_error.message),
                        )
                    }
                    "InvalidIfMatchVersion" => {
                        return RusotoError::Service(
                            DeleteStreamingDistributionError::InvalidIfMatchVersion(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchStreamingDistribution" => {
                        return RusotoError::Service(
                            DeleteStreamingDistributionError::NoSuchStreamingDistribution(
                                parsed_error.message,
                            ),
                        )
                    }
                    "PreconditionFailed" => {
                        return RusotoError::Service(
                            DeleteStreamingDistributionError::PreconditionFailed(
                                parsed_error.message,
                            ),
                        )
                    }
                    "StreamingDistributionNotDisabled" => {
                        return RusotoError::Service(
                            DeleteStreamingDistributionError::StreamingDistributionNotDisabled(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteStreamingDistributionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteStreamingDistributionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteStreamingDistributionError::InvalidIfMatchVersion(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteStreamingDistributionError::NoSuchStreamingDistribution(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteStreamingDistributionError::PreconditionFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteStreamingDistributionError::StreamingDistributionNotDisabled(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteStreamingDistributionError {}
/// Errors returned by GetCachePolicy
#[derive(Debug, PartialEq)]
pub enum GetCachePolicyError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The cache policy does not exist.</p>
    NoSuchCachePolicy(String),
}

impl GetCachePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCachePolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(GetCachePolicyError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchCachePolicy" => {
                        return RusotoError::Service(GetCachePolicyError::NoSuchCachePolicy(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetCachePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCachePolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetCachePolicyError::NoSuchCachePolicy(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCachePolicyError {}
/// Errors returned by GetCachePolicyConfig
#[derive(Debug, PartialEq)]
pub enum GetCachePolicyConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The cache policy does not exist.</p>
    NoSuchCachePolicy(String),
}

impl GetCachePolicyConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCachePolicyConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(GetCachePolicyConfigError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchCachePolicy" => {
                        return RusotoError::Service(GetCachePolicyConfigError::NoSuchCachePolicy(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetCachePolicyConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCachePolicyConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetCachePolicyConfigError::NoSuchCachePolicy(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCachePolicyConfigError {}
/// Errors returned by GetCloudFrontOriginAccessIdentity
#[derive(Debug, PartialEq)]
pub enum GetCloudFrontOriginAccessIdentityError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified origin access identity does not exist.</p>
    NoSuchCloudFrontOriginAccessIdentity(String),
}

impl GetCloudFrontOriginAccessIdentityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetCloudFrontOriginAccessIdentityError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "AccessDenied" => return RusotoError::Service(GetCloudFrontOriginAccessIdentityError::AccessDenied(parsed_error.message)),"NoSuchCloudFrontOriginAccessIdentity" => return RusotoError::Service(GetCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetCloudFrontOriginAccessIdentityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCloudFrontOriginAccessIdentityError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCloudFrontOriginAccessIdentityError {}
/// Errors returned by GetCloudFrontOriginAccessIdentityConfig
#[derive(Debug, PartialEq)]
pub enum GetCloudFrontOriginAccessIdentityConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified origin access identity does not exist.</p>
    NoSuchCloudFrontOriginAccessIdentity(String),
}

impl GetCloudFrontOriginAccessIdentityConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetCloudFrontOriginAccessIdentityConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "AccessDenied" => return RusotoError::Service(GetCloudFrontOriginAccessIdentityConfigError::AccessDenied(parsed_error.message)),"NoSuchCloudFrontOriginAccessIdentity" => return RusotoError::Service(GetCloudFrontOriginAccessIdentityConfigError::NoSuchCloudFrontOriginAccessIdentity(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetCloudFrontOriginAccessIdentityConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCloudFrontOriginAccessIdentityConfigError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCloudFrontOriginAccessIdentityConfigError::NoSuchCloudFrontOriginAccessIdentity(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCloudFrontOriginAccessIdentityConfigError {}
/// Errors returned by GetDistribution
#[derive(Debug, PartialEq)]
pub enum GetDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
}

impl GetDistributionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDistributionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(GetDistributionError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDistribution" => {
                        return RusotoError::Service(GetDistributionError::NoSuchDistribution(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetDistributionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDistributionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDistributionError::NoSuchDistribution(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDistributionError {}
/// Errors returned by GetDistributionConfig
#[derive(Debug, PartialEq)]
pub enum GetDistributionConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
}

impl GetDistributionConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDistributionConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(GetDistributionConfigError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDistribution" => {
                        return RusotoError::Service(
                            GetDistributionConfigError::NoSuchDistribution(parsed_error.message),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetDistributionConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDistributionConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDistributionConfigError::NoSuchDistribution(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDistributionConfigError {}
/// Errors returned by GetFieldLevelEncryption
#[derive(Debug, PartialEq)]
pub enum GetFieldLevelEncryptionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified configuration for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionConfig(String),
}

impl GetFieldLevelEncryptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFieldLevelEncryptionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(GetFieldLevelEncryptionError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchFieldLevelEncryptionConfig" => {
                        return RusotoError::Service(
                            GetFieldLevelEncryptionError::NoSuchFieldLevelEncryptionConfig(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetFieldLevelEncryptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFieldLevelEncryptionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetFieldLevelEncryptionError::NoSuchFieldLevelEncryptionConfig(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetFieldLevelEncryptionError {}
/// Errors returned by GetFieldLevelEncryptionConfig
#[derive(Debug, PartialEq)]
pub enum GetFieldLevelEncryptionConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified configuration for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionConfig(String),
}

impl GetFieldLevelEncryptionConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetFieldLevelEncryptionConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(
                            GetFieldLevelEncryptionConfigError::AccessDenied(parsed_error.message),
                        )
                    }
                    "NoSuchFieldLevelEncryptionConfig" => {
                        return RusotoError::Service(
                            GetFieldLevelEncryptionConfigError::NoSuchFieldLevelEncryptionConfig(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetFieldLevelEncryptionConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFieldLevelEncryptionConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetFieldLevelEncryptionConfigError::NoSuchFieldLevelEncryptionConfig(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetFieldLevelEncryptionConfigError {}
/// Errors returned by GetFieldLevelEncryptionProfile
#[derive(Debug, PartialEq)]
pub enum GetFieldLevelEncryptionProfileError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified profile for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionProfile(String),
}

impl GetFieldLevelEncryptionProfileError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetFieldLevelEncryptionProfileError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(
                            GetFieldLevelEncryptionProfileError::AccessDenied(parsed_error.message),
                        )
                    }
                    "NoSuchFieldLevelEncryptionProfile" => {
                        return RusotoError::Service(
                            GetFieldLevelEncryptionProfileError::NoSuchFieldLevelEncryptionProfile(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetFieldLevelEncryptionProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFieldLevelEncryptionProfileError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetFieldLevelEncryptionProfileError::NoSuchFieldLevelEncryptionProfile(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetFieldLevelEncryptionProfileError {}
/// Errors returned by GetFieldLevelEncryptionProfileConfig
#[derive(Debug, PartialEq)]
pub enum GetFieldLevelEncryptionProfileConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified profile for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionProfile(String),
}

impl GetFieldLevelEncryptionProfileConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetFieldLevelEncryptionProfileConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "AccessDenied" => return RusotoError::Service(GetFieldLevelEncryptionProfileConfigError::AccessDenied(parsed_error.message)),"NoSuchFieldLevelEncryptionProfile" => return RusotoError::Service(GetFieldLevelEncryptionProfileConfigError::NoSuchFieldLevelEncryptionProfile(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetFieldLevelEncryptionProfileConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFieldLevelEncryptionProfileConfigError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetFieldLevelEncryptionProfileConfigError::NoSuchFieldLevelEncryptionProfile(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFieldLevelEncryptionProfileConfigError {}
/// Errors returned by GetInvalidation
#[derive(Debug, PartialEq)]
pub enum GetInvalidationError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
    /// <p>The specified invalidation does not exist.</p>
    NoSuchInvalidation(String),
}

impl GetInvalidationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInvalidationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(GetInvalidationError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDistribution" => {
                        return RusotoError::Service(GetInvalidationError::NoSuchDistribution(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchInvalidation" => {
                        return RusotoError::Service(GetInvalidationError::NoSuchInvalidation(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetInvalidationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInvalidationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetInvalidationError::NoSuchDistribution(ref cause) => write!(f, "{}", cause),
            GetInvalidationError::NoSuchInvalidation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInvalidationError {}
/// Errors returned by GetKeyGroup
#[derive(Debug, PartialEq)]
pub enum GetKeyGroupError {
    /// <p>A resource that was specified is not valid.</p>
    NoSuchResource(String),
}

impl GetKeyGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetKeyGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "NoSuchResource" => {
                        return RusotoError::Service(GetKeyGroupError::NoSuchResource(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetKeyGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetKeyGroupError::NoSuchResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetKeyGroupError {}
/// Errors returned by GetKeyGroupConfig
#[derive(Debug, PartialEq)]
pub enum GetKeyGroupConfigError {
    /// <p>A resource that was specified is not valid.</p>
    NoSuchResource(String),
}

impl GetKeyGroupConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetKeyGroupConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "NoSuchResource" => {
                        return RusotoError::Service(GetKeyGroupConfigError::NoSuchResource(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetKeyGroupConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetKeyGroupConfigError::NoSuchResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetKeyGroupConfigError {}
/// Errors returned by GetMonitoringSubscription
#[derive(Debug, PartialEq)]
pub enum GetMonitoringSubscriptionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
}

impl GetMonitoringSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMonitoringSubscriptionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(GetMonitoringSubscriptionError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDistribution" => {
                        return RusotoError::Service(
                            GetMonitoringSubscriptionError::NoSuchDistribution(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetMonitoringSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMonitoringSubscriptionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetMonitoringSubscriptionError::NoSuchDistribution(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMonitoringSubscriptionError {}
/// Errors returned by GetOriginRequestPolicy
#[derive(Debug, PartialEq)]
pub enum GetOriginRequestPolicyError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The origin request policy does not exist.</p>
    NoSuchOriginRequestPolicy(String),
}

impl GetOriginRequestPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOriginRequestPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(GetOriginRequestPolicyError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchOriginRequestPolicy" => {
                        return RusotoError::Service(
                            GetOriginRequestPolicyError::NoSuchOriginRequestPolicy(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetOriginRequestPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOriginRequestPolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetOriginRequestPolicyError::NoSuchOriginRequestPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetOriginRequestPolicyError {}
/// Errors returned by GetOriginRequestPolicyConfig
#[derive(Debug, PartialEq)]
pub enum GetOriginRequestPolicyConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The origin request policy does not exist.</p>
    NoSuchOriginRequestPolicy(String),
}

impl GetOriginRequestPolicyConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetOriginRequestPolicyConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(
                            GetOriginRequestPolicyConfigError::AccessDenied(parsed_error.message),
                        )
                    }
                    "NoSuchOriginRequestPolicy" => {
                        return RusotoError::Service(
                            GetOriginRequestPolicyConfigError::NoSuchOriginRequestPolicy(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetOriginRequestPolicyConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOriginRequestPolicyConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetOriginRequestPolicyConfigError::NoSuchOriginRequestPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetOriginRequestPolicyConfigError {}
/// Errors returned by GetPublicKey
#[derive(Debug, PartialEq)]
pub enum GetPublicKeyError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified public key doesn't exist.</p>
    NoSuchPublicKey(String),
}

impl GetPublicKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPublicKeyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(GetPublicKeyError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchPublicKey" => {
                        return RusotoError::Service(GetPublicKeyError::NoSuchPublicKey(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetPublicKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPublicKeyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::NoSuchPublicKey(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPublicKeyError {}
/// Errors returned by GetPublicKeyConfig
#[derive(Debug, PartialEq)]
pub enum GetPublicKeyConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified public key doesn't exist.</p>
    NoSuchPublicKey(String),
}

impl GetPublicKeyConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPublicKeyConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(GetPublicKeyConfigError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchPublicKey" => {
                        return RusotoError::Service(GetPublicKeyConfigError::NoSuchPublicKey(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetPublicKeyConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPublicKeyConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetPublicKeyConfigError::NoSuchPublicKey(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPublicKeyConfigError {}
/// Errors returned by GetRealtimeLogConfig
#[derive(Debug, PartialEq)]
pub enum GetRealtimeLogConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The real-time log configuration does not exist.</p>
    NoSuchRealtimeLogConfig(String),
}

impl GetRealtimeLogConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRealtimeLogConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(GetRealtimeLogConfigError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(GetRealtimeLogConfigError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchRealtimeLogConfig" => {
                        return RusotoError::Service(
                            GetRealtimeLogConfigError::NoSuchRealtimeLogConfig(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetRealtimeLogConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRealtimeLogConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRealtimeLogConfigError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            GetRealtimeLogConfigError::NoSuchRealtimeLogConfig(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRealtimeLogConfigError {}
/// Errors returned by GetStreamingDistribution
#[derive(Debug, PartialEq)]
pub enum GetStreamingDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified streaming distribution does not exist.</p>
    NoSuchStreamingDistribution(String),
}

impl GetStreamingDistributionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetStreamingDistributionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(GetStreamingDistributionError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchStreamingDistribution" => {
                        return RusotoError::Service(
                            GetStreamingDistributionError::NoSuchStreamingDistribution(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetStreamingDistributionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetStreamingDistributionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetStreamingDistributionError::NoSuchStreamingDistribution(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetStreamingDistributionError {}
/// Errors returned by GetStreamingDistributionConfig
#[derive(Debug, PartialEq)]
pub enum GetStreamingDistributionConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified streaming distribution does not exist.</p>
    NoSuchStreamingDistribution(String),
}

impl GetStreamingDistributionConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetStreamingDistributionConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(
                            GetStreamingDistributionConfigError::AccessDenied(parsed_error.message),
                        )
                    }
                    "NoSuchStreamingDistribution" => {
                        return RusotoError::Service(
                            GetStreamingDistributionConfigError::NoSuchStreamingDistribution(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetStreamingDistributionConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetStreamingDistributionConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetStreamingDistributionConfigError::NoSuchStreamingDistribution(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetStreamingDistributionConfigError {}
/// Errors returned by ListCachePolicies
#[derive(Debug, PartialEq)]
pub enum ListCachePoliciesError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The cache policy does not exist.</p>
    NoSuchCachePolicy(String),
}

impl ListCachePoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCachePoliciesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(ListCachePoliciesError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(ListCachePoliciesError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchCachePolicy" => {
                        return RusotoError::Service(ListCachePoliciesError::NoSuchCachePolicy(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListCachePoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListCachePoliciesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListCachePoliciesError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListCachePoliciesError::NoSuchCachePolicy(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListCachePoliciesError {}
/// Errors returned by ListCloudFrontOriginAccessIdentities
#[derive(Debug, PartialEq)]
pub enum ListCloudFrontOriginAccessIdentitiesError {
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
}

impl ListCloudFrontOriginAccessIdentitiesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListCloudFrontOriginAccessIdentitiesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            ListCloudFrontOriginAccessIdentitiesError::InvalidArgument(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListCloudFrontOriginAccessIdentitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListCloudFrontOriginAccessIdentitiesError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListCloudFrontOriginAccessIdentitiesError {}
/// Errors returned by ListDistributions
#[derive(Debug, PartialEq)]
pub enum ListDistributionsError {
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
}

impl ListDistributionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDistributionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidArgument" => {
                        return RusotoError::Service(ListDistributionsError::InvalidArgument(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListDistributionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDistributionsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDistributionsError {}
/// Errors returned by ListDistributionsByCachePolicyId
#[derive(Debug, PartialEq)]
pub enum ListDistributionsByCachePolicyIdError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The cache policy does not exist.</p>
    NoSuchCachePolicy(String),
}

impl ListDistributionsByCachePolicyIdError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDistributionsByCachePolicyIdError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(
                            ListDistributionsByCachePolicyIdError::AccessDenied(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            ListDistributionsByCachePolicyIdError::InvalidArgument(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchCachePolicy" => {
                        return RusotoError::Service(
                            ListDistributionsByCachePolicyIdError::NoSuchCachePolicy(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListDistributionsByCachePolicyIdError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDistributionsByCachePolicyIdError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDistributionsByCachePolicyIdError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDistributionsByCachePolicyIdError::NoSuchCachePolicy(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDistributionsByCachePolicyIdError {}
/// Errors returned by ListDistributionsByKeyGroup
#[derive(Debug, PartialEq)]
pub enum ListDistributionsByKeyGroupError {
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>A resource that was specified is not valid.</p>
    NoSuchResource(String),
}

impl ListDistributionsByKeyGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDistributionsByKeyGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            ListDistributionsByKeyGroupError::InvalidArgument(parsed_error.message),
                        )
                    }
                    "NoSuchResource" => {
                        return RusotoError::Service(
                            ListDistributionsByKeyGroupError::NoSuchResource(parsed_error.message),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListDistributionsByKeyGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDistributionsByKeyGroupError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListDistributionsByKeyGroupError::NoSuchResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDistributionsByKeyGroupError {}
/// Errors returned by ListDistributionsByOriginRequestPolicyId
#[derive(Debug, PartialEq)]
pub enum ListDistributionsByOriginRequestPolicyIdError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The origin request policy does not exist.</p>
    NoSuchOriginRequestPolicy(String),
}

impl ListDistributionsByOriginRequestPolicyIdError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDistributionsByOriginRequestPolicyIdError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(
                            ListDistributionsByOriginRequestPolicyIdError::AccessDenied(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            ListDistributionsByOriginRequestPolicyIdError::InvalidArgument(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchOriginRequestPolicy" => return RusotoError::Service(
                        ListDistributionsByOriginRequestPolicyIdError::NoSuchOriginRequestPolicy(
                            parsed_error.message,
                        ),
                    ),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListDistributionsByOriginRequestPolicyIdError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDistributionsByOriginRequestPolicyIdError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDistributionsByOriginRequestPolicyIdError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDistributionsByOriginRequestPolicyIdError::NoSuchOriginRequestPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDistributionsByOriginRequestPolicyIdError {}
/// Errors returned by ListDistributionsByRealtimeLogConfig
#[derive(Debug, PartialEq)]
pub enum ListDistributionsByRealtimeLogConfigError {
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
}

impl ListDistributionsByRealtimeLogConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDistributionsByRealtimeLogConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            ListDistributionsByRealtimeLogConfigError::InvalidArgument(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListDistributionsByRealtimeLogConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDistributionsByRealtimeLogConfigError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDistributionsByRealtimeLogConfigError {}
/// Errors returned by ListDistributionsByWebACLId
#[derive(Debug, PartialEq)]
pub enum ListDistributionsByWebACLIdError {
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>A web ACL ID specified is not valid. To specify a web ACL created using the latest version of AWS WAF, use the ACL ARN, for example <code>arn:aws:wafv2:us-east-1:123456789012:global/webacl/ExampleWebACL/473e64fd-f30b-4765-81a0-62ad96dd167a</code>. To specify a web ACL created using AWS WAF Classic, use the ACL ID, for example <code>473e64fd-f30b-4765-81a0-62ad96dd167a</code>.</p>
    InvalidWebACLId(String),
}

impl ListDistributionsByWebACLIdError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDistributionsByWebACLIdError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            ListDistributionsByWebACLIdError::InvalidArgument(parsed_error.message),
                        )
                    }
                    "InvalidWebACLId" => {
                        return RusotoError::Service(
                            ListDistributionsByWebACLIdError::InvalidWebACLId(parsed_error.message),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListDistributionsByWebACLIdError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDistributionsByWebACLIdError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListDistributionsByWebACLIdError::InvalidWebACLId(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDistributionsByWebACLIdError {}
/// Errors returned by ListFieldLevelEncryptionConfigs
#[derive(Debug, PartialEq)]
pub enum ListFieldLevelEncryptionConfigsError {
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
}

impl ListFieldLevelEncryptionConfigsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListFieldLevelEncryptionConfigsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            ListFieldLevelEncryptionConfigsError::InvalidArgument(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListFieldLevelEncryptionConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFieldLevelEncryptionConfigsError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListFieldLevelEncryptionConfigsError {}
/// Errors returned by ListFieldLevelEncryptionProfiles
#[derive(Debug, PartialEq)]
pub enum ListFieldLevelEncryptionProfilesError {
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
}

impl ListFieldLevelEncryptionProfilesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListFieldLevelEncryptionProfilesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            ListFieldLevelEncryptionProfilesError::InvalidArgument(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListFieldLevelEncryptionProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFieldLevelEncryptionProfilesError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListFieldLevelEncryptionProfilesError {}
/// Errors returned by ListInvalidations
#[derive(Debug, PartialEq)]
pub enum ListInvalidationsError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
}

impl ListInvalidationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInvalidationsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(ListInvalidationsError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(ListInvalidationsError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDistribution" => {
                        return RusotoError::Service(ListInvalidationsError::NoSuchDistribution(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListInvalidationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInvalidationsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListInvalidationsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListInvalidationsError::NoSuchDistribution(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInvalidationsError {}
/// Errors returned by ListKeyGroups
#[derive(Debug, PartialEq)]
pub enum ListKeyGroupsError {
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
}

impl ListKeyGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListKeyGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidArgument" => {
                        return RusotoError::Service(ListKeyGroupsError::InvalidArgument(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListKeyGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListKeyGroupsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListKeyGroupsError {}
/// Errors returned by ListOriginRequestPolicies
#[derive(Debug, PartialEq)]
pub enum ListOriginRequestPoliciesError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The origin request policy does not exist.</p>
    NoSuchOriginRequestPolicy(String),
}

impl ListOriginRequestPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOriginRequestPoliciesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(ListOriginRequestPoliciesError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            ListOriginRequestPoliciesError::InvalidArgument(parsed_error.message),
                        )
                    }
                    "NoSuchOriginRequestPolicy" => {
                        return RusotoError::Service(
                            ListOriginRequestPoliciesError::NoSuchOriginRequestPolicy(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListOriginRequestPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOriginRequestPoliciesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListOriginRequestPoliciesError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListOriginRequestPoliciesError::NoSuchOriginRequestPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListOriginRequestPoliciesError {}
/// Errors returned by ListPublicKeys
#[derive(Debug, PartialEq)]
pub enum ListPublicKeysError {
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
}

impl ListPublicKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPublicKeysError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidArgument" => {
                        return RusotoError::Service(ListPublicKeysError::InvalidArgument(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListPublicKeysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPublicKeysError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPublicKeysError {}
/// Errors returned by ListRealtimeLogConfigs
#[derive(Debug, PartialEq)]
pub enum ListRealtimeLogConfigsError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The real-time log configuration does not exist.</p>
    NoSuchRealtimeLogConfig(String),
}

impl ListRealtimeLogConfigsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRealtimeLogConfigsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(ListRealtimeLogConfigsError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(ListRealtimeLogConfigsError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchRealtimeLogConfig" => {
                        return RusotoError::Service(
                            ListRealtimeLogConfigsError::NoSuchRealtimeLogConfig(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListRealtimeLogConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRealtimeLogConfigsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListRealtimeLogConfigsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListRealtimeLogConfigsError::NoSuchRealtimeLogConfig(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListRealtimeLogConfigsError {}
/// Errors returned by ListStreamingDistributions
#[derive(Debug, PartialEq)]
pub enum ListStreamingDistributionsError {
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
}

impl ListStreamingDistributionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListStreamingDistributionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            ListStreamingDistributionsError::InvalidArgument(parsed_error.message),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListStreamingDistributionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListStreamingDistributionsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListStreamingDistributionsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The tagging specified is not valid.</p>
    InvalidTagging(String),
    /// <p>A resource that was specified is not valid.</p>
    NoSuchResource(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(ListTagsForResourceError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(ListTagsForResourceError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "InvalidTagging" => {
                        return RusotoError::Service(ListTagsForResourceError::InvalidTagging(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchResource" => {
                        return RusotoError::Service(ListTagsForResourceError::NoSuchResource(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidTagging(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NoSuchResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The tagging specified is not valid.</p>
    InvalidTagging(String),
    /// <p>A resource that was specified is not valid.</p>
    NoSuchResource(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(TagResourceError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(TagResourceError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "InvalidTagging" => {
                        return RusotoError::Service(TagResourceError::InvalidTagging(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchResource" => {
                        return RusotoError::Service(TagResourceError::NoSuchResource(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidTagging(ref cause) => write!(f, "{}", cause),
            TagResourceError::NoSuchResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The tagging specified is not valid.</p>
    InvalidTagging(String),
    /// <p>A resource that was specified is not valid.</p>
    NoSuchResource(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(UntagResourceError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(UntagResourceError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "InvalidTagging" => {
                        return RusotoError::Service(UntagResourceError::InvalidTagging(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchResource" => {
                        return RusotoError::Service(UntagResourceError::NoSuchResource(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidTagging(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NoSuchResource(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateCachePolicy
#[derive(Debug, PartialEq)]
pub enum UpdateCachePolicyError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>A cache policy with this name already exists. You must provide a unique name. To modify an existing cache policy, use <code>UpdateCachePolicy</code>.</p>
    CachePolicyAlreadyExists(String),
    /// <p>The update contains modifications that are not allowed.</p>
    IllegalUpdate(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The cache policy does not exist.</p>
    NoSuchCachePolicy(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
    /// <p>The number of cookies in the cache policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyCookiesInCachePolicy(String),
    /// <p>The number of headers in the cache policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyHeadersInCachePolicy(String),
    /// <p>The number of query strings in the cache policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyQueryStringsInCachePolicy(String),
}

impl UpdateCachePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCachePolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(UpdateCachePolicyError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "CachePolicyAlreadyExists" => {
                        return RusotoError::Service(
                            UpdateCachePolicyError::CachePolicyAlreadyExists(parsed_error.message),
                        )
                    }
                    "IllegalUpdate" => {
                        return RusotoError::Service(UpdateCachePolicyError::IllegalUpdate(
                            parsed_error.message,
                        ))
                    }
                    "InconsistentQuantities" => {
                        return RusotoError::Service(
                            UpdateCachePolicyError::InconsistentQuantities(parsed_error.message),
                        )
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(UpdateCachePolicyError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "InvalidIfMatchVersion" => {
                        return RusotoError::Service(UpdateCachePolicyError::InvalidIfMatchVersion(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchCachePolicy" => {
                        return RusotoError::Service(UpdateCachePolicyError::NoSuchCachePolicy(
                            parsed_error.message,
                        ))
                    }
                    "PreconditionFailed" => {
                        return RusotoError::Service(UpdateCachePolicyError::PreconditionFailed(
                            parsed_error.message,
                        ))
                    }
                    "TooManyCookiesInCachePolicy" => {
                        return RusotoError::Service(
                            UpdateCachePolicyError::TooManyCookiesInCachePolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyHeadersInCachePolicy" => {
                        return RusotoError::Service(
                            UpdateCachePolicyError::TooManyHeadersInCachePolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyQueryStringsInCachePolicy" => {
                        return RusotoError::Service(
                            UpdateCachePolicyError::TooManyQueryStringsInCachePolicy(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateCachePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateCachePolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateCachePolicyError::CachePolicyAlreadyExists(ref cause) => write!(f, "{}", cause),
            UpdateCachePolicyError::IllegalUpdate(ref cause) => write!(f, "{}", cause),
            UpdateCachePolicyError::InconsistentQuantities(ref cause) => write!(f, "{}", cause),
            UpdateCachePolicyError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateCachePolicyError::InvalidIfMatchVersion(ref cause) => write!(f, "{}", cause),
            UpdateCachePolicyError::NoSuchCachePolicy(ref cause) => write!(f, "{}", cause),
            UpdateCachePolicyError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
            UpdateCachePolicyError::TooManyCookiesInCachePolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCachePolicyError::TooManyHeadersInCachePolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCachePolicyError::TooManyQueryStringsInCachePolicy(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateCachePolicyError {}
/// Errors returned by UpdateCloudFrontOriginAccessIdentity
#[derive(Debug, PartialEq)]
pub enum UpdateCloudFrontOriginAccessIdentityError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The update contains modifications that are not allowed.</p>
    IllegalUpdate(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>The specified origin access identity does not exist.</p>
    NoSuchCloudFrontOriginAccessIdentity(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
}

impl UpdateCloudFrontOriginAccessIdentityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateCloudFrontOriginAccessIdentityError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "AccessDenied" => return RusotoError::Service(UpdateCloudFrontOriginAccessIdentityError::AccessDenied(parsed_error.message)),"IllegalUpdate" => return RusotoError::Service(UpdateCloudFrontOriginAccessIdentityError::IllegalUpdate(parsed_error.message)),"InconsistentQuantities" => return RusotoError::Service(UpdateCloudFrontOriginAccessIdentityError::InconsistentQuantities(parsed_error.message)),"InvalidArgument" => return RusotoError::Service(UpdateCloudFrontOriginAccessIdentityError::InvalidArgument(parsed_error.message)),"InvalidIfMatchVersion" => return RusotoError::Service(UpdateCloudFrontOriginAccessIdentityError::InvalidIfMatchVersion(parsed_error.message)),"MissingBody" => return RusotoError::Service(UpdateCloudFrontOriginAccessIdentityError::MissingBody(parsed_error.message)),"NoSuchCloudFrontOriginAccessIdentity" => return RusotoError::Service(UpdateCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity(parsed_error.message)),"PreconditionFailed" => return RusotoError::Service(UpdateCloudFrontOriginAccessIdentityError::PreconditionFailed(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateCloudFrontOriginAccessIdentityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateCloudFrontOriginAccessIdentityError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCloudFrontOriginAccessIdentityError::IllegalUpdate(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCloudFrontOriginAccessIdentityError::InconsistentQuantities(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCloudFrontOriginAccessIdentityError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCloudFrontOriginAccessIdentityError::InvalidIfMatchVersion(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCloudFrontOriginAccessIdentityError::MissingBody(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity(
                ref cause,
            ) => write!(f, "{}", cause),
            UpdateCloudFrontOriginAccessIdentityError::PreconditionFailed(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateCloudFrontOriginAccessIdentityError {}
/// Errors returned by UpdateDistribution
#[derive(Debug, PartialEq)]
pub enum UpdateDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The CNAME specified is already defined for CloudFront.</p>
    CNAMEAlreadyExists(String),
    /// <p>The specified configuration for field-level encryption can't be associated with the specified cache behavior.</p>
    IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(String),
    /// <p>The update contains modifications that are not allowed.</p>
    IllegalUpdate(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The default root object file name is too big or contains an invalid character.</p>
    InvalidDefaultRootObject(String),
    /// <p>An invalid error code was specified.</p>
    InvalidErrorCode(String),
    /// <p>Your request contains forward cookies option which doesn't match with the expectation for the <code>whitelisted</code> list of cookie names. Either list of cookie names has been specified when not allowed or list of cookie names is missing when expected.</p>
    InvalidForwardCookies(String),
    /// <p>The specified geo restriction parameter is not valid.</p>
    InvalidGeoRestrictionParameter(String),
    /// <p>The headers specified are not valid for an Amazon S3 origin.</p>
    InvalidHeadersForS3Origin(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified Lambda function association is invalid.</p>
    InvalidLambdaFunctionAssociation(String),
    /// <p>The location code specified is not valid.</p>
    InvalidLocationCode(String),
    /// <p>The minimum protocol version specified is not valid.</p>
    InvalidMinimumProtocolVersion(String),
    /// <p>The origin access identity is not valid or doesn't exist.</p>
    InvalidOriginAccessIdentity(String),
    /// <p>The keep alive timeout specified for the origin is not valid.</p>
    InvalidOriginKeepaliveTimeout(String),
    /// <p>The read timeout specified for the origin is not valid.</p>
    InvalidOriginReadTimeout(String),
    /// <p>The query string parameters specified are not valid.</p>
    InvalidQueryStringParameters(String),
    /// <p>The relative path is too big, is not URL-encoded, or does not begin with a slash (/).</p>
    InvalidRelativePath(String),
    /// <p>This operation requires the HTTPS protocol. Ensure that you specify the HTTPS protocol in your request, or omit the <code>RequiredProtocols</code> element from your distribution configuration.</p>
    InvalidRequiredProtocol(String),
    /// <p>A response code is not valid.</p>
    InvalidResponseCode(String),
    /// <p>The TTL order specified is not valid.</p>
    InvalidTTLOrder(String),
    /// <p>A viewer certificate specified is not valid.</p>
    InvalidViewerCertificate(String),
    /// <p>A web ACL ID specified is not valid. To specify a web ACL created using the latest version of AWS WAF, use the ACL ARN, for example <code>arn:aws:wafv2:us-east-1:123456789012:global/webacl/ExampleWebACL/473e64fd-f30b-4765-81a0-62ad96dd167a</code>. To specify a web ACL created using AWS WAF Classic, use the ACL ID, for example <code>473e64fd-f30b-4765-81a0-62ad96dd167a</code>.</p>
    InvalidWebACLId(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>The cache policy does not exist.</p>
    NoSuchCachePolicy(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
    /// <p>The specified configuration for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionConfig(String),
    /// <p>No origin exists with the specified <code>Origin Id</code>. </p>
    NoSuchOrigin(String),
    /// <p>The origin request policy does not exist.</p>
    NoSuchOriginRequestPolicy(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
    /// <p>You cannot create more cache behaviors for the distribution.</p>
    TooManyCacheBehaviors(String),
    /// <p>You cannot create anymore custom SSL/TLS certificates.</p>
    TooManyCertificates(String),
    /// <p>Your request contains more cookie names in the whitelist than are allowed per cache behavior.</p>
    TooManyCookieNamesInWhiteList(String),
    /// <p>Your request contains more CNAMEs than are allowed per distribution.</p>
    TooManyDistributionCNAMEs(String),
    /// <p>The maximum number of distributions have been associated with the specified cache policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyDistributionsAssociatedToCachePolicy(String),
    /// <p>The maximum number of distributions have been associated with the specified configuration for field-level encryption.</p>
    TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(String),
    /// <p>The number of distributions that reference this key group is more than the maximum allowed. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyDistributionsAssociatedToKeyGroup(String),
    /// <p>The maximum number of distributions have been associated with the specified origin request policy. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyDistributionsAssociatedToOriginRequestPolicy(String),
    /// <p>Processing your request would cause the maximum number of distributions with Lambda function associations per owner to be exceeded.</p>
    TooManyDistributionsWithLambdaAssociations(String),
    /// <p>The maximum number of distributions have been associated with the specified Lambda function.</p>
    TooManyDistributionsWithSingleFunctionARN(String),
    /// <p>Your request contains too many headers in forwarded values.</p>
    TooManyHeadersInForwardedValues(String),
    /// <p>The number of key groups referenced by this distribution is more than the maximum allowed. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyKeyGroupsAssociatedToDistribution(String),
    /// <p>Your request contains more Lambda function associations than are allowed per distribution.</p>
    TooManyLambdaFunctionAssociations(String),
    /// <p>Your request contains too many origin custom headers.</p>
    TooManyOriginCustomHeaders(String),
    /// <p>Processing your request would cause you to exceed the maximum number of origin groups allowed.</p>
    TooManyOriginGroupsPerDistribution(String),
    /// <p>You cannot create more origins for the distribution.</p>
    TooManyOrigins(String),
    /// <p>Your request contains too many query string parameters.</p>
    TooManyQueryStringParameters(String),
    /// <p>Your request contains more trusted signers than are allowed per distribution.</p>
    TooManyTrustedSigners(String),
    /// <p>The specified key group does not exist.</p>
    TrustedKeyGroupDoesNotExist(String),
    /// <p>One or more of your trusted signers don't exist.</p>
    TrustedSignerDoesNotExist(String),
}

impl UpdateDistributionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDistributionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "AccessDenied" => return RusotoError::Service(UpdateDistributionError::AccessDenied(parsed_error.message)),"CNAMEAlreadyExists" => return RusotoError::Service(UpdateDistributionError::CNAMEAlreadyExists(parsed_error.message)),"IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior" => return RusotoError::Service(UpdateDistributionError::IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(parsed_error.message)),"IllegalUpdate" => return RusotoError::Service(UpdateDistributionError::IllegalUpdate(parsed_error.message)),"InconsistentQuantities" => return RusotoError::Service(UpdateDistributionError::InconsistentQuantities(parsed_error.message)),"InvalidArgument" => return RusotoError::Service(UpdateDistributionError::InvalidArgument(parsed_error.message)),"InvalidDefaultRootObject" => return RusotoError::Service(UpdateDistributionError::InvalidDefaultRootObject(parsed_error.message)),"InvalidErrorCode" => return RusotoError::Service(UpdateDistributionError::InvalidErrorCode(parsed_error.message)),"InvalidForwardCookies" => return RusotoError::Service(UpdateDistributionError::InvalidForwardCookies(parsed_error.message)),"InvalidGeoRestrictionParameter" => return RusotoError::Service(UpdateDistributionError::InvalidGeoRestrictionParameter(parsed_error.message)),"InvalidHeadersForS3Origin" => return RusotoError::Service(UpdateDistributionError::InvalidHeadersForS3Origin(parsed_error.message)),"InvalidIfMatchVersion" => return RusotoError::Service(UpdateDistributionError::InvalidIfMatchVersion(parsed_error.message)),"InvalidLambdaFunctionAssociation" => return RusotoError::Service(UpdateDistributionError::InvalidLambdaFunctionAssociation(parsed_error.message)),"InvalidLocationCode" => return RusotoError::Service(UpdateDistributionError::InvalidLocationCode(parsed_error.message)),"InvalidMinimumProtocolVersion" => return RusotoError::Service(UpdateDistributionError::InvalidMinimumProtocolVersion(parsed_error.message)),"InvalidOriginAccessIdentity" => return RusotoError::Service(UpdateDistributionError::InvalidOriginAccessIdentity(parsed_error.message)),"InvalidOriginKeepaliveTimeout" => return RusotoError::Service(UpdateDistributionError::InvalidOriginKeepaliveTimeout(parsed_error.message)),"InvalidOriginReadTimeout" => return RusotoError::Service(UpdateDistributionError::InvalidOriginReadTimeout(parsed_error.message)),"InvalidQueryStringParameters" => return RusotoError::Service(UpdateDistributionError::InvalidQueryStringParameters(parsed_error.message)),"InvalidRelativePath" => return RusotoError::Service(UpdateDistributionError::InvalidRelativePath(parsed_error.message)),"InvalidRequiredProtocol" => return RusotoError::Service(UpdateDistributionError::InvalidRequiredProtocol(parsed_error.message)),"InvalidResponseCode" => return RusotoError::Service(UpdateDistributionError::InvalidResponseCode(parsed_error.message)),"InvalidTTLOrder" => return RusotoError::Service(UpdateDistributionError::InvalidTTLOrder(parsed_error.message)),"InvalidViewerCertificate" => return RusotoError::Service(UpdateDistributionError::InvalidViewerCertificate(parsed_error.message)),"InvalidWebACLId" => return RusotoError::Service(UpdateDistributionError::InvalidWebACLId(parsed_error.message)),"MissingBody" => return RusotoError::Service(UpdateDistributionError::MissingBody(parsed_error.message)),"NoSuchCachePolicy" => return RusotoError::Service(UpdateDistributionError::NoSuchCachePolicy(parsed_error.message)),"NoSuchDistribution" => return RusotoError::Service(UpdateDistributionError::NoSuchDistribution(parsed_error.message)),"NoSuchFieldLevelEncryptionConfig" => return RusotoError::Service(UpdateDistributionError::NoSuchFieldLevelEncryptionConfig(parsed_error.message)),"NoSuchOrigin" => return RusotoError::Service(UpdateDistributionError::NoSuchOrigin(parsed_error.message)),"NoSuchOriginRequestPolicy" => return RusotoError::Service(UpdateDistributionError::NoSuchOriginRequestPolicy(parsed_error.message)),"PreconditionFailed" => return RusotoError::Service(UpdateDistributionError::PreconditionFailed(parsed_error.message)),"TooManyCacheBehaviors" => return RusotoError::Service(UpdateDistributionError::TooManyCacheBehaviors(parsed_error.message)),"TooManyCertificates" => return RusotoError::Service(UpdateDistributionError::TooManyCertificates(parsed_error.message)),"TooManyCookieNamesInWhiteList" => return RusotoError::Service(UpdateDistributionError::TooManyCookieNamesInWhiteList(parsed_error.message)),"TooManyDistributionCNAMEs" => return RusotoError::Service(UpdateDistributionError::TooManyDistributionCNAMEs(parsed_error.message)),"TooManyDistributionsAssociatedToCachePolicy" => return RusotoError::Service(UpdateDistributionError::TooManyDistributionsAssociatedToCachePolicy(parsed_error.message)),"TooManyDistributionsAssociatedToFieldLevelEncryptionConfig" => return RusotoError::Service(UpdateDistributionError::TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(parsed_error.message)),"TooManyDistributionsAssociatedToKeyGroup" => return RusotoError::Service(UpdateDistributionError::TooManyDistributionsAssociatedToKeyGroup(parsed_error.message)),"TooManyDistributionsAssociatedToOriginRequestPolicy" => return RusotoError::Service(UpdateDistributionError::TooManyDistributionsAssociatedToOriginRequestPolicy(parsed_error.message)),"TooManyDistributionsWithLambdaAssociations" => return RusotoError::Service(UpdateDistributionError::TooManyDistributionsWithLambdaAssociations(parsed_error.message)),"TooManyDistributionsWithSingleFunctionARN" => return RusotoError::Service(UpdateDistributionError::TooManyDistributionsWithSingleFunctionARN(parsed_error.message)),"TooManyHeadersInForwardedValues" => return RusotoError::Service(UpdateDistributionError::TooManyHeadersInForwardedValues(parsed_error.message)),"TooManyKeyGroupsAssociatedToDistribution" => return RusotoError::Service(UpdateDistributionError::TooManyKeyGroupsAssociatedToDistribution(parsed_error.message)),"TooManyLambdaFunctionAssociations" => return RusotoError::Service(UpdateDistributionError::TooManyLambdaFunctionAssociations(parsed_error.message)),"TooManyOriginCustomHeaders" => return RusotoError::Service(UpdateDistributionError::TooManyOriginCustomHeaders(parsed_error.message)),"TooManyOriginGroupsPerDistribution" => return RusotoError::Service(UpdateDistributionError::TooManyOriginGroupsPerDistribution(parsed_error.message)),"TooManyOrigins" => return RusotoError::Service(UpdateDistributionError::TooManyOrigins(parsed_error.message)),"TooManyQueryStringParameters" => return RusotoError::Service(UpdateDistributionError::TooManyQueryStringParameters(parsed_error.message)),"TooManyTrustedSigners" => return RusotoError::Service(UpdateDistributionError::TooManyTrustedSigners(parsed_error.message)),"TrustedKeyGroupDoesNotExist" => return RusotoError::Service(UpdateDistributionError::TrustedKeyGroupDoesNotExist(parsed_error.message)),"TrustedSignerDoesNotExist" => return RusotoError::Service(UpdateDistributionError::TrustedSignerDoesNotExist(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateDistributionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            UpdateDistributionError::AccessDenied(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::CNAMEAlreadyExists(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::IllegalUpdate(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InconsistentQuantities(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidArgument(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidDefaultRootObject(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidErrorCode(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidForwardCookies(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidGeoRestrictionParameter(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidHeadersForS3Origin(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidIfMatchVersion(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidLambdaFunctionAssociation(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidLocationCode(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidMinimumProtocolVersion(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidOriginAccessIdentity(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidOriginKeepaliveTimeout(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidOriginReadTimeout(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidQueryStringParameters(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidRelativePath(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidRequiredProtocol(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidResponseCode(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidTTLOrder(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidViewerCertificate(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::InvalidWebACLId(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::MissingBody(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::NoSuchCachePolicy(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::NoSuchDistribution(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::NoSuchFieldLevelEncryptionConfig(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::NoSuchOrigin(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::NoSuchOriginRequestPolicy(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyCacheBehaviors(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyCertificates(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyCookieNamesInWhiteList(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyDistributionCNAMEs(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyDistributionsAssociatedToCachePolicy(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyDistributionsAssociatedToKeyGroup(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyDistributionsAssociatedToOriginRequestPolicy(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyDistributionsWithLambdaAssociations(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyDistributionsWithSingleFunctionARN(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyHeadersInForwardedValues(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyKeyGroupsAssociatedToDistribution(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyLambdaFunctionAssociations(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyOriginCustomHeaders(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyOriginGroupsPerDistribution(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyOrigins(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyQueryStringParameters(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyTrustedSigners(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TrustedKeyGroupDoesNotExist(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TrustedSignerDoesNotExist(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for UpdateDistributionError {}
/// Errors returned by UpdateFieldLevelEncryptionConfig
#[derive(Debug, PartialEq)]
pub enum UpdateFieldLevelEncryptionConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The update contains modifications that are not allowed.</p>
    IllegalUpdate(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified configuration for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionConfig(String),
    /// <p>The specified profile for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionProfile(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
    /// <p>No profile specified for the field-level encryption query argument.</p>
    QueryArgProfileEmpty(String),
    /// <p>The maximum number of content type profiles for field-level encryption have been created.</p>
    TooManyFieldLevelEncryptionContentTypeProfiles(String),
    /// <p>The maximum number of query arg profiles for field-level encryption have been created.</p>
    TooManyFieldLevelEncryptionQueryArgProfiles(String),
}

impl UpdateFieldLevelEncryptionConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateFieldLevelEncryptionConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "AccessDenied" => return RusotoError::Service(UpdateFieldLevelEncryptionConfigError::AccessDenied(parsed_error.message)),"IllegalUpdate" => return RusotoError::Service(UpdateFieldLevelEncryptionConfigError::IllegalUpdate(parsed_error.message)),"InconsistentQuantities" => return RusotoError::Service(UpdateFieldLevelEncryptionConfigError::InconsistentQuantities(parsed_error.message)),"InvalidArgument" => return RusotoError::Service(UpdateFieldLevelEncryptionConfigError::InvalidArgument(parsed_error.message)),"InvalidIfMatchVersion" => return RusotoError::Service(UpdateFieldLevelEncryptionConfigError::InvalidIfMatchVersion(parsed_error.message)),"NoSuchFieldLevelEncryptionConfig" => return RusotoError::Service(UpdateFieldLevelEncryptionConfigError::NoSuchFieldLevelEncryptionConfig(parsed_error.message)),"NoSuchFieldLevelEncryptionProfile" => return RusotoError::Service(UpdateFieldLevelEncryptionConfigError::NoSuchFieldLevelEncryptionProfile(parsed_error.message)),"PreconditionFailed" => return RusotoError::Service(UpdateFieldLevelEncryptionConfigError::PreconditionFailed(parsed_error.message)),"QueryArgProfileEmpty" => return RusotoError::Service(UpdateFieldLevelEncryptionConfigError::QueryArgProfileEmpty(parsed_error.message)),"TooManyFieldLevelEncryptionContentTypeProfiles" => return RusotoError::Service(UpdateFieldLevelEncryptionConfigError::TooManyFieldLevelEncryptionContentTypeProfiles(parsed_error.message)),"TooManyFieldLevelEncryptionQueryArgProfiles" => return RusotoError::Service(UpdateFieldLevelEncryptionConfigError::TooManyFieldLevelEncryptionQueryArgProfiles(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateFieldLevelEncryptionConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            UpdateFieldLevelEncryptionConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionConfigError::IllegalUpdate(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionConfigError::InconsistentQuantities(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionConfigError::InvalidArgument(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionConfigError::InvalidIfMatchVersion(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionConfigError::NoSuchFieldLevelEncryptionConfig(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionConfigError::NoSuchFieldLevelEncryptionProfile(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionConfigError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionConfigError::QueryArgProfileEmpty(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionConfigError::TooManyFieldLevelEncryptionContentTypeProfiles(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionConfigError::TooManyFieldLevelEncryptionQueryArgProfiles(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for UpdateFieldLevelEncryptionConfigError {}
/// Errors returned by UpdateFieldLevelEncryptionProfile
#[derive(Debug, PartialEq)]
pub enum UpdateFieldLevelEncryptionProfileError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified profile for field-level encryption already exists.</p>
    FieldLevelEncryptionProfileAlreadyExists(String),
    /// <p>The maximum size of a profile for field-level encryption was exceeded.</p>
    FieldLevelEncryptionProfileSizeExceeded(String),
    /// <p>The update contains modifications that are not allowed.</p>
    IllegalUpdate(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified profile for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionProfile(String),
    /// <p>The specified public key doesn't exist.</p>
    NoSuchPublicKey(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
    /// <p>The maximum number of encryption entities for field-level encryption have been created.</p>
    TooManyFieldLevelEncryptionEncryptionEntities(String),
    /// <p>The maximum number of field patterns for field-level encryption have been created.</p>
    TooManyFieldLevelEncryptionFieldPatterns(String),
}

impl UpdateFieldLevelEncryptionProfileError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateFieldLevelEncryptionProfileError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "AccessDenied" => return RusotoError::Service(UpdateFieldLevelEncryptionProfileError::AccessDenied(parsed_error.message)),"FieldLevelEncryptionProfileAlreadyExists" => return RusotoError::Service(UpdateFieldLevelEncryptionProfileError::FieldLevelEncryptionProfileAlreadyExists(parsed_error.message)),"FieldLevelEncryptionProfileSizeExceeded" => return RusotoError::Service(UpdateFieldLevelEncryptionProfileError::FieldLevelEncryptionProfileSizeExceeded(parsed_error.message)),"IllegalUpdate" => return RusotoError::Service(UpdateFieldLevelEncryptionProfileError::IllegalUpdate(parsed_error.message)),"InconsistentQuantities" => return RusotoError::Service(UpdateFieldLevelEncryptionProfileError::InconsistentQuantities(parsed_error.message)),"InvalidArgument" => return RusotoError::Service(UpdateFieldLevelEncryptionProfileError::InvalidArgument(parsed_error.message)),"InvalidIfMatchVersion" => return RusotoError::Service(UpdateFieldLevelEncryptionProfileError::InvalidIfMatchVersion(parsed_error.message)),"NoSuchFieldLevelEncryptionProfile" => return RusotoError::Service(UpdateFieldLevelEncryptionProfileError::NoSuchFieldLevelEncryptionProfile(parsed_error.message)),"NoSuchPublicKey" => return RusotoError::Service(UpdateFieldLevelEncryptionProfileError::NoSuchPublicKey(parsed_error.message)),"PreconditionFailed" => return RusotoError::Service(UpdateFieldLevelEncryptionProfileError::PreconditionFailed(parsed_error.message)),"TooManyFieldLevelEncryptionEncryptionEntities" => return RusotoError::Service(UpdateFieldLevelEncryptionProfileError::TooManyFieldLevelEncryptionEncryptionEntities(parsed_error.message)),"TooManyFieldLevelEncryptionFieldPatterns" => return RusotoError::Service(UpdateFieldLevelEncryptionProfileError::TooManyFieldLevelEncryptionFieldPatterns(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateFieldLevelEncryptionProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            UpdateFieldLevelEncryptionProfileError::AccessDenied(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionProfileError::FieldLevelEncryptionProfileAlreadyExists(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionProfileError::FieldLevelEncryptionProfileSizeExceeded(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionProfileError::IllegalUpdate(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionProfileError::InconsistentQuantities(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionProfileError::InvalidArgument(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionProfileError::InvalidIfMatchVersion(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionProfileError::NoSuchFieldLevelEncryptionProfile(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionProfileError::NoSuchPublicKey(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionProfileError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionProfileError::TooManyFieldLevelEncryptionEncryptionEntities(ref cause) => write!(f, "{}", cause),
UpdateFieldLevelEncryptionProfileError::TooManyFieldLevelEncryptionFieldPatterns(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for UpdateFieldLevelEncryptionProfileError {}
/// Errors returned by UpdateKeyGroup
#[derive(Debug, PartialEq)]
pub enum UpdateKeyGroupError {
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>A key group with this name already exists. You must provide a unique name. To modify an existing key group, use <code>UpdateKeyGroup</code>.</p>
    KeyGroupAlreadyExists(String),
    /// <p>A resource that was specified is not valid.</p>
    NoSuchResource(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
    /// <p>The number of public keys in this key group is more than the maximum allowed. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyPublicKeysInKeyGroup(String),
}

impl UpdateKeyGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateKeyGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidArgument" => {
                        return RusotoError::Service(UpdateKeyGroupError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "InvalidIfMatchVersion" => {
                        return RusotoError::Service(UpdateKeyGroupError::InvalidIfMatchVersion(
                            parsed_error.message,
                        ))
                    }
                    "KeyGroupAlreadyExists" => {
                        return RusotoError::Service(UpdateKeyGroupError::KeyGroupAlreadyExists(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchResource" => {
                        return RusotoError::Service(UpdateKeyGroupError::NoSuchResource(
                            parsed_error.message,
                        ))
                    }
                    "PreconditionFailed" => {
                        return RusotoError::Service(UpdateKeyGroupError::PreconditionFailed(
                            parsed_error.message,
                        ))
                    }
                    "TooManyPublicKeysInKeyGroup" => {
                        return RusotoError::Service(
                            UpdateKeyGroupError::TooManyPublicKeysInKeyGroup(parsed_error.message),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateKeyGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateKeyGroupError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateKeyGroupError::InvalidIfMatchVersion(ref cause) => write!(f, "{}", cause),
            UpdateKeyGroupError::KeyGroupAlreadyExists(ref cause) => write!(f, "{}", cause),
            UpdateKeyGroupError::NoSuchResource(ref cause) => write!(f, "{}", cause),
            UpdateKeyGroupError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
            UpdateKeyGroupError::TooManyPublicKeysInKeyGroup(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateKeyGroupError {}
/// Errors returned by UpdateOriginRequestPolicy
#[derive(Debug, PartialEq)]
pub enum UpdateOriginRequestPolicyError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The update contains modifications that are not allowed.</p>
    IllegalUpdate(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The origin request policy does not exist.</p>
    NoSuchOriginRequestPolicy(String),
    /// <p>An origin request policy with this name already exists. You must provide a unique name. To modify an existing origin request policy, use <code>UpdateOriginRequestPolicy</code>.</p>
    OriginRequestPolicyAlreadyExists(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
    /// <p>The number of cookies in the origin request policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyCookiesInOriginRequestPolicy(String),
    /// <p>The number of headers in the origin request policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyHeadersInOriginRequestPolicy(String),
    /// <p>The number of query strings in the origin request policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyQueryStringsInOriginRequestPolicy(String),
}

impl UpdateOriginRequestPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateOriginRequestPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(UpdateOriginRequestPolicyError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "IllegalUpdate" => {
                        return RusotoError::Service(UpdateOriginRequestPolicyError::IllegalUpdate(
                            parsed_error.message,
                        ))
                    }
                    "InconsistentQuantities" => {
                        return RusotoError::Service(
                            UpdateOriginRequestPolicyError::InconsistentQuantities(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            UpdateOriginRequestPolicyError::InvalidArgument(parsed_error.message),
                        )
                    }
                    "InvalidIfMatchVersion" => {
                        return RusotoError::Service(
                            UpdateOriginRequestPolicyError::InvalidIfMatchVersion(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchOriginRequestPolicy" => {
                        return RusotoError::Service(
                            UpdateOriginRequestPolicyError::NoSuchOriginRequestPolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "OriginRequestPolicyAlreadyExists" => {
                        return RusotoError::Service(
                            UpdateOriginRequestPolicyError::OriginRequestPolicyAlreadyExists(
                                parsed_error.message,
                            ),
                        )
                    }
                    "PreconditionFailed" => {
                        return RusotoError::Service(
                            UpdateOriginRequestPolicyError::PreconditionFailed(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyCookiesInOriginRequestPolicy" => {
                        return RusotoError::Service(
                            UpdateOriginRequestPolicyError::TooManyCookiesInOriginRequestPolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyHeadersInOriginRequestPolicy" => {
                        return RusotoError::Service(
                            UpdateOriginRequestPolicyError::TooManyHeadersInOriginRequestPolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyQueryStringsInOriginRequestPolicy" => return RusotoError::Service(
                        UpdateOriginRequestPolicyError::TooManyQueryStringsInOriginRequestPolicy(
                            parsed_error.message,
                        ),
                    ),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateOriginRequestPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateOriginRequestPolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateOriginRequestPolicyError::IllegalUpdate(ref cause) => write!(f, "{}", cause),
            UpdateOriginRequestPolicyError::InconsistentQuantities(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateOriginRequestPolicyError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateOriginRequestPolicyError::InvalidIfMatchVersion(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateOriginRequestPolicyError::NoSuchOriginRequestPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateOriginRequestPolicyError::OriginRequestPolicyAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateOriginRequestPolicyError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
            UpdateOriginRequestPolicyError::TooManyCookiesInOriginRequestPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateOriginRequestPolicyError::TooManyHeadersInOriginRequestPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateOriginRequestPolicyError::TooManyQueryStringsInOriginRequestPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateOriginRequestPolicyError {}
/// Errors returned by UpdatePublicKey
#[derive(Debug, PartialEq)]
pub enum UpdatePublicKeyError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>You can't change the value of a public key.</p>
    CannotChangeImmutablePublicKeyFields(String),
    /// <p>The update contains modifications that are not allowed.</p>
    IllegalUpdate(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified public key doesn't exist.</p>
    NoSuchPublicKey(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
}

impl UpdatePublicKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePublicKeyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(UpdatePublicKeyError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "CannotChangeImmutablePublicKeyFields" => {
                        return RusotoError::Service(
                            UpdatePublicKeyError::CannotChangeImmutablePublicKeyFields(
                                parsed_error.message,
                            ),
                        )
                    }
                    "IllegalUpdate" => {
                        return RusotoError::Service(UpdatePublicKeyError::IllegalUpdate(
                            parsed_error.message,
                        ))
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(UpdatePublicKeyError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "InvalidIfMatchVersion" => {
                        return RusotoError::Service(UpdatePublicKeyError::InvalidIfMatchVersion(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchPublicKey" => {
                        return RusotoError::Service(UpdatePublicKeyError::NoSuchPublicKey(
                            parsed_error.message,
                        ))
                    }
                    "PreconditionFailed" => {
                        return RusotoError::Service(UpdatePublicKeyError::PreconditionFailed(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdatePublicKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePublicKeyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdatePublicKeyError::CannotChangeImmutablePublicKeyFields(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdatePublicKeyError::IllegalUpdate(ref cause) => write!(f, "{}", cause),
            UpdatePublicKeyError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdatePublicKeyError::InvalidIfMatchVersion(ref cause) => write!(f, "{}", cause),
            UpdatePublicKeyError::NoSuchPublicKey(ref cause) => write!(f, "{}", cause),
            UpdatePublicKeyError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdatePublicKeyError {}
/// Errors returned by UpdateRealtimeLogConfig
#[derive(Debug, PartialEq)]
pub enum UpdateRealtimeLogConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The real-time log configuration does not exist.</p>
    NoSuchRealtimeLogConfig(String),
}

impl UpdateRealtimeLogConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRealtimeLogConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(UpdateRealtimeLogConfigError::AccessDenied(
                            parsed_error.message,
                        ))
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(UpdateRealtimeLogConfigError::InvalidArgument(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchRealtimeLogConfig" => {
                        return RusotoError::Service(
                            UpdateRealtimeLogConfigError::NoSuchRealtimeLogConfig(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateRealtimeLogConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRealtimeLogConfigError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateRealtimeLogConfigError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateRealtimeLogConfigError::NoSuchRealtimeLogConfig(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateRealtimeLogConfigError {}
/// Errors returned by UpdateStreamingDistribution
#[derive(Debug, PartialEq)]
pub enum UpdateStreamingDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The CNAME specified is already defined for CloudFront.</p>
    CNAMEAlreadyExists(String),
    /// <p>The update contains modifications that are not allowed.</p>
    IllegalUpdate(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>An argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The <code>If-Match</code> version is missing or not valid.</p>
    InvalidIfMatchVersion(String),
    /// <p>The origin access identity is not valid or doesn't exist.</p>
    InvalidOriginAccessIdentity(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>The specified streaming distribution does not exist.</p>
    NoSuchStreamingDistribution(String),
    /// <p>The precondition given in one or more of the request header fields evaluated to <code>false</code>.</p>
    PreconditionFailed(String),
    /// <p>Your request contains more CNAMEs than are allowed per distribution.</p>
    TooManyStreamingDistributionCNAMEs(String),
    /// <p>Your request contains more trusted signers than are allowed per distribution.</p>
    TooManyTrustedSigners(String),
    /// <p>One or more of your trusted signers don't exist.</p>
    TrustedSignerDoesNotExist(String),
}

impl UpdateStreamingDistributionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateStreamingDistributionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AccessDenied" => {
                        return RusotoError::Service(
                            UpdateStreamingDistributionError::AccessDenied(parsed_error.message),
                        )
                    }
                    "CNAMEAlreadyExists" => {
                        return RusotoError::Service(
                            UpdateStreamingDistributionError::CNAMEAlreadyExists(
                                parsed_error.message,
                            ),
                        )
                    }
                    "IllegalUpdate" => {
                        return RusotoError::Service(
                            UpdateStreamingDistributionError::IllegalUpdate(parsed_error.message),
                        )
                    }
                    "InconsistentQuantities" => {
                        return RusotoError::Service(
                            UpdateStreamingDistributionError::InconsistentQuantities(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            UpdateStreamingDistributionError::InvalidArgument(parsed_error.message),
                        )
                    }
                    "InvalidIfMatchVersion" => {
                        return RusotoError::Service(
                            UpdateStreamingDistributionError::InvalidIfMatchVersion(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidOriginAccessIdentity" => {
                        return RusotoError::Service(
                            UpdateStreamingDistributionError::InvalidOriginAccessIdentity(
                                parsed_error.message,
                            ),
                        )
                    }
                    "MissingBody" => {
                        return RusotoError::Service(UpdateStreamingDistributionError::MissingBody(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchStreamingDistribution" => {
                        return RusotoError::Service(
                            UpdateStreamingDistributionError::NoSuchStreamingDistribution(
                                parsed_error.message,
                            ),
                        )
                    }
                    "PreconditionFailed" => {
                        return RusotoError::Service(
                            UpdateStreamingDistributionError::PreconditionFailed(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyStreamingDistributionCNAMEs" => {
                        return RusotoError::Service(
                            UpdateStreamingDistributionError::TooManyStreamingDistributionCNAMEs(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyTrustedSigners" => {
                        return RusotoError::Service(
                            UpdateStreamingDistributionError::TooManyTrustedSigners(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TrustedSignerDoesNotExist" => {
                        return RusotoError::Service(
                            UpdateStreamingDistributionError::TrustedSignerDoesNotExist(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateStreamingDistributionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateStreamingDistributionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateStreamingDistributionError::CNAMEAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateStreamingDistributionError::IllegalUpdate(ref cause) => write!(f, "{}", cause),
            UpdateStreamingDistributionError::InconsistentQuantities(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateStreamingDistributionError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateStreamingDistributionError::InvalidIfMatchVersion(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateStreamingDistributionError::InvalidOriginAccessIdentity(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateStreamingDistributionError::MissingBody(ref cause) => write!(f, "{}", cause),
            UpdateStreamingDistributionError::NoSuchStreamingDistribution(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateStreamingDistributionError::PreconditionFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateStreamingDistributionError::TooManyStreamingDistributionCNAMEs(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateStreamingDistributionError::TooManyTrustedSigners(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateStreamingDistributionError::TrustedSignerDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateStreamingDistributionError {}
/// Trait representing the capabilities of the CloudFront API. CloudFront clients implement this trait.
#[async_trait]
pub trait CloudFront {
    /// <p>Creates a cache policy.</p> <p>After you create a cache policy, you can attach it to one or more cache behaviors. When it’s attached to a cache behavior, the cache policy determines the following:</p> <ul> <li> <p>The values that CloudFront includes in the <i>cache key</i>. These values can include HTTP headers, cookies, and URL query strings. CloudFront uses the cache key to find an object in its cache that it can return to the viewer.</p> </li> <li> <p>The default, minimum, and maximum time to live (TTL) values that you want objects to stay in the CloudFront cache.</p> </li> </ul> <p>The headers, cookies, and query strings that are included in the cache key are automatically included in requests that CloudFront sends to the origin. CloudFront sends a request when it can’t find an object in its cache that matches the request’s cache key. If you want to send values to the origin but <i>not</i> include them in the cache key, use <code>OriginRequestPolicy</code>.</p> <p>For more information about cache policies, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html">Controlling the cache key</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    async fn create_cache_policy(
        &self,
        input: CreateCachePolicyRequest,
    ) -> Result<CreateCachePolicyResult, RusotoError<CreateCachePolicyError>>;

    /// <p>Creates a new origin access identity. If you're using Amazon S3 for your origin, you can use an origin access identity to require users to access your content using a CloudFront URL instead of the Amazon S3 URL. For more information about how to use origin access identities, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    async fn create_cloud_front_origin_access_identity(
        &self,
        input: CreateCloudFrontOriginAccessIdentityRequest,
    ) -> Result<
        CreateCloudFrontOriginAccessIdentityResult,
        RusotoError<CreateCloudFrontOriginAccessIdentityError>,
    >;

    /// <p><p>Creates a new web distribution. You create a CloudFront distribution to tell CloudFront where you want content to be delivered from, and the details about how to track and manage content delivery. Send a <code>POST</code> request to the <code>/<i>CloudFront API version</i>/distribution</code>/<code>distribution ID</code> resource.</p> <important> <p>When you update a distribution, there are more required fields than when you create a distribution. When you update your distribution by using <a href="https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_UpdateDistribution.html">UpdateDistribution</a>, follow the steps included in the documentation to get the current configuration and then make your updates. This helps to make sure that you include all of the required fields. To view a summary, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-overview-required-fields.html">Required Fields for Create Distribution and Update Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </important></p>
    async fn create_distribution(
        &self,
        input: CreateDistributionRequest,
    ) -> Result<CreateDistributionResult, RusotoError<CreateDistributionError>>;

    /// <p>Create a new distribution with tags.</p>
    async fn create_distribution_with_tags(
        &self,
        input: CreateDistributionWithTagsRequest,
    ) -> Result<CreateDistributionWithTagsResult, RusotoError<CreateDistributionWithTagsError>>;

    /// <p>Create a new field-level encryption configuration.</p>
    async fn create_field_level_encryption_config(
        &self,
        input: CreateFieldLevelEncryptionConfigRequest,
    ) -> Result<
        CreateFieldLevelEncryptionConfigResult,
        RusotoError<CreateFieldLevelEncryptionConfigError>,
    >;

    /// <p>Create a field-level encryption profile.</p>
    async fn create_field_level_encryption_profile(
        &self,
        input: CreateFieldLevelEncryptionProfileRequest,
    ) -> Result<
        CreateFieldLevelEncryptionProfileResult,
        RusotoError<CreateFieldLevelEncryptionProfileError>,
    >;

    /// <p>Create a new invalidation. </p>
    async fn create_invalidation(
        &self,
        input: CreateInvalidationRequest,
    ) -> Result<CreateInvalidationResult, RusotoError<CreateInvalidationError>>;

    /// <p>Creates a key group that you can use with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">CloudFront signed URLs and signed cookies</a>.</p> <p>To create a key group, you must specify at least one public key for the key group. After you create a key group, you can reference it from one or more cache behaviors. When you reference a key group in a cache behavior, CloudFront requires signed URLs or signed cookies for all requests that match the cache behavior. The URLs or cookies must be signed with a private key whose corresponding public key is in the key group. The signed URL or cookie contains information about which public key CloudFront should use to verify the signature. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving private content</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    async fn create_key_group(
        &self,
        input: CreateKeyGroupRequest,
    ) -> Result<CreateKeyGroupResult, RusotoError<CreateKeyGroupError>>;

    /// <p>Enables additional CloudWatch metrics for the specified CloudFront distribution. The additional metrics incur an additional cost.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/viewing-cloudfront-metrics.html#monitoring-console.distributions-additional">Viewing additional CloudFront distribution metrics</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    async fn create_monitoring_subscription(
        &self,
        input: CreateMonitoringSubscriptionRequest,
    ) -> Result<CreateMonitoringSubscriptionResult, RusotoError<CreateMonitoringSubscriptionError>>;

    /// <p>Creates an origin request policy.</p> <p>After you create an origin request policy, you can attach it to one or more cache behaviors. When it’s attached to a cache behavior, the origin request policy determines the values that CloudFront includes in requests that it sends to the origin. Each request that CloudFront sends to the origin includes the following:</p> <ul> <li> <p>The request body and the URL path (without the domain name) from the viewer request.</p> </li> <li> <p>The headers that CloudFront automatically includes in every origin request, including <code>Host</code>, <code>User-Agent</code>, and <code>X-Amz-Cf-Id</code>.</p> </li> <li> <p>All HTTP headers, cookies, and URL query strings that are specified in the cache policy or the origin request policy. These can include items from the viewer request and, in the case of headers, additional ones that are added by CloudFront.</p> </li> </ul> <p>CloudFront sends a request when it can’t find a valid object in its cache that matches the request. If you want to send values to the origin and also include them in the cache key, use <code>CachePolicy</code>.</p> <p>For more information about origin request policies, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-origin-requests.html">Controlling origin requests</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    async fn create_origin_request_policy(
        &self,
        input: CreateOriginRequestPolicyRequest,
    ) -> Result<CreateOriginRequestPolicyResult, RusotoError<CreateOriginRequestPolicyError>>;

    /// <p>Uploads a public key to CloudFront that you can use with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">signed URLs and signed cookies</a>, or with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/field-level-encryption.html">field-level encryption</a>.</p>
    async fn create_public_key(
        &self,
        input: CreatePublicKeyRequest,
    ) -> Result<CreatePublicKeyResult, RusotoError<CreatePublicKeyError>>;

    /// <p>Creates a real-time log configuration.</p> <p>After you create a real-time log configuration, you can attach it to one or more cache behaviors to send real-time log data to the specified Amazon Kinesis data stream.</p> <p>For more information about real-time log configurations, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html">Real-time logs</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    async fn create_realtime_log_config(
        &self,
        input: CreateRealtimeLogConfigRequest,
    ) -> Result<CreateRealtimeLogConfigResult, RusotoError<CreateRealtimeLogConfigError>>;

    /// <p><p>Creates a new RTMP distribution. An RTMP distribution is similar to a web distribution, but an RTMP distribution streams media files using the Adobe Real-Time Messaging Protocol (RTMP) instead of serving files using HTTP. </p> <p>To create a new distribution, submit a <code>POST</code> request to the <i>CloudFront API version</i>/distribution resource. The request body must include a document with a <i>StreamingDistributionConfig</i> element. The response echoes the <code>StreamingDistributionConfig</code> element and returns other information about the RTMP distribution.</p> <p>To get the status of your request, use the <i>GET StreamingDistribution</i> API action. When the value of <code>Enabled</code> is <code>true</code> and the value of <code>Status</code> is <code>Deployed</code>, your distribution is ready. A distribution usually deploys in less than 15 minutes.</p> <p>For more information about web distributions, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-rtmp.html">Working with RTMP Distributions</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <important> <p>Beginning with the 2012-05-05 version of the CloudFront API, we made substantial changes to the format of the XML document that you include in the request body when you create or update a web distribution or an RTMP distribution, and when you invalidate objects. With previous versions of the API, we discovered that it was too easy to accidentally delete one or more values for an element that accepts multiple values, for example, CNAMEs and trusted signers. Our changes for the 2012-05-05 release are intended to prevent these accidental deletions and to notify you when there&#39;s a mismatch between the number of values you say you&#39;re specifying in the <code>Quantity</code> element and the number of values specified.</p> </important></p>
    async fn create_streaming_distribution(
        &self,
        input: CreateStreamingDistributionRequest,
    ) -> Result<CreateStreamingDistributionResult, RusotoError<CreateStreamingDistributionError>>;

    /// <p>Create a new streaming distribution with tags.</p>
    async fn create_streaming_distribution_with_tags(
        &self,
        input: CreateStreamingDistributionWithTagsRequest,
    ) -> Result<
        CreateStreamingDistributionWithTagsResult,
        RusotoError<CreateStreamingDistributionWithTagsError>,
    >;

    /// <p>Deletes a cache policy.</p> <p>You cannot delete a cache policy if it’s attached to a cache behavior. First update your distributions to remove the cache policy from all cache behaviors, then delete the cache policy.</p> <p>To delete a cache policy, you must provide the policy’s identifier and version. To get these values, you can use <code>ListCachePolicies</code> or <code>GetCachePolicy</code>.</p>
    async fn delete_cache_policy(
        &self,
        input: DeleteCachePolicyRequest,
    ) -> Result<(), RusotoError<DeleteCachePolicyError>>;

    /// <p>Delete an origin access identity. </p>
    async fn delete_cloud_front_origin_access_identity(
        &self,
        input: DeleteCloudFrontOriginAccessIdentityRequest,
    ) -> Result<(), RusotoError<DeleteCloudFrontOriginAccessIdentityError>>;

    /// <p>Delete a distribution. </p>
    async fn delete_distribution(
        &self,
        input: DeleteDistributionRequest,
    ) -> Result<(), RusotoError<DeleteDistributionError>>;

    /// <p>Remove a field-level encryption configuration.</p>
    async fn delete_field_level_encryption_config(
        &self,
        input: DeleteFieldLevelEncryptionConfigRequest,
    ) -> Result<(), RusotoError<DeleteFieldLevelEncryptionConfigError>>;

    /// <p>Remove a field-level encryption profile.</p>
    async fn delete_field_level_encryption_profile(
        &self,
        input: DeleteFieldLevelEncryptionProfileRequest,
    ) -> Result<(), RusotoError<DeleteFieldLevelEncryptionProfileError>>;

    /// <p>Deletes a key group.</p> <p>You cannot delete a key group that is referenced in a cache behavior. First update your distributions to remove the key group from all cache behaviors, then delete the key group.</p> <p>To delete a key group, you must provide the key group’s identifier and version. To get these values, use <code>ListKeyGroups</code> followed by <code>GetKeyGroup</code> or <code>GetKeyGroupConfig</code>.</p>
    async fn delete_key_group(
        &self,
        input: DeleteKeyGroupRequest,
    ) -> Result<(), RusotoError<DeleteKeyGroupError>>;

    /// <p>Disables additional CloudWatch metrics for the specified CloudFront distribution.</p>
    async fn delete_monitoring_subscription(
        &self,
        input: DeleteMonitoringSubscriptionRequest,
    ) -> Result<DeleteMonitoringSubscriptionResult, RusotoError<DeleteMonitoringSubscriptionError>>;

    /// <p>Deletes an origin request policy.</p> <p>You cannot delete an origin request policy if it’s attached to any cache behaviors. First update your distributions to remove the origin request policy from all cache behaviors, then delete the origin request policy.</p> <p>To delete an origin request policy, you must provide the policy’s identifier and version. To get the identifier, you can use <code>ListOriginRequestPolicies</code> or <code>GetOriginRequestPolicy</code>.</p>
    async fn delete_origin_request_policy(
        &self,
        input: DeleteOriginRequestPolicyRequest,
    ) -> Result<(), RusotoError<DeleteOriginRequestPolicyError>>;

    /// <p>Remove a public key you previously added to CloudFront.</p>
    async fn delete_public_key(
        &self,
        input: DeletePublicKeyRequest,
    ) -> Result<(), RusotoError<DeletePublicKeyError>>;

    /// <p>Deletes a real-time log configuration.</p> <p>You cannot delete a real-time log configuration if it’s attached to a cache behavior. First update your distributions to remove the real-time log configuration from all cache behaviors, then delete the real-time log configuration.</p> <p>To delete a real-time log configuration, you can provide the configuration’s name or its Amazon Resource Name (ARN). You must provide at least one. If you provide both, CloudFront uses the name to identify the real-time log configuration to delete.</p>
    async fn delete_realtime_log_config(
        &self,
        input: DeleteRealtimeLogConfigRequest,
    ) -> Result<(), RusotoError<DeleteRealtimeLogConfigError>>;

    /// <p>Delete a streaming distribution. To delete an RTMP distribution using the CloudFront API, perform the following steps.</p> <p> <b>To delete an RTMP distribution using the CloudFront API</b>:</p> <ol> <li> <p>Disable the RTMP distribution.</p> </li> <li> <p>Submit a <code>GET Streaming Distribution Config</code> request to get the current configuration and the <code>Etag</code> header for the distribution. </p> </li> <li> <p>Update the XML document that was returned in the response to your <code>GET Streaming Distribution Config</code> request to change the value of <code>Enabled</code> to <code>false</code>.</p> </li> <li> <p>Submit a <code>PUT Streaming Distribution Config</code> request to update the configuration for your distribution. In the request body, include the XML document that you updated in Step 3. Then set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Streaming Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to the <code>PUT Streaming Distribution Config</code> request to confirm that the distribution was successfully disabled.</p> </li> <li> <p>Submit a <code>GET Streaming Distribution Config</code> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> </li> <li> <p>Submit a <code>DELETE Streaming Distribution</code> request. Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Streaming Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to your <code>DELETE Streaming Distribution</code> request to confirm that the distribution was successfully deleted.</p> </li> </ol> <p>For information about deleting a distribution using the CloudFront console, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/HowToDeleteDistribution.html">Deleting a Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    async fn delete_streaming_distribution(
        &self,
        input: DeleteStreamingDistributionRequest,
    ) -> Result<(), RusotoError<DeleteStreamingDistributionError>>;

    /// <p>Gets a cache policy, including the following metadata:</p> <ul> <li> <p>The policy’s identifier.</p> </li> <li> <p>The date and time when the policy was last modified.</p> </li> </ul> <p>To get a cache policy, you must provide the policy’s identifier. If the cache policy is attached to a distribution’s cache behavior, you can get the policy’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the cache policy is not attached to a cache behavior, you can get the identifier using <code>ListCachePolicies</code>.</p>
    async fn get_cache_policy(
        &self,
        input: GetCachePolicyRequest,
    ) -> Result<GetCachePolicyResult, RusotoError<GetCachePolicyError>>;

    /// <p>Gets a cache policy configuration.</p> <p>To get a cache policy configuration, you must provide the policy’s identifier. If the cache policy is attached to a distribution’s cache behavior, you can get the policy’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the cache policy is not attached to a cache behavior, you can get the identifier using <code>ListCachePolicies</code>.</p>
    async fn get_cache_policy_config(
        &self,
        input: GetCachePolicyConfigRequest,
    ) -> Result<GetCachePolicyConfigResult, RusotoError<GetCachePolicyConfigError>>;

    /// <p>Get the information about an origin access identity. </p>
    async fn get_cloud_front_origin_access_identity(
        &self,
        input: GetCloudFrontOriginAccessIdentityRequest,
    ) -> Result<
        GetCloudFrontOriginAccessIdentityResult,
        RusotoError<GetCloudFrontOriginAccessIdentityError>,
    >;

    /// <p>Get the configuration information about an origin access identity. </p>
    async fn get_cloud_front_origin_access_identity_config(
        &self,
        input: GetCloudFrontOriginAccessIdentityConfigRequest,
    ) -> Result<
        GetCloudFrontOriginAccessIdentityConfigResult,
        RusotoError<GetCloudFrontOriginAccessIdentityConfigError>,
    >;

    /// <p>Get the information about a distribution.</p>
    async fn get_distribution(
        &self,
        input: GetDistributionRequest,
    ) -> Result<GetDistributionResult, RusotoError<GetDistributionError>>;

    /// <p>Get the configuration information about a distribution. </p>
    async fn get_distribution_config(
        &self,
        input: GetDistributionConfigRequest,
    ) -> Result<GetDistributionConfigResult, RusotoError<GetDistributionConfigError>>;

    /// <p>Get the field-level encryption configuration information.</p>
    async fn get_field_level_encryption(
        &self,
        input: GetFieldLevelEncryptionRequest,
    ) -> Result<GetFieldLevelEncryptionResult, RusotoError<GetFieldLevelEncryptionError>>;

    /// <p>Get the field-level encryption configuration information.</p>
    async fn get_field_level_encryption_config(
        &self,
        input: GetFieldLevelEncryptionConfigRequest,
    ) -> Result<GetFieldLevelEncryptionConfigResult, RusotoError<GetFieldLevelEncryptionConfigError>>;

    /// <p>Get the field-level encryption profile information.</p>
    async fn get_field_level_encryption_profile(
        &self,
        input: GetFieldLevelEncryptionProfileRequest,
    ) -> Result<
        GetFieldLevelEncryptionProfileResult,
        RusotoError<GetFieldLevelEncryptionProfileError>,
    >;

    /// <p>Get the field-level encryption profile configuration information.</p>
    async fn get_field_level_encryption_profile_config(
        &self,
        input: GetFieldLevelEncryptionProfileConfigRequest,
    ) -> Result<
        GetFieldLevelEncryptionProfileConfigResult,
        RusotoError<GetFieldLevelEncryptionProfileConfigError>,
    >;

    /// <p>Get the information about an invalidation. </p>
    async fn get_invalidation(
        &self,
        input: GetInvalidationRequest,
    ) -> Result<GetInvalidationResult, RusotoError<GetInvalidationError>>;

    /// <p>Gets a key group, including the date and time when the key group was last modified.</p> <p>To get a key group, you must provide the key group’s identifier. If the key group is referenced in a distribution’s cache behavior, you can get the key group’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the key group is not referenced in a cache behavior, you can get the identifier using <code>ListKeyGroups</code>.</p>
    async fn get_key_group(
        &self,
        input: GetKeyGroupRequest,
    ) -> Result<GetKeyGroupResult, RusotoError<GetKeyGroupError>>;

    /// <p>Gets a key group configuration.</p> <p>To get a key group configuration, you must provide the key group’s identifier. If the key group is referenced in a distribution’s cache behavior, you can get the key group’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the key group is not referenced in a cache behavior, you can get the identifier using <code>ListKeyGroups</code>.</p>
    async fn get_key_group_config(
        &self,
        input: GetKeyGroupConfigRequest,
    ) -> Result<GetKeyGroupConfigResult, RusotoError<GetKeyGroupConfigError>>;

    /// <p>Gets information about whether additional CloudWatch metrics are enabled for the specified CloudFront distribution.</p>
    async fn get_monitoring_subscription(
        &self,
        input: GetMonitoringSubscriptionRequest,
    ) -> Result<GetMonitoringSubscriptionResult, RusotoError<GetMonitoringSubscriptionError>>;

    /// <p>Gets an origin request policy, including the following metadata:</p> <ul> <li> <p>The policy’s identifier.</p> </li> <li> <p>The date and time when the policy was last modified.</p> </li> </ul> <p>To get an origin request policy, you must provide the policy’s identifier. If the origin request policy is attached to a distribution’s cache behavior, you can get the policy’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the origin request policy is not attached to a cache behavior, you can get the identifier using <code>ListOriginRequestPolicies</code>.</p>
    async fn get_origin_request_policy(
        &self,
        input: GetOriginRequestPolicyRequest,
    ) -> Result<GetOriginRequestPolicyResult, RusotoError<GetOriginRequestPolicyError>>;

    /// <p>Gets an origin request policy configuration.</p> <p>To get an origin request policy configuration, you must provide the policy’s identifier. If the origin request policy is attached to a distribution’s cache behavior, you can get the policy’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the origin request policy is not attached to a cache behavior, you can get the identifier using <code>ListOriginRequestPolicies</code>.</p>
    async fn get_origin_request_policy_config(
        &self,
        input: GetOriginRequestPolicyConfigRequest,
    ) -> Result<GetOriginRequestPolicyConfigResult, RusotoError<GetOriginRequestPolicyConfigError>>;

    /// <p>Gets a public key.</p>
    async fn get_public_key(
        &self,
        input: GetPublicKeyRequest,
    ) -> Result<GetPublicKeyResult, RusotoError<GetPublicKeyError>>;

    /// <p>Gets a public key configuration.</p>
    async fn get_public_key_config(
        &self,
        input: GetPublicKeyConfigRequest,
    ) -> Result<GetPublicKeyConfigResult, RusotoError<GetPublicKeyConfigError>>;

    /// <p>Gets a real-time log configuration.</p> <p>To get a real-time log configuration, you can provide the configuration’s name or its Amazon Resource Name (ARN). You must provide at least one. If you provide both, CloudFront uses the name to identify the real-time log configuration to get.</p>
    async fn get_realtime_log_config(
        &self,
        input: GetRealtimeLogConfigRequest,
    ) -> Result<GetRealtimeLogConfigResult, RusotoError<GetRealtimeLogConfigError>>;

    /// <p>Gets information about a specified RTMP distribution, including the distribution configuration.</p>
    async fn get_streaming_distribution(
        &self,
        input: GetStreamingDistributionRequest,
    ) -> Result<GetStreamingDistributionResult, RusotoError<GetStreamingDistributionError>>;

    /// <p>Get the configuration information about a streaming distribution. </p>
    async fn get_streaming_distribution_config(
        &self,
        input: GetStreamingDistributionConfigRequest,
    ) -> Result<
        GetStreamingDistributionConfigResult,
        RusotoError<GetStreamingDistributionConfigError>,
    >;

    /// <p>Gets a list of cache policies.</p> <p>You can optionally apply a filter to return only the managed policies created by AWS, or only the custom policies created in your AWS account.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
    async fn list_cache_policies(
        &self,
        input: ListCachePoliciesRequest,
    ) -> Result<ListCachePoliciesResult, RusotoError<ListCachePoliciesError>>;

    /// <p>Lists origin access identities.</p>
    async fn list_cloud_front_origin_access_identities(
        &self,
        input: ListCloudFrontOriginAccessIdentitiesRequest,
    ) -> Result<
        ListCloudFrontOriginAccessIdentitiesResult,
        RusotoError<ListCloudFrontOriginAccessIdentitiesError>,
    >;

    /// <p>List CloudFront distributions.</p>
    async fn list_distributions(
        &self,
        input: ListDistributionsRequest,
    ) -> Result<ListDistributionsResult, RusotoError<ListDistributionsError>>;

    /// <p>Gets a list of distribution IDs for distributions that have a cache behavior that’s associated with the specified cache policy.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
    async fn list_distributions_by_cache_policy_id(
        &self,
        input: ListDistributionsByCachePolicyIdRequest,
    ) -> Result<
        ListDistributionsByCachePolicyIdResult,
        RusotoError<ListDistributionsByCachePolicyIdError>,
    >;

    /// <p>Gets a list of distribution IDs for distributions that have a cache behavior that references the specified key group.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
    async fn list_distributions_by_key_group(
        &self,
        input: ListDistributionsByKeyGroupRequest,
    ) -> Result<ListDistributionsByKeyGroupResult, RusotoError<ListDistributionsByKeyGroupError>>;

    /// <p>Gets a list of distribution IDs for distributions that have a cache behavior that’s associated with the specified origin request policy.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
    async fn list_distributions_by_origin_request_policy_id(
        &self,
        input: ListDistributionsByOriginRequestPolicyIdRequest,
    ) -> Result<
        ListDistributionsByOriginRequestPolicyIdResult,
        RusotoError<ListDistributionsByOriginRequestPolicyIdError>,
    >;

    /// <p>Gets a list of distributions that have a cache behavior that’s associated with the specified real-time log configuration.</p> <p>You can specify the real-time log configuration by its name or its Amazon Resource Name (ARN). You must provide at least one. If you provide both, CloudFront uses the name to identify the real-time log configuration to list distributions for.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request. </p>
    async fn list_distributions_by_realtime_log_config(
        &self,
        input: ListDistributionsByRealtimeLogConfigRequest,
    ) -> Result<
        ListDistributionsByRealtimeLogConfigResult,
        RusotoError<ListDistributionsByRealtimeLogConfigError>,
    >;

    /// <p>List the distributions that are associated with a specified AWS WAF web ACL. </p>
    async fn list_distributions_by_web_acl_id(
        &self,
        input: ListDistributionsByWebACLIdRequest,
    ) -> Result<ListDistributionsByWebACLIdResult, RusotoError<ListDistributionsByWebACLIdError>>;

    /// <p>List all field-level encryption configurations that have been created in CloudFront for this account.</p>
    async fn list_field_level_encryption_configs(
        &self,
        input: ListFieldLevelEncryptionConfigsRequest,
    ) -> Result<
        ListFieldLevelEncryptionConfigsResult,
        RusotoError<ListFieldLevelEncryptionConfigsError>,
    >;

    /// <p>Request a list of field-level encryption profiles that have been created in CloudFront for this account.</p>
    async fn list_field_level_encryption_profiles(
        &self,
        input: ListFieldLevelEncryptionProfilesRequest,
    ) -> Result<
        ListFieldLevelEncryptionProfilesResult,
        RusotoError<ListFieldLevelEncryptionProfilesError>,
    >;

    /// <p>Lists invalidation batches. </p>
    async fn list_invalidations(
        &self,
        input: ListInvalidationsRequest,
    ) -> Result<ListInvalidationsResult, RusotoError<ListInvalidationsError>>;

    /// <p>Gets a list of key groups.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
    async fn list_key_groups(
        &self,
        input: ListKeyGroupsRequest,
    ) -> Result<ListKeyGroupsResult, RusotoError<ListKeyGroupsError>>;

    /// <p>Gets a list of origin request policies.</p> <p>You can optionally apply a filter to return only the managed policies created by AWS, or only the custom policies created in your AWS account.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
    async fn list_origin_request_policies(
        &self,
        input: ListOriginRequestPoliciesRequest,
    ) -> Result<ListOriginRequestPoliciesResult, RusotoError<ListOriginRequestPoliciesError>>;

    /// <p>List all public keys that have been added to CloudFront for this account.</p>
    async fn list_public_keys(
        &self,
        input: ListPublicKeysRequest,
    ) -> Result<ListPublicKeysResult, RusotoError<ListPublicKeysError>>;

    /// <p>Gets a list of real-time log configurations.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request. </p>
    async fn list_realtime_log_configs(
        &self,
        input: ListRealtimeLogConfigsRequest,
    ) -> Result<ListRealtimeLogConfigsResult, RusotoError<ListRealtimeLogConfigsError>>;

    /// <p>List streaming distributions. </p>
    async fn list_streaming_distributions(
        &self,
        input: ListStreamingDistributionsRequest,
    ) -> Result<ListStreamingDistributionsResult, RusotoError<ListStreamingDistributionsError>>;

    /// <p>List tags for a CloudFront resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResult, RusotoError<ListTagsForResourceError>>;

    /// <p>Add tags to a CloudFront resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Remove tags from a CloudFront resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p><p>Updates a cache policy configuration.</p> <p>When you update a cache policy configuration, all the fields are updated with the values provided in the request. You cannot update some fields independent of others. To update a cache policy configuration:</p> <ol> <li> <p>Use <code>GetCachePolicyConfig</code> to get the current configuration.</p> </li> <li> <p>Locally modify the fields in the cache policy configuration that you want to update.</p> </li> <li> <p>Call <code>UpdateCachePolicy</code> by providing the entire cache policy configuration, including the fields that you modified and those that you didn’t.</p> </li> </ol></p>
    async fn update_cache_policy(
        &self,
        input: UpdateCachePolicyRequest,
    ) -> Result<UpdateCachePolicyResult, RusotoError<UpdateCachePolicyError>>;

    /// <p>Update an origin access identity. </p>
    async fn update_cloud_front_origin_access_identity(
        &self,
        input: UpdateCloudFrontOriginAccessIdentityRequest,
    ) -> Result<
        UpdateCloudFrontOriginAccessIdentityResult,
        RusotoError<UpdateCloudFrontOriginAccessIdentityError>,
    >;

    /// <p><p>Updates the configuration for a web distribution. </p> <important> <p>When you update a distribution, there are more required fields than when you create a distribution. When you update your distribution by using this API action, follow the steps here to get the current configuration and then make your updates, to make sure that you include all of the required fields. To view a summary, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-overview-required-fields.html">Required Fields for Create Distribution and Update Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </important> <p>The update process includes getting the current distribution configuration, updating the XML document that is returned to make your changes, and then submitting an <code>UpdateDistribution</code> request to make the updates.</p> <p>For information about updating a distribution using the CloudFront console instead, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-creating-console.html">Creating a Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p> <b>To update a web distribution using the CloudFront API</b> </p> <ol> <li> <p>Submit a <a href="https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_GetDistributionConfig.html">GetDistributionConfig</a> request to get the current configuration and an <code>Etag</code> header for the distribution.</p> <note> <p>If you update the distribution again, you must get a new <code>Etag</code> header.</p> </note> </li> <li> <p>Update the XML document that was returned in the response to your <code>GetDistributionConfig</code> request to include your changes. </p> <important> <p>When you edit the XML file, be aware of the following:</p> <ul> <li> <p>You must strip out the ETag parameter that is returned.</p> </li> <li> <p>Additional fields are required when you update a distribution. There may be fields included in the XML file for features that you haven&#39;t configured for your distribution. This is expected and required to successfully update the distribution.</p> </li> <li> <p>You can&#39;t change the value of <code>CallerReference</code>. If you try to change this value, CloudFront returns an <code>IllegalUpdate</code> error. </p> </li> <li> <p>The new configuration replaces the existing configuration; the values that you specify in an <code>UpdateDistribution</code> request are not merged into your existing configuration. When you add, delete, or replace values in an element that allows multiple values (for example, <code>CNAME</code>), you must specify all of the values that you want to appear in the updated distribution. In addition, you must update the corresponding <code>Quantity</code> element.</p> </li> </ul> </important> </li> <li> <p>Submit an <code>UpdateDistribution</code> request to update the configuration for your distribution:</p> <ul> <li> <p>In the request body, include the XML document that you updated in Step 2. The request body must include an XML document with a <code>DistributionConfig</code> element.</p> </li> <li> <p>Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GetDistributionConfig</code> request in Step 1.</p> </li> </ul> </li> <li> <p>Review the response to the <code>UpdateDistribution</code> request to confirm that the configuration was successfully updated.</p> </li> <li> <p>Optional: Submit a <a href="https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_GetDistribution.html">GetDistribution</a> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> </li> </ol></p>
    async fn update_distribution(
        &self,
        input: UpdateDistributionRequest,
    ) -> Result<UpdateDistributionResult, RusotoError<UpdateDistributionError>>;

    /// <p>Update a field-level encryption configuration. </p>
    async fn update_field_level_encryption_config(
        &self,
        input: UpdateFieldLevelEncryptionConfigRequest,
    ) -> Result<
        UpdateFieldLevelEncryptionConfigResult,
        RusotoError<UpdateFieldLevelEncryptionConfigError>,
    >;

    /// <p>Update a field-level encryption profile. </p>
    async fn update_field_level_encryption_profile(
        &self,
        input: UpdateFieldLevelEncryptionProfileRequest,
    ) -> Result<
        UpdateFieldLevelEncryptionProfileResult,
        RusotoError<UpdateFieldLevelEncryptionProfileError>,
    >;

    /// <p><p>Updates a key group.</p> <p>When you update a key group, all the fields are updated with the values provided in the request. You cannot update some fields independent of others. To update a key group:</p> <ol> <li> <p>Get the current key group with <code>GetKeyGroup</code> or <code>GetKeyGroupConfig</code>.</p> </li> <li> <p>Locally modify the fields in the key group that you want to update. For example, add or remove public key IDs.</p> </li> <li> <p>Call <code>UpdateKeyGroup</code> with the entire key group object, including the fields that you modified and those that you didn’t.</p> </li> </ol></p>
    async fn update_key_group(
        &self,
        input: UpdateKeyGroupRequest,
    ) -> Result<UpdateKeyGroupResult, RusotoError<UpdateKeyGroupError>>;

    /// <p><p>Updates an origin request policy configuration.</p> <p>When you update an origin request policy configuration, all the fields are updated with the values provided in the request. You cannot update some fields independent of others. To update an origin request policy configuration:</p> <ol> <li> <p>Use <code>GetOriginRequestPolicyConfig</code> to get the current configuration.</p> </li> <li> <p>Locally modify the fields in the origin request policy configuration that you want to update.</p> </li> <li> <p>Call <code>UpdateOriginRequestPolicy</code> by providing the entire origin request policy configuration, including the fields that you modified and those that you didn’t.</p> </li> </ol></p>
    async fn update_origin_request_policy(
        &self,
        input: UpdateOriginRequestPolicyRequest,
    ) -> Result<UpdateOriginRequestPolicyResult, RusotoError<UpdateOriginRequestPolicyError>>;

    /// <p>Update public key information. Note that the only value you can change is the comment.</p>
    async fn update_public_key(
        &self,
        input: UpdatePublicKeyRequest,
    ) -> Result<UpdatePublicKeyResult, RusotoError<UpdatePublicKeyError>>;

    /// <p>Updates a real-time log configuration.</p> <p>When you update a real-time log configuration, all the parameters are updated with the values provided in the request. You cannot update some parameters independent of others. To update a real-time log configuration:</p> <ol> <li> <p>Call <code>GetRealtimeLogConfig</code> to get the current real-time log configuration.</p> </li> <li> <p>Locally modify the parameters in the real-time log configuration that you want to update.</p> </li> <li> <p>Call this API (<code>UpdateRealtimeLogConfig</code>) by providing the entire real-time log configuration, including the parameters that you modified and those that you didn’t.</p> </li> </ol> <p>You cannot update a real-time log configuration’s <code>Name</code> or <code>ARN</code>.</p>
    async fn update_realtime_log_config(
        &self,
        input: UpdateRealtimeLogConfigRequest,
    ) -> Result<UpdateRealtimeLogConfigResult, RusotoError<UpdateRealtimeLogConfigError>>;

    /// <p>Update a streaming distribution. </p>
    async fn update_streaming_distribution(
        &self,
        input: UpdateStreamingDistributionRequest,
    ) -> Result<UpdateStreamingDistributionResult, RusotoError<UpdateStreamingDistributionError>>;
}
/// A client for the CloudFront API.
#[derive(Clone)]
pub struct CloudFrontClient {
    client: Client,
    region: region::Region,
}

impl CloudFrontClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CloudFrontClient {
        CloudFrontClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CloudFrontClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CloudFrontClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CloudFrontClient {
        CloudFrontClient { client, region }
    }
}

#[async_trait]
impl CloudFront for CloudFrontClient {
    /// <p>Creates a cache policy.</p> <p>After you create a cache policy, you can attach it to one or more cache behaviors. When it’s attached to a cache behavior, the cache policy determines the following:</p> <ul> <li> <p>The values that CloudFront includes in the <i>cache key</i>. These values can include HTTP headers, cookies, and URL query strings. CloudFront uses the cache key to find an object in its cache that it can return to the viewer.</p> </li> <li> <p>The default, minimum, and maximum time to live (TTL) values that you want objects to stay in the CloudFront cache.</p> </li> </ul> <p>The headers, cookies, and query strings that are included in the cache key are automatically included in requests that CloudFront sends to the origin. CloudFront sends a request when it can’t find an object in its cache that matches the request’s cache key. If you want to send values to the origin but <i>not</i> include them in the cache key, use <code>OriginRequestPolicy</code>.</p> <p>For more information about cache policies, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html">Controlling the cache key</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn create_cache_policy(
        &self,
        input: CreateCachePolicyRequest,
    ) -> Result<CreateCachePolicyResult, RusotoError<CreateCachePolicyError>> {
        let request_uri = "/2020-05-31/cache-policy";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CachePolicyConfigSerializer::serialize(
            &mut writer,
            "CachePolicyConfig",
            &input.cache_policy_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, CreateCachePolicyError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreateCachePolicyResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag");
        result.location = response.headers.remove("Location"); // parse non-payload
        Ok(result)
    }

    /// <p>Creates a new origin access identity. If you're using Amazon S3 for your origin, you can use an origin access identity to require users to access your content using a CloudFront URL instead of the Amazon S3 URL. For more information about how to use origin access identities, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn create_cloud_front_origin_access_identity(
        &self,
        input: CreateCloudFrontOriginAccessIdentityRequest,
    ) -> Result<
        CreateCloudFrontOriginAccessIdentityResult,
        RusotoError<CreateCloudFrontOriginAccessIdentityError>,
    > {
        let request_uri = "/2020-05-31/origin-access-identity/cloudfront";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CloudFrontOriginAccessIdentityConfigSerializer::serialize(
            &mut writer,
            "CloudFrontOriginAccessIdentityConfig",
            &input.cloud_front_origin_access_identity_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(
                request,
                CreateCloudFrontOriginAccessIdentityError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreateCloudFrontOriginAccessIdentityResultDeserializer::deserialize(
                actual_tag_name,
                stack,
            )
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag");
        result.location = response.headers.remove("Location"); // parse non-payload
        Ok(result)
    }

    /// <p><p>Creates a new web distribution. You create a CloudFront distribution to tell CloudFront where you want content to be delivered from, and the details about how to track and manage content delivery. Send a <code>POST</code> request to the <code>/<i>CloudFront API version</i>/distribution</code>/<code>distribution ID</code> resource.</p> <important> <p>When you update a distribution, there are more required fields than when you create a distribution. When you update your distribution by using <a href="https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_UpdateDistribution.html">UpdateDistribution</a>, follow the steps included in the documentation to get the current configuration and then make your updates. This helps to make sure that you include all of the required fields. To view a summary, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-overview-required-fields.html">Required Fields for Create Distribution and Update Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </important></p>
    #[allow(unused_variables, warnings)]
    async fn create_distribution(
        &self,
        input: CreateDistributionRequest,
    ) -> Result<CreateDistributionResult, RusotoError<CreateDistributionError>> {
        let request_uri = "/2020-05-31/distribution";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        DistributionConfigSerializer::serialize(
            &mut writer,
            "DistributionConfig",
            &input.distribution_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, CreateDistributionError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreateDistributionResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag");
        result.location = response.headers.remove("Location"); // parse non-payload
        Ok(result)
    }

    /// <p>Create a new distribution with tags.</p>
    #[allow(unused_variables, warnings)]
    async fn create_distribution_with_tags(
        &self,
        input: CreateDistributionWithTagsRequest,
    ) -> Result<CreateDistributionWithTagsResult, RusotoError<CreateDistributionWithTagsError>>
    {
        let request_uri = "/2020-05-31/distribution";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("WithTags");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        DistributionConfigWithTagsSerializer::serialize(
            &mut writer,
            "DistributionConfigWithTags",
            &input.distribution_config_with_tags,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, CreateDistributionWithTagsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreateDistributionWithTagsResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag");
        result.location = response.headers.remove("Location"); // parse non-payload
        Ok(result)
    }

    /// <p>Create a new field-level encryption configuration.</p>
    #[allow(unused_variables, warnings)]
    async fn create_field_level_encryption_config(
        &self,
        input: CreateFieldLevelEncryptionConfigRequest,
    ) -> Result<
        CreateFieldLevelEncryptionConfigResult,
        RusotoError<CreateFieldLevelEncryptionConfigError>,
    > {
        let request_uri = "/2020-05-31/field-level-encryption";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        FieldLevelEncryptionConfigSerializer::serialize(
            &mut writer,
            "FieldLevelEncryptionConfig",
            &input.field_level_encryption_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(
                request,
                CreateFieldLevelEncryptionConfigError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreateFieldLevelEncryptionConfigResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag");
        result.location = response.headers.remove("Location"); // parse non-payload
        Ok(result)
    }

    /// <p>Create a field-level encryption profile.</p>
    #[allow(unused_variables, warnings)]
    async fn create_field_level_encryption_profile(
        &self,
        input: CreateFieldLevelEncryptionProfileRequest,
    ) -> Result<
        CreateFieldLevelEncryptionProfileResult,
        RusotoError<CreateFieldLevelEncryptionProfileError>,
    > {
        let request_uri = "/2020-05-31/field-level-encryption-profile";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        FieldLevelEncryptionProfileConfigSerializer::serialize(
            &mut writer,
            "FieldLevelEncryptionProfileConfig",
            &input.field_level_encryption_profile_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(
                request,
                CreateFieldLevelEncryptionProfileError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreateFieldLevelEncryptionProfileResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag");
        result.location = response.headers.remove("Location"); // parse non-payload
        Ok(result)
    }

    /// <p>Create a new invalidation. </p>
    #[allow(unused_variables, warnings)]
    async fn create_invalidation(
        &self,
        input: CreateInvalidationRequest,
    ) -> Result<CreateInvalidationResult, RusotoError<CreateInvalidationError>> {
        let request_uri = format!(
            "/2020-05-31/distribution/{distribution_id}/invalidation",
            distribution_id = input.distribution_id
        );

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        InvalidationBatchSerializer::serialize(
            &mut writer,
            "InvalidationBatch",
            &input.invalidation_batch,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, CreateInvalidationError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreateInvalidationResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.location = response.headers.remove("Location"); // parse non-payload
        Ok(result)
    }

    /// <p>Creates a key group that you can use with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">CloudFront signed URLs and signed cookies</a>.</p> <p>To create a key group, you must specify at least one public key for the key group. After you create a key group, you can reference it from one or more cache behaviors. When you reference a key group in a cache behavior, CloudFront requires signed URLs or signed cookies for all requests that match the cache behavior. The URLs or cookies must be signed with a private key whose corresponding public key is in the key group. The signed URL or cookie contains information about which public key CloudFront should use to verify the signature. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving private content</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn create_key_group(
        &self,
        input: CreateKeyGroupRequest,
    ) -> Result<CreateKeyGroupResult, RusotoError<CreateKeyGroupError>> {
        let request_uri = "/2020-05-31/key-group";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        KeyGroupConfigSerializer::serialize(&mut writer, "KeyGroupConfig", &input.key_group_config);
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, CreateKeyGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreateKeyGroupResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag");
        result.location = response.headers.remove("Location"); // parse non-payload
        Ok(result)
    }

    /// <p>Enables additional CloudWatch metrics for the specified CloudFront distribution. The additional metrics incur an additional cost.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/viewing-cloudfront-metrics.html#monitoring-console.distributions-additional">Viewing additional CloudFront distribution metrics</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn create_monitoring_subscription(
        &self,
        input: CreateMonitoringSubscriptionRequest,
    ) -> Result<CreateMonitoringSubscriptionResult, RusotoError<CreateMonitoringSubscriptionError>>
    {
        let request_uri = format!(
            "/2020-05-31/distributions/{distribution_id}/monitoring-subscription",
            distribution_id = input.distribution_id
        );

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        MonitoringSubscriptionSerializer::serialize(
            &mut writer,
            "MonitoringSubscription",
            &input.monitoring_subscription,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, CreateMonitoringSubscriptionError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreateMonitoringSubscriptionResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Creates an origin request policy.</p> <p>After you create an origin request policy, you can attach it to one or more cache behaviors. When it’s attached to a cache behavior, the origin request policy determines the values that CloudFront includes in requests that it sends to the origin. Each request that CloudFront sends to the origin includes the following:</p> <ul> <li> <p>The request body and the URL path (without the domain name) from the viewer request.</p> </li> <li> <p>The headers that CloudFront automatically includes in every origin request, including <code>Host</code>, <code>User-Agent</code>, and <code>X-Amz-Cf-Id</code>.</p> </li> <li> <p>All HTTP headers, cookies, and URL query strings that are specified in the cache policy or the origin request policy. These can include items from the viewer request and, in the case of headers, additional ones that are added by CloudFront.</p> </li> </ul> <p>CloudFront sends a request when it can’t find a valid object in its cache that matches the request. If you want to send values to the origin and also include them in the cache key, use <code>CachePolicy</code>.</p> <p>For more information about origin request policies, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-origin-requests.html">Controlling origin requests</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn create_origin_request_policy(
        &self,
        input: CreateOriginRequestPolicyRequest,
    ) -> Result<CreateOriginRequestPolicyResult, RusotoError<CreateOriginRequestPolicyError>> {
        let request_uri = "/2020-05-31/origin-request-policy";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        OriginRequestPolicyConfigSerializer::serialize(
            &mut writer,
            "OriginRequestPolicyConfig",
            &input.origin_request_policy_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, CreateOriginRequestPolicyError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreateOriginRequestPolicyResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag");
        result.location = response.headers.remove("Location"); // parse non-payload
        Ok(result)
    }

    /// <p>Uploads a public key to CloudFront that you can use with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">signed URLs and signed cookies</a>, or with <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/field-level-encryption.html">field-level encryption</a>.</p>
    #[allow(unused_variables, warnings)]
    async fn create_public_key(
        &self,
        input: CreatePublicKeyRequest,
    ) -> Result<CreatePublicKeyResult, RusotoError<CreatePublicKeyError>> {
        let request_uri = "/2020-05-31/public-key";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        PublicKeyConfigSerializer::serialize(
            &mut writer,
            "PublicKeyConfig",
            &input.public_key_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, CreatePublicKeyError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreatePublicKeyResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag");
        result.location = response.headers.remove("Location"); // parse non-payload
        Ok(result)
    }

    /// <p>Creates a real-time log configuration.</p> <p>After you create a real-time log configuration, you can attach it to one or more cache behaviors to send real-time log data to the specified Amazon Kinesis data stream.</p> <p>For more information about real-time log configurations, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html">Real-time logs</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn create_realtime_log_config(
        &self,
        input: CreateRealtimeLogConfigRequest,
    ) -> Result<CreateRealtimeLogConfigResult, RusotoError<CreateRealtimeLogConfigError>> {
        let request_uri = "/2020-05-31/realtime-log-config";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CreateRealtimeLogConfigRequestSerializer::serialize(
            &mut writer,
            "CreateRealtimeLogConfigRequest",
            &input,
            "http://cloudfront.amazonaws.com/doc/2020-05-31/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, CreateRealtimeLogConfigError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreateRealtimeLogConfigResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Creates a new RTMP distribution. An RTMP distribution is similar to a web distribution, but an RTMP distribution streams media files using the Adobe Real-Time Messaging Protocol (RTMP) instead of serving files using HTTP. </p> <p>To create a new distribution, submit a <code>POST</code> request to the <i>CloudFront API version</i>/distribution resource. The request body must include a document with a <i>StreamingDistributionConfig</i> element. The response echoes the <code>StreamingDistributionConfig</code> element and returns other information about the RTMP distribution.</p> <p>To get the status of your request, use the <i>GET StreamingDistribution</i> API action. When the value of <code>Enabled</code> is <code>true</code> and the value of <code>Status</code> is <code>Deployed</code>, your distribution is ready. A distribution usually deploys in less than 15 minutes.</p> <p>For more information about web distributions, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-rtmp.html">Working with RTMP Distributions</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <important> <p>Beginning with the 2012-05-05 version of the CloudFront API, we made substantial changes to the format of the XML document that you include in the request body when you create or update a web distribution or an RTMP distribution, and when you invalidate objects. With previous versions of the API, we discovered that it was too easy to accidentally delete one or more values for an element that accepts multiple values, for example, CNAMEs and trusted signers. Our changes for the 2012-05-05 release are intended to prevent these accidental deletions and to notify you when there&#39;s a mismatch between the number of values you say you&#39;re specifying in the <code>Quantity</code> element and the number of values specified.</p> </important></p>
    #[allow(unused_variables, warnings)]
    async fn create_streaming_distribution(
        &self,
        input: CreateStreamingDistributionRequest,
    ) -> Result<CreateStreamingDistributionResult, RusotoError<CreateStreamingDistributionError>>
    {
        let request_uri = "/2020-05-31/streaming-distribution";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        StreamingDistributionConfigSerializer::serialize(
            &mut writer,
            "StreamingDistributionConfig",
            &input.streaming_distribution_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, CreateStreamingDistributionError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreateStreamingDistributionResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag");
        result.location = response.headers.remove("Location"); // parse non-payload
        Ok(result)
    }

    /// <p>Create a new streaming distribution with tags.</p>
    #[allow(unused_variables, warnings)]
    async fn create_streaming_distribution_with_tags(
        &self,
        input: CreateStreamingDistributionWithTagsRequest,
    ) -> Result<
        CreateStreamingDistributionWithTagsResult,
        RusotoError<CreateStreamingDistributionWithTagsError>,
    > {
        let request_uri = "/2020-05-31/streaming-distribution";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("WithTags");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        StreamingDistributionConfigWithTagsSerializer::serialize(
            &mut writer,
            "StreamingDistributionConfigWithTags",
            &input.streaming_distribution_config_with_tags,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(
                request,
                CreateStreamingDistributionWithTagsError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            CreateStreamingDistributionWithTagsResultDeserializer::deserialize(
                actual_tag_name,
                stack,
            )
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag");
        result.location = response.headers.remove("Location"); // parse non-payload
        Ok(result)
    }

    /// <p>Deletes a cache policy.</p> <p>You cannot delete a cache policy if it’s attached to a cache behavior. First update your distributions to remove the cache policy from all cache behaviors, then delete the cache policy.</p> <p>To delete a cache policy, you must provide the policy’s identifier and version. To get these values, you can use <code>ListCachePolicies</code> or <code>GetCachePolicy</code>.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_cache_policy(
        &self,
        input: DeleteCachePolicyRequest,
    ) -> Result<(), RusotoError<DeleteCachePolicyError>> {
        let request_uri = format!("/2020-05-31/cache-policy/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut response = self
            .sign_and_dispatch(request, DeleteCachePolicyError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Delete an origin access identity. </p>
    #[allow(unused_variables, warnings)]
    async fn delete_cloud_front_origin_access_identity(
        &self,
        input: DeleteCloudFrontOriginAccessIdentityRequest,
    ) -> Result<(), RusotoError<DeleteCloudFrontOriginAccessIdentityError>> {
        let request_uri = format!(
            "/2020-05-31/origin-access-identity/cloudfront/{id}",
            id = input.id
        );

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut response = self
            .sign_and_dispatch(
                request,
                DeleteCloudFrontOriginAccessIdentityError::from_response,
            )
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Delete a distribution. </p>
    #[allow(unused_variables, warnings)]
    async fn delete_distribution(
        &self,
        input: DeleteDistributionRequest,
    ) -> Result<(), RusotoError<DeleteDistributionError>> {
        let request_uri = format!("/2020-05-31/distribution/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut response = self
            .sign_and_dispatch(request, DeleteDistributionError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Remove a field-level encryption configuration.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_field_level_encryption_config(
        &self,
        input: DeleteFieldLevelEncryptionConfigRequest,
    ) -> Result<(), RusotoError<DeleteFieldLevelEncryptionConfigError>> {
        let request_uri = format!("/2020-05-31/field-level-encryption/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut response = self
            .sign_and_dispatch(
                request,
                DeleteFieldLevelEncryptionConfigError::from_response,
            )
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Remove a field-level encryption profile.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_field_level_encryption_profile(
        &self,
        input: DeleteFieldLevelEncryptionProfileRequest,
    ) -> Result<(), RusotoError<DeleteFieldLevelEncryptionProfileError>> {
        let request_uri = format!(
            "/2020-05-31/field-level-encryption-profile/{id}",
            id = input.id
        );

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut response = self
            .sign_and_dispatch(
                request,
                DeleteFieldLevelEncryptionProfileError::from_response,
            )
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes a key group.</p> <p>You cannot delete a key group that is referenced in a cache behavior. First update your distributions to remove the key group from all cache behaviors, then delete the key group.</p> <p>To delete a key group, you must provide the key group’s identifier and version. To get these values, use <code>ListKeyGroups</code> followed by <code>GetKeyGroup</code> or <code>GetKeyGroupConfig</code>.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_key_group(
        &self,
        input: DeleteKeyGroupRequest,
    ) -> Result<(), RusotoError<DeleteKeyGroupError>> {
        let request_uri = format!("/2020-05-31/key-group/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut response = self
            .sign_and_dispatch(request, DeleteKeyGroupError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Disables additional CloudWatch metrics for the specified CloudFront distribution.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_monitoring_subscription(
        &self,
        input: DeleteMonitoringSubscriptionRequest,
    ) -> Result<DeleteMonitoringSubscriptionResult, RusotoError<DeleteMonitoringSubscriptionError>>
    {
        let request_uri = format!(
            "/2020-05-31/distributions/{distribution_id}/monitoring-subscription",
            distribution_id = input.distribution_id
        );

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, DeleteMonitoringSubscriptionError::from_response)
            .await?;

        let result = DeleteMonitoringSubscriptionResult::default();
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes an origin request policy.</p> <p>You cannot delete an origin request policy if it’s attached to any cache behaviors. First update your distributions to remove the origin request policy from all cache behaviors, then delete the origin request policy.</p> <p>To delete an origin request policy, you must provide the policy’s identifier and version. To get the identifier, you can use <code>ListOriginRequestPolicies</code> or <code>GetOriginRequestPolicy</code>.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_origin_request_policy(
        &self,
        input: DeleteOriginRequestPolicyRequest,
    ) -> Result<(), RusotoError<DeleteOriginRequestPolicyError>> {
        let request_uri = format!("/2020-05-31/origin-request-policy/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut response = self
            .sign_and_dispatch(request, DeleteOriginRequestPolicyError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Remove a public key you previously added to CloudFront.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_public_key(
        &self,
        input: DeletePublicKeyRequest,
    ) -> Result<(), RusotoError<DeletePublicKeyError>> {
        let request_uri = format!("/2020-05-31/public-key/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut response = self
            .sign_and_dispatch(request, DeletePublicKeyError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes a real-time log configuration.</p> <p>You cannot delete a real-time log configuration if it’s attached to a cache behavior. First update your distributions to remove the real-time log configuration from all cache behaviors, then delete the real-time log configuration.</p> <p>To delete a real-time log configuration, you can provide the configuration’s name or its Amazon Resource Name (ARN). You must provide at least one. If you provide both, CloudFront uses the name to identify the real-time log configuration to delete.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_realtime_log_config(
        &self,
        input: DeleteRealtimeLogConfigRequest,
    ) -> Result<(), RusotoError<DeleteRealtimeLogConfigError>> {
        let request_uri = "/2020-05-31/delete-realtime-log-config/";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        DeleteRealtimeLogConfigRequestSerializer::serialize(
            &mut writer,
            "DeleteRealtimeLogConfigRequest",
            &input,
            "http://cloudfront.amazonaws.com/doc/2020-05-31/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, DeleteRealtimeLogConfigError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Delete a streaming distribution. To delete an RTMP distribution using the CloudFront API, perform the following steps.</p> <p> <b>To delete an RTMP distribution using the CloudFront API</b>:</p> <ol> <li> <p>Disable the RTMP distribution.</p> </li> <li> <p>Submit a <code>GET Streaming Distribution Config</code> request to get the current configuration and the <code>Etag</code> header for the distribution. </p> </li> <li> <p>Update the XML document that was returned in the response to your <code>GET Streaming Distribution Config</code> request to change the value of <code>Enabled</code> to <code>false</code>.</p> </li> <li> <p>Submit a <code>PUT Streaming Distribution Config</code> request to update the configuration for your distribution. In the request body, include the XML document that you updated in Step 3. Then set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Streaming Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to the <code>PUT Streaming Distribution Config</code> request to confirm that the distribution was successfully disabled.</p> </li> <li> <p>Submit a <code>GET Streaming Distribution Config</code> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> </li> <li> <p>Submit a <code>DELETE Streaming Distribution</code> request. Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Streaming Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to your <code>DELETE Streaming Distribution</code> request to confirm that the distribution was successfully deleted.</p> </li> </ol> <p>For information about deleting a distribution using the CloudFront console, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/HowToDeleteDistribution.html">Deleting a Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_streaming_distribution(
        &self,
        input: DeleteStreamingDistributionRequest,
    ) -> Result<(), RusotoError<DeleteStreamingDistributionError>> {
        let request_uri = format!("/2020-05-31/streaming-distribution/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut response = self
            .sign_and_dispatch(request, DeleteStreamingDistributionError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Gets a cache policy, including the following metadata:</p> <ul> <li> <p>The policy’s identifier.</p> </li> <li> <p>The date and time when the policy was last modified.</p> </li> </ul> <p>To get a cache policy, you must provide the policy’s identifier. If the cache policy is attached to a distribution’s cache behavior, you can get the policy’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the cache policy is not attached to a cache behavior, you can get the identifier using <code>ListCachePolicies</code>.</p>
    #[allow(unused_variables, warnings)]
    async fn get_cache_policy(
        &self,
        input: GetCachePolicyRequest,
    ) -> Result<GetCachePolicyResult, RusotoError<GetCachePolicyError>> {
        let request_uri = format!("/2020-05-31/cache-policy/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetCachePolicyError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetCachePolicyResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Gets a cache policy configuration.</p> <p>To get a cache policy configuration, you must provide the policy’s identifier. If the cache policy is attached to a distribution’s cache behavior, you can get the policy’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the cache policy is not attached to a cache behavior, you can get the identifier using <code>ListCachePolicies</code>.</p>
    #[allow(unused_variables, warnings)]
    async fn get_cache_policy_config(
        &self,
        input: GetCachePolicyConfigRequest,
    ) -> Result<GetCachePolicyConfigResult, RusotoError<GetCachePolicyConfigError>> {
        let request_uri = format!("/2020-05-31/cache-policy/{id}/config", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetCachePolicyConfigError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetCachePolicyConfigResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Get the information about an origin access identity. </p>
    #[allow(unused_variables, warnings)]
    async fn get_cloud_front_origin_access_identity(
        &self,
        input: GetCloudFrontOriginAccessIdentityRequest,
    ) -> Result<
        GetCloudFrontOriginAccessIdentityResult,
        RusotoError<GetCloudFrontOriginAccessIdentityError>,
    > {
        let request_uri = format!(
            "/2020-05-31/origin-access-identity/cloudfront/{id}",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(
                request,
                GetCloudFrontOriginAccessIdentityError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetCloudFrontOriginAccessIdentityResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Get the configuration information about an origin access identity. </p>
    #[allow(unused_variables, warnings)]
    async fn get_cloud_front_origin_access_identity_config(
        &self,
        input: GetCloudFrontOriginAccessIdentityConfigRequest,
    ) -> Result<
        GetCloudFrontOriginAccessIdentityConfigResult,
        RusotoError<GetCloudFrontOriginAccessIdentityConfigError>,
    > {
        let request_uri = format!(
            "/2020-05-31/origin-access-identity/cloudfront/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(
                request,
                GetCloudFrontOriginAccessIdentityConfigError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetCloudFrontOriginAccessIdentityConfigResultDeserializer::deserialize(
                actual_tag_name,
                stack,
            )
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Get the information about a distribution.</p>
    #[allow(unused_variables, warnings)]
    async fn get_distribution(
        &self,
        input: GetDistributionRequest,
    ) -> Result<GetDistributionResult, RusotoError<GetDistributionError>> {
        let request_uri = format!("/2020-05-31/distribution/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetDistributionError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetDistributionResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Get the configuration information about a distribution. </p>
    #[allow(unused_variables, warnings)]
    async fn get_distribution_config(
        &self,
        input: GetDistributionConfigRequest,
    ) -> Result<GetDistributionConfigResult, RusotoError<GetDistributionConfigError>> {
        let request_uri = format!("/2020-05-31/distribution/{id}/config", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetDistributionConfigError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetDistributionConfigResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Get the field-level encryption configuration information.</p>
    #[allow(unused_variables, warnings)]
    async fn get_field_level_encryption(
        &self,
        input: GetFieldLevelEncryptionRequest,
    ) -> Result<GetFieldLevelEncryptionResult, RusotoError<GetFieldLevelEncryptionError>> {
        let request_uri = format!("/2020-05-31/field-level-encryption/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetFieldLevelEncryptionError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetFieldLevelEncryptionResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Get the field-level encryption configuration information.</p>
    #[allow(unused_variables, warnings)]
    async fn get_field_level_encryption_config(
        &self,
        input: GetFieldLevelEncryptionConfigRequest,
    ) -> Result<GetFieldLevelEncryptionConfigResult, RusotoError<GetFieldLevelEncryptionConfigError>>
    {
        let request_uri = format!(
            "/2020-05-31/field-level-encryption/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetFieldLevelEncryptionConfigError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetFieldLevelEncryptionConfigResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Get the field-level encryption profile information.</p>
    #[allow(unused_variables, warnings)]
    async fn get_field_level_encryption_profile(
        &self,
        input: GetFieldLevelEncryptionProfileRequest,
    ) -> Result<
        GetFieldLevelEncryptionProfileResult,
        RusotoError<GetFieldLevelEncryptionProfileError>,
    > {
        let request_uri = format!(
            "/2020-05-31/field-level-encryption-profile/{id}",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetFieldLevelEncryptionProfileError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetFieldLevelEncryptionProfileResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Get the field-level encryption profile configuration information.</p>
    #[allow(unused_variables, warnings)]
    async fn get_field_level_encryption_profile_config(
        &self,
        input: GetFieldLevelEncryptionProfileConfigRequest,
    ) -> Result<
        GetFieldLevelEncryptionProfileConfigResult,
        RusotoError<GetFieldLevelEncryptionProfileConfigError>,
    > {
        let request_uri = format!(
            "/2020-05-31/field-level-encryption-profile/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(
                request,
                GetFieldLevelEncryptionProfileConfigError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetFieldLevelEncryptionProfileConfigResultDeserializer::deserialize(
                actual_tag_name,
                stack,
            )
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Get the information about an invalidation. </p>
    #[allow(unused_variables, warnings)]
    async fn get_invalidation(
        &self,
        input: GetInvalidationRequest,
    ) -> Result<GetInvalidationResult, RusotoError<GetInvalidationError>> {
        let request_uri = format!(
            "/2020-05-31/distribution/{distribution_id}/invalidation/{id}",
            distribution_id = input.distribution_id,
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetInvalidationError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetInvalidationResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets a key group, including the date and time when the key group was last modified.</p> <p>To get a key group, you must provide the key group’s identifier. If the key group is referenced in a distribution’s cache behavior, you can get the key group’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the key group is not referenced in a cache behavior, you can get the identifier using <code>ListKeyGroups</code>.</p>
    #[allow(unused_variables, warnings)]
    async fn get_key_group(
        &self,
        input: GetKeyGroupRequest,
    ) -> Result<GetKeyGroupResult, RusotoError<GetKeyGroupError>> {
        let request_uri = format!("/2020-05-31/key-group/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetKeyGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetKeyGroupResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Gets a key group configuration.</p> <p>To get a key group configuration, you must provide the key group’s identifier. If the key group is referenced in a distribution’s cache behavior, you can get the key group’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the key group is not referenced in a cache behavior, you can get the identifier using <code>ListKeyGroups</code>.</p>
    #[allow(unused_variables, warnings)]
    async fn get_key_group_config(
        &self,
        input: GetKeyGroupConfigRequest,
    ) -> Result<GetKeyGroupConfigResult, RusotoError<GetKeyGroupConfigError>> {
        let request_uri = format!("/2020-05-31/key-group/{id}/config", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetKeyGroupConfigError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetKeyGroupConfigResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about whether additional CloudWatch metrics are enabled for the specified CloudFront distribution.</p>
    #[allow(unused_variables, warnings)]
    async fn get_monitoring_subscription(
        &self,
        input: GetMonitoringSubscriptionRequest,
    ) -> Result<GetMonitoringSubscriptionResult, RusotoError<GetMonitoringSubscriptionError>> {
        let request_uri = format!(
            "/2020-05-31/distributions/{distribution_id}/monitoring-subscription",
            distribution_id = input.distribution_id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetMonitoringSubscriptionError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetMonitoringSubscriptionResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets an origin request policy, including the following metadata:</p> <ul> <li> <p>The policy’s identifier.</p> </li> <li> <p>The date and time when the policy was last modified.</p> </li> </ul> <p>To get an origin request policy, you must provide the policy’s identifier. If the origin request policy is attached to a distribution’s cache behavior, you can get the policy’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the origin request policy is not attached to a cache behavior, you can get the identifier using <code>ListOriginRequestPolicies</code>.</p>
    #[allow(unused_variables, warnings)]
    async fn get_origin_request_policy(
        &self,
        input: GetOriginRequestPolicyRequest,
    ) -> Result<GetOriginRequestPolicyResult, RusotoError<GetOriginRequestPolicyError>> {
        let request_uri = format!("/2020-05-31/origin-request-policy/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetOriginRequestPolicyError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetOriginRequestPolicyResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Gets an origin request policy configuration.</p> <p>To get an origin request policy configuration, you must provide the policy’s identifier. If the origin request policy is attached to a distribution’s cache behavior, you can get the policy’s identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the origin request policy is not attached to a cache behavior, you can get the identifier using <code>ListOriginRequestPolicies</code>.</p>
    #[allow(unused_variables, warnings)]
    async fn get_origin_request_policy_config(
        &self,
        input: GetOriginRequestPolicyConfigRequest,
    ) -> Result<GetOriginRequestPolicyConfigResult, RusotoError<GetOriginRequestPolicyConfigError>>
    {
        let request_uri = format!(
            "/2020-05-31/origin-request-policy/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetOriginRequestPolicyConfigError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetOriginRequestPolicyConfigResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Gets a public key.</p>
    #[allow(unused_variables, warnings)]
    async fn get_public_key(
        &self,
        input: GetPublicKeyRequest,
    ) -> Result<GetPublicKeyResult, RusotoError<GetPublicKeyError>> {
        let request_uri = format!("/2020-05-31/public-key/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetPublicKeyError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetPublicKeyResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Gets a public key configuration.</p>
    #[allow(unused_variables, warnings)]
    async fn get_public_key_config(
        &self,
        input: GetPublicKeyConfigRequest,
    ) -> Result<GetPublicKeyConfigResult, RusotoError<GetPublicKeyConfigError>> {
        let request_uri = format!("/2020-05-31/public-key/{id}/config", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetPublicKeyConfigError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetPublicKeyConfigResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Gets a real-time log configuration.</p> <p>To get a real-time log configuration, you can provide the configuration’s name or its Amazon Resource Name (ARN). You must provide at least one. If you provide both, CloudFront uses the name to identify the real-time log configuration to get.</p>
    #[allow(unused_variables, warnings)]
    async fn get_realtime_log_config(
        &self,
        input: GetRealtimeLogConfigRequest,
    ) -> Result<GetRealtimeLogConfigResult, RusotoError<GetRealtimeLogConfigError>> {
        let request_uri = "/2020-05-31/get-realtime-log-config/";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        GetRealtimeLogConfigRequestSerializer::serialize(
            &mut writer,
            "GetRealtimeLogConfigRequest",
            &input,
            "http://cloudfront.amazonaws.com/doc/2020-05-31/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, GetRealtimeLogConfigError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetRealtimeLogConfigResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about a specified RTMP distribution, including the distribution configuration.</p>
    #[allow(unused_variables, warnings)]
    async fn get_streaming_distribution(
        &self,
        input: GetStreamingDistributionRequest,
    ) -> Result<GetStreamingDistributionResult, RusotoError<GetStreamingDistributionError>> {
        let request_uri = format!("/2020-05-31/streaming-distribution/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetStreamingDistributionError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetStreamingDistributionResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Get the configuration information about a streaming distribution. </p>
    #[allow(unused_variables, warnings)]
    async fn get_streaming_distribution_config(
        &self,
        input: GetStreamingDistributionConfigRequest,
    ) -> Result<
        GetStreamingDistributionConfigResult,
        RusotoError<GetStreamingDistributionConfigError>,
    > {
        let request_uri = format!(
            "/2020-05-31/streaming-distribution/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .sign_and_dispatch(request, GetStreamingDistributionConfigError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            GetStreamingDistributionConfigResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Gets a list of cache policies.</p> <p>You can optionally apply a filter to return only the managed policies created by AWS, or only the custom policies created in your AWS account.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
    #[allow(unused_variables, warnings)]
    async fn list_cache_policies(
        &self,
        input: ListCachePoliciesRequest,
    ) -> Result<ListCachePoliciesResult, RusotoError<ListCachePoliciesError>> {
        let request_uri = "/2020-05-31/cache-policy";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        if let Some(ref x) = input.type_ {
            params.put("Type", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(request, ListCachePoliciesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListCachePoliciesResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists origin access identities.</p>
    #[allow(unused_variables, warnings)]
    async fn list_cloud_front_origin_access_identities(
        &self,
        input: ListCloudFrontOriginAccessIdentitiesRequest,
    ) -> Result<
        ListCloudFrontOriginAccessIdentitiesResult,
        RusotoError<ListCloudFrontOriginAccessIdentitiesError>,
    > {
        let request_uri = "/2020-05-31/origin-access-identity/cloudfront";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(
                request,
                ListCloudFrontOriginAccessIdentitiesError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListCloudFrontOriginAccessIdentitiesResultDeserializer::deserialize(
                actual_tag_name,
                stack,
            )
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>List CloudFront distributions.</p>
    #[allow(unused_variables, warnings)]
    async fn list_distributions(
        &self,
        input: ListDistributionsRequest,
    ) -> Result<ListDistributionsResult, RusotoError<ListDistributionsError>> {
        let request_uri = "/2020-05-31/distribution";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(request, ListDistributionsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListDistributionsResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets a list of distribution IDs for distributions that have a cache behavior that’s associated with the specified cache policy.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
    #[allow(unused_variables, warnings)]
    async fn list_distributions_by_cache_policy_id(
        &self,
        input: ListDistributionsByCachePolicyIdRequest,
    ) -> Result<
        ListDistributionsByCachePolicyIdResult,
        RusotoError<ListDistributionsByCachePolicyIdError>,
    > {
        let request_uri = format!(
            "/2020-05-31/distributionsByCachePolicyId/{cache_policy_id}",
            cache_policy_id = input.cache_policy_id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(
                request,
                ListDistributionsByCachePolicyIdError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListDistributionsByCachePolicyIdResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets a list of distribution IDs for distributions that have a cache behavior that references the specified key group.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
    #[allow(unused_variables, warnings)]
    async fn list_distributions_by_key_group(
        &self,
        input: ListDistributionsByKeyGroupRequest,
    ) -> Result<ListDistributionsByKeyGroupResult, RusotoError<ListDistributionsByKeyGroupError>>
    {
        let request_uri = format!(
            "/2020-05-31/distributionsByKeyGroupId/{key_group_id}",
            key_group_id = input.key_group_id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(request, ListDistributionsByKeyGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListDistributionsByKeyGroupResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets a list of distribution IDs for distributions that have a cache behavior that’s associated with the specified origin request policy.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
    #[allow(unused_variables, warnings)]
    async fn list_distributions_by_origin_request_policy_id(
        &self,
        input: ListDistributionsByOriginRequestPolicyIdRequest,
    ) -> Result<
        ListDistributionsByOriginRequestPolicyIdResult,
        RusotoError<ListDistributionsByOriginRequestPolicyIdError>,
    > {
        let request_uri = format!(
            "/2020-05-31/distributionsByOriginRequestPolicyId/{origin_request_policy_id}",
            origin_request_policy_id = input.origin_request_policy_id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(
                request,
                ListDistributionsByOriginRequestPolicyIdError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListDistributionsByOriginRequestPolicyIdResultDeserializer::deserialize(
                actual_tag_name,
                stack,
            )
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets a list of distributions that have a cache behavior that’s associated with the specified real-time log configuration.</p> <p>You can specify the real-time log configuration by its name or its Amazon Resource Name (ARN). You must provide at least one. If you provide both, CloudFront uses the name to identify the real-time log configuration to list distributions for.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request. </p>
    #[allow(unused_variables, warnings)]
    async fn list_distributions_by_realtime_log_config(
        &self,
        input: ListDistributionsByRealtimeLogConfigRequest,
    ) -> Result<
        ListDistributionsByRealtimeLogConfigResult,
        RusotoError<ListDistributionsByRealtimeLogConfigError>,
    > {
        let request_uri = "/2020-05-31/distributionsByRealtimeLogConfig/";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        ListDistributionsByRealtimeLogConfigRequestSerializer::serialize(
            &mut writer,
            "ListDistributionsByRealtimeLogConfigRequest",
            &input,
            "http://cloudfront.amazonaws.com/doc/2020-05-31/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(
                request,
                ListDistributionsByRealtimeLogConfigError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListDistributionsByRealtimeLogConfigResultDeserializer::deserialize(
                actual_tag_name,
                stack,
            )
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>List the distributions that are associated with a specified AWS WAF web ACL. </p>
    #[allow(unused_variables, warnings)]
    async fn list_distributions_by_web_acl_id(
        &self,
        input: ListDistributionsByWebACLIdRequest,
    ) -> Result<ListDistributionsByWebACLIdResult, RusotoError<ListDistributionsByWebACLIdError>>
    {
        let request_uri = format!(
            "/2020-05-31/distributionsByWebACLId/{web_acl_id}",
            web_acl_id = input.web_acl_id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(request, ListDistributionsByWebACLIdError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListDistributionsByWebACLIdResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>List all field-level encryption configurations that have been created in CloudFront for this account.</p>
    #[allow(unused_variables, warnings)]
    async fn list_field_level_encryption_configs(
        &self,
        input: ListFieldLevelEncryptionConfigsRequest,
    ) -> Result<
        ListFieldLevelEncryptionConfigsResult,
        RusotoError<ListFieldLevelEncryptionConfigsError>,
    > {
        let request_uri = "/2020-05-31/field-level-encryption";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(request, ListFieldLevelEncryptionConfigsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListFieldLevelEncryptionConfigsResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Request a list of field-level encryption profiles that have been created in CloudFront for this account.</p>
    #[allow(unused_variables, warnings)]
    async fn list_field_level_encryption_profiles(
        &self,
        input: ListFieldLevelEncryptionProfilesRequest,
    ) -> Result<
        ListFieldLevelEncryptionProfilesResult,
        RusotoError<ListFieldLevelEncryptionProfilesError>,
    > {
        let request_uri = "/2020-05-31/field-level-encryption-profile";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(
                request,
                ListFieldLevelEncryptionProfilesError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListFieldLevelEncryptionProfilesResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists invalidation batches. </p>
    #[allow(unused_variables, warnings)]
    async fn list_invalidations(
        &self,
        input: ListInvalidationsRequest,
    ) -> Result<ListInvalidationsResult, RusotoError<ListInvalidationsError>> {
        let request_uri = format!(
            "/2020-05-31/distribution/{distribution_id}/invalidation",
            distribution_id = input.distribution_id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(request, ListInvalidationsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListInvalidationsResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets a list of key groups.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
    #[allow(unused_variables, warnings)]
    async fn list_key_groups(
        &self,
        input: ListKeyGroupsRequest,
    ) -> Result<ListKeyGroupsResult, RusotoError<ListKeyGroupsError>> {
        let request_uri = "/2020-05-31/key-group";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(request, ListKeyGroupsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListKeyGroupsResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets a list of origin request policies.</p> <p>You can optionally apply a filter to return only the managed policies created by AWS, or only the custom policies created in your AWS account.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
    #[allow(unused_variables, warnings)]
    async fn list_origin_request_policies(
        &self,
        input: ListOriginRequestPoliciesRequest,
    ) -> Result<ListOriginRequestPoliciesResult, RusotoError<ListOriginRequestPoliciesError>> {
        let request_uri = "/2020-05-31/origin-request-policy";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        if let Some(ref x) = input.type_ {
            params.put("Type", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(request, ListOriginRequestPoliciesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListOriginRequestPoliciesResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>List all public keys that have been added to CloudFront for this account.</p>
    #[allow(unused_variables, warnings)]
    async fn list_public_keys(
        &self,
        input: ListPublicKeysRequest,
    ) -> Result<ListPublicKeysResult, RusotoError<ListPublicKeysError>> {
        let request_uri = "/2020-05-31/public-key";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(request, ListPublicKeysError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListPublicKeysResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets a list of real-time log configurations.</p> <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request. </p>
    #[allow(unused_variables, warnings)]
    async fn list_realtime_log_configs(
        &self,
        input: ListRealtimeLogConfigsRequest,
    ) -> Result<ListRealtimeLogConfigsResult, RusotoError<ListRealtimeLogConfigsError>> {
        let request_uri = "/2020-05-31/realtime-log-config";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(request, ListRealtimeLogConfigsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListRealtimeLogConfigsResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>List streaming distributions. </p>
    #[allow(unused_variables, warnings)]
    async fn list_streaming_distributions(
        &self,
        input: ListStreamingDistributionsRequest,
    ) -> Result<ListStreamingDistributionsResult, RusotoError<ListStreamingDistributionsError>>
    {
        let request_uri = "/2020-05-31/streaming-distribution";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(request, ListStreamingDistributionsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListStreamingDistributionsResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>List tags for a CloudFront resource.</p>
    #[allow(unused_variables, warnings)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResult, RusotoError<ListTagsForResourceError>> {
        let request_uri = "/2020-05-31/tagging";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("Resource", &input.resource);
        request.set_params(params);

        let mut response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            ListTagsForResourceResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Add tags to a CloudFront resource.</p>
    #[allow(unused_variables, warnings)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = "/2020-05-31/tagging";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("Resource", &input.resource);
        params.put("Operation", "Tag");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        TagsSerializer::serialize(&mut writer, "Tags", &input.tags);
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Remove tags from a CloudFront resource.</p>
    #[allow(unused_variables, warnings)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = "/2020-05-31/tagging";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("Resource", &input.resource);
        params.put("Operation", "Untag");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        TagKeysSerializer::serialize(&mut writer, "TagKeys", &input.tag_keys);
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Updates a cache policy configuration.</p> <p>When you update a cache policy configuration, all the fields are updated with the values provided in the request. You cannot update some fields independent of others. To update a cache policy configuration:</p> <ol> <li> <p>Use <code>GetCachePolicyConfig</code> to get the current configuration.</p> </li> <li> <p>Locally modify the fields in the cache policy configuration that you want to update.</p> </li> <li> <p>Call <code>UpdateCachePolicy</code> by providing the entire cache policy configuration, including the fields that you modified and those that you didn’t.</p> </li> </ol></p>
    #[allow(unused_variables, warnings)]
    async fn update_cache_policy(
        &self,
        input: UpdateCachePolicyRequest,
    ) -> Result<UpdateCachePolicyResult, RusotoError<UpdateCachePolicyError>> {
        let request_uri = format!("/2020-05-31/cache-policy/{id}", id = input.id);

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut writer = EventWriter::new(Vec::new());
        CachePolicyConfigSerializer::serialize(
            &mut writer,
            "CachePolicyConfig",
            &input.cache_policy_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, UpdateCachePolicyError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            UpdateCachePolicyResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Update an origin access identity. </p>
    #[allow(unused_variables, warnings)]
    async fn update_cloud_front_origin_access_identity(
        &self,
        input: UpdateCloudFrontOriginAccessIdentityRequest,
    ) -> Result<
        UpdateCloudFrontOriginAccessIdentityResult,
        RusotoError<UpdateCloudFrontOriginAccessIdentityError>,
    > {
        let request_uri = format!(
            "/2020-05-31/origin-access-identity/cloudfront/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut writer = EventWriter::new(Vec::new());
        CloudFrontOriginAccessIdentityConfigSerializer::serialize(
            &mut writer,
            "CloudFrontOriginAccessIdentityConfig",
            &input.cloud_front_origin_access_identity_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(
                request,
                UpdateCloudFrontOriginAccessIdentityError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            UpdateCloudFrontOriginAccessIdentityResultDeserializer::deserialize(
                actual_tag_name,
                stack,
            )
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p><p>Updates the configuration for a web distribution. </p> <important> <p>When you update a distribution, there are more required fields than when you create a distribution. When you update your distribution by using this API action, follow the steps here to get the current configuration and then make your updates, to make sure that you include all of the required fields. To view a summary, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-overview-required-fields.html">Required Fields for Create Distribution and Update Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </important> <p>The update process includes getting the current distribution configuration, updating the XML document that is returned to make your changes, and then submitting an <code>UpdateDistribution</code> request to make the updates.</p> <p>For information about updating a distribution using the CloudFront console instead, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-creating-console.html">Creating a Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p> <b>To update a web distribution using the CloudFront API</b> </p> <ol> <li> <p>Submit a <a href="https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_GetDistributionConfig.html">GetDistributionConfig</a> request to get the current configuration and an <code>Etag</code> header for the distribution.</p> <note> <p>If you update the distribution again, you must get a new <code>Etag</code> header.</p> </note> </li> <li> <p>Update the XML document that was returned in the response to your <code>GetDistributionConfig</code> request to include your changes. </p> <important> <p>When you edit the XML file, be aware of the following:</p> <ul> <li> <p>You must strip out the ETag parameter that is returned.</p> </li> <li> <p>Additional fields are required when you update a distribution. There may be fields included in the XML file for features that you haven&#39;t configured for your distribution. This is expected and required to successfully update the distribution.</p> </li> <li> <p>You can&#39;t change the value of <code>CallerReference</code>. If you try to change this value, CloudFront returns an <code>IllegalUpdate</code> error. </p> </li> <li> <p>The new configuration replaces the existing configuration; the values that you specify in an <code>UpdateDistribution</code> request are not merged into your existing configuration. When you add, delete, or replace values in an element that allows multiple values (for example, <code>CNAME</code>), you must specify all of the values that you want to appear in the updated distribution. In addition, you must update the corresponding <code>Quantity</code> element.</p> </li> </ul> </important> </li> <li> <p>Submit an <code>UpdateDistribution</code> request to update the configuration for your distribution:</p> <ul> <li> <p>In the request body, include the XML document that you updated in Step 2. The request body must include an XML document with a <code>DistributionConfig</code> element.</p> </li> <li> <p>Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GetDistributionConfig</code> request in Step 1.</p> </li> </ul> </li> <li> <p>Review the response to the <code>UpdateDistribution</code> request to confirm that the configuration was successfully updated.</p> </li> <li> <p>Optional: Submit a <a href="https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_GetDistribution.html">GetDistribution</a> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> </li> </ol></p>
    #[allow(unused_variables, warnings)]
    async fn update_distribution(
        &self,
        input: UpdateDistributionRequest,
    ) -> Result<UpdateDistributionResult, RusotoError<UpdateDistributionError>> {
        let request_uri = format!("/2020-05-31/distribution/{id}/config", id = input.id);

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut writer = EventWriter::new(Vec::new());
        DistributionConfigSerializer::serialize(
            &mut writer,
            "DistributionConfig",
            &input.distribution_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, UpdateDistributionError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            UpdateDistributionResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Update a field-level encryption configuration. </p>
    #[allow(unused_variables, warnings)]
    async fn update_field_level_encryption_config(
        &self,
        input: UpdateFieldLevelEncryptionConfigRequest,
    ) -> Result<
        UpdateFieldLevelEncryptionConfigResult,
        RusotoError<UpdateFieldLevelEncryptionConfigError>,
    > {
        let request_uri = format!(
            "/2020-05-31/field-level-encryption/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut writer = EventWriter::new(Vec::new());
        FieldLevelEncryptionConfigSerializer::serialize(
            &mut writer,
            "FieldLevelEncryptionConfig",
            &input.field_level_encryption_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(
                request,
                UpdateFieldLevelEncryptionConfigError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            UpdateFieldLevelEncryptionConfigResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Update a field-level encryption profile. </p>
    #[allow(unused_variables, warnings)]
    async fn update_field_level_encryption_profile(
        &self,
        input: UpdateFieldLevelEncryptionProfileRequest,
    ) -> Result<
        UpdateFieldLevelEncryptionProfileResult,
        RusotoError<UpdateFieldLevelEncryptionProfileError>,
    > {
        let request_uri = format!(
            "/2020-05-31/field-level-encryption-profile/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut writer = EventWriter::new(Vec::new());
        FieldLevelEncryptionProfileConfigSerializer::serialize(
            &mut writer,
            "FieldLevelEncryptionProfileConfig",
            &input.field_level_encryption_profile_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(
                request,
                UpdateFieldLevelEncryptionProfileError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            UpdateFieldLevelEncryptionProfileResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p><p>Updates a key group.</p> <p>When you update a key group, all the fields are updated with the values provided in the request. You cannot update some fields independent of others. To update a key group:</p> <ol> <li> <p>Get the current key group with <code>GetKeyGroup</code> or <code>GetKeyGroupConfig</code>.</p> </li> <li> <p>Locally modify the fields in the key group that you want to update. For example, add or remove public key IDs.</p> </li> <li> <p>Call <code>UpdateKeyGroup</code> with the entire key group object, including the fields that you modified and those that you didn’t.</p> </li> </ol></p>
    #[allow(unused_variables, warnings)]
    async fn update_key_group(
        &self,
        input: UpdateKeyGroupRequest,
    ) -> Result<UpdateKeyGroupResult, RusotoError<UpdateKeyGroupError>> {
        let request_uri = format!("/2020-05-31/key-group/{id}", id = input.id);

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut writer = EventWriter::new(Vec::new());
        KeyGroupConfigSerializer::serialize(&mut writer, "KeyGroupConfig", &input.key_group_config);
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, UpdateKeyGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            UpdateKeyGroupResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p><p>Updates an origin request policy configuration.</p> <p>When you update an origin request policy configuration, all the fields are updated with the values provided in the request. You cannot update some fields independent of others. To update an origin request policy configuration:</p> <ol> <li> <p>Use <code>GetOriginRequestPolicyConfig</code> to get the current configuration.</p> </li> <li> <p>Locally modify the fields in the origin request policy configuration that you want to update.</p> </li> <li> <p>Call <code>UpdateOriginRequestPolicy</code> by providing the entire origin request policy configuration, including the fields that you modified and those that you didn’t.</p> </li> </ol></p>
    #[allow(unused_variables, warnings)]
    async fn update_origin_request_policy(
        &self,
        input: UpdateOriginRequestPolicyRequest,
    ) -> Result<UpdateOriginRequestPolicyResult, RusotoError<UpdateOriginRequestPolicyError>> {
        let request_uri = format!("/2020-05-31/origin-request-policy/{id}", id = input.id);

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut writer = EventWriter::new(Vec::new());
        OriginRequestPolicyConfigSerializer::serialize(
            &mut writer,
            "OriginRequestPolicyConfig",
            &input.origin_request_policy_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, UpdateOriginRequestPolicyError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            UpdateOriginRequestPolicyResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Update public key information. Note that the only value you can change is the comment.</p>
    #[allow(unused_variables, warnings)]
    async fn update_public_key(
        &self,
        input: UpdatePublicKeyRequest,
    ) -> Result<UpdatePublicKeyResult, RusotoError<UpdatePublicKeyError>> {
        let request_uri = format!("/2020-05-31/public-key/{id}/config", id = input.id);

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut writer = EventWriter::new(Vec::new());
        PublicKeyConfigSerializer::serialize(
            &mut writer,
            "PublicKeyConfig",
            &input.public_key_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, UpdatePublicKeyError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            UpdatePublicKeyResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
        Ok(result)
    }

    /// <p>Updates a real-time log configuration.</p> <p>When you update a real-time log configuration, all the parameters are updated with the values provided in the request. You cannot update some parameters independent of others. To update a real-time log configuration:</p> <ol> <li> <p>Call <code>GetRealtimeLogConfig</code> to get the current real-time log configuration.</p> </li> <li> <p>Locally modify the parameters in the real-time log configuration that you want to update.</p> </li> <li> <p>Call this API (<code>UpdateRealtimeLogConfig</code>) by providing the entire real-time log configuration, including the parameters that you modified and those that you didn’t.</p> </li> </ol> <p>You cannot update a real-time log configuration’s <code>Name</code> or <code>ARN</code>.</p>
    #[allow(unused_variables, warnings)]
    async fn update_realtime_log_config(
        &self,
        input: UpdateRealtimeLogConfigRequest,
    ) -> Result<UpdateRealtimeLogConfigResult, RusotoError<UpdateRealtimeLogConfigError>> {
        let request_uri = "/2020-05-31/realtime-log-config/";

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        UpdateRealtimeLogConfigRequestSerializer::serialize(
            &mut writer,
            "UpdateRealtimeLogConfigRequest",
            &input,
            "http://cloudfront.amazonaws.com/doc/2020-05-31/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, UpdateRealtimeLogConfigError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            UpdateRealtimeLogConfigResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        // parse non-payload
        Ok(result)
    }

    /// <p>Update a streaming distribution. </p>
    #[allow(unused_variables, warnings)]
    async fn update_streaming_distribution(
        &self,
        input: UpdateStreamingDistributionRequest,
    ) -> Result<UpdateStreamingDistributionResult, RusotoError<UpdateStreamingDistributionError>>
    {
        let request_uri = format!(
            "/2020-05-31/streaming-distribution/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        request.add_optional_header("If-Match", input.if_match.as_ref());

        let mut writer = EventWriter::new(Vec::new());
        StreamingDistributionConfigSerializer::serialize(
            &mut writer,
            "StreamingDistributionConfig",
            &input.streaming_distribution_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .sign_and_dispatch(request, UpdateStreamingDistributionError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            UpdateStreamingDistributionResultDeserializer::deserialize(actual_tag_name, stack)
        })
        .await?;
        let mut result = result;
        result.e_tag = response.headers.remove("ETag"); // parse non-payload
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
    async fn test_parse_valid_cloudfront_get_cloud_front_origin_access_identity() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-get-cloud-front-origin-access-identity.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFrontClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetCloudFrontOriginAccessIdentityRequest::default();
        let result = client.get_cloud_front_origin_access_identity(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_cloudfront_get_distribution() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-get-distribution.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFrontClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetDistributionRequest::default();
        let result = client.get_distribution(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_cloudfront_get_invalidation() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-get-invalidation.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFrontClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetInvalidationRequest::default();
        let result = client.get_invalidation(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_cloudfront_get_streaming_distribution() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-get-streaming-distribution.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFrontClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetStreamingDistributionRequest::default();
        let result = client.get_streaming_distribution(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_cloudfront_list_cloud_front_origin_access_identities() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-list-cloud-front-origin-access-identities.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFrontClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListCloudFrontOriginAccessIdentitiesRequest::default();
        let result = client
            .list_cloud_front_origin_access_identities(request)
            .await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_cloudfront_list_distributions() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-list-distributions.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFrontClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListDistributionsRequest::default();
        let result = client.list_distributions(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_cloudfront_list_invalidations() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-list-invalidations.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFrontClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListInvalidationsRequest::default();
        let result = client.list_invalidations(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[tokio::test]
    async fn test_parse_valid_cloudfront_list_streaming_distributions() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-list-streaming-distributions.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFrontClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListStreamingDistributionsRequest::default();
        let result = client.list_streaming_distributions(request).await;
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
