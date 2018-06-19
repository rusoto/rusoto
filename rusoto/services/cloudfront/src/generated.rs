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
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use rusoto_core::xmlerror::*;
use rusoto_core::xmlutil::{
    characters, end_element, find_start_element, peek_at_name, skip_tree, start_element,
};
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use std::io::Write;
use std::str::FromStr;
use xml;
use xml::reader::ParserConfig;
use xml::reader::XmlEvent;
use xml::EventReader;
use xml::EventWriter;
enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
/// <p>A complex type that lists the AWS accounts, if any, that you included in the <code>TrustedSigners</code> complex type for this distribution. These are the accounts that you want to allow to create signed URLs for private content.</p> <p>The <code>Signer</code> complex type lists the AWS account number of the trusted signer or <code>self</code> if the signer is the AWS account that created the distribution. The <code>Signer</code> element also includes the IDs of any active CloudFront key pairs that are associated with the trusted signer's AWS account. If no <code>KeyPairId</code> element appears for a <code>Signer</code>, that signer can't create signed URLs. </p> <p>For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ActiveTrustedSigners, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ActiveTrustedSigners::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Enabled" => {
                        obj.enabled = try!(BooleanDeserializer::deserialize("Enabled", stack));
                    }
                    "Items" => {
                        obj.items = Some(try!(SignerListDeserializer::deserialize("Items", stack)));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct AliasListDeserializer;
impl AliasListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "CNAME" {
                        obj.push(try!(StringDeserializer::deserialize("CNAME", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
pub struct Aliases {
    /// <p>A complex type that contains the CNAME aliases, if any, that you want to associate with this distribution.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of CNAME aliases, if any, that you want to associate with this distribution.</p>
    pub quantity: i64,
}

struct AliasesDeserializer;
impl AliasesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Aliases, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Aliases::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items = Some(try!(AliasListDeserializer::deserialize("Items", stack)));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AllowedMethods, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AllowedMethods::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CachedMethods" => {
                        obj.cached_methods = Some(try!(CachedMethodsDeserializer::deserialize(
                            "CachedMethods",
                            stack
                        )));
                    }
                    "Items" => {
                        obj.items = try!(MethodsListDeserializer::deserialize("Items", stack));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "AwsAccountNumber" {
                        obj.push(try!(StringDeserializer::deserialize(
                            "AwsAccountNumber",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

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
pub struct CacheBehavior {
    pub allowed_methods: Option<AllowedMethods>,
    /// <p>Whether you want CloudFront to automatically compress certain files for this cache behavior. If so, specify true; if not, specify false. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/ServingCompressedFiles.html">Serving Compressed Files</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub compress: Option<bool>,
    /// <p>The default amount of time that you want objects to stay in CloudFront caches before CloudFront forwards another request to your origin to determine whether the object has been updated. The value that you specify applies only when your origin does not add HTTP headers such as <code>Cache-Control max-age</code>, <code>Cache-Control s-maxage</code>, and <code>Expires</code> to objects. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Specifying How Long Objects and Errors Stay in a CloudFront Edge Cache (Expiration)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub default_ttl: Option<i64>,
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
    /// <p>The value of <code>ID</code> for the origin that you want CloudFront to route requests to when a request matches the path pattern either for a cache behavior or for the default cache behavior.</p>
    pub target_origin_id: String,
    /// <p>A complex type that specifies the AWS accounts, if any, that you want to allow to create signed URLs for private content.</p> <p>If you want to require signed URLs in requests for objects in the target origin that match the <code>PathPattern</code> for this cache behavior, specify <code>true</code> for <code>Enabled</code>, and specify the applicable values for <code>Quantity</code> and <code>Items</code>. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon Amazon CloudFront Developer Guide</i>.</p> <p>If you don't want to require signed URLs in requests for objects that match <code>PathPattern</code>, specify <code>false</code> for <code>Enabled</code> and <code>0</code> for <code>Quantity</code>. Omit <code>Items</code>.</p> <p>To add, change, or remove one or more trusted signers, change <code>Enabled</code> to <code>true</code> (if it's currently <code>false</code>), change <code>Quantity</code> as applicable, and specify all of the trusted signers that you want to include in the updated distribution.</p>
    pub trusted_signers: TrustedSigners,
    /// <p><p>The protocol that viewers can use to access the files in the origin specified by <code>TargetOriginId</code> when a request matches the path pattern in <code>PathPattern</code>. You can specify the following options:</p> <ul> <li> <p> <code>allow-all</code>: Viewers can use HTTP or HTTPS.</p> </li> <li> <p> <code>redirect-to-https</code>: If a viewer submits an HTTP request, CloudFront returns an HTTP status code of 301 (Moved Permanently) to the viewer along with the HTTPS URL. The viewer then resubmits the request using the new URL. </p> </li> <li> <p> <code>https-only</code>: If a viewer sends an HTTP request, CloudFront returns an HTTP status code of 403 (Forbidden). </p> </li> </ul> <p>For more information about requiring the HTTPS protocol, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/SecureConnections.html">Using an HTTPS Connection to Access Your Objects</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <note> <p>The only way to guarantee that viewers retrieve an object that was fetched from the origin using HTTPS is never to use any other protocol to fetch the object. If you have recently changed from HTTP to HTTPS, we recommend that you clear your objects&#39; cache because cached objects are protocol agnostic. That means that an edge location will return an object from the cache regardless of whether the current request protocol matches the protocol used previously. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Specifying How Long Objects and Errors Stay in a CloudFront Edge Cache (Expiration)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </note></p>
    pub viewer_protocol_policy: String,
}

struct CacheBehaviorDeserializer;
impl CacheBehaviorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheBehavior, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheBehavior::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AllowedMethods" => {
                        obj.allowed_methods = Some(try!(AllowedMethodsDeserializer::deserialize(
                            "AllowedMethods",
                            stack
                        )));
                    }
                    "Compress" => {
                        obj.compress =
                            Some(try!(BooleanDeserializer::deserialize("Compress", stack)));
                    }
                    "DefaultTTL" => {
                        obj.default_ttl =
                            Some(try!(LongDeserializer::deserialize("DefaultTTL", stack)));
                    }
                    "ForwardedValues" => {
                        obj.forwarded_values = try!(ForwardedValuesDeserializer::deserialize(
                            "ForwardedValues",
                            stack
                        ));
                    }
                    "LambdaFunctionAssociations" => {
                        obj.lambda_function_associations =
                            Some(try!(LambdaFunctionAssociationsDeserializer::deserialize(
                                "LambdaFunctionAssociations",
                                stack
                            )));
                    }
                    "MaxTTL" => {
                        obj.max_ttl = Some(try!(LongDeserializer::deserialize("MaxTTL", stack)));
                    }
                    "MinTTL" => {
                        obj.min_ttl = try!(LongDeserializer::deserialize("MinTTL", stack));
                    }
                    "PathPattern" => {
                        obj.path_pattern =
                            try!(StringDeserializer::deserialize("PathPattern", stack));
                    }
                    "SmoothStreaming" => {
                        obj.smooth_streaming = Some(try!(BooleanDeserializer::deserialize(
                            "SmoothStreaming",
                            stack
                        )));
                    }
                    "TargetOriginId" => {
                        obj.target_origin_id =
                            try!(StringDeserializer::deserialize("TargetOriginId", stack));
                    }
                    "TrustedSigners" => {
                        obj.trusted_signers = try!(TrustedSignersDeserializer::deserialize(
                            "TrustedSigners",
                            stack
                        ));
                    }
                    "ViewerProtocolPolicy" => {
                        obj.viewer_protocol_policy =
                            try!(ViewerProtocolPolicyDeserializer::deserialize(
                                "ViewerProtocolPolicy",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CacheBehavior>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "CacheBehavior" {
                        obj.push(try!(CacheBehaviorDeserializer::deserialize(
                            "CacheBehavior",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
pub struct CacheBehaviors {
    /// <p>Optional: A complex type that contains cache behaviors for this distribution. If <code>Quantity</code> is <code>0</code>, you can omit <code>Items</code>.</p>
    pub items: Option<Vec<CacheBehavior>>,
    /// <p>The number of cache behaviors for this distribution. </p>
    pub quantity: i64,
}

struct CacheBehaviorsDeserializer;
impl CacheBehaviorsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheBehaviors, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheBehaviors::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items = Some(try!(CacheBehaviorListDeserializer::deserialize(
                            "Items", stack
                        )));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
pub struct CachedMethods {
    /// <p>A complex type that contains the HTTP methods that you want CloudFront to cache responses to.</p>
    pub items: Vec<String>,
    /// <p>The number of HTTP methods for which you want CloudFront to cache responses. Valid values are <code>2</code> (for caching responses to <code>GET</code> and <code>HEAD</code> requests) and <code>3</code> (for caching responses to <code>GET</code>, <code>HEAD</code>, and <code>OPTIONS</code> requests).</p>
    pub quantity: i64,
}

struct CachedMethodsDeserializer;
impl CachedMethodsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CachedMethods, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CachedMethods::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items = try!(MethodsListDeserializer::deserialize("Items", stack));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudFrontOriginAccessIdentity, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CloudFrontOriginAccessIdentity::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CloudFrontOriginAccessIdentityConfig" => {
                        obj.cloud_front_origin_access_identity_config = Some(try!(
                            CloudFrontOriginAccessIdentityConfigDeserializer::deserialize(
                                "CloudFrontOriginAccessIdentityConfig",
                                stack
                            )
                        ));
                    }
                    "Id" => {
                        obj.id = try!(StringDeserializer::deserialize("Id", stack));
                    }
                    "S3CanonicalUserId" => {
                        obj.s3_canonical_user_id =
                            try!(StringDeserializer::deserialize("S3CanonicalUserId", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Origin access identity configuration. Send a <code>GET</code> request to the <code>/<i>CloudFront API version</i>/CloudFront/identity ID/config</code> resource. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CloudFrontOriginAccessIdentityConfig {
    /// <p>A unique number that ensures the request can't be replayed.</p> <p>If the <code>CallerReference</code> is new (no matter the content of the <code>CloudFrontOriginAccessIdentityConfig</code> object), a new origin access identity is created.</p> <p>If the <code>CallerReference</code> is a value already sent in a previous identity request, and the content of the <code>CloudFrontOriginAccessIdentityConfig</code> is identical to the original request (ignoring white space), the response includes the same information returned to the original request. </p> <p>If the <code>CallerReference</code> is a value you already sent in a previous request to create an identity, but the content of the <code>CloudFrontOriginAccessIdentityConfig</code> is different from the original request, CloudFront returns a <code>CloudFrontOriginAccessIdentityAlreadyExists</code> error. </p>
    pub caller_reference: String,
    /// <p>Any comments you want to include about the origin access identity. </p>
    pub comment: String,
}

struct CloudFrontOriginAccessIdentityConfigDeserializer;
impl CloudFrontOriginAccessIdentityConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudFrontOriginAccessIdentityConfig, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CloudFrontOriginAccessIdentityConfig::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CallerReference" => {
                        obj.caller_reference =
                            try!(StringDeserializer::deserialize("CallerReference", stack));
                    }
                    "Comment" => {
                        obj.comment = try!(StringDeserializer::deserialize("Comment", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudFrontOriginAccessIdentityList, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CloudFrontOriginAccessIdentityList::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "IsTruncated" => {
                        obj.is_truncated =
                            try!(BooleanDeserializer::deserialize("IsTruncated", stack));
                    }
                    "Items" => {
                        obj.items = Some(try!(
                            CloudFrontOriginAccessIdentitySummaryListDeserializer::deserialize(
                                "Items", stack
                            )
                        ));
                    }
                    "Marker" => {
                        obj.marker = try!(StringDeserializer::deserialize("Marker", stack));
                    }
                    "MaxItems" => {
                        obj.max_items = try!(IntegerDeserializer::deserialize("MaxItems", stack));
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(try!(StringDeserializer::deserialize("NextMarker", stack)));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Summary of the information about a CloudFront origin access identity.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudFrontOriginAccessIdentitySummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CloudFrontOriginAccessIdentitySummary::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Comment" => {
                        obj.comment = try!(StringDeserializer::deserialize("Comment", stack));
                    }
                    "Id" => {
                        obj.id = try!(StringDeserializer::deserialize("Id", stack));
                    }
                    "S3CanonicalUserId" => {
                        obj.s3_canonical_user_id =
                            try!(StringDeserializer::deserialize("S3CanonicalUserId", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct CloudFrontOriginAccessIdentitySummaryListDeserializer;
impl CloudFrontOriginAccessIdentitySummaryListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CloudFrontOriginAccessIdentitySummary>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "CloudFrontOriginAccessIdentitySummary" {
                        obj.push(try!(
                            CloudFrontOriginAccessIdentitySummaryDeserializer::deserialize(
                                "CloudFrontOriginAccessIdentitySummary",
                                stack
                            )
                        ));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct CookieNameListDeserializer;
impl CookieNameListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Name" {
                        obj.push(try!(StringDeserializer::deserialize("Name", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
pub struct CookieNames {
    /// <p>A complex type that contains one <code>Name</code> element for each cookie that you want CloudFront to forward to the origin for this cache behavior.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of different cookies that you want CloudFront to forward to the origin for this cache behavior.</p>
    pub quantity: i64,
}

struct CookieNamesDeserializer;
impl CookieNamesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CookieNames, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CookieNames::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items = Some(try!(CookieNameListDeserializer::deserialize(
                            "Items", stack
                        )));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
pub struct CookiePreference {
    /// <p>Specifies which cookies to forward to the origin for this cache behavior: all, none, or the list of cookies specified in the <code>WhitelistedNames</code> complex type.</p> <p>Amazon S3 doesn't process cookies. When the cache behavior is forwarding requests to an Amazon S3 origin, specify none for the <code>Forward</code> element. </p>
    pub forward: String,
    /// <p>Required if you specify <code>whitelist</code> for the value of <code>Forward:</code>. A complex type that specifies how many different cookies you want CloudFront to forward to the origin for this cache behavior and, if you want to forward selected cookies, the names of those cookies.</p> <p>If you specify <code>all</code> or none for the value of <code>Forward</code>, omit <code>WhitelistedNames</code>. If you change the value of <code>Forward</code> from <code>whitelist</code> to all or none and you don't delete the <code>WhitelistedNames</code> element and its child elements, CloudFront deletes them automatically.</p> <p>For the current limit on the number of cookie names that you can whitelist for each cache behavior, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_cloudfront">Amazon CloudFront Limits</a> in the <i>AWS General Reference</i>.</p>
    pub whitelisted_names: Option<CookieNames>,
}

struct CookiePreferenceDeserializer;
impl CookiePreferenceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CookiePreference, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CookiePreference::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Forward" => {
                        obj.forward =
                            try!(ItemSelectionDeserializer::deserialize("Forward", stack));
                    }
                    "WhitelistedNames" => {
                        obj.whitelisted_names = Some(try!(CookieNamesDeserializer::deserialize(
                            "WhitelistedNames",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
pub struct CreateCloudFrontOriginAccessIdentityRequest {
    /// <p>The current configuration information for the identity.</p>
    pub cloud_front_origin_access_identity_config: CloudFrontOriginAccessIdentityConfig,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateCloudFrontOriginAccessIdentityResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateCloudFrontOriginAccessIdentityResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CloudFrontOriginAccessIdentity" => {
                        obj.cloud_front_origin_access_identity = Some(try!(
                            CloudFrontOriginAccessIdentityDeserializer::deserialize(
                                "CloudFrontOriginAccessIdentity",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to create a new distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDistributionRequest {
    /// <p>The distribution's configuration information.</p>
    pub distribution_config: DistributionConfig,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDistributionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateDistributionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Distribution" => {
                        obj.distribution = Some(try!(DistributionDeserializer::deserialize(
                            "Distribution",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to create a new distribution with tags. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDistributionWithTagsRequest {
    /// <p>The distribution's configuration information. </p>
    pub distribution_config_with_tags: DistributionConfigWithTags,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateDistributionWithTagsResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateDistributionWithTagsResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Distribution" => {
                        obj.distribution = Some(try!(DistributionDeserializer::deserialize(
                            "Distribution",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to create an invalidation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateInvalidationRequest {
    /// <p>The distribution's id.</p>
    pub distribution_id: String,
    /// <p>The batch information for the invalidation.</p>
    pub invalidation_batch: InvalidationBatch,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateInvalidationResult {
    /// <p>The invalidation's information.</p>
    pub invalidation: Option<Invalidation>,
    /// <p>The fully qualified URI of the distribution and invalidation batch request, including the <code>Invalidation ID</code>.</p>
    pub location: Option<String>,
}

struct CreateInvalidationResultDeserializer;
impl CreateInvalidationResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateInvalidationResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateInvalidationResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Invalidation" => {
                        obj.invalidation = Some(try!(InvalidationDeserializer::deserialize(
                            "Invalidation",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to create a new streaming distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStreamingDistributionRequest {
    /// <p>The streaming distribution's configuration information.</p>
    pub streaming_distribution_config: StreamingDistributionConfig,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateStreamingDistributionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateStreamingDistributionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "StreamingDistribution" => {
                        obj.streaming_distribution =
                            Some(try!(StreamingDistributionDeserializer::deserialize(
                                "StreamingDistribution",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to create a new streaming distribution with tags.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStreamingDistributionWithTagsRequest {
    /// <p> The streaming distribution's configuration information. </p>
    pub streaming_distribution_config_with_tags: StreamingDistributionConfigWithTags,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStreamingDistributionWithTagsResult {
    pub e_tag: Option<String>,
    /// <p>The fully qualified URI of the new streaming distribution resource just created. For example:<code> https://cloudfront.amazonaws.com/2010-11-01/streaming-distribution/EGTXBD79H29TRA8</code>.</p>
    pub location: Option<String>,
    /// <p>The streaming distribution's information. </p>
    pub streaming_distribution: Option<StreamingDistribution>,
}

struct CreateStreamingDistributionWithTagsResultDeserializer;
impl CreateStreamingDistributionWithTagsResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateStreamingDistributionWithTagsResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateStreamingDistributionWithTagsResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "StreamingDistribution" => {
                        obj.streaming_distribution =
                            Some(try!(StreamingDistributionDeserializer::deserialize(
                                "StreamingDistribution",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A complex type that controls:</p> <ul> <li> <p>Whether CloudFront replaces HTTP status codes in the 4xx and 5xx range with custom error messages before returning the response to the viewer. </p> </li> <li> <p>How long CloudFront caches HTTP status codes in the 4xx and 5xx range.</p> </li> </ul> <p>For more information about custom error pages, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/custom-error-pages.html">Customizing Error Responses</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CustomErrorResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CustomErrorResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ErrorCachingMinTTL" => {
                        obj.error_caching_min_ttl = Some(try!(LongDeserializer::deserialize(
                            "ErrorCachingMinTTL",
                            stack
                        )));
                    }
                    "ErrorCode" => {
                        obj.error_code = try!(IntegerDeserializer::deserialize("ErrorCode", stack));
                    }
                    "ResponseCode" => {
                        obj.response_code =
                            Some(try!(StringDeserializer::deserialize("ResponseCode", stack)));
                    }
                    "ResponsePagePath" => {
                        obj.response_page_path = Some(try!(StringDeserializer::deserialize(
                            "ResponsePagePath",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CustomErrorResponse>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "CustomErrorResponse" {
                        obj.push(try!(CustomErrorResponseDeserializer::deserialize(
                            "CustomErrorResponse",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
pub struct CustomErrorResponses {
    /// <p>A complex type that contains a <code>CustomErrorResponse</code> element for each HTTP status code for which you want to specify a custom error page and/or a caching duration. </p>
    pub items: Option<Vec<CustomErrorResponse>>,
    /// <p>The number of HTTP status codes for which you want to specify a custom error page and/or a caching duration. If <code>Quantity</code> is <code>0</code>, you can omit <code>Items</code>.</p>
    pub quantity: i64,
}

struct CustomErrorResponsesDeserializer;
impl CustomErrorResponsesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CustomErrorResponses, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CustomErrorResponses::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items = Some(try!(CustomErrorResponseListDeserializer::deserialize(
                            "Items", stack
                        )));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
pub struct CustomHeaders {
    /// <p> <b>Optional</b>: A list that contains one <code>OriginCustomHeader</code> element for each custom header that you want CloudFront to forward to the origin. If Quantity is <code>0</code>, omit <code>Items</code>.</p>
    pub items: Option<Vec<OriginCustomHeader>>,
    /// <p>The number of custom headers, if any, for this distribution.</p>
    pub quantity: i64,
}

struct CustomHeadersDeserializer;
impl CustomHeadersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CustomHeaders, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CustomHeaders::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items = Some(try!(OriginCustomHeadersListDeserializer::deserialize(
                            "Items", stack
                        )));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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

/// <p>A customer origin.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CustomOriginConfig, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CustomOriginConfig::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "HTTPPort" => {
                        obj.http_port = try!(IntegerDeserializer::deserialize("HTTPPort", stack));
                    }
                    "HTTPSPort" => {
                        obj.https_port = try!(IntegerDeserializer::deserialize("HTTPSPort", stack));
                    }
                    "OriginKeepaliveTimeout" => {
                        obj.origin_keepalive_timeout = Some(try!(
                            IntegerDeserializer::deserialize("OriginKeepaliveTimeout", stack)
                        ));
                    }
                    "OriginProtocolPolicy" => {
                        obj.origin_protocol_policy =
                            try!(OriginProtocolPolicyDeserializer::deserialize(
                                "OriginProtocolPolicy",
                                stack
                            ));
                    }
                    "OriginReadTimeout" => {
                        obj.origin_read_timeout = Some(try!(IntegerDeserializer::deserialize(
                            "OriginReadTimeout",
                            stack
                        )));
                    }
                    "OriginSslProtocols" => {
                        obj.origin_ssl_protocols =
                            Some(try!(OriginSslProtocolsDeserializer::deserialize(
                                "OriginSslProtocols",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
pub struct DefaultCacheBehavior {
    pub allowed_methods: Option<AllowedMethods>,
    /// <p>Whether you want CloudFront to automatically compress certain files for this cache behavior. If so, specify <code>true</code>; if not, specify <code>false</code>. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/ServingCompressedFiles.html">Serving Compressed Files</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub compress: Option<bool>,
    /// <p>The default amount of time that you want objects to stay in CloudFront caches before CloudFront forwards another request to your origin to determine whether the object has been updated. The value that you specify applies only when your origin does not add HTTP headers such as <code>Cache-Control max-age</code>, <code>Cache-Control s-maxage</code>, and <code>Expires</code> to objects. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Specifying How Long Objects and Errors Stay in a CloudFront Edge Cache (Expiration)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub default_ttl: Option<i64>,
    /// <p>A complex type that specifies how CloudFront handles query strings and cookies.</p>
    pub forwarded_values: ForwardedValues,
    /// <p>A complex type that contains zero or more Lambda function associations for a cache behavior.</p>
    pub lambda_function_associations: Option<LambdaFunctionAssociations>,
    pub max_ttl: Option<i64>,
    /// <p>The minimum amount of time that you want objects to stay in CloudFront caches before CloudFront forwards another request to your origin to determine whether the object has been updated. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Specifying How Long Objects and Errors Stay in a CloudFront Edge Cache (Expiration)</a> in the <i>Amazon Amazon CloudFront Developer Guide</i>.</p> <p>You must specify <code>0</code> for <code>MinTTL</code> if you configure CloudFront to forward all headers to your origin (under <code>Headers</code>, if you specify <code>1</code> for <code>Quantity</code> and <code>*</code> for <code>Name</code>).</p>
    pub min_ttl: i64,
    /// <p>Indicates whether you want to distribute media files in the Microsoft Smooth Streaming format using the origin that is associated with this cache behavior. If so, specify <code>true</code>; if not, specify <code>false</code>. If you specify <code>true</code> for <code>SmoothStreaming</code>, you can still distribute other content using this cache behavior if the content matches the value of <code>PathPattern</code>. </p>
    pub smooth_streaming: Option<bool>,
    /// <p>The value of <code>ID</code> for the origin that you want CloudFront to route requests to when a request matches the path pattern either for a cache behavior or for the default cache behavior.</p>
    pub target_origin_id: String,
    /// <p>A complex type that specifies the AWS accounts, if any, that you want to allow to create signed URLs for private content.</p> <p>If you want to require signed URLs in requests for objects in the target origin that match the <code>PathPattern</code> for this cache behavior, specify <code>true</code> for <code>Enabled</code>, and specify the applicable values for <code>Quantity</code> and <code>Items</code>. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon Amazon CloudFront Developer Guide</i>.</p> <p>If you don't want to require signed URLs in requests for objects that match <code>PathPattern</code>, specify <code>false</code> for <code>Enabled</code> and <code>0</code> for <code>Quantity</code>. Omit <code>Items</code>.</p> <p>To add, change, or remove one or more trusted signers, change <code>Enabled</code> to <code>true</code> (if it's currently <code>false</code>), change <code>Quantity</code> as applicable, and specify all of the trusted signers that you want to include in the updated distribution.</p>
    pub trusted_signers: TrustedSigners,
    /// <p><p>The protocol that viewers can use to access the files in the origin specified by <code>TargetOriginId</code> when a request matches the path pattern in <code>PathPattern</code>. You can specify the following options:</p> <ul> <li> <p> <code>allow-all</code>: Viewers can use HTTP or HTTPS.</p> </li> <li> <p> <code>redirect-to-https</code>: If a viewer submits an HTTP request, CloudFront returns an HTTP status code of 301 (Moved Permanently) to the viewer along with the HTTPS URL. The viewer then resubmits the request using the new URL.</p> </li> <li> <p> <code>https-only</code>: If a viewer sends an HTTP request, CloudFront returns an HTTP status code of 403 (Forbidden).</p> </li> </ul> <p>For more information about requiring the HTTPS protocol, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/SecureConnections.html">Using an HTTPS Connection to Access Your Objects</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <note> <p>The only way to guarantee that viewers retrieve an object that was fetched from the origin using HTTPS is never to use any other protocol to fetch the object. If you have recently changed from HTTP to HTTPS, we recommend that you clear your objects&#39; cache because cached objects are protocol agnostic. That means that an edge location will return an object from the cache regardless of whether the current request protocol matches the protocol used previously. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Expiration.html">Specifying How Long Objects and Errors Stay in a CloudFront Edge Cache (Expiration)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </note></p>
    pub viewer_protocol_policy: String,
}

struct DefaultCacheBehaviorDeserializer;
impl DefaultCacheBehaviorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DefaultCacheBehavior, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DefaultCacheBehavior::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AllowedMethods" => {
                        obj.allowed_methods = Some(try!(AllowedMethodsDeserializer::deserialize(
                            "AllowedMethods",
                            stack
                        )));
                    }
                    "Compress" => {
                        obj.compress =
                            Some(try!(BooleanDeserializer::deserialize("Compress", stack)));
                    }
                    "DefaultTTL" => {
                        obj.default_ttl =
                            Some(try!(LongDeserializer::deserialize("DefaultTTL", stack)));
                    }
                    "ForwardedValues" => {
                        obj.forwarded_values = try!(ForwardedValuesDeserializer::deserialize(
                            "ForwardedValues",
                            stack
                        ));
                    }
                    "LambdaFunctionAssociations" => {
                        obj.lambda_function_associations =
                            Some(try!(LambdaFunctionAssociationsDeserializer::deserialize(
                                "LambdaFunctionAssociations",
                                stack
                            )));
                    }
                    "MaxTTL" => {
                        obj.max_ttl = Some(try!(LongDeserializer::deserialize("MaxTTL", stack)));
                    }
                    "MinTTL" => {
                        obj.min_ttl = try!(LongDeserializer::deserialize("MinTTL", stack));
                    }
                    "SmoothStreaming" => {
                        obj.smooth_streaming = Some(try!(BooleanDeserializer::deserialize(
                            "SmoothStreaming",
                            stack
                        )));
                    }
                    "TargetOriginId" => {
                        obj.target_origin_id =
                            try!(StringDeserializer::deserialize("TargetOriginId", stack));
                    }
                    "TrustedSigners" => {
                        obj.trusted_signers = try!(TrustedSignersDeserializer::deserialize(
                            "TrustedSigners",
                            stack
                        ));
                    }
                    "ViewerProtocolPolicy" => {
                        obj.viewer_protocol_policy =
                            try!(ViewerProtocolPolicyDeserializer::deserialize(
                                "ViewerProtocolPolicy",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
pub struct DeleteCloudFrontOriginAccessIdentityRequest {
    /// <p>The origin access identity's ID.</p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header you received from a previous <code>GET</code> or <code>PUT</code> request. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

/// <p>This action deletes a web distribution. To delete a web distribution using the CloudFront API, perform the following steps.</p> <p> <b>To delete a web distribution using the CloudFront API:</b> </p> <ol> <li> <p>Disable the web distribution </p> </li> <li> <p>Submit a <code>GET Distribution Config</code> request to get the current configuration and the <code>Etag</code> header for the distribution.</p> </li> <li> <p>Update the XML document that was returned in the response to your <code>GET Distribution Config</code> request to change the value of <code>Enabled</code> to <code>false</code>.</p> </li> <li> <p>Submit a <code>PUT Distribution Config</code> request to update the configuration for your distribution. In the request body, include the XML document that you updated in Step 3. Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to the <code>PUT Distribution Config</code> request to confirm that the distribution was successfully disabled.</p> </li> <li> <p>Submit a <code>GET Distribution</code> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> </li> <li> <p>Submit a <code>DELETE Distribution</code> request. Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Distribution Config</code> request in Step 6.</p> </li> <li> <p>Review the response to your <code>DELETE Distribution</code> request to confirm that the distribution was successfully deleted.</p> </li> </ol> <p>For information about deleting a distribution using the CloudFront console, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/HowToDeleteDistribution.html">Deleting a Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDistributionRequest {
    /// <p>The distribution ID. </p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when you disabled the distribution. For example: <code>E2QWRUHAPOMQZL</code>. </p>
    pub if_match: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteServiceLinkedRoleRequest {
    pub role_name: String,
}

/// <p>The request to delete a streaming distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteStreamingDistributionRequest {
    /// <p>The distribution ID. </p>
    pub id: String,
    /// <p>The value of the <code>ETag</code> header that you received when you disabled the streaming distribution. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub if_match: Option<String>,
}

/// <p>The distribution's information.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Distribution, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Distribution::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ARN" => {
                        obj.arn = try!(StringDeserializer::deserialize("ARN", stack));
                    }
                    "ActiveTrustedSigners" => {
                        obj.active_trusted_signers =
                            try!(ActiveTrustedSignersDeserializer::deserialize(
                                "ActiveTrustedSigners",
                                stack
                            ));
                    }
                    "DistributionConfig" => {
                        obj.distribution_config =
                            try!(DistributionConfigDeserializer::deserialize(
                                "DistributionConfig",
                                stack
                            ));
                    }
                    "DomainName" => {
                        obj.domain_name =
                            try!(StringDeserializer::deserialize("DomainName", stack));
                    }
                    "Id" => {
                        obj.id = try!(StringDeserializer::deserialize("Id", stack));
                    }
                    "InProgressInvalidationBatches" => {
                        obj.in_progress_invalidation_batches =
                            try!(IntegerDeserializer::deserialize(
                                "InProgressInvalidationBatches",
                                stack
                            ));
                    }
                    "LastModifiedTime" => {
                        obj.last_modified_time = try!(TimestampDeserializer::deserialize(
                            "LastModifiedTime",
                            stack
                        ));
                    }
                    "Status" => {
                        obj.status = try!(StringDeserializer::deserialize("Status", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A distribution configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DistributionConfig {
    /// <p>A complex type that contains information about CNAMEs (alternate domain names), if any, for this distribution.</p>
    pub aliases: Option<Aliases>,
    /// <p>A complex type that contains zero or more <code>CacheBehavior</code> elements. </p>
    pub cache_behaviors: Option<CacheBehaviors>,
    /// <p>A unique value (for example, a date-time stamp) that ensures that the request can't be replayed.</p> <p>If the value of <code>CallerReference</code> is new (regardless of the content of the <code>DistributionConfig</code> object), CloudFront creates a new distribution.</p> <p>If <code>CallerReference</code> is a value you already sent in a previous request to create a distribution, and if the content of the <code>DistributionConfig</code> is identical to the original request (ignoring white space), CloudFront returns the same the response that it returned to the original request.</p> <p>If <code>CallerReference</code> is a value you already sent in a previous request to create a distribution but the content of the <code>DistributionConfig</code> is different from the original request, CloudFront returns a <code>DistributionAlreadyExists</code> error.</p>
    pub caller_reference: String,
    /// <p>Any comments you want to include about the distribution.</p> <p>If you don't want to specify a comment, include an empty <code>Comment</code> element.</p> <p>To delete an existing comment, update the distribution configuration and include an empty <code>Comment</code> element.</p> <p>To add or change a comment, update the distribution configuration and specify the new comment.</p>
    pub comment: String,
    /// <p>A complex type that controls the following:</p> <ul> <li> <p>Whether CloudFront replaces HTTP status codes in the 4xx and 5xx range with custom error messages before returning the response to the viewer.</p> </li> <li> <p>How long CloudFront caches HTTP status codes in the 4xx and 5xx range.</p> </li> </ul> <p>For more information about custom error pages, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/custom-error-pages.html">Customizing Error Responses</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub custom_error_responses: Option<CustomErrorResponses>,
    /// <p>A complex type that describes the default cache behavior if you don't specify a <code>CacheBehavior</code> element or if files don't match any of the values of <code>PathPattern</code> in <code>CacheBehavior</code> elements. You must create exactly one default cache behavior.</p>
    pub default_cache_behavior: DefaultCacheBehavior,
    /// <p>The object that you want CloudFront to request from your origin (for example, <code>index.html</code>) when a viewer requests the root URL for your distribution (<code>http://www.example.com</code>) instead of an object in your distribution (<code>http://www.example.com/product-description.html</code>). Specifying a default root object avoids exposing the contents of your distribution.</p> <p>Specify only the object name, for example, <code>index.html</code>. Don't add a <code>/</code> before the object name.</p> <p>If you don't want to specify a default root object when you create a distribution, include an empty <code>DefaultRootObject</code> element.</p> <p>To delete the default root object from an existing distribution, update the distribution configuration and include an empty <code>DefaultRootObject</code> element.</p> <p>To replace the default root object, update the distribution configuration and specify the new object.</p> <p>For more information about the default root object, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/DefaultRootObject.html">Creating a Default Root Object</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub default_root_object: Option<String>,
    /// <p>From this field, you can enable or disable the selected distribution.</p> <p>If you specify <code>false</code> for <code>Enabled</code> but you specify values for <code>Bucket</code> and <code>Prefix</code>, the values are automatically deleted.</p>
    pub enabled: bool,
    /// <p>(Optional) Specify the maximum HTTP version that you want viewers to use to communicate with CloudFront. The default value for new web distributions is http2. Viewers that don't support HTTP/2 automatically use an earlier HTTP version.</p> <p>For viewers and CloudFront to use HTTP/2, viewers must support TLS 1.2 or later, and must support Server Name Identification (SNI).</p> <p>In general, configuring CloudFront to communicate with viewers using HTTP/2 reduces latency. You can improve performance by optimizing for HTTP/2. For more information, do an Internet search for "http/2 optimization." </p>
    pub http_version: Option<String>,
    /// <p>If you want CloudFront to respond to IPv6 DNS requests with an IPv6 address for your distribution, specify <code>true</code>. If you specify <code>false</code>, CloudFront responds to IPv6 DNS requests with the DNS response code <code>NOERROR</code> and with no IP addresses. This allows viewers to submit a second request, for an IPv4 address for your distribution. </p> <p>In general, you should enable IPv6 if you have users on IPv6 networks who want to access your content. However, if you're using signed URLs or signed cookies to restrict access to your content, and if you're using a custom policy that includes the <code>IpAddress</code> parameter to restrict the IP addresses that can access your content, don't enable IPv6. If you want to restrict access to some content by IP address and not restrict access to other content (or restrict access but not by IP address), you can create two distributions. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-creating-signed-url-custom-policy.html">Creating a Signed URL Using a Custom Policy</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>If you're using an Amazon Route 53 alias resource record set to route traffic to your CloudFront distribution, you need to create a second alias resource record set when both of the following are true:</p> <ul> <li> <p>You enable IPv6 for the distribution</p> </li> <li> <p>You're using alternate domain names in the URLs for your objects</p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-to-cloudfront-distribution.html">Routing Traffic to an Amazon CloudFront Web Distribution by Using Your Domain Name</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If you created a CNAME resource record set, either with Amazon Route 53 or with another DNS service, you don't need to make any changes. A CNAME record will route traffic to your distribution regardless of the IP address format of the viewer request.</p>
    pub is_ipv6_enabled: Option<bool>,
    /// <p>A complex type that controls whether access logs are written for the distribution.</p> <p>For more information about logging, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/AccessLogs.html">Access Logs</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub logging: Option<LoggingConfig>,
    /// <p>A complex type that contains information about origins for this distribution. </p>
    pub origins: Origins,
    /// <p>The price class that corresponds with the maximum price that you want to pay for CloudFront service. If you specify <code>PriceClass_All</code>, CloudFront responds to requests for your objects from all CloudFront edge locations.</p> <p>If you specify a price class other than <code>PriceClass_All</code>, CloudFront serves your objects from the CloudFront edge location that has the lowest latency among the edge locations in your price class. Viewers who are in or near regions that are excluded from your specified price class may encounter slower performance.</p> <p>For more information about price classes, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PriceClass.html">Choosing the Price Class for a CloudFront Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>. For information about CloudFront pricing, including how price classes map to CloudFront regions, see <a href="https://aws.amazon.com/cloudfront/pricing/">Amazon CloudFront Pricing</a>.</p>
    pub price_class: Option<String>,
    pub restrictions: Option<Restrictions>,
    pub viewer_certificate: Option<ViewerCertificate>,
    /// <p>A unique identifier that specifies the AWS WAF web ACL, if any, to associate with this distribution.</p> <p>AWS WAF is a web application firewall that lets you monitor the HTTP and HTTPS requests that are forwarded to CloudFront, and lets you control access to your content. Based on conditions that you specify, such as the IP addresses that requests originate from or the values of query strings, CloudFront responds to requests either with the requested content or with an HTTP 403 status code (Forbidden). You can also configure CloudFront to return a custom error page when a request is blocked. For more information about AWS WAF, see the <a href="http://docs.aws.amazon.com/waf/latest/developerguide/what-is-aws-waf.html">AWS WAF Developer Guide</a>. </p>
    pub web_acl_id: Option<String>,
}

struct DistributionConfigDeserializer;
impl DistributionConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DistributionConfig, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DistributionConfig::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Aliases" => {
                        obj.aliases =
                            Some(try!(AliasesDeserializer::deserialize("Aliases", stack)));
                    }
                    "CacheBehaviors" => {
                        obj.cache_behaviors = Some(try!(CacheBehaviorsDeserializer::deserialize(
                            "CacheBehaviors",
                            stack
                        )));
                    }
                    "CallerReference" => {
                        obj.caller_reference =
                            try!(StringDeserializer::deserialize("CallerReference", stack));
                    }
                    "Comment" => {
                        obj.comment = try!(StringDeserializer::deserialize("Comment", stack));
                    }
                    "CustomErrorResponses" => {
                        obj.custom_error_responses =
                            Some(try!(CustomErrorResponsesDeserializer::deserialize(
                                "CustomErrorResponses",
                                stack
                            )));
                    }
                    "DefaultCacheBehavior" => {
                        obj.default_cache_behavior =
                            try!(DefaultCacheBehaviorDeserializer::deserialize(
                                "DefaultCacheBehavior",
                                stack
                            ));
                    }
                    "DefaultRootObject" => {
                        obj.default_root_object = Some(try!(StringDeserializer::deserialize(
                            "DefaultRootObject",
                            stack
                        )));
                    }
                    "Enabled" => {
                        obj.enabled = try!(BooleanDeserializer::deserialize("Enabled", stack));
                    }
                    "HttpVersion" => {
                        obj.http_version = Some(try!(HttpVersionDeserializer::deserialize(
                            "HttpVersion",
                            stack
                        )));
                    }
                    "IsIPV6Enabled" => {
                        obj.is_ipv6_enabled = Some(try!(BooleanDeserializer::deserialize(
                            "IsIPV6Enabled",
                            stack
                        )));
                    }
                    "Logging" => {
                        obj.logging = Some(try!(LoggingConfigDeserializer::deserialize(
                            "Logging", stack
                        )));
                    }
                    "Origins" => {
                        obj.origins = try!(OriginsDeserializer::deserialize("Origins", stack));
                    }
                    "PriceClass" => {
                        obj.price_class = Some(try!(PriceClassDeserializer::deserialize(
                            "PriceClass",
                            stack
                        )));
                    }
                    "Restrictions" => {
                        obj.restrictions = Some(try!(RestrictionsDeserializer::deserialize(
                            "Restrictions",
                            stack
                        )));
                    }
                    "ViewerCertificate" => {
                        obj.viewer_certificate = Some(try!(
                            ViewerCertificateDeserializer::deserialize("ViewerCertificate", stack)
                        ));
                    }
                    "WebACLId" => {
                        obj.web_acl_id =
                            Some(try!(StringDeserializer::deserialize("WebACLId", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DistributionList, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DistributionList::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "IsTruncated" => {
                        obj.is_truncated =
                            try!(BooleanDeserializer::deserialize("IsTruncated", stack));
                    }
                    "Items" => {
                        obj.items = Some(try!(DistributionSummaryListDeserializer::deserialize(
                            "Items", stack
                        )));
                    }
                    "Marker" => {
                        obj.marker = try!(StringDeserializer::deserialize("Marker", stack));
                    }
                    "MaxItems" => {
                        obj.max_items = try!(IntegerDeserializer::deserialize("MaxItems", stack));
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(try!(StringDeserializer::deserialize("NextMarker", stack)));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A summary of the information about a CloudFront distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    /// <p>A complex type that contains information about origins for this distribution.</p>
    pub origins: Origins,
    pub price_class: String,
    pub restrictions: Restrictions,
    /// <p>The current status of the distribution. When the status is <code>Deployed</code>, the distribution's information is propagated to all CloudFront edge locations.</p>
    pub status: String,
    pub viewer_certificate: ViewerCertificate,
    /// <p>The Web ACL Id (if any) associated with the distribution.</p>
    pub web_acl_id: String,
}

struct DistributionSummaryDeserializer;
impl DistributionSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DistributionSummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DistributionSummary::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ARN" => {
                        obj.arn = try!(StringDeserializer::deserialize("ARN", stack));
                    }
                    "Aliases" => {
                        obj.aliases = try!(AliasesDeserializer::deserialize("Aliases", stack));
                    }
                    "CacheBehaviors" => {
                        obj.cache_behaviors = try!(CacheBehaviorsDeserializer::deserialize(
                            "CacheBehaviors",
                            stack
                        ));
                    }
                    "Comment" => {
                        obj.comment = try!(StringDeserializer::deserialize("Comment", stack));
                    }
                    "CustomErrorResponses" => {
                        obj.custom_error_responses =
                            try!(CustomErrorResponsesDeserializer::deserialize(
                                "CustomErrorResponses",
                                stack
                            ));
                    }
                    "DefaultCacheBehavior" => {
                        obj.default_cache_behavior =
                            try!(DefaultCacheBehaviorDeserializer::deserialize(
                                "DefaultCacheBehavior",
                                stack
                            ));
                    }
                    "DomainName" => {
                        obj.domain_name =
                            try!(StringDeserializer::deserialize("DomainName", stack));
                    }
                    "Enabled" => {
                        obj.enabled = try!(BooleanDeserializer::deserialize("Enabled", stack));
                    }
                    "HttpVersion" => {
                        obj.http_version =
                            try!(HttpVersionDeserializer::deserialize("HttpVersion", stack));
                    }
                    "Id" => {
                        obj.id = try!(StringDeserializer::deserialize("Id", stack));
                    }
                    "IsIPV6Enabled" => {
                        obj.is_ipv6_enabled =
                            try!(BooleanDeserializer::deserialize("IsIPV6Enabled", stack));
                    }
                    "LastModifiedTime" => {
                        obj.last_modified_time = try!(TimestampDeserializer::deserialize(
                            "LastModifiedTime",
                            stack
                        ));
                    }
                    "Origins" => {
                        obj.origins = try!(OriginsDeserializer::deserialize("Origins", stack));
                    }
                    "PriceClass" => {
                        obj.price_class =
                            try!(PriceClassDeserializer::deserialize("PriceClass", stack));
                    }
                    "Restrictions" => {
                        obj.restrictions =
                            try!(RestrictionsDeserializer::deserialize("Restrictions", stack));
                    }
                    "Status" => {
                        obj.status = try!(StringDeserializer::deserialize("Status", stack));
                    }
                    "ViewerCertificate" => {
                        obj.viewer_certificate = try!(ViewerCertificateDeserializer::deserialize(
                            "ViewerCertificate",
                            stack
                        ));
                    }
                    "WebACLId" => {
                        obj.web_acl_id = try!(StringDeserializer::deserialize("WebACLId", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DistributionSummaryListDeserializer;
impl DistributionSummaryListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DistributionSummary>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DistributionSummary" {
                        obj.push(try!(DistributionSummaryDeserializer::deserialize(
                            "DistributionSummary",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct EventTypeDeserializer;
impl EventTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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

/// <p>A complex type that specifies how CloudFront handles query strings and cookies.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ForwardedValues, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ForwardedValues::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Cookies" => {
                        obj.cookies =
                            try!(CookiePreferenceDeserializer::deserialize("Cookies", stack));
                    }
                    "Headers" => {
                        obj.headers =
                            Some(try!(HeadersDeserializer::deserialize("Headers", stack)));
                    }
                    "QueryString" => {
                        obj.query_string =
                            try!(BooleanDeserializer::deserialize("QueryString", stack));
                    }
                    "QueryStringCacheKeys" => {
                        obj.query_string_cache_keys =
                            Some(try!(QueryStringCacheKeysDeserializer::deserialize(
                                "QueryStringCacheKeys",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GeoRestriction, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GeoRestriction::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items =
                            Some(try!(LocationListDeserializer::deserialize("Items", stack)));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    "RestrictionType" => {
                        obj.restriction_type = try!(GeoRestrictionTypeDeserializer::deserialize(
                            "RestrictionType",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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
pub struct GetCloudFrontOriginAccessIdentityConfigRequest {
    /// <p>The identity's ID. </p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetCloudFrontOriginAccessIdentityConfigResult {
    /// <p>The origin access identity's configuration information. </p>
    pub cloud_front_origin_access_identity_config: Option<CloudFrontOriginAccessIdentityConfig>,
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

struct GetCloudFrontOriginAccessIdentityConfigResultDeserializer;
impl GetCloudFrontOriginAccessIdentityConfigResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetCloudFrontOriginAccessIdentityConfigResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetCloudFrontOriginAccessIdentityConfigResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CloudFrontOriginAccessIdentityConfig" => {
                        obj.cloud_front_origin_access_identity_config = Some(try!(
                            CloudFrontOriginAccessIdentityConfigDeserializer::deserialize(
                                "CloudFrontOriginAccessIdentityConfig",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to get an origin access identity's information.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetCloudFrontOriginAccessIdentityRequest {
    /// <p>The identity's ID.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetCloudFrontOriginAccessIdentityResult {
    /// <p>The origin access identity's information.</p>
    pub cloud_front_origin_access_identity: Option<CloudFrontOriginAccessIdentity>,
    /// <p>The current version of the origin access identity's information. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

struct GetCloudFrontOriginAccessIdentityResultDeserializer;
impl GetCloudFrontOriginAccessIdentityResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetCloudFrontOriginAccessIdentityResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetCloudFrontOriginAccessIdentityResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CloudFrontOriginAccessIdentity" => {
                        obj.cloud_front_origin_access_identity = Some(try!(
                            CloudFrontOriginAccessIdentityDeserializer::deserialize(
                                "CloudFrontOriginAccessIdentity",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to get a distribution configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetDistributionConfigRequest {
    /// <p>The distribution's ID.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetDistributionConfigResult {
    /// <p>The distribution's configuration information.</p>
    pub distribution_config: Option<DistributionConfig>,
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

struct GetDistributionConfigResultDeserializer;
impl GetDistributionConfigResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetDistributionConfigResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetDistributionConfigResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DistributionConfig" => {
                        obj.distribution_config =
                            Some(try!(DistributionConfigDeserializer::deserialize(
                                "DistributionConfig",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to get a distribution's information.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetDistributionRequest {
    /// <p>The distribution's ID.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetDistributionResult {
    /// <p>The distribution's information.</p>
    pub distribution: Option<Distribution>,
    /// <p>The current version of the distribution's information. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

struct GetDistributionResultDeserializer;
impl GetDistributionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetDistributionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetDistributionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Distribution" => {
                        obj.distribution = Some(try!(DistributionDeserializer::deserialize(
                            "Distribution",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to get an invalidation's information. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetInvalidationRequest {
    /// <p>The distribution's ID.</p>
    pub distribution_id: String,
    /// <p>The identifier for the invalidation request, for example, <code>IDFDVBD632BHDS5</code>.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetInvalidationResult {
    /// <p>The invalidation's information. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/InvalidationDatatype.html">Invalidation Complex Type</a>. </p>
    pub invalidation: Option<Invalidation>,
}

struct GetInvalidationResultDeserializer;
impl GetInvalidationResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetInvalidationResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetInvalidationResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Invalidation" => {
                        obj.invalidation = Some(try!(InvalidationDeserializer::deserialize(
                            "Invalidation",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>To request to get a streaming distribution configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetStreamingDistributionConfigRequest {
    /// <p>The streaming distribution's ID.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetStreamingDistributionConfigResult {
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>. </p>
    pub e_tag: Option<String>,
    /// <p>The streaming distribution's configuration information.</p>
    pub streaming_distribution_config: Option<StreamingDistributionConfig>,
}

struct GetStreamingDistributionConfigResultDeserializer;
impl GetStreamingDistributionConfigResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetStreamingDistributionConfigResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetStreamingDistributionConfigResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "StreamingDistributionConfig" => {
                        obj.streaming_distribution_config =
                            Some(try!(StreamingDistributionConfigDeserializer::deserialize(
                                "StreamingDistributionConfig",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to get a streaming distribution's information.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetStreamingDistributionRequest {
    /// <p>The streaming distribution's ID.</p>
    pub id: String,
}

/// <p>The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetStreamingDistributionResult {
    /// <p>The current version of the streaming distribution's information. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>The streaming distribution's information.</p>
    pub streaming_distribution: Option<StreamingDistribution>,
}

struct GetStreamingDistributionResultDeserializer;
impl GetStreamingDistributionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetStreamingDistributionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetStreamingDistributionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "StreamingDistribution" => {
                        obj.streaming_distribution =
                            Some(try!(StreamingDistributionDeserializer::deserialize(
                                "StreamingDistribution",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct HeaderListDeserializer;
impl HeaderListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Name" {
                        obj.push(try!(StringDeserializer::deserialize("Name", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
pub struct Headers {
    /// <p>A list that contains one <code>Name</code> element for each header that you want CloudFront to use for caching in this cache behavior. If <code>Quantity</code> is <code>0</code>, omit <code>Items</code>.</p>
    pub items: Option<Vec<String>>,
    /// <p><p>The number of different headers that you want CloudFront to base caching on for this cache behavior. You can configure each cache behavior in a web distribution to do one of the following:</p> <ul> <li> <p> <b>Forward all headers to your origin</b>: Specify <code>1</code> for <code>Quantity</code> and <code>*</code> for <code>Name</code>.</p> <important> <p>CloudFront doesn&#39;t cache the objects that are associated with this cache behavior. Instead, CloudFront sends every request to the origin. </p> </important> </li> <li> <p> <b>Forward a whitelist of headers you specify</b>: Specify the number of headers that you want CloudFront to base caching on. Then specify the header names in <code>Name</code> elements. CloudFront caches your objects based on the values in the specified headers.</p> </li> <li> <p> <b>Forward only the default headers</b>: Specify <code>0</code> for <code>Quantity</code> and omit <code>Items</code>. In this configuration, CloudFront doesn&#39;t cache based on the values in the request headers.</p> </li> </ul> <p>Regardless of which option you choose, CloudFront forwards headers to your origin based on whether the origin is an S3 bucket or a custom origin. See the following documentation:</p> <ul> <li> <p> <b>S3 bucket</b>: See <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/RequestAndResponseBehaviorS3Origin.html#request-s3-removed-headers">HTTP Request Headers That CloudFront Removes or Updates</a> </p> </li> <li> <p> <b>Custom origin</b>: See <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/RequestAndResponseBehaviorCustomOrigin.html#request-custom-headers-behavior">HTTP Request Headers and CloudFront Behavior</a> </p> </li> </ul></p>
    pub quantity: i64,
}

struct HeadersDeserializer;
impl HeadersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Headers, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Headers::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items = Some(try!(HeaderListDeserializer::deserialize("Items", stack)));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Invalidation, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Invalidation::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CreateTime" => {
                        obj.create_time =
                            try!(TimestampDeserializer::deserialize("CreateTime", stack));
                    }
                    "Id" => {
                        obj.id = try!(StringDeserializer::deserialize("Id", stack));
                    }
                    "InvalidationBatch" => {
                        obj.invalidation_batch = try!(InvalidationBatchDeserializer::deserialize(
                            "InvalidationBatch",
                            stack
                        ));
                    }
                    "Status" => {
                        obj.status = try!(StringDeserializer::deserialize("Status", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>An invalidation batch.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InvalidationBatch {
    /// <p>A value that you specify to uniquely identify an invalidation request. CloudFront uses the value to prevent you from accidentally resubmitting an identical request. Whenever you create a new invalidation request, you must specify a new value for <code>CallerReference</code> and change other values in the request as applicable. One way to ensure that the value of <code>CallerReference</code> is unique is to use a <code>timestamp</code>, for example, <code>20120301090000</code>.</p> <p>If you make a second invalidation request with the same value for <code>CallerReference</code>, and if the rest of the request is the same, CloudFront doesn't create a new invalidation request. Instead, CloudFront returns information about the invalidation request that you previously created with the same <code>CallerReference</code>.</p> <p>If <code>CallerReference</code> is a value you already sent in a previous invalidation batch request but the content of any <code>Path</code> is different from the original request, CloudFront returns an <code>InvalidationBatchAlreadyExists</code> error.</p>
    pub caller_reference: String,
    /// <p>A complex type that contains information about the objects that you want to invalidate. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Invalidation.html#invalidation-specifying-objects">Specifying the Objects to Invalidate</a> in the <i>Amazon CloudFront Developer Guide</i>. </p>
    pub paths: Paths,
}

struct InvalidationBatchDeserializer;
impl InvalidationBatchDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InvalidationBatch, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InvalidationBatch::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CallerReference" => {
                        obj.caller_reference =
                            try!(StringDeserializer::deserialize("CallerReference", stack));
                    }
                    "Paths" => {
                        obj.paths = try!(PathsDeserializer::deserialize("Paths", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InvalidationList, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InvalidationList::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "IsTruncated" => {
                        obj.is_truncated =
                            try!(BooleanDeserializer::deserialize("IsTruncated", stack));
                    }
                    "Items" => {
                        obj.items = Some(try!(InvalidationSummaryListDeserializer::deserialize(
                            "Items", stack
                        )));
                    }
                    "Marker" => {
                        obj.marker = try!(StringDeserializer::deserialize("Marker", stack));
                    }
                    "MaxItems" => {
                        obj.max_items = try!(IntegerDeserializer::deserialize("MaxItems", stack));
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(try!(StringDeserializer::deserialize("NextMarker", stack)));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A summary of an invalidation request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InvalidationSummary {
    pub create_time: String,
    /// <p>The unique ID for an invalidation request.</p>
    pub id: String,
    /// <p>The status of an invalidation request.</p>
    pub status: String,
}

struct InvalidationSummaryDeserializer;
impl InvalidationSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InvalidationSummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InvalidationSummary::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CreateTime" => {
                        obj.create_time =
                            try!(TimestampDeserializer::deserialize("CreateTime", stack));
                    }
                    "Id" => {
                        obj.id = try!(StringDeserializer::deserialize("Id", stack));
                    }
                    "Status" => {
                        obj.status = try!(StringDeserializer::deserialize("Status", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct InvalidationSummaryListDeserializer;
impl InvalidationSummaryListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<InvalidationSummary>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "InvalidationSummary" {
                        obj.push(try!(InvalidationSummaryDeserializer::deserialize(
                            "InvalidationSummary",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct ItemSelectionDeserializer;
impl ItemSelectionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "KeyPairId" {
                        obj.push(try!(StringDeserializer::deserialize("KeyPairId", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>A complex type that lists the active CloudFront key pairs, if any, that are associated with <code>AwsAccountNumber</code>. </p> <p>For more information, see <a>ActiveTrustedSigners</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct KeyPairIds {
    /// <p>A complex type that lists the active CloudFront key pairs, if any, that are associated with <code>AwsAccountNumber</code>.</p> <p>For more information, see <a>ActiveTrustedSigners</a>.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of active CloudFront key pairs for <code>AwsAccountNumber</code>.</p> <p>For more information, see <a>ActiveTrustedSigners</a>.</p>
    pub quantity: i64,
}

struct KeyPairIdsDeserializer;
impl KeyPairIdsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<KeyPairIds, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = KeyPairIds::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items =
                            Some(try!(KeyPairIdListDeserializer::deserialize("Items", stack)));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A complex type that contains a Lambda function association.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LambdaFunctionAssociation {
    /// <p><p>Specifies the event type that triggers a Lambda function invocation. You can specify the following values:</p> <ul> <li> <p> <code>viewer-request</code>: The function executes when CloudFront receives a request from a viewer and before it checks to see whether the requested object is in the edge cache. </p> </li> <li> <p> <code>origin-request</code>: The function executes only when CloudFront forwards a request to your origin. When the requested object is in the edge cache, the function doesn&#39;t execute.</p> </li> <li> <p> <code>origin-response</code>: The function executes after CloudFront receives a response from the origin and before it caches the object in the response. When the requested object is in the edge cache, the function doesn&#39;t execute.</p> <p>If the origin returns an HTTP status code other than HTTP 200 (OK), the function doesn&#39;t execute.</p> </li> <li> <p> <code>viewer-response</code>: The function executes before CloudFront returns the requested object to the viewer. The function executes regardless of whether the object was already in the edge cache.</p> <p>If the origin returns an HTTP status code other than HTTP 200 (OK), the function doesn&#39;t execute.</p> </li> </ul></p>
    pub event_type: Option<String>,
    /// <p>The ARN of the Lambda function. You must specify the ARN of a function version; you can't specify a Lambda alias or $LATEST.</p>
    pub lambda_function_arn: Option<String>,
}

struct LambdaFunctionAssociationDeserializer;
impl LambdaFunctionAssociationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LambdaFunctionAssociation, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LambdaFunctionAssociation::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "EventType" => {
                        obj.event_type =
                            Some(try!(EventTypeDeserializer::deserialize("EventType", stack)));
                    }
                    "LambdaFunctionARN" => {
                        obj.lambda_function_arn = Some(try!(StringDeserializer::deserialize(
                            "LambdaFunctionARN",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
        if let Some(ref value) = obj.event_type {
            writer.write(xml::writer::XmlEvent::start_element("EventType"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.lambda_function_arn {
            writer.write(xml::writer::XmlEvent::start_element("LambdaFunctionARN"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct LambdaFunctionAssociationListDeserializer;
impl LambdaFunctionAssociationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LambdaFunctionAssociation>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "LambdaFunctionAssociation" {
                        obj.push(try!(LambdaFunctionAssociationDeserializer::deserialize(
                            "LambdaFunctionAssociation",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
pub struct LambdaFunctionAssociations {
    /// <p> <b>Optional</b>: A complex type that contains <code>LambdaFunctionAssociation</code> items for this cache behavior. If <code>Quantity</code> is <code>0</code>, you can omit <code>Items</code>.</p>
    pub items: Option<Vec<LambdaFunctionAssociation>>,
    /// <p>The number of Lambda function associations for this cache behavior.</p>
    pub quantity: i64,
}

struct LambdaFunctionAssociationsDeserializer;
impl LambdaFunctionAssociationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LambdaFunctionAssociations, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LambdaFunctionAssociations::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items = Some(try!(
                            LambdaFunctionAssociationListDeserializer::deserialize("Items", stack)
                        ));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
pub struct ListCloudFrontOriginAccessIdentitiesRequest {
    /// <p>Use this when paginating results to indicate where to begin in your list of origin access identities. The results include identities in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last identity on that page).</p>
    pub marker: Option<String>,
    /// <p>The maximum number of origin access identities you want in the response body. </p>
    pub max_items: Option<String>,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListCloudFrontOriginAccessIdentitiesResult {
    /// <p>The <code>CloudFrontOriginAccessIdentityList</code> type. </p>
    pub cloud_front_origin_access_identity_list: Option<CloudFrontOriginAccessIdentityList>,
}

struct ListCloudFrontOriginAccessIdentitiesResultDeserializer;
impl ListCloudFrontOriginAccessIdentitiesResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListCloudFrontOriginAccessIdentitiesResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListCloudFrontOriginAccessIdentitiesResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CloudFrontOriginAccessIdentityList" => {
                        obj.cloud_front_origin_access_identity_list = Some(try!(
                            CloudFrontOriginAccessIdentityListDeserializer::deserialize(
                                "CloudFrontOriginAccessIdentityList",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to list distributions that are associated with a specified AWS WAF web ACL. </p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct ListDistributionsByWebACLIdResult {
    /// <p>The <code>DistributionList</code> type. </p>
    pub distribution_list: Option<DistributionList>,
}

struct ListDistributionsByWebACLIdResultDeserializer;
impl ListDistributionsByWebACLIdResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDistributionsByWebACLIdResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListDistributionsByWebACLIdResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DistributionList" => {
                        obj.distribution_list = Some(try!(
                            DistributionListDeserializer::deserialize("DistributionList", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to list your distributions. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListDistributionsRequest {
    /// <p>Use this when paginating results to indicate where to begin in your list of distributions. The results include distributions in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last distribution on that page).</p>
    pub marker: Option<String>,
    /// <p>The maximum number of distributions you want in the response body.</p>
    pub max_items: Option<String>,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListDistributionsResult {
    /// <p>The <code>DistributionList</code> type. </p>
    pub distribution_list: Option<DistributionList>,
}

struct ListDistributionsResultDeserializer;
impl ListDistributionsResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDistributionsResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListDistributionsResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DistributionList" => {
                        obj.distribution_list = Some(try!(
                            DistributionListDeserializer::deserialize("DistributionList", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to list invalidations. </p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct ListInvalidationsResult {
    /// <p>Information about invalidation batches. </p>
    pub invalidation_list: Option<InvalidationList>,
}

struct ListInvalidationsResultDeserializer;
impl ListInvalidationsResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListInvalidationsResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListInvalidationsResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "InvalidationList" => {
                        obj.invalidation_list = Some(try!(
                            InvalidationListDeserializer::deserialize("InvalidationList", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to list your streaming distributions. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStreamingDistributionsRequest {
    /// <p>The value that you provided for the <code>Marker</code> request parameter.</p>
    pub marker: Option<String>,
    /// <p>The value that you provided for the <code>MaxItems</code> request parameter.</p>
    pub max_items: Option<String>,
}

/// <p>The returned result of the corresponding request. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStreamingDistributionsResult {
    /// <p>The <code>StreamingDistributionList</code> type. </p>
    pub streaming_distribution_list: Option<StreamingDistributionList>,
}

struct ListStreamingDistributionsResultDeserializer;
impl ListStreamingDistributionsResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStreamingDistributionsResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListStreamingDistributionsResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "StreamingDistributionList" => {
                        obj.streaming_distribution_list =
                            Some(try!(StreamingDistributionListDeserializer::deserialize(
                                "StreamingDistributionList",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p> The request to list tags for a CloudFront resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTagsForResourceRequest {
    /// <p> An ARN of a CloudFront resource.</p>
    pub resource: String,
}

/// <p> The returned result of the corresponding request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTagsForResourceResult {
    /// <p> A complex type that contains zero or more <code>Tag</code> elements.</p>
    pub tags: Tags,
}

struct ListTagsForResourceResultDeserializer;
impl ListTagsForResourceResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTagsForResourceResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListTagsForResourceResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Tags" => {
                        obj.tags = try!(TagsDeserializer::deserialize("Tags", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct LocationListDeserializer;
impl LocationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Location" {
                        obj.push(try!(StringDeserializer::deserialize("Location", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoggingConfig, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LoggingConfig::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Bucket" => {
                        obj.bucket = try!(StringDeserializer::deserialize("Bucket", stack));
                    }
                    "Enabled" => {
                        obj.enabled = try!(BooleanDeserializer::deserialize("Enabled", stack));
                    }
                    "IncludeCookies" => {
                        obj.include_cookies =
                            try!(BooleanDeserializer::deserialize("IncludeCookies", stack));
                    }
                    "Prefix" => {
                        obj.prefix = try!(StringDeserializer::deserialize("Prefix", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Method" {
                        obj.push(try!(MethodDeserializer::deserialize("Method", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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

/// <p>A complex type that describes the Amazon S3 bucket or the HTTP server (for example, a web server) from which CloudFront gets your files. You must create at least one origin.</p> <p>For the current limit on the number of origins that you can create for a distribution, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_cloudfront">Amazon CloudFront Limits</a> in the <i>AWS General Reference</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Origin {
    /// <p>A complex type that contains names and values for the custom headers that you want.</p>
    pub custom_headers: Option<CustomHeaders>,
    /// <p>A complex type that contains information about a custom origin. If the origin is an Amazon S3 bucket, use the <code>S3OriginConfig</code> element instead.</p>
    pub custom_origin_config: Option<CustomOriginConfig>,
    /// <p><p> <b>Amazon S3 origins</b>: The DNS name of the Amazon S3 bucket from which you want CloudFront to get objects for this origin, for example, <code>myawsbucket.s3.amazonaws.com</code>.</p> <p>Constraints for Amazon S3 origins: </p> <ul> <li> <p>If you configured Amazon S3 Transfer Acceleration for your bucket, don&#39;t specify the <code>s3-accelerate</code> endpoint for <code>DomainName</code>.</p> </li> <li> <p>The bucket name must be between 3 and 63 characters long (inclusive).</p> </li> <li> <p>The bucket name must contain only lowercase characters, numbers, periods, underscores, and dashes.</p> </li> <li> <p>The bucket name must not contain adjacent periods.</p> </li> </ul> <p> <b>Custom Origins</b>: The DNS domain name for the HTTP server from which you want CloudFront to get objects for this origin, for example, <code>www.example.com</code>. </p> <p>Constraints for custom origins:</p> <ul> <li> <p> <code>DomainName</code> must be a valid DNS name that contains only a-z, A-Z, 0-9, dot (.), hyphen (-), or underscore (_) characters.</p> </li> <li> <p>The name cannot exceed 128 characters.</p> </li> </ul></p>
    pub domain_name: String,
    /// <p>A unique identifier for the origin. The value of <code>Id</code> must be unique within the distribution.</p> <p>When you specify the value of <code>TargetOriginId</code> for the default cache behavior or for another cache behavior, you indicate the origin to which you want the cache behavior to route requests by specifying the value of the <code>Id</code> element for that origin. When a request matches the path pattern for that cache behavior, CloudFront routes the request to the specified origin. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesCacheBehavior">Cache Behavior Settings</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub id: String,
    /// <p>An optional element that causes CloudFront to request your content from a directory in your Amazon S3 bucket or your custom origin. When you include the <code>OriginPath</code> element, specify the directory name, beginning with a <code>/</code>. CloudFront appends the directory name to the value of <code>DomainName</code>, for example, <code>example.com/production</code>. Do not include a <code>/</code> at the end of the directory name.</p> <p>For example, suppose you've specified the following values for your distribution:</p> <ul> <li> <p> <code>DomainName</code>: An Amazon S3 bucket named <code>myawsbucket</code>.</p> </li> <li> <p> <code>OriginPath</code>: <code>/production</code> </p> </li> <li> <p> <code>CNAME</code>: <code>example.com</code> </p> </li> </ul> <p>When a user enters <code>example.com/index.html</code> in a browser, CloudFront sends a request to Amazon S3 for <code>myawsbucket/production/index.html</code>.</p> <p>When a user enters <code>example.com/acme/index.html</code> in a browser, CloudFront sends a request to Amazon S3 for <code>myawsbucket/production/acme/index.html</code>.</p>
    pub origin_path: Option<String>,
    /// <p>A complex type that contains information about the Amazon S3 origin. If the origin is a custom origin, use the <code>CustomOriginConfig</code> element instead.</p>
    pub s3_origin_config: Option<S3OriginConfig>,
}

struct OriginDeserializer;
impl OriginDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Origin, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Origin::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CustomHeaders" => {
                            obj.custom_headers = Some(try!(
                                CustomHeadersDeserializer::deserialize("CustomHeaders", stack)
                            ));
                        }
                        "CustomOriginConfig" => {
                            obj.custom_origin_config =
                                Some(try!(CustomOriginConfigDeserializer::deserialize(
                                    "CustomOriginConfig",
                                    stack
                                )));
                        }
                        "DomainName" => {
                            obj.domain_name =
                                try!(StringDeserializer::deserialize("DomainName", stack));
                        }
                        "Id" => {
                            obj.id = try!(StringDeserializer::deserialize("Id", stack));
                        }
                        "OriginPath" => {
                            obj.origin_path =
                                Some(try!(StringDeserializer::deserialize("OriginPath", stack)));
                        }
                        "S3OriginConfig" => {
                            obj.s3_origin_config = Some(try!(
                                S3OriginConfigDeserializer::deserialize("S3OriginConfig", stack)
                            ));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
pub struct OriginCustomHeader {
    /// <p>The name of a header that you want CloudFront to forward to your origin. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/forward-custom-headers.html">Forwarding Custom Headers to Your Origin (Web Distributions Only)</a> in the <i>Amazon Amazon CloudFront Developer Guide</i>.</p>
    pub header_name: String,
    /// <p>The value for the header that you specified in the <code>HeaderName</code> field.</p>
    pub header_value: String,
}

struct OriginCustomHeaderDeserializer;
impl OriginCustomHeaderDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginCustomHeader, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = OriginCustomHeader::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "HeaderName" => {
                        obj.header_name =
                            try!(StringDeserializer::deserialize("HeaderName", stack));
                    }
                    "HeaderValue" => {
                        obj.header_value =
                            try!(StringDeserializer::deserialize("HeaderValue", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<OriginCustomHeader>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "OriginCustomHeader" {
                        obj.push(try!(OriginCustomHeaderDeserializer::deserialize(
                            "OriginCustomHeader",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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

struct OriginListDeserializer;
impl OriginListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Origin>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Origin" {
                        obj.push(try!(OriginDeserializer::deserialize("Origin", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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
pub struct OriginSslProtocols {
    /// <p>A list that contains allowed SSL/TLS protocols for this distribution.</p>
    pub items: Vec<String>,
    /// <p>The number of SSL/TLS protocols that you want to allow CloudFront to use when establishing an HTTPS connection with this origin. </p>
    pub quantity: i64,
}

struct OriginSslProtocolsDeserializer;
impl OriginSslProtocolsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OriginSslProtocols, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = OriginSslProtocols::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items = try!(SslProtocolsListDeserializer::deserialize("Items", stack));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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

/// <p>A complex type that contains information about origins for this distribution. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Origins {
    /// <p>A complex type that contains origins for this distribution.</p>
    pub items: Option<Vec<Origin>>,
    /// <p>The number of origins for this distribution.</p>
    pub quantity: i64,
}

struct OriginsDeserializer;
impl OriginsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Origins, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Origins::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items = Some(try!(OriginListDeserializer::deserialize("Items", stack)));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
        if let Some(ref value) = obj.items {
            &OriginListSerializer::serialize(&mut writer, "Items", value)?;
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

struct PathListDeserializer;
impl PathListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Path" {
                        obj.push(try!(StringDeserializer::deserialize("Path", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
pub struct Paths {
    /// <p>A complex type that contains a list of the paths that you want to invalidate.</p>
    pub items: Option<Vec<String>>,
    /// <p>The number of objects that you want to invalidate.</p>
    pub quantity: i64,
}

struct PathsDeserializer;
impl PathsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Paths, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Paths::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items = Some(try!(PathListDeserializer::deserialize("Items", stack)));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct QueryStringCacheKeys {
    /// <p>(Optional) A list that contains the query string parameters that you want CloudFront to use as a basis for caching for this cache behavior. If <code>Quantity</code> is 0, you can omit <code>Items</code>. </p>
    pub items: Option<Vec<String>>,
    /// <p>The number of <code>whitelisted</code> query string parameters for this cache behavior.</p>
    pub quantity: i64,
}

struct QueryStringCacheKeysDeserializer;
impl QueryStringCacheKeysDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueryStringCacheKeys, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = QueryStringCacheKeys::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items = Some(try!(QueryStringCacheKeysListDeserializer::deserialize(
                            "Items", stack
                        )));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Name" {
                        obj.push(try!(StringDeserializer::deserialize("Name", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
pub struct Restrictions {
    pub geo_restriction: GeoRestriction,
}

struct RestrictionsDeserializer;
impl RestrictionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Restrictions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Restrictions::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "GeoRestriction" => {
                        obj.geo_restriction = try!(GeoRestrictionDeserializer::deserialize(
                            "GeoRestriction",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
pub struct S3Origin {
    /// <p>The DNS name of the Amazon S3 origin. </p>
    pub domain_name: String,
    /// <p>The CloudFront origin access identity to associate with the RTMP distribution. Use an origin access identity to configure the distribution so that end users can only access objects in an Amazon S3 bucket through CloudFront.</p> <p>If you want end users to be able to access objects using either the CloudFront URL or the Amazon S3 URL, specify an empty <code>OriginAccessIdentity</code> element.</p> <p>To delete the origin access identity from an existing distribution, update the distribution configuration and include an empty <code>OriginAccessIdentity</code> element.</p> <p>To replace the origin access identity, update the distribution configuration and specify the new origin access identity.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-restricting-access-to-s3.html">Using an Origin Access Identity to Restrict Access to Your Amazon S3 Content</a> in the <i>Amazon Amazon CloudFront Developer Guide</i>.</p>
    pub origin_access_identity: String,
}

struct S3OriginDeserializer;
impl S3OriginDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<S3Origin, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = S3Origin::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DomainName" => {
                        obj.domain_name =
                            try!(StringDeserializer::deserialize("DomainName", stack));
                    }
                    "OriginAccessIdentity" => {
                        obj.origin_access_identity = try!(StringDeserializer::deserialize(
                            "OriginAccessIdentity",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
pub struct S3OriginConfig {
    /// <p>The CloudFront origin access identity to associate with the origin. Use an origin access identity to configure the origin so that viewers can <i>only</i> access objects in an Amazon S3 bucket through CloudFront. The format of the value is:</p> <p>origin-access-identity/cloudfront/<i>ID-of-origin-access-identity</i> </p> <p>where <code> <i>ID-of-origin-access-identity</i> </code> is the value that CloudFront returned in the <code>ID</code> element when you created the origin access identity.</p> <p>If you want viewers to be able to access objects using either the CloudFront URL or the Amazon S3 URL, specify an empty <code>OriginAccessIdentity</code> element.</p> <p>To delete the origin access identity from an existing distribution, update the distribution configuration and include an empty <code>OriginAccessIdentity</code> element.</p> <p>To replace the origin access identity, update the distribution configuration and specify the new origin access identity.</p> <p>For more information about the origin access identity, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub origin_access_identity: String,
}

struct S3OriginConfigDeserializer;
impl S3OriginConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<S3OriginConfig, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = S3OriginConfig::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "OriginAccessIdentity" => {
                        obj.origin_access_identity = try!(StringDeserializer::deserialize(
                            "OriginAccessIdentity",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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
pub struct Signer {
    /// <p><p>An AWS account that is included in the <code>TrustedSigners</code> complex type for this RTMP distribution. Valid values include:</p> <ul> <li> <p> <code>self</code>, which is the AWS account used to create the distribution.</p> </li> <li> <p>An AWS account number.</p> </li> </ul></p>
    pub aws_account_number: Option<String>,
    /// <p>A complex type that lists the active CloudFront key pairs, if any, that are associated with <code>AwsAccountNumber</code>.</p>
    pub key_pair_ids: Option<KeyPairIds>,
}

struct SignerDeserializer;
impl SignerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Signer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Signer::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AwsAccountNumber" => {
                        obj.aws_account_number = Some(try!(StringDeserializer::deserialize(
                            "AwsAccountNumber",
                            stack
                        )));
                    }
                    "KeyPairIds" => {
                        obj.key_pair_ids = Some(try!(KeyPairIdsDeserializer::deserialize(
                            "KeyPairIds",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SignerListDeserializer;
impl SignerListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Signer>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Signer" {
                        obj.push(try!(SignerDeserializer::deserialize("Signer", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct SslProtocolDeserializer;
impl SslProtocolDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "SslProtocol" {
                        obj.push(try!(SslProtocolDeserializer::deserialize(
                            "SslProtocol",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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

/// <p>A streaming distribution. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StreamingDistribution {
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StreamingDistribution, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StreamingDistribution::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ARN" => {
                        obj.arn = try!(StringDeserializer::deserialize("ARN", stack));
                    }
                    "ActiveTrustedSigners" => {
                        obj.active_trusted_signers =
                            try!(ActiveTrustedSignersDeserializer::deserialize(
                                "ActiveTrustedSigners",
                                stack
                            ));
                    }
                    "DomainName" => {
                        obj.domain_name =
                            try!(StringDeserializer::deserialize("DomainName", stack));
                    }
                    "Id" => {
                        obj.id = try!(StringDeserializer::deserialize("Id", stack));
                    }
                    "LastModifiedTime" => {
                        obj.last_modified_time = Some(try!(TimestampDeserializer::deserialize(
                            "LastModifiedTime",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = try!(StringDeserializer::deserialize("Status", stack));
                    }
                    "StreamingDistributionConfig" => {
                        obj.streaming_distribution_config =
                            try!(StreamingDistributionConfigDeserializer::deserialize(
                                "StreamingDistributionConfig",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The RTMP distribution's configuration information.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StreamingDistributionConfig {
    /// <p>A complex type that contains information about CNAMEs (alternate domain names), if any, for this streaming distribution. </p>
    pub aliases: Option<Aliases>,
    /// <p>A unique number that ensures that the request can't be replayed. If the <code>CallerReference</code> is new (no matter the content of the <code>StreamingDistributionConfig</code> object), a new streaming distribution is created. If the <code>CallerReference</code> is a value that you already sent in a previous request to create a streaming distribution, and the content of the <code>StreamingDistributionConfig</code> is identical to the original request (ignoring white space), the response includes the same information returned to the original request. If the <code>CallerReference</code> is a value that you already sent in a previous request to create a streaming distribution but the content of the <code>StreamingDistributionConfig</code> is different from the original request, CloudFront returns a <code>DistributionAlreadyExists</code> error. </p>
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StreamingDistributionConfig, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StreamingDistributionConfig::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Aliases" => {
                        obj.aliases =
                            Some(try!(AliasesDeserializer::deserialize("Aliases", stack)));
                    }
                    "CallerReference" => {
                        obj.caller_reference =
                            try!(StringDeserializer::deserialize("CallerReference", stack));
                    }
                    "Comment" => {
                        obj.comment = try!(StringDeserializer::deserialize("Comment", stack));
                    }
                    "Enabled" => {
                        obj.enabled = try!(BooleanDeserializer::deserialize("Enabled", stack));
                    }
                    "Logging" => {
                        obj.logging = Some(try!(StreamingLoggingConfigDeserializer::deserialize(
                            "Logging", stack
                        )));
                    }
                    "PriceClass" => {
                        obj.price_class = Some(try!(PriceClassDeserializer::deserialize(
                            "PriceClass",
                            stack
                        )));
                    }
                    "S3Origin" => {
                        obj.s3_origin = try!(S3OriginDeserializer::deserialize("S3Origin", stack));
                    }
                    "TrustedSigners" => {
                        obj.trusted_signers = try!(TrustedSignersDeserializer::deserialize(
                            "TrustedSigners",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StreamingDistributionList, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StreamingDistributionList::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "IsTruncated" => {
                        obj.is_truncated =
                            try!(BooleanDeserializer::deserialize("IsTruncated", stack));
                    }
                    "Items" => {
                        obj.items = Some(try!(
                            StreamingDistributionSummaryListDeserializer::deserialize(
                                "Items", stack
                            )
                        ));
                    }
                    "Marker" => {
                        obj.marker = try!(StringDeserializer::deserialize("Marker", stack));
                    }
                    "MaxItems" => {
                        obj.max_items = try!(IntegerDeserializer::deserialize("MaxItems", stack));
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(try!(StringDeserializer::deserialize("NextMarker", stack)));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p> A summary of the information for an Amazon CloudFront streaming distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StreamingDistributionSummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StreamingDistributionSummary::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ARN" => {
                        obj.arn = try!(StringDeserializer::deserialize("ARN", stack));
                    }
                    "Aliases" => {
                        obj.aliases = try!(AliasesDeserializer::deserialize("Aliases", stack));
                    }
                    "Comment" => {
                        obj.comment = try!(StringDeserializer::deserialize("Comment", stack));
                    }
                    "DomainName" => {
                        obj.domain_name =
                            try!(StringDeserializer::deserialize("DomainName", stack));
                    }
                    "Enabled" => {
                        obj.enabled = try!(BooleanDeserializer::deserialize("Enabled", stack));
                    }
                    "Id" => {
                        obj.id = try!(StringDeserializer::deserialize("Id", stack));
                    }
                    "LastModifiedTime" => {
                        obj.last_modified_time = try!(TimestampDeserializer::deserialize(
                            "LastModifiedTime",
                            stack
                        ));
                    }
                    "PriceClass" => {
                        obj.price_class =
                            try!(PriceClassDeserializer::deserialize("PriceClass", stack));
                    }
                    "S3Origin" => {
                        obj.s3_origin = try!(S3OriginDeserializer::deserialize("S3Origin", stack));
                    }
                    "Status" => {
                        obj.status = try!(StringDeserializer::deserialize("Status", stack));
                    }
                    "TrustedSigners" => {
                        obj.trusted_signers = try!(TrustedSignersDeserializer::deserialize(
                            "TrustedSigners",
                            stack
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct StreamingDistributionSummaryListDeserializer;
impl StreamingDistributionSummaryListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StreamingDistributionSummary>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "StreamingDistributionSummary" {
                        obj.push(try!(StreamingDistributionSummaryDeserializer::deserialize(
                            "StreamingDistributionSummary",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>A complex type that controls whether access logs are written for this streaming distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StreamingLoggingConfig, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StreamingLoggingConfig::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Bucket" => {
                        obj.bucket = try!(StringDeserializer::deserialize("Bucket", stack));
                    }
                    "Enabled" => {
                        obj.enabled = try!(BooleanDeserializer::deserialize("Enabled", stack));
                    }
                    "Prefix" => {
                        obj.prefix = try!(StringDeserializer::deserialize("Prefix", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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
pub struct Tag {
    /// <p> A string that contains <code>Tag</code> key.</p> <p>The string length should be between 1 and 128 characters. Valid characters include <code>a-z</code>, <code>A-Z</code>, <code>0-9</code>, space, and the special characters <code>_ - . : / = + @</code>.</p>
    pub key: String,
    /// <p> A string that contains an optional <code>Tag</code> value.</p> <p>The string length should be between 0 and 256 characters. Valid characters include <code>a-z</code>, <code>A-Z</code>, <code>0-9</code>, space, and the special characters <code>_ - . : / = + @</code>.</p>
    pub value: Option<String>,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Tag, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Tag::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Key" => {
                        obj.key = try!(TagKeyDeserializer::deserialize("Key", stack));
                    }
                    "Value" => {
                        obj.value = Some(try!(TagValueDeserializer::deserialize("Value", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Tag" {
                        obj.push(try!(TagDeserializer::deserialize("Tag", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
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
pub struct TagResourceRequest {
    /// <p> An ARN of a CloudFront resource.</p>
    pub resource: String,
    /// <p> A complex type that contains zero or more <code>Tag</code> elements.</p>
    pub tags: Tags,
}

struct TagValueDeserializer;
impl TagValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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
pub struct Tags {
    /// <p> A complex type that contains <code>Tag</code> elements.</p>
    pub items: Option<Vec<Tag>>,
}

struct TagsDeserializer;
impl TagsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Tags, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Tags::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Items" => {
                        obj.items = Some(try!(TagListDeserializer::deserialize("Items", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A complex type that specifies the AWS accounts, if any, that you want to allow to create signed URLs for private content.</p> <p>If you want to require signed URLs in requests for objects in the target origin that match the <code>PathPattern</code> for this cache behavior, specify <code>true</code> for <code>Enabled</code>, and specify the applicable values for <code>Quantity</code> and <code>Items</code>. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon Amazon CloudFront Developer Guide</i>.</p> <p>If you don't want to require signed URLs in requests for objects that match <code>PathPattern</code>, specify <code>false</code> for <code>Enabled</code> and <code>0</code> for <code>Quantity</code>. Omit <code>Items</code>.</p> <p>To add, change, or remove one or more trusted signers, change <code>Enabled</code> to <code>true</code> (if it's currently <code>false</code>), change <code>Quantity</code> as applicable, and specify all of the trusted signers that you want to include in the updated distribution.</p> <p>For more information about updating the distribution configuration, see <a>DistributionConfig</a> .</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TrustedSigners, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TrustedSigners::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Enabled" => {
                        obj.enabled = try!(BooleanDeserializer::deserialize("Enabled", stack));
                    }
                    "Items" => {
                        obj.items = Some(try!(AwsAccountNumberListDeserializer::deserialize(
                            "Items", stack
                        )));
                    }
                    "Quantity" => {
                        obj.quantity = try!(IntegerDeserializer::deserialize("Quantity", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
pub struct UntagResourceRequest {
    /// <p> An ARN of a CloudFront resource.</p>
    pub resource: String,
    /// <p> A complex type that contains zero or more <code>Tag</code> key elements.</p>
    pub tag_keys: TagKeys,
}

/// <p>The request to update an origin access identity.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct UpdateCloudFrontOriginAccessIdentityResult {
    /// <p>The origin access identity's information.</p>
    pub cloud_front_origin_access_identity: Option<CloudFrontOriginAccessIdentity>,
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

struct UpdateCloudFrontOriginAccessIdentityResultDeserializer;
impl UpdateCloudFrontOriginAccessIdentityResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateCloudFrontOriginAccessIdentityResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateCloudFrontOriginAccessIdentityResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CloudFrontOriginAccessIdentity" => {
                        obj.cloud_front_origin_access_identity = Some(try!(
                            CloudFrontOriginAccessIdentityDeserializer::deserialize(
                                "CloudFrontOriginAccessIdentity",
                                stack
                            )
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to update a distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct UpdateDistributionResult {
    /// <p>The distribution's information.</p>
    pub distribution: Option<Distribution>,
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
}

struct UpdateDistributionResultDeserializer;
impl UpdateDistributionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateDistributionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateDistributionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Distribution" => {
                        obj.distribution = Some(try!(DistributionDeserializer::deserialize(
                            "Distribution",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The request to update a streaming distribution.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
pub struct UpdateStreamingDistributionResult {
    /// <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub e_tag: Option<String>,
    /// <p>The streaming distribution's information.</p>
    pub streaming_distribution: Option<StreamingDistribution>,
}

struct UpdateStreamingDistributionResultDeserializer;
impl UpdateStreamingDistributionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateStreamingDistributionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateStreamingDistributionResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "StreamingDistribution" => {
                        obj.streaming_distribution =
                            Some(try!(StreamingDistributionDeserializer::deserialize(
                                "StreamingDistribution",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A complex type that specifies the following:</p> <ul> <li> <p>Whether you want viewers to use HTTP or HTTPS to request your objects.</p> </li> <li> <p>If you want viewers to use HTTPS, whether you're using an alternate domain name such as <code>example.com</code> or the CloudFront domain name for your distribution, such as <code>d111111abcdef8.cloudfront.net</code>.</p> </li> <li> <p>If you're using an alternate domain name, whether AWS Certificate Manager (ACM) provided the certificate, or you purchased a certificate from a third-party certificate authority and imported it into ACM or uploaded it to the IAM certificate store.</p> </li> </ul> <p>You must specify only one of the following values: </p> <ul> <li> <p> <a>ViewerCertificate$ACMCertificateArn</a> </p> </li> <li> <p> <a>ViewerCertificate$IAMCertificateId</a> </p> </li> <li> <p> <a>ViewerCertificate$CloudFrontDefaultCertificate</a> </p> </li> </ul> <p>Don't specify <code>false</code> for <code>CloudFrontDefaultCertificate</code>.</p> <p> <b>If you want viewers to use HTTP instead of HTTPS to request your objects</b>: Specify the following value:</p> <p> <code>&lt;CloudFrontDefaultCertificate&gt;true&lt;CloudFrontDefaultCertificate&gt;</code> </p> <p>In addition, specify <code>allow-all</code> for <code>ViewerProtocolPolicy</code> for all of your cache behaviors.</p> <p> <b>If you want viewers to use HTTPS to request your objects</b>: Choose the type of certificate that you want to use based on whether you're using an alternate domain name for your objects or the CloudFront domain name:</p> <ul> <li> <p> <b>If you're using an alternate domain name, such as example.com</b>: Specify one of the following values, depending on whether ACM provided your certificate or you purchased your certificate from third-party certificate authority:</p> <ul> <li> <p> <code>&lt;ACMCertificateArn&gt;<i>ARN for ACM SSL/TLS certificate</i>&lt;ACMCertificateArn&gt;</code> where <code> <i>ARN for ACM SSL/TLS certificate</i> </code> is the ARN for the ACM SSL/TLS certificate that you want to use for this distribution.</p> </li> <li> <p> <code>&lt;IAMCertificateId&gt;<i>IAM certificate ID</i>&lt;IAMCertificateId&gt;</code> where <code> <i>IAM certificate ID</i> </code> is the ID that IAM returned when you added the certificate to the IAM certificate store.</p> </li> </ul> <p>If you specify <code>ACMCertificateArn</code> or <code>IAMCertificateId</code>, you must also specify a value for <code>SSLSupportMethod</code>.</p> <p>If you choose to use an ACM certificate or a certificate in the IAM certificate store, we recommend that you use only an alternate domain name in your object URLs (<code>https://example.com/logo.jpg</code>). If you use the domain name that is associated with your CloudFront distribution (such as <code>https://d111111abcdef8.cloudfront.net/logo.jpg</code>) and the viewer supports <code>SNI</code>, then CloudFront behaves normally. However, if the browser does not support SNI, the user's experience depends on the value that you choose for <code>SSLSupportMethod</code>:</p> <ul> <li> <p> <code>vip</code>: The viewer displays a warning because there is a mismatch between the CloudFront domain name and the domain name in your SSL/TLS certificate.</p> </li> <li> <p> <code>sni-only</code>: CloudFront drops the connection with the browser without returning the object.</p> </li> </ul> </li> <li> <p> <b>If you're using the CloudFront domain name for your distribution, such as <code>d111111abcdef8.cloudfront.net</code> </b>: Specify the following value:</p> <p> <code>&lt;CloudFrontDefaultCertificate&gt;true&lt;CloudFrontDefaultCertificate&gt; </code> </p> </li> </ul> <p>If you want viewers to use HTTPS, you must also specify one of the following values in your cache behaviors:</p> <ul> <li> <p> <code> &lt;ViewerProtocolPolicy&gt;https-only&lt;ViewerProtocolPolicy&gt;</code> </p> </li> <li> <p> <code>&lt;ViewerProtocolPolicy&gt;redirect-to-https&lt;ViewerProtocolPolicy&gt;</code> </p> </li> </ul> <p>You can also optionally require that CloudFront use HTTPS to communicate with your origin by specifying one of the following values for the applicable origins:</p> <ul> <li> <p> <code>&lt;OriginProtocolPolicy&gt;https-only&lt;OriginProtocolPolicy&gt; </code> </p> </li> <li> <p> <code>&lt;OriginProtocolPolicy&gt;match-viewer&lt;OriginProtocolPolicy&gt; </code> </p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/SecureConnections.html#CNAMEsAndHTTPS">Using Alternate Domain Names and HTTPS</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ViewerCertificate, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ViewerCertificate::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ACMCertificateArn" => {
                        obj.acm_certificate_arn = Some(try!(StringDeserializer::deserialize(
                            "ACMCertificateArn",
                            stack
                        )));
                    }
                    "CloudFrontDefaultCertificate" => {
                        obj.cloud_front_default_certificate = Some(try!(
                            BooleanDeserializer::deserialize("CloudFrontDefaultCertificate", stack)
                        ));
                    }
                    "IAMCertificateId" => {
                        obj.iam_certificate_id = Some(try!(StringDeserializer::deserialize(
                            "IAMCertificateId",
                            stack
                        )));
                    }
                    "MinimumProtocolVersion" => {
                        obj.minimum_protocol_version =
                            Some(try!(MinimumProtocolVersionDeserializer::deserialize(
                                "MinimumProtocolVersion",
                                stack
                            )));
                    }
                    "SSLSupportMethod" => {
                        obj.ssl_support_method = Some(try!(
                            SSLSupportMethodDeserializer::deserialize("SSLSupportMethod", stack)
                        ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
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
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateCloudFrontOriginAccessIdentityError {
    pub fn from_body(body: &str) -> CreateCloudFrontOriginAccessIdentityError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "CloudFrontOriginAccessIdentityAlreadyExists" => CreateCloudFrontOriginAccessIdentityError::CloudFrontOriginAccessIdentityAlreadyExists(String::from(parsed_error.message)),"InconsistentQuantities" => CreateCloudFrontOriginAccessIdentityError::InconsistentQuantities(String::from(parsed_error.message)),"InvalidArgument" => CreateCloudFrontOriginAccessIdentityError::InvalidArgument(String::from(parsed_error.message)),"MissingBody" => CreateCloudFrontOriginAccessIdentityError::MissingBody(String::from(parsed_error.message)),"TooManyCloudFrontOriginAccessIdentities" => CreateCloudFrontOriginAccessIdentityError::TooManyCloudFrontOriginAccessIdentities(String::from(parsed_error.message)),_ => CreateCloudFrontOriginAccessIdentityError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => CreateCloudFrontOriginAccessIdentityError::Unknown(body.to_string())
                       }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateCloudFrontOriginAccessIdentityError {
    fn from(err: XmlParseError) -> CreateCloudFrontOriginAccessIdentityError {
        let XmlParseError(message) = err;
        CreateCloudFrontOriginAccessIdentityError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateCloudFrontOriginAccessIdentityError {
    fn from(err: CredentialsError) -> CreateCloudFrontOriginAccessIdentityError {
        CreateCloudFrontOriginAccessIdentityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCloudFrontOriginAccessIdentityError {
    fn from(err: HttpDispatchError) -> CreateCloudFrontOriginAccessIdentityError {
        CreateCloudFrontOriginAccessIdentityError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCloudFrontOriginAccessIdentityError {
    fn from(err: io::Error) -> CreateCloudFrontOriginAccessIdentityError {
        CreateCloudFrontOriginAccessIdentityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCloudFrontOriginAccessIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCloudFrontOriginAccessIdentityError {
    fn description(&self) -> &str {
        match *self {
                            CreateCloudFrontOriginAccessIdentityError::CloudFrontOriginAccessIdentityAlreadyExists(ref cause) => cause,
CreateCloudFrontOriginAccessIdentityError::InconsistentQuantities(ref cause) => cause,
CreateCloudFrontOriginAccessIdentityError::InvalidArgument(ref cause) => cause,
CreateCloudFrontOriginAccessIdentityError::MissingBody(ref cause) => cause,
CreateCloudFrontOriginAccessIdentityError::TooManyCloudFrontOriginAccessIdentities(ref cause) => cause,
CreateCloudFrontOriginAccessIdentityError::Validation(ref cause) => cause,
CreateCloudFrontOriginAccessIdentityError::Credentials(ref err) => err.description(),
CreateCloudFrontOriginAccessIdentityError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
CreateCloudFrontOriginAccessIdentityError::Unknown(ref cause) => cause
                        }
    }
}
/// Errors returned by CreateDistribution
#[derive(Debug, PartialEq)]
pub enum CreateDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),

    CNAMEAlreadyExists(String),
    /// <p>The caller reference you attempted to create the distribution with is associated with another distribution.</p>
    DistributionAlreadyExists(String),
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
    /// <p>Processing your request would cause the maximum number of distributions with Lambda function associations per owner to be exceeded.</p>
    TooManyDistributionsWithLambdaAssociations(String),

    TooManyHeadersInForwardedValues(String),
    /// <p>Your request contains more Lambda function associations than are allowed per distribution.</p>
    TooManyLambdaFunctionAssociations(String),

    TooManyOriginCustomHeaders(String),
    /// <p>You cannot create more origins for the distribution.</p>
    TooManyOrigins(String),

    TooManyQueryStringParameters(String),
    /// <p>Your request contains more trusted signers than are allowed per distribution.</p>
    TooManyTrustedSigners(String),
    /// <p>One or more of your trusted signers don't exist.</p>
    TrustedSignerDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDistributionError {
    pub fn from_body(body: &str) -> CreateDistributionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => {
                    CreateDistributionError::AccessDenied(String::from(parsed_error.message))
                }
                "CNAMEAlreadyExists" => {
                    CreateDistributionError::CNAMEAlreadyExists(String::from(parsed_error.message))
                }
                "DistributionAlreadyExists" => CreateDistributionError::DistributionAlreadyExists(
                    String::from(parsed_error.message),
                ),
                "InconsistentQuantities" => CreateDistributionError::InconsistentQuantities(
                    String::from(parsed_error.message),
                ),
                "InvalidArgument" => {
                    CreateDistributionError::InvalidArgument(String::from(parsed_error.message))
                }
                "InvalidDefaultRootObject" => CreateDistributionError::InvalidDefaultRootObject(
                    String::from(parsed_error.message),
                ),
                "InvalidErrorCode" => {
                    CreateDistributionError::InvalidErrorCode(String::from(parsed_error.message))
                }
                "InvalidForwardCookies" => CreateDistributionError::InvalidForwardCookies(
                    String::from(parsed_error.message),
                ),
                "InvalidGeoRestrictionParameter" => {
                    CreateDistributionError::InvalidGeoRestrictionParameter(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidHeadersForS3Origin" => CreateDistributionError::InvalidHeadersForS3Origin(
                    String::from(parsed_error.message),
                ),
                "InvalidLambdaFunctionAssociation" => {
                    CreateDistributionError::InvalidLambdaFunctionAssociation(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidLocationCode" => {
                    CreateDistributionError::InvalidLocationCode(String::from(parsed_error.message))
                }
                "InvalidMinimumProtocolVersion" => {
                    CreateDistributionError::InvalidMinimumProtocolVersion(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidOrigin" => {
                    CreateDistributionError::InvalidOrigin(String::from(parsed_error.message))
                }
                "InvalidOriginAccessIdentity" => {
                    CreateDistributionError::InvalidOriginAccessIdentity(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidOriginKeepaliveTimeout" => {
                    CreateDistributionError::InvalidOriginKeepaliveTimeout(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidOriginReadTimeout" => CreateDistributionError::InvalidOriginReadTimeout(
                    String::from(parsed_error.message),
                ),
                "InvalidProtocolSettings" => CreateDistributionError::InvalidProtocolSettings(
                    String::from(parsed_error.message),
                ),
                "InvalidQueryStringParameters" => {
                    CreateDistributionError::InvalidQueryStringParameters(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidRelativePath" => {
                    CreateDistributionError::InvalidRelativePath(String::from(parsed_error.message))
                }
                "InvalidRequiredProtocol" => CreateDistributionError::InvalidRequiredProtocol(
                    String::from(parsed_error.message),
                ),
                "InvalidResponseCode" => {
                    CreateDistributionError::InvalidResponseCode(String::from(parsed_error.message))
                }
                "InvalidTTLOrder" => {
                    CreateDistributionError::InvalidTTLOrder(String::from(parsed_error.message))
                }
                "InvalidViewerCertificate" => CreateDistributionError::InvalidViewerCertificate(
                    String::from(parsed_error.message),
                ),
                "InvalidWebACLId" => {
                    CreateDistributionError::InvalidWebACLId(String::from(parsed_error.message))
                }
                "MissingBody" => {
                    CreateDistributionError::MissingBody(String::from(parsed_error.message))
                }
                "NoSuchOrigin" => {
                    CreateDistributionError::NoSuchOrigin(String::from(parsed_error.message))
                }
                "TooManyCacheBehaviors" => CreateDistributionError::TooManyCacheBehaviors(
                    String::from(parsed_error.message),
                ),
                "TooManyCertificates" => {
                    CreateDistributionError::TooManyCertificates(String::from(parsed_error.message))
                }
                "TooManyCookieNamesInWhiteList" => {
                    CreateDistributionError::TooManyCookieNamesInWhiteList(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyDistributionCNAMEs" => CreateDistributionError::TooManyDistributionCNAMEs(
                    String::from(parsed_error.message),
                ),
                "TooManyDistributions" => CreateDistributionError::TooManyDistributions(
                    String::from(parsed_error.message),
                ),
                "TooManyDistributionsWithLambdaAssociations" => {
                    CreateDistributionError::TooManyDistributionsWithLambdaAssociations(
                        String::from(parsed_error.message),
                    )
                }
                "TooManyHeadersInForwardedValues" => {
                    CreateDistributionError::TooManyHeadersInForwardedValues(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyLambdaFunctionAssociations" => {
                    CreateDistributionError::TooManyLambdaFunctionAssociations(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyOriginCustomHeaders" => {
                    CreateDistributionError::TooManyOriginCustomHeaders(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyOrigins" => {
                    CreateDistributionError::TooManyOrigins(String::from(parsed_error.message))
                }
                "TooManyQueryStringParameters" => {
                    CreateDistributionError::TooManyQueryStringParameters(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyTrustedSigners" => CreateDistributionError::TooManyTrustedSigners(
                    String::from(parsed_error.message),
                ),
                "TrustedSignerDoesNotExist" => CreateDistributionError::TrustedSignerDoesNotExist(
                    String::from(parsed_error.message),
                ),
                _ => CreateDistributionError::Unknown(String::from(body)),
            },
            Err(_) => CreateDistributionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateDistributionError {
    fn from(err: XmlParseError) -> CreateDistributionError {
        let XmlParseError(message) = err;
        CreateDistributionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateDistributionError {
    fn from(err: CredentialsError) -> CreateDistributionError {
        CreateDistributionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDistributionError {
    fn from(err: HttpDispatchError) -> CreateDistributionError {
        CreateDistributionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDistributionError {
    fn from(err: io::Error) -> CreateDistributionError {
        CreateDistributionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDistributionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDistributionError {
    fn description(&self) -> &str {
        match *self {
            CreateDistributionError::AccessDenied(ref cause) => cause,
            CreateDistributionError::CNAMEAlreadyExists(ref cause) => cause,
            CreateDistributionError::DistributionAlreadyExists(ref cause) => cause,
            CreateDistributionError::InconsistentQuantities(ref cause) => cause,
            CreateDistributionError::InvalidArgument(ref cause) => cause,
            CreateDistributionError::InvalidDefaultRootObject(ref cause) => cause,
            CreateDistributionError::InvalidErrorCode(ref cause) => cause,
            CreateDistributionError::InvalidForwardCookies(ref cause) => cause,
            CreateDistributionError::InvalidGeoRestrictionParameter(ref cause) => cause,
            CreateDistributionError::InvalidHeadersForS3Origin(ref cause) => cause,
            CreateDistributionError::InvalidLambdaFunctionAssociation(ref cause) => cause,
            CreateDistributionError::InvalidLocationCode(ref cause) => cause,
            CreateDistributionError::InvalidMinimumProtocolVersion(ref cause) => cause,
            CreateDistributionError::InvalidOrigin(ref cause) => cause,
            CreateDistributionError::InvalidOriginAccessIdentity(ref cause) => cause,
            CreateDistributionError::InvalidOriginKeepaliveTimeout(ref cause) => cause,
            CreateDistributionError::InvalidOriginReadTimeout(ref cause) => cause,
            CreateDistributionError::InvalidProtocolSettings(ref cause) => cause,
            CreateDistributionError::InvalidQueryStringParameters(ref cause) => cause,
            CreateDistributionError::InvalidRelativePath(ref cause) => cause,
            CreateDistributionError::InvalidRequiredProtocol(ref cause) => cause,
            CreateDistributionError::InvalidResponseCode(ref cause) => cause,
            CreateDistributionError::InvalidTTLOrder(ref cause) => cause,
            CreateDistributionError::InvalidViewerCertificate(ref cause) => cause,
            CreateDistributionError::InvalidWebACLId(ref cause) => cause,
            CreateDistributionError::MissingBody(ref cause) => cause,
            CreateDistributionError::NoSuchOrigin(ref cause) => cause,
            CreateDistributionError::TooManyCacheBehaviors(ref cause) => cause,
            CreateDistributionError::TooManyCertificates(ref cause) => cause,
            CreateDistributionError::TooManyCookieNamesInWhiteList(ref cause) => cause,
            CreateDistributionError::TooManyDistributionCNAMEs(ref cause) => cause,
            CreateDistributionError::TooManyDistributions(ref cause) => cause,
            CreateDistributionError::TooManyDistributionsWithLambdaAssociations(ref cause) => cause,
            CreateDistributionError::TooManyHeadersInForwardedValues(ref cause) => cause,
            CreateDistributionError::TooManyLambdaFunctionAssociations(ref cause) => cause,
            CreateDistributionError::TooManyOriginCustomHeaders(ref cause) => cause,
            CreateDistributionError::TooManyOrigins(ref cause) => cause,
            CreateDistributionError::TooManyQueryStringParameters(ref cause) => cause,
            CreateDistributionError::TooManyTrustedSigners(ref cause) => cause,
            CreateDistributionError::TrustedSignerDoesNotExist(ref cause) => cause,
            CreateDistributionError::Validation(ref cause) => cause,
            CreateDistributionError::Credentials(ref err) => err.description(),
            CreateDistributionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDistributionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDistributionWithTags
#[derive(Debug, PartialEq)]
pub enum CreateDistributionWithTagsError {
    /// <p>Access denied.</p>
    AccessDenied(String),

    CNAMEAlreadyExists(String),
    /// <p>The caller reference you attempted to create the distribution with is associated with another distribution.</p>
    DistributionAlreadyExists(String),
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
    /// <p>Processing your request would cause the maximum number of distributions with Lambda function associations per owner to be exceeded.</p>
    TooManyDistributionsWithLambdaAssociations(String),

    TooManyHeadersInForwardedValues(String),
    /// <p>Your request contains more Lambda function associations than are allowed per distribution.</p>
    TooManyLambdaFunctionAssociations(String),

    TooManyOriginCustomHeaders(String),
    /// <p>You cannot create more origins for the distribution.</p>
    TooManyOrigins(String),

    TooManyQueryStringParameters(String),
    /// <p>Your request contains more trusted signers than are allowed per distribution.</p>
    TooManyTrustedSigners(String),
    /// <p>One or more of your trusted signers don't exist.</p>
    TrustedSignerDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDistributionWithTagsError {
    pub fn from_body(body: &str) -> CreateDistributionWithTagsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => CreateDistributionWithTagsError::AccessDenied(String::from(
                    parsed_error.message,
                )),
                "CNAMEAlreadyExists" => CreateDistributionWithTagsError::CNAMEAlreadyExists(
                    String::from(parsed_error.message),
                ),
                "DistributionAlreadyExists" => {
                    CreateDistributionWithTagsError::DistributionAlreadyExists(String::from(
                        parsed_error.message,
                    ))
                }
                "InconsistentQuantities" => {
                    CreateDistributionWithTagsError::InconsistentQuantities(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidArgument" => CreateDistributionWithTagsError::InvalidArgument(
                    String::from(parsed_error.message),
                ),
                "InvalidDefaultRootObject" => {
                    CreateDistributionWithTagsError::InvalidDefaultRootObject(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidErrorCode" => CreateDistributionWithTagsError::InvalidErrorCode(
                    String::from(parsed_error.message),
                ),
                "InvalidForwardCookies" => CreateDistributionWithTagsError::InvalidForwardCookies(
                    String::from(parsed_error.message),
                ),
                "InvalidGeoRestrictionParameter" => {
                    CreateDistributionWithTagsError::InvalidGeoRestrictionParameter(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidHeadersForS3Origin" => {
                    CreateDistributionWithTagsError::InvalidHeadersForS3Origin(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidLambdaFunctionAssociation" => {
                    CreateDistributionWithTagsError::InvalidLambdaFunctionAssociation(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidLocationCode" => CreateDistributionWithTagsError::InvalidLocationCode(
                    String::from(parsed_error.message),
                ),
                "InvalidMinimumProtocolVersion" => {
                    CreateDistributionWithTagsError::InvalidMinimumProtocolVersion(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidOrigin" => CreateDistributionWithTagsError::InvalidOrigin(String::from(
                    parsed_error.message,
                )),
                "InvalidOriginAccessIdentity" => {
                    CreateDistributionWithTagsError::InvalidOriginAccessIdentity(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidOriginKeepaliveTimeout" => {
                    CreateDistributionWithTagsError::InvalidOriginKeepaliveTimeout(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidOriginReadTimeout" => {
                    CreateDistributionWithTagsError::InvalidOriginReadTimeout(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidProtocolSettings" => {
                    CreateDistributionWithTagsError::InvalidProtocolSettings(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidQueryStringParameters" => {
                    CreateDistributionWithTagsError::InvalidQueryStringParameters(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidRelativePath" => CreateDistributionWithTagsError::InvalidRelativePath(
                    String::from(parsed_error.message),
                ),
                "InvalidRequiredProtocol" => {
                    CreateDistributionWithTagsError::InvalidRequiredProtocol(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidResponseCode" => CreateDistributionWithTagsError::InvalidResponseCode(
                    String::from(parsed_error.message),
                ),
                "InvalidTTLOrder" => CreateDistributionWithTagsError::InvalidTTLOrder(
                    String::from(parsed_error.message),
                ),
                "InvalidTagging" => CreateDistributionWithTagsError::InvalidTagging(String::from(
                    parsed_error.message,
                )),
                "InvalidViewerCertificate" => {
                    CreateDistributionWithTagsError::InvalidViewerCertificate(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidWebACLId" => CreateDistributionWithTagsError::InvalidWebACLId(
                    String::from(parsed_error.message),
                ),
                "MissingBody" => {
                    CreateDistributionWithTagsError::MissingBody(String::from(parsed_error.message))
                }
                "NoSuchOrigin" => CreateDistributionWithTagsError::NoSuchOrigin(String::from(
                    parsed_error.message,
                )),
                "TooManyCacheBehaviors" => CreateDistributionWithTagsError::TooManyCacheBehaviors(
                    String::from(parsed_error.message),
                ),
                "TooManyCertificates" => CreateDistributionWithTagsError::TooManyCertificates(
                    String::from(parsed_error.message),
                ),
                "TooManyCookieNamesInWhiteList" => {
                    CreateDistributionWithTagsError::TooManyCookieNamesInWhiteList(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyDistributionCNAMEs" => {
                    CreateDistributionWithTagsError::TooManyDistributionCNAMEs(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyDistributions" => CreateDistributionWithTagsError::TooManyDistributions(
                    String::from(parsed_error.message),
                ),
                "TooManyDistributionsWithLambdaAssociations" => {
                    CreateDistributionWithTagsError::TooManyDistributionsWithLambdaAssociations(
                        String::from(parsed_error.message),
                    )
                }
                "TooManyHeadersInForwardedValues" => {
                    CreateDistributionWithTagsError::TooManyHeadersInForwardedValues(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyLambdaFunctionAssociations" => {
                    CreateDistributionWithTagsError::TooManyLambdaFunctionAssociations(
                        String::from(parsed_error.message),
                    )
                }
                "TooManyOriginCustomHeaders" => {
                    CreateDistributionWithTagsError::TooManyOriginCustomHeaders(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyOrigins" => CreateDistributionWithTagsError::TooManyOrigins(String::from(
                    parsed_error.message,
                )),
                "TooManyQueryStringParameters" => {
                    CreateDistributionWithTagsError::TooManyQueryStringParameters(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyTrustedSigners" => CreateDistributionWithTagsError::TooManyTrustedSigners(
                    String::from(parsed_error.message),
                ),
                "TrustedSignerDoesNotExist" => {
                    CreateDistributionWithTagsError::TrustedSignerDoesNotExist(String::from(
                        parsed_error.message,
                    ))
                }
                _ => CreateDistributionWithTagsError::Unknown(String::from(body)),
            },
            Err(_) => CreateDistributionWithTagsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateDistributionWithTagsError {
    fn from(err: XmlParseError) -> CreateDistributionWithTagsError {
        let XmlParseError(message) = err;
        CreateDistributionWithTagsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateDistributionWithTagsError {
    fn from(err: CredentialsError) -> CreateDistributionWithTagsError {
        CreateDistributionWithTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDistributionWithTagsError {
    fn from(err: HttpDispatchError) -> CreateDistributionWithTagsError {
        CreateDistributionWithTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDistributionWithTagsError {
    fn from(err: io::Error) -> CreateDistributionWithTagsError {
        CreateDistributionWithTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDistributionWithTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDistributionWithTagsError {
    fn description(&self) -> &str {
        match *self {
            CreateDistributionWithTagsError::AccessDenied(ref cause) => cause,
            CreateDistributionWithTagsError::CNAMEAlreadyExists(ref cause) => cause,
            CreateDistributionWithTagsError::DistributionAlreadyExists(ref cause) => cause,
            CreateDistributionWithTagsError::InconsistentQuantities(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidArgument(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidDefaultRootObject(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidErrorCode(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidForwardCookies(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidGeoRestrictionParameter(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidHeadersForS3Origin(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidLambdaFunctionAssociation(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidLocationCode(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidMinimumProtocolVersion(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidOrigin(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidOriginAccessIdentity(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidOriginKeepaliveTimeout(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidOriginReadTimeout(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidProtocolSettings(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidQueryStringParameters(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidRelativePath(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidRequiredProtocol(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidResponseCode(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidTTLOrder(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidTagging(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidViewerCertificate(ref cause) => cause,
            CreateDistributionWithTagsError::InvalidWebACLId(ref cause) => cause,
            CreateDistributionWithTagsError::MissingBody(ref cause) => cause,
            CreateDistributionWithTagsError::NoSuchOrigin(ref cause) => cause,
            CreateDistributionWithTagsError::TooManyCacheBehaviors(ref cause) => cause,
            CreateDistributionWithTagsError::TooManyCertificates(ref cause) => cause,
            CreateDistributionWithTagsError::TooManyCookieNamesInWhiteList(ref cause) => cause,
            CreateDistributionWithTagsError::TooManyDistributionCNAMEs(ref cause) => cause,
            CreateDistributionWithTagsError::TooManyDistributions(ref cause) => cause,
            CreateDistributionWithTagsError::TooManyDistributionsWithLambdaAssociations(
                ref cause,
            ) => cause,
            CreateDistributionWithTagsError::TooManyHeadersInForwardedValues(ref cause) => cause,
            CreateDistributionWithTagsError::TooManyLambdaFunctionAssociations(ref cause) => cause,
            CreateDistributionWithTagsError::TooManyOriginCustomHeaders(ref cause) => cause,
            CreateDistributionWithTagsError::TooManyOrigins(ref cause) => cause,
            CreateDistributionWithTagsError::TooManyQueryStringParameters(ref cause) => cause,
            CreateDistributionWithTagsError::TooManyTrustedSigners(ref cause) => cause,
            CreateDistributionWithTagsError::TrustedSignerDoesNotExist(ref cause) => cause,
            CreateDistributionWithTagsError::Validation(ref cause) => cause,
            CreateDistributionWithTagsError::Credentials(ref err) => err.description(),
            CreateDistributionWithTagsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDistributionWithTagsError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateInvalidationError {
    pub fn from_body(body: &str) -> CreateInvalidationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => {
                    CreateInvalidationError::AccessDenied(String::from(parsed_error.message))
                }
                "BatchTooLarge" => {
                    CreateInvalidationError::BatchTooLarge(String::from(parsed_error.message))
                }
                "InconsistentQuantities" => CreateInvalidationError::InconsistentQuantities(
                    String::from(parsed_error.message),
                ),
                "InvalidArgument" => {
                    CreateInvalidationError::InvalidArgument(String::from(parsed_error.message))
                }
                "MissingBody" => {
                    CreateInvalidationError::MissingBody(String::from(parsed_error.message))
                }
                "NoSuchDistribution" => {
                    CreateInvalidationError::NoSuchDistribution(String::from(parsed_error.message))
                }
                "TooManyInvalidationsInProgress" => {
                    CreateInvalidationError::TooManyInvalidationsInProgress(String::from(
                        parsed_error.message,
                    ))
                }
                _ => CreateInvalidationError::Unknown(String::from(body)),
            },
            Err(_) => CreateInvalidationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateInvalidationError {
    fn from(err: XmlParseError) -> CreateInvalidationError {
        let XmlParseError(message) = err;
        CreateInvalidationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateInvalidationError {
    fn from(err: CredentialsError) -> CreateInvalidationError {
        CreateInvalidationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateInvalidationError {
    fn from(err: HttpDispatchError) -> CreateInvalidationError {
        CreateInvalidationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateInvalidationError {
    fn from(err: io::Error) -> CreateInvalidationError {
        CreateInvalidationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateInvalidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateInvalidationError {
    fn description(&self) -> &str {
        match *self {
            CreateInvalidationError::AccessDenied(ref cause) => cause,
            CreateInvalidationError::BatchTooLarge(ref cause) => cause,
            CreateInvalidationError::InconsistentQuantities(ref cause) => cause,
            CreateInvalidationError::InvalidArgument(ref cause) => cause,
            CreateInvalidationError::MissingBody(ref cause) => cause,
            CreateInvalidationError::NoSuchDistribution(ref cause) => cause,
            CreateInvalidationError::TooManyInvalidationsInProgress(ref cause) => cause,
            CreateInvalidationError::Validation(ref cause) => cause,
            CreateInvalidationError::Credentials(ref err) => err.description(),
            CreateInvalidationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateInvalidationError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateStreamingDistributionError {
    pub fn from_body(body: &str) -> CreateStreamingDistributionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => CreateStreamingDistributionError::AccessDenied(String::from(
                    parsed_error.message,
                )),
                "CNAMEAlreadyExists" => CreateStreamingDistributionError::CNAMEAlreadyExists(
                    String::from(parsed_error.message),
                ),
                "InconsistentQuantities" => {
                    CreateStreamingDistributionError::InconsistentQuantities(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidArgument" => CreateStreamingDistributionError::InvalidArgument(
                    String::from(parsed_error.message),
                ),
                "InvalidOrigin" => CreateStreamingDistributionError::InvalidOrigin(String::from(
                    parsed_error.message,
                )),
                "InvalidOriginAccessIdentity" => {
                    CreateStreamingDistributionError::InvalidOriginAccessIdentity(String::from(
                        parsed_error.message,
                    ))
                }
                "MissingBody" => CreateStreamingDistributionError::MissingBody(String::from(
                    parsed_error.message,
                )),
                "StreamingDistributionAlreadyExists" => {
                    CreateStreamingDistributionError::StreamingDistributionAlreadyExists(
                        String::from(parsed_error.message),
                    )
                }
                "TooManyStreamingDistributionCNAMEs" => {
                    CreateStreamingDistributionError::TooManyStreamingDistributionCNAMEs(
                        String::from(parsed_error.message),
                    )
                }
                "TooManyStreamingDistributions" => {
                    CreateStreamingDistributionError::TooManyStreamingDistributions(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyTrustedSigners" => CreateStreamingDistributionError::TooManyTrustedSigners(
                    String::from(parsed_error.message),
                ),
                "TrustedSignerDoesNotExist" => {
                    CreateStreamingDistributionError::TrustedSignerDoesNotExist(String::from(
                        parsed_error.message,
                    ))
                }
                _ => CreateStreamingDistributionError::Unknown(String::from(body)),
            },
            Err(_) => CreateStreamingDistributionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateStreamingDistributionError {
    fn from(err: XmlParseError) -> CreateStreamingDistributionError {
        let XmlParseError(message) = err;
        CreateStreamingDistributionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateStreamingDistributionError {
    fn from(err: CredentialsError) -> CreateStreamingDistributionError {
        CreateStreamingDistributionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStreamingDistributionError {
    fn from(err: HttpDispatchError) -> CreateStreamingDistributionError {
        CreateStreamingDistributionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStreamingDistributionError {
    fn from(err: io::Error) -> CreateStreamingDistributionError {
        CreateStreamingDistributionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStreamingDistributionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStreamingDistributionError {
    fn description(&self) -> &str {
        match *self {
            CreateStreamingDistributionError::AccessDenied(ref cause) => cause,
            CreateStreamingDistributionError::CNAMEAlreadyExists(ref cause) => cause,
            CreateStreamingDistributionError::InconsistentQuantities(ref cause) => cause,
            CreateStreamingDistributionError::InvalidArgument(ref cause) => cause,
            CreateStreamingDistributionError::InvalidOrigin(ref cause) => cause,
            CreateStreamingDistributionError::InvalidOriginAccessIdentity(ref cause) => cause,
            CreateStreamingDistributionError::MissingBody(ref cause) => cause,
            CreateStreamingDistributionError::StreamingDistributionAlreadyExists(ref cause) => {
                cause
            }
            CreateStreamingDistributionError::TooManyStreamingDistributionCNAMEs(ref cause) => {
                cause
            }
            CreateStreamingDistributionError::TooManyStreamingDistributions(ref cause) => cause,
            CreateStreamingDistributionError::TooManyTrustedSigners(ref cause) => cause,
            CreateStreamingDistributionError::TrustedSignerDoesNotExist(ref cause) => cause,
            CreateStreamingDistributionError::Validation(ref cause) => cause,
            CreateStreamingDistributionError::Credentials(ref err) => err.description(),
            CreateStreamingDistributionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateStreamingDistributionError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateStreamingDistributionWithTagsError {
    pub fn from_body(body: &str) -> CreateStreamingDistributionWithTagsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => CreateStreamingDistributionWithTagsError::AccessDenied(
                    String::from(parsed_error.message),
                ),
                "CNAMEAlreadyExists" => {
                    CreateStreamingDistributionWithTagsError::CNAMEAlreadyExists(String::from(
                        parsed_error.message,
                    ))
                }
                "InconsistentQuantities" => {
                    CreateStreamingDistributionWithTagsError::InconsistentQuantities(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidArgument" => CreateStreamingDistributionWithTagsError::InvalidArgument(
                    String::from(parsed_error.message),
                ),
                "InvalidOrigin" => CreateStreamingDistributionWithTagsError::InvalidOrigin(
                    String::from(parsed_error.message),
                ),
                "InvalidOriginAccessIdentity" => {
                    CreateStreamingDistributionWithTagsError::InvalidOriginAccessIdentity(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidTagging" => CreateStreamingDistributionWithTagsError::InvalidTagging(
                    String::from(parsed_error.message),
                ),
                "MissingBody" => CreateStreamingDistributionWithTagsError::MissingBody(
                    String::from(parsed_error.message),
                ),
                "StreamingDistributionAlreadyExists" => {
                    CreateStreamingDistributionWithTagsError::StreamingDistributionAlreadyExists(
                        String::from(parsed_error.message),
                    )
                }
                "TooManyStreamingDistributionCNAMEs" => {
                    CreateStreamingDistributionWithTagsError::TooManyStreamingDistributionCNAMEs(
                        String::from(parsed_error.message),
                    )
                }
                "TooManyStreamingDistributions" => {
                    CreateStreamingDistributionWithTagsError::TooManyStreamingDistributions(
                        String::from(parsed_error.message),
                    )
                }
                "TooManyTrustedSigners" => {
                    CreateStreamingDistributionWithTagsError::TooManyTrustedSigners(String::from(
                        parsed_error.message,
                    ))
                }
                "TrustedSignerDoesNotExist" => {
                    CreateStreamingDistributionWithTagsError::TrustedSignerDoesNotExist(
                        String::from(parsed_error.message),
                    )
                }
                _ => CreateStreamingDistributionWithTagsError::Unknown(String::from(body)),
            },
            Err(_) => CreateStreamingDistributionWithTagsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateStreamingDistributionWithTagsError {
    fn from(err: XmlParseError) -> CreateStreamingDistributionWithTagsError {
        let XmlParseError(message) = err;
        CreateStreamingDistributionWithTagsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateStreamingDistributionWithTagsError {
    fn from(err: CredentialsError) -> CreateStreamingDistributionWithTagsError {
        CreateStreamingDistributionWithTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStreamingDistributionWithTagsError {
    fn from(err: HttpDispatchError) -> CreateStreamingDistributionWithTagsError {
        CreateStreamingDistributionWithTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStreamingDistributionWithTagsError {
    fn from(err: io::Error) -> CreateStreamingDistributionWithTagsError {
        CreateStreamingDistributionWithTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStreamingDistributionWithTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStreamingDistributionWithTagsError {
    fn description(&self) -> &str {
        match *self {
            CreateStreamingDistributionWithTagsError::AccessDenied(ref cause) => cause,
            CreateStreamingDistributionWithTagsError::CNAMEAlreadyExists(ref cause) => cause,
            CreateStreamingDistributionWithTagsError::InconsistentQuantities(ref cause) => cause,
            CreateStreamingDistributionWithTagsError::InvalidArgument(ref cause) => cause,
            CreateStreamingDistributionWithTagsError::InvalidOrigin(ref cause) => cause,
            CreateStreamingDistributionWithTagsError::InvalidOriginAccessIdentity(ref cause) => {
                cause
            }
            CreateStreamingDistributionWithTagsError::InvalidTagging(ref cause) => cause,
            CreateStreamingDistributionWithTagsError::MissingBody(ref cause) => cause,
            CreateStreamingDistributionWithTagsError::StreamingDistributionAlreadyExists(
                ref cause,
            ) => cause,
            CreateStreamingDistributionWithTagsError::TooManyStreamingDistributionCNAMEs(
                ref cause,
            ) => cause,
            CreateStreamingDistributionWithTagsError::TooManyStreamingDistributions(ref cause) => {
                cause
            }
            CreateStreamingDistributionWithTagsError::TooManyTrustedSigners(ref cause) => cause,
            CreateStreamingDistributionWithTagsError::TrustedSignerDoesNotExist(ref cause) => cause,
            CreateStreamingDistributionWithTagsError::Validation(ref cause) => cause,
            CreateStreamingDistributionWithTagsError::Credentials(ref err) => err.description(),
            CreateStreamingDistributionWithTagsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateStreamingDistributionWithTagsError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteCloudFrontOriginAccessIdentityError {
    pub fn from_body(body: &str) -> DeleteCloudFrontOriginAccessIdentityError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => DeleteCloudFrontOriginAccessIdentityError::AccessDenied(
                    String::from(parsed_error.message),
                ),
                "CloudFrontOriginAccessIdentityInUse" => {
                    DeleteCloudFrontOriginAccessIdentityError::CloudFrontOriginAccessIdentityInUse(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidIfMatchVersion" => {
                    DeleteCloudFrontOriginAccessIdentityError::InvalidIfMatchVersion(String::from(
                        parsed_error.message,
                    ))
                }
                "NoSuchCloudFrontOriginAccessIdentity" => {
                    DeleteCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity(
                        String::from(parsed_error.message),
                    )
                }
                "PreconditionFailed" => {
                    DeleteCloudFrontOriginAccessIdentityError::PreconditionFailed(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteCloudFrontOriginAccessIdentityError::Unknown(String::from(body)),
            },
            Err(_) => DeleteCloudFrontOriginAccessIdentityError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteCloudFrontOriginAccessIdentityError {
    fn from(err: XmlParseError) -> DeleteCloudFrontOriginAccessIdentityError {
        let XmlParseError(message) = err;
        DeleteCloudFrontOriginAccessIdentityError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteCloudFrontOriginAccessIdentityError {
    fn from(err: CredentialsError) -> DeleteCloudFrontOriginAccessIdentityError {
        DeleteCloudFrontOriginAccessIdentityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCloudFrontOriginAccessIdentityError {
    fn from(err: HttpDispatchError) -> DeleteCloudFrontOriginAccessIdentityError {
        DeleteCloudFrontOriginAccessIdentityError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCloudFrontOriginAccessIdentityError {
    fn from(err: io::Error) -> DeleteCloudFrontOriginAccessIdentityError {
        DeleteCloudFrontOriginAccessIdentityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCloudFrontOriginAccessIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCloudFrontOriginAccessIdentityError {
    fn description(&self) -> &str {
        match *self {
            DeleteCloudFrontOriginAccessIdentityError::AccessDenied(ref cause) => cause,
            DeleteCloudFrontOriginAccessIdentityError::CloudFrontOriginAccessIdentityInUse(
                ref cause,
            ) => cause,
            DeleteCloudFrontOriginAccessIdentityError::InvalidIfMatchVersion(ref cause) => cause,
            DeleteCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity(
                ref cause,
            ) => cause,
            DeleteCloudFrontOriginAccessIdentityError::PreconditionFailed(ref cause) => cause,
            DeleteCloudFrontOriginAccessIdentityError::Validation(ref cause) => cause,
            DeleteCloudFrontOriginAccessIdentityError::Credentials(ref err) => err.description(),
            DeleteCloudFrontOriginAccessIdentityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCloudFrontOriginAccessIdentityError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteDistributionError {
    pub fn from_body(body: &str) -> DeleteDistributionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => {
                    DeleteDistributionError::AccessDenied(String::from(parsed_error.message))
                }
                "DistributionNotDisabled" => DeleteDistributionError::DistributionNotDisabled(
                    String::from(parsed_error.message),
                ),
                "InvalidIfMatchVersion" => DeleteDistributionError::InvalidIfMatchVersion(
                    String::from(parsed_error.message),
                ),
                "NoSuchDistribution" => {
                    DeleteDistributionError::NoSuchDistribution(String::from(parsed_error.message))
                }
                "PreconditionFailed" => {
                    DeleteDistributionError::PreconditionFailed(String::from(parsed_error.message))
                }
                _ => DeleteDistributionError::Unknown(String::from(body)),
            },
            Err(_) => DeleteDistributionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteDistributionError {
    fn from(err: XmlParseError) -> DeleteDistributionError {
        let XmlParseError(message) = err;
        DeleteDistributionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteDistributionError {
    fn from(err: CredentialsError) -> DeleteDistributionError {
        DeleteDistributionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDistributionError {
    fn from(err: HttpDispatchError) -> DeleteDistributionError {
        DeleteDistributionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDistributionError {
    fn from(err: io::Error) -> DeleteDistributionError {
        DeleteDistributionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDistributionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDistributionError {
    fn description(&self) -> &str {
        match *self {
            DeleteDistributionError::AccessDenied(ref cause) => cause,
            DeleteDistributionError::DistributionNotDisabled(ref cause) => cause,
            DeleteDistributionError::InvalidIfMatchVersion(ref cause) => cause,
            DeleteDistributionError::NoSuchDistribution(ref cause) => cause,
            DeleteDistributionError::PreconditionFailed(ref cause) => cause,
            DeleteDistributionError::Validation(ref cause) => cause,
            DeleteDistributionError::Credentials(ref err) => err.description(),
            DeleteDistributionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDistributionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteServiceLinkedRole
#[derive(Debug, PartialEq)]
pub enum DeleteServiceLinkedRoleError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),

    NoSuchResource(String),

    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteServiceLinkedRoleError {
    pub fn from_body(body: &str) -> DeleteServiceLinkedRoleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => {
                    DeleteServiceLinkedRoleError::AccessDenied(String::from(parsed_error.message))
                }
                "InvalidArgument" => DeleteServiceLinkedRoleError::InvalidArgument(String::from(
                    parsed_error.message,
                )),
                "NoSuchResource" => {
                    DeleteServiceLinkedRoleError::NoSuchResource(String::from(parsed_error.message))
                }
                "ResourceInUse" => {
                    DeleteServiceLinkedRoleError::ResourceInUse(String::from(parsed_error.message))
                }
                _ => DeleteServiceLinkedRoleError::Unknown(String::from(body)),
            },
            Err(_) => DeleteServiceLinkedRoleError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteServiceLinkedRoleError {
    fn from(err: XmlParseError) -> DeleteServiceLinkedRoleError {
        let XmlParseError(message) = err;
        DeleteServiceLinkedRoleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteServiceLinkedRoleError {
    fn from(err: CredentialsError) -> DeleteServiceLinkedRoleError {
        DeleteServiceLinkedRoleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteServiceLinkedRoleError {
    fn from(err: HttpDispatchError) -> DeleteServiceLinkedRoleError {
        DeleteServiceLinkedRoleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteServiceLinkedRoleError {
    fn from(err: io::Error) -> DeleteServiceLinkedRoleError {
        DeleteServiceLinkedRoleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteServiceLinkedRoleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteServiceLinkedRoleError {
    fn description(&self) -> &str {
        match *self {
            DeleteServiceLinkedRoleError::AccessDenied(ref cause) => cause,
            DeleteServiceLinkedRoleError::InvalidArgument(ref cause) => cause,
            DeleteServiceLinkedRoleError::NoSuchResource(ref cause) => cause,
            DeleteServiceLinkedRoleError::ResourceInUse(ref cause) => cause,
            DeleteServiceLinkedRoleError::Validation(ref cause) => cause,
            DeleteServiceLinkedRoleError::Credentials(ref err) => err.description(),
            DeleteServiceLinkedRoleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteServiceLinkedRoleError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteStreamingDistributionError {
    pub fn from_body(body: &str) -> DeleteStreamingDistributionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => DeleteStreamingDistributionError::AccessDenied(String::from(
                    parsed_error.message,
                )),
                "InvalidIfMatchVersion" => DeleteStreamingDistributionError::InvalidIfMatchVersion(
                    String::from(parsed_error.message),
                ),
                "NoSuchStreamingDistribution" => {
                    DeleteStreamingDistributionError::NoSuchStreamingDistribution(String::from(
                        parsed_error.message,
                    ))
                }
                "PreconditionFailed" => DeleteStreamingDistributionError::PreconditionFailed(
                    String::from(parsed_error.message),
                ),
                "StreamingDistributionNotDisabled" => {
                    DeleteStreamingDistributionError::StreamingDistributionNotDisabled(
                        String::from(parsed_error.message),
                    )
                }
                _ => DeleteStreamingDistributionError::Unknown(String::from(body)),
            },
            Err(_) => DeleteStreamingDistributionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteStreamingDistributionError {
    fn from(err: XmlParseError) -> DeleteStreamingDistributionError {
        let XmlParseError(message) = err;
        DeleteStreamingDistributionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteStreamingDistributionError {
    fn from(err: CredentialsError) -> DeleteStreamingDistributionError {
        DeleteStreamingDistributionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteStreamingDistributionError {
    fn from(err: HttpDispatchError) -> DeleteStreamingDistributionError {
        DeleteStreamingDistributionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteStreamingDistributionError {
    fn from(err: io::Error) -> DeleteStreamingDistributionError {
        DeleteStreamingDistributionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteStreamingDistributionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteStreamingDistributionError {
    fn description(&self) -> &str {
        match *self {
            DeleteStreamingDistributionError::AccessDenied(ref cause) => cause,
            DeleteStreamingDistributionError::InvalidIfMatchVersion(ref cause) => cause,
            DeleteStreamingDistributionError::NoSuchStreamingDistribution(ref cause) => cause,
            DeleteStreamingDistributionError::PreconditionFailed(ref cause) => cause,
            DeleteStreamingDistributionError::StreamingDistributionNotDisabled(ref cause) => cause,
            DeleteStreamingDistributionError::Validation(ref cause) => cause,
            DeleteStreamingDistributionError::Credentials(ref err) => err.description(),
            DeleteStreamingDistributionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteStreamingDistributionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCloudFrontOriginAccessIdentity
#[derive(Debug, PartialEq)]
pub enum GetCloudFrontOriginAccessIdentityError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified origin access identity does not exist.</p>
    NoSuchCloudFrontOriginAccessIdentity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetCloudFrontOriginAccessIdentityError {
    pub fn from_body(body: &str) -> GetCloudFrontOriginAccessIdentityError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => GetCloudFrontOriginAccessIdentityError::AccessDenied(
                    String::from(parsed_error.message),
                ),
                "NoSuchCloudFrontOriginAccessIdentity" => {
                    GetCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity(
                        String::from(parsed_error.message),
                    )
                }
                _ => GetCloudFrontOriginAccessIdentityError::Unknown(String::from(body)),
            },
            Err(_) => GetCloudFrontOriginAccessIdentityError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetCloudFrontOriginAccessIdentityError {
    fn from(err: XmlParseError) -> GetCloudFrontOriginAccessIdentityError {
        let XmlParseError(message) = err;
        GetCloudFrontOriginAccessIdentityError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetCloudFrontOriginAccessIdentityError {
    fn from(err: CredentialsError) -> GetCloudFrontOriginAccessIdentityError {
        GetCloudFrontOriginAccessIdentityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCloudFrontOriginAccessIdentityError {
    fn from(err: HttpDispatchError) -> GetCloudFrontOriginAccessIdentityError {
        GetCloudFrontOriginAccessIdentityError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCloudFrontOriginAccessIdentityError {
    fn from(err: io::Error) -> GetCloudFrontOriginAccessIdentityError {
        GetCloudFrontOriginAccessIdentityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCloudFrontOriginAccessIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCloudFrontOriginAccessIdentityError {
    fn description(&self) -> &str {
        match *self {
            GetCloudFrontOriginAccessIdentityError::AccessDenied(ref cause) => cause,
            GetCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity(
                ref cause,
            ) => cause,
            GetCloudFrontOriginAccessIdentityError::Validation(ref cause) => cause,
            GetCloudFrontOriginAccessIdentityError::Credentials(ref err) => err.description(),
            GetCloudFrontOriginAccessIdentityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCloudFrontOriginAccessIdentityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCloudFrontOriginAccessIdentityConfig
#[derive(Debug, PartialEq)]
pub enum GetCloudFrontOriginAccessIdentityConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified origin access identity does not exist.</p>
    NoSuchCloudFrontOriginAccessIdentity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetCloudFrontOriginAccessIdentityConfigError {
    pub fn from_body(body: &str) -> GetCloudFrontOriginAccessIdentityConfigError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "AccessDenied" => GetCloudFrontOriginAccessIdentityConfigError::AccessDenied(String::from(parsed_error.message)),"NoSuchCloudFrontOriginAccessIdentity" => GetCloudFrontOriginAccessIdentityConfigError::NoSuchCloudFrontOriginAccessIdentity(String::from(parsed_error.message)),_ => GetCloudFrontOriginAccessIdentityConfigError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetCloudFrontOriginAccessIdentityConfigError::Unknown(body.to_string())
                       }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetCloudFrontOriginAccessIdentityConfigError {
    fn from(err: XmlParseError) -> GetCloudFrontOriginAccessIdentityConfigError {
        let XmlParseError(message) = err;
        GetCloudFrontOriginAccessIdentityConfigError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetCloudFrontOriginAccessIdentityConfigError {
    fn from(err: CredentialsError) -> GetCloudFrontOriginAccessIdentityConfigError {
        GetCloudFrontOriginAccessIdentityConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCloudFrontOriginAccessIdentityConfigError {
    fn from(err: HttpDispatchError) -> GetCloudFrontOriginAccessIdentityConfigError {
        GetCloudFrontOriginAccessIdentityConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCloudFrontOriginAccessIdentityConfigError {
    fn from(err: io::Error) -> GetCloudFrontOriginAccessIdentityConfigError {
        GetCloudFrontOriginAccessIdentityConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCloudFrontOriginAccessIdentityConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCloudFrontOriginAccessIdentityConfigError {
    fn description(&self) -> &str {
        match *self {
            GetCloudFrontOriginAccessIdentityConfigError::AccessDenied(ref cause) => cause,
            GetCloudFrontOriginAccessIdentityConfigError::NoSuchCloudFrontOriginAccessIdentity(
                ref cause,
            ) => cause,
            GetCloudFrontOriginAccessIdentityConfigError::Validation(ref cause) => cause,
            GetCloudFrontOriginAccessIdentityConfigError::Credentials(ref err) => err.description(),
            GetCloudFrontOriginAccessIdentityConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCloudFrontOriginAccessIdentityConfigError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDistribution
#[derive(Debug, PartialEq)]
pub enum GetDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDistributionError {
    pub fn from_body(body: &str) -> GetDistributionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => {
                    GetDistributionError::AccessDenied(String::from(parsed_error.message))
                }
                "NoSuchDistribution" => {
                    GetDistributionError::NoSuchDistribution(String::from(parsed_error.message))
                }
                _ => GetDistributionError::Unknown(String::from(body)),
            },
            Err(_) => GetDistributionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetDistributionError {
    fn from(err: XmlParseError) -> GetDistributionError {
        let XmlParseError(message) = err;
        GetDistributionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetDistributionError {
    fn from(err: CredentialsError) -> GetDistributionError {
        GetDistributionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDistributionError {
    fn from(err: HttpDispatchError) -> GetDistributionError {
        GetDistributionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDistributionError {
    fn from(err: io::Error) -> GetDistributionError {
        GetDistributionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDistributionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDistributionError {
    fn description(&self) -> &str {
        match *self {
            GetDistributionError::AccessDenied(ref cause) => cause,
            GetDistributionError::NoSuchDistribution(ref cause) => cause,
            GetDistributionError::Validation(ref cause) => cause,
            GetDistributionError::Credentials(ref err) => err.description(),
            GetDistributionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDistributionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDistributionConfig
#[derive(Debug, PartialEq)]
pub enum GetDistributionConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDistributionConfigError {
    pub fn from_body(body: &str) -> GetDistributionConfigError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => {
                    GetDistributionConfigError::AccessDenied(String::from(parsed_error.message))
                }
                "NoSuchDistribution" => GetDistributionConfigError::NoSuchDistribution(
                    String::from(parsed_error.message),
                ),
                _ => GetDistributionConfigError::Unknown(String::from(body)),
            },
            Err(_) => GetDistributionConfigError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetDistributionConfigError {
    fn from(err: XmlParseError) -> GetDistributionConfigError {
        let XmlParseError(message) = err;
        GetDistributionConfigError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetDistributionConfigError {
    fn from(err: CredentialsError) -> GetDistributionConfigError {
        GetDistributionConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDistributionConfigError {
    fn from(err: HttpDispatchError) -> GetDistributionConfigError {
        GetDistributionConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDistributionConfigError {
    fn from(err: io::Error) -> GetDistributionConfigError {
        GetDistributionConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDistributionConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDistributionConfigError {
    fn description(&self) -> &str {
        match *self {
            GetDistributionConfigError::AccessDenied(ref cause) => cause,
            GetDistributionConfigError::NoSuchDistribution(ref cause) => cause,
            GetDistributionConfigError::Validation(ref cause) => cause,
            GetDistributionConfigError::Credentials(ref err) => err.description(),
            GetDistributionConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDistributionConfigError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInvalidation
#[derive(Debug, PartialEq)]
pub enum GetInvalidationError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
    /// <p>The specified invalidation does not exist.</p>
    NoSuchInvalidation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetInvalidationError {
    pub fn from_body(body: &str) -> GetInvalidationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => {
                    GetInvalidationError::AccessDenied(String::from(parsed_error.message))
                }
                "NoSuchDistribution" => {
                    GetInvalidationError::NoSuchDistribution(String::from(parsed_error.message))
                }
                "NoSuchInvalidation" => {
                    GetInvalidationError::NoSuchInvalidation(String::from(parsed_error.message))
                }
                _ => GetInvalidationError::Unknown(String::from(body)),
            },
            Err(_) => GetInvalidationError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetInvalidationError {
    fn from(err: XmlParseError) -> GetInvalidationError {
        let XmlParseError(message) = err;
        GetInvalidationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetInvalidationError {
    fn from(err: CredentialsError) -> GetInvalidationError {
        GetInvalidationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInvalidationError {
    fn from(err: HttpDispatchError) -> GetInvalidationError {
        GetInvalidationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInvalidationError {
    fn from(err: io::Error) -> GetInvalidationError {
        GetInvalidationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInvalidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInvalidationError {
    fn description(&self) -> &str {
        match *self {
            GetInvalidationError::AccessDenied(ref cause) => cause,
            GetInvalidationError::NoSuchDistribution(ref cause) => cause,
            GetInvalidationError::NoSuchInvalidation(ref cause) => cause,
            GetInvalidationError::Validation(ref cause) => cause,
            GetInvalidationError::Credentials(ref err) => err.description(),
            GetInvalidationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetInvalidationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetStreamingDistribution
#[derive(Debug, PartialEq)]
pub enum GetStreamingDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified streaming distribution does not exist.</p>
    NoSuchStreamingDistribution(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetStreamingDistributionError {
    pub fn from_body(body: &str) -> GetStreamingDistributionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => {
                    GetStreamingDistributionError::AccessDenied(String::from(parsed_error.message))
                }
                "NoSuchStreamingDistribution" => {
                    GetStreamingDistributionError::NoSuchStreamingDistribution(String::from(
                        parsed_error.message,
                    ))
                }
                _ => GetStreamingDistributionError::Unknown(String::from(body)),
            },
            Err(_) => GetStreamingDistributionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetStreamingDistributionError {
    fn from(err: XmlParseError) -> GetStreamingDistributionError {
        let XmlParseError(message) = err;
        GetStreamingDistributionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetStreamingDistributionError {
    fn from(err: CredentialsError) -> GetStreamingDistributionError {
        GetStreamingDistributionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetStreamingDistributionError {
    fn from(err: HttpDispatchError) -> GetStreamingDistributionError {
        GetStreamingDistributionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetStreamingDistributionError {
    fn from(err: io::Error) -> GetStreamingDistributionError {
        GetStreamingDistributionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetStreamingDistributionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetStreamingDistributionError {
    fn description(&self) -> &str {
        match *self {
            GetStreamingDistributionError::AccessDenied(ref cause) => cause,
            GetStreamingDistributionError::NoSuchStreamingDistribution(ref cause) => cause,
            GetStreamingDistributionError::Validation(ref cause) => cause,
            GetStreamingDistributionError::Credentials(ref err) => err.description(),
            GetStreamingDistributionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetStreamingDistributionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetStreamingDistributionConfig
#[derive(Debug, PartialEq)]
pub enum GetStreamingDistributionConfigError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The specified streaming distribution does not exist.</p>
    NoSuchStreamingDistribution(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetStreamingDistributionConfigError {
    pub fn from_body(body: &str) -> GetStreamingDistributionConfigError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => GetStreamingDistributionConfigError::AccessDenied(String::from(
                    parsed_error.message,
                )),
                "NoSuchStreamingDistribution" => {
                    GetStreamingDistributionConfigError::NoSuchStreamingDistribution(String::from(
                        parsed_error.message,
                    ))
                }
                _ => GetStreamingDistributionConfigError::Unknown(String::from(body)),
            },
            Err(_) => GetStreamingDistributionConfigError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetStreamingDistributionConfigError {
    fn from(err: XmlParseError) -> GetStreamingDistributionConfigError {
        let XmlParseError(message) = err;
        GetStreamingDistributionConfigError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetStreamingDistributionConfigError {
    fn from(err: CredentialsError) -> GetStreamingDistributionConfigError {
        GetStreamingDistributionConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetStreamingDistributionConfigError {
    fn from(err: HttpDispatchError) -> GetStreamingDistributionConfigError {
        GetStreamingDistributionConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetStreamingDistributionConfigError {
    fn from(err: io::Error) -> GetStreamingDistributionConfigError {
        GetStreamingDistributionConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetStreamingDistributionConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetStreamingDistributionConfigError {
    fn description(&self) -> &str {
        match *self {
            GetStreamingDistributionConfigError::AccessDenied(ref cause) => cause,
            GetStreamingDistributionConfigError::NoSuchStreamingDistribution(ref cause) => cause,
            GetStreamingDistributionConfigError::Validation(ref cause) => cause,
            GetStreamingDistributionConfigError::Credentials(ref err) => err.description(),
            GetStreamingDistributionConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetStreamingDistributionConfigError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCloudFrontOriginAccessIdentities
#[derive(Debug, PartialEq)]
pub enum ListCloudFrontOriginAccessIdentitiesError {
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListCloudFrontOriginAccessIdentitiesError {
    pub fn from_body(body: &str) -> ListCloudFrontOriginAccessIdentitiesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidArgument" => ListCloudFrontOriginAccessIdentitiesError::InvalidArgument(
                    String::from(parsed_error.message),
                ),
                _ => ListCloudFrontOriginAccessIdentitiesError::Unknown(String::from(body)),
            },
            Err(_) => ListCloudFrontOriginAccessIdentitiesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListCloudFrontOriginAccessIdentitiesError {
    fn from(err: XmlParseError) -> ListCloudFrontOriginAccessIdentitiesError {
        let XmlParseError(message) = err;
        ListCloudFrontOriginAccessIdentitiesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListCloudFrontOriginAccessIdentitiesError {
    fn from(err: CredentialsError) -> ListCloudFrontOriginAccessIdentitiesError {
        ListCloudFrontOriginAccessIdentitiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCloudFrontOriginAccessIdentitiesError {
    fn from(err: HttpDispatchError) -> ListCloudFrontOriginAccessIdentitiesError {
        ListCloudFrontOriginAccessIdentitiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCloudFrontOriginAccessIdentitiesError {
    fn from(err: io::Error) -> ListCloudFrontOriginAccessIdentitiesError {
        ListCloudFrontOriginAccessIdentitiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCloudFrontOriginAccessIdentitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCloudFrontOriginAccessIdentitiesError {
    fn description(&self) -> &str {
        match *self {
            ListCloudFrontOriginAccessIdentitiesError::InvalidArgument(ref cause) => cause,
            ListCloudFrontOriginAccessIdentitiesError::Validation(ref cause) => cause,
            ListCloudFrontOriginAccessIdentitiesError::Credentials(ref err) => err.description(),
            ListCloudFrontOriginAccessIdentitiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListCloudFrontOriginAccessIdentitiesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDistributions
#[derive(Debug, PartialEq)]
pub enum ListDistributionsError {
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDistributionsError {
    pub fn from_body(body: &str) -> ListDistributionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidArgument" => {
                    ListDistributionsError::InvalidArgument(String::from(parsed_error.message))
                }
                _ => ListDistributionsError::Unknown(String::from(body)),
            },
            Err(_) => ListDistributionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListDistributionsError {
    fn from(err: XmlParseError) -> ListDistributionsError {
        let XmlParseError(message) = err;
        ListDistributionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListDistributionsError {
    fn from(err: CredentialsError) -> ListDistributionsError {
        ListDistributionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDistributionsError {
    fn from(err: HttpDispatchError) -> ListDistributionsError {
        ListDistributionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDistributionsError {
    fn from(err: io::Error) -> ListDistributionsError {
        ListDistributionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDistributionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDistributionsError {
    fn description(&self) -> &str {
        match *self {
            ListDistributionsError::InvalidArgument(ref cause) => cause,
            ListDistributionsError::Validation(ref cause) => cause,
            ListDistributionsError::Credentials(ref err) => err.description(),
            ListDistributionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDistributionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDistributionsByWebACLId
#[derive(Debug, PartialEq)]
pub enum ListDistributionsByWebACLIdError {
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),

    InvalidWebACLId(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDistributionsByWebACLIdError {
    pub fn from_body(body: &str) -> ListDistributionsByWebACLIdError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidArgument" => ListDistributionsByWebACLIdError::InvalidArgument(
                    String::from(parsed_error.message),
                ),
                "InvalidWebACLId" => ListDistributionsByWebACLIdError::InvalidWebACLId(
                    String::from(parsed_error.message),
                ),
                _ => ListDistributionsByWebACLIdError::Unknown(String::from(body)),
            },
            Err(_) => ListDistributionsByWebACLIdError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListDistributionsByWebACLIdError {
    fn from(err: XmlParseError) -> ListDistributionsByWebACLIdError {
        let XmlParseError(message) = err;
        ListDistributionsByWebACLIdError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListDistributionsByWebACLIdError {
    fn from(err: CredentialsError) -> ListDistributionsByWebACLIdError {
        ListDistributionsByWebACLIdError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDistributionsByWebACLIdError {
    fn from(err: HttpDispatchError) -> ListDistributionsByWebACLIdError {
        ListDistributionsByWebACLIdError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDistributionsByWebACLIdError {
    fn from(err: io::Error) -> ListDistributionsByWebACLIdError {
        ListDistributionsByWebACLIdError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDistributionsByWebACLIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDistributionsByWebACLIdError {
    fn description(&self) -> &str {
        match *self {
            ListDistributionsByWebACLIdError::InvalidArgument(ref cause) => cause,
            ListDistributionsByWebACLIdError::InvalidWebACLId(ref cause) => cause,
            ListDistributionsByWebACLIdError::Validation(ref cause) => cause,
            ListDistributionsByWebACLIdError::Credentials(ref err) => err.description(),
            ListDistributionsByWebACLIdError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDistributionsByWebACLIdError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListInvalidations
#[derive(Debug, PartialEq)]
pub enum ListInvalidationsError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// <p>The specified distribution does not exist.</p>
    NoSuchDistribution(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListInvalidationsError {
    pub fn from_body(body: &str) -> ListInvalidationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => {
                    ListInvalidationsError::AccessDenied(String::from(parsed_error.message))
                }
                "InvalidArgument" => {
                    ListInvalidationsError::InvalidArgument(String::from(parsed_error.message))
                }
                "NoSuchDistribution" => {
                    ListInvalidationsError::NoSuchDistribution(String::from(parsed_error.message))
                }
                _ => ListInvalidationsError::Unknown(String::from(body)),
            },
            Err(_) => ListInvalidationsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListInvalidationsError {
    fn from(err: XmlParseError) -> ListInvalidationsError {
        let XmlParseError(message) = err;
        ListInvalidationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListInvalidationsError {
    fn from(err: CredentialsError) -> ListInvalidationsError {
        ListInvalidationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListInvalidationsError {
    fn from(err: HttpDispatchError) -> ListInvalidationsError {
        ListInvalidationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListInvalidationsError {
    fn from(err: io::Error) -> ListInvalidationsError {
        ListInvalidationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListInvalidationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListInvalidationsError {
    fn description(&self) -> &str {
        match *self {
            ListInvalidationsError::AccessDenied(ref cause) => cause,
            ListInvalidationsError::InvalidArgument(ref cause) => cause,
            ListInvalidationsError::NoSuchDistribution(ref cause) => cause,
            ListInvalidationsError::Validation(ref cause) => cause,
            ListInvalidationsError::Credentials(ref err) => err.description(),
            ListInvalidationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListInvalidationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListStreamingDistributions
#[derive(Debug, PartialEq)]
pub enum ListStreamingDistributionsError {
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListStreamingDistributionsError {
    pub fn from_body(body: &str) -> ListStreamingDistributionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidArgument" => ListStreamingDistributionsError::InvalidArgument(
                    String::from(parsed_error.message),
                ),
                _ => ListStreamingDistributionsError::Unknown(String::from(body)),
            },
            Err(_) => ListStreamingDistributionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListStreamingDistributionsError {
    fn from(err: XmlParseError) -> ListStreamingDistributionsError {
        let XmlParseError(message) = err;
        ListStreamingDistributionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListStreamingDistributionsError {
    fn from(err: CredentialsError) -> ListStreamingDistributionsError {
        ListStreamingDistributionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListStreamingDistributionsError {
    fn from(err: HttpDispatchError) -> ListStreamingDistributionsError {
        ListStreamingDistributionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListStreamingDistributionsError {
    fn from(err: io::Error) -> ListStreamingDistributionsError {
        ListStreamingDistributionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListStreamingDistributionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStreamingDistributionsError {
    fn description(&self) -> &str {
        match *self {
            ListStreamingDistributionsError::InvalidArgument(ref cause) => cause,
            ListStreamingDistributionsError::Validation(ref cause) => cause,
            ListStreamingDistributionsError::Credentials(ref err) => err.description(),
            ListStreamingDistributionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListStreamingDistributionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),

    InvalidTagging(String),

    NoSuchResource(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsForResourceError {
    pub fn from_body(body: &str) -> ListTagsForResourceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => {
                    ListTagsForResourceError::AccessDenied(String::from(parsed_error.message))
                }
                "InvalidArgument" => {
                    ListTagsForResourceError::InvalidArgument(String::from(parsed_error.message))
                }
                "InvalidTagging" => {
                    ListTagsForResourceError::InvalidTagging(String::from(parsed_error.message))
                }
                "NoSuchResource" => {
                    ListTagsForResourceError::NoSuchResource(String::from(parsed_error.message))
                }
                _ => ListTagsForResourceError::Unknown(String::from(body)),
            },
            Err(_) => ListTagsForResourceError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListTagsForResourceError {
    fn from(err: XmlParseError) -> ListTagsForResourceError {
        let XmlParseError(message) = err;
        ListTagsForResourceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListTagsForResourceError {
    fn from(err: CredentialsError) -> ListTagsForResourceError {
        ListTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForResourceError {
    fn from(err: HttpDispatchError) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForResourceError {
    fn from(err: io::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::AccessDenied(ref cause) => cause,
            ListTagsForResourceError::InvalidArgument(ref cause) => cause,
            ListTagsForResourceError::InvalidTagging(ref cause) => cause,
            ListTagsForResourceError::NoSuchResource(ref cause) => cause,
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),

    InvalidTagging(String),

    NoSuchResource(String),
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
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => {
                    TagResourceError::AccessDenied(String::from(parsed_error.message))
                }
                "InvalidArgument" => {
                    TagResourceError::InvalidArgument(String::from(parsed_error.message))
                }
                "InvalidTagging" => {
                    TagResourceError::InvalidTagging(String::from(parsed_error.message))
                }
                "NoSuchResource" => {
                    TagResourceError::NoSuchResource(String::from(parsed_error.message))
                }
                _ => TagResourceError::Unknown(String::from(body)),
            },
            Err(_) => TagResourceError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for TagResourceError {
    fn from(err: XmlParseError) -> TagResourceError {
        let XmlParseError(message) = err;
        TagResourceError::Unknown(message.to_string())
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
            TagResourceError::AccessDenied(ref cause) => cause,
            TagResourceError::InvalidArgument(ref cause) => cause,
            TagResourceError::InvalidTagging(ref cause) => cause,
            TagResourceError::NoSuchResource(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Access denied.</p>
    AccessDenied(String),
    /// <p>The argument is invalid.</p>
    InvalidArgument(String),

    InvalidTagging(String),

    NoSuchResource(String),
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
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => {
                    UntagResourceError::AccessDenied(String::from(parsed_error.message))
                }
                "InvalidArgument" => {
                    UntagResourceError::InvalidArgument(String::from(parsed_error.message))
                }
                "InvalidTagging" => {
                    UntagResourceError::InvalidTagging(String::from(parsed_error.message))
                }
                "NoSuchResource" => {
                    UntagResourceError::NoSuchResource(String::from(parsed_error.message))
                }
                _ => UntagResourceError::Unknown(String::from(body)),
            },
            Err(_) => UntagResourceError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UntagResourceError {
    fn from(err: XmlParseError) -> UntagResourceError {
        let XmlParseError(message) = err;
        UntagResourceError::Unknown(message.to_string())
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
            UntagResourceError::AccessDenied(ref cause) => cause,
            UntagResourceError::InvalidArgument(ref cause) => cause,
            UntagResourceError::InvalidTagging(ref cause) => cause,
            UntagResourceError::NoSuchResource(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateCloudFrontOriginAccessIdentityError {
    pub fn from_body(body: &str) -> UpdateCloudFrontOriginAccessIdentityError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => UpdateCloudFrontOriginAccessIdentityError::AccessDenied(
                    String::from(parsed_error.message),
                ),
                "IllegalUpdate" => UpdateCloudFrontOriginAccessIdentityError::IllegalUpdate(
                    String::from(parsed_error.message),
                ),
                "InconsistentQuantities" => {
                    UpdateCloudFrontOriginAccessIdentityError::InconsistentQuantities(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidArgument" => UpdateCloudFrontOriginAccessIdentityError::InvalidArgument(
                    String::from(parsed_error.message),
                ),
                "InvalidIfMatchVersion" => {
                    UpdateCloudFrontOriginAccessIdentityError::InvalidIfMatchVersion(String::from(
                        parsed_error.message,
                    ))
                }
                "MissingBody" => UpdateCloudFrontOriginAccessIdentityError::MissingBody(
                    String::from(parsed_error.message),
                ),
                "NoSuchCloudFrontOriginAccessIdentity" => {
                    UpdateCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity(
                        String::from(parsed_error.message),
                    )
                }
                "PreconditionFailed" => {
                    UpdateCloudFrontOriginAccessIdentityError::PreconditionFailed(String::from(
                        parsed_error.message,
                    ))
                }
                _ => UpdateCloudFrontOriginAccessIdentityError::Unknown(String::from(body)),
            },
            Err(_) => UpdateCloudFrontOriginAccessIdentityError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateCloudFrontOriginAccessIdentityError {
    fn from(err: XmlParseError) -> UpdateCloudFrontOriginAccessIdentityError {
        let XmlParseError(message) = err;
        UpdateCloudFrontOriginAccessIdentityError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateCloudFrontOriginAccessIdentityError {
    fn from(err: CredentialsError) -> UpdateCloudFrontOriginAccessIdentityError {
        UpdateCloudFrontOriginAccessIdentityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCloudFrontOriginAccessIdentityError {
    fn from(err: HttpDispatchError) -> UpdateCloudFrontOriginAccessIdentityError {
        UpdateCloudFrontOriginAccessIdentityError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateCloudFrontOriginAccessIdentityError {
    fn from(err: io::Error) -> UpdateCloudFrontOriginAccessIdentityError {
        UpdateCloudFrontOriginAccessIdentityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateCloudFrontOriginAccessIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCloudFrontOriginAccessIdentityError {
    fn description(&self) -> &str {
        match *self {
            UpdateCloudFrontOriginAccessIdentityError::AccessDenied(ref cause) => cause,
            UpdateCloudFrontOriginAccessIdentityError::IllegalUpdate(ref cause) => cause,
            UpdateCloudFrontOriginAccessIdentityError::InconsistentQuantities(ref cause) => cause,
            UpdateCloudFrontOriginAccessIdentityError::InvalidArgument(ref cause) => cause,
            UpdateCloudFrontOriginAccessIdentityError::InvalidIfMatchVersion(ref cause) => cause,
            UpdateCloudFrontOriginAccessIdentityError::MissingBody(ref cause) => cause,
            UpdateCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity(
                ref cause,
            ) => cause,
            UpdateCloudFrontOriginAccessIdentityError::PreconditionFailed(ref cause) => cause,
            UpdateCloudFrontOriginAccessIdentityError::Validation(ref cause) => cause,
            UpdateCloudFrontOriginAccessIdentityError::Credentials(ref err) => err.description(),
            UpdateCloudFrontOriginAccessIdentityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateCloudFrontOriginAccessIdentityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDistribution
#[derive(Debug, PartialEq)]
pub enum UpdateDistributionError {
    /// <p>Access denied.</p>
    AccessDenied(String),

    CNAMEAlreadyExists(String),
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
    /// <p>Processing your request would cause the maximum number of distributions with Lambda function associations per owner to be exceeded.</p>
    TooManyDistributionsWithLambdaAssociations(String),

    TooManyHeadersInForwardedValues(String),
    /// <p>Your request contains more Lambda function associations than are allowed per distribution.</p>
    TooManyLambdaFunctionAssociations(String),

    TooManyOriginCustomHeaders(String),
    /// <p>You cannot create more origins for the distribution.</p>
    TooManyOrigins(String),

    TooManyQueryStringParameters(String),
    /// <p>Your request contains more trusted signers than are allowed per distribution.</p>
    TooManyTrustedSigners(String),
    /// <p>One or more of your trusted signers don't exist.</p>
    TrustedSignerDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDistributionError {
    pub fn from_body(body: &str) -> UpdateDistributionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => {
                    UpdateDistributionError::AccessDenied(String::from(parsed_error.message))
                }
                "CNAMEAlreadyExists" => {
                    UpdateDistributionError::CNAMEAlreadyExists(String::from(parsed_error.message))
                }
                "IllegalUpdate" => {
                    UpdateDistributionError::IllegalUpdate(String::from(parsed_error.message))
                }
                "InconsistentQuantities" => UpdateDistributionError::InconsistentQuantities(
                    String::from(parsed_error.message),
                ),
                "InvalidArgument" => {
                    UpdateDistributionError::InvalidArgument(String::from(parsed_error.message))
                }
                "InvalidDefaultRootObject" => UpdateDistributionError::InvalidDefaultRootObject(
                    String::from(parsed_error.message),
                ),
                "InvalidErrorCode" => {
                    UpdateDistributionError::InvalidErrorCode(String::from(parsed_error.message))
                }
                "InvalidForwardCookies" => UpdateDistributionError::InvalidForwardCookies(
                    String::from(parsed_error.message),
                ),
                "InvalidGeoRestrictionParameter" => {
                    UpdateDistributionError::InvalidGeoRestrictionParameter(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidHeadersForS3Origin" => UpdateDistributionError::InvalidHeadersForS3Origin(
                    String::from(parsed_error.message),
                ),
                "InvalidIfMatchVersion" => UpdateDistributionError::InvalidIfMatchVersion(
                    String::from(parsed_error.message),
                ),
                "InvalidLambdaFunctionAssociation" => {
                    UpdateDistributionError::InvalidLambdaFunctionAssociation(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidLocationCode" => {
                    UpdateDistributionError::InvalidLocationCode(String::from(parsed_error.message))
                }
                "InvalidMinimumProtocolVersion" => {
                    UpdateDistributionError::InvalidMinimumProtocolVersion(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidOriginAccessIdentity" => {
                    UpdateDistributionError::InvalidOriginAccessIdentity(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidOriginKeepaliveTimeout" => {
                    UpdateDistributionError::InvalidOriginKeepaliveTimeout(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidOriginReadTimeout" => UpdateDistributionError::InvalidOriginReadTimeout(
                    String::from(parsed_error.message),
                ),
                "InvalidQueryStringParameters" => {
                    UpdateDistributionError::InvalidQueryStringParameters(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidRelativePath" => {
                    UpdateDistributionError::InvalidRelativePath(String::from(parsed_error.message))
                }
                "InvalidRequiredProtocol" => UpdateDistributionError::InvalidRequiredProtocol(
                    String::from(parsed_error.message),
                ),
                "InvalidResponseCode" => {
                    UpdateDistributionError::InvalidResponseCode(String::from(parsed_error.message))
                }
                "InvalidTTLOrder" => {
                    UpdateDistributionError::InvalidTTLOrder(String::from(parsed_error.message))
                }
                "InvalidViewerCertificate" => UpdateDistributionError::InvalidViewerCertificate(
                    String::from(parsed_error.message),
                ),
                "InvalidWebACLId" => {
                    UpdateDistributionError::InvalidWebACLId(String::from(parsed_error.message))
                }
                "MissingBody" => {
                    UpdateDistributionError::MissingBody(String::from(parsed_error.message))
                }
                "NoSuchDistribution" => {
                    UpdateDistributionError::NoSuchDistribution(String::from(parsed_error.message))
                }
                "NoSuchOrigin" => {
                    UpdateDistributionError::NoSuchOrigin(String::from(parsed_error.message))
                }
                "PreconditionFailed" => {
                    UpdateDistributionError::PreconditionFailed(String::from(parsed_error.message))
                }
                "TooManyCacheBehaviors" => UpdateDistributionError::TooManyCacheBehaviors(
                    String::from(parsed_error.message),
                ),
                "TooManyCertificates" => {
                    UpdateDistributionError::TooManyCertificates(String::from(parsed_error.message))
                }
                "TooManyCookieNamesInWhiteList" => {
                    UpdateDistributionError::TooManyCookieNamesInWhiteList(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyDistributionCNAMEs" => UpdateDistributionError::TooManyDistributionCNAMEs(
                    String::from(parsed_error.message),
                ),
                "TooManyDistributionsWithLambdaAssociations" => {
                    UpdateDistributionError::TooManyDistributionsWithLambdaAssociations(
                        String::from(parsed_error.message),
                    )
                }
                "TooManyHeadersInForwardedValues" => {
                    UpdateDistributionError::TooManyHeadersInForwardedValues(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyLambdaFunctionAssociations" => {
                    UpdateDistributionError::TooManyLambdaFunctionAssociations(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyOriginCustomHeaders" => {
                    UpdateDistributionError::TooManyOriginCustomHeaders(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyOrigins" => {
                    UpdateDistributionError::TooManyOrigins(String::from(parsed_error.message))
                }
                "TooManyQueryStringParameters" => {
                    UpdateDistributionError::TooManyQueryStringParameters(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyTrustedSigners" => UpdateDistributionError::TooManyTrustedSigners(
                    String::from(parsed_error.message),
                ),
                "TrustedSignerDoesNotExist" => UpdateDistributionError::TrustedSignerDoesNotExist(
                    String::from(parsed_error.message),
                ),
                _ => UpdateDistributionError::Unknown(String::from(body)),
            },
            Err(_) => UpdateDistributionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateDistributionError {
    fn from(err: XmlParseError) -> UpdateDistributionError {
        let XmlParseError(message) = err;
        UpdateDistributionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateDistributionError {
    fn from(err: CredentialsError) -> UpdateDistributionError {
        UpdateDistributionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDistributionError {
    fn from(err: HttpDispatchError) -> UpdateDistributionError {
        UpdateDistributionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDistributionError {
    fn from(err: io::Error) -> UpdateDistributionError {
        UpdateDistributionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDistributionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDistributionError {
    fn description(&self) -> &str {
        match *self {
            UpdateDistributionError::AccessDenied(ref cause) => cause,
            UpdateDistributionError::CNAMEAlreadyExists(ref cause) => cause,
            UpdateDistributionError::IllegalUpdate(ref cause) => cause,
            UpdateDistributionError::InconsistentQuantities(ref cause) => cause,
            UpdateDistributionError::InvalidArgument(ref cause) => cause,
            UpdateDistributionError::InvalidDefaultRootObject(ref cause) => cause,
            UpdateDistributionError::InvalidErrorCode(ref cause) => cause,
            UpdateDistributionError::InvalidForwardCookies(ref cause) => cause,
            UpdateDistributionError::InvalidGeoRestrictionParameter(ref cause) => cause,
            UpdateDistributionError::InvalidHeadersForS3Origin(ref cause) => cause,
            UpdateDistributionError::InvalidIfMatchVersion(ref cause) => cause,
            UpdateDistributionError::InvalidLambdaFunctionAssociation(ref cause) => cause,
            UpdateDistributionError::InvalidLocationCode(ref cause) => cause,
            UpdateDistributionError::InvalidMinimumProtocolVersion(ref cause) => cause,
            UpdateDistributionError::InvalidOriginAccessIdentity(ref cause) => cause,
            UpdateDistributionError::InvalidOriginKeepaliveTimeout(ref cause) => cause,
            UpdateDistributionError::InvalidOriginReadTimeout(ref cause) => cause,
            UpdateDistributionError::InvalidQueryStringParameters(ref cause) => cause,
            UpdateDistributionError::InvalidRelativePath(ref cause) => cause,
            UpdateDistributionError::InvalidRequiredProtocol(ref cause) => cause,
            UpdateDistributionError::InvalidResponseCode(ref cause) => cause,
            UpdateDistributionError::InvalidTTLOrder(ref cause) => cause,
            UpdateDistributionError::InvalidViewerCertificate(ref cause) => cause,
            UpdateDistributionError::InvalidWebACLId(ref cause) => cause,
            UpdateDistributionError::MissingBody(ref cause) => cause,
            UpdateDistributionError::NoSuchDistribution(ref cause) => cause,
            UpdateDistributionError::NoSuchOrigin(ref cause) => cause,
            UpdateDistributionError::PreconditionFailed(ref cause) => cause,
            UpdateDistributionError::TooManyCacheBehaviors(ref cause) => cause,
            UpdateDistributionError::TooManyCertificates(ref cause) => cause,
            UpdateDistributionError::TooManyCookieNamesInWhiteList(ref cause) => cause,
            UpdateDistributionError::TooManyDistributionCNAMEs(ref cause) => cause,
            UpdateDistributionError::TooManyDistributionsWithLambdaAssociations(ref cause) => cause,
            UpdateDistributionError::TooManyHeadersInForwardedValues(ref cause) => cause,
            UpdateDistributionError::TooManyLambdaFunctionAssociations(ref cause) => cause,
            UpdateDistributionError::TooManyOriginCustomHeaders(ref cause) => cause,
            UpdateDistributionError::TooManyOrigins(ref cause) => cause,
            UpdateDistributionError::TooManyQueryStringParameters(ref cause) => cause,
            UpdateDistributionError::TooManyTrustedSigners(ref cause) => cause,
            UpdateDistributionError::TrustedSignerDoesNotExist(ref cause) => cause,
            UpdateDistributionError::Validation(ref cause) => cause,
            UpdateDistributionError::Credentials(ref err) => err.description(),
            UpdateDistributionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDistributionError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateStreamingDistributionError {
    pub fn from_body(body: &str) -> UpdateStreamingDistributionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AccessDenied" => UpdateStreamingDistributionError::AccessDenied(String::from(
                    parsed_error.message,
                )),
                "CNAMEAlreadyExists" => UpdateStreamingDistributionError::CNAMEAlreadyExists(
                    String::from(parsed_error.message),
                ),
                "IllegalUpdate" => UpdateStreamingDistributionError::IllegalUpdate(String::from(
                    parsed_error.message,
                )),
                "InconsistentQuantities" => {
                    UpdateStreamingDistributionError::InconsistentQuantities(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidArgument" => UpdateStreamingDistributionError::InvalidArgument(
                    String::from(parsed_error.message),
                ),
                "InvalidIfMatchVersion" => UpdateStreamingDistributionError::InvalidIfMatchVersion(
                    String::from(parsed_error.message),
                ),
                "InvalidOriginAccessIdentity" => {
                    UpdateStreamingDistributionError::InvalidOriginAccessIdentity(String::from(
                        parsed_error.message,
                    ))
                }
                "MissingBody" => UpdateStreamingDistributionError::MissingBody(String::from(
                    parsed_error.message,
                )),
                "NoSuchStreamingDistribution" => {
                    UpdateStreamingDistributionError::NoSuchStreamingDistribution(String::from(
                        parsed_error.message,
                    ))
                }
                "PreconditionFailed" => UpdateStreamingDistributionError::PreconditionFailed(
                    String::from(parsed_error.message),
                ),
                "TooManyStreamingDistributionCNAMEs" => {
                    UpdateStreamingDistributionError::TooManyStreamingDistributionCNAMEs(
                        String::from(parsed_error.message),
                    )
                }
                "TooManyTrustedSigners" => UpdateStreamingDistributionError::TooManyTrustedSigners(
                    String::from(parsed_error.message),
                ),
                "TrustedSignerDoesNotExist" => {
                    UpdateStreamingDistributionError::TrustedSignerDoesNotExist(String::from(
                        parsed_error.message,
                    ))
                }
                _ => UpdateStreamingDistributionError::Unknown(String::from(body)),
            },
            Err(_) => UpdateStreamingDistributionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateStreamingDistributionError {
    fn from(err: XmlParseError) -> UpdateStreamingDistributionError {
        let XmlParseError(message) = err;
        UpdateStreamingDistributionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateStreamingDistributionError {
    fn from(err: CredentialsError) -> UpdateStreamingDistributionError {
        UpdateStreamingDistributionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateStreamingDistributionError {
    fn from(err: HttpDispatchError) -> UpdateStreamingDistributionError {
        UpdateStreamingDistributionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateStreamingDistributionError {
    fn from(err: io::Error) -> UpdateStreamingDistributionError {
        UpdateStreamingDistributionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateStreamingDistributionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateStreamingDistributionError {
    fn description(&self) -> &str {
        match *self {
            UpdateStreamingDistributionError::AccessDenied(ref cause) => cause,
            UpdateStreamingDistributionError::CNAMEAlreadyExists(ref cause) => cause,
            UpdateStreamingDistributionError::IllegalUpdate(ref cause) => cause,
            UpdateStreamingDistributionError::InconsistentQuantities(ref cause) => cause,
            UpdateStreamingDistributionError::InvalidArgument(ref cause) => cause,
            UpdateStreamingDistributionError::InvalidIfMatchVersion(ref cause) => cause,
            UpdateStreamingDistributionError::InvalidOriginAccessIdentity(ref cause) => cause,
            UpdateStreamingDistributionError::MissingBody(ref cause) => cause,
            UpdateStreamingDistributionError::NoSuchStreamingDistribution(ref cause) => cause,
            UpdateStreamingDistributionError::PreconditionFailed(ref cause) => cause,
            UpdateStreamingDistributionError::TooManyStreamingDistributionCNAMEs(ref cause) => {
                cause
            }
            UpdateStreamingDistributionError::TooManyTrustedSigners(ref cause) => cause,
            UpdateStreamingDistributionError::TrustedSignerDoesNotExist(ref cause) => cause,
            UpdateStreamingDistributionError::Validation(ref cause) => cause,
            UpdateStreamingDistributionError::Credentials(ref err) => err.description(),
            UpdateStreamingDistributionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateStreamingDistributionError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the CloudFront API. CloudFront clients implement this trait.
pub trait CloudFront {
    /// <p>Creates a new origin access identity. If you're using Amazon S3 for your origin, you can use an origin access identity to require users to access your content using a CloudFront URL instead of the Amazon S3 URL. For more information about how to use origin access identities, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    fn create_cloud_front_origin_access_identity(
        &self,
        input: CreateCloudFrontOriginAccessIdentityRequest,
    ) -> RusotoFuture<
        CreateCloudFrontOriginAccessIdentityResult,
        CreateCloudFrontOriginAccessIdentityError,
    >;

    /// <p>Creates a new web distribution. Send a <code>POST</code> request to the <code>/<i>CloudFront API version</i>/distribution</code>/<code>distribution ID</code> resource.</p>
    fn create_distribution(
        &self,
        input: CreateDistributionRequest,
    ) -> RusotoFuture<CreateDistributionResult, CreateDistributionError>;

    /// <p>Create a new distribution with tags.</p>
    fn create_distribution_with_tags(
        &self,
        input: CreateDistributionWithTagsRequest,
    ) -> RusotoFuture<CreateDistributionWithTagsResult, CreateDistributionWithTagsError>;

    /// <p>Create a new invalidation. </p>
    fn create_invalidation(
        &self,
        input: CreateInvalidationRequest,
    ) -> RusotoFuture<CreateInvalidationResult, CreateInvalidationError>;

    /// <p><p>Creates a new RMTP distribution. An RTMP distribution is similar to a web distribution, but an RTMP distribution streams media files using the Adobe Real-Time Messaging Protocol (RTMP) instead of serving files using HTTP. </p> <p>To create a new web distribution, submit a <code>POST</code> request to the <i>CloudFront API version</i>/distribution resource. The request body must include a document with a <i>StreamingDistributionConfig</i> element. The response echoes the <code>StreamingDistributionConfig</code> element and returns other information about the RTMP distribution.</p> <p>To get the status of your request, use the <i>GET StreamingDistribution</i> API action. When the value of <code>Enabled</code> is <code>true</code> and the value of <code>Status</code> is <code>Deployed</code>, your distribution is ready. A distribution usually deploys in less than 15 minutes.</p> <p>For more information about web distributions, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-rtmp.html">Working with RTMP Distributions</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <important> <p>Beginning with the 2012-05-05 version of the CloudFront API, we made substantial changes to the format of the XML document that you include in the request body when you create or update a web distribution or an RTMP distribution, and when you invalidate objects. With previous versions of the API, we discovered that it was too easy to accidentally delete one or more values for an element that accepts multiple values, for example, CNAMEs and trusted signers. Our changes for the 2012-05-05 release are intended to prevent these accidental deletions and to notify you when there&#39;s a mismatch between the number of values you say you&#39;re specifying in the <code>Quantity</code> element and the number of values specified.</p> </important></p>
    fn create_streaming_distribution(
        &self,
        input: CreateStreamingDistributionRequest,
    ) -> RusotoFuture<CreateStreamingDistributionResult, CreateStreamingDistributionError>;

    /// <p>Create a new streaming distribution with tags.</p>
    fn create_streaming_distribution_with_tags(
        &self,
        input: CreateStreamingDistributionWithTagsRequest,
    ) -> RusotoFuture<
        CreateStreamingDistributionWithTagsResult,
        CreateStreamingDistributionWithTagsError,
    >;

    /// <p>Delete an origin access identity. </p>
    fn delete_cloud_front_origin_access_identity(
        &self,
        input: DeleteCloudFrontOriginAccessIdentityRequest,
    ) -> RusotoFuture<(), DeleteCloudFrontOriginAccessIdentityError>;

    /// <p>Delete a distribution. </p>
    fn delete_distribution(
        &self,
        input: DeleteDistributionRequest,
    ) -> RusotoFuture<(), DeleteDistributionError>;

    fn delete_service_linked_role(
        &self,
        input: DeleteServiceLinkedRoleRequest,
    ) -> RusotoFuture<(), DeleteServiceLinkedRoleError>;

    /// <p>Delete a streaming distribution. To delete an RTMP distribution using the CloudFront API, perform the following steps.</p> <p> <b>To delete an RTMP distribution using the CloudFront API</b>:</p> <ol> <li> <p>Disable the RTMP distribution.</p> </li> <li> <p>Submit a <code>GET Streaming Distribution Config</code> request to get the current configuration and the <code>Etag</code> header for the distribution. </p> </li> <li> <p>Update the XML document that was returned in the response to your <code>GET Streaming Distribution Config</code> request to change the value of <code>Enabled</code> to <code>false</code>.</p> </li> <li> <p>Submit a <code>PUT Streaming Distribution Config</code> request to update the configuration for your distribution. In the request body, include the XML document that you updated in Step 3. Then set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Streaming Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to the <code>PUT Streaming Distribution Config</code> request to confirm that the distribution was successfully disabled.</p> </li> <li> <p>Submit a <code>GET Streaming Distribution Config</code> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> </li> <li> <p>Submit a <code>DELETE Streaming Distribution</code> request. Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Streaming Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to your <code>DELETE Streaming Distribution</code> request to confirm that the distribution was successfully deleted.</p> </li> </ol> <p>For information about deleting a distribution using the CloudFront console, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/HowToDeleteDistribution.html">Deleting a Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    fn delete_streaming_distribution(
        &self,
        input: DeleteStreamingDistributionRequest,
    ) -> RusotoFuture<(), DeleteStreamingDistributionError>;

    /// <p>Get the information about an origin access identity. </p>
    fn get_cloud_front_origin_access_identity(
        &self,
        input: GetCloudFrontOriginAccessIdentityRequest,
    ) -> RusotoFuture<GetCloudFrontOriginAccessIdentityResult, GetCloudFrontOriginAccessIdentityError>;

    /// <p>Get the configuration information about an origin access identity. </p>
    fn get_cloud_front_origin_access_identity_config(
        &self,
        input: GetCloudFrontOriginAccessIdentityConfigRequest,
    ) -> RusotoFuture<
        GetCloudFrontOriginAccessIdentityConfigResult,
        GetCloudFrontOriginAccessIdentityConfigError,
    >;

    /// <p>Get the information about a distribution. </p>
    fn get_distribution(
        &self,
        input: GetDistributionRequest,
    ) -> RusotoFuture<GetDistributionResult, GetDistributionError>;

    /// <p>Get the configuration information about a distribution. </p>
    fn get_distribution_config(
        &self,
        input: GetDistributionConfigRequest,
    ) -> RusotoFuture<GetDistributionConfigResult, GetDistributionConfigError>;

    /// <p>Get the information about an invalidation. </p>
    fn get_invalidation(
        &self,
        input: GetInvalidationRequest,
    ) -> RusotoFuture<GetInvalidationResult, GetInvalidationError>;

    /// <p>Gets information about a specified RTMP distribution, including the distribution configuration.</p>
    fn get_streaming_distribution(
        &self,
        input: GetStreamingDistributionRequest,
    ) -> RusotoFuture<GetStreamingDistributionResult, GetStreamingDistributionError>;

    /// <p>Get the configuration information about a streaming distribution. </p>
    fn get_streaming_distribution_config(
        &self,
        input: GetStreamingDistributionConfigRequest,
    ) -> RusotoFuture<GetStreamingDistributionConfigResult, GetStreamingDistributionConfigError>;

    /// <p>Lists origin access identities.</p>
    fn list_cloud_front_origin_access_identities(
        &self,
        input: ListCloudFrontOriginAccessIdentitiesRequest,
    ) -> RusotoFuture<
        ListCloudFrontOriginAccessIdentitiesResult,
        ListCloudFrontOriginAccessIdentitiesError,
    >;

    /// <p>List distributions. </p>
    fn list_distributions(
        &self,
        input: ListDistributionsRequest,
    ) -> RusotoFuture<ListDistributionsResult, ListDistributionsError>;

    /// <p>List the distributions that are associated with a specified AWS WAF web ACL. </p>
    fn list_distributions_by_web_acl_id(
        &self,
        input: ListDistributionsByWebACLIdRequest,
    ) -> RusotoFuture<ListDistributionsByWebACLIdResult, ListDistributionsByWebACLIdError>;

    /// <p>Lists invalidation batches. </p>
    fn list_invalidations(
        &self,
        input: ListInvalidationsRequest,
    ) -> RusotoFuture<ListInvalidationsResult, ListInvalidationsError>;

    /// <p>List streaming distributions. </p>
    fn list_streaming_distributions(
        &self,
        input: ListStreamingDistributionsRequest,
    ) -> RusotoFuture<ListStreamingDistributionsResult, ListStreamingDistributionsError>;

    /// <p>List tags for a CloudFront resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResult, ListTagsForResourceError>;

    /// <p>Add tags to a CloudFront resource.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError>;

    /// <p>Remove tags from a CloudFront resource.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError>;

    /// <p>Update an origin access identity. </p>
    fn update_cloud_front_origin_access_identity(
        &self,
        input: UpdateCloudFrontOriginAccessIdentityRequest,
    ) -> RusotoFuture<
        UpdateCloudFrontOriginAccessIdentityResult,
        UpdateCloudFrontOriginAccessIdentityError,
    >;

    /// <p><p>Updates the configuration for a web distribution. Perform the following steps.</p> <p>For information about updating a distribution using the CloudFront console, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-creating-console.html">Creating or Updating a Web Distribution Using the CloudFront Console </a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p> <b>To update a web distribution using the CloudFront API</b> </p> <ol> <li> <p>Submit a <a>GetDistributionConfig</a> request to get the current configuration and an <code>Etag</code> header for the distribution.</p> <note> <p>If you update the distribution again, you need to get a new <code>Etag</code> header.</p> </note> </li> <li> <p>Update the XML document that was returned in the response to your <code>GetDistributionConfig</code> request to include the desired changes. You can&#39;t change the value of <code>CallerReference</code>. If you try to change this value, CloudFront returns an <code>IllegalUpdate</code> error.</p> <important> <p>The new configuration replaces the existing configuration; the values that you specify in an <code>UpdateDistribution</code> request are not merged into the existing configuration. When you add, delete, or replace values in an element that allows multiple values (for example, <code>CNAME</code>), you must specify all of the values that you want to appear in the updated distribution. In addition, you must update the corresponding <code>Quantity</code> element.</p> </important> </li> <li> <p>Submit an <code>UpdateDistribution</code> request to update the configuration for your distribution:</p> <ul> <li> <p>In the request body, include the XML document that you updated in Step 2. The request body must include an XML document with a <code>DistributionConfig</code> element.</p> </li> <li> <p>Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GetDistributionConfig</code> request in Step 1.</p> </li> </ul> </li> <li> <p>Review the response to the <code>UpdateDistribution</code> request to confirm that the configuration was successfully updated.</p> </li> <li> <p>Optional: Submit a <a>GetDistribution</a> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> <important> <p>Beginning with the 2012-05-05 version of the CloudFront API, we made substantial changes to the format of the XML document that you include in the request body when you create or update a distribution. With previous versions of the API, we discovered that it was too easy to accidentally delete one or more values for an element that accepts multiple values, for example, CNAMEs and trusted signers. Our changes for the 2012-05-05 release are intended to prevent these accidental deletions and to notify you when there&#39;s a mismatch between the number of values you say you&#39;re specifying in the <code>Quantity</code> element and the number of values you&#39;re actually specifying.</p> </important> </li> </ol></p>
    fn update_distribution(
        &self,
        input: UpdateDistributionRequest,
    ) -> RusotoFuture<UpdateDistributionResult, UpdateDistributionError>;

    /// <p>Update a streaming distribution. </p>
    fn update_streaming_distribution(
        &self,
        input: UpdateStreamingDistributionRequest,
    ) -> RusotoFuture<UpdateStreamingDistributionResult, UpdateStreamingDistributionError>;
}
/// A client for the CloudFront API.
pub struct CloudFrontClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl CloudFrontClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> CloudFrontClient {
        CloudFrontClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> CloudFrontClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CloudFrontClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> CloudFront for CloudFrontClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Creates a new origin access identity. If you're using Amazon S3 for your origin, you can use an origin access identity to require users to access your content using a CloudFront URL instead of the Amazon S3 URL. For more information about how to use origin access identities, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    fn create_cloud_front_origin_access_identity(
        &self,
        input: CreateCloudFrontOriginAccessIdentityRequest,
    ) -> RusotoFuture<
        CreateCloudFrontOriginAccessIdentityResult,
        CreateCloudFrontOriginAccessIdentityError,
    > {
        let request_uri = "/2017-03-25/origin-access-identity/cloudfront";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CloudFrontOriginAccessIdentityConfigSerializer::serialize(
            &mut writer,
            "CloudFrontOriginAccessIdentityConfig",
            &input.cloud_front_origin_access_identity_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateCloudFrontOriginAccessIdentityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateCloudFrontOriginAccessIdentityResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        CreateCloudFrontOriginAccessIdentityResultDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                if let Some(location) = response.headers.get("Location") {
                    let value = location.to_owned();
                    result.location = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new web distribution. Send a <code>POST</code> request to the <code>/<i>CloudFront API version</i>/distribution</code>/<code>distribution ID</code> resource.</p>
    #[allow(unused_variables, warnings)]
    fn create_distribution(
        &self,
        input: CreateDistributionRequest,
    ) -> RusotoFuture<CreateDistributionResult, CreateDistributionError> {
        let request_uri = "/2017-03-25/distribution";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        DistributionConfigSerializer::serialize(
            &mut writer,
            "DistributionConfig",
            &input.distribution_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateDistributionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateDistributionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CreateDistributionResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                if let Some(location) = response.headers.get("Location") {
                    let value = location.to_owned();
                    result.location = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Create a new distribution with tags.</p>
    #[allow(unused_variables, warnings)]
    fn create_distribution_with_tags(
        &self,
        input: CreateDistributionWithTagsRequest,
    ) -> RusotoFuture<CreateDistributionWithTagsResult, CreateDistributionWithTagsError> {
        let request_uri = "/2017-03-25/distribution";

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

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateDistributionWithTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateDistributionWithTagsResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CreateDistributionWithTagsResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                if let Some(location) = response.headers.get("Location") {
                    let value = location.to_owned();
                    result.location = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Create a new invalidation. </p>
    #[allow(unused_variables, warnings)]
    fn create_invalidation(
        &self,
        input: CreateInvalidationRequest,
    ) -> RusotoFuture<CreateInvalidationResult, CreateInvalidationError> {
        let request_uri = format!(
            "/2017-03-25/distribution/{distribution_id}/invalidation",
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

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateInvalidationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateInvalidationResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CreateInvalidationResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(location) = response.headers.get("Location") {
                    let value = location.to_owned();
                    result.location = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a new RMTP distribution. An RTMP distribution is similar to a web distribution, but an RTMP distribution streams media files using the Adobe Real-Time Messaging Protocol (RTMP) instead of serving files using HTTP. </p> <p>To create a new web distribution, submit a <code>POST</code> request to the <i>CloudFront API version</i>/distribution resource. The request body must include a document with a <i>StreamingDistributionConfig</i> element. The response echoes the <code>StreamingDistributionConfig</code> element and returns other information about the RTMP distribution.</p> <p>To get the status of your request, use the <i>GET StreamingDistribution</i> API action. When the value of <code>Enabled</code> is <code>true</code> and the value of <code>Status</code> is <code>Deployed</code>, your distribution is ready. A distribution usually deploys in less than 15 minutes.</p> <p>For more information about web distributions, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-rtmp.html">Working with RTMP Distributions</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <important> <p>Beginning with the 2012-05-05 version of the CloudFront API, we made substantial changes to the format of the XML document that you include in the request body when you create or update a web distribution or an RTMP distribution, and when you invalidate objects. With previous versions of the API, we discovered that it was too easy to accidentally delete one or more values for an element that accepts multiple values, for example, CNAMEs and trusted signers. Our changes for the 2012-05-05 release are intended to prevent these accidental deletions and to notify you when there&#39;s a mismatch between the number of values you say you&#39;re specifying in the <code>Quantity</code> element and the number of values specified.</p> </important></p>
    #[allow(unused_variables, warnings)]
    fn create_streaming_distribution(
        &self,
        input: CreateStreamingDistributionRequest,
    ) -> RusotoFuture<CreateStreamingDistributionResult, CreateStreamingDistributionError> {
        let request_uri = "/2017-03-25/streaming-distribution";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        StreamingDistributionConfigSerializer::serialize(
            &mut writer,
            "StreamingDistributionConfig",
            &input.streaming_distribution_config,
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateStreamingDistributionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateStreamingDistributionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CreateStreamingDistributionResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                if let Some(location) = response.headers.get("Location") {
                    let value = location.to_owned();
                    result.location = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Create a new streaming distribution with tags.</p>
    #[allow(unused_variables, warnings)]
    fn create_streaming_distribution_with_tags(
        &self,
        input: CreateStreamingDistributionWithTagsRequest,
    ) -> RusotoFuture<
        CreateStreamingDistributionWithTagsResult,
        CreateStreamingDistributionWithTagsError,
    > {
        let request_uri = "/2017-03-25/streaming-distribution";

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

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateStreamingDistributionWithTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateStreamingDistributionWithTagsResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        CreateStreamingDistributionWithTagsResultDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                if let Some(location) = response.headers.get("Location") {
                    let value = location.to_owned();
                    result.location = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Delete an origin access identity. </p>
    #[allow(unused_variables, warnings)]
    fn delete_cloud_front_origin_access_identity(
        &self,
        input: DeleteCloudFrontOriginAccessIdentityRequest,
    ) -> RusotoFuture<(), DeleteCloudFrontOriginAccessIdentityError> {
        let request_uri = format!(
            "/2017-03-25/origin-access-identity/cloudfront/{id}",
            id = input.id
        );

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteCloudFrontOriginAccessIdentityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Delete a distribution. </p>
    #[allow(unused_variables, warnings)]
    fn delete_distribution(
        &self,
        input: DeleteDistributionRequest,
    ) -> RusotoFuture<(), DeleteDistributionError> {
        let request_uri = format!("/2017-03-25/distribution/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDistributionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    #[allow(unused_variables, warnings)]
    fn delete_service_linked_role(
        &self,
        input: DeleteServiceLinkedRoleRequest,
    ) -> RusotoFuture<(), DeleteServiceLinkedRoleError> {
        let request_uri = format!(
            "/2017-03-25/service-linked-role/{role_name}",
            role_name = input.role_name
        );

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteServiceLinkedRoleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Delete a streaming distribution. To delete an RTMP distribution using the CloudFront API, perform the following steps.</p> <p> <b>To delete an RTMP distribution using the CloudFront API</b>:</p> <ol> <li> <p>Disable the RTMP distribution.</p> </li> <li> <p>Submit a <code>GET Streaming Distribution Config</code> request to get the current configuration and the <code>Etag</code> header for the distribution. </p> </li> <li> <p>Update the XML document that was returned in the response to your <code>GET Streaming Distribution Config</code> request to change the value of <code>Enabled</code> to <code>false</code>.</p> </li> <li> <p>Submit a <code>PUT Streaming Distribution Config</code> request to update the configuration for your distribution. In the request body, include the XML document that you updated in Step 3. Then set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Streaming Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to the <code>PUT Streaming Distribution Config</code> request to confirm that the distribution was successfully disabled.</p> </li> <li> <p>Submit a <code>GET Streaming Distribution Config</code> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> </li> <li> <p>Submit a <code>DELETE Streaming Distribution</code> request. Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GET Streaming Distribution Config</code> request in Step 2.</p> </li> <li> <p>Review the response to your <code>DELETE Streaming Distribution</code> request to confirm that the distribution was successfully deleted.</p> </li> </ol> <p>For information about deleting a distribution using the CloudFront console, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/HowToDeleteDistribution.html">Deleting a Distribution</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    fn delete_streaming_distribution(
        &self,
        input: DeleteStreamingDistributionRequest,
    ) -> RusotoFuture<(), DeleteStreamingDistributionError> {
        let request_uri = format!("/2017-03-25/streaming-distribution/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "cloudfront", &self.region, &request_uri);

        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteStreamingDistributionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Get the information about an origin access identity. </p>
    #[allow(unused_variables, warnings)]
    fn get_cloud_front_origin_access_identity(
        &self,
        input: GetCloudFrontOriginAccessIdentityRequest,
    ) -> RusotoFuture<GetCloudFrontOriginAccessIdentityResult, GetCloudFrontOriginAccessIdentityError>
    {
        let request_uri = format!(
            "/2017-03-25/origin-access-identity/cloudfront/{id}",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetCloudFrontOriginAccessIdentityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetCloudFrontOriginAccessIdentityResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        GetCloudFrontOriginAccessIdentityResultDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Get the configuration information about an origin access identity. </p>
    #[allow(unused_variables, warnings)]
    fn get_cloud_front_origin_access_identity_config(
        &self,
        input: GetCloudFrontOriginAccessIdentityConfigRequest,
    ) -> RusotoFuture<
        GetCloudFrontOriginAccessIdentityConfigResult,
        GetCloudFrontOriginAccessIdentityConfigError,
    > {
        let request_uri = format!(
            "/2017-03-25/origin-access-identity/cloudfront/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetCloudFrontOriginAccessIdentityConfigError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetCloudFrontOriginAccessIdentityConfigResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        GetCloudFrontOriginAccessIdentityConfigResultDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Get the information about a distribution. </p>
    #[allow(unused_variables, warnings)]
    fn get_distribution(
        &self,
        input: GetDistributionRequest,
    ) -> RusotoFuture<GetDistributionResult, GetDistributionError> {
        let request_uri = format!("/2017-03-25/distribution/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDistributionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetDistributionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetDistributionResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Get the configuration information about a distribution. </p>
    #[allow(unused_variables, warnings)]
    fn get_distribution_config(
        &self,
        input: GetDistributionConfigRequest,
    ) -> RusotoFuture<GetDistributionConfigResult, GetDistributionConfigError> {
        let request_uri = format!("/2017-03-25/distribution/{id}/config", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDistributionConfigError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetDistributionConfigResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetDistributionConfigResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Get the information about an invalidation. </p>
    #[allow(unused_variables, warnings)]
    fn get_invalidation(
        &self,
        input: GetInvalidationRequest,
    ) -> RusotoFuture<GetInvalidationResult, GetInvalidationError> {
        let request_uri = format!(
            "/2017-03-25/distribution/{distribution_id}/invalidation/{id}",
            distribution_id = input.distribution_id,
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetInvalidationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetInvalidationResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetInvalidationResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about a specified RTMP distribution, including the distribution configuration.</p>
    #[allow(unused_variables, warnings)]
    fn get_streaming_distribution(
        &self,
        input: GetStreamingDistributionRequest,
    ) -> RusotoFuture<GetStreamingDistributionResult, GetStreamingDistributionError> {
        let request_uri = format!("/2017-03-25/streaming-distribution/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetStreamingDistributionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetStreamingDistributionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetStreamingDistributionResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Get the configuration information about a streaming distribution. </p>
    #[allow(unused_variables, warnings)]
    fn get_streaming_distribution_config(
        &self,
        input: GetStreamingDistributionConfigRequest,
    ) -> RusotoFuture<GetStreamingDistributionConfigResult, GetStreamingDistributionConfigError>
    {
        let request_uri = format!(
            "/2017-03-25/streaming-distribution/{id}/config",
            id = input.id
        );

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetStreamingDistributionConfigError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetStreamingDistributionConfigResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        GetStreamingDistributionConfigResultDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists origin access identities.</p>
    #[allow(unused_variables, warnings)]
    fn list_cloud_front_origin_access_identities(
        &self,
        input: ListCloudFrontOriginAccessIdentitiesRequest,
    ) -> RusotoFuture<
        ListCloudFrontOriginAccessIdentitiesResult,
        ListCloudFrontOriginAccessIdentitiesError,
    > {
        let request_uri = "/2017-03-25/origin-access-identity/cloudfront";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListCloudFrontOriginAccessIdentitiesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListCloudFrontOriginAccessIdentitiesResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        ListCloudFrontOriginAccessIdentitiesResultDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>List distributions. </p>
    #[allow(unused_variables, warnings)]
    fn list_distributions(
        &self,
        input: ListDistributionsRequest,
    ) -> RusotoFuture<ListDistributionsResult, ListDistributionsError> {
        let request_uri = "/2017-03-25/distribution";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListDistributionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListDistributionsResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListDistributionsResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>List the distributions that are associated with a specified AWS WAF web ACL. </p>
    #[allow(unused_variables, warnings)]
    fn list_distributions_by_web_acl_id(
        &self,
        input: ListDistributionsByWebACLIdRequest,
    ) -> RusotoFuture<ListDistributionsByWebACLIdResult, ListDistributionsByWebACLIdError> {
        let request_uri = format!(
            "/2017-03-25/distributionsByWebACLId/{web_acl_id}",
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

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListDistributionsByWebACLIdError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListDistributionsByWebACLIdResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListDistributionsByWebACLIdResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists invalidation batches. </p>
    #[allow(unused_variables, warnings)]
    fn list_invalidations(
        &self,
        input: ListInvalidationsRequest,
    ) -> RusotoFuture<ListInvalidationsResult, ListInvalidationsError> {
        let request_uri = format!(
            "/2017-03-25/distribution/{distribution_id}/invalidation",
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

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListInvalidationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListInvalidationsResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListInvalidationsResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>List streaming distributions. </p>
    #[allow(unused_variables, warnings)]
    fn list_streaming_distributions(
        &self,
        input: ListStreamingDistributionsRequest,
    ) -> RusotoFuture<ListStreamingDistributionsResult, ListStreamingDistributionsError> {
        let request_uri = "/2017-03-25/streaming-distribution";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListStreamingDistributionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListStreamingDistributionsResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListStreamingDistributionsResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>List tags for a CloudFront resource.</p>
    #[allow(unused_variables, warnings)]
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResult, ListTagsForResourceError> {
        let request_uri = "/2017-03-25/tagging";

        let mut request = SignedRequest::new("GET", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("Resource", &input.resource);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsForResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListTagsForResourceResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListTagsForResourceResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Add tags to a CloudFront resource.</p>
    #[allow(unused_variables, warnings)]
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError> {
        let request_uri = "/2017-03-25/tagging";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("Resource", &input.resource);
        params.put("Operation", "Tag");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        TagsSerializer::serialize(&mut writer, "Tags", &input.tags);
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Remove tags from a CloudFront resource.</p>
    #[allow(unused_variables, warnings)]
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError> {
        let request_uri = "/2017-03-25/tagging";

        let mut request = SignedRequest::new("POST", "cloudfront", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("Resource", &input.resource);
        params.put("Operation", "Untag");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        TagKeysSerializer::serialize(&mut writer, "TagKeys", &input.tag_keys);
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UntagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Update an origin access identity. </p>
    #[allow(unused_variables, warnings)]
    fn update_cloud_front_origin_access_identity(
        &self,
        input: UpdateCloudFrontOriginAccessIdentityRequest,
    ) -> RusotoFuture<
        UpdateCloudFrontOriginAccessIdentityResult,
        UpdateCloudFrontOriginAccessIdentityError,
    > {
        let request_uri = format!(
            "/2017-03-25/origin-access-identity/cloudfront/{id}/config",
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

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateCloudFrontOriginAccessIdentityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = UpdateCloudFrontOriginAccessIdentityResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        UpdateCloudFrontOriginAccessIdentityResultDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Updates the configuration for a web distribution. Perform the following steps.</p> <p>For information about updating a distribution using the CloudFront console, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-creating-console.html">Creating or Updating a Web Distribution Using the CloudFront Console </a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p> <b>To update a web distribution using the CloudFront API</b> </p> <ol> <li> <p>Submit a <a>GetDistributionConfig</a> request to get the current configuration and an <code>Etag</code> header for the distribution.</p> <note> <p>If you update the distribution again, you need to get a new <code>Etag</code> header.</p> </note> </li> <li> <p>Update the XML document that was returned in the response to your <code>GetDistributionConfig</code> request to include the desired changes. You can&#39;t change the value of <code>CallerReference</code>. If you try to change this value, CloudFront returns an <code>IllegalUpdate</code> error.</p> <important> <p>The new configuration replaces the existing configuration; the values that you specify in an <code>UpdateDistribution</code> request are not merged into the existing configuration. When you add, delete, or replace values in an element that allows multiple values (for example, <code>CNAME</code>), you must specify all of the values that you want to appear in the updated distribution. In addition, you must update the corresponding <code>Quantity</code> element.</p> </important> </li> <li> <p>Submit an <code>UpdateDistribution</code> request to update the configuration for your distribution:</p> <ul> <li> <p>In the request body, include the XML document that you updated in Step 2. The request body must include an XML document with a <code>DistributionConfig</code> element.</p> </li> <li> <p>Set the value of the HTTP <code>If-Match</code> header to the value of the <code>ETag</code> header that CloudFront returned when you submitted the <code>GetDistributionConfig</code> request in Step 1.</p> </li> </ul> </li> <li> <p>Review the response to the <code>UpdateDistribution</code> request to confirm that the configuration was successfully updated.</p> </li> <li> <p>Optional: Submit a <a>GetDistribution</a> request to confirm that your changes have propagated. When propagation is complete, the value of <code>Status</code> is <code>Deployed</code>.</p> <important> <p>Beginning with the 2012-05-05 version of the CloudFront API, we made substantial changes to the format of the XML document that you include in the request body when you create or update a distribution. With previous versions of the API, we discovered that it was too easy to accidentally delete one or more values for an element that accepts multiple values, for example, CNAMEs and trusted signers. Our changes for the 2012-05-05 release are intended to prevent these accidental deletions and to notify you when there&#39;s a mismatch between the number of values you say you&#39;re specifying in the <code>Quantity</code> element and the number of values you&#39;re actually specifying.</p> </important> </li> </ol></p>
    #[allow(unused_variables, warnings)]
    fn update_distribution(
        &self,
        input: UpdateDistributionRequest,
    ) -> RusotoFuture<UpdateDistributionResult, UpdateDistributionError> {
        let request_uri = format!("/2017-03-25/distribution/{id}/config", id = input.id);

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

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDistributionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = UpdateDistributionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(UpdateDistributionResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Update a streaming distribution. </p>
    #[allow(unused_variables, warnings)]
    fn update_streaming_distribution(
        &self,
        input: UpdateStreamingDistributionRequest,
    ) -> RusotoFuture<UpdateStreamingDistributionResult, UpdateStreamingDistributionError> {
        let request_uri = format!(
            "/2017-03-25/streaming-distribution/{id}/config",
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

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateStreamingDistributionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = UpdateStreamingDistributionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(UpdateStreamingDistributionResultDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[test]
    fn test_parse_valid_cloudfront_get_cloud_front_origin_access_identity() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-get-cloud-front-origin-access-identity.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetCloudFrontOriginAccessIdentityRequest::default();
        let result = client
            .get_cloud_front_origin_access_identity(request)
            .sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudfront_get_distribution() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-get-distribution.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetDistributionRequest::default();
        let result = client.get_distribution(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudfront_get_invalidation() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-get-invalidation.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetInvalidationRequest::default();
        let result = client.get_invalidation(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudfront_get_streaming_distribution() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-get-streaming-distribution.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetStreamingDistributionRequest::default();
        let result = client.get_streaming_distribution(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudfront_list_cloud_front_origin_access_identities() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-list-cloud-front-origin-access-identities.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListCloudFrontOriginAccessIdentitiesRequest::default();
        let result = client
            .list_cloud_front_origin_access_identities(request)
            .sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudfront_list_distributions() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-list-distributions.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListDistributionsRequest::default();
        let result = client.list_distributions(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudfront_list_invalidations() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-list-invalidations.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListInvalidationsRequest::default();
        let result = client.list_invalidations(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudfront_list_streaming_distributions() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-list-streaming-distributions.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListStreamingDistributionsRequest::default();
        let result = client.list_streaming_distributions(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
