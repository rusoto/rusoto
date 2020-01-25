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
use std::io::Write;
use std::str::FromStr;
use xml;
use xml::reader::ParserConfig;
use xml::EventReader;
use xml::EventWriter;

/// <p>A complex type that lists the AWS accounts, if any, that you included in the <code>TrustedSigners</code> complex type for this distribution. These are the accounts that you want to allow to create signed URLs for private content.</p> <p>The <code>Signer</code> complex type lists the AWS account number of the trusted signer or <code>self</code> if the signer is the AWS account that created the distribution. The <code>Signer</code> element also includes the IDs of any active CloudFront key pairs that are associated with the trusted signer's AWS account. If no <code>KeyPairId</code> element appears for a <code>Signer</code>, that signer can't create signed URLs. </p> <p>For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ActiveTrustedSigners {
    /// <p>Enabled is <code>true</code> if any of the AWS accounts listed in the <code>TrustedSigners</code> complex type for this RTMP distribution have active CloudFront key pairs. If not, <code>Enabled</code> is <code>false</code>.</p> <p>For more information, see <a>ActiveTrustedSigners</a>.</p>
    pub enabled: bool,
    /// <p>A complex type that contains one <code>Signer</code> complex type for each trusted signer that is specified in the <code>TrustedSigners</code> complex type.</p> <p>For more information, see <a>ActiveTrustedSigners</a>. </p>
    pub items: Option<Vec<Signer>>,
    /// <p>A complex type that contains one <code>Signer</code> complex type for each trusted signer specified in the <code>TrustedSigners</code> complex type.</p> <p>For more information, see <a>ActiveTrustedSigners</a>.</p>
    pub quantity: i64,
}

struct ActiveTrustedSignersDeserializer;
impl ActiveTrustedSignersDeserializer {
    #[allow(unused_variables)]
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
struct AliasListDeserializer;
impl AliasListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Aliases {
    /// <p>A complex type that contains the CNAME aliases, if any, that you want to associate with this distribution.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of CNAME aliases, if any, that you want to associate with this distribution.</p>
    pub quantity: i64,
}

struct AliasesDeserializer;
impl AliasesDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that controls which HTTP methods CloudFront processes and forwards to your Amazon S3 bucket or your custom origin. There are three choices:</p> <ul> <li> <p>CloudFront forwards only <code>GET</code> and <code>HEAD</code> requests.</p> </li> <li> <p>CloudFront forwards only <code>GET</code>, <code>HEAD</code>, and <code>OPTIONS</code> requests.</p> </li> <li> <p>CloudFront forwards <code>GET, HEAD, OPTIONS, PUT, PATCH, POST</code>, and <code>DELETE</code> requests.</p> </li> </ul> <p>If you pick the third choice, you may need to restrict access to your Amazon S3 bucket or to your custom origin so users can't perform operations that you don't want them to. For example, you might not want users to have permissions to delete objects from your origin.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AllowedMethods {
    pub cached_methods: Option<CachedMethods>,
    /// <p>A complex type that contains the HTTP methods that you want CloudFront to process and forward to your origin.</p>
    pub items: Vec<String>,
    /// <p>The number of HTTP methods that you want CloudFront to forward to your origin. Valid values are 2 (for <code>GET</code> and <code>HEAD</code> requests), 3 (for <code>GET</code>, <code>HEAD</code>, and <code>OPTIONS</code> requests) and 7 (for <code>GET, HEAD, OPTIONS, PUT, PATCH, POST</code>, and <code>DELETE</code> requests).</p>
    pub quantity: i64,
}

struct AllowedMethodsDeserializer;
impl AllowedMethodsDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AwsAccountNumberListDeserializer;
impl AwsAccountNumberListDeserializer {
    #[allow(unused_variables)]
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

struct BooleanDeserializer;
impl BooleanDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that describes how CloudFront processes requests.</p> <p>You must create at least as many cache behaviors (including the default cache behavior) as you have origins if you want CloudFront to distribute objects from all of the origins. Each cache behavior specifies the one origin from which you want CloudFront to get objects. If you have two origins and only the default cache behavior, the default cache behavior will cause CloudFront to get objects from one of the origins, but the other origin is never used.</p> <p>For the current limit on the number of cache behaviors that you can add to a distribution, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_cloudfront">Amazon CloudFront Limits</a> in the <i>AWS General Reference</i>.</p> <p>If you don't want to specify any cache behaviors, include only an empty <code>CacheBehaviors</code> element. Don't include an empty <code>CacheBehavior</code> element, or CloudFront returns a <code>MalformedXML</code> error.</p> <p>To delete all cache behaviors in an existing distribution, update the distribution configuration and include only an empty <code>CacheBehaviors</code> element.</p> <p>To add, change, or remove one or more cache behaviors, update the distribution configuration and specify all of the cache behaviors that you want to include in the updated distribution.</p> <p>For more information about cache behaviors, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesCacheBehavior">Cache Behaviors</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CacheBehavior {
    pub allowed_methods: Option<AllowedMethods>,
    /// <p>Whether you want CloudFront to automatically compress certain files for this cache behavior. If so, specify true; if not, specify false. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/ServingCompressedFiles.html">Serving Compressed Files</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub compress: Option<bool>,
    /// <p>The default amount of time that you want objects to stay in CloudFront caches before CloudFront forwards another request to your origin to determine whether the object has been updated. The value that you specify applies only when your origin does not add HTTP headers such as <code>Cache-Control max-age</code>, <code>Cache-Control s-maxage</code>, and <code>Expires</code> to objects. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Specifying How Long Objects and Errors Stay in a CloudFront Edge Cache (Expiration)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub default_ttl: Option<i64>,
    /// <p>The value of <code>ID</code> for the field-level encryption configuration that you want CloudFront to use for encrypting specific fields of data for a cache behavior or for the default cache behavior in your distribution.</p>
    pub field_level_encryption_id: Option<String>,
    /// <p>A complex type that specifies how CloudFront handles query strings and cookies.</p>
    pub forwarded_values: ForwardedValues,
    /// <p>A complex type that contains zero or more Lambda function associations for a cache behavior.</p>
    pub lambda_function_associations: Option<LambdaFunctionAssociations>,
    /// <p>The maximum amount of time that you want objects to stay in CloudFront caches before CloudFront forwards another request to your origin to determine whether the object has been updated. The value that you specify applies only when your origin adds HTTP headers such as <code>Cache-Control max-age</code>, <code>Cache-Control s-maxage</code>, and <code>Expires</code> to objects. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Specifying How Long Objects and Errors Stay in a CloudFront Edge Cache (Expiration)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub max_ttl: Option<i64>,
    /// <p>The minimum amount of time that you want objects to stay in CloudFront caches before CloudFront forwards another request to your origin to determine whether the object has been updated. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Specifying How Long Objects and Errors Stay in a CloudFront Edge Cache (Expiration)</a> in the <i>Amazon Amazon CloudFront Developer Guide</i>.</p> <p>You must specify <code>0</code> for <code>MinTTL</code> if you configure CloudFront to forward all headers to your origin (under <code>Headers</code>, if you specify <code>1</code> for <code>Quantity</code> and <code>*</code> for <code>Name</code>).</p>
    pub min_ttl: i64,
    /// <p>The pattern (for example, <code>images/*.jpg</code>) that specifies which requests to apply the behavior to. When CloudFront receives a viewer request, the requested path is compared with path patterns in the order in which cache behaviors are listed in the distribution.</p> <note> <p>You can optionally include a slash (<code>/</code>) at the beginning of the path pattern. For example, <code>/images/*.jpg</code>. CloudFront behavior is the same with or without the leading <code>/</code>.</p> </note> <p>The path pattern for the default cache behavior is <code>*</code> and cannot be changed. If the request for an object does not match the path pattern for any cache behaviors, CloudFront applies the behavior in the default cache behavior.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesPathPattern">Path Pattern</a> in the <i> Amazon CloudFront Developer Guide</i>.</p>
    pub path_pattern: String,
    /// <p>Indicates whether you want to distribute media files in the Microsoft Smooth Streaming format using the origin that is associated with this cache behavior. If so, specify <code>true</code>; if not, specify <code>false</code>. If you specify <code>true</code> for <code>SmoothStreaming</code>, you can still distribute other content using this cache behavior if the content matches the value of <code>PathPattern</code>. </p>
    pub smooth_streaming: Option<bool>,
    /// <p>The value of <code>ID</code> for the origin that you want CloudFront to route requests to when a request matches the path pattern either for a cache behavior or for the default cache behavior in your distribution.</p>
    pub target_origin_id: String,
    /// <p>A complex type that specifies the AWS accounts, if any, that you want to allow to create signed URLs for private content.</p> <p>If you want to require signed URLs in requests for objects in the target origin that match the <code>PathPattern</code> for this cache behavior, specify <code>true</code> for <code>Enabled</code>, and specify the applicable values for <code>Quantity</code> and <code>Items</code>. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon Amazon CloudFront Developer Guide</i>.</p> <p>If you don't want to require signed URLs in requests for objects that match <code>PathPattern</code>, specify <code>false</code> for <code>Enabled</code> and <code>0</code> for <code>Quantity</code>. Omit <code>Items</code>.</p> <p>To add, change, or remove one or more trusted signers, change <code>Enabled</code> to <code>true</code> (if it's currently <code>false</code>), change <code>Quantity</code> as applicable, and specify all of the trusted signers that you want to include in the updated distribution.</p>
    pub trusted_signers: TrustedSigners,
    /// <p><p>The protocol that viewers can use to access the files in the origin specified by <code>TargetOriginId</code> when a request matches the path pattern in <code>PathPattern</code>. You can specify the following options:</p> <ul> <li> <p> <code>allow-all</code>: Viewers can use HTTP or HTTPS.</p> </li> <li> <p> <code>redirect-to-https</code>: If a viewer submits an HTTP request, CloudFront returns an HTTP status code of 301 (Moved Permanently) to the viewer along with the HTTPS URL. The viewer then resubmits the request using the new URL. </p> </li> <li> <p> <code>https-only</code>: If a viewer sends an HTTP request, CloudFront returns an HTTP status code of 403 (Forbidden). </p> </li> </ul> <p>For more information about requiring the HTTPS protocol, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/SecureConnections.html">Using an HTTPS Connection to Access Your Objects</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <note> <p>The only way to guarantee that viewers retrieve an object that was fetched from the origin using HTTPS is never to use any other protocol to fetch the object. If you have recently changed from HTTP to HTTPS, we recommend that you clear your objects&#39; cache because cached objects are protocol agnostic. That means that an edge location will return an object from the cache regardless of whether the current request protocol matches the protocol used previously. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Specifying How Long Objects and Errors Stay in a CloudFront Edge Cache (Expiration)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </note></p>
    pub viewer_protocol_policy: String,
}

struct CacheBehaviorDeserializer;
impl CacheBehaviorDeserializer {
    #[allow(unused_variables)]
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
                "Compress" => {
                    obj.compress = Some(BooleanDeserializer::deserialize("Compress", stack)?);
                }
                "DefaultTTL" => {
                    obj.default_ttl = Some(LongDeserializer::deserialize("DefaultTTL", stack)?);
                }
                "FieldLevelEncryptionId" => {
                    obj.field_level_encryption_id = Some(StringDeserializer::deserialize(
                        "FieldLevelEncryptionId",
                        stack,
                    )?);
                }
                "ForwardedValues" => {
                    obj.forwarded_values =
                        ForwardedValuesDeserializer::deserialize("ForwardedValues", stack)?;
                }
                "LambdaFunctionAssociations" => {
                    obj.lambda_function_associations =
                        Some(LambdaFunctionAssociationsDeserializer::deserialize(
                            "LambdaFunctionAssociations",
                            stack,
                        )?);
                }
                "MaxTTL" => {
                    obj.max_ttl = Some(LongDeserializer::deserialize("MaxTTL", stack)?);
                }
                "MinTTL" => {
                    obj.min_ttl = LongDeserializer::deserialize("MinTTL", stack)?;
                }
                "PathPattern" => {
                    obj.path_pattern = StringDeserializer::deserialize("PathPattern", stack)?;
                }
                "SmoothStreaming" => {
                    obj.smooth_streaming =
                        Some(BooleanDeserializer::deserialize("SmoothStreaming", stack)?);
                }
                "TargetOriginId" => {
                    obj.target_origin_id =
                        StringDeserializer::deserialize("TargetOriginId", stack)?;
                }
                "TrustedSigners" => {
                    obj.trusted_signers =
                        TrustedSignersDeserializer::deserialize("TrustedSigners", stack)?;
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
        if let Some(ref value) = obj.compress {
            writer.write(xml::writer::XmlEvent::start_element("Compress"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.default_ttl {
            writer.write(xml::writer::XmlEvent::start_element("DefaultTTL"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.field_level_encryption_id {
            writer.write(xml::writer::XmlEvent::start_element(
                "FieldLevelEncryptionId",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        ForwardedValuesSerializer::serialize(
            &mut writer,
            "ForwardedValues",
            &obj.forwarded_values,
        )?;
        if let Some(ref value) = obj.lambda_function_associations {
            &LambdaFunctionAssociationsSerializer::serialize(
                &mut writer,
                "LambdaFunctionAssociations",
                value,
            )?;
        }
        if let Some(ref value) = obj.max_ttl {
            writer.write(xml::writer::XmlEvent::start_element("MaxTTL"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("MinTTL"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.min_ttl
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("PathPattern"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.path_pattern
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.smooth_streaming {
            writer.write(xml::writer::XmlEvent::start_element("SmoothStreaming"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("TargetOriginId"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.target_origin_id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        TrustedSignersSerializer::serialize(&mut writer, "TrustedSigners", &obj.trusted_signers)?;
        writer.write(xml::writer::XmlEvent::start_element("ViewerProtocolPolicy"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.viewer_protocol_policy
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct CacheBehaviorListDeserializer;
impl CacheBehaviorListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CacheBehaviors {
    /// <p>Optional: A complex type that contains cache behaviors for this distribution. If <code>Quantity</code> is <code>0</code>, you can omit <code>Items</code>.</p>
    pub items: Option<Vec<CacheBehavior>>,
    /// <p>The number of cache behaviors for this distribution. </p>
    pub quantity: i64,
}

struct CacheBehaviorsDeserializer;
impl CacheBehaviorsDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that controls whether CloudFront caches the response to requests using the specified HTTP methods. There are two choices:</p> <ul> <li> <p>CloudFront caches responses to <code>GET</code> and <code>HEAD</code> requests.</p> </li> <li> <p>CloudFront caches responses to <code>GET</code>, <code>HEAD</code>, and <code>OPTIONS</code> requests.</p> </li> </ul> <p>If you pick the second choice for your Amazon S3 Origin, you may need to forward Access-Control-Request-Method, Access-Control-Request-Headers, and Origin headers for the responses to be cached correctly. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CachedMethods {
    /// <p>A complex type that contains the HTTP methods that you want CloudFront to cache responses to.</p>
    pub items: Vec<String>,
    /// <p>The number of HTTP methods for which you want CloudFront to cache responses. Valid values are <code>2</code> (for caching responses to <code>GET</code> and <code>HEAD</code> requests) and <code>3</code> (for caching responses to <code>GET</code>, <code>HEAD</code>, and <code>OPTIONS</code> requests).</p>
    pub quantity: i64,
}

struct CachedMethodsDeserializer;
impl CachedMethodsDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>CloudFront origin access identity.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CloudFrontOriginAccessIdentity {
    /// <p>The current configuration information for the identity. </p>
    pub cloud_front_origin_access_identity_config: Option<CloudFrontOriginAccessIdentityConfig>,
    /// <p>The ID for the origin access identity, for example, <code>E74FTE3AJFJ256A</code>. </p>
    pub id: String,
    /// <p>The Amazon S3 canonical user ID for the origin access identity, used when giving the origin access identity read permission to an object in Amazon S3. </p>
    pub s3_canonical_user_id: String,
}

struct CloudFrontOriginAccessIdentityDeserializer;
impl CloudFrontOriginAccessIdentityDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CloudFrontOriginAccessIdentityConfig {
    /// <p>A unique value (for example, a date-time stamp) that ensures that the request can't be replayed.</p> <p>If the value of <code>CallerReference</code> is new (regardless of the content of the <code>CloudFrontOriginAccessIdentityConfig</code> object), a new origin access identity is created.</p> <p>If the <code>CallerReference</code> is a value already sent in a previous identity request, and the content of the <code>CloudFrontOriginAccessIdentityConfig</code> is identical to the original request (ignoring white space), the response includes the same information returned to the original request. </p> <p>If the <code>CallerReference</code> is a value you already sent in a previous request to create an identity, but the content of the <code>CloudFrontOriginAccessIdentityConfig</code> is different from the original request, CloudFront returns a <code>CloudFrontOriginAccessIdentityAlreadyExists</code> error. </p>
    pub caller_reference: String,
    /// <p>Any comments you want to include about the origin access identity. </p>
    pub comment: String,
}

struct CloudFrontOriginAccessIdentityConfigDeserializer;
impl CloudFrontOriginAccessIdentityConfigDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("CallerReference"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.caller_reference
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Comment"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.comment
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Lists the origin access identities for CloudFront.Send a <code>GET</code> request to the <code>/<i>CloudFront API version</i>/origin-access-identity/cloudfront</code> resource. The response includes a <code>CloudFrontOriginAccessIdentityList</code> element with zero or more <code>CloudFrontOriginAccessIdentitySummary</code> child elements. By default, your entire list of origin access identities is returned in one single page. If the list is long, you can paginate it using the <code>MaxItems</code> and <code>Marker</code> parameters.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

struct CloudFrontOriginAccessIdentityListDeserializer;
impl CloudFrontOriginAccessIdentityListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CloudFrontOriginAccessIdentitySummary {
    /// <p>The comment for this origin access identity, as originally specified when created.</p>
    pub comment: String,
    /// <p>The ID for the origin access identity. For example: <code>E74FTE3AJFJ256A</code>.</p>
    pub id: String,
    /// <p>The Amazon S3 canonical user ID for the origin access identity, which you use when giving the origin access identity read permission to an object in Amazon S3.</p>
    pub s3_canonical_user_id: String,
}

struct CloudFrontOriginAccessIdentitySummaryDeserializer;
impl CloudFrontOriginAccessIdentitySummaryDeserializer {
    #[allow(unused_variables)]
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
struct CloudFrontOriginAccessIdentitySummaryListDeserializer;
impl CloudFrontOriginAccessIdentitySummaryListDeserializer {
    #[allow(unused_variables)]
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
/// <p>A field-level encryption content type profile. </p>
#[derive(Default, Debug, Clone, PartialEq)]
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

struct ContentTypeProfileDeserializer;
impl ContentTypeProfileDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("ContentType"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.content_type
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Format"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.format
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.profile_id {
            writer.write(xml::writer::XmlEvent::start_element("ProfileId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>The configuration for a field-level encryption content type-profile mapping. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ContentTypeProfileConfig {
    /// <p>The configuration for a field-level encryption content type-profile. </p>
    pub content_type_profiles: Option<ContentTypeProfiles>,
    /// <p>The setting in a field-level encryption content type-profile mapping that specifies what to do when an unknown content type is provided for the profile. If true, content is forwarded without being encrypted when the content type is unknown. If false (the default), an error is returned when the content type is unknown. </p>
    pub forward_when_content_type_is_unknown: bool,
}

struct ContentTypeProfileConfigDeserializer;
impl ContentTypeProfileConfigDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element(
            "ForwardWhenContentTypeIsUnknown",
        ))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.forward_when_content_type_is_unknown
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ContentTypeProfileListDeserializer;
impl ContentTypeProfileListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ContentTypeProfiles {
    /// <p>Items in a field-level encryption content type-profile mapping. </p>
    pub items: Option<Vec<ContentTypeProfile>>,
    /// <p>The number of field-level encryption content type-profile mappings. </p>
    pub quantity: i64,
}

struct ContentTypeProfilesDeserializer;
impl ContentTypeProfilesDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct CookieNameListDeserializer;
impl CookieNameListDeserializer {
    #[allow(unused_variables)]
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

/// <p>A complex type that specifies whether you want CloudFront to forward cookies to the origin and, if so, which ones. For more information about forwarding cookies to the origin, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Cookies.html">How CloudFront Forwards, Caches, and Logs Cookies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CookieNames {
    /// <p>A complex type that contains one <code>Name</code> element for each cookie that you want CloudFront to forward to the origin for this cache behavior.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of different cookies that you want CloudFront to forward to the origin for this cache behavior.</p>
    pub quantity: i64,
}

struct CookieNamesDeserializer;
impl CookieNamesDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that specifies whether you want CloudFront to forward cookies to the origin and, if so, which ones. For more information about forwarding cookies to the origin, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Cookies.html">How CloudFront Forwards, Caches, and Logs Cookies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CookiePreference {
    /// <p>Specifies which cookies to forward to the origin for this cache behavior: all, none, or the list of cookies specified in the <code>WhitelistedNames</code> complex type.</p> <p>Amazon S3 doesn't process cookies. When the cache behavior is forwarding requests to an Amazon S3 origin, specify none for the <code>Forward</code> element. </p>
    pub forward: String,
    /// <p>Required if you specify <code>whitelist</code> for the value of <code>Forward:</code>. A complex type that specifies how many different cookies you want CloudFront to forward to the origin for this cache behavior and, if you want to forward selected cookies, the names of those cookies.</p> <p>If you specify <code>all</code> or none for the value of <code>Forward</code>, omit <code>WhitelistedNames</code>. If you change the value of <code>Forward</code> from <code>whitelist</code> to all or none and you don't delete the <code>WhitelistedNames</code> element and its child elements, CloudFront deletes them automatically.</p> <p>For the current limit on the number of cookie names that you can whitelist for each cache behavior, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_cloudfront">Amazon CloudFront Limits</a> in the <i>AWS General Reference</i>.</p>
    pub whitelisted_names: Option<CookieNames>,
}

struct CookiePreferenceDeserializer;
impl CookiePreferenceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CookiePreference, XmlParseError> {
        deserialize_elements::<_, CookiePreference, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Forward" => {
                    obj.forward = ItemSelectionDeserializer::deserialize("Forward", stack)?;
                }
                "WhitelistedNames" => {
                    obj.whitelisted_names = Some(CookieNamesDeserializer::deserialize(
                        "WhitelistedNames",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct CookiePreferenceSerializer;
impl CookiePreferenceSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CookiePreference,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Forward"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.forward
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.whitelisted_names {
            &CookieNamesSerializer::serialize(&mut writer, "WhitelistedNames", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>The request to create a new origin access identity.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCloudFrontOriginAccessIdentityRequest {
    /// <p>The current configuration information for the identity.</p>
    pub cloud_front_origin_access_identity_config: CloudFrontOriginAccessIdentityConfig,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateCloudFrontOriginAccessIdentityResult {
    /// <p>The origin access identity's information.</p>
    pub cloud_front_origin_access_identity: Option<CloudFrontOriginAccessIdentity>,
    /// <p>The current version of the origin access identity created.</p>
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the new origin access identity just created. For example: <code>https://cloudfront.amazonaws.com/2010-11-01/origin-access-identity/cloudfront/E74FTE3AJFJ256A</code>.</p>
    pub location: Option<String>,
}

struct CreateCloudFrontOriginAccessIdentityResultDeserializer;
impl CreateCloudFrontOriginAccessIdentityResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDistributionRequest {
    /// <p>The distribution's configuration information.</p>
    pub distribution_config: DistributionConfig,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateDistributionResult {
    /// <p>The distribution's information.</p>
    pub distribution: Option<Distribution>,
    /// <p>The current version of the distribution created.</p>
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the new distribution resource just created. For example: <code>https://cloudfront.amazonaws.com/2010-11-01/distribution/EDFDVBD632BHDS5</code>.</p>
    pub location: Option<String>,
}

struct CreateDistributionResultDeserializer;
impl CreateDistributionResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDistributionWithTagsRequest {
    /// <p>The distribution's configuration information. </p>
    pub distribution_config_with_tags: DistributionConfigWithTags,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateDistributionWithTagsResult {
    /// <p>The distribution's information. </p>
    pub distribution: Option<Distribution>,
    /// <p>The current version of the distribution created.</p>
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the new distribution resource just created. For example: <code>https://cloudfront.amazonaws.com/2010-11-01/distribution/EDFDVBD632BHDS5</code>. </p>
    pub location: Option<String>,
}

struct CreateDistributionWithTagsResultDeserializer;
impl CreateDistributionWithTagsResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFieldLevelEncryptionConfigRequest {
    /// <p>The request to create a new field-level encryption configuration.</p>
    pub field_level_encryption_config: FieldLevelEncryptionConfig,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateFieldLevelEncryptionConfigResult {
    /// <p>The current version of the field level encryption configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Returned when you create a new field-level encryption configuration.</p>
    pub field_level_encryption: Option<FieldLevelEncryption>,
    /// <p>The fully qualified URI of the new configuration resource just created. For example: <code>https://cloudfront.amazonaws.com/2010-11-01/field-level-encryption-config/EDFDVBD632BHDS5</code>.</p>
    pub location: Option<String>,
}

struct CreateFieldLevelEncryptionConfigResultDeserializer;
impl CreateFieldLevelEncryptionConfigResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFieldLevelEncryptionProfileRequest {
    /// <p>The request to create a field-level encryption profile.</p>
    pub field_level_encryption_profile_config: FieldLevelEncryptionProfileConfig,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateFieldLevelEncryptionProfileResult {
    /// <p>The current version of the field level encryption profile. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Returned when you create a new field-level encryption profile.</p>
    pub field_level_encryption_profile: Option<FieldLevelEncryptionProfile>,
    /// <p>The fully qualified URI of the new profile resource just created. For example: <code>https://cloudfront.amazonaws.com/2010-11-01/field-level-encryption-profile/EDFDVBD632BHDS5</code>.</p>
    pub location: Option<String>,
}

struct CreateFieldLevelEncryptionProfileResultDeserializer;
impl CreateFieldLevelEncryptionProfileResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInvalidationRequest {
    /// <p>The distribution's id.</p>
    pub distribution_id: String,
    /// <p>The batch information for the invalidation.</p>
    pub invalidation_batch: InvalidationBatch,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateInvalidationResult {
    /// <p>The invalidation's information.</p>
    pub invalidation: Option<Invalidation>,
    /// <p>The fully qualified URI of the distribution and invalidation batch request, including the <code>Invalidation ID</code>.</p>
    pub location: Option<String>,
}

struct CreateInvalidationResultDeserializer;
impl CreateInvalidationResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePublicKeyRequest {
    /// <p>The request to add a public key to CloudFront.</p>
    pub public_key_config: PublicKeyConfig,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreatePublicKeyResult {
    /// <p>The current version of the public key. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the new public key resource just created. For example: <code>https://cloudfront.amazonaws.com/2010-11-01/cloudfront-public-key/EDFDVBD632BHDS5</code>.</p>
    pub location: Option<String>,
    /// <p>Returned when you add a public key.</p>
    pub public_key: Option<PublicKey>,
}

struct CreatePublicKeyResultDeserializer;
impl CreatePublicKeyResultDeserializer {
    #[allow(unused_variables)]
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
/// <p>The request to create a new streaming distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateStreamingDistributionRequest {
    /// <p>The streaming distribution's configuration information.</p>
    pub streaming_distribution_config: StreamingDistributionConfig,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateStreamingDistributionResult {
    /// <p>The current version of the streaming distribution created.</p>
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the new streaming distribution resource just created. For example: <code>https://cloudfront.amazonaws.com/2010-11-01/streaming-distribution/EGTXBD79H29TRA8</code>.</p>
    pub location: Option<String>,
    /// <p>The streaming distribution's information.</p>
    pub streaming_distribution: Option<StreamingDistribution>,
}

struct CreateStreamingDistributionResultDeserializer;
impl CreateStreamingDistributionResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateStreamingDistributionWithTagsRequest {
    /// <p> The streaming distribution's configuration information. </p>
    pub streaming_distribution_config_with_tags: StreamingDistributionConfigWithTags,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateStreamingDistributionWithTagsResult {
    /// <p>The current version of the distribution created.</p>
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the new streaming distribution resource just created. For example:<code> https://cloudfront.amazonaws.com/2010-11-01/streaming-distribution/EGTXBD79H29TRA8</code>.</p>
    pub location: Option<String>,
    /// <p>The streaming distribution's information. </p>
    pub streaming_distribution: Option<StreamingDistribution>,
}

struct CreateStreamingDistributionWithTagsResultDeserializer;
impl CreateStreamingDistributionWithTagsResultDeserializer {
    #[allow(unused_variables)]
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
/// <p>A complex type that controls:</p> <ul> <li> <p>Whether CloudFront replaces HTTP status codes in the 4xx and 5xx range with custom error messages before returning the response to the viewer. </p> </li> <li> <p>How long CloudFront caches HTTP status codes in the 4xx and 5xx range.</p> </li> </ul> <p>For more information about custom error pages, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/custom-error-pages.html">Customizing Error Responses</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CustomErrorResponse {
    /// <p>The minimum amount of time, in seconds, that you want CloudFront to cache the HTTP status code specified in <code>ErrorCode</code>. When this time period has elapsed, CloudFront queries your origin to see whether the problem that caused the error has been resolved and the requested object is now available.</p> <p>If you don't want to specify a value, include an empty element, <code>&lt;ErrorCachingMinTTL&gt;</code>, in the XML document.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/custom-error-pages.html">Customizing Error Responses</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub error_caching_min_ttl: Option<i64>,
    /// <p>The HTTP status code for which you want to specify a custom error page and/or a caching duration.</p>
    pub error_code: i64,
    /// <p>The HTTP status code that you want CloudFront to return to the viewer along with the custom error page. There are a variety of reasons that you might want CloudFront to return a status code different from the status code that your origin returned to CloudFront, for example:</p> <ul> <li> <p>Some Internet devices (some firewalls and corporate proxies, for example) intercept HTTP 4xx and 5xx and prevent the response from being returned to the viewer. If you substitute <code>200</code>, the response typically won't be intercepted.</p> </li> <li> <p>If you don't care about distinguishing among different client errors or server errors, you can specify <code>400</code> or <code>500</code> as the <code>ResponseCode</code> for all 4xx or 5xx errors.</p> </li> <li> <p>You might want to return a <code>200</code> status code (OK) and static website so your customers don't know that your website is down.</p> </li> </ul> <p>If you specify a value for <code>ResponseCode</code>, you must also specify a value for <code>ResponsePagePath</code>. If you don't want to specify a value, include an empty element, <code>&lt;ResponseCode&gt;</code>, in the XML document.</p>
    pub response_code: Option<String>,
    /// <p>The path to the custom error page that you want CloudFront to return to a viewer when your origin returns the HTTP status code specified by <code>ErrorCode</code>, for example, <code>/4xx-errors/403-forbidden.html</code>. If you want to store your objects and your custom error pages in different locations, your distribution must include a cache behavior for which the following is true:</p> <ul> <li> <p>The value of <code>PathPattern</code> matches the path to your custom error messages. For example, suppose you saved custom error pages for 4xx errors in an Amazon S3 bucket in a directory named <code>/4xx-errors</code>. Your distribution must include a cache behavior for which the path pattern routes requests for your custom error pages to that location, for example, <code>/4xx-errors/*</code>. </p> </li> <li> <p>The value of <code>TargetOriginId</code> specifies the value of the <code>ID</code> element for the origin that contains your custom error pages.</p> </li> </ul> <p>If you specify a value for <code>ResponsePagePath</code>, you must also specify a value for <code>ResponseCode</code>. If you don't want to specify a value, include an empty element, <code>&lt;ResponsePagePath&gt;</code>, in the XML document.</p> <p>We recommend that you store custom error pages in an Amazon S3 bucket. If you store custom error pages on an HTTP server and the server starts to return 5xx errors, CloudFront can't get the files that you want to return to viewers because the origin server is unavailable.</p>
    pub response_page_path: Option<String>,
}

struct CustomErrorResponseDeserializer;
impl CustomErrorResponseDeserializer {
    #[allow(unused_variables)]
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
            writer.write(xml::writer::XmlEvent::start_element("ErrorCachingMinTTL"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("ErrorCode"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.error_code
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.response_code {
            writer.write(xml::writer::XmlEvent::start_element("ResponseCode"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.response_page_path {
            writer.write(xml::writer::XmlEvent::start_element("ResponsePagePath"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct CustomErrorResponseListDeserializer;
impl CustomErrorResponseListDeserializer {
    #[allow(unused_variables)]
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

/// <p>A complex type that controls:</p> <ul> <li> <p>Whether CloudFront replaces HTTP status codes in the 4xx and 5xx range with custom error messages before returning the response to the viewer.</p> </li> <li> <p>How long CloudFront caches HTTP status codes in the 4xx and 5xx range.</p> </li> </ul> <p>For more information about custom error pages, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/custom-error-pages.html">Customizing Error Responses</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CustomErrorResponses {
    /// <p>A complex type that contains a <code>CustomErrorResponse</code> element for each HTTP status code for which you want to specify a custom error page and/or a caching duration. </p>
    pub items: Option<Vec<CustomErrorResponse>>,
    /// <p>The number of HTTP status codes for which you want to specify a custom error page and/or a caching duration. If <code>Quantity</code> is <code>0</code>, you can omit <code>Items</code>.</p>
    pub quantity: i64,
}

struct CustomErrorResponsesDeserializer;
impl CustomErrorResponsesDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that contains the list of Custom Headers for each origin. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CustomHeaders {
    /// <p> <b>Optional</b>: A list that contains one <code>OriginCustomHeader</code> element for each custom header that you want CloudFront to forward to the origin. If Quantity is <code>0</code>, omit <code>Items</code>.</p>
    pub items: Option<Vec<OriginCustomHeader>>,
    /// <p>The number of custom headers, if any, for this distribution.</p>
    pub quantity: i64,
}

struct CustomHeadersDeserializer;
impl CustomHeadersDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A customer origin or an Amazon S3 bucket configured as a website endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CustomOriginConfig {
    /// <p>The HTTP port the custom origin listens on.</p>
    pub http_port: i64,
    /// <p>The HTTPS port the custom origin listens on.</p>
    pub https_port: i64,
    /// <p>You can create a custom keep-alive timeout. All timeout units are in seconds. The default keep-alive timeout is 5 seconds, but you can configure custom timeout lengths using the CloudFront API. The minimum timeout length is 1 second; the maximum is 60 seconds.</p> <p>If you need to increase the maximum time limit, contact the <a href="https://console.aws.amazon.com/support/home#/">AWS Support Center</a>.</p>
    pub origin_keepalive_timeout: Option<i64>,
    /// <p>The origin protocol policy to apply to your origin.</p>
    pub origin_protocol_policy: String,
    /// <p>You can create a custom origin read timeout. All timeout units are in seconds. The default origin read timeout is 30 seconds, but you can configure custom timeout lengths using the CloudFront API. The minimum timeout length is 4 seconds; the maximum is 60 seconds.</p> <p>If you need to increase the maximum time limit, contact the <a href="https://console.aws.amazon.com/support/home#/">AWS Support Center</a>.</p>
    pub origin_read_timeout: Option<i64>,
    /// <p>The SSL/TLS protocols that you want CloudFront to use when communicating with your origin over HTTPS.</p>
    pub origin_ssl_protocols: Option<OriginSslProtocols>,
}

struct CustomOriginConfigDeserializer;
impl CustomOriginConfigDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("HTTPPort"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.http_port
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("HTTPSPort"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.https_port
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.origin_keepalive_timeout {
            writer.write(xml::writer::XmlEvent::start_element(
                "OriginKeepaliveTimeout",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("OriginProtocolPolicy"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.origin_protocol_policy
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.origin_read_timeout {
            writer.write(xml::writer::XmlEvent::start_element("OriginReadTimeout"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.origin_ssl_protocols {
            &OriginSslProtocolsSerializer::serialize(&mut writer, "OriginSslProtocols", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that describes the default cache behavior if you don't specify a <code>CacheBehavior</code> element or if files don't match any of the values of <code>PathPattern</code> in <code>CacheBehavior</code> elements. You must create exactly one default cache behavior.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DefaultCacheBehavior {
    pub allowed_methods: Option<AllowedMethods>,
    /// <p>Whether you want CloudFront to automatically compress certain files for this cache behavior. If so, specify <code>true</code>; if not, specify <code>false</code>. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/ServingCompressedFiles.html">Serving Compressed Files</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub compress: Option<bool>,
    /// <p>The default amount of time that you want objects to stay in CloudFront caches before CloudFront forwards another request to your origin to determine whether the object has been updated. The value that you specify applies only when your origin does not add HTTP headers such as <code>Cache-Control max-age</code>, <code>Cache-Control s-maxage</code>, and <code>Expires</code> to objects. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Specifying How Long Objects and Errors Stay in a CloudFront Edge Cache (Expiration)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub default_ttl: Option<i64>,
    /// <p>The value of <code>ID</code> for the field-level encryption configuration that you want CloudFront to use for encrypting specific fields of data for a cache behavior or for the default cache behavior in your distribution.</p>
    pub field_level_encryption_id: Option<String>,
    /// <p>A complex type that specifies how CloudFront handles query strings and cookies.</p>
    pub forwarded_values: ForwardedValues,
    /// <p>A complex type that contains zero or more Lambda function associations for a cache behavior.</p>
    pub lambda_function_associations: Option<LambdaFunctionAssociations>,
    pub max_ttl: Option<i64>,
    /// <p>The minimum amount of time that you want objects to stay in CloudFront caches before CloudFront forwards another request to your origin to determine whether the object has been updated. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Specifying How Long Objects and Errors Stay in a CloudFront Edge Cache (Expiration)</a> in the <i>Amazon Amazon CloudFront Developer Guide</i>.</p> <p>You must specify <code>0</code> for <code>MinTTL</code> if you configure CloudFront to forward all headers to your origin (under <code>Headers</code>, if you specify <code>1</code> for <code>Quantity</code> and <code>*</code> for <code>Name</code>).</p>
    pub min_ttl: i64,
    /// <p>Indicates whether you want to distribute media files in the Microsoft Smooth Streaming format using the origin that is associated with this cache behavior. If so, specify <code>true</code>; if not, specify <code>false</code>. If you specify <code>true</code> for <code>SmoothStreaming</code>, you can still distribute other content using this cache behavior if the content matches the value of <code>PathPattern</code>. </p>
    pub smooth_streaming: Option<bool>,
    /// <p>The value of <code>ID</code> for the origin that you want CloudFront to route requests to when a request matches the path pattern either for a cache behavior or for the default cache behavior in your distribution.</p>
    pub target_origin_id: String,
    /// <p>A complex type that specifies the AWS accounts, if any, that you want to allow to create signed URLs for private content.</p> <p>If you want to require signed URLs in requests for objects in the target origin that match the <code>PathPattern</code> for this cache behavior, specify <code>true</code> for <code>Enabled</code>, and specify the applicable values for <code>Quantity</code> and <code>Items</code>. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon Amazon CloudFront Developer Guide</i>.</p> <p>If you don't want to require signed URLs in requests for objects that match <code>PathPattern</code>, specify <code>false</code> for <code>Enabled</code> and <code>0</code> for <code>Quantity</code>. Omit <code>Items</code>.</p> <p>To add, change, or remove one or more trusted signers, change <code>Enabled</code> to <code>true</code> (if it's currently <code>false</code>), change <code>Quantity</code> as applicable, and specify all of the trusted signers that you want to include in the updated distribution.</p>
    pub trusted_signers: TrustedSigners,
    /// <p><p>The protocol that viewers can use to access the files in the origin specified by <code>TargetOriginId</code> when a request matches the path pattern in <code>PathPattern</code>. You can specify the following options:</p> <ul> <li> <p> <code>allow-all</code>: Viewers can use HTTP or HTTPS.</p> </li> <li> <p> <code>redirect-to-https</code>: If a viewer submits an HTTP request, CloudFront returns an HTTP status code of 301 (Moved Permanently) to the viewer along with the HTTPS URL. The viewer then resubmits the request using the new URL.</p> </li> <li> <p> <code>https-only</code>: If a viewer sends an HTTP request, CloudFront returns an HTTP status code of 403 (Forbidden).</p> </li> </ul> <p>For more information about requiring the HTTPS protocol, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/SecureConnections.html">Using an HTTPS Connection to Access Your Objects</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <note> <p>The only way to guarantee that viewers retrieve an object that was fetched from the origin using HTTPS is never to use any other protocol to fetch the object. If you have recently changed from HTTP to HTTPS, we recommend that you clear your objects&#39; cache because cached objects are protocol agnostic. That means that an edge location will return an object from the cache regardless of whether the current request protocol matches the protocol used previously. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Specifying How Long Objects and Errors Stay in a CloudFront Edge Cache (Expiration)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </note></p>
    pub viewer_protocol_policy: String,
}

struct DefaultCacheBehaviorDeserializer;
impl DefaultCacheBehaviorDeserializer {
    #[allow(unused_variables)]
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
                "Compress" => {
                    obj.compress = Some(BooleanDeserializer::deserialize("Compress", stack)?);
                }
                "DefaultTTL" => {
                    obj.default_ttl = Some(LongDeserializer::deserialize("DefaultTTL", stack)?);
                }
                "FieldLevelEncryptionId" => {
                    obj.field_level_encryption_id = Some(StringDeserializer::deserialize(
                        "FieldLevelEncryptionId",
                        stack,
                    )?);
                }
                "ForwardedValues" => {
                    obj.forwarded_values =
                        ForwardedValuesDeserializer::deserialize("ForwardedValues", stack)?;
                }
                "LambdaFunctionAssociations" => {
                    obj.lambda_function_associations =
                        Some(LambdaFunctionAssociationsDeserializer::deserialize(
                            "LambdaFunctionAssociations",
                            stack,
                        )?);
                }
                "MaxTTL" => {
                    obj.max_ttl = Some(LongDeserializer::deserialize("MaxTTL", stack)?);
                }
                "MinTTL" => {
                    obj.min_ttl = LongDeserializer::deserialize("MinTTL", stack)?;
                }
                "SmoothStreaming" => {
                    obj.smooth_streaming =
                        Some(BooleanDeserializer::deserialize("SmoothStreaming", stack)?);
                }
                "TargetOriginId" => {
                    obj.target_origin_id =
                        StringDeserializer::deserialize("TargetOriginId", stack)?;
                }
                "TrustedSigners" => {
                    obj.trusted_signers =
                        TrustedSignersDeserializer::deserialize("TrustedSigners", stack)?;
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
        if let Some(ref value) = obj.compress {
            writer.write(xml::writer::XmlEvent::start_element("Compress"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.default_ttl {
            writer.write(xml::writer::XmlEvent::start_element("DefaultTTL"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.field_level_encryption_id {
            writer.write(xml::writer::XmlEvent::start_element(
                "FieldLevelEncryptionId",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        ForwardedValuesSerializer::serialize(
            &mut writer,
            "ForwardedValues",
            &obj.forwarded_values,
        )?;
        if let Some(ref value) = obj.lambda_function_associations {
            &LambdaFunctionAssociationsSerializer::serialize(
                &mut writer,
                "LambdaFunctionAssociations",
                value,
            )?;
        }
        if let Some(ref value) = obj.max_ttl {
            writer.write(xml::writer::XmlEvent::start_element("MaxTTL"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("MinTTL"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.min_ttl
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.smooth_streaming {
            writer.write(xml::writer::XmlEvent::start_element("SmoothStreaming"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("TargetOriginId"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.target_origin_id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        TrustedSignersSerializer::serialize(&mut writer, "TrustedSigners", &obj.trusted_signers)?;
        writer.write(xml::writer::XmlEvent::start_element("ViewerProtocolPolicy"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.viewer_protocol_policy
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Deletes a origin access identity.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCloudFrontOriginAccessIdentityRequest {
    /// <p>The origin access identity's ID.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header you received from a previous <code>GET</code> or <code>PUT</code> request. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

/// <p>This action deletes a web distribution. To delete a web distribution using the CloudFront API, perform the following steps.</p> <p> <b>To delete a web distribution using the CloudFront API:</b> </p> <ol> <li> <p>Disable the web distribution </p> </li> <li> <p>Submit a <code>GET Distribution Config</code> request to get the current configuration and the <code>Etag</code> header for the distribution.</p> </li> <li> <p>Update the XML document that was returned in the response to your <code>GET Distribution Config</code> request to change the value of <code>Enabled</code> to <code>false</code>.</p> </li> <li> <p>Submit a <code>PUT Distribution Config</code> request to update the configuration for your distribution. In the request body, include the XML document that you updated in Step 3. Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to the <code>PUT Distribution Config</code> request to confirm that the distribution was successfully disabled.</p> </li> <li> <p>Submit a <code>GET Distribution</code> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> </li> <li> <p>Submit a <code>DELETE Distribution</code> request. Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Distribution Config</code> request in Step 6.</p> </li> <li> <p>Review the response to your <code>DELETE Distribution</code> request to confirm that the distribution was successfully deleted.</p> </li> </ol> <p>For information about deleting a distribution using the CloudFront console, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/HowToDeleteDistribution.html">Deleting a Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDistributionRequest {
    /// <p>The distribution ID. </p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when you disabled the distribution. For example: <code>E2QWRUHAPOMQZL</code>. </p>
    pub if_match: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFieldLevelEncryptionConfigRequest {
    /// <p>The ID of the configuration you want to delete from CloudFront.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the configuration identity to delete. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFieldLevelEncryptionProfileRequest {
    /// <p>Request the ID of the profile you want to delete from CloudFront.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the profile to delete. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePublicKeyRequest {
    /// <p>The ID of the public key you want to remove from CloudFront.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the public key identity to delete. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

/// <p>The request to delete a streaming distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteStreamingDistributionRequest {
    /// <p>The distribution ID. </p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when you disabled the streaming distribution. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

/// <p>The distribution's information.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Distribution {
    /// <p>The ARN (Amazon Resource Name) for the distribution. For example: <code>arn:aws:cloudfront::123456789012:distribution/EDFDVBD632BHDS5</code>, where <code>123456789012</code> is your AWS account ID.</p>
    pub arn: String,
    /// <p>CloudFront automatically adds this element to the response only if you've set up the distribution to serve private content with signed URLs. The element lists the key pair IDs that CloudFront is aware of for each trusted signer. The <code>Signer</code> child element lists the AWS account number of the trusted signer (or an empty <code>Self</code> element if the signer is you). The <code>Signer</code> element also includes the IDs of any active key pairs associated with the trusted signer's AWS account. If no <code>KeyPairId</code> element appears for a <code>Signer</code>, that signer can't create working signed URLs.</p>
    pub active_trusted_signers: ActiveTrustedSigners,
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

struct DistributionDeserializer;
impl DistributionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Distribution, XmlParseError> {
        deserialize_elements::<_, Distribution, _>(tag_name, stack, |name, stack, obj| {
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
#[derive(Default, Debug, Clone, PartialEq)]
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
    /// <p>A complex type that controls the following:</p> <ul> <li> <p>Whether CloudFront replaces HTTP status codes in the 4xx and 5xx range with custom error messages before returning the response to the viewer.</p> </li> <li> <p>How long CloudFront caches HTTP status codes in the 4xx and 5xx range.</p> </li> </ul> <p>For more information about custom error pages, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/custom-error-pages.html">Customizing Error Responses</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub custom_error_responses: Option<CustomErrorResponses>,
    /// <p>A complex type that describes the default cache behavior if you don't specify a <code>CacheBehavior</code> element or if files don't match any of the values of <code>PathPattern</code> in <code>CacheBehavior</code> elements. You must create exactly one default cache behavior.</p>
    pub default_cache_behavior: DefaultCacheBehavior,
    /// <p>The object that you want CloudFront to request from your origin (for example, <code>index.html</code>) when a viewer requests the root URL for your distribution (<code>http://www.example.com</code>) instead of an object in your distribution (<code>http://www.example.com/product-description.html</code>). Specifying a default root object avoids exposing the contents of your distribution.</p> <p>Specify only the object name, for example, <code>index.html</code>. Don't add a <code>/</code> before the object name.</p> <p>If you don't want to specify a default root object when you create a distribution, include an empty <code>DefaultRootObject</code> element.</p> <p>To delete the default root object from an existing distribution, update the distribution configuration and include an empty <code>DefaultRootObject</code> element.</p> <p>To replace the default root object, update the distribution configuration and specify the new object.</p> <p>For more information about the default root object, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/DefaultRootObject.html">Creating a Default Root Object</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub default_root_object: Option<String>,
    /// <p>From this field, you can enable or disable the selected distribution.</p>
    pub enabled: bool,
    /// <p>(Optional) Specify the maximum HTTP version that you want viewers to use to communicate with CloudFront. The default value for new web distributions is http2. Viewers that don't support HTTP/2 automatically use an earlier HTTP version.</p> <p>For viewers and CloudFront to use HTTP/2, viewers must support TLS 1.2 or later, and must support Server Name Identification (SNI).</p> <p>In general, configuring CloudFront to communicate with viewers using HTTP/2 reduces latency. You can improve performance by optimizing for HTTP/2. For more information, do an Internet search for "http/2 optimization." </p>
    pub http_version: Option<String>,
    /// <p>If you want CloudFront to respond to IPv6 DNS requests with an IPv6 address for your distribution, specify <code>true</code>. If you specify <code>false</code>, CloudFront responds to IPv6 DNS requests with the DNS response code <code>NOERROR</code> and with no IP addresses. This allows viewers to submit a second request, for an IPv4 address for your distribution. </p> <p>In general, you should enable IPv6 if you have users on IPv6 networks who want to access your content. However, if you're using signed URLs or signed cookies to restrict access to your content, and if you're using a custom policy that includes the <code>IpAddress</code> parameter to restrict the IP addresses that can access your content, don't enable IPv6. If you want to restrict access to some content by IP address and not restrict access to other content (or restrict access but not by IP address), you can create two distributions. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-creating-signed-url-custom-policy.html">Creating a Signed URL Using a Custom Policy</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>If you're using an Amazon Route 53 alias resource record set to route traffic to your CloudFront distribution, you need to create a second alias resource record set when both of the following are true:</p> <ul> <li> <p>You enable IPv6 for the distribution</p> </li> <li> <p>You're using alternate domain names in the URLs for your objects</p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-to-cloudfront-distribution.html">Routing Traffic to an Amazon CloudFront Web Distribution by Using Your Domain Name</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If you created a CNAME resource record set, either with Amazon Route 53 or with another DNS service, you don't need to make any changes. A CNAME record will route traffic to your distribution regardless of the IP address format of the viewer request.</p>
    pub is_ipv6_enabled: Option<bool>,
    /// <p>A complex type that controls whether access logs are written for the distribution.</p> <p>For more information about logging, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/AccessLogs.html">Access Logs</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub logging: Option<LoggingConfig>,
    /// <p> A complex type that contains information about origin groups for this distribution.</p>
    pub origin_groups: Option<OriginGroups>,
    /// <p>A complex type that contains information about origins for this distribution. </p>
    pub origins: Origins,
    /// <p>The price class that corresponds with the maximum price that you want to pay for CloudFront service. If you specify <code>PriceClass_All</code>, CloudFront responds to requests for your objects from all CloudFront edge locations.</p> <p>If you specify a price class other than <code>PriceClass_All</code>, CloudFront serves your objects from the CloudFront edge location that has the lowest latency among the edge locations in your price class. Viewers who are in or near regions that are excluded from your specified price class may encounter slower performance.</p> <p>For more information about price classes, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PriceClass.html">Choosing the Price Class for a CloudFront Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>. For information about CloudFront pricing, including how price classes (such as Price Class 100) map to CloudFront regions, see <a href="https://aws.amazon.com/cloudfront/pricing/">Amazon CloudFront Pricing</a>. For price class information, scroll down to see the table at the bottom of the page.</p>
    pub price_class: Option<String>,
    /// <p><p/></p>
    pub restrictions: Option<Restrictions>,
    /// <p><p/></p>
    pub viewer_certificate: Option<ViewerCertificate>,
    /// <p>A unique identifier that specifies the AWS WAF web ACL, if any, to associate with this distribution.</p> <p>AWS WAF is a web application firewall that lets you monitor the HTTP and HTTPS requests that are forwarded to CloudFront, and lets you control access to your content. Based on conditions that you specify, such as the IP addresses that requests originate from or the values of query strings, CloudFront responds to requests either with the requested content or with an HTTP 403 status code (Forbidden). You can also configure CloudFront to return a custom error page when a request is blocked. For more information about AWS WAF, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/what-is-aws-waf.html">AWS WAF Developer Guide</a>. </p>
    pub web_acl_id: Option<String>,
}

struct DistributionConfigDeserializer;
impl DistributionConfigDeserializer {
    #[allow(unused_variables)]
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
                    obj.comment = StringDeserializer::deserialize("Comment", stack)?;
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
        writer.write(xml::writer::XmlEvent::start_element("CallerReference"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.caller_reference
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Comment"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.comment
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.custom_error_responses {
            &CustomErrorResponsesSerializer::serialize(&mut writer, "CustomErrorResponses", value)?;
        }
        DefaultCacheBehaviorSerializer::serialize(
            &mut writer,
            "DefaultCacheBehavior",
            &obj.default_cache_behavior,
        )?;
        if let Some(ref value) = obj.default_root_object {
            writer.write(xml::writer::XmlEvent::start_element("DefaultRootObject"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Enabled"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.enabled
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.http_version {
            writer.write(xml::writer::XmlEvent::start_element("HttpVersion"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.is_ipv6_enabled {
            writer.write(xml::writer::XmlEvent::start_element("IsIPV6Enabled"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.logging {
            &LoggingConfigSerializer::serialize(&mut writer, "Logging", value)?;
        }
        if let Some(ref value) = obj.origin_groups {
            &OriginGroupsSerializer::serialize(&mut writer, "OriginGroups", value)?;
        }
        OriginsSerializer::serialize(&mut writer, "Origins", &obj.origins)?;
        if let Some(ref value) = obj.price_class {
            writer.write(xml::writer::XmlEvent::start_element("PriceClass"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.restrictions {
            &RestrictionsSerializer::serialize(&mut writer, "Restrictions", value)?;
        }
        if let Some(ref value) = obj.viewer_certificate {
            &ViewerCertificateSerializer::serialize(&mut writer, "ViewerCertificate", value)?;
        }
        if let Some(ref value) = obj.web_acl_id {
            writer.write(xml::writer::XmlEvent::start_element("WebACLId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A distribution Configuration and a list of tags to be associated with the distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

/// <p>A distribution list.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

struct DistributionListDeserializer;
impl DistributionListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DistributionSummary {
    /// <p>The ARN (Amazon Resource Name) for the distribution. For example: <code>arn:aws:cloudfront::123456789012:distribution/EDFDVBD632BHDS5</code>, where <code>123456789012</code> is your AWS account ID.</p>
    pub arn: String,
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
    /// <p><p/></p>
    pub restrictions: Restrictions,
    /// <p>The current status of the distribution. When the status is <code>Deployed</code>, the distribution's information is propagated to all CloudFront edge locations.</p>
    pub status: String,
    /// <p><p/></p>
    pub viewer_certificate: ViewerCertificate,
    /// <p>The Web ACL Id (if any) associated with the distribution.</p>
    pub web_acl_id: String,
}

struct DistributionSummaryDeserializer;
impl DistributionSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DistributionSummary, XmlParseError> {
        deserialize_elements::<_, DistributionSummary, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = StringDeserializer::deserialize("ARN", stack)?;
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
struct DistributionSummaryListDeserializer;
impl DistributionSummaryListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EncryptionEntities {
    /// <p>An array of field patterns in a field-level encryption content type-profile mapping. </p>
    pub items: Option<Vec<EncryptionEntity>>,
    /// <p>Number of field pattern items in a field-level encryption content type-profile mapping. </p>
    pub quantity: i64,
}

struct EncryptionEntitiesDeserializer;
impl EncryptionEntitiesDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Complex data type for field-level encryption profiles that includes the encryption key and field pattern specifications. </p>
#[derive(Default, Debug, Clone, PartialEq)]
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

struct EncryptionEntityDeserializer;
impl EncryptionEntityDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("ProviderId"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.provider_id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("PublicKeyId"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.public_key_id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct EncryptionEntityListDeserializer;
impl EncryptionEntityListDeserializer {
    #[allow(unused_variables)]
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

struct EventTypeDeserializer;
impl EventTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex data type that includes the profile configurations and other options specified for field-level encryption. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct FieldLevelEncryption {
    /// <p>A complex data type that includes the profile configurations specified for field-level encryption. </p>
    pub field_level_encryption_config: FieldLevelEncryptionConfig,
    /// <p>The configuration ID for a field-level encryption configuration which includes a set of profiles that specify certain selected data fields to be encrypted by specific public keys.</p>
    pub id: String,
    /// <p>The last time the field-level encryption configuration was changed. </p>
    pub last_modified_time: String,
}

struct FieldLevelEncryptionDeserializer;
impl FieldLevelEncryptionDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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

struct FieldLevelEncryptionConfigDeserializer;
impl FieldLevelEncryptionConfigDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("CallerReference"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.caller_reference
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.comment {
            writer.write(xml::writer::XmlEvent::start_element("Comment"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
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
#[derive(Default, Debug, Clone, PartialEq)]
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

struct FieldLevelEncryptionListDeserializer;
impl FieldLevelEncryptionListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct FieldLevelEncryptionProfile {
    /// <p>A complex data type that includes the profile name and the encryption entities for the field-level encryption profile.</p>
    pub field_level_encryption_profile_config: FieldLevelEncryptionProfileConfig,
    /// <p>The ID for a field-level encryption profile configuration which includes a set of profiles that specify certain selected data fields to be encrypted by specific public keys.</p>
    pub id: String,
    /// <p>The last time the field-level encryption profile was updated.</p>
    pub last_modified_time: String,
}

struct FieldLevelEncryptionProfileDeserializer;
impl FieldLevelEncryptionProfileDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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

struct FieldLevelEncryptionProfileConfigDeserializer;
impl FieldLevelEncryptionProfileConfigDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("CallerReference"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.caller_reference
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.comment {
            writer.write(xml::writer::XmlEvent::start_element("Comment"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        EncryptionEntitiesSerializer::serialize(
            &mut writer,
            "EncryptionEntities",
            &obj.encryption_entities,
        )?;
        writer.write(xml::writer::XmlEvent::start_element("Name"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.name
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>List of field-level encryption profiles.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

struct FieldLevelEncryptionProfileListDeserializer;
impl FieldLevelEncryptionProfileListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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

struct FieldLevelEncryptionProfileSummaryDeserializer;
impl FieldLevelEncryptionProfileSummaryDeserializer {
    #[allow(unused_variables)]
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
struct FieldLevelEncryptionProfileSummaryListDeserializer;
impl FieldLevelEncryptionProfileSummaryListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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

struct FieldLevelEncryptionSummaryDeserializer;
impl FieldLevelEncryptionSummaryDeserializer {
    #[allow(unused_variables)]
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
struct FieldLevelEncryptionSummaryListDeserializer;
impl FieldLevelEncryptionSummaryListDeserializer {
    #[allow(unused_variables)]
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
struct FieldPatternListDeserializer;
impl FieldPatternListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FieldPatterns {
    /// <p>An array of the field-level encryption field patterns.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of field-level encryption field patterns.</p>
    pub quantity: i64,
}

struct FieldPatternsDeserializer;
impl FieldPatternsDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct FormatDeserializer;
impl FormatDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that specifies how CloudFront handles query strings and cookies.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ForwardedValues {
    /// <p>A complex type that specifies whether you want CloudFront to forward cookies to the origin and, if so, which ones. For more information about forwarding cookies to the origin, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Cookies.html">How CloudFront Forwards, Caches, and Logs Cookies</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub cookies: CookiePreference,
    /// <p>A complex type that specifies the <code>Headers</code>, if any, that you want CloudFront to base caching on for this cache behavior. </p>
    pub headers: Option<Headers>,
    /// <p>Indicates whether you want CloudFront to forward query strings to the origin that is associated with this cache behavior and cache based on the query string parameters. CloudFront behavior depends on the value of <code>QueryString</code> and on the values that you specify for <code>QueryStringCacheKeys</code>, if any:</p> <p>If you specify true for <code>QueryString</code> and you don't specify any values for <code>QueryStringCacheKeys</code>, CloudFront forwards all query string parameters to the origin and caches based on all query string parameters. Depending on how many query string parameters and values you have, this can adversely affect performance because CloudFront must forward more requests to the origin.</p> <p>If you specify true for <code>QueryString</code> and you specify one or more values for <code>QueryStringCacheKeys</code>, CloudFront forwards all query string parameters to the origin, but it only caches based on the query string parameters that you specify.</p> <p>If you specify false for <code>QueryString</code>, CloudFront doesn't forward any query string parameters to the origin, and doesn't cache based on query string parameters.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/QueryStringParameters.html">Configuring CloudFront to Cache Based on Query String Parameters</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub query_string: bool,
    /// <p>A complex type that contains information about the query string parameters that you want CloudFront to use for caching for this cache behavior.</p>
    pub query_string_cache_keys: Option<QueryStringCacheKeys>,
}

struct ForwardedValuesDeserializer;
impl ForwardedValuesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ForwardedValues, XmlParseError> {
        deserialize_elements::<_, ForwardedValues, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Cookies" => {
                    obj.cookies = CookiePreferenceDeserializer::deserialize("Cookies", stack)?;
                }
                "Headers" => {
                    obj.headers = Some(HeadersDeserializer::deserialize("Headers", stack)?);
                }
                "QueryString" => {
                    obj.query_string = BooleanDeserializer::deserialize("QueryString", stack)?;
                }
                "QueryStringCacheKeys" => {
                    obj.query_string_cache_keys =
                        Some(QueryStringCacheKeysDeserializer::deserialize(
                            "QueryStringCacheKeys",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct ForwardedValuesSerializer;
impl ForwardedValuesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ForwardedValues,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        CookiePreferenceSerializer::serialize(&mut writer, "Cookies", &obj.cookies)?;
        if let Some(ref value) = obj.headers {
            &HeadersSerializer::serialize(&mut writer, "Headers", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("QueryString"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.query_string
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.query_string_cache_keys {
            &QueryStringCacheKeysSerializer::serialize(&mut writer, "QueryStringCacheKeys", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that controls the countries in which your content is distributed. CloudFront determines the location of your users using <code>MaxMind</code> GeoIP databases. </p>
#[derive(Default, Debug, Clone, PartialEq)]
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

struct GeoRestrictionDeserializer;
impl GeoRestrictionDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("RestrictionType"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.restriction_type
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct GeoRestrictionTypeDeserializer;
impl GeoRestrictionTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>The origin access identity's configuration information. For more information, see <a>CloudFrontOriginAccessIdentityConfigComplexType</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCloudFrontOriginAccessIdentityConfigRequest {
    /// <p>The identity's ID. </p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetCloudFrontOriginAccessIdentityConfigResult {
    /// <p>The origin access identity's configuration information. </p>
    pub cloud_front_origin_access_identity_config: Option<CloudFrontOriginAccessIdentityConfig>,
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

struct GetCloudFrontOriginAccessIdentityConfigResultDeserializer;
impl GetCloudFrontOriginAccessIdentityConfigResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCloudFrontOriginAccessIdentityRequest {
    /// <p>The identity's ID.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetCloudFrontOriginAccessIdentityResult {
    /// <p>The origin access identity's information.</p>
    pub cloud_front_origin_access_identity: Option<CloudFrontOriginAccessIdentity>,
    /// <p>The current version of the origin access identity's information. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

struct GetCloudFrontOriginAccessIdentityResultDeserializer;
impl GetCloudFrontOriginAccessIdentityResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDistributionConfigRequest {
    /// <p>The distribution's ID.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetDistributionConfigResult {
    /// <p>The distribution's configuration information.</p>
    pub distribution_config: Option<DistributionConfig>,
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

struct GetDistributionConfigResultDeserializer;
impl GetDistributionConfigResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDistributionRequest {
    /// <p>The distribution's ID.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetDistributionResult {
    /// <p>The distribution's information.</p>
    pub distribution: Option<Distribution>,
    /// <p>The current version of the distribution's information. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

struct GetDistributionResultDeserializer;
impl GetDistributionResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFieldLevelEncryptionConfigRequest {
    /// <p>Request the ID for the field-level encryption configuration information.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetFieldLevelEncryptionConfigResult {
    /// <p>The current version of the field level encryption configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Return the field-level encryption configuration information.</p>
    pub field_level_encryption_config: Option<FieldLevelEncryptionConfig>,
}

struct GetFieldLevelEncryptionConfigResultDeserializer;
impl GetFieldLevelEncryptionConfigResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFieldLevelEncryptionProfileConfigRequest {
    /// <p>Get the ID for the field-level encryption profile configuration information.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetFieldLevelEncryptionProfileConfigResult {
    /// <p>The current version of the field-level encryption profile configuration result. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Return the field-level encryption profile configuration information.</p>
    pub field_level_encryption_profile_config: Option<FieldLevelEncryptionProfileConfig>,
}

struct GetFieldLevelEncryptionProfileConfigResultDeserializer;
impl GetFieldLevelEncryptionProfileConfigResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFieldLevelEncryptionProfileRequest {
    /// <p>Get the ID for the field-level encryption profile information.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetFieldLevelEncryptionProfileResult {
    /// <p>The current version of the field level encryption profile. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Return the field-level encryption profile information.</p>
    pub field_level_encryption_profile: Option<FieldLevelEncryptionProfile>,
}

struct GetFieldLevelEncryptionProfileResultDeserializer;
impl GetFieldLevelEncryptionProfileResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFieldLevelEncryptionRequest {
    /// <p>Request the ID for the field-level encryption configuration information.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetFieldLevelEncryptionResult {
    /// <p>The current version of the field level encryption configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Return the field-level encryption configuration information.</p>
    pub field_level_encryption: Option<FieldLevelEncryption>,
}

struct GetFieldLevelEncryptionResultDeserializer;
impl GetFieldLevelEncryptionResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInvalidationRequest {
    /// <p>The distribution's ID.</p>
    pub distribution_id: String,
    /// <p>The identifier for the invalidation request, for example, <code>IDFDVBD632BHDS5</code>.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetInvalidationResult {
    /// <p>The invalidation's information. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/InvalidationDatatype.html">Invalidation Complex Type</a>. </p>
    pub invalidation: Option<Invalidation>,
}

struct GetInvalidationResultDeserializer;
impl GetInvalidationResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPublicKeyConfigRequest {
    /// <p>Request the ID for the public key configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetPublicKeyConfigResult {
    /// <p>The current version of the public key configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Return the result for the public key configuration.</p>
    pub public_key_config: Option<PublicKeyConfig>,
}

struct GetPublicKeyConfigResultDeserializer;
impl GetPublicKeyConfigResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPublicKeyRequest {
    /// <p>Request the ID for the public key.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetPublicKeyResult {
    /// <p>The current version of the public key. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Return the public key.</p>
    pub public_key: Option<PublicKey>,
}

struct GetPublicKeyResultDeserializer;
impl GetPublicKeyResultDeserializer {
    #[allow(unused_variables)]
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
/// <p>To request to get a streaming distribution configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetStreamingDistributionConfigRequest {
    /// <p>The streaming distribution's ID.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetStreamingDistributionConfigResult {
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>. </p>
    pub e_tag: Option<String>,
    /// <p>The streaming distribution's configuration information.</p>
    pub streaming_distribution_config: Option<StreamingDistributionConfig>,
}

struct GetStreamingDistributionConfigResultDeserializer;
impl GetStreamingDistributionConfigResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetStreamingDistributionRequest {
    /// <p>The streaming distribution's ID.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetStreamingDistributionResult {
    /// <p>The current version of the streaming distribution's information. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>The streaming distribution's information.</p>
    pub streaming_distribution: Option<StreamingDistribution>,
}

struct GetStreamingDistributionResultDeserializer;
impl GetStreamingDistributionResultDeserializer {
    #[allow(unused_variables)]
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
struct HeaderListDeserializer;
impl HeaderListDeserializer {
    #[allow(unused_variables)]
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

/// <p>A complex type that specifies the request headers, if any, that you want CloudFront to base caching on for this cache behavior. </p> <p>For the headers that you specify, CloudFront caches separate versions of a specified object based on the header values in viewer requests. For example, suppose viewer requests for <code>logo.jpg</code> contain a custom <code>product</code> header that has a value of either <code>acme</code> or <code>apex</code>, and you configure CloudFront to cache your content based on values in the <code>product</code> header. CloudFront forwards the <code>product</code> header to the origin and caches the response from the origin once for each header value. For more information about caching based on header values, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/header-caching.html">How CloudFront Forwards and Caches Headers</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Headers {
    /// <p>A list that contains one <code>Name</code> element for each header that you want CloudFront to use for caching in this cache behavior. If <code>Quantity</code> is <code>0</code>, omit <code>Items</code>.</p>
    pub items: Option<Vec<String>>,
    /// <p><p>The number of different headers that you want CloudFront to base caching on for this cache behavior. You can configure each cache behavior in a web distribution to do one of the following:</p> <ul> <li> <p> <b>Forward all headers to your origin</b>: Specify <code>1</code> for <code>Quantity</code> and <code>*</code> for <code>Name</code>.</p> <important> <p>CloudFront doesn&#39;t cache the objects that are associated with this cache behavior. Instead, CloudFront sends every request to the origin. </p> </important> </li> <li> <p> <b>Forward a whitelist of headers you specify</b>: Specify the number of headers that you want CloudFront to base caching on. Then specify the header names in <code>Name</code> elements. CloudFront caches your objects based on the values in the specified headers.</p> </li> <li> <p> <b>Forward only the default headers</b>: Specify <code>0</code> for <code>Quantity</code> and omit <code>Items</code>. In this configuration, CloudFront doesn&#39;t cache based on the values in the request headers.</p> </li> </ul> <p>Regardless of which option you choose, CloudFront forwards headers to your origin based on whether the origin is an S3 bucket or a custom origin. See the following documentation:</p> <ul> <li> <p> <b>S3 bucket</b>: See <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/RequestAndResponseBehaviorS3Origin.html#request-s3-removed-headers">HTTP Request Headers That CloudFront Removes or Updates</a> </p> </li> <li> <p> <b>Custom origin</b>: See <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/RequestAndResponseBehaviorCustomOrigin.html#request-custom-headers-behavior">HTTP Request Headers and CloudFront Behavior</a> </p> </li> </ul></p>
    pub quantity: i64,
}

struct HeadersDeserializer;
impl HeadersDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct HttpVersionDeserializer;
impl HttpVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct IntegerDeserializer;
impl IntegerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>An invalidation. </p>
#[derive(Default, Debug, Clone, PartialEq)]
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

struct InvalidationDeserializer;
impl InvalidationDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InvalidationBatch {
    /// <p>A value that you specify to uniquely identify an invalidation request. CloudFront uses the value to prevent you from accidentally resubmitting an identical request. Whenever you create a new invalidation request, you must specify a new value for <code>CallerReference</code> and change other values in the request as applicable. One way to ensure that the value of <code>CallerReference</code> is unique is to use a <code>timestamp</code>, for example, <code>20120301090000</code>.</p> <p>If you make a second invalidation request with the same value for <code>CallerReference</code>, and if the rest of the request is the same, CloudFront doesn't create a new invalidation request. Instead, CloudFront returns information about the invalidation request that you previously created with the same <code>CallerReference</code>.</p> <p>If <code>CallerReference</code> is a value you already sent in a previous invalidation batch request but the content of any <code>Path</code> is different from the original request, CloudFront returns an <code>InvalidationBatchAlreadyExists</code> error.</p>
    pub caller_reference: String,
    /// <p>A complex type that contains information about the objects that you want to invalidate. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Invalidation.html#invalidation-specifying-objects">Specifying the Objects to Invalidate</a> in the <i>Amazon CloudFront Developer Guide</i>. </p>
    pub paths: Paths,
}

struct InvalidationBatchDeserializer;
impl InvalidationBatchDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("CallerReference"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.caller_reference
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        PathsSerializer::serialize(&mut writer, "Paths", &obj.paths)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>The <code>InvalidationList</code> complex type describes the list of invalidation objects. For more information about invalidation, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Invalidation.html">Invalidating Objects (Web Distributions Only)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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

struct InvalidationListDeserializer;
impl InvalidationListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct InvalidationSummary {
    /// <p>The time that an invalidation request was created.</p>
    pub create_time: String,
    /// <p>The unique ID for an invalidation request.</p>
    pub id: String,
    /// <p>The status of an invalidation request.</p>
    pub status: String,
}

struct InvalidationSummaryDeserializer;
impl InvalidationSummaryDeserializer {
    #[allow(unused_variables)]
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
struct InvalidationSummaryListDeserializer;
impl InvalidationSummaryListDeserializer {
    #[allow(unused_variables)]
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
struct ItemSelectionDeserializer;
impl ItemSelectionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ItemSelectionSerializer;
impl ItemSelectionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct KeyPairIdListDeserializer;
impl KeyPairIdListDeserializer {
    #[allow(unused_variables)]
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
/// <p>A complex type that lists the active CloudFront key pairs, if any, that are associated with <code>AwsAccountNumber</code>. </p> <p>For more information, see <a>ActiveTrustedSigners</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct KeyPairIds {
    /// <p>A complex type that lists the active CloudFront key pairs, if any, that are associated with <code>AwsAccountNumber</code>.</p> <p>For more information, see <a>ActiveTrustedSigners</a>.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of active CloudFront key pairs for <code>AwsAccountNumber</code>.</p> <p>For more information, see <a>ActiveTrustedSigners</a>.</p>
    pub quantity: i64,
}

struct KeyPairIdsDeserializer;
impl KeyPairIdsDeserializer {
    #[allow(unused_variables)]
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
struct LambdaFunctionARNDeserializer;
impl LambdaFunctionARNDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that contains a Lambda function association.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LambdaFunctionAssociation {
    /// <p><p>Specifies the event type that triggers a Lambda function invocation. You can specify the following values:</p> <ul> <li> <p> <code>viewer-request</code>: The function executes when CloudFront receives a request from a viewer and before it checks to see whether the requested object is in the edge cache. </p> </li> <li> <p> <code>origin-request</code>: The function executes only when CloudFront forwards a request to your origin. When the requested object is in the edge cache, the function doesn&#39;t execute.</p> </li> <li> <p> <code>origin-response</code>: The function executes after CloudFront receives a response from the origin and before it caches the object in the response. When the requested object is in the edge cache, the function doesn&#39;t execute.</p> </li> <li> <p> <code>viewer-response</code>: The function executes before CloudFront returns the requested object to the viewer. The function executes regardless of whether the object was already in the edge cache.</p> <p>If the origin returns an HTTP status code other than HTTP 200 (OK), the function doesn&#39;t execute.</p> </li> </ul></p>
    pub event_type: String,
    /// <p>A flag that allows a Lambda function to have read access to the body content. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/lambda-include-body-access.html">Accessing the Request Body by Choosing the Include Body Option</a> in the Amazon CloudFront Developer Guide.</p>
    pub include_body: Option<bool>,
    /// <p>The ARN of the Lambda function. You must specify the ARN of a function version; you can't specify a Lambda alias or $LATEST.</p>
    pub lambda_function_arn: String,
}

struct LambdaFunctionAssociationDeserializer;
impl LambdaFunctionAssociationDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("EventType"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.event_type
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.include_body {
            writer.write(xml::writer::XmlEvent::start_element("IncludeBody"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("LambdaFunctionARN"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.lambda_function_arn
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct LambdaFunctionAssociationListDeserializer;
impl LambdaFunctionAssociationListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LambdaFunctionAssociations {
    /// <p> <b>Optional</b>: A complex type that contains <code>LambdaFunctionAssociation</code> items for this cache behavior. If <code>Quantity</code> is <code>0</code>, you can omit <code>Items</code>.</p>
    pub items: Option<Vec<LambdaFunctionAssociation>>,
    /// <p>The number of Lambda function associations for this cache behavior.</p>
    pub quantity: i64,
}

struct LambdaFunctionAssociationsDeserializer;
impl LambdaFunctionAssociationsDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>The request to list origin access identities. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCloudFrontOriginAccessIdentitiesRequest {
    /// <p>Use this when paginating results to indicate where to begin in your list of origin access identities. The results include identities in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last identity on that page).</p>
    pub marker: Option<String>,
    /// <p>The maximum number of origin access identities you want in the response body. </p>
    pub max_items: Option<String>,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListCloudFrontOriginAccessIdentitiesResult {
    /// <p>The <code>CloudFrontOriginAccessIdentityList</code> type. </p>
    pub cloud_front_origin_access_identity_list: Option<CloudFrontOriginAccessIdentityList>,
}

struct ListCloudFrontOriginAccessIdentitiesResultDeserializer;
impl ListCloudFrontOriginAccessIdentitiesResultDeserializer {
    #[allow(unused_variables)]
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
/// <p>The request to list distributions that are associated with a specified AWS WAF web ACL. </p>
#[derive(Default, Debug, Clone, PartialEq)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListDistributionsByWebACLIdResult {
    /// <p>The <code>DistributionList</code> type. </p>
    pub distribution_list: Option<DistributionList>,
}

struct ListDistributionsByWebACLIdResultDeserializer;
impl ListDistributionsByWebACLIdResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDistributionsRequest {
    /// <p>Use this when paginating results to indicate where to begin in your list of distributions. The results include distributions in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last distribution on that page).</p>
    pub marker: Option<String>,
    /// <p>The maximum number of distributions you want in the response body.</p>
    pub max_items: Option<String>,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListDistributionsResult {
    /// <p>The <code>DistributionList</code> type. </p>
    pub distribution_list: Option<DistributionList>,
}

struct ListDistributionsResultDeserializer;
impl ListDistributionsResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFieldLevelEncryptionConfigsRequest {
    /// <p>Use this when paginating results to indicate where to begin in your list of configurations. The results include configurations in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last configuration on that page). </p>
    pub marker: Option<String>,
    /// <p>The maximum number of field-level encryption configurations you want in the response body. </p>
    pub max_items: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListFieldLevelEncryptionConfigsResult {
    /// <p>Returns a list of all field-level encryption configurations that have been created in CloudFront for this account.</p>
    pub field_level_encryption_list: Option<FieldLevelEncryptionList>,
}

struct ListFieldLevelEncryptionConfigsResultDeserializer;
impl ListFieldLevelEncryptionConfigsResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFieldLevelEncryptionProfilesRequest {
    /// <p>Use this when paginating results to indicate where to begin in your list of profiles. The results include profiles in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last profile on that page). </p>
    pub marker: Option<String>,
    /// <p>The maximum number of field-level encryption profiles you want in the response body. </p>
    pub max_items: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListFieldLevelEncryptionProfilesResult {
    /// <p>Returns a list of the field-level encryption profiles that have been created in CloudFront for this account.</p>
    pub field_level_encryption_profile_list: Option<FieldLevelEncryptionProfileList>,
}

struct ListFieldLevelEncryptionProfilesResultDeserializer;
impl ListFieldLevelEncryptionProfilesResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListInvalidationsResult {
    /// <p>Information about invalidation batches. </p>
    pub invalidation_list: Option<InvalidationList>,
}

struct ListInvalidationsResultDeserializer;
impl ListInvalidationsResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPublicKeysRequest {
    /// <p>Use this when paginating results to indicate where to begin in your list of public keys. The results include public keys in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last public key on that page). </p>
    pub marker: Option<String>,
    /// <p>The maximum number of public keys you want in the response body. </p>
    pub max_items: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListPublicKeysResult {
    /// <p>Returns a list of all public keys that have been added to CloudFront for this account.</p>
    pub public_key_list: Option<PublicKeyList>,
}

struct ListPublicKeysResultDeserializer;
impl ListPublicKeysResultDeserializer {
    #[allow(unused_variables)]
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
/// <p>The request to list your streaming distributions. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListStreamingDistributionsRequest {
    /// <p>The value that you provided for the <code>Marker</code> request parameter.</p>
    pub marker: Option<String>,
    /// <p>The value that you provided for the <code>MaxItems</code> request parameter.</p>
    pub max_items: Option<String>,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListStreamingDistributionsResult {
    /// <p>The <code>StreamingDistributionList</code> type. </p>
    pub streaming_distribution_list: Option<StreamingDistributionList>,
}

struct ListStreamingDistributionsResultDeserializer;
impl ListStreamingDistributionsResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p> An ARN of a CloudFront resource.</p>
    pub resource: String,
}

/// <p> The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListTagsForResourceResult {
    /// <p> A complex type that contains zero or more <code>Tag</code> elements.</p>
    pub tags: Tags,
}

struct ListTagsForResourceResultDeserializer;
impl ListTagsForResourceResultDeserializer {
    #[allow(unused_variables)]
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
struct LocationListDeserializer;
impl LocationListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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

struct LoggingConfigDeserializer;
impl LoggingConfigDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Bucket"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.bucket
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Enabled"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.enabled
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("IncludeCookies"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.include_cookies
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.prefix
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct LongDeserializer;
impl LongDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MethodDeserializer;
impl MethodDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MethodsListDeserializer;
impl MethodsListDeserializer {
    #[allow(unused_variables)]
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

struct MinimumProtocolVersionDeserializer;
impl MinimumProtocolVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that describes the Amazon S3 bucket, HTTP server (for example, a web server), Amazon MediaStore, or other server from which CloudFront gets your files. This can also be an origin group, if you've created an origin group. You must specify at least one origin or origin group.</p> <p>For the current limit on the number of origins or origin groups that you can specify for a distribution, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_cloudfront">Amazon CloudFront Limits</a> in the <i>AWS General Reference</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Origin {
    /// <p>A complex type that contains names and values for the custom headers that you want.</p>
    pub custom_headers: Option<CustomHeaders>,
    /// <p>A complex type that contains information about a custom origin. If the origin is an Amazon S3 bucket, use the <code>S3OriginConfig</code> element instead.</p>
    pub custom_origin_config: Option<CustomOriginConfig>,
    /// <p><p> <b>Amazon S3 origins</b>: The DNS name of the Amazon S3 bucket from which you want CloudFront to get objects for this origin, for example, <code>myawsbucket.s3.amazonaws.com</code>. If you set up your bucket to be configured as a website endpoint, enter the Amazon S3 static website hosting endpoint for the bucket.</p> <p>For more information about specifying this value for different types of origins, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesDomainName">Origin Domain Name</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>Constraints for Amazon S3 origins: </p> <ul> <li> <p>If you configured Amazon S3 Transfer Acceleration for your bucket, don&#39;t specify the <code>s3-accelerate</code> endpoint for <code>DomainName</code>.</p> </li> <li> <p>The bucket name must be between 3 and 63 characters long (inclusive).</p> </li> <li> <p>The bucket name must contain only lowercase characters, numbers, periods, underscores, and dashes.</p> </li> <li> <p>The bucket name must not contain adjacent periods.</p> </li> </ul> <p> <b>Custom Origins</b>: The DNS domain name for the HTTP server from which you want CloudFront to get objects for this origin, for example, <code>www.example.com</code>. </p> <p>Constraints for custom origins:</p> <ul> <li> <p> <code>DomainName</code> must be a valid DNS name that contains only a-z, A-Z, 0-9, dot (.), hyphen (-), or underscore (_) characters.</p> </li> <li> <p>The name cannot exceed 128 characters.</p> </li> </ul></p>
    pub domain_name: String,
    /// <p>A unique identifier for the origin or origin group. The value of <code>Id</code> must be unique within the distribution.</p> <p>When you specify the value of <code>TargetOriginId</code> for the default cache behavior or for another cache behavior, you indicate the origin to which you want the cache behavior to route requests by specifying the value of the <code>Id</code> element for that origin. When a request matches the path pattern for that cache behavior, CloudFront routes the request to the specified origin. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesCacheBehavior">Cache Behavior Settings</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub id: String,
    /// <p>An optional element that causes CloudFront to request your content from a directory in your Amazon S3 bucket or your custom origin. When you include the <code>OriginPath</code> element, specify the directory name, beginning with a <code>/</code>. CloudFront appends the directory name to the value of <code>DomainName</code>, for example, <code>example.com/production</code>. Do not include a <code>/</code> at the end of the directory name.</p> <p>For example, suppose you've specified the following values for your distribution:</p> <ul> <li> <p> <code>DomainName</code>: An Amazon S3 bucket named <code>myawsbucket</code>.</p> </li> <li> <p> <code>OriginPath</code>: <code>/production</code> </p> </li> <li> <p> <code>CNAME</code>: <code>example.com</code> </p> </li> </ul> <p>When a user enters <code>example.com/index.html</code> in a browser, CloudFront sends a request to Amazon S3 for <code>myawsbucket/production/index.html</code>.</p> <p>When a user enters <code>example.com/acme/index.html</code> in a browser, CloudFront sends a request to Amazon S3 for <code>myawsbucket/production/acme/index.html</code>.</p>
    pub origin_path: Option<String>,
    /// <p>A complex type that contains information about the Amazon S3 origin. If the origin is a custom origin, use the <code>CustomOriginConfig</code> element instead.</p>
    pub s3_origin_config: Option<S3OriginConfig>,
}

struct OriginDeserializer;
impl OriginDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Origin, XmlParseError> {
        deserialize_elements::<_, Origin, _>(tag_name, stack, |name, stack, obj| {
            match name {
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
        if let Some(ref value) = obj.custom_headers {
            &CustomHeadersSerializer::serialize(&mut writer, "CustomHeaders", value)?;
        }
        if let Some(ref value) = obj.custom_origin_config {
            &CustomOriginConfigSerializer::serialize(&mut writer, "CustomOriginConfig", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("DomainName"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.domain_name
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Id"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.origin_path {
            writer.write(xml::writer::XmlEvent::start_element("OriginPath"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.s3_origin_config {
            &S3OriginConfigSerializer::serialize(&mut writer, "S3OriginConfig", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that contains <code>HeaderName</code> and <code>HeaderValue</code> elements, if any, for this distribution. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginCustomHeader {
    /// <p>The name of a header that you want CloudFront to forward to your origin. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/forward-custom-headers.html">Forwarding Custom Headers to Your Origin (Web Distributions Only)</a> in the <i>Amazon Amazon CloudFront Developer Guide</i>.</p>
    pub header_name: String,
    /// <p>The value for the header that you specified in the <code>HeaderName</code> field.</p>
    pub header_value: String,
}

struct OriginCustomHeaderDeserializer;
impl OriginCustomHeaderDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("HeaderName"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.header_name
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("HeaderValue"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.header_value
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct OriginCustomHeadersListDeserializer;
impl OriginCustomHeadersListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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

struct OriginGroupDeserializer;
impl OriginGroupDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Id"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        OriginGroupMembersSerializer::serialize(&mut writer, "Members", &obj.members)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex data type that includes information about the failover criteria for an origin group, including the status codes for which CloudFront will failover from the primary origin to the second origin.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginGroupFailoverCriteria {
    /// <p>The status codes that, when returned from the primary origin, will trigger CloudFront to failover to the second origin.</p>
    pub status_codes: StatusCodes,
}

struct OriginGroupFailoverCriteriaDeserializer;
impl OriginGroupFailoverCriteriaDeserializer {
    #[allow(unused_variables)]
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

struct OriginGroupListDeserializer;
impl OriginGroupListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginGroupMember {
    /// <p>The ID for an origin in an origin group.</p>
    pub origin_id: String,
}

struct OriginGroupMemberDeserializer;
impl OriginGroupMemberDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("OriginId"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.origin_id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct OriginGroupMemberListDeserializer;
impl OriginGroupMemberListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginGroupMembers {
    /// <p>Items (origins) in an origin group.</p>
    pub items: Vec<OriginGroupMember>,
    /// <p>The number of origins in an origin group.</p>
    pub quantity: i64,
}

struct OriginGroupMembersDeserializer;
impl OriginGroupMembersDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex data type for the origin groups specified for a distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginGroups {
    /// <p>The items (origin groups) in a distribution.</p>
    pub items: Option<Vec<OriginGroup>>,
    /// <p>The number of origin groups.</p>
    pub quantity: i64,
}

struct OriginGroupsDeserializer;
impl OriginGroupsDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct OriginListDeserializer;
impl OriginListDeserializer {
    #[allow(unused_variables)]
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

struct OriginProtocolPolicyDeserializer;
impl OriginProtocolPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that contains information about the SSL/TLS protocols that CloudFront can use when establishing an HTTPS connection with your origin. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OriginSslProtocols {
    /// <p>A list that contains allowed SSL/TLS protocols for this distribution.</p>
    pub items: Vec<String>,
    /// <p>The number of SSL/TLS protocols that you want to allow CloudFront to use when establishing an HTTPS connection with this origin. </p>
    pub quantity: i64,
}

struct OriginSslProtocolsDeserializer;
impl OriginSslProtocolsDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that contains information about origins and origin groups for this distribution. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Origins {
    /// <p>A complex type that contains origins or origin groups for this distribution.</p>
    pub items: Vec<Origin>,
    /// <p>The number of origins or origin groups for this distribution.</p>
    pub quantity: i64,
}

struct OriginsDeserializer;
impl OriginsDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct PathListDeserializer;
impl PathListDeserializer {
    #[allow(unused_variables)]
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

/// <p>A complex type that contains information about the objects that you want to invalidate. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Invalidation.html#invalidation-specifying-objects">Specifying the Objects to Invalidate</a> in the <i>Amazon CloudFront Developer Guide</i>. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Paths {
    /// <p>A complex type that contains a list of the paths that you want to invalidate.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of objects that you want to invalidate.</p>
    pub quantity: i64,
}

struct PathsDeserializer;
impl PathsDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct PriceClassDeserializer;
impl PriceClassDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex data type of public keys you add to CloudFront to use with features like field-level encryption.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PublicKey {
    /// <p>A time you added a public key to CloudFront.</p>
    pub created_time: String,
    /// <p>A unique ID assigned to a public key you've added to CloudFront.</p>
    pub id: String,
    /// <p>A complex data type for a public key you add to CloudFront to use with features like field-level encryption.</p>
    pub public_key_config: PublicKeyConfig,
}

struct PublicKeyDeserializer;
impl PublicKeyDeserializer {
    #[allow(unused_variables)]
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
/// <p>Information about a public key you add to CloudFront to use with features like field-level encryption.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PublicKeyConfig {
    /// <p>A unique number that ensures that the request can't be replayed.</p>
    pub caller_reference: String,
    /// <p>An optional comment about a public key.</p>
    pub comment: Option<String>,
    /// <p>The encoded public key that you want to add to CloudFront to use with features like field-level encryption.</p>
    pub encoded_key: String,
    /// <p>The name for a public key you add to CloudFront to use with features like field-level encryption.</p>
    pub name: String,
}

struct PublicKeyConfigDeserializer;
impl PublicKeyConfigDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("CallerReference"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.caller_reference
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.comment {
            writer.write(xml::writer::XmlEvent::start_element("Comment"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("EncodedKey"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.encoded_key
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Name"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.name
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A list of public keys you've added to CloudFront to use with features like field-level encryption.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PublicKeyList {
    /// <p>An array of information about a public key you add to CloudFront to use with features like field-level encryption.</p>
    pub items: Option<Vec<PublicKeySummary>>,
    /// <p>The maximum number of public keys you want in the response body. </p>
    pub max_items: i64,
    /// <p>If there are more elements to be listed, this element is present and contains the value that you can use for the <code>Marker</code> request parameter to continue listing your public keys where you left off.</p>
    pub next_marker: Option<String>,
    /// <p>The number of public keys you added to CloudFront to use with features like field-level encryption.</p>
    pub quantity: i64,
}

struct PublicKeyListDeserializer;
impl PublicKeyListDeserializer {
    #[allow(unused_variables)]
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
/// <p>A complex data type for public key information. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PublicKeySummary {
    /// <p> Comment for public key information summary. </p>
    pub comment: Option<String>,
    /// <p> Creation time for public key information summary. </p>
    pub created_time: String,
    /// <p> Encoded key for public key information summary. </p>
    pub encoded_key: String,
    /// <p> ID for public key information summary. </p>
    pub id: String,
    /// <p> Name for public key information summary. </p>
    pub name: String,
}

struct PublicKeySummaryDeserializer;
impl PublicKeySummaryDeserializer {
    #[allow(unused_variables)]
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
struct PublicKeySummaryListDeserializer;
impl PublicKeySummaryListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct QueryArgProfile {
    /// <p>ID of profile to use for field-level encryption query argument-profile mapping</p>
    pub profile_id: String,
    /// <p>Query argument for field-level encryption query argument-profile mapping.</p>
    pub query_arg: String,
}

struct QueryArgProfileDeserializer;
impl QueryArgProfileDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("ProfileId"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.profile_id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("QueryArg"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.query_arg
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Configuration for query argument-profile mapping for field-level encryption.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct QueryArgProfileConfig {
    /// <p>Flag to set if you want a request to be forwarded to the origin even if the profile specified by the field-level encryption query argument, fle-profile, is unknown.</p>
    pub forward_when_query_arg_profile_is_unknown: bool,
    /// <p>Profiles specified for query argument-profile mapping for field-level encryption.</p>
    pub query_arg_profiles: Option<QueryArgProfiles>,
}

struct QueryArgProfileConfigDeserializer;
impl QueryArgProfileConfigDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element(
            "ForwardWhenQueryArgProfileIsUnknown",
        ))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.forward_when_query_arg_profile_is_unknown
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.query_arg_profiles {
            &QueryArgProfilesSerializer::serialize(&mut writer, "QueryArgProfiles", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct QueryArgProfileListDeserializer;
impl QueryArgProfileListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct QueryArgProfiles {
    /// <p>Number of items for query argument-profile mapping for field-level encryption.</p>
    pub items: Option<Vec<QueryArgProfile>>,
    /// <p>Number of profiles for query argument-profile mapping for field-level encryption.</p>
    pub quantity: i64,
}

struct QueryArgProfilesDeserializer;
impl QueryArgProfilesDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct QueryStringCacheKeys {
    /// <p>(Optional) A list that contains the query string parameters that you want CloudFront to use as a basis for caching for this cache behavior. If <code>Quantity</code> is 0, you can omit <code>Items</code>. </p>
    pub items: Option<Vec<String>>,
    /// <p>The number of <code>whitelisted</code> query string parameters for this cache behavior.</p>
    pub quantity: i64,
}

struct QueryStringCacheKeysDeserializer;
impl QueryStringCacheKeysDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueryStringCacheKeys, XmlParseError> {
        deserialize_elements::<_, QueryStringCacheKeys, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Items" => {
                    obj.items.get_or_insert(vec![]).extend(
                        QueryStringCacheKeysListDeserializer::deserialize("Items", stack)?,
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

pub struct QueryStringCacheKeysSerializer;
impl QueryStringCacheKeysSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &QueryStringCacheKeys,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.items {
            &QueryStringCacheKeysListSerializer::serialize(&mut writer, "Items", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct QueryStringCacheKeysListDeserializer;
impl QueryStringCacheKeysListDeserializer {
    #[allow(unused_variables)]
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

pub struct QueryStringCacheKeysListSerializer;
impl QueryStringCacheKeysListSerializer {
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that identifies ways in which you want to restrict distribution of your content.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Restrictions {
    pub geo_restriction: GeoRestriction,
}

struct RestrictionsDeserializer;
impl RestrictionsDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3Origin {
    /// <p>The DNS name of the Amazon S3 origin. </p>
    pub domain_name: String,
    /// <p>The CloudFront origin access identity to associate with the RTMP distribution. Use an origin access identity to configure the distribution so that end users can only access objects in an Amazon S3 bucket through CloudFront.</p> <p>If you want end users to be able to access objects using either the CloudFront URL or the Amazon S3 URL, specify an empty <code>OriginAccessIdentity</code> element.</p> <p>To delete the origin access identity from an existing distribution, update the distribution configuration and include an empty <code>OriginAccessIdentity</code> element.</p> <p>To replace the origin access identity, update the distribution configuration and specify the new origin access identity.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-restricting-access-to-s3.html">Using an Origin Access Identity to Restrict Access to Your Amazon S3 Content</a> in the <i>Amazon Amazon CloudFront Developer Guide</i>.</p>
    pub origin_access_identity: String,
}

struct S3OriginDeserializer;
impl S3OriginDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("DomainName"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.domain_name
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("OriginAccessIdentity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.origin_access_identity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that contains information about the Amazon S3 origin. If the origin is a custom origin, use the <code>CustomOriginConfig</code> element instead.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3OriginConfig {
    /// <p>The CloudFront origin access identity to associate with the origin. Use an origin access identity to configure the origin so that viewers can <i>only</i> access objects in an Amazon S3 bucket through CloudFront. The format of the value is:</p> <p>origin-access-identity/cloudfront/<i>ID-of-origin-access-identity</i> </p> <p>where <code> <i>ID-of-origin-access-identity</i> </code> is the value that CloudFront returned in the <code>ID</code> element when you created the origin access identity.</p> <p>If you want viewers to be able to access objects using either the CloudFront URL or the Amazon S3 URL, specify an empty <code>OriginAccessIdentity</code> element.</p> <p>To delete the origin access identity from an existing distribution, update the distribution configuration and include an empty <code>OriginAccessIdentity</code> element.</p> <p>To replace the origin access identity, update the distribution configuration and specify the new origin access identity.</p> <p>For more information about the origin access identity, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub origin_access_identity: String,
}

struct S3OriginConfigDeserializer;
impl S3OriginConfigDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("OriginAccessIdentity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.origin_access_identity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct SSLSupportMethodDeserializer;
impl SSLSupportMethodDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that lists the AWS accounts that were included in the <code>TrustedSigners</code> complex type, as well as their active CloudFront key pair IDs, if any. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Signer {
    /// <p><p>An AWS account that is included in the <code>TrustedSigners</code> complex type for this RTMP distribution. Valid values include:</p> <ul> <li> <p> <code>self</code>, which is the AWS account used to create the distribution.</p> </li> <li> <p>An AWS account number.</p> </li> </ul></p>
    pub aws_account_number: Option<String>,
    /// <p>A complex type that lists the active CloudFront key pairs, if any, that are associated with <code>AwsAccountNumber</code>.</p>
    pub key_pair_ids: Option<KeyPairIds>,
}

struct SignerDeserializer;
impl SignerDeserializer {
    #[allow(unused_variables)]
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
struct SignerListDeserializer;
impl SignerListDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct SslProtocolsListDeserializer;
impl SslProtocolsListDeserializer {
    #[allow(unused_variables)]
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

struct StatusCodeListDeserializer;
impl StatusCodeListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StatusCodes {
    /// <p>The items (status codes) for an origin group.</p>
    pub items: Vec<i64>,
    /// <p>The number of status codes.</p>
    pub quantity: i64,
}

struct StatusCodesDeserializer;
impl StatusCodesDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A streaming distribution. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct StreamingDistribution {
    /// <p>The ARN (Amazon Resource Name) for the distribution. For example: <code>arn:aws:cloudfront::123456789012:distribution/EDFDVBD632BHDS5</code>, where <code>123456789012</code> is your AWS account ID.</p>
    pub arn: String,
    /// <p>A complex type that lists the AWS accounts, if any, that you included in the <code>TrustedSigners</code> complex type for this distribution. These are the accounts that you want to allow to create signed URLs for private content.</p> <p>The <code>Signer</code> complex type lists the AWS account number of the trusted signer or <code>self</code> if the signer is the AWS account that created the distribution. The <code>Signer</code> element also includes the IDs of any active CloudFront key pairs that are associated with the trusted signer's AWS account. If no <code>KeyPairId</code> element appears for a <code>Signer</code>, that signer can't create signed URLs.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>. </p>
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

struct StreamingDistributionDeserializer;
impl StreamingDistributionDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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
    /// <p>A complex type that specifies any AWS accounts that you want to permit to create signed URLs for private content. If you want the distribution to use signed URLs, include this element; if you want the distribution to use public URLs, remove this element. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>. </p>
    pub trusted_signers: TrustedSigners,
}

struct StreamingDistributionConfigDeserializer;
impl StreamingDistributionConfigDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("CallerReference"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.caller_reference
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Comment"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.comment
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Enabled"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.enabled
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.logging {
            &StreamingLoggingConfigSerializer::serialize(&mut writer, "Logging", value)?;
        }
        if let Some(ref value) = obj.price_class {
            writer.write(xml::writer::XmlEvent::start_element("PriceClass"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        S3OriginSerializer::serialize(&mut writer, "S3Origin", &obj.s3_origin)?;
        TrustedSignersSerializer::serialize(&mut writer, "TrustedSigners", &obj.trusted_signers)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A streaming distribution Configuration and a list of tags to be associated with the streaming distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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

struct StreamingDistributionListDeserializer;
impl StreamingDistributionListDeserializer {
    #[allow(unused_variables)]
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
/// <p> A summary of the information for an Amazon CloudFront streaming distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    /// <p><p/></p>
    pub price_class: String,
    /// <p>A complex type that contains information about the Amazon S3 bucket from which you want CloudFront to get your media files for distribution.</p>
    pub s3_origin: S3Origin,
    /// <p> Indicates the current status of the distribution. When the status is <code>Deployed</code>, the distribution's information is fully propagated throughout the Amazon CloudFront system.</p>
    pub status: String,
    /// <p>A complex type that specifies the AWS accounts, if any, that you want to allow to create signed URLs for private content. If you want to require signed URLs in requests for objects in the target origin that match the <code>PathPattern</code> for this cache behavior, specify <code>true</code> for <code>Enabled</code>, and specify the applicable values for <code>Quantity</code> and <code>Items</code>.If you don't want to require signed URLs in requests for objects that match <code>PathPattern</code>, specify <code>false</code> for <code>Enabled</code> and <code>0</code> for <code>Quantity</code>. Omit <code>Items</code>. To add, change, or remove one or more trusted signers, change <code>Enabled</code> to <code>true</code> (if it's currently <code>false</code>), change <code>Quantity</code> as applicable, and specify all of the trusted signers that you want to include in the updated distribution.</p>
    pub trusted_signers: TrustedSigners,
}

struct StreamingDistributionSummaryDeserializer;
impl StreamingDistributionSummaryDeserializer {
    #[allow(unused_variables)]
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
struct StreamingDistributionSummaryListDeserializer;
impl StreamingDistributionSummaryListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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

struct StreamingLoggingConfigDeserializer;
impl StreamingLoggingConfigDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Bucket"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.bucket
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Enabled"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.enabled
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.prefix
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p> A complex type that contains <code>Tag</code> key and <code>Tag</code> value.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tag {
    /// <p> A string that contains <code>Tag</code> key.</p> <p>The string length should be between 1 and 128 characters. Valid characters include <code>a-z</code>, <code>A-Z</code>, <code>0-9</code>, space, and the special characters <code>_ - . : / = + @</code>.</p>
    pub key: String,
    /// <p> A string that contains an optional <code>Tag</code> value.</p> <p>The string length should be between 0 and 256 characters. Valid characters include <code>a-z</code>, <code>A-Z</code>, <code>0-9</code>, space, and the special characters <code>_ - . : / = + @</code>.</p>
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
        writer.write(xml::writer::XmlEvent::start_element("Key"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.key
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.value {
            writer.write(xml::writer::XmlEvent::start_element("Value"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
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
#[derive(Default, Debug, Clone, PartialEq)]
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

struct TagListDeserializer;
impl TagListDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p> An ARN of a CloudFront resource.</p>
    pub resource: String,
    /// <p> A complex type that contains zero or more <code>Tag</code> elements.</p>
    pub tags: Tags,
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p> A complex type that contains zero or more <code>Tag</code> elements.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tags {
    /// <p> A complex type that contains <code>Tag</code> elements.</p>
    pub items: Option<Vec<Tag>>,
}

struct TagsDeserializer;
impl TagsDeserializer {
    #[allow(unused_variables)]
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

struct TimestampDeserializer;
impl TimestampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A complex type that specifies the AWS accounts, if any, that you want to allow to create signed URLs for private content.</p> <p>If you want to require signed URLs in requests for objects in the target origin that match the <code>PathPattern</code> for this cache behavior, specify <code>true</code> for <code>Enabled</code>, and specify the applicable values for <code>Quantity</code> and <code>Items</code>. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon Amazon CloudFront Developer Guide</i>.</p> <p>If you don't want to require signed URLs in requests for objects that match <code>PathPattern</code>, specify <code>false</code> for <code>Enabled</code> and <code>0</code> for <code>Quantity</code>. Omit <code>Items</code>.</p> <p>To add, change, or remove one or more trusted signers, change <code>Enabled</code> to <code>true</code> (if it's currently <code>false</code>), change <code>Quantity</code> as applicable, and specify all of the trusted signers that you want to include in the updated distribution.</p> <p>For more information about updating the distribution configuration, see <a>DistributionConfig</a> .</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TrustedSigners {
    /// <p>Specifies whether you want to require viewers to use signed URLs to access the files specified by <code>PathPattern</code> and <code>TargetOriginId</code>.</p>
    pub enabled: bool,
    /// <p> <b>Optional</b>: A complex type that contains trusted signers for this cache behavior. If <code>Quantity</code> is <code>0</code>, you can omit <code>Items</code>.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of trusted signers for this cache behavior.</p>
    pub quantity: i64,
}

struct TrustedSignersDeserializer;
impl TrustedSignersDeserializer {
    #[allow(unused_variables)]
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
        writer.write(xml::writer::XmlEvent::start_element("Enabled"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.enabled
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.items {
            &AwsAccountNumberListSerializer::serialize(&mut writer, "Items", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Quantity"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.quantity
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p> The request to remove tags from a CloudFront resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p> An ARN of a CloudFront resource.</p>
    pub resource: String,
    /// <p> A complex type that contains zero or more <code>Tag</code> key elements.</p>
    pub tag_keys: TagKeys,
}

/// <p>The request to update an origin access identity.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateCloudFrontOriginAccessIdentityResult {
    /// <p>The origin access identity's information.</p>
    pub cloud_front_origin_access_identity: Option<CloudFrontOriginAccessIdentity>,
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

struct UpdateCloudFrontOriginAccessIdentityResultDeserializer;
impl UpdateCloudFrontOriginAccessIdentityResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateDistributionResult {
    /// <p>The distribution's information.</p>
    pub distribution: Option<Distribution>,
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

struct UpdateDistributionResultDeserializer;
impl UpdateDistributionResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFieldLevelEncryptionConfigRequest {
    /// <p>Request to update a field-level encryption configuration. </p>
    pub field_level_encryption_config: FieldLevelEncryptionConfig,
    /// <p>The ID of the configuration you want to update.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the configuration identity to update. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateFieldLevelEncryptionConfigResult {
    /// <p>The value of the <code>ETag</code> header that you received when updating the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Return the results of updating the configuration.</p>
    pub field_level_encryption: Option<FieldLevelEncryption>,
}

struct UpdateFieldLevelEncryptionConfigResultDeserializer;
impl UpdateFieldLevelEncryptionConfigResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFieldLevelEncryptionProfileRequest {
    /// <p>Request to update a field-level encryption profile. </p>
    pub field_level_encryption_profile_config: FieldLevelEncryptionProfileConfig,
    /// <p>The ID of the field-level encryption profile request. </p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the profile identity to update. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateFieldLevelEncryptionProfileResult {
    /// <p>The result of the field-level encryption profile request. </p>
    pub e_tag: Option<String>,
    /// <p>Return the results of updating the profile.</p>
    pub field_level_encryption_profile: Option<FieldLevelEncryptionProfile>,
}

struct UpdateFieldLevelEncryptionProfileResultDeserializer;
impl UpdateFieldLevelEncryptionProfileResultDeserializer {
    #[allow(unused_variables)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePublicKeyRequest {
    /// <p>ID of the public key to be updated.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the public key to update. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
    /// <p>Request to update public key information.</p>
    pub public_key_config: PublicKeyConfig,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdatePublicKeyResult {
    /// <p>The current version of the update public key result. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>Return the results of updating the public key.</p>
    pub public_key: Option<PublicKey>,
}

struct UpdatePublicKeyResultDeserializer;
impl UpdatePublicKeyResultDeserializer {
    #[allow(unused_variables)]
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
/// <p>The request to update a streaming distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateStreamingDistributionResult {
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>The streaming distribution's information.</p>
    pub streaming_distribution: Option<StreamingDistribution>,
}

struct UpdateStreamingDistributionResultDeserializer;
impl UpdateStreamingDistributionResultDeserializer {
    #[allow(unused_variables)]
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
/// <p>A complex type that specifies the following:</p> <ul> <li> <p>Whether you want viewers to use HTTP or HTTPS to request your objects.</p> </li> <li> <p>If you want viewers to use HTTPS, whether you're using an alternate domain name such as <code>example.com</code> or the CloudFront domain name for your distribution, such as <code>d111111abcdef8.cloudfront.net</code>.</p> </li> <li> <p>If you're using an alternate domain name, whether AWS Certificate Manager (ACM) provided the certificate, or you purchased a certificate from a third-party certificate authority and imported it into ACM or uploaded it to the IAM certificate store.</p> </li> </ul> <p>You must specify only one of the following values: </p> <ul> <li> <p> <a>ViewerCertificate$ACMCertificateArn</a> </p> </li> <li> <p> <a>ViewerCertificate$IAMCertificateId</a> </p> </li> <li> <p> <a>ViewerCertificate$CloudFrontDefaultCertificate</a> </p> </li> </ul> <p>Don't specify <code>false</code> for <code>CloudFrontDefaultCertificate</code>.</p> <p> <b>If you want viewers to use HTTP instead of HTTPS to request your objects</b>: Specify the following value:</p> <p> <code>&lt;CloudFrontDefaultCertificate&gt;true&lt;CloudFrontDefaultCertificate&gt;</code> </p> <p>In addition, specify <code>allow-all</code> for <code>ViewerProtocolPolicy</code> for all of your cache behaviors.</p> <p> <b>If you want viewers to use HTTPS to request your objects</b>: Choose the type of certificate that you want to use based on whether you're using an alternate domain name for your objects or the CloudFront domain name:</p> <ul> <li> <p> <b>If you're using an alternate domain name, such as example.com</b>: Specify one of the following values, depending on whether ACM provided your certificate or you purchased your certificate from third-party certificate authority:</p> <ul> <li> <p> <code>&lt;ACMCertificateArn&gt;<i>ARN for ACM SSL/TLS certificate</i>&lt;ACMCertificateArn&gt;</code> where <code> <i>ARN for ACM SSL/TLS certificate</i> </code> is the ARN for the ACM SSL/TLS certificate that you want to use for this distribution.</p> </li> <li> <p> <code>&lt;IAMCertificateId&gt;<i>IAM certificate ID</i>&lt;IAMCertificateId&gt;</code> where <code> <i>IAM certificate ID</i> </code> is the ID that IAM returned when you added the certificate to the IAM certificate store.</p> </li> </ul> <p>If you specify <code>ACMCertificateArn</code> or <code>IAMCertificateId</code>, you must also specify a value for <code>SSLSupportMethod</code>.</p> <p>If you choose to use an ACM certificate or a certificate in the IAM certificate store, we recommend that you use only an alternate domain name in your object URLs (<code>https://example.com/logo.jpg</code>). If you use the domain name that is associated with your CloudFront distribution (such as <code>https://d111111abcdef8.cloudfront.net/logo.jpg</code>) and the viewer supports <code>SNI</code>, then CloudFront behaves normally. However, if the browser does not support SNI, the user's experience depends on the value that you choose for <code>SSLSupportMethod</code>:</p> <ul> <li> <p> <code>vip</code>: The viewer displays a warning because there is a mismatch between the CloudFront domain name and the domain name in your SSL/TLS certificate.</p> </li> <li> <p> <code>sni-only</code>: CloudFront drops the connection with the browser without returning the object.</p> </li> </ul> </li> <li> <p> <b>If you're using the CloudFront domain name for your distribution, such as <code>d111111abcdef8.cloudfront.net</code> </b>: Specify the following value:</p> <p> <code>&lt;CloudFrontDefaultCertificate&gt;true&lt;CloudFrontDefaultCertificate&gt; </code> </p> </li> </ul> <p>If you want viewers to use HTTPS, you must also specify one of the following values in your cache behaviors:</p> <ul> <li> <p> <code> &lt;ViewerProtocolPolicy&gt;https-only&lt;ViewerProtocolPolicy&gt;</code> </p> </li> <li> <p> <code>&lt;ViewerProtocolPolicy&gt;redirect-to-https&lt;ViewerProtocolPolicy&gt;</code> </p> </li> </ul> <p>You can also optionally require that CloudFront use HTTPS to communicate with your origin by specifying one of the following values for the applicable origins:</p> <ul> <li> <p> <code>&lt;OriginProtocolPolicy&gt;https-only&lt;OriginProtocolPolicy&gt; </code> </p> </li> <li> <p> <code>&lt;OriginProtocolPolicy&gt;match-viewer&lt;OriginProtocolPolicy&gt; </code> </p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/SecureConnections.html#CNAMEsAndHTTPS">Using Alternate Domain Names and HTTPS</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ViewerCertificate {
    /// <p>For information about how and when to use <code>ACMCertificateArn</code>, see <a>ViewerCertificate</a>.</p>
    pub acm_certificate_arn: Option<String>,
    /// <p>For information about how and when to use <code>CloudFrontDefaultCertificate</code>, see <a>ViewerCertificate</a>.</p>
    pub cloud_front_default_certificate: Option<bool>,
    /// <p>For information about how and when to use <code>IAMCertificateId</code>, see <a>ViewerCertificate</a>.</p>
    pub iam_certificate_id: Option<String>,
    /// <p>Specify the security policy that you want CloudFront to use for HTTPS connections. A security policy determines two settings:</p> <ul> <li> <p>The minimum SSL/TLS protocol that CloudFront uses to communicate with viewers</p> </li> <li> <p>The cipher that CloudFront uses to encrypt the content that it returns to viewers</p> </li> </ul> <note> <p>On the CloudFront console, this setting is called <b>Security policy</b>.</p> </note> <p>We recommend that you specify <code>TLSv1.1_2016</code> unless your users are using browsers or devices that do not support TLSv1.1 or later.</p> <p>When both of the following are true, you must specify <code>TLSv1</code> or later for the security policy: </p> <ul> <li> <p>You're using a custom certificate: you specified a value for <code>ACMCertificateArn</code> or for <code>IAMCertificateId</code> </p> </li> <li> <p>You're using SNI: you specified <code>sni-only</code> for <code>SSLSupportMethod</code> </p> </li> </ul> <p>If you specify <code>true</code> for <code>CloudFrontDefaultCertificate</code>, CloudFront automatically sets the security policy to <code>TLSv1</code> regardless of the value that you specify for <code>MinimumProtocolVersion</code>.</p> <p>For information about the relationship between the security policy that you choose and the protocols and ciphers that CloudFront uses to communicate with viewers, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/secure-connections-supported-viewer-protocols-ciphers.html#secure-connections-supported-ciphers"> Supported SSL/TLS Protocols and Ciphers for Communication Between Viewers and CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub minimum_protocol_version: Option<String>,
    /// <p>If you specify a value for <a>ViewerCertificate$ACMCertificateArn</a> or for <a>ViewerCertificate$IAMCertificateId</a>, you must also specify how you want CloudFront to serve HTTPS requests: using a method that works for all clients or one that works for most clients:</p> <ul> <li> <p> <code>vip</code>: CloudFront uses dedicated IP addresses for your content and can respond to HTTPS requests from any viewer. However, you will incur additional monthly charges.</p> </li> <li> <p> <code>sni-only</code>: CloudFront can respond to HTTPS requests from viewers that support Server Name Indication (SNI). All modern browsers support SNI, but some browsers still in use don't support SNI. If some of your users' browsers don't support SNI, we recommend that you do one of the following:</p> <ul> <li> <p>Use the <code>vip</code> option (dedicated IP addresses) instead of <code>sni-only</code>.</p> </li> <li> <p>Use the CloudFront SSL/TLS certificate instead of a custom certificate. This requires that you use the CloudFront domain name of your distribution in the URLs for your objects, for example, <code>https://d111111abcdef8.cloudfront.net/logo.png</code>.</p> </li> <li> <p>If you can control which browser your users use, upgrade the browser to one that supports SNI.</p> </li> <li> <p>Use HTTP instead of HTTPS.</p> </li> </ul> </li> </ul> <p>Don't specify a value for <code>SSLSupportMethod</code> if you specified <code>&lt;CloudFrontDefaultCertificate&gt;true&lt;CloudFrontDefaultCertificate&gt;</code>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/SecureConnections.html#CNAMEsAndHTTPS.html">Using Alternate Domain Names and HTTPS</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub ssl_support_method: Option<String>,
}

struct ViewerCertificateDeserializer;
impl ViewerCertificateDeserializer {
    #[allow(unused_variables)]
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
            writer.write(xml::writer::XmlEvent::start_element("ACMCertificateArn"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.cloud_front_default_certificate {
            writer.write(xml::writer::XmlEvent::start_element(
                "CloudFrontDefaultCertificate",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.iam_certificate_id {
            writer.write(xml::writer::XmlEvent::start_element("IAMCertificateId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.minimum_protocol_version {
            writer.write(xml::writer::XmlEvent::start_element(
                "MinimumProtocolVersion",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.ssl_support_method {
            writer.write(xml::writer::XmlEvent::start_element("SSLSupportMethod"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ViewerProtocolPolicyDeserializer;
impl ViewerProtocolPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
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
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// Errors returned by CreateCloudFrontOriginAccessIdentity
#[derive(Debug, PartialEq)]
pub enum CreateCloudFrontOriginAccessIdentityError {
    /// <p>If the <code>CallerReference</code> is a value you already sent in a previous request to create an identity but the content of the <code>CloudFrontOriginAccessIdentityConfig</code> is different from the original request, CloudFront returns a <code>CloudFrontOriginAccessIdentityAlreadyExists</code> error. </p>
    CloudFrontOriginAccessIdentityAlreadyExists(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>The argument is invalid.</p>
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
        start_element("ErrorResponse", stack)?;
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

    CNAMEAlreadyExists(String),
    /// <p>The caller reference you attempted to create the distribution with is associated with another distribution.</p>
    DistributionAlreadyExists(String),
    /// <p>The specified configuration for field-level encryption can't be associated with the specified cache behavior.</p>
    IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The default root object file name is too big or contains an invalid character.</p>
    InvalidDefaultRootObject(String),

    InvalidErrorCode(String),
    /// <p>Your request contains forward cookies option which doesn't match with the expectation for the <code>whitelisted</code> list of cookie names. Either list of cookie names has been specified when not allowed or list of cookie names is missing when expected.</p>
    InvalidForwardCookies(String),

    InvalidGeoRestrictionParameter(String),

    InvalidHeadersForS3Origin(String),
    /// <p>The specified Lambda function association is invalid.</p>
    InvalidLambdaFunctionAssociation(String),

    InvalidLocationCode(String),

    InvalidMinimumProtocolVersion(String),
    /// <p>The Amazon S3 origin server specified does not refer to a valid Amazon S3 bucket.</p>
    InvalidOrigin(String),
    /// <p>The origin access identity is not valid or doesn't exist.</p>
    InvalidOriginAccessIdentity(String),

    InvalidOriginKeepaliveTimeout(String),

    InvalidOriginReadTimeout(String),
    /// <p>You cannot specify SSLv3 as the minimum protocol version if you only want to support only clients that support Server Name Indication (SNI).</p>
    InvalidProtocolSettings(String),

    InvalidQueryStringParameters(String),
    /// <p>The relative path is too big, is not URL-encoded, or does not begin with a slash (/).</p>
    InvalidRelativePath(String),
    /// <p>This operation requires the HTTPS protocol. Ensure that you specify the HTTPS protocol in your request, or omit the <code>RequiredProtocols</code> element from your distribution configuration.</p>
    InvalidRequiredProtocol(String),

    InvalidResponseCode(String),

    InvalidTTLOrder(String),

    InvalidViewerCertificate(String),

    InvalidWebACLId(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>The specified configuration for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionConfig(String),
    /// <p>No origin exists with the specified <code>Origin Id</code>. </p>
    NoSuchOrigin(String),
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
    /// <p>The maximum number of distributions have been associated with the specified configuration for field-level encryption.</p>
    TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(String),
    /// <p>Processing your request would cause the maximum number of distributions with Lambda function associations per owner to be exceeded.</p>
    TooManyDistributionsWithLambdaAssociations(String),

    TooManyHeadersInForwardedValues(String),
    /// <p>Your request contains more Lambda function associations than are allowed per distribution.</p>
    TooManyLambdaFunctionAssociations(String),

    TooManyOriginCustomHeaders(String),
    /// <p>Processing your request would cause you to exceed the maximum number of origin groups allowed.</p>
    TooManyOriginGroupsPerDistribution(String),
    /// <p>You cannot create more origins for the distribution.</p>
    TooManyOrigins(String),

    TooManyQueryStringParameters(String),
    /// <p>Your request contains more trusted signers than are allowed per distribution.</p>
    TooManyTrustedSigners(String),
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
                                    "AccessDenied" => return RusotoError::Service(CreateDistributionError::AccessDenied(parsed_error.message)),"CNAMEAlreadyExists" => return RusotoError::Service(CreateDistributionError::CNAMEAlreadyExists(parsed_error.message)),"DistributionAlreadyExists" => return RusotoError::Service(CreateDistributionError::DistributionAlreadyExists(parsed_error.message)),"IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior" => return RusotoError::Service(CreateDistributionError::IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(parsed_error.message)),"InconsistentQuantities" => return RusotoError::Service(CreateDistributionError::InconsistentQuantities(parsed_error.message)),"InvalidArgument" => return RusotoError::Service(CreateDistributionError::InvalidArgument(parsed_error.message)),"InvalidDefaultRootObject" => return RusotoError::Service(CreateDistributionError::InvalidDefaultRootObject(parsed_error.message)),"InvalidErrorCode" => return RusotoError::Service(CreateDistributionError::InvalidErrorCode(parsed_error.message)),"InvalidForwardCookies" => return RusotoError::Service(CreateDistributionError::InvalidForwardCookies(parsed_error.message)),"InvalidGeoRestrictionParameter" => return RusotoError::Service(CreateDistributionError::InvalidGeoRestrictionParameter(parsed_error.message)),"InvalidHeadersForS3Origin" => return RusotoError::Service(CreateDistributionError::InvalidHeadersForS3Origin(parsed_error.message)),"InvalidLambdaFunctionAssociation" => return RusotoError::Service(CreateDistributionError::InvalidLambdaFunctionAssociation(parsed_error.message)),"InvalidLocationCode" => return RusotoError::Service(CreateDistributionError::InvalidLocationCode(parsed_error.message)),"InvalidMinimumProtocolVersion" => return RusotoError::Service(CreateDistributionError::InvalidMinimumProtocolVersion(parsed_error.message)),"InvalidOrigin" => return RusotoError::Service(CreateDistributionError::InvalidOrigin(parsed_error.message)),"InvalidOriginAccessIdentity" => return RusotoError::Service(CreateDistributionError::InvalidOriginAccessIdentity(parsed_error.message)),"InvalidOriginKeepaliveTimeout" => return RusotoError::Service(CreateDistributionError::InvalidOriginKeepaliveTimeout(parsed_error.message)),"InvalidOriginReadTimeout" => return RusotoError::Service(CreateDistributionError::InvalidOriginReadTimeout(parsed_error.message)),"InvalidProtocolSettings" => return RusotoError::Service(CreateDistributionError::InvalidProtocolSettings(parsed_error.message)),"InvalidQueryStringParameters" => return RusotoError::Service(CreateDistributionError::InvalidQueryStringParameters(parsed_error.message)),"InvalidRelativePath" => return RusotoError::Service(CreateDistributionError::InvalidRelativePath(parsed_error.message)),"InvalidRequiredProtocol" => return RusotoError::Service(CreateDistributionError::InvalidRequiredProtocol(parsed_error.message)),"InvalidResponseCode" => return RusotoError::Service(CreateDistributionError::InvalidResponseCode(parsed_error.message)),"InvalidTTLOrder" => return RusotoError::Service(CreateDistributionError::InvalidTTLOrder(parsed_error.message)),"InvalidViewerCertificate" => return RusotoError::Service(CreateDistributionError::InvalidViewerCertificate(parsed_error.message)),"InvalidWebACLId" => return RusotoError::Service(CreateDistributionError::InvalidWebACLId(parsed_error.message)),"MissingBody" => return RusotoError::Service(CreateDistributionError::MissingBody(parsed_error.message)),"NoSuchFieldLevelEncryptionConfig" => return RusotoError::Service(CreateDistributionError::NoSuchFieldLevelEncryptionConfig(parsed_error.message)),"NoSuchOrigin" => return RusotoError::Service(CreateDistributionError::NoSuchOrigin(parsed_error.message)),"TooManyCacheBehaviors" => return RusotoError::Service(CreateDistributionError::TooManyCacheBehaviors(parsed_error.message)),"TooManyCertificates" => return RusotoError::Service(CreateDistributionError::TooManyCertificates(parsed_error.message)),"TooManyCookieNamesInWhiteList" => return RusotoError::Service(CreateDistributionError::TooManyCookieNamesInWhiteList(parsed_error.message)),"TooManyDistributionCNAMEs" => return RusotoError::Service(CreateDistributionError::TooManyDistributionCNAMEs(parsed_error.message)),"TooManyDistributions" => return RusotoError::Service(CreateDistributionError::TooManyDistributions(parsed_error.message)),"TooManyDistributionsAssociatedToFieldLevelEncryptionConfig" => return RusotoError::Service(CreateDistributionError::TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(parsed_error.message)),"TooManyDistributionsWithLambdaAssociations" => return RusotoError::Service(CreateDistributionError::TooManyDistributionsWithLambdaAssociations(parsed_error.message)),"TooManyHeadersInForwardedValues" => return RusotoError::Service(CreateDistributionError::TooManyHeadersInForwardedValues(parsed_error.message)),"TooManyLambdaFunctionAssociations" => return RusotoError::Service(CreateDistributionError::TooManyLambdaFunctionAssociations(parsed_error.message)),"TooManyOriginCustomHeaders" => return RusotoError::Service(CreateDistributionError::TooManyOriginCustomHeaders(parsed_error.message)),"TooManyOriginGroupsPerDistribution" => return RusotoError::Service(CreateDistributionError::TooManyOriginGroupsPerDistribution(parsed_error.message)),"TooManyOrigins" => return RusotoError::Service(CreateDistributionError::TooManyOrigins(parsed_error.message)),"TooManyQueryStringParameters" => return RusotoError::Service(CreateDistributionError::TooManyQueryStringParameters(parsed_error.message)),"TooManyTrustedSigners" => return RusotoError::Service(CreateDistributionError::TooManyTrustedSigners(parsed_error.message)),"TrustedSignerDoesNotExist" => return RusotoError::Service(CreateDistributionError::TrustedSignerDoesNotExist(parsed_error.message)),_ => {}
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
CreateDistributionError::NoSuchFieldLevelEncryptionConfig(ref cause) => write!(f, "{}", cause),
CreateDistributionError::NoSuchOrigin(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyCacheBehaviors(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyCertificates(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyCookieNamesInWhiteList(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyDistributionCNAMEs(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyDistributions(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyDistributionsWithLambdaAssociations(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyHeadersInForwardedValues(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyLambdaFunctionAssociations(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyOriginCustomHeaders(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyOriginGroupsPerDistribution(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyOrigins(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyQueryStringParameters(ref cause) => write!(f, "{}", cause),
CreateDistributionError::TooManyTrustedSigners(ref cause) => write!(f, "{}", cause),
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

    CNAMEAlreadyExists(String),
    /// <p>The caller reference you attempted to create the distribution with is associated with another distribution.</p>
    DistributionAlreadyExists(String),
    /// <p>The specified configuration for field-level encryption can't be associated with the specified cache behavior.</p>
    IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The default root object file name is too big or contains an invalid character.</p>
    InvalidDefaultRootObject(String),

    InvalidErrorCode(String),
    /// <p>Your request contains forward cookies option which doesn't match with the expectation for the <code>whitelisted</code> list of cookie names. Either list of cookie names has been specified when not allowed or list of cookie names is missing when expected.</p>
    InvalidForwardCookies(String),

    InvalidGeoRestrictionParameter(String),

    InvalidHeadersForS3Origin(String),
    /// <p>The specified Lambda function association is invalid.</p>
    InvalidLambdaFunctionAssociation(String),

    InvalidLocationCode(String),

    InvalidMinimumProtocolVersion(String),
    /// <p>The Amazon S3 origin server specified does not refer to a valid Amazon S3 bucket.</p>
    InvalidOrigin(String),
    /// <p>The origin access identity is not valid or doesn't exist.</p>
    InvalidOriginAccessIdentity(String),

    InvalidOriginKeepaliveTimeout(String),

    InvalidOriginReadTimeout(String),
    /// <p>You cannot specify SSLv3 as the minimum protocol version if you only want to support only clients that support Server Name Indication (SNI).</p>
    InvalidProtocolSettings(String),

    InvalidQueryStringParameters(String),
    /// <p>The relative path is too big, is not URL-encoded, or does not begin with a slash (/).</p>
    InvalidRelativePath(String),
    /// <p>This operation requires the HTTPS protocol. Ensure that you specify the HTTPS protocol in your request, or omit the <code>RequiredProtocols</code> element from your distribution configuration.</p>
    InvalidRequiredProtocol(String),

    InvalidResponseCode(String),

    InvalidTTLOrder(String),

    InvalidTagging(String),

    InvalidViewerCertificate(String),

    InvalidWebACLId(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>The specified configuration for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionConfig(String),
    /// <p>No origin exists with the specified <code>Origin Id</code>. </p>
    NoSuchOrigin(String),
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
    /// <p>The maximum number of distributions have been associated with the specified configuration for field-level encryption.</p>
    TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(String),
    /// <p>Processing your request would cause the maximum number of distributions with Lambda function associations per owner to be exceeded.</p>
    TooManyDistributionsWithLambdaAssociations(String),

    TooManyHeadersInForwardedValues(String),
    /// <p>Your request contains more Lambda function associations than are allowed per distribution.</p>
    TooManyLambdaFunctionAssociations(String),

    TooManyOriginCustomHeaders(String),
    /// <p>Processing your request would cause you to exceed the maximum number of origin groups allowed.</p>
    TooManyOriginGroupsPerDistribution(String),
    /// <p>You cannot create more origins for the distribution.</p>
    TooManyOrigins(String),

    TooManyQueryStringParameters(String),
    /// <p>Your request contains more trusted signers than are allowed per distribution.</p>
    TooManyTrustedSigners(String),
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
                                    "AccessDenied" => return RusotoError::Service(CreateDistributionWithTagsError::AccessDenied(parsed_error.message)),"CNAMEAlreadyExists" => return RusotoError::Service(CreateDistributionWithTagsError::CNAMEAlreadyExists(parsed_error.message)),"DistributionAlreadyExists" => return RusotoError::Service(CreateDistributionWithTagsError::DistributionAlreadyExists(parsed_error.message)),"IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior" => return RusotoError::Service(CreateDistributionWithTagsError::IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(parsed_error.message)),"InconsistentQuantities" => return RusotoError::Service(CreateDistributionWithTagsError::InconsistentQuantities(parsed_error.message)),"InvalidArgument" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidArgument(parsed_error.message)),"InvalidDefaultRootObject" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidDefaultRootObject(parsed_error.message)),"InvalidErrorCode" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidErrorCode(parsed_error.message)),"InvalidForwardCookies" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidForwardCookies(parsed_error.message)),"InvalidGeoRestrictionParameter" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidGeoRestrictionParameter(parsed_error.message)),"InvalidHeadersForS3Origin" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidHeadersForS3Origin(parsed_error.message)),"InvalidLambdaFunctionAssociation" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidLambdaFunctionAssociation(parsed_error.message)),"InvalidLocationCode" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidLocationCode(parsed_error.message)),"InvalidMinimumProtocolVersion" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidMinimumProtocolVersion(parsed_error.message)),"InvalidOrigin" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidOrigin(parsed_error.message)),"InvalidOriginAccessIdentity" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidOriginAccessIdentity(parsed_error.message)),"InvalidOriginKeepaliveTimeout" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidOriginKeepaliveTimeout(parsed_error.message)),"InvalidOriginReadTimeout" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidOriginReadTimeout(parsed_error.message)),"InvalidProtocolSettings" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidProtocolSettings(parsed_error.message)),"InvalidQueryStringParameters" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidQueryStringParameters(parsed_error.message)),"InvalidRelativePath" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidRelativePath(parsed_error.message)),"InvalidRequiredProtocol" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidRequiredProtocol(parsed_error.message)),"InvalidResponseCode" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidResponseCode(parsed_error.message)),"InvalidTTLOrder" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidTTLOrder(parsed_error.message)),"InvalidTagging" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidTagging(parsed_error.message)),"InvalidViewerCertificate" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidViewerCertificate(parsed_error.message)),"InvalidWebACLId" => return RusotoError::Service(CreateDistributionWithTagsError::InvalidWebACLId(parsed_error.message)),"MissingBody" => return RusotoError::Service(CreateDistributionWithTagsError::MissingBody(parsed_error.message)),"NoSuchFieldLevelEncryptionConfig" => return RusotoError::Service(CreateDistributionWithTagsError::NoSuchFieldLevelEncryptionConfig(parsed_error.message)),"NoSuchOrigin" => return RusotoError::Service(CreateDistributionWithTagsError::NoSuchOrigin(parsed_error.message)),"TooManyCacheBehaviors" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyCacheBehaviors(parsed_error.message)),"TooManyCertificates" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyCertificates(parsed_error.message)),"TooManyCookieNamesInWhiteList" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyCookieNamesInWhiteList(parsed_error.message)),"TooManyDistributionCNAMEs" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyDistributionCNAMEs(parsed_error.message)),"TooManyDistributions" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyDistributions(parsed_error.message)),"TooManyDistributionsAssociatedToFieldLevelEncryptionConfig" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(parsed_error.message)),"TooManyDistributionsWithLambdaAssociations" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyDistributionsWithLambdaAssociations(parsed_error.message)),"TooManyHeadersInForwardedValues" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyHeadersInForwardedValues(parsed_error.message)),"TooManyLambdaFunctionAssociations" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyLambdaFunctionAssociations(parsed_error.message)),"TooManyOriginCustomHeaders" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyOriginCustomHeaders(parsed_error.message)),"TooManyOriginGroupsPerDistribution" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyOriginGroupsPerDistribution(parsed_error.message)),"TooManyOrigins" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyOrigins(parsed_error.message)),"TooManyQueryStringParameters" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyQueryStringParameters(parsed_error.message)),"TooManyTrustedSigners" => return RusotoError::Service(CreateDistributionWithTagsError::TooManyTrustedSigners(parsed_error.message)),"TrustedSignerDoesNotExist" => return RusotoError::Service(CreateDistributionWithTagsError::TrustedSignerDoesNotExist(parsed_error.message)),_ => {}
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
CreateDistributionWithTagsError::NoSuchFieldLevelEncryptionConfig(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::NoSuchOrigin(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyCacheBehaviors(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyCertificates(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyCookieNamesInWhiteList(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyDistributionCNAMEs(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyDistributions(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyDistributionsWithLambdaAssociations(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyHeadersInForwardedValues(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyLambdaFunctionAssociations(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyOriginCustomHeaders(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyOriginGroupsPerDistribution(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyOrigins(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyQueryStringParameters(ref cause) => write!(f, "{}", cause),
CreateDistributionWithTagsError::TooManyTrustedSigners(ref cause) => write!(f, "{}", cause),
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
    /// <p>The argument is invalid.</p>
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
        start_element("ErrorResponse", stack)?;
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
    /// <p>The argument is invalid.</p>
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
        start_element("ErrorResponse", stack)?;
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

    BatchTooLarge(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>The argument is invalid.</p>
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
        start_element("ErrorResponse", stack)?;
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
/// Errors returned by CreatePublicKey
#[derive(Debug, PartialEq)]
pub enum CreatePublicKeyError {
    /// <p>The argument is invalid.</p>
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
        start_element("ErrorResponse", stack)?;
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
/// Errors returned by CreateStreamingDistribution
#[derive(Debug, PartialEq)]
pub enum CreateStreamingDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),

    CNAMEAlreadyExists(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The Amazon S3 origin server specified does not refer to a valid Amazon S3 bucket.</p>
    InvalidOrigin(String),
    /// <p>The origin access identity is not valid or doesn't exist.</p>
    InvalidOriginAccessIdentity(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),

    StreamingDistributionAlreadyExists(String),

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
        start_element("ErrorResponse", stack)?;
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

    CNAMEAlreadyExists(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The Amazon S3 origin server specified does not refer to a valid Amazon S3 bucket.</p>
    InvalidOrigin(String),
    /// <p>The origin access identity is not valid or doesn't exist.</p>
    InvalidOriginAccessIdentity(String),

    InvalidTagging(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),

    StreamingDistributionAlreadyExists(String),

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
        start_element("ErrorResponse", stack)?;
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
/// Errors returned by DeleteCloudFrontOriginAccessIdentity
#[derive(Debug, PartialEq)]
pub enum DeleteCloudFrontOriginAccessIdentityError {
    /// <p>Access denied.</p>
    AccessDenied(String),

    CloudFrontOriginAccessIdentityInUse(String),
    /// <p>The <code>If-Match</code> version is missing or not valid for the distribution.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified origin access identity does not exist.</p>
    NoSuchCloudFrontOriginAccessIdentity(String),
    /// <p>The precondition given in one or more of the request-header fields evaluated to <code>false</code>. </p>
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
        start_element("ErrorResponse", stack)?;
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

    DistributionNotDisabled(String),
    /// <p>The <code>If-Match</code> version is missing or not valid for the distribution.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
    /// <p>The precondition given in one or more of the request-header fields evaluated to <code>false</code>. </p>
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
        start_element("ErrorResponse", stack)?;
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
    /// <p>The <code>If-Match</code> version is missing or not valid for the distribution.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified configuration for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionConfig(String),
    /// <p>The precondition given in one or more of the request-header fields evaluated to <code>false</code>. </p>
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
        start_element("ErrorResponse", stack)?;
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
    /// <p>The <code>If-Match</code> version is missing or not valid for the distribution.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified profile for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionProfile(String),
    /// <p>The precondition given in one or more of the request-header fields evaluated to <code>false</code>. </p>
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
        start_element("ErrorResponse", stack)?;
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
/// Errors returned by DeletePublicKey
#[derive(Debug, PartialEq)]
pub enum DeletePublicKeyError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The <code>If-Match</code> version is missing or not valid for the distribution.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified public key doesn't exist.</p>
    NoSuchPublicKey(String),
    /// <p>The precondition given in one or more of the request-header fields evaluated to <code>false</code>. </p>
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
        start_element("ErrorResponse", stack)?;
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
/// Errors returned by DeleteStreamingDistribution
#[derive(Debug, PartialEq)]
pub enum DeleteStreamingDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The <code>If-Match</code> version is missing or not valid for the distribution.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified streaming distribution does not exist.</p>
    NoSuchStreamingDistribution(String),
    /// <p>The precondition given in one or more of the request-header fields evaluated to <code>false</code>. </p>
    PreconditionFailed(String),

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
        start_element("ErrorResponse", stack)?;
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
        start_element("ErrorResponse", stack)?;
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
        start_element("ErrorResponse", stack)?;
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
        start_element("ErrorResponse", stack)?;
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
        start_element("ErrorResponse", stack)?;
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
        start_element("ErrorResponse", stack)?;
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
        start_element("ErrorResponse", stack)?;
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
        start_element("ErrorResponse", stack)?;
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
        start_element("ErrorResponse", stack)?;
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
        start_element("ErrorResponse", stack)?;
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
        start_element("ErrorResponse", stack)?;
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
        start_element("ErrorResponse", stack)?;
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
        start_element("ErrorResponse", stack)?;
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
        start_element("ErrorResponse", stack)?;
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
/// Errors returned by ListCloudFrontOriginAccessIdentities
#[derive(Debug, PartialEq)]
pub enum ListCloudFrontOriginAccessIdentitiesError {
    /// <p>The argument is invalid.</p>
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
        start_element("ErrorResponse", stack)?;
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
    /// <p>The argument is invalid.</p>
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
        start_element("ErrorResponse", stack)?;
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
/// Errors returned by ListDistributionsByWebACLId
#[derive(Debug, PartialEq)]
pub enum ListDistributionsByWebACLIdError {
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),

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
        start_element("ErrorResponse", stack)?;
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
    /// <p>The argument is invalid.</p>
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
        start_element("ErrorResponse", stack)?;
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
    /// <p>The argument is invalid.</p>
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
        start_element("ErrorResponse", stack)?;
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
    /// <p>The argument is invalid.</p>
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
        start_element("ErrorResponse", stack)?;
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
/// Errors returned by ListPublicKeys
#[derive(Debug, PartialEq)]
pub enum ListPublicKeysError {
    /// <p>The argument is invalid.</p>
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
        start_element("ErrorResponse", stack)?;
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
/// Errors returned by ListStreamingDistributions
#[derive(Debug, PartialEq)]
pub enum ListStreamingDistributionsError {
    /// <p>The argument is invalid.</p>
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
        start_element("ErrorResponse", stack)?;
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
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),

    InvalidTagging(String),

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
        start_element("ErrorResponse", stack)?;
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
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),

    InvalidTagging(String),

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
        start_element("ErrorResponse", stack)?;
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
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),

    InvalidTagging(String),

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
        start_element("ErrorResponse", stack)?;
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
/// Errors returned by UpdateCloudFrontOriginAccessIdentity
#[derive(Debug, PartialEq)]
pub enum UpdateCloudFrontOriginAccessIdentityError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>Origin and <code>CallerReference</code> cannot be updated. </p>
    IllegalUpdate(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The <code>If-Match</code> version is missing or not valid for the distribution.</p>
    InvalidIfMatchVersion(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>The specified origin access identity does not exist.</p>
    NoSuchCloudFrontOriginAccessIdentity(String),
    /// <p>The precondition given in one or more of the request-header fields evaluated to <code>false</code>. </p>
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
        start_element("ErrorResponse", stack)?;
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

    CNAMEAlreadyExists(String),
    /// <p>The specified configuration for field-level encryption can't be associated with the specified cache behavior.</p>
    IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(String),
    /// <p>Origin and <code>CallerReference</code> cannot be updated. </p>
    IllegalUpdate(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The default root object file name is too big or contains an invalid character.</p>
    InvalidDefaultRootObject(String),

    InvalidErrorCode(String),
    /// <p>Your request contains forward cookies option which doesn't match with the expectation for the <code>whitelisted</code> list of cookie names. Either list of cookie names has been specified when not allowed or list of cookie names is missing when expected.</p>
    InvalidForwardCookies(String),

    InvalidGeoRestrictionParameter(String),

    InvalidHeadersForS3Origin(String),
    /// <p>The <code>If-Match</code> version is missing or not valid for the distribution.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified Lambda function association is invalid.</p>
    InvalidLambdaFunctionAssociation(String),

    InvalidLocationCode(String),

    InvalidMinimumProtocolVersion(String),
    /// <p>The origin access identity is not valid or doesn't exist.</p>
    InvalidOriginAccessIdentity(String),

    InvalidOriginKeepaliveTimeout(String),

    InvalidOriginReadTimeout(String),

    InvalidQueryStringParameters(String),
    /// <p>The relative path is too big, is not URL-encoded, or does not begin with a slash (/).</p>
    InvalidRelativePath(String),
    /// <p>This operation requires the HTTPS protocol. Ensure that you specify the HTTPS protocol in your request, or omit the <code>RequiredProtocols</code> element from your distribution configuration.</p>
    InvalidRequiredProtocol(String),

    InvalidResponseCode(String),

    InvalidTTLOrder(String),

    InvalidViewerCertificate(String),

    InvalidWebACLId(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
    /// <p>The specified configuration for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionConfig(String),
    /// <p>No origin exists with the specified <code>Origin Id</code>. </p>
    NoSuchOrigin(String),
    /// <p>The precondition given in one or more of the request-header fields evaluated to <code>false</code>. </p>
    PreconditionFailed(String),
    /// <p>You cannot create more cache behaviors for the distribution.</p>
    TooManyCacheBehaviors(String),
    /// <p>You cannot create anymore custom SSL/TLS certificates.</p>
    TooManyCertificates(String),
    /// <p>Your request contains more cookie names in the whitelist than are allowed per cache behavior.</p>
    TooManyCookieNamesInWhiteList(String),
    /// <p>Your request contains more CNAMEs than are allowed per distribution.</p>
    TooManyDistributionCNAMEs(String),
    /// <p>The maximum number of distributions have been associated with the specified configuration for field-level encryption.</p>
    TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(String),
    /// <p>Processing your request would cause the maximum number of distributions with Lambda function associations per owner to be exceeded.</p>
    TooManyDistributionsWithLambdaAssociations(String),

    TooManyHeadersInForwardedValues(String),
    /// <p>Your request contains more Lambda function associations than are allowed per distribution.</p>
    TooManyLambdaFunctionAssociations(String),

    TooManyOriginCustomHeaders(String),
    /// <p>Processing your request would cause you to exceed the maximum number of origin groups allowed.</p>
    TooManyOriginGroupsPerDistribution(String),
    /// <p>You cannot create more origins for the distribution.</p>
    TooManyOrigins(String),

    TooManyQueryStringParameters(String),
    /// <p>Your request contains more trusted signers than are allowed per distribution.</p>
    TooManyTrustedSigners(String),
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
                                    "AccessDenied" => return RusotoError::Service(UpdateDistributionError::AccessDenied(parsed_error.message)),"CNAMEAlreadyExists" => return RusotoError::Service(UpdateDistributionError::CNAMEAlreadyExists(parsed_error.message)),"IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior" => return RusotoError::Service(UpdateDistributionError::IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior(parsed_error.message)),"IllegalUpdate" => return RusotoError::Service(UpdateDistributionError::IllegalUpdate(parsed_error.message)),"InconsistentQuantities" => return RusotoError::Service(UpdateDistributionError::InconsistentQuantities(parsed_error.message)),"InvalidArgument" => return RusotoError::Service(UpdateDistributionError::InvalidArgument(parsed_error.message)),"InvalidDefaultRootObject" => return RusotoError::Service(UpdateDistributionError::InvalidDefaultRootObject(parsed_error.message)),"InvalidErrorCode" => return RusotoError::Service(UpdateDistributionError::InvalidErrorCode(parsed_error.message)),"InvalidForwardCookies" => return RusotoError::Service(UpdateDistributionError::InvalidForwardCookies(parsed_error.message)),"InvalidGeoRestrictionParameter" => return RusotoError::Service(UpdateDistributionError::InvalidGeoRestrictionParameter(parsed_error.message)),"InvalidHeadersForS3Origin" => return RusotoError::Service(UpdateDistributionError::InvalidHeadersForS3Origin(parsed_error.message)),"InvalidIfMatchVersion" => return RusotoError::Service(UpdateDistributionError::InvalidIfMatchVersion(parsed_error.message)),"InvalidLambdaFunctionAssociation" => return RusotoError::Service(UpdateDistributionError::InvalidLambdaFunctionAssociation(parsed_error.message)),"InvalidLocationCode" => return RusotoError::Service(UpdateDistributionError::InvalidLocationCode(parsed_error.message)),"InvalidMinimumProtocolVersion" => return RusotoError::Service(UpdateDistributionError::InvalidMinimumProtocolVersion(parsed_error.message)),"InvalidOriginAccessIdentity" => return RusotoError::Service(UpdateDistributionError::InvalidOriginAccessIdentity(parsed_error.message)),"InvalidOriginKeepaliveTimeout" => return RusotoError::Service(UpdateDistributionError::InvalidOriginKeepaliveTimeout(parsed_error.message)),"InvalidOriginReadTimeout" => return RusotoError::Service(UpdateDistributionError::InvalidOriginReadTimeout(parsed_error.message)),"InvalidQueryStringParameters" => return RusotoError::Service(UpdateDistributionError::InvalidQueryStringParameters(parsed_error.message)),"InvalidRelativePath" => return RusotoError::Service(UpdateDistributionError::InvalidRelativePath(parsed_error.message)),"InvalidRequiredProtocol" => return RusotoError::Service(UpdateDistributionError::InvalidRequiredProtocol(parsed_error.message)),"InvalidResponseCode" => return RusotoError::Service(UpdateDistributionError::InvalidResponseCode(parsed_error.message)),"InvalidTTLOrder" => return RusotoError::Service(UpdateDistributionError::InvalidTTLOrder(parsed_error.message)),"InvalidViewerCertificate" => return RusotoError::Service(UpdateDistributionError::InvalidViewerCertificate(parsed_error.message)),"InvalidWebACLId" => return RusotoError::Service(UpdateDistributionError::InvalidWebACLId(parsed_error.message)),"MissingBody" => return RusotoError::Service(UpdateDistributionError::MissingBody(parsed_error.message)),"NoSuchDistribution" => return RusotoError::Service(UpdateDistributionError::NoSuchDistribution(parsed_error.message)),"NoSuchFieldLevelEncryptionConfig" => return RusotoError::Service(UpdateDistributionError::NoSuchFieldLevelEncryptionConfig(parsed_error.message)),"NoSuchOrigin" => return RusotoError::Service(UpdateDistributionError::NoSuchOrigin(parsed_error.message)),"PreconditionFailed" => return RusotoError::Service(UpdateDistributionError::PreconditionFailed(parsed_error.message)),"TooManyCacheBehaviors" => return RusotoError::Service(UpdateDistributionError::TooManyCacheBehaviors(parsed_error.message)),"TooManyCertificates" => return RusotoError::Service(UpdateDistributionError::TooManyCertificates(parsed_error.message)),"TooManyCookieNamesInWhiteList" => return RusotoError::Service(UpdateDistributionError::TooManyCookieNamesInWhiteList(parsed_error.message)),"TooManyDistributionCNAMEs" => return RusotoError::Service(UpdateDistributionError::TooManyDistributionCNAMEs(parsed_error.message)),"TooManyDistributionsAssociatedToFieldLevelEncryptionConfig" => return RusotoError::Service(UpdateDistributionError::TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(parsed_error.message)),"TooManyDistributionsWithLambdaAssociations" => return RusotoError::Service(UpdateDistributionError::TooManyDistributionsWithLambdaAssociations(parsed_error.message)),"TooManyHeadersInForwardedValues" => return RusotoError::Service(UpdateDistributionError::TooManyHeadersInForwardedValues(parsed_error.message)),"TooManyLambdaFunctionAssociations" => return RusotoError::Service(UpdateDistributionError::TooManyLambdaFunctionAssociations(parsed_error.message)),"TooManyOriginCustomHeaders" => return RusotoError::Service(UpdateDistributionError::TooManyOriginCustomHeaders(parsed_error.message)),"TooManyOriginGroupsPerDistribution" => return RusotoError::Service(UpdateDistributionError::TooManyOriginGroupsPerDistribution(parsed_error.message)),"TooManyOrigins" => return RusotoError::Service(UpdateDistributionError::TooManyOrigins(parsed_error.message)),"TooManyQueryStringParameters" => return RusotoError::Service(UpdateDistributionError::TooManyQueryStringParameters(parsed_error.message)),"TooManyTrustedSigners" => return RusotoError::Service(UpdateDistributionError::TooManyTrustedSigners(parsed_error.message)),"TrustedSignerDoesNotExist" => return RusotoError::Service(UpdateDistributionError::TrustedSignerDoesNotExist(parsed_error.message)),_ => {}
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
UpdateDistributionError::NoSuchDistribution(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::NoSuchFieldLevelEncryptionConfig(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::NoSuchOrigin(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyCacheBehaviors(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyCertificates(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyCookieNamesInWhiteList(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyDistributionCNAMEs(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyDistributionsAssociatedToFieldLevelEncryptionConfig(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyDistributionsWithLambdaAssociations(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyHeadersInForwardedValues(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyLambdaFunctionAssociations(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyOriginCustomHeaders(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyOriginGroupsPerDistribution(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyOrigins(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyQueryStringParameters(ref cause) => write!(f, "{}", cause),
UpdateDistributionError::TooManyTrustedSigners(ref cause) => write!(f, "{}", cause),
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
    /// <p>Origin and <code>CallerReference</code> cannot be updated. </p>
    IllegalUpdate(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The <code>If-Match</code> version is missing or not valid for the distribution.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified configuration for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionConfig(String),
    /// <p>The specified profile for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionProfile(String),
    /// <p>The precondition given in one or more of the request-header fields evaluated to <code>false</code>. </p>
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
        start_element("ErrorResponse", stack)?;
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
    /// <p>Origin and <code>CallerReference</code> cannot be updated. </p>
    IllegalUpdate(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The <code>If-Match</code> version is missing or not valid for the distribution.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified profile for field-level encryption doesn't exist.</p>
    NoSuchFieldLevelEncryptionProfile(String),
    /// <p>The specified public key doesn't exist.</p>
    NoSuchPublicKey(String),
    /// <p>The precondition given in one or more of the request-header fields evaluated to <code>false</code>. </p>
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
        start_element("ErrorResponse", stack)?;
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
/// Errors returned by UpdatePublicKey
#[derive(Debug, PartialEq)]
pub enum UpdatePublicKeyError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>You can't change the value of a public key.</p>
    CannotChangeImmutablePublicKeyFields(String),
    /// <p>Origin and <code>CallerReference</code> cannot be updated. </p>
    IllegalUpdate(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The <code>If-Match</code> version is missing or not valid for the distribution.</p>
    InvalidIfMatchVersion(String),
    /// <p>The specified public key doesn't exist.</p>
    NoSuchPublicKey(String),
    /// <p>The precondition given in one or more of the request-header fields evaluated to <code>false</code>. </p>
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
        start_element("ErrorResponse", stack)?;
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
/// Errors returned by UpdateStreamingDistribution
#[derive(Debug, PartialEq)]
pub enum UpdateStreamingDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),

    CNAMEAlreadyExists(String),
    /// <p>Origin and <code>CallerReference</code> cannot be updated. </p>
    IllegalUpdate(String),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The <code>If-Match</code> version is missing or not valid for the distribution.</p>
    InvalidIfMatchVersion(String),
    /// <p>The origin access identity is not valid or doesn't exist.</p>
    InvalidOriginAccessIdentity(String),
    /// <p>This operation requires a body. Ensure that the body is present and the <code>Content-Type</code> header is set.</p>
    MissingBody(String),
    /// <p>The specified streaming distribution does not exist.</p>
    NoSuchStreamingDistribution(String),
    /// <p>The precondition given in one or more of the request-header fields evaluated to <code>false</code>. </p>
    PreconditionFailed(String),

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
        start_element("ErrorResponse", stack)?;
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
    /// <p>Creates a new origin access identity. If you're using Amazon S3 for your origin, you can use an origin access identity to require users to access your content using a CloudFront URL instead of the Amazon S3 URL. For more information about how to use origin access identities, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    async fn create_cloud_front_origin_access_identity(
        &self,
        input: CreateCloudFrontOriginAccessIdentityRequest,
    ) -> Result<
        CreateCloudFrontOriginAccessIdentityResult,
        RusotoError<CreateCloudFrontOriginAccessIdentityError>,
    >;

    /// <p>Creates a new web distribution. You create a CloudFront distribution to tell CloudFront where you want content to be delivered from, and the details about how to track and manage content delivery. Send a <code>POST</code> request to the <code>/<i>CloudFront API version</i>/distribution</code>/<code>distribution ID</code> resource.</p> <important> <p>When you update a distribution, there are more required fields than when you create a distribution. When you update your distribution by using <a>UpdateDistribution</a>, follow the steps included in the documentation to get the current configuration and then make your updates. This helps to make sure that you include all of the required fields. To view a summary, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-overview-required-fields.html">Required Fields for Create Distribution and Update Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </important> <p>If you are using Adobe Flash Media Server's RTMP protocol, you set up a different kind of CloudFront distribution. For more information, see <a>CreateStreamingDistribution</a>.</p>
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

    /// <p>Add a new public key to CloudFront to use, for example, for field-level encryption. You can add a maximum of 10 public keys with one AWS account.</p>
    async fn create_public_key(
        &self,
        input: CreatePublicKeyRequest,
    ) -> Result<CreatePublicKeyResult, RusotoError<CreatePublicKeyError>>;

    /// <p><p>Creates a new RMTP distribution. An RTMP distribution is similar to a web distribution, but an RTMP distribution streams media files using the Adobe Real-Time Messaging Protocol (RTMP) instead of serving files using HTTP. </p> <p>To create a new web distribution, submit a <code>POST</code> request to the <i>CloudFront API version</i>/distribution resource. The request body must include a document with a <i>StreamingDistributionConfig</i> element. The response echoes the <code>StreamingDistributionConfig</code> element and returns other information about the RTMP distribution.</p> <p>To get the status of your request, use the <i>GET StreamingDistribution</i> API action. When the value of <code>Enabled</code> is <code>true</code> and the value of <code>Status</code> is <code>Deployed</code>, your distribution is ready. A distribution usually deploys in less than 15 minutes.</p> <p>For more information about web distributions, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-rtmp.html">Working with RTMP Distributions</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <important> <p>Beginning with the 2012-05-05 version of the CloudFront API, we made substantial changes to the format of the XML document that you include in the request body when you create or update a web distribution or an RTMP distribution, and when you invalidate objects. With previous versions of the API, we discovered that it was too easy to accidentally delete one or more values for an element that accepts multiple values, for example, CNAMEs and trusted signers. Our changes for the 2012-05-05 release are intended to prevent these accidental deletions and to notify you when there&#39;s a mismatch between the number of values you say you&#39;re specifying in the <code>Quantity</code> element and the number of values specified.</p> </important></p>
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

    /// <p>Remove a public key you previously added to CloudFront.</p>
    async fn delete_public_key(
        &self,
        input: DeletePublicKeyRequest,
    ) -> Result<(), RusotoError<DeletePublicKeyError>>;

    /// <p>Delete a streaming distribution. To delete an RTMP distribution using the CloudFront API, perform the following steps.</p> <p> <b>To delete an RTMP distribution using the CloudFront API</b>:</p> <ol> <li> <p>Disable the RTMP distribution.</p> </li> <li> <p>Submit a <code>GET Streaming Distribution Config</code> request to get the current configuration and the <code>Etag</code> header for the distribution. </p> </li> <li> <p>Update the XML document that was returned in the response to your <code>GET Streaming Distribution Config</code> request to change the value of <code>Enabled</code> to <code>false</code>.</p> </li> <li> <p>Submit a <code>PUT Streaming Distribution Config</code> request to update the configuration for your distribution. In the request body, include the XML document that you updated in Step 3. Then set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Streaming Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to the <code>PUT Streaming Distribution Config</code> request to confirm that the distribution was successfully disabled.</p> </li> <li> <p>Submit a <code>GET Streaming Distribution Config</code> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> </li> <li> <p>Submit a <code>DELETE Streaming Distribution</code> request. Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Streaming Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to your <code>DELETE Streaming Distribution</code> request to confirm that the distribution was successfully deleted.</p> </li> </ol> <p>For information about deleting a distribution using the CloudFront console, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/HowToDeleteDistribution.html">Deleting a Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    async fn delete_streaming_distribution(
        &self,
        input: DeleteStreamingDistributionRequest,
    ) -> Result<(), RusotoError<DeleteStreamingDistributionError>>;

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

    /// <p>Get the information about a distribution. </p>
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

    /// <p>Get the public key information.</p>
    async fn get_public_key(
        &self,
        input: GetPublicKeyRequest,
    ) -> Result<GetPublicKeyResult, RusotoError<GetPublicKeyError>>;

    /// <p>Return public key configuration informaation</p>
    async fn get_public_key_config(
        &self,
        input: GetPublicKeyConfigRequest,
    ) -> Result<GetPublicKeyConfigResult, RusotoError<GetPublicKeyConfigError>>;

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

    /// <p>Lists origin access identities.</p>
    async fn list_cloud_front_origin_access_identities(
        &self,
        input: ListCloudFrontOriginAccessIdentitiesRequest,
    ) -> Result<
        ListCloudFrontOriginAccessIdentitiesResult,
        RusotoError<ListCloudFrontOriginAccessIdentitiesError>,
    >;

    /// <p>List distributions. </p>
    async fn list_distributions(
        &self,
        input: ListDistributionsRequest,
    ) -> Result<ListDistributionsResult, RusotoError<ListDistributionsError>>;

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

    /// <p>List all public keys that have been added to CloudFront for this account.</p>
    async fn list_public_keys(
        &self,
        input: ListPublicKeysRequest,
    ) -> Result<ListPublicKeysResult, RusotoError<ListPublicKeysError>>;

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

    /// <p>Update an origin access identity. </p>
    async fn update_cloud_front_origin_access_identity(
        &self,
        input: UpdateCloudFrontOriginAccessIdentityRequest,
    ) -> Result<
        UpdateCloudFrontOriginAccessIdentityResult,
        RusotoError<UpdateCloudFrontOriginAccessIdentityError>,
    >;

    /// <p><p>Updates the configuration for a web distribution. </p> <important> <p>When you update a distribution, there are more required fields than when you create a distribution. When you update your distribution by using this API action, follow the steps here to get the current configuration and then make your updates, to make sure that you include all of the required fields. To view a summary, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-overview-required-fields.html">Required Fields for Create Distribution and Update Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </important> <p>The update process includes getting the current distribution configuration, updating the XML document that is returned to make your changes, and then submitting an <code>UpdateDistribution</code> request to make the updates.</p> <p>For information about updating a distribution using the CloudFront console instead, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-creating-console.html">Creating a Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p> <b>To update a web distribution using the CloudFront API</b> </p> <ol> <li> <p>Submit a <a>GetDistributionConfig</a> request to get the current configuration and an <code>Etag</code> header for the distribution.</p> <note> <p>If you update the distribution again, you must get a new <code>Etag</code> header.</p> </note> </li> <li> <p>Update the XML document that was returned in the response to your <code>GetDistributionConfig</code> request to include your changes. </p> <important> <p>When you edit the XML file, be aware of the following:</p> <ul> <li> <p>You must strip out the ETag parameter that is returned.</p> </li> <li> <p>Additional fields are required when you update a distribution. There may be fields included in the XML file for features that you haven&#39;t configured for your distribution. This is expected and required to successfully update the distribution.</p> </li> <li> <p>You can&#39;t change the value of <code>CallerReference</code>. If you try to change this value, CloudFront returns an <code>IllegalUpdate</code> error. </p> </li> <li> <p>The new configuration replaces the existing configuration; the values that you specify in an <code>UpdateDistribution</code> request are not merged into your existing configuration. When you add, delete, or replace values in an element that allows multiple values (for example, <code>CNAME</code>), you must specify all of the values that you want to appear in the updated distribution. In addition, you must update the corresponding <code>Quantity</code> element.</p> </li> </ul> </important> </li> <li> <p>Submit an <code>UpdateDistribution</code> request to update the configuration for your distribution:</p> <ul> <li> <p>In the request body, include the XML document that you updated in Step 2. The request body must include an XML document with a <code>DistributionConfig</code> element.</p> </li> <li> <p>Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GetDistributionConfig</code> request in Step 1.</p> </li> </ul> </li> <li> <p>Review the response to the <code>UpdateDistribution</code> request to confirm that the configuration was successfully updated.</p> </li> <li> <p>Optional: Submit a <a>GetDistribution</a> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> </li> </ol></p>
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

    /// <p>Update public key information. Note that the only value you can change is the comment.</p>
    async fn update_public_key(
        &self,
        input: UpdatePublicKeyRequest,
    ) -> Result<UpdatePublicKeyResult, RusotoError<UpdatePublicKeyError>>;

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
    /// <p>Creates a new origin access identity. If you're using Amazon S3 for your origin, you can use an origin access identity to require users to access your content using a CloudFront URL instead of the Amazon S3 URL. For more information about how to use origin access identities, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn create_cloud_front_origin_access_identity(
        &self,
        input: CreateCloudFrontOriginAccessIdentityRequest,
    ) -> Result<
        CreateCloudFrontOriginAccessIdentityResult,
        RusotoError<CreateCloudFrontOriginAccessIdentityError>,
    > {
        let request_uri = "/2018-11-05/origin-access-identity/cloudfront";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CloudFrontOriginAccessIdentityConfigSerializer::serialize(
            &mut writer,
            "CloudFrontOriginAccessIdentityConfig",
            &input.cloud_front_origin_access_identity_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateCloudFrontOriginAccessIdentityError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateCloudFrontOriginAccessIdentityResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = CreateCloudFrontOriginAccessIdentityResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        };
        if let Some(location) = response.headers.get("Location") {
            let value = location.to_owned();
            result.location = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p>Creates a new web distribution. You create a CloudFront distribution to tell CloudFront where you want content to be delivered from, and the details about how to track and manage content delivery. Send a <code>POST</code> request to the <code>/<i>CloudFront API version</i>/distribution</code>/<code>distribution ID</code> resource.</p> <important> <p>When you update a distribution, there are more required fields than when you create a distribution. When you update your distribution by using <a>UpdateDistribution</a>, follow the steps included in the documentation to get the current configuration and then make your updates. This helps to make sure that you include all of the required fields. To view a summary, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-overview-required-fields.html">Required Fields for Create Distribution and Update Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </important> <p>If you are using Adobe Flash Media Server's RTMP protocol, you set up a different kind of CloudFront distribution. For more information, see <a>CreateStreamingDistribution</a>.</p>
    #[allow(unused_variables, warnings)]
    async fn create_distribution(
        &self,
        input: CreateDistributionRequest,
    ) -> Result<CreateDistributionResult, RusotoError<CreateDistributionError>> {
        let request_uri = "/2018-11-05/distribution";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        DistributionConfigSerializer::serialize(
            &mut writer,
            "DistributionConfig",
            &input.distribution_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateDistributionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateDistributionResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                CreateDistributionResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        };
        if let Some(location) = response.headers.get("Location") {
            let value = location.to_owned();
            result.location = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p>Create a new distribution with tags.</p>
    #[allow(unused_variables, warnings)]
    async fn create_distribution_with_tags(
        &self,
        input: CreateDistributionWithTagsRequest,
    ) -> Result<CreateDistributionWithTagsResult, RusotoError<CreateDistributionWithTagsError>>
    {
        let request_uri = "/2018-11-05/distribution";

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
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateDistributionWithTagsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateDistributionWithTagsResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = CreateDistributionWithTagsResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        };
        if let Some(location) = response.headers.get("Location") {
            let value = location.to_owned();
            result.location = Some(value)
        }; // parse non-payload
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
        let request_uri = "/2018-11-05/field-level-encryption";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        FieldLevelEncryptionConfigSerializer::serialize(
            &mut writer,
            "FieldLevelEncryptionConfig",
            &input.field_level_encryption_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateFieldLevelEncryptionConfigError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateFieldLevelEncryptionConfigResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = CreateFieldLevelEncryptionConfigResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        };
        if let Some(location) = response.headers.get("Location") {
            let value = location.to_owned();
            result.location = Some(value)
        }; // parse non-payload
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
        let request_uri = "/2018-11-05/field-level-encryption-profile";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        FieldLevelEncryptionProfileConfigSerializer::serialize(
            &mut writer,
            "FieldLevelEncryptionProfileConfig",
            &input.field_level_encryption_profile_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateFieldLevelEncryptionProfileError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateFieldLevelEncryptionProfileResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = CreateFieldLevelEncryptionProfileResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        };
        if let Some(location) = response.headers.get("Location") {
            let value = location.to_owned();
            result.location = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p>Create a new invalidation. </p>
    #[allow(unused_variables, warnings)]
    async fn create_invalidation(
        &self,
        input: CreateInvalidationRequest,
    ) -> Result<CreateInvalidationResult, RusotoError<CreateInvalidationError>> {
        let request_uri = format!(
            "/2018-11-05/distribution/{distribution_id}/invalidation",
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
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateInvalidationError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateInvalidationResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                CreateInvalidationResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        if let Some(location) = response.headers.get("Location") {
            let value = location.to_owned();
            result.location = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p>Add a new public key to CloudFront to use, for example, for field-level encryption. You can add a maximum of 10 public keys with one AWS account.</p>
    #[allow(unused_variables, warnings)]
    async fn create_public_key(
        &self,
        input: CreatePublicKeyRequest,
    ) -> Result<CreatePublicKeyResult, RusotoError<CreatePublicKeyError>> {
        let request_uri = "/2018-11-05/public-key";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        PublicKeyConfigSerializer::serialize(
            &mut writer,
            "PublicKeyConfig",
            &input.public_key_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreatePublicKeyError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreatePublicKeyResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = CreatePublicKeyResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        };
        if let Some(location) = response.headers.get("Location") {
            let value = location.to_owned();
            result.location = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p><p>Creates a new RMTP distribution. An RTMP distribution is similar to a web distribution, but an RTMP distribution streams media files using the Adobe Real-Time Messaging Protocol (RTMP) instead of serving files using HTTP. </p> <p>To create a new web distribution, submit a <code>POST</code> request to the <i>CloudFront API version</i>/distribution resource. The request body must include a document with a <i>StreamingDistributionConfig</i> element. The response echoes the <code>StreamingDistributionConfig</code> element and returns other information about the RTMP distribution.</p> <p>To get the status of your request, use the <i>GET StreamingDistribution</i> API action. When the value of <code>Enabled</code> is <code>true</code> and the value of <code>Status</code> is <code>Deployed</code>, your distribution is ready. A distribution usually deploys in less than 15 minutes.</p> <p>For more information about web distributions, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-rtmp.html">Working with RTMP Distributions</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <important> <p>Beginning with the 2012-05-05 version of the CloudFront API, we made substantial changes to the format of the XML document that you include in the request body when you create or update a web distribution or an RTMP distribution, and when you invalidate objects. With previous versions of the API, we discovered that it was too easy to accidentally delete one or more values for an element that accepts multiple values, for example, CNAMEs and trusted signers. Our changes for the 2012-05-05 release are intended to prevent these accidental deletions and to notify you when there&#39;s a mismatch between the number of values you say you&#39;re specifying in the <code>Quantity</code> element and the number of values specified.</p> </important></p>
    #[allow(unused_variables, warnings)]
    async fn create_streaming_distribution(
        &self,
        input: CreateStreamingDistributionRequest,
    ) -> Result<CreateStreamingDistributionResult, RusotoError<CreateStreamingDistributionError>>
    {
        let request_uri = "/2018-11-05/streaming-distribution";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        StreamingDistributionConfigSerializer::serialize(
            &mut writer,
            "StreamingDistributionConfig",
            &input.streaming_distribution_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateStreamingDistributionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateStreamingDistributionResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = CreateStreamingDistributionResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        };
        if let Some(location) = response.headers.get("Location") {
            let value = location.to_owned();
            result.location = Some(value)
        }; // parse non-payload
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
        let request_uri = "/2018-11-05/streaming-distribution";

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
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateStreamingDistributionWithTagsError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateStreamingDistributionWithTagsResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = CreateStreamingDistributionWithTagsResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        };
        if let Some(location) = response.headers.get("Location") {
            let value = location.to_owned();
            result.location = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p>Delete an origin access identity. </p>
    #[allow(unused_variables, warnings)]
    async fn delete_cloud_front_origin_access_identity(
        &self,
        input: DeleteCloudFrontOriginAccessIdentityRequest,
    ) -> Result<(), RusotoError<DeleteCloudFrontOriginAccessIdentityError>> {
        let request_uri = format!(
            "/2018-11-05/origin-access-identity/cloudfront/{id}",
            id = input.id
        );

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteCloudFrontOriginAccessIdentityError::from_response(
                response,
            ));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Delete a distribution. </p>
    #[allow(unused_variables, warnings)]
    async fn delete_distribution(
        &self,
        input: DeleteDistributionRequest,
    ) -> Result<(), RusotoError<DeleteDistributionError>> {
        let request_uri = format!("/2018-11-05/distribution/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteDistributionError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Remove a field-level encryption configuration.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_field_level_encryption_config(
        &self,
        input: DeleteFieldLevelEncryptionConfigRequest,
    ) -> Result<(), RusotoError<DeleteFieldLevelEncryptionConfigError>> {
        let request_uri = format!("/2018-11-05/field-level-encryption/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteFieldLevelEncryptionConfigError::from_response(
                response,
            ));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Remove a field-level encryption profile.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_field_level_encryption_profile(
        &self,
        input: DeleteFieldLevelEncryptionProfileRequest,
    ) -> Result<(), RusotoError<DeleteFieldLevelEncryptionProfileError>> {
        let request_uri = format!(
            "/2018-11-05/field-level-encryption-profile/{id}",
            id = input.id
        );

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteFieldLevelEncryptionProfileError::from_response(
                response,
            ));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Remove a public key you previously added to CloudFront.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_public_key(
        &self,
        input: DeletePublicKeyRequest,
    ) -> Result<(), RusotoError<DeletePublicKeyError>> {
        let request_uri = format!("/2018-11-05/public-key/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeletePublicKeyError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Delete a streaming distribution. To delete an RTMP distribution using the CloudFront API, perform the following steps.</p> <p> <b>To delete an RTMP distribution using the CloudFront API</b>:</p> <ol> <li> <p>Disable the RTMP distribution.</p> </li> <li> <p>Submit a <code>GET Streaming Distribution Config</code> request to get the current configuration and the <code>Etag</code> header for the distribution. </p> </li> <li> <p>Update the XML document that was returned in the response to your <code>GET Streaming Distribution Config</code> request to change the value of <code>Enabled</code> to <code>false</code>.</p> </li> <li> <p>Submit a <code>PUT Streaming Distribution Config</code> request to update the configuration for your distribution. In the request body, include the XML document that you updated in Step 3. Then set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Streaming Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to the <code>PUT Streaming Distribution Config</code> request to confirm that the distribution was successfully disabled.</p> </li> <li> <p>Submit a <code>GET Streaming Distribution Config</code> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> </li> <li> <p>Submit a <code>DELETE Streaming Distribution</code> request. Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Streaming Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to your <code>DELETE Streaming Distribution</code> request to confirm that the distribution was successfully deleted.</p> </li> </ol> <p>For information about deleting a distribution using the CloudFront console, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/HowToDeleteDistribution.html">Deleting a Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_streaming_distribution(
        &self,
        input: DeleteStreamingDistributionRequest,
    ) -> Result<(), RusotoError<DeleteStreamingDistributionError>> {
        let request_uri = format!("/2018-11-05/streaming-distribution/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteStreamingDistributionError::from_response(response));
        }

        Ok(std::mem::drop(response))
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
            "/2018-11-05/origin-access-identity/cloudfront/{id}",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetCloudFrontOriginAccessIdentityError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetCloudFrontOriginAccessIdentityResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetCloudFrontOriginAccessIdentityResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
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
            "/2018-11-05/origin-access-identity/cloudfront/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetCloudFrontOriginAccessIdentityConfigError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetCloudFrontOriginAccessIdentityConfigResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetCloudFrontOriginAccessIdentityConfigResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p>Get the information about a distribution. </p>
    #[allow(unused_variables, warnings)]
    async fn get_distribution(
        &self,
        input: GetDistributionRequest,
    ) -> Result<GetDistributionResult, RusotoError<GetDistributionError>> {
        let request_uri = format!("/2018-11-05/distribution/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetDistributionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetDistributionResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetDistributionResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p>Get the configuration information about a distribution. </p>
    #[allow(unused_variables, warnings)]
    async fn get_distribution_config(
        &self,
        input: GetDistributionConfigRequest,
    ) -> Result<GetDistributionConfigResult, RusotoError<GetDistributionConfigError>> {
        let request_uri = format!("/2018-11-05/distribution/{id}/config", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetDistributionConfigError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetDistributionConfigResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                GetDistributionConfigResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p>Get the field-level encryption configuration information.</p>
    #[allow(unused_variables, warnings)]
    async fn get_field_level_encryption(
        &self,
        input: GetFieldLevelEncryptionRequest,
    ) -> Result<GetFieldLevelEncryptionResult, RusotoError<GetFieldLevelEncryptionError>> {
        let request_uri = format!("/2018-11-05/field-level-encryption/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetFieldLevelEncryptionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetFieldLevelEncryptionResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetFieldLevelEncryptionResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
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
            "/2018-11-05/field-level-encryption/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetFieldLevelEncryptionConfigError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetFieldLevelEncryptionConfigResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetFieldLevelEncryptionConfigResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
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
            "/2018-11-05/field-level-encryption-profile/{id}",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetFieldLevelEncryptionProfileError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetFieldLevelEncryptionProfileResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetFieldLevelEncryptionProfileResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
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
            "/2018-11-05/field-level-encryption-profile/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetFieldLevelEncryptionProfileConfigError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetFieldLevelEncryptionProfileConfigResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetFieldLevelEncryptionProfileConfigResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p>Get the information about an invalidation. </p>
    #[allow(unused_variables, warnings)]
    async fn get_invalidation(
        &self,
        input: GetInvalidationRequest,
    ) -> Result<GetInvalidationResult, RusotoError<GetInvalidationError>> {
        let request_uri = format!(
            "/2018-11-05/distribution/{distribution_id}/invalidation/{id}",
            distribution_id = input.distribution_id,
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetInvalidationError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetInvalidationResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetInvalidationResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Get the public key information.</p>
    #[allow(unused_variables, warnings)]
    async fn get_public_key(
        &self,
        input: GetPublicKeyRequest,
    ) -> Result<GetPublicKeyResult, RusotoError<GetPublicKeyError>> {
        let request_uri = format!("/2018-11-05/public-key/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetPublicKeyError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetPublicKeyResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetPublicKeyResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p>Return public key configuration informaation</p>
    #[allow(unused_variables, warnings)]
    async fn get_public_key_config(
        &self,
        input: GetPublicKeyConfigRequest,
    ) -> Result<GetPublicKeyConfigResult, RusotoError<GetPublicKeyConfigError>> {
        let request_uri = format!("/2018-11-05/public-key/{id}/config", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetPublicKeyConfigError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetPublicKeyConfigResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                GetPublicKeyConfigResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about a specified RTMP distribution, including the distribution configuration.</p>
    #[allow(unused_variables, warnings)]
    async fn get_streaming_distribution(
        &self,
        input: GetStreamingDistributionRequest,
    ) -> Result<GetStreamingDistributionResult, RusotoError<GetStreamingDistributionError>> {
        let request_uri = format!("/2018-11-05/streaming-distribution/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetStreamingDistributionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetStreamingDistributionResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetStreamingDistributionResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
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
            "/2018-11-05/streaming-distribution/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetStreamingDistributionConfigError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetStreamingDistributionConfigResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetStreamingDistributionConfigResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
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
        let request_uri = "/2018-11-05/origin-access-identity/cloudfront";

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
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListCloudFrontOriginAccessIdentitiesError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListCloudFrontOriginAccessIdentitiesResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListCloudFrontOriginAccessIdentitiesResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>List distributions. </p>
    #[allow(unused_variables, warnings)]
    async fn list_distributions(
        &self,
        input: ListDistributionsRequest,
    ) -> Result<ListDistributionsResult, RusotoError<ListDistributionsError>> {
        let request_uri = "/2018-11-05/distribution";

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
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListDistributionsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListDistributionsResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                ListDistributionsResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
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
            "/2018-11-05/distributionsByWebACLId/{web_acl_id}",
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
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListDistributionsByWebACLIdError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListDistributionsByWebACLIdResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListDistributionsByWebACLIdResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
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
        let request_uri = "/2018-11-05/field-level-encryption";

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
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListFieldLevelEncryptionConfigsError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListFieldLevelEncryptionConfigsResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListFieldLevelEncryptionConfigsResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
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
        let request_uri = "/2018-11-05/field-level-encryption-profile";

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
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListFieldLevelEncryptionProfilesError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListFieldLevelEncryptionProfilesResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListFieldLevelEncryptionProfilesResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
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
            "/2018-11-05/distribution/{distribution_id}/invalidation",
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
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListInvalidationsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListInvalidationsResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                ListInvalidationsResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>List all public keys that have been added to CloudFront for this account.</p>
    #[allow(unused_variables, warnings)]
    async fn list_public_keys(
        &self,
        input: ListPublicKeysRequest,
    ) -> Result<ListPublicKeysResult, RusotoError<ListPublicKeysError>> {
        let request_uri = "/2018-11-05/public-key";

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
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListPublicKeysError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListPublicKeysResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListPublicKeysResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
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
        let request_uri = "/2018-11-05/streaming-distribution";

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
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListStreamingDistributionsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListStreamingDistributionsResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListStreamingDistributionsResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>List tags for a CloudFront resource.</p>
    #[allow(unused_variables, warnings)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResult, RusotoError<ListTagsForResourceError>> {
        let request_uri = "/2018-11-05/tagging";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("Resource", &input.resource);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListTagsForResourceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListTagsForResourceResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                ListTagsForResourceResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Add tags to a CloudFront resource.</p>
    #[allow(unused_variables, warnings)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = "/2018-11-05/tagging";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("Resource", &input.resource);
        params.put("Operation", "Tag");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        TagsSerializer::serialize(&mut writer, "Tags", &input.tags);
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(TagResourceError::from_response(response));
        }

        Ok(std::mem::drop(response))
    }

    /// <p>Remove tags from a CloudFront resource.</p>
    #[allow(unused_variables, warnings)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = "/2018-11-05/tagging";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("Resource", &input.resource);
        params.put("Operation", "Untag");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        TagKeysSerializer::serialize(&mut writer, "TagKeys", &input.tag_keys);
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UntagResourceError::from_response(response));
        }

        Ok(std::mem::drop(response))
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
            "/2018-11-05/origin-access-identity/cloudfront/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let mut writer = EventWriter::new(Vec::new());
        CloudFrontOriginAccessIdentityConfigSerializer::serialize(
            &mut writer,
            "CloudFrontOriginAccessIdentityConfig",
            &input.cloud_front_origin_access_identity_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateCloudFrontOriginAccessIdentityError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = UpdateCloudFrontOriginAccessIdentityResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = UpdateCloudFrontOriginAccessIdentityResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p><p>Updates the configuration for a web distribution. </p> <important> <p>When you update a distribution, there are more required fields than when you create a distribution. When you update your distribution by using this API action, follow the steps here to get the current configuration and then make your updates, to make sure that you include all of the required fields. To view a summary, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-overview-required-fields.html">Required Fields for Create Distribution and Update Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </important> <p>The update process includes getting the current distribution configuration, updating the XML document that is returned to make your changes, and then submitting an <code>UpdateDistribution</code> request to make the updates.</p> <p>For information about updating a distribution using the CloudFront console instead, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-creating-console.html">Creating a Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p> <b>To update a web distribution using the CloudFront API</b> </p> <ol> <li> <p>Submit a <a>GetDistributionConfig</a> request to get the current configuration and an <code>Etag</code> header for the distribution.</p> <note> <p>If you update the distribution again, you must get a new <code>Etag</code> header.</p> </note> </li> <li> <p>Update the XML document that was returned in the response to your <code>GetDistributionConfig</code> request to include your changes. </p> <important> <p>When you edit the XML file, be aware of the following:</p> <ul> <li> <p>You must strip out the ETag parameter that is returned.</p> </li> <li> <p>Additional fields are required when you update a distribution. There may be fields included in the XML file for features that you haven&#39;t configured for your distribution. This is expected and required to successfully update the distribution.</p> </li> <li> <p>You can&#39;t change the value of <code>CallerReference</code>. If you try to change this value, CloudFront returns an <code>IllegalUpdate</code> error. </p> </li> <li> <p>The new configuration replaces the existing configuration; the values that you specify in an <code>UpdateDistribution</code> request are not merged into your existing configuration. When you add, delete, or replace values in an element that allows multiple values (for example, <code>CNAME</code>), you must specify all of the values that you want to appear in the updated distribution. In addition, you must update the corresponding <code>Quantity</code> element.</p> </li> </ul> </important> </li> <li> <p>Submit an <code>UpdateDistribution</code> request to update the configuration for your distribution:</p> <ul> <li> <p>In the request body, include the XML document that you updated in Step 2. The request body must include an XML document with a <code>DistributionConfig</code> element.</p> </li> <li> <p>Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GetDistributionConfig</code> request in Step 1.</p> </li> </ul> </li> <li> <p>Review the response to the <code>UpdateDistribution</code> request to confirm that the configuration was successfully updated.</p> </li> <li> <p>Optional: Submit a <a>GetDistribution</a> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> </li> </ol></p>
    #[allow(unused_variables, warnings)]
    async fn update_distribution(
        &self,
        input: UpdateDistributionRequest,
    ) -> Result<UpdateDistributionResult, RusotoError<UpdateDistributionError>> {
        let request_uri = format!("/2018-11-05/distribution/{id}/config", id = input.id);

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let mut writer = EventWriter::new(Vec::new());
        DistributionConfigSerializer::serialize(
            &mut writer,
            "DistributionConfig",
            &input.distribution_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateDistributionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = UpdateDistributionResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                UpdateDistributionResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
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
            "/2018-11-05/field-level-encryption/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let mut writer = EventWriter::new(Vec::new());
        FieldLevelEncryptionConfigSerializer::serialize(
            &mut writer,
            "FieldLevelEncryptionConfig",
            &input.field_level_encryption_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateFieldLevelEncryptionConfigError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = UpdateFieldLevelEncryptionConfigResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = UpdateFieldLevelEncryptionConfigResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
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
            "/2018-11-05/field-level-encryption-profile/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let mut writer = EventWriter::new(Vec::new());
        FieldLevelEncryptionProfileConfigSerializer::serialize(
            &mut writer,
            "FieldLevelEncryptionProfileConfig",
            &input.field_level_encryption_profile_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateFieldLevelEncryptionProfileError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = UpdateFieldLevelEncryptionProfileResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = UpdateFieldLevelEncryptionProfileResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
        Ok(result)
    }

    /// <p>Update public key information. Note that the only value you can change is the comment.</p>
    #[allow(unused_variables, warnings)]
    async fn update_public_key(
        &self,
        input: UpdatePublicKeyRequest,
    ) -> Result<UpdatePublicKeyResult, RusotoError<UpdatePublicKeyError>> {
        let request_uri = format!("/2018-11-05/public-key/{id}/config", id = input.id);

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let mut writer = EventWriter::new(Vec::new());
        PublicKeyConfigSerializer::serialize(
            &mut writer,
            "PublicKeyConfig",
            &input.public_key_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdatePublicKeyError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = UpdatePublicKeyResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = UpdatePublicKeyResultDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
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
            "/2018-11-05/streaming-distribution/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("PUT", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let mut writer = EventWriter::new(Vec::new());
        StreamingDistributionConfigSerializer::serialize(
            &mut writer,
            "StreamingDistributionConfig",
            &input.streaming_distribution_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateStreamingDistributionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = UpdateStreamingDistributionResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = UpdateStreamingDistributionResultDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        if let Some(e_tag) = response.headers.get("ETag") {
            let value = e_tag.to_owned();
            result.e_tag = Some(value)
        }; // parse non-payload
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
