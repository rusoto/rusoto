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
/// <p>A complex type that contains the type of limit that you specified in the request and the current value for that limit.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccountLimit {
    /// <p><p>The limit that you requested. Valid values include the following:</p> <ul> <li> <p> <b>MAX<em>HEALTH</em>CHECKS<em>BY</em>OWNER</b>: The maximum number of health checks that you can create using the current account.</p> </li> <li> <p> <b>MAX<em>HOSTED</em>ZONES<em>BY</em>OWNER</b>: The maximum number of hosted zones that you can create using the current account.</p> </li> <li> <p> <b>MAX<em>REUSABLE</em>DELEGATION<em>SETS</em>BY<em>OWNER</b>: The maximum number of reusable delegation sets that you can create using the current account.</p> </li> <li> <p> <b>MAX</em>TRAFFIC<em>POLICIES</em>BY<em>OWNER</b>: The maximum number of traffic policies that you can create using the current account.</p> </li> <li> <p> <b>MAX</em>TRAFFIC<em>POLICY</em>INSTANCES<em>BY</em>OWNER</b>: The maximum number of traffic policy instances that you can create using the current account. (Traffic policy instances are referred to as traffic flow policy records in the Amazon Route 53 console.)</p> </li> </ul></p>
    pub type_: String,
    /// <p>The current value for the limit that is specified by <a>AccountLimit$Type</a>.</p>
    pub value: i64,
}

struct AccountLimitDeserializer;
impl AccountLimitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccountLimit, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AccountLimit::default();

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
                    "Type" => {
                        obj.type_ = try!(AccountLimitTypeDeserializer::deserialize("Type", stack));
                    }
                    "Value" => {
                        obj.value = try!(LimitValueDeserializer::deserialize("Value", stack));
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
struct AccountLimitTypeDeserializer;
impl AccountLimitTypeDeserializer {
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

pub struct AccountLimitTypeSerializer;
impl AccountLimitTypeSerializer {
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

/// <p>A complex type that identifies the CloudWatch alarm that you want Amazon Route 53 health checkers to use to determine whether this health check is healthy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AlarmIdentifier {
    /// <p>The name of the CloudWatch alarm that you want Amazon Route 53 health checkers to use to determine whether this health check is healthy.</p>
    pub name: String,
    /// <p>A complex type that identifies the CloudWatch alarm that you want Amazon Route 53 health checkers to use to determine whether this health check is healthy.</p> <p>For the current list of CloudWatch regions, see <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#cw_region">Amazon CloudWatch</a> in the <i>AWS Regions and Endpoints</i> chapter of the <i>Amazon Web Services General Reference</i>.</p>
    pub region: String,
}

struct AlarmIdentifierDeserializer;
impl AlarmIdentifierDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AlarmIdentifier, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AlarmIdentifier::default();

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
                    "Name" => {
                        obj.name = try!(AlarmNameDeserializer::deserialize("Name", stack));
                    }
                    "Region" => {
                        obj.region =
                            try!(CloudWatchRegionDeserializer::deserialize("Region", stack));
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

pub struct AlarmIdentifierSerializer;
impl AlarmIdentifierSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AlarmIdentifier,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Name"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.name
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Region"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.region
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AlarmNameDeserializer;
impl AlarmNameDeserializer {
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

pub struct AlarmNameSerializer;
impl AlarmNameSerializer {
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

struct AliasHealthEnabledDeserializer;
impl AliasHealthEnabledDeserializer {
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

pub struct AliasHealthEnabledSerializer;
impl AliasHealthEnabledSerializer {
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

/// <p><p> <i>Alias resource record sets only:</i> Information about the CloudFront distribution, Elastic Beanstalk environment, ELB load balancer, Amazon S3 bucket, or Amazon Route 53 resource record set that you&#39;re redirecting queries to. An Elastic Beanstalk environment must have a regionalized subdomain.</p> <p>When creating resource record sets for a private hosted zone, note the following:</p> <ul> <li> <p>Resource record sets can&#39;t be created for CloudFront distributions in a private hosted zone.</p> </li> <li> <p>Creating geolocation alias resource record sets or latency alias resource record sets in a private hosted zone is unsupported.</p> </li> <li> <p>For information about creating failover resource record sets in a private hosted zone, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-private-hosted-zones.html">Configuring Failover in a Private Hosted Zone</a>.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AliasTarget {
    /// <p><p> <i>Alias resource record sets only:</i> The value that you specify depends on where you want to route queries:</p> <dl> <dt>CloudFront distribution</dt> <dd> <p>Specify the domain name that CloudFront assigned when you created your distribution.</p> <p>Your CloudFront distribution must include an alternate domain name that matches the name of the resource record set. For example, if the name of the resource record set is <i>acme.example.com</i>, your CloudFront distribution must include <i>acme.example.com</i> as one of the alternate domain names. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/CNAMEs.html">Using Alternate Domain Names (CNAMEs)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </dd> <dt>Elastic Beanstalk environment</dt> <dd> <p>Specify the <code>CNAME</code> attribute for the environment. (The environment must have a regionalized domain name.) You can use the following methods to get the value of the CNAME attribute:</p> <ul> <li> <p> <i>AWS Management Console</i>: For information about how to get the value by using the console, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/customdomains.html">Using Custom Domains with AWS Elastic Beanstalk</a> in the <i>AWS Elastic Beanstalk Developer Guide</i>.</p> </li> <li> <p> <i>Elastic Beanstalk API</i>: Use the <code>DescribeEnvironments</code> action to get the value of the <code>CNAME</code> attribute. For more information, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/api/API_DescribeEnvironments.html">DescribeEnvironments</a> in the <i>AWS Elastic Beanstalk API Reference</i>.</p> </li> <li> <p> <i>AWS CLI</i>: Use the <code>describe-environments</code> command to get the value of the <code>CNAME</code> attribute. For more information, see <a href="http://docs.aws.amazon.com/cli/latest/reference/elasticbeanstalk/describe-environments.html">describe-environments</a> in the <i>AWS Command Line Interface Reference</i>.</p> </li> </ul> </dd> <dt>ELB load balancer</dt> <dd> <p>Specify the DNS name that is associated with the load balancer. Get the DNS name by using the AWS Management Console, the ELB API, or the AWS CLI. </p> <ul> <li> <p> <b>AWS Management Console</b>: Go to the EC2 page, choose <b>Load Balancers</b> in the navigation pane, choose the load balancer, choose the <b>Description</b> tab, and get the value of the <b>DNS name</b> field. (If you&#39;re routing traffic to a Classic Load Balancer, get the value that begins with <b>dualstack</b>.) </p> </li> <li> <p> <b>Elastic Load Balancing API</b>: Use <code>DescribeLoadBalancers</code> to get the value of <code>DNSName</code>. For more information, see the applicable guide:</p> <ul> <li> <p>Classic Load Balancers: <a href="http://docs.aws.amazon.com/elasticloadbalancing/2012-06-01/APIReference/API_DescribeLoadBalancers.html">DescribeLoadBalancers</a> </p> </li> <li> <p>Application and Network Load Balancers: <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DescribeLoadBalancers.html">DescribeLoadBalancers</a> </p> </li> </ul> </li> <li> <p> <b>AWS CLI</b>: Use <code>describe-load-balancers</code> to get the value of <code>DNSName</code>. For more information, see the applicable guide:</p> <ul> <li> <p>Classic Load Balancers: <a href="http://docs.aws.amazon.com/cli/latest/reference/elb/describe-load-balancers.html">describe-load-balancers</a> </p> </li> <li> <p>Application and Network Load Balancers: <a href="http://docs.aws.amazon.com/cli/latest/reference/elbv2/describe-load-balancers.html">describe-load-balancers</a> </p> </li> </ul> </li> </ul> </dd> <dt>Amazon S3 bucket that is configured as a static website</dt> <dd> <p>Specify the domain name of the Amazon S3 website endpoint in which you created the bucket, for example, <code>s3-website-us-east-2.amazonaws.com</code>. For more information about valid values, see the table <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region">Amazon Simple Storage Service (S3) Website Endpoints</a> in the <i>Amazon Web Services General Reference</i>. For more information about using S3 buckets for websites, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/getting-started.html">Getting Started with Amazon Route 53</a> in the <i>Amazon Route 53 Developer Guide.</i> </p> </dd> <dt>Another Amazon Route 53 resource record set</dt> <dd> <p>Specify the value of the <code>Name</code> element for a resource record set in the current hosted zone.</p> </dd> </dl></p>
    pub dns_name: String,
    /// <p> <i>Applies only to alias, failover alias, geolocation alias, latency alias, and weighted alias resource record sets:</i> When <code>EvaluateTargetHealth</code> is <code>true</code>, an alias resource record set inherits the health of the referenced AWS resource, such as an ELB load balancer, or the referenced resource record set.</p> <p>Note the following:</p> <ul> <li> <p>You can't set <code>EvaluateTargetHealth</code> to <code>true</code> when the alias target is a CloudFront distribution.</p> </li> <li> <p>If the AWS resource that you specify in <code>AliasTarget</code> is a resource record set or a group of resource record sets (for example, a group of weighted resource record sets), but it is not another alias resource record set, we recommend that you associate a health check with all of the resource record sets in the alias target. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-complex-configs.html#dns-failover-complex-configs-hc-omitting">What Happens When You Omit Health Checks?</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </li> <li> <p>If you specify an Elastic Beanstalk environment in <code>HostedZoneId</code> and <code>DNSName</code>, and if the environment contains an ELB load balancer, Elastic Load Balancing routes queries only to the healthy Amazon EC2 instances that are registered with the load balancer. (An environment automatically contains an ELB load balancer if it includes more than one EC2 instance.) If you set <code>EvaluateTargetHealth</code> to <code>true</code> and either no EC2 instances are healthy or the load balancer itself is unhealthy, Amazon Route 53 routes queries to other available resources that are healthy, if any.</p> <p>If the environment contains a single EC2 instance, there are no special requirements.</p> </li> <li> <p>If you specify an ELB load balancer in <code> <a>AliasTarget</a> </code>, ELB routes queries only to the healthy EC2 instances that are registered with the load balancer. If no EC2 instances are healthy or if the load balancer itself is unhealthy, and if <code>EvaluateTargetHealth</code> is true for the corresponding alias resource record set, Amazon Route 53 routes queries to other resources. When you create a load balancer, you configure settings for ELB health checks; they're not Amazon Route 53 health checks, but they perform a similar function. Do not create Amazon Route 53 health checks for the EC2 instances that you register with an ELB load balancer.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-complex-configs.html">How Health Checks Work in More Complex Amazon Route 53 Configurations</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </li> <li> <p>We recommend that you set <code>EvaluateTargetHealth</code> to true only when you have enough idle capacity to handle the failure of one or more endpoints.</p> </li> </ul> <p>For more information and examples, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover.html">Amazon Route 53 Health Checks and DNS Failover</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    pub evaluate_target_health: bool,
    /// <p><p> <i>Alias resource records sets only</i>: The value used depends on where you want to route traffic:</p> <dl> <dt>CloudFront distribution</dt> <dd> <p>Specify <code>Z2FDTNDATAQYW2</code>.</p> <note> <p>Alias resource record sets for CloudFront can&#39;t be created in a private zone.</p> </note> </dd> <dt>Elastic Beanstalk environment</dt> <dd> <p>Specify the hosted zone ID for the region in which you created the environment. The environment must have a regionalized subdomain. For a list of regions and the corresponding hosted zone IDs, see <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#elasticbeanstalk_region">AWS Elastic Beanstalk</a> in the &quot;AWS Regions and Endpoints&quot; chapter of the <i>Amazon Web Services General Reference</i>.</p> </dd> <dt>ELB load balancer</dt> <dd> <p>Specify the value of the hosted zone ID for the load balancer. Use the following methods to get the hosted zone ID:</p> <ul> <li> <p> <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#elb_region">Elastic Load Balancing</a> table in the &quot;AWS Regions and Endpoints&quot; chapter of the <i>Amazon Web Services General Reference</i>: Use the value that corresponds with the region that you created your load balancer in. Note that there are separate columns for Application and Classic Load Balancers and for Network Load Balancers.</p> </li> <li> <p> <b>AWS Management Console</b>: Go to the Amazon EC2 page, choose <b>Load Balancers</b> in the navigation pane, select the load balancer, and get the value of the <b>Hosted zone</b> field on the <b>Description</b> tab.</p> </li> <li> <p> <b>Elastic Load Balancing API</b>: Use <code>DescribeLoadBalancers</code> to get the applicable value. For more information, see the applicable guide:</p> <ul> <li> <p>Classic Load Balancers: Use <a href="http://docs.aws.amazon.com/elasticloadbalancing/2012-06-01/APIReference/API_DescribeLoadBalancers.html">DescribeLoadBalancers</a> to get the value of <code>CanonicalHostedZoneNameId</code>.</p> </li> <li> <p>Application and Network Load Balancers: Use <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DescribeLoadBalancers.html">DescribeLoadBalancers</a> to get the value of <code>CanonicalHostedZoneId</code>.</p> </li> </ul> </li> <li> <p> <b>AWS CLI</b>: Use <code>describe-load-balancers</code> to get the applicable value. For more information, see the applicable guide:</p> <ul> <li> <p>Classic Load Balancers: Use <a href="http://docs.aws.amazon.com/cli/latest/reference/elb/describe-load-balancers.html">describe-load-balancers</a> to get the value of <code>CanonicalHostedZoneNameId</code>.</p> </li> <li> <p>Application and Network Load Balancers: Use <a href="http://docs.aws.amazon.com/cli/latest/reference/elbv2/describe-load-balancers.html">describe-load-balancers</a> to get the value of <code>CanonicalHostedZoneId</code>.</p> </li> </ul> </li> </ul> </dd> <dt>An Amazon S3 bucket configured as a static website</dt> <dd> <p>Specify the hosted zone ID for the region that you created the bucket in. For more information about valid values, see the <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region">Amazon Simple Storage Service Website Endpoints</a> table in the &quot;AWS Regions and Endpoints&quot; chapter of the <i>Amazon Web Services General Reference</i>.</p> </dd> <dt>Another Amazon Route 53 resource record set in your hosted zone</dt> <dd> <p>Specify the hosted zone ID of your hosted zone. (An alias resource record set can&#39;t reference a resource record set in a different hosted zone.)</p> </dd> </dl></p>
    pub hosted_zone_id: String,
}

struct AliasTargetDeserializer;
impl AliasTargetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AliasTarget, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AliasTarget::default();

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
                    "DNSName" => {
                        obj.dns_name = try!(DNSNameDeserializer::deserialize("DNSName", stack));
                    }
                    "EvaluateTargetHealth" => {
                        obj.evaluate_target_health =
                            try!(AliasHealthEnabledDeserializer::deserialize(
                                "EvaluateTargetHealth",
                                stack
                            ));
                    }
                    "HostedZoneId" => {
                        obj.hosted_zone_id =
                            try!(ResourceIdDeserializer::deserialize("HostedZoneId", stack));
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

pub struct AliasTargetSerializer;
impl AliasTargetSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AliasTarget,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("DNSName"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.dns_name
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("EvaluateTargetHealth"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.evaluate_target_health
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("HostedZoneId"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.hosted_zone_id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct AssociateVPCCommentSerializer;
impl AssociateVPCCommentSerializer {
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

/// <p>A complex type that contains information about the request to associate a VPC with a private hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AssociateVPCWithHostedZoneRequest {
    /// <p> <i>Optional:</i> A comment about the association request.</p>
    pub comment: Option<String>,
    /// <p>The ID of the private hosted zone that you want to associate an Amazon VPC with.</p> <p>Note that you can't associate a VPC with a hosted zone that doesn't have an existing VPC association.</p>
    pub hosted_zone_id: String,
    /// <p>A complex type that contains information about the VPC that you want to associate with a private hosted zone.</p>
    pub vpc: VPC,
}

pub struct AssociateVPCWithHostedZoneRequestSerializer;
impl AssociateVPCWithHostedZoneRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AssociateVPCWithHostedZoneRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        if let Some(ref value) = obj.comment {
            &AssociateVPCCommentSerializer::serialize(&mut writer, "Comment", value)?;
        }
        VPCSerializer::serialize(&mut writer, "VPC", &obj.vpc)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>A complex type that contains the response information for the <code>AssociateVPCWithHostedZone</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AssociateVPCWithHostedZoneResponse {
    /// <p>A complex type that describes the changes made to your hosted zone.</p>
    pub change_info: ChangeInfo,
}

struct AssociateVPCWithHostedZoneResponseDeserializer;
impl AssociateVPCWithHostedZoneResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AssociateVPCWithHostedZoneResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AssociateVPCWithHostedZoneResponse::default();

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
                    "ChangeInfo" => {
                        obj.change_info =
                            try!(ChangeInfoDeserializer::deserialize("ChangeInfo", stack));
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
/// <p>The information for each resource record set that you want to change.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Change {
    /// <p><p>The action to perform:</p> <ul> <li> <p> <code>CREATE</code>: Creates a resource record set that has the specified values.</p> </li> <li> <p> <code>DELETE</code>: Deletes a existing resource record set.</p> <important> <p>To delete the resource record set that is associated with a traffic policy instance, use <code> <a>DeleteTrafficPolicyInstance</a> </code>. Amazon Route 53 will delete the resource record set automatically. If you delete the resource record set by using <code>ChangeResourceRecordSets</code>, Amazon Route 53 doesn&#39;t automatically delete the traffic policy instance, and you&#39;ll continue to be charged for it even though it&#39;s no longer in use. </p> </important> </li> <li> <p> <code>UPSERT</code>: If a resource record set doesn&#39;t already exist, Amazon Route 53 creates it. If a resource record set does exist, Amazon Route 53 updates it with the values in the request.</p> </li> </ul></p>
    pub action: String,
    /// <p>Information about the resource record set to create, delete, or update.</p>
    pub resource_record_set: ResourceRecordSet,
}

pub struct ChangeSerializer;
impl ChangeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Change,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Action"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.action
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        ResourceRecordSetSerializer::serialize(
            &mut writer,
            "ResourceRecordSet",
            &obj.resource_record_set,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ChangeActionSerializer;
impl ChangeActionSerializer {
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

/// <p>The information for a change request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeBatch {
    /// <p>Information about the changes to make to the record sets.</p>
    pub changes: Vec<Change>,
    /// <p> <i>Optional:</i> Any comments you want to include about a change batch request.</p>
    pub comment: Option<String>,
}

pub struct ChangeBatchSerializer;
impl ChangeBatchSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ChangeBatch,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        ChangesSerializer::serialize(&mut writer, "Changes", &obj.changes)?;
        if let Some(ref value) = obj.comment {
            writer.write(xml::writer::XmlEvent::start_element("Comment"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A complex type that describes change information about changes made to your hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeInfo {
    /// <p>A complex type that describes change information about changes made to your hosted zone.</p> <p>This element contains an ID that you use when performing a <a>GetChange</a> action to get detailed information about the change.</p>
    pub comment: Option<String>,
    /// <p>The ID of the request.</p>
    pub id: String,
    /// <p>The current state of the request. <code>PENDING</code> indicates that this request has not yet been applied to all Amazon Route 53 DNS servers.</p>
    pub status: String,
    /// <p>The date and time that the change request was submitted in <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601 format</a> and Coordinated Universal Time (UTC). For example, the value <code>2017-03-27T17:48:16.751Z</code> represents March 27, 2017 at 17:48:16.751 UTC.</p>
    pub submitted_at: String,
}

struct ChangeInfoDeserializer;
impl ChangeInfoDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ChangeInfo, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ChangeInfo::default();

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
                        obj.comment = Some(try!(ResourceDescriptionDeserializer::deserialize(
                            "Comment", stack
                        )));
                    }
                    "Id" => {
                        obj.id = try!(ResourceIdDeserializer::deserialize("Id", stack));
                    }
                    "Status" => {
                        obj.status = try!(ChangeStatusDeserializer::deserialize("Status", stack));
                    }
                    "SubmittedAt" => {
                        obj.submitted_at =
                            try!(TimeStampDeserializer::deserialize("SubmittedAt", stack));
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
/// <p>A complex type that contains change information for the resource record set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeResourceRecordSetsRequest {
    /// <p>A complex type that contains an optional comment and the <code>Changes</code> element.</p>
    pub change_batch: ChangeBatch,
    /// <p>The ID of the hosted zone that contains the resource record sets that you want to change.</p>
    pub hosted_zone_id: String,
}

pub struct ChangeResourceRecordSetsRequestSerializer;
impl ChangeResourceRecordSetsRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ChangeResourceRecordSetsRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        ChangeBatchSerializer::serialize(&mut writer, "ChangeBatch", &obj.change_batch)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>A complex type containing the response for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeResourceRecordSetsResponse {
    /// <p>A complex type that contains information about changes made to your hosted zone.</p> <p>This element contains an ID that you use when performing a <a>GetChange</a> action to get detailed information about the change.</p>
    pub change_info: ChangeInfo,
}

struct ChangeResourceRecordSetsResponseDeserializer;
impl ChangeResourceRecordSetsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ChangeResourceRecordSetsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ChangeResourceRecordSetsResponse::default();

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
                    "ChangeInfo" => {
                        obj.change_info =
                            try!(ChangeInfoDeserializer::deserialize("ChangeInfo", stack));
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
struct ChangeStatusDeserializer;
impl ChangeStatusDeserializer {
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
/// <p>A complex type that contains information about the tags that you want to add, edit, or delete.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeTagsForResourceRequest {
    /// <p>A complex type that contains a list of the tags that you want to add to the specified health check or hosted zone and/or the tags that you want to edit <code>Value</code> for.</p> <p>You can add a maximum of 10 tags to a health check or a hosted zone.</p>
    pub add_tags: Option<Vec<Tag>>,
    /// <p>A complex type that contains a list of the tags that you want to delete from the specified health check or hosted zone. You can specify up to 10 keys.</p>
    pub remove_tag_keys: Option<Vec<String>>,
    /// <p>The ID of the resource for which you want to add, change, or delete tags.</p>
    pub resource_id: String,
    /// <p><p>The type of the resource.</p> <ul> <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li> <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li> </ul></p>
    pub resource_type: String,
}

pub struct ChangeTagsForResourceRequestSerializer;
impl ChangeTagsForResourceRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ChangeTagsForResourceRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        if let Some(ref value) = obj.add_tags {
            &TagListSerializer::serialize(&mut writer, "AddTags", value)?;
        }
        if let Some(ref value) = obj.remove_tag_keys {
            &TagKeyListSerializer::serialize(&mut writer, "RemoveTagKeys", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>Empty response for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeTagsForResourceResponse {}

struct ChangeTagsForResourceResponseDeserializer;
impl ChangeTagsForResourceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ChangeTagsForResourceResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = ChangeTagsForResourceResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

pub struct ChangesSerializer;
impl ChangesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<Change>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            ChangeSerializer::serialize(writer, "Change", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

struct CheckerIpRangesDeserializer;
impl CheckerIpRangesDeserializer {
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
                    if name == "member" {
                        obj.push(try!(IPAddressCidrDeserializer::deserialize(
                            "member", stack
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
struct ChildHealthCheckListDeserializer;
impl ChildHealthCheckListDeserializer {
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
                    if name == "ChildHealthCheck" {
                        obj.push(try!(HealthCheckIdDeserializer::deserialize(
                            "ChildHealthCheck",
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

pub struct ChildHealthCheckListSerializer;
impl ChildHealthCheckListSerializer {
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
            HealthCheckIdSerializer::serialize(writer, "ChildHealthCheck", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A complex type that contains information about the CloudWatch alarm that Amazon Route 53 is monitoring for this health check.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CloudWatchAlarmConfiguration {
    /// <p>For the metric that the CloudWatch alarm is associated with, the arithmetic operation that is used for the comparison.</p>
    pub comparison_operator: String,
    /// <p>For the metric that the CloudWatch alarm is associated with, a complex type that contains information about the dimensions for the metric. For information, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CW_Support_For_AWS.html">Amazon CloudWatch Namespaces, Dimensions, and Metrics Reference</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
    pub dimensions: Option<Vec<Dimension>>,
    /// <p>For the metric that the CloudWatch alarm is associated with, the number of periods that the metric is compared to the threshold.</p>
    pub evaluation_periods: i64,
    /// <p>The name of the CloudWatch metric that the alarm is associated with.</p>
    pub metric_name: String,
    /// <p>The namespace of the metric that the alarm is associated with. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CW_Support_For_AWS.html">Amazon CloudWatch Namespaces, Dimensions, and Metrics Reference</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
    pub namespace: String,
    /// <p>For the metric that the CloudWatch alarm is associated with, the duration of one evaluation period in seconds.</p>
    pub period: i64,
    /// <p>For the metric that the CloudWatch alarm is associated with, the statistic that is applied to the metric.</p>
    pub statistic: String,
    /// <p>For the metric that the CloudWatch alarm is associated with, the value the metric is compared with.</p>
    pub threshold: f64,
}

struct CloudWatchAlarmConfigurationDeserializer;
impl CloudWatchAlarmConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudWatchAlarmConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CloudWatchAlarmConfiguration::default();

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
                    "ComparisonOperator" => {
                        obj.comparison_operator =
                            try!(ComparisonOperatorDeserializer::deserialize(
                                "ComparisonOperator",
                                stack
                            ));
                    }
                    "Dimensions" => {
                        obj.dimensions = Some(try!(DimensionListDeserializer::deserialize(
                            "Dimensions",
                            stack
                        )));
                    }
                    "EvaluationPeriods" => {
                        obj.evaluation_periods = try!(EvaluationPeriodsDeserializer::deserialize(
                            "EvaluationPeriods",
                            stack
                        ));
                    }
                    "MetricName" => {
                        obj.metric_name =
                            try!(MetricNameDeserializer::deserialize("MetricName", stack));
                    }
                    "Namespace" => {
                        obj.namespace =
                            try!(NamespaceDeserializer::deserialize("Namespace", stack));
                    }
                    "Period" => {
                        obj.period = try!(PeriodDeserializer::deserialize("Period", stack));
                    }
                    "Statistic" => {
                        obj.statistic =
                            try!(StatisticDeserializer::deserialize("Statistic", stack));
                    }
                    "Threshold" => {
                        obj.threshold =
                            try!(ThresholdDeserializer::deserialize("Threshold", stack));
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
struct CloudWatchLogsLogGroupArnDeserializer;
impl CloudWatchLogsLogGroupArnDeserializer {
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

pub struct CloudWatchLogsLogGroupArnSerializer;
impl CloudWatchLogsLogGroupArnSerializer {
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

struct CloudWatchRegionDeserializer;
impl CloudWatchRegionDeserializer {
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

pub struct CloudWatchRegionSerializer;
impl CloudWatchRegionSerializer {
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

struct ComparisonOperatorDeserializer;
impl ComparisonOperatorDeserializer {
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
/// <p>A complex type that contains the health check request information.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateHealthCheckRequest {
    /// <p><p>A unique string that identifies the request and that allows you to retry a failed <code>CreateHealthCheck</code> request without the risk of creating two identical health checks:</p> <ul> <li> <p>If you send a <code>CreateHealthCheck</code> request with the same <code>CallerReference</code> and settings as a previous request, and if the health check doesn&#39;t exist, Amazon Route 53 creates the health check. If the health check does exist, Amazon Route 53 returns the settings for the existing health check.</p> </li> <li> <p>If you send a <code>CreateHealthCheck</code> request with the same <code>CallerReference</code> as a deleted health check, regardless of the settings, Amazon Route 53 returns a <code>HealthCheckAlreadyExists</code> error.</p> </li> <li> <p>If you send a <code>CreateHealthCheck</code> request with the same <code>CallerReference</code> as an existing health check but with different settings, Amazon Route 53 returns a <code>HealthCheckAlreadyExists</code> error.</p> </li> <li> <p>If you send a <code>CreateHealthCheck</code> request with a unique <code>CallerReference</code> but settings identical to an existing health check, Amazon Route 53 creates the health check.</p> </li> </ul></p>
    pub caller_reference: String,
    /// <p>A complex type that contains the response to a <code>CreateHealthCheck</code> request. </p>
    pub health_check_config: HealthCheckConfig,
}

pub struct CreateHealthCheckRequestSerializer;
impl CreateHealthCheckRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CreateHealthCheckRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        HealthCheckNonceSerializer::serialize(
            &mut writer,
            "CallerReference",
            &obj.caller_reference,
        )?;
        HealthCheckConfigSerializer::serialize(
            &mut writer,
            "HealthCheckConfig",
            &obj.health_check_config,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>A complex type containing the response information for the new health check.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateHealthCheckResponse {
    /// <p>A complex type that contains identifying information about the health check.</p>
    pub health_check: HealthCheck,
    /// <p>The unique URL representing the new health check.</p>
    pub location: String,
}

struct CreateHealthCheckResponseDeserializer;
impl CreateHealthCheckResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateHealthCheckResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateHealthCheckResponse::default();

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
                    "HealthCheck" => {
                        obj.health_check =
                            try!(HealthCheckDeserializer::deserialize("HealthCheck", stack));
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
/// <p>A complex type that contains information about the request to create a hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateHostedZoneRequest {
    /// <p>A unique string that identifies the request and that allows failed <code>CreateHostedZone</code> requests to be retried without the risk of executing the operation twice. You must use a unique <code>CallerReference</code> string every time you submit a <code>CreateHostedZone</code> request. <code>CallerReference</code> can be any unique string, for example, a date/time stamp.</p>
    pub caller_reference: String,
    /// <p>If you want to associate a reusable delegation set with this hosted zone, the ID that Amazon Route 53 assigned to the reusable delegation set when you created it. For more information about reusable delegation sets, see <a>CreateReusableDelegationSet</a>.</p>
    pub delegation_set_id: Option<String>,
    /// <p>(Optional) A complex type that contains the following optional values:</p> <ul> <li> <p>For public and private hosted zones, an optional comment</p> </li> <li> <p>For private hosted zones, an optional <code>PrivateZone</code> element</p> </li> </ul> <p>If you don't specify a comment or the <code>PrivateZone</code> element, omit <code>HostedZoneConfig</code> and the other elements.</p>
    pub hosted_zone_config: Option<HostedZoneConfig>,
    /// <p>The name of the domain. For resource record types that include a domain name, specify a fully qualified domain name, for example, <i>www.example.com</i>. The trailing dot is optional; Amazon Route 53 assumes that the domain name is fully qualified. This means that Amazon Route 53 treats <i>www.example.com</i> (without a trailing dot) and <i>www.example.com.</i> (with a trailing dot) as identical.</p> <p>If you're creating a public hosted zone, this is the name you have registered with your DNS registrar. If your domain name is registered with a registrar other than Amazon Route 53, change the name servers for your domain to the set of <code>NameServers</code> that <code>CreateHostedZone</code> returns in <code>DelegationSet</code>.</p>
    pub name: String,
    /// <p>(Private hosted zones only) A complex type that contains information about the Amazon VPC that you're associating with this hosted zone.</p> <p>You can specify only one Amazon VPC when you create a private hosted zone. To associate additional Amazon VPCs with the hosted zone, use <a>AssociateVPCWithHostedZone</a> after you create a hosted zone.</p>
    pub vpc: Option<VPC>,
}

pub struct CreateHostedZoneRequestSerializer;
impl CreateHostedZoneRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CreateHostedZoneRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        NonceSerializer::serialize(&mut writer, "CallerReference", &obj.caller_reference)?;
        if let Some(ref value) = obj.delegation_set_id {
            &ResourceIdSerializer::serialize(&mut writer, "DelegationSetId", value)?;
        }
        if let Some(ref value) = obj.hosted_zone_config {
            &HostedZoneConfigSerializer::serialize(&mut writer, "HostedZoneConfig", value)?;
        }
        DNSNameSerializer::serialize(&mut writer, "Name", &obj.name)?;
        if let Some(ref value) = obj.vpc {
            &VPCSerializer::serialize(&mut writer, "VPC", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>A complex type containing the response information for the hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateHostedZoneResponse {
    /// <p>A complex type that contains information about the <code>CreateHostedZone</code> request.</p>
    pub change_info: ChangeInfo,
    /// <p>A complex type that describes the name servers for this hosted zone.</p>
    pub delegation_set: DelegationSet,
    /// <p>A complex type that contains general information about the hosted zone.</p>
    pub hosted_zone: HostedZone,
    /// <p>The unique URL representing the new hosted zone.</p>
    pub location: String,
    /// <p>A complex type that contains information about an Amazon VPC that you associated with this hosted zone.</p>
    pub vpc: Option<VPC>,
}

struct CreateHostedZoneResponseDeserializer;
impl CreateHostedZoneResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateHostedZoneResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateHostedZoneResponse::default();

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
                    "ChangeInfo" => {
                        obj.change_info =
                            try!(ChangeInfoDeserializer::deserialize("ChangeInfo", stack));
                    }
                    "DelegationSet" => {
                        obj.delegation_set = try!(DelegationSetDeserializer::deserialize(
                            "DelegationSet",
                            stack
                        ));
                    }
                    "HostedZone" => {
                        obj.hosted_zone =
                            try!(HostedZoneDeserializer::deserialize("HostedZone", stack));
                    }
                    "VPC" => {
                        obj.vpc = Some(try!(VPCDeserializer::deserialize("VPC", stack)));
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateQueryLoggingConfigRequest {
    /// <p>The Amazon Resource Name (ARN) for the log group that you want to Amazon Route 53 to send query logs to. This is the format of the ARN:</p> <p>arn:aws:logs:<i>region</i>:<i>account-id</i>:log-group:<i>log_group_name</i> </p> <p>To get the ARN for a log group, you can use the CloudWatch console, the <a href="http://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogGroups.html">DescribeLogGroups</a> API action, the <a href="http://docs.aws.amazon.com/cli/latest/reference/logs/describe-log-groups.html">describe-log-groups</a> command, or the applicable command in one of the AWS SDKs.</p>
    pub cloud_watch_logs_log_group_arn: String,
    /// <p>The ID of the hosted zone that you want to log queries for. You can log queries only for public hosted zones.</p>
    pub hosted_zone_id: String,
}

pub struct CreateQueryLoggingConfigRequestSerializer;
impl CreateQueryLoggingConfigRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CreateQueryLoggingConfigRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        CloudWatchLogsLogGroupArnSerializer::serialize(
            &mut writer,
            "CloudWatchLogsLogGroupArn",
            &obj.cloud_watch_logs_log_group_arn,
        )?;
        ResourceIdSerializer::serialize(&mut writer, "HostedZoneId", &obj.hosted_zone_id)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateQueryLoggingConfigResponse {
    /// <p>The unique URL representing the new query logging configuration.</p>
    pub location: String,
    /// <p>A complex type that contains the ID for a query logging configuration, the ID of the hosted zone that you want to log queries for, and the ARN for the log group that you want Amazon Route 53 to send query logs to.</p>
    pub query_logging_config: QueryLoggingConfig,
}

struct CreateQueryLoggingConfigResponseDeserializer;
impl CreateQueryLoggingConfigResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateQueryLoggingConfigResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateQueryLoggingConfigResponse::default();

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
                    "QueryLoggingConfig" => {
                        obj.query_logging_config =
                            try!(QueryLoggingConfigDeserializer::deserialize(
                                "QueryLoggingConfig",
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateReusableDelegationSetRequest {
    /// <p>A unique string that identifies the request, and that allows you to retry failed <code>CreateReusableDelegationSet</code> requests without the risk of executing the operation twice. You must use a unique <code>CallerReference</code> string every time you submit a <code>CreateReusableDelegationSet</code> request. <code>CallerReference</code> can be any unique string, for example a date/time stamp.</p>
    pub caller_reference: String,
    /// <p>If you want to mark the delegation set for an existing hosted zone as reusable, the ID for that hosted zone.</p>
    pub hosted_zone_id: Option<String>,
}

pub struct CreateReusableDelegationSetRequestSerializer;
impl CreateReusableDelegationSetRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CreateReusableDelegationSetRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        NonceSerializer::serialize(&mut writer, "CallerReference", &obj.caller_reference)?;
        if let Some(ref value) = obj.hosted_zone_id {
            &ResourceIdSerializer::serialize(&mut writer, "HostedZoneId", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateReusableDelegationSetResponse {
    /// <p>A complex type that contains name server information.</p>
    pub delegation_set: DelegationSet,
    /// <p>The unique URL representing the new reusable delegation set.</p>
    pub location: String,
}

struct CreateReusableDelegationSetResponseDeserializer;
impl CreateReusableDelegationSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateReusableDelegationSetResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateReusableDelegationSetResponse::default();

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
                    "DelegationSet" => {
                        obj.delegation_set = try!(DelegationSetDeserializer::deserialize(
                            "DelegationSet",
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
/// <p>A complex type that contains information about the resource record sets that you want to create based on a specified traffic policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateTrafficPolicyInstanceRequest {
    /// <p>The ID of the hosted zone in which you want Amazon Route 53 to create resource record sets by using the configuration in a traffic policy.</p>
    pub hosted_zone_id: String,
    /// <p>The domain name (such as example.com) or subdomain name (such as www.example.com) for which Amazon Route 53 responds to DNS queries by using the resource record sets that Amazon Route 53 creates for this traffic policy instance.</p>
    pub name: String,
    /// <p>(Optional) The TTL that you want Amazon Route 53 to assign to all of the resource record sets that it creates in the specified hosted zone.</p>
    pub ttl: i64,
    /// <p>The ID of the traffic policy that you want to use to create resource record sets in the specified hosted zone.</p>
    pub traffic_policy_id: String,
    /// <p>The version of the traffic policy that you want to use to create resource record sets in the specified hosted zone.</p>
    pub traffic_policy_version: i64,
}

pub struct CreateTrafficPolicyInstanceRequestSerializer;
impl CreateTrafficPolicyInstanceRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CreateTrafficPolicyInstanceRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        ResourceIdSerializer::serialize(&mut writer, "HostedZoneId", &obj.hosted_zone_id)?;
        DNSNameSerializer::serialize(&mut writer, "Name", &obj.name)?;
        TTLSerializer::serialize(&mut writer, "TTL", &obj.ttl)?;
        TrafficPolicyIdSerializer::serialize(
            &mut writer,
            "TrafficPolicyId",
            &obj.traffic_policy_id,
        )?;
        TrafficPolicyVersionSerializer::serialize(
            &mut writer,
            "TrafficPolicyVersion",
            &obj.traffic_policy_version,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>A complex type that contains the response information for the <code>CreateTrafficPolicyInstance</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateTrafficPolicyInstanceResponse {
    /// <p>A unique URL that represents a new traffic policy instance.</p>
    pub location: String,
    /// <p>A complex type that contains settings for the new traffic policy instance.</p>
    pub traffic_policy_instance: TrafficPolicyInstance,
}

struct CreateTrafficPolicyInstanceResponseDeserializer;
impl CreateTrafficPolicyInstanceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateTrafficPolicyInstanceResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateTrafficPolicyInstanceResponse::default();

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
                    "TrafficPolicyInstance" => {
                        obj.traffic_policy_instance =
                            try!(TrafficPolicyInstanceDeserializer::deserialize(
                                "TrafficPolicyInstance",
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
/// <p>A complex type that contains information about the traffic policy that you want to create.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateTrafficPolicyRequest {
    /// <p>(Optional) Any comments that you want to include about the traffic policy.</p>
    pub comment: Option<String>,
    /// <p>The definition of this traffic policy in JSON format. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/APIReference/api-policies-traffic-policy-document-format.html">Traffic Policy Document Format</a>.</p>
    pub document: String,
    /// <p>The name of the traffic policy.</p>
    pub name: String,
}

pub struct CreateTrafficPolicyRequestSerializer;
impl CreateTrafficPolicyRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CreateTrafficPolicyRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        if let Some(ref value) = obj.comment {
            &TrafficPolicyCommentSerializer::serialize(&mut writer, "Comment", value)?;
        }
        TrafficPolicyDocumentSerializer::serialize(&mut writer, "Document", &obj.document)?;
        TrafficPolicyNameSerializer::serialize(&mut writer, "Name", &obj.name)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>A complex type that contains the response information for the <code>CreateTrafficPolicy</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateTrafficPolicyResponse {
    /// <p>A unique URL that represents a new traffic policy.</p>
    pub location: String,
    /// <p>A complex type that contains settings for the new traffic policy.</p>
    pub traffic_policy: TrafficPolicy,
}

struct CreateTrafficPolicyResponseDeserializer;
impl CreateTrafficPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateTrafficPolicyResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateTrafficPolicyResponse::default();

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
                    "TrafficPolicy" => {
                        obj.traffic_policy = try!(TrafficPolicyDeserializer::deserialize(
                            "TrafficPolicy",
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
/// <p>A complex type that contains information about the traffic policy that you want to create a new version for.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateTrafficPolicyVersionRequest {
    /// <p>The comment that you specified in the <code>CreateTrafficPolicyVersion</code> request, if any.</p>
    pub comment: Option<String>,
    /// <p>The definition of this version of the traffic policy, in JSON format. You specified the JSON in the <code>CreateTrafficPolicyVersion</code> request. For more information about the JSON format, see <a>CreateTrafficPolicy</a>.</p>
    pub document: String,
    /// <p>The ID of the traffic policy for which you want to create a new version.</p>
    pub id: String,
}

pub struct CreateTrafficPolicyVersionRequestSerializer;
impl CreateTrafficPolicyVersionRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CreateTrafficPolicyVersionRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        if let Some(ref value) = obj.comment {
            &TrafficPolicyCommentSerializer::serialize(&mut writer, "Comment", value)?;
        }
        TrafficPolicyDocumentSerializer::serialize(&mut writer, "Document", &obj.document)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>A complex type that contains the response information for the <code>CreateTrafficPolicyVersion</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateTrafficPolicyVersionResponse {
    /// <p>A unique URL that represents a new traffic policy version.</p>
    pub location: String,
    /// <p>A complex type that contains settings for the new version of the traffic policy.</p>
    pub traffic_policy: TrafficPolicy,
}

struct CreateTrafficPolicyVersionResponseDeserializer;
impl CreateTrafficPolicyVersionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateTrafficPolicyVersionResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateTrafficPolicyVersionResponse::default();

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
                    "TrafficPolicy" => {
                        obj.traffic_policy = try!(TrafficPolicyDeserializer::deserialize(
                            "TrafficPolicy",
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
/// <p>A complex type that contains information about the request to authorize associating a VPC with your private hosted zone. Authorization is only required when a private hosted zone and a VPC were created by using different accounts.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateVPCAssociationAuthorizationRequest {
    /// <p>The ID of the private hosted zone that you want to authorize associating a VPC with.</p>
    pub hosted_zone_id: String,
    /// <p>A complex type that contains the VPC ID and region for the VPC that you want to authorize associating with your hosted zone.</p>
    pub vpc: VPC,
}

pub struct CreateVPCAssociationAuthorizationRequestSerializer;
impl CreateVPCAssociationAuthorizationRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CreateVPCAssociationAuthorizationRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        VPCSerializer::serialize(&mut writer, "VPC", &obj.vpc)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>A complex type that contains the response information from a <code>CreateVPCAssociationAuthorization</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateVPCAssociationAuthorizationResponse {
    /// <p>The ID of the hosted zone that you authorized associating a VPC with.</p>
    pub hosted_zone_id: String,
    /// <p>The VPC that you authorized associating with a hosted zone.</p>
    pub vpc: VPC,
}

struct CreateVPCAssociationAuthorizationResponseDeserializer;
impl CreateVPCAssociationAuthorizationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateVPCAssociationAuthorizationResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateVPCAssociationAuthorizationResponse::default();

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
                    "HostedZoneId" => {
                        obj.hosted_zone_id =
                            try!(ResourceIdDeserializer::deserialize("HostedZoneId", stack));
                    }
                    "VPC" => {
                        obj.vpc = try!(VPCDeserializer::deserialize("VPC", stack));
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
struct DNSNameDeserializer;
impl DNSNameDeserializer {
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

pub struct DNSNameSerializer;
impl DNSNameSerializer {
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

struct DNSRCodeDeserializer;
impl DNSRCodeDeserializer {
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
/// <p>A complex type that lists the name servers in a delegation set, as well as the <code>CallerReference</code> and the <code>ID</code> for the delegation set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DelegationSet {
    /// <p>The value that you specified for <code>CallerReference</code> when you created the reusable delegation set.</p>
    pub caller_reference: Option<String>,
    /// <p>The ID that Amazon Route 53 assigns to a reusable delegation set.</p>
    pub id: Option<String>,
    /// <p>A complex type that contains a list of the authoritative name servers for a hosted zone or for a reusable delegation set.</p>
    pub name_servers: Vec<String>,
}

struct DelegationSetDeserializer;
impl DelegationSetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DelegationSet, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DelegationSet::default();

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
                        obj.caller_reference = Some(try!(NonceDeserializer::deserialize(
                            "CallerReference",
                            stack
                        )));
                    }
                    "Id" => {
                        obj.id = Some(try!(ResourceIdDeserializer::deserialize("Id", stack)));
                    }
                    "NameServers" => {
                        obj.name_servers = try!(DelegationSetNameServersDeserializer::deserialize(
                            "NameServers",
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
struct DelegationSetNameServersDeserializer;
impl DelegationSetNameServersDeserializer {
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
                    if name == "NameServer" {
                        obj.push(try!(DNSNameDeserializer::deserialize("NameServer", stack)));
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
struct DelegationSetsDeserializer;
impl DelegationSetsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DelegationSet>, XmlParseError> {
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
                    if name == "DelegationSet" {
                        obj.push(try!(DelegationSetDeserializer::deserialize(
                            "DelegationSet",
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
/// <p>This action deletes a health check.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteHealthCheckRequest {
    /// <p>The ID of the health check that you want to delete.</p>
    pub health_check_id: String,
}

/// <p>An empty element.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteHealthCheckResponse {}

struct DeleteHealthCheckResponseDeserializer;
impl DeleteHealthCheckResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteHealthCheckResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteHealthCheckResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A request to delete a hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteHostedZoneRequest {
    /// <p>The ID of the hosted zone you want to delete.</p>
    pub id: String,
}

/// <p>A complex type that contains the response to a <code>DeleteHostedZone</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteHostedZoneResponse {
    /// <p>A complex type that contains the ID, the status, and the date and time of a request to delete a hosted zone.</p>
    pub change_info: ChangeInfo,
}

struct DeleteHostedZoneResponseDeserializer;
impl DeleteHostedZoneResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteHostedZoneResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteHostedZoneResponse::default();

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
                    "ChangeInfo" => {
                        obj.change_info =
                            try!(ChangeInfoDeserializer::deserialize("ChangeInfo", stack));
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteQueryLoggingConfigRequest {
    /// <p>The ID of the configuration that you want to delete. </p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteQueryLoggingConfigResponse {}

struct DeleteQueryLoggingConfigResponseDeserializer;
impl DeleteQueryLoggingConfigResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteQueryLoggingConfigResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteQueryLoggingConfigResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A request to delete a reusable delegation set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteReusableDelegationSetRequest {
    /// <p>The ID of the reusable delegation set that you want to delete.</p>
    pub id: String,
}

/// <p>An empty element.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteReusableDelegationSetResponse {}

struct DeleteReusableDelegationSetResponseDeserializer;
impl DeleteReusableDelegationSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteReusableDelegationSetResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteReusableDelegationSetResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A request to delete a specified traffic policy instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteTrafficPolicyInstanceRequest {
    /// <p><p>The ID of the traffic policy instance that you want to delete. </p> <important> <p>When you delete a traffic policy instance, Amazon Route 53 also deletes all of the resource record sets that were created when you created the traffic policy instance.</p> </important></p>
    pub id: String,
}

/// <p>An empty element.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteTrafficPolicyInstanceResponse {}

struct DeleteTrafficPolicyInstanceResponseDeserializer;
impl DeleteTrafficPolicyInstanceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteTrafficPolicyInstanceResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteTrafficPolicyInstanceResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A request to delete a specified traffic policy version.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteTrafficPolicyRequest {
    /// <p>The ID of the traffic policy that you want to delete.</p>
    pub id: String,
    /// <p>The version number of the traffic policy that you want to delete.</p>
    pub version: i64,
}

/// <p>An empty element.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteTrafficPolicyResponse {}

struct DeleteTrafficPolicyResponseDeserializer;
impl DeleteTrafficPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteTrafficPolicyResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteTrafficPolicyResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A complex type that contains information about the request to remove authorization to associate a VPC that was created by one AWS account with a hosted zone that was created with a different AWS account. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteVPCAssociationAuthorizationRequest {
    /// <p>When removing authorization to associate a VPC that was created by one AWS account with a hosted zone that was created with a different AWS account, the ID of the hosted zone.</p>
    pub hosted_zone_id: String,
    /// <p>When removing authorization to associate a VPC that was created by one AWS account with a hosted zone that was created with a different AWS account, a complex type that includes the ID and region of the VPC.</p>
    pub vpc: VPC,
}

pub struct DeleteVPCAssociationAuthorizationRequestSerializer;
impl DeleteVPCAssociationAuthorizationRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &DeleteVPCAssociationAuthorizationRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        VPCSerializer::serialize(&mut writer, "VPC", &obj.vpc)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>Empty response for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteVPCAssociationAuthorizationResponse {}

struct DeleteVPCAssociationAuthorizationResponseDeserializer;
impl DeleteVPCAssociationAuthorizationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteVPCAssociationAuthorizationResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteVPCAssociationAuthorizationResponse::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>For the metric that the CloudWatch alarm is associated with, a complex type that contains information about one dimension.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Dimension {
    /// <p>For the metric that the CloudWatch alarm is associated with, the name of one dimension.</p>
    pub name: String,
    /// <p>For the metric that the CloudWatch alarm is associated with, the value of one dimension.</p>
    pub value: String,
}

struct DimensionDeserializer;
impl DimensionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Dimension, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Dimension::default();

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
                    "Name" => {
                        obj.name = try!(DimensionFieldDeserializer::deserialize("Name", stack));
                    }
                    "Value" => {
                        obj.value = try!(DimensionFieldDeserializer::deserialize("Value", stack));
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
struct DimensionFieldDeserializer;
impl DimensionFieldDeserializer {
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
struct DimensionListDeserializer;
impl DimensionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Dimension>, XmlParseError> {
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
                    if name == "Dimension" {
                        obj.push(try!(DimensionDeserializer::deserialize("Dimension", stack)));
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

pub struct DisassociateVPCCommentSerializer;
impl DisassociateVPCCommentSerializer {
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

/// <p>A complex type that contains information about the VPC that you want to disassociate from a specified private hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DisassociateVPCFromHostedZoneRequest {
    /// <p> <i>Optional:</i> A comment about the disassociation request.</p>
    pub comment: Option<String>,
    /// <p>The ID of the private hosted zone that you want to disassociate a VPC from.</p>
    pub hosted_zone_id: String,
    /// <p>A complex type that contains information about the VPC that you're disassociating from the specified hosted zone.</p>
    pub vpc: VPC,
}

pub struct DisassociateVPCFromHostedZoneRequestSerializer;
impl DisassociateVPCFromHostedZoneRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &DisassociateVPCFromHostedZoneRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        if let Some(ref value) = obj.comment {
            &DisassociateVPCCommentSerializer::serialize(&mut writer, "Comment", value)?;
        }
        VPCSerializer::serialize(&mut writer, "VPC", &obj.vpc)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>A complex type that contains the response information for the disassociate request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DisassociateVPCFromHostedZoneResponse {
    /// <p>A complex type that describes the changes made to the specified private hosted zone.</p>
    pub change_info: ChangeInfo,
}

struct DisassociateVPCFromHostedZoneResponseDeserializer;
impl DisassociateVPCFromHostedZoneResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DisassociateVPCFromHostedZoneResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DisassociateVPCFromHostedZoneResponse::default();

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
                    "ChangeInfo" => {
                        obj.change_info =
                            try!(ChangeInfoDeserializer::deserialize("ChangeInfo", stack));
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
struct EnableSNIDeserializer;
impl EnableSNIDeserializer {
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

pub struct EnableSNISerializer;
impl EnableSNISerializer {
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

struct EvaluationPeriodsDeserializer;
impl EvaluationPeriodsDeserializer {
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
struct FailureThresholdDeserializer;
impl FailureThresholdDeserializer {
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

pub struct FailureThresholdSerializer;
impl FailureThresholdSerializer {
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

struct FullyQualifiedDomainNameDeserializer;
impl FullyQualifiedDomainNameDeserializer {
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

pub struct FullyQualifiedDomainNameSerializer;
impl FullyQualifiedDomainNameSerializer {
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

/// <p>A complex type that contains information about a geo location.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GeoLocation {
    /// <p>The two-letter code for the continent.</p> <p>Valid values: <code>AF</code> | <code>AN</code> | <code>AS</code> | <code>EU</code> | <code>OC</code> | <code>NA</code> | <code>SA</code> </p> <p>Constraint: Specifying <code>ContinentCode</code> with either <code>CountryCode</code> or <code>SubdivisionCode</code> returns an <code>InvalidInput</code> error.</p>
    pub continent_code: Option<String>,
    /// <p>The two-letter code for the country.</p>
    pub country_code: Option<String>,
    /// <p>The code for the subdivision, for example, a state in the United States or a province in Canada.</p>
    pub subdivision_code: Option<String>,
}

struct GeoLocationDeserializer;
impl GeoLocationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GeoLocation, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GeoLocation::default();

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
                    "ContinentCode" => {
                        obj.continent_code =
                            Some(try!(GeoLocationContinentCodeDeserializer::deserialize(
                                "ContinentCode",
                                stack
                            )));
                    }
                    "CountryCode" => {
                        obj.country_code = Some(try!(
                            GeoLocationCountryCodeDeserializer::deserialize("CountryCode", stack)
                        ));
                    }
                    "SubdivisionCode" => {
                        obj.subdivision_code =
                            Some(try!(GeoLocationSubdivisionCodeDeserializer::deserialize(
                                "SubdivisionCode",
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

pub struct GeoLocationSerializer;
impl GeoLocationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &GeoLocation,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.continent_code {
            writer.write(xml::writer::XmlEvent::start_element("ContinentCode"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.country_code {
            writer.write(xml::writer::XmlEvent::start_element("CountryCode"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.subdivision_code {
            writer.write(xml::writer::XmlEvent::start_element("SubdivisionCode"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct GeoLocationContinentCodeDeserializer;
impl GeoLocationContinentCodeDeserializer {
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

pub struct GeoLocationContinentCodeSerializer;
impl GeoLocationContinentCodeSerializer {
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

struct GeoLocationContinentNameDeserializer;
impl GeoLocationContinentNameDeserializer {
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
struct GeoLocationCountryCodeDeserializer;
impl GeoLocationCountryCodeDeserializer {
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

pub struct GeoLocationCountryCodeSerializer;
impl GeoLocationCountryCodeSerializer {
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

struct GeoLocationCountryNameDeserializer;
impl GeoLocationCountryNameDeserializer {
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
/// <p>A complex type that contains the codes and full continent, country, and subdivision names for the specified <code>geolocation</code> code.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GeoLocationDetails {
    /// <p>The two-letter code for the continent.</p>
    pub continent_code: Option<String>,
    /// <p>The full name of the continent.</p>
    pub continent_name: Option<String>,
    /// <p>The two-letter code for the country.</p>
    pub country_code: Option<String>,
    /// <p>The name of the country.</p>
    pub country_name: Option<String>,
    /// <p>The code for the subdivision, for example, a state in the United States or a province in Canada.</p>
    pub subdivision_code: Option<String>,
    /// <p>The full name of the subdivision, for example, a state in the United States or a province in Canada.</p>
    pub subdivision_name: Option<String>,
}

struct GeoLocationDetailsDeserializer;
impl GeoLocationDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GeoLocationDetails, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GeoLocationDetails::default();

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
                    "ContinentCode" => {
                        obj.continent_code =
                            Some(try!(GeoLocationContinentCodeDeserializer::deserialize(
                                "ContinentCode",
                                stack
                            )));
                    }
                    "ContinentName" => {
                        obj.continent_name =
                            Some(try!(GeoLocationContinentNameDeserializer::deserialize(
                                "ContinentName",
                                stack
                            )));
                    }
                    "CountryCode" => {
                        obj.country_code = Some(try!(
                            GeoLocationCountryCodeDeserializer::deserialize("CountryCode", stack)
                        ));
                    }
                    "CountryName" => {
                        obj.country_name = Some(try!(
                            GeoLocationCountryNameDeserializer::deserialize("CountryName", stack)
                        ));
                    }
                    "SubdivisionCode" => {
                        obj.subdivision_code =
                            Some(try!(GeoLocationSubdivisionCodeDeserializer::deserialize(
                                "SubdivisionCode",
                                stack
                            )));
                    }
                    "SubdivisionName" => {
                        obj.subdivision_name =
                            Some(try!(GeoLocationSubdivisionNameDeserializer::deserialize(
                                "SubdivisionName",
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
struct GeoLocationDetailsListDeserializer;
impl GeoLocationDetailsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<GeoLocationDetails>, XmlParseError> {
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
                    if name == "GeoLocationDetails" {
                        obj.push(try!(GeoLocationDetailsDeserializer::deserialize(
                            "GeoLocationDetails",
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
struct GeoLocationSubdivisionCodeDeserializer;
impl GeoLocationSubdivisionCodeDeserializer {
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

pub struct GeoLocationSubdivisionCodeSerializer;
impl GeoLocationSubdivisionCodeSerializer {
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

struct GeoLocationSubdivisionNameDeserializer;
impl GeoLocationSubdivisionNameDeserializer {
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
/// <p>A complex type that contains information about the request to create a hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetAccountLimitRequest {
    /// <p><p>The limit that you want to get. Valid values include the following:</p> <ul> <li> <p> <b>MAX<em>HEALTH</em>CHECKS<em>BY</em>OWNER</b>: The maximum number of health checks that you can create using the current account.</p> </li> <li> <p> <b>MAX<em>HOSTED</em>ZONES<em>BY</em>OWNER</b>: The maximum number of hosted zones that you can create using the current account.</p> </li> <li> <p> <b>MAX<em>REUSABLE</em>DELEGATION<em>SETS</em>BY<em>OWNER</b>: The maximum number of reusable delegation sets that you can create using the current account.</p> </li> <li> <p> <b>MAX</em>TRAFFIC<em>POLICIES</em>BY<em>OWNER</b>: The maximum number of traffic policies that you can create using the current account.</p> </li> <li> <p> <b>MAX</em>TRAFFIC<em>POLICY</em>INSTANCES<em>BY</em>OWNER</b>: The maximum number of traffic policy instances that you can create using the current account. (Traffic policy instances are referred to as traffic flow policy records in the Amazon Route 53 console.)</p> </li> </ul></p>
    pub type_: String,
}

/// <p>A complex type that contains the requested limit. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetAccountLimitResponse {
    /// <p>The current number of entities that you have created of the specified type. For example, if you specified <code>MAX_HEALTH_CHECKS_BY_OWNER</code> for the value of <code>Type</code> in the request, the value of <code>Count</code> is the current number of health checks that you have created using the current account.</p>
    pub count: i64,
    /// <p>The current setting for the specified limit. For example, if you specified <code>MAX_HEALTH_CHECKS_BY_OWNER</code> for the value of <code>Type</code> in the request, the value of <code>Limit</code> is the maximum number of health checks that you can create using the current account.</p>
    pub limit: AccountLimit,
}

struct GetAccountLimitResponseDeserializer;
impl GetAccountLimitResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetAccountLimitResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetAccountLimitResponse::default();

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
                    "Count" => {
                        obj.count = try!(UsageCountDeserializer::deserialize("Count", stack));
                    }
                    "Limit" => {
                        obj.limit = try!(AccountLimitDeserializer::deserialize("Limit", stack));
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
/// <p>The input for a GetChange request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetChangeRequest {
    /// <p>The ID of the change batch request. The value that you specify here is the value that <code>ChangeResourceRecordSets</code> returned in the <code>Id</code> element when you submitted the request.</p>
    pub id: String,
}

/// <p>A complex type that contains the <code>ChangeInfo</code> element.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetChangeResponse {
    /// <p>A complex type that contains information about the specified change batch.</p>
    pub change_info: ChangeInfo,
}

struct GetChangeResponseDeserializer;
impl GetChangeResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetChangeResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetChangeResponse::default();

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
                    "ChangeInfo" => {
                        obj.change_info =
                            try!(ChangeInfoDeserializer::deserialize("ChangeInfo", stack));
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetCheckerIpRangesRequest {}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetCheckerIpRangesResponse {
    pub checker_ip_ranges: Vec<String>,
}

struct GetCheckerIpRangesResponseDeserializer;
impl GetCheckerIpRangesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetCheckerIpRangesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetCheckerIpRangesResponse::default();

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
                    "CheckerIpRanges" => {
                        obj.checker_ip_ranges = try!(CheckerIpRangesDeserializer::deserialize(
                            "CheckerIpRanges",
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
/// <p>A request for information about whether a specified geographic location is supported for Amazon Route 53 geolocation resource record sets.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetGeoLocationRequest {
    /// <p><p>Amazon Route 53 supports the following continent codes:</p> <ul> <li> <p> <b>AF</b>: Africa</p> </li> <li> <p> <b>AN</b>: Antarctica</p> </li> <li> <p> <b>AS</b>: Asia</p> </li> <li> <p> <b>EU</b>: Europe</p> </li> <li> <p> <b>OC</b>: Oceania</p> </li> <li> <p> <b>NA</b>: North America</p> </li> <li> <p> <b>SA</b>: South America</p> </li> </ul></p>
    pub continent_code: Option<String>,
    /// <p>Amazon Route 53 uses the two-letter country codes that are specified in <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO standard 3166-1 alpha-2</a>.</p>
    pub country_code: Option<String>,
    /// <p>Amazon Route 53 uses the one- to three-letter subdivision codes that are specified in <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO standard 3166-1 alpha-2</a>. Amazon Route 53 doesn't support subdivision codes for all countries. If you specify <code>SubdivisionCode</code>, you must also specify <code>CountryCode</code>. </p>
    pub subdivision_code: Option<String>,
}

/// <p>A complex type that contains the response information for the specified geolocation code.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetGeoLocationResponse {
    /// <p>A complex type that contains the codes and full continent, country, and subdivision names for the specified geolocation code.</p>
    pub geo_location_details: GeoLocationDetails,
}

struct GetGeoLocationResponseDeserializer;
impl GetGeoLocationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetGeoLocationResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetGeoLocationResponse::default();

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
                    "GeoLocationDetails" => {
                        obj.geo_location_details =
                            try!(GeoLocationDetailsDeserializer::deserialize(
                                "GeoLocationDetails",
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
/// <p>A request for the number of health checks that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHealthCheckCountRequest {}

/// <p>A complex type that contains the response to a <code>GetHealthCheckCount</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHealthCheckCountResponse {
    /// <p>The number of health checks associated with the current AWS account.</p>
    pub health_check_count: i64,
}

struct GetHealthCheckCountResponseDeserializer;
impl GetHealthCheckCountResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHealthCheckCountResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetHealthCheckCountResponse::default();

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
                    "HealthCheckCount" => {
                        obj.health_check_count = try!(HealthCheckCountDeserializer::deserialize(
                            "HealthCheckCount",
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
/// <p>A request for the reason that a health check failed most recently.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHealthCheckLastFailureReasonRequest {
    /// <p><p>The ID for the health check for which you want the last failure reason. When you created the health check, <code>CreateHealthCheck</code> returned the ID in the response, in the <code>HealthCheckId</code> element.</p> <note> <p>If you want to get the last failure reason for a calculated health check, you must use the Amazon Route 53 console or the CloudWatch console. You can&#39;t use <code>GetHealthCheckLastFailureReason</code> for a calculated health check.</p> </note></p>
    pub health_check_id: String,
}

/// <p>A complex type that contains the response to a <code>GetHealthCheckLastFailureReason</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHealthCheckLastFailureReasonResponse {
    /// <p>A list that contains one <code>Observation</code> element for each Amazon Route 53 health checker that is reporting a last failure reason. </p>
    pub health_check_observations: Vec<HealthCheckObservation>,
}

struct GetHealthCheckLastFailureReasonResponseDeserializer;
impl GetHealthCheckLastFailureReasonResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHealthCheckLastFailureReasonResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetHealthCheckLastFailureReasonResponse::default();

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
                    "HealthCheckObservations" => {
                        obj.health_check_observations =
                            try!(HealthCheckObservationsDeserializer::deserialize(
                                "HealthCheckObservations",
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
/// <p>A request to get information about a specified health check. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHealthCheckRequest {
    /// <p>The identifier that Amazon Route 53 assigned to the health check when you created it. When you add or update a resource record set, you use this value to specify which health check to use. The value can be up to 64 characters long.</p>
    pub health_check_id: String,
}

/// <p>A complex type that contains the response to a <code>GetHealthCheck</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHealthCheckResponse {
    /// <p>A complex type that contains information about one health check that is associated with the current AWS account.</p>
    pub health_check: HealthCheck,
}

struct GetHealthCheckResponseDeserializer;
impl GetHealthCheckResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHealthCheckResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetHealthCheckResponse::default();

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
                    "HealthCheck" => {
                        obj.health_check =
                            try!(HealthCheckDeserializer::deserialize("HealthCheck", stack));
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
/// <p>A request to get the status for a health check.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHealthCheckStatusRequest {
    /// <p><p>The ID for the health check that you want the current status for. When you created the health check, <code>CreateHealthCheck</code> returned the ID in the response, in the <code>HealthCheckId</code> element.</p> <note> <p>If you want to check the status of a calculated health check, you must use the Amazon Route 53 console or the CloudWatch console. You can&#39;t use <code>GetHealthCheckStatus</code> to get the status of a calculated health check.</p> </note></p>
    pub health_check_id: String,
}

/// <p>A complex type that contains the response to a <code>GetHealthCheck</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHealthCheckStatusResponse {
    /// <p>A list that contains one <code>HealthCheckObservation</code> element for each Amazon Route 53 health checker that is reporting a status about the health check endpoint.</p>
    pub health_check_observations: Vec<HealthCheckObservation>,
}

struct GetHealthCheckStatusResponseDeserializer;
impl GetHealthCheckStatusResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHealthCheckStatusResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetHealthCheckStatusResponse::default();

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
                    "HealthCheckObservations" => {
                        obj.health_check_observations =
                            try!(HealthCheckObservationsDeserializer::deserialize(
                                "HealthCheckObservations",
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
/// <p>A request to retrieve a count of all the hosted zones that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHostedZoneCountRequest {}

/// <p>A complex type that contains the response to a <code>GetHostedZoneCount</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHostedZoneCountResponse {
    /// <p>The total number of public and private hosted zones that are associated with the current AWS account.</p>
    pub hosted_zone_count: i64,
}

struct GetHostedZoneCountResponseDeserializer;
impl GetHostedZoneCountResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHostedZoneCountResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetHostedZoneCountResponse::default();

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
                    "HostedZoneCount" => {
                        obj.hosted_zone_count = try!(HostedZoneCountDeserializer::deserialize(
                            "HostedZoneCount",
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
/// <p>A complex type that contains information about the request to create a hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHostedZoneLimitRequest {
    /// <p>The ID of the hosted zone that you want to get a limit for.</p>
    pub hosted_zone_id: String,
    /// <p><p>The limit that you want to get. Valid values include the following:</p> <ul> <li> <p> <b>MAX<em>RRSETS</em>BY<em>ZONE</b>: The maximum number of records that you can create in the specified hosted zone.</p> </li> <li> <p> <b>MAX</em>VPCS<em>ASSOCIATED</em>BY_ZONE</b>: The maximum number of Amazon VPCs that you can associate with the specified private hosted zone.</p> </li> </ul></p>
    pub type_: String,
}

/// <p>A complex type that contains the requested limit. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHostedZoneLimitResponse {
    /// <p>The current number of entities that you have created of the specified type. For example, if you specified <code>MAX_RRSETS_BY_ZONE</code> for the value of <code>Type</code> in the request, the value of <code>Count</code> is the current number of records that you have created in the specified hosted zone.</p>
    pub count: i64,
    /// <p>The current setting for the specified limit. For example, if you specified <code>MAX_RRSETS_BY_ZONE</code> for the value of <code>Type</code> in the request, the value of <code>Limit</code> is the maximum number of records that you can create in the specified hosted zone.</p>
    pub limit: HostedZoneLimit,
}

struct GetHostedZoneLimitResponseDeserializer;
impl GetHostedZoneLimitResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHostedZoneLimitResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetHostedZoneLimitResponse::default();

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
                    "Count" => {
                        obj.count = try!(UsageCountDeserializer::deserialize("Count", stack));
                    }
                    "Limit" => {
                        obj.limit = try!(HostedZoneLimitDeserializer::deserialize("Limit", stack));
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
/// <p>A request to get information about a specified hosted zone. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHostedZoneRequest {
    /// <p>The ID of the hosted zone that you want to get information about.</p>
    pub id: String,
}

/// <p>A complex type that contain the response to a <code>GetHostedZone</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetHostedZoneResponse {
    /// <p>A complex type that lists the Amazon Route 53 name servers for the specified hosted zone.</p>
    pub delegation_set: Option<DelegationSet>,
    /// <p>A complex type that contains general information about the specified hosted zone.</p>
    pub hosted_zone: HostedZone,
    /// <p>A complex type that contains information about the VPCs that are associated with the specified hosted zone.</p>
    pub vp_cs: Option<Vec<VPC>>,
}

struct GetHostedZoneResponseDeserializer;
impl GetHostedZoneResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHostedZoneResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetHostedZoneResponse::default();

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
                    "DelegationSet" => {
                        obj.delegation_set = Some(try!(DelegationSetDeserializer::deserialize(
                            "DelegationSet",
                            stack
                        )));
                    }
                    "HostedZone" => {
                        obj.hosted_zone =
                            try!(HostedZoneDeserializer::deserialize("HostedZone", stack));
                    }
                    "VPCs" => {
                        obj.vp_cs = Some(try!(VPCsDeserializer::deserialize("VPCs", stack)));
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetQueryLoggingConfigRequest {
    /// <p>The ID of the configuration for DNS query logging that you want to get information about.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetQueryLoggingConfigResponse {
    /// <p>A complex type that contains information about the query logging configuration that you specified in a <a>GetQueryLoggingConfig</a> request.</p>
    pub query_logging_config: QueryLoggingConfig,
}

struct GetQueryLoggingConfigResponseDeserializer;
impl GetQueryLoggingConfigResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetQueryLoggingConfigResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetQueryLoggingConfigResponse::default();

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
                    "QueryLoggingConfig" => {
                        obj.query_logging_config =
                            try!(QueryLoggingConfigDeserializer::deserialize(
                                "QueryLoggingConfig",
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
/// <p>A complex type that contains information about the request to create a hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetReusableDelegationSetLimitRequest {
    /// <p>The ID of the delegation set that you want to get the limit for.</p>
    pub delegation_set_id: String,
    /// <p>Specify <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code> to get the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p>
    pub type_: String,
}

/// <p>A complex type that contains the requested limit. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetReusableDelegationSetLimitResponse {
    /// <p>The current number of hosted zones that you can associate with the specified reusable delegation set.</p>
    pub count: i64,
    /// <p>The current setting for the limit on hosted zones that you can associate with the specified reusable delegation set.</p>
    pub limit: ReusableDelegationSetLimit,
}

struct GetReusableDelegationSetLimitResponseDeserializer;
impl GetReusableDelegationSetLimitResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetReusableDelegationSetLimitResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetReusableDelegationSetLimitResponse::default();

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
                    "Count" => {
                        obj.count = try!(UsageCountDeserializer::deserialize("Count", stack));
                    }
                    "Limit" => {
                        obj.limit = try!(ReusableDelegationSetLimitDeserializer::deserialize(
                            "Limit", stack
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
/// <p>A request to get information about a specified reusable delegation set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetReusableDelegationSetRequest {
    /// <p>The ID of the reusable delegation set that you want to get a list of name servers for.</p>
    pub id: String,
}

/// <p>A complex type that contains the response to the <code>GetReusableDelegationSet</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetReusableDelegationSetResponse {
    /// <p>A complex type that contains information about the reusable delegation set.</p>
    pub delegation_set: DelegationSet,
}

struct GetReusableDelegationSetResponseDeserializer;
impl GetReusableDelegationSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetReusableDelegationSetResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetReusableDelegationSetResponse::default();

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
                    "DelegationSet" => {
                        obj.delegation_set = try!(DelegationSetDeserializer::deserialize(
                            "DelegationSet",
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
/// <p>Request to get the number of traffic policy instances that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTrafficPolicyInstanceCountRequest {}

/// <p>A complex type that contains information about the resource record sets that Amazon Route 53 created based on a specified traffic policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTrafficPolicyInstanceCountResponse {
    /// <p>The number of traffic policy instances that are associated with the current AWS account.</p>
    pub traffic_policy_instance_count: i64,
}

struct GetTrafficPolicyInstanceCountResponseDeserializer;
impl GetTrafficPolicyInstanceCountResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTrafficPolicyInstanceCountResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetTrafficPolicyInstanceCountResponse::default();

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
                    "TrafficPolicyInstanceCount" => {
                        obj.traffic_policy_instance_count =
                            try!(TrafficPolicyInstanceCountDeserializer::deserialize(
                                "TrafficPolicyInstanceCount",
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
/// <p>Gets information about a specified traffic policy instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTrafficPolicyInstanceRequest {
    /// <p>The ID of the traffic policy instance that you want to get information about.</p>
    pub id: String,
}

/// <p>A complex type that contains information about the resource record sets that Amazon Route 53 created based on a specified traffic policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTrafficPolicyInstanceResponse {
    /// <p>A complex type that contains settings for the traffic policy instance.</p>
    pub traffic_policy_instance: TrafficPolicyInstance,
}

struct GetTrafficPolicyInstanceResponseDeserializer;
impl GetTrafficPolicyInstanceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTrafficPolicyInstanceResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetTrafficPolicyInstanceResponse::default();

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
                    "TrafficPolicyInstance" => {
                        obj.traffic_policy_instance =
                            try!(TrafficPolicyInstanceDeserializer::deserialize(
                                "TrafficPolicyInstance",
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
/// <p>Gets information about a specific traffic policy version.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTrafficPolicyRequest {
    /// <p>The ID of the traffic policy that you want to get information about.</p>
    pub id: String,
    /// <p>The version number of the traffic policy that you want to get information about.</p>
    pub version: i64,
}

/// <p>A complex type that contains the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTrafficPolicyResponse {
    /// <p>A complex type that contains settings for the specified traffic policy.</p>
    pub traffic_policy: TrafficPolicy,
}

struct GetTrafficPolicyResponseDeserializer;
impl GetTrafficPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTrafficPolicyResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetTrafficPolicyResponse::default();

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
                    "TrafficPolicy" => {
                        obj.traffic_policy = try!(TrafficPolicyDeserializer::deserialize(
                            "TrafficPolicy",
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
/// <p>A complex type that contains information about one health check that is associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HealthCheck {
    /// <p>A unique string that you specified when you created the health check.</p>
    pub caller_reference: String,
    /// <p>A complex type that contains information about the CloudWatch alarm that Amazon Route 53 is monitoring for this health check.</p>
    pub cloud_watch_alarm_configuration: Option<CloudWatchAlarmConfiguration>,
    /// <p>A complex type that contains detailed information about one health check.</p>
    pub health_check_config: HealthCheckConfig,
    /// <p>The version of the health check. You can optionally pass this value in a call to <code>UpdateHealthCheck</code> to prevent overwriting another change to the health check.</p>
    pub health_check_version: i64,
    /// <p>The identifier that Amazon Route 53assigned to the health check when you created it. When you add or update a resource record set, you use this value to specify which health check to use. The value can be up to 64 characters long. </p>
    pub id: String,
    /// <p>If the health check was created by another service, the service that created the health check. When a health check is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    pub linked_service: Option<LinkedService>,
}

struct HealthCheckDeserializer;
impl HealthCheckDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HealthCheck, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = HealthCheck::default();

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
                        obj.caller_reference = try!(HealthCheckNonceDeserializer::deserialize(
                            "CallerReference",
                            stack
                        ));
                    }
                    "CloudWatchAlarmConfiguration" => {
                        obj.cloud_watch_alarm_configuration =
                            Some(try!(CloudWatchAlarmConfigurationDeserializer::deserialize(
                                "CloudWatchAlarmConfiguration",
                                stack
                            )));
                    }
                    "HealthCheckConfig" => {
                        obj.health_check_config = try!(HealthCheckConfigDeserializer::deserialize(
                            "HealthCheckConfig",
                            stack
                        ));
                    }
                    "HealthCheckVersion" => {
                        obj.health_check_version =
                            try!(HealthCheckVersionDeserializer::deserialize(
                                "HealthCheckVersion",
                                stack
                            ));
                    }
                    "Id" => {
                        obj.id = try!(HealthCheckIdDeserializer::deserialize("Id", stack));
                    }
                    "LinkedService" => {
                        obj.linked_service = Some(try!(LinkedServiceDeserializer::deserialize(
                            "LinkedService",
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
/// <p>A complex type that contains information about the health check.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HealthCheckConfig {
    /// <p>A complex type that identifies the CloudWatch alarm that you want Amazon Route 53 health checkers to use to determine whether this health check is healthy.</p>
    pub alarm_identifier: Option<AlarmIdentifier>,
    /// <p>(CALCULATED Health Checks Only) A complex type that contains one <code>ChildHealthCheck</code> element for each health check that you want to associate with a <code>CALCULATED</code> health check.</p>
    pub child_health_checks: Option<Vec<String>>,
    /// <p>Specify whether you want Amazon Route 53 to send the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>client_hello</code> message during TLS negotiation. This allows the endpoint to respond to <code>HTTPS</code> health check requests with the applicable SSL/TLS certificate.</p> <p>Some endpoints require that <code>HTTPS</code> requests include the host name in the <code>client_hello</code> message. If you don't enable SNI, the status of the health check will be <code>SSL alert handshake_failure</code>. A health check can also have that status for other reasons. If SNI is enabled and you're still getting the error, check the SSL/TLS configuration on your endpoint and confirm that your certificate is valid.</p> <p>The SSL/TLS certificate on your endpoint includes a domain name in the <code>Common Name</code> field and possibly several more in the <code>Subject Alternative Names</code> field. One of the domain names in the certificate should match the value that you specify for <code>FullyQualifiedDomainName</code>. If the endpoint responds to the <code>client_hello</code> message with a certificate that does not include the domain name that you specified in <code>FullyQualifiedDomainName</code>, a health checker will retry the handshake. In the second attempt, the health checker will omit <code>FullyQualifiedDomainName</code> from the <code>client_hello</code> message.</p>
    pub enable_sni: Option<bool>,
    /// <p>The number of consecutive health checks that an endpoint must pass or fail for Amazon Route 53 to change the current status of the endpoint from unhealthy to healthy or vice versa. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html">How Amazon Route 53 Determines Whether an Endpoint Is Healthy</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If you don't specify a value for <code>FailureThreshold</code>, the default value is three health checks.</p>
    pub failure_threshold: Option<i64>,
    /// <p>Amazon Route 53 behavior depends on whether you specify a value for <code>IPAddress</code>.</p> <p> <b>If you specify a value for</b> <code>IPAddress</code>:</p> <p>Amazon Route 53 sends health check requests to the specified IPv4 or IPv6 address and passes the value of <code>FullyQualifiedDomainName</code> in the <code>Host</code> header for all health checks except TCP health checks. This is typically the fully qualified DNS name of the endpoint on which you want Amazon Route 53 to perform health checks.</p> <p>When Amazon Route 53 checks the health of an endpoint, here is how it constructs the <code>Host</code> header:</p> <ul> <li> <p>If you specify a value of <code>80</code> for <code>Port</code> and <code>HTTP</code> or <code>HTTP_STR_MATCH</code> for <code>Type</code>, Amazon Route 53 passes the value of <code>FullyQualifiedDomainName</code> to the endpoint in the Host header. </p> </li> <li> <p>If you specify a value of <code>443</code> for <code>Port</code> and <code>HTTPS</code> or <code>HTTPS_STR_MATCH</code> for <code>Type</code>, Amazon Route 53 passes the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>Host</code> header.</p> </li> <li> <p>If you specify another value for <code>Port</code> and any value except <code>TCP</code> for <code>Type</code>, Amazon Route 53 passes <code>FullyQualifiedDomainName:Port</code> to the endpoint in the <code>Host</code> header.</p> </li> </ul> <p>If you don't specify a value for <code>FullyQualifiedDomainName</code>, Amazon Route 53 substitutes the value of <code>IPAddress</code> in the <code>Host</code> header in each of the preceding cases.</p> <p> <b>If you don't specify a value for <code>IPAddress</code> </b>:</p> <p>Amazon Route 53 sends a DNS request to the domain that you specify for <code>FullyQualifiedDomainName</code> at the interval that you specify for <code>RequestInterval</code>. Using an IPv4 address that DNS returns, Amazon Route 53 then checks the health of the endpoint.</p> <note> <p>If you don't specify a value for <code>IPAddress</code>, Amazon Route 53 uses only IPv4 to send health checks to the endpoint. If there's no resource record set with a type of A for the name that you specify for <code>FullyQualifiedDomainName</code>, the health check fails with a "DNS resolution failed" error.</p> </note> <p>If you want to check the health of weighted, latency, or failover resource record sets and you choose to specify the endpoint only by <code>FullyQualifiedDomainName</code>, we recommend that you create a separate health check for each endpoint. For example, create a health check for each HTTP server that is serving content for www.example.com. For the value of <code>FullyQualifiedDomainName</code>, specify the domain name of the server (such as us-east-2-www.example.com), not the name of the resource record sets (www.example.com).</p> <important> <p>In this configuration, if you create a health check for which the value of <code>FullyQualifiedDomainName</code> matches the name of the resource record sets and you then associate the health check with those resource record sets, health check results will be unpredictable.</p> </important> <p>In addition, if the value that you specify for <code>Type</code> is <code>HTTP</code>, <code>HTTPS</code>, <code>HTTP_STR_MATCH</code>, or <code>HTTPS_STR_MATCH</code>, Amazon Route 53 passes the value of <code>FullyQualifiedDomainName</code> in the <code>Host</code> header, as it does when you specify a value for <code>IPAddress</code>. If the value of <code>Type</code> is <code>TCP</code>, Amazon Route 53 doesn't pass a <code>Host</code> header.</p>
    pub fully_qualified_domain_name: Option<String>,
    /// <p><p>The number of child health checks that are associated with a <code>CALCULATED</code> health that Amazon Route 53 must consider healthy for the <code>CALCULATED</code> health check to be considered healthy. To specify the child health checks that you want to associate with a <code>CALCULATED</code> health check, use the <a>HealthCheckConfig$ChildHealthChecks</a> and <a>HealthCheckConfig$ChildHealthChecks</a> elements.</p> <p>Note the following:</p> <ul> <li> <p>If you specify a number greater than the number of child health checks, Amazon Route 53 always considers this health check to be unhealthy.</p> </li> <li> <p>If you specify <code>0</code>, Amazon Route 53 always considers this health check to be healthy.</p> </li> </ul></p>
    pub health_threshold: Option<i64>,
    /// <p>The IPv4 or IPv6 IP address of the endpoint that you want Amazon Route 53 to perform health checks on. If you don't specify a value for <code>IPAddress</code>, Amazon Route 53 sends a DNS request to resolve the domain name that you specify in <code>FullyQualifiedDomainName</code> at the interval that you specify in <code>RequestInterval</code>. Using an IP address returned by DNS, Amazon Route 53 then checks the health of the endpoint.</p> <p>Use one of the following formats for the value of <code>IPAddress</code>: </p> <ul> <li> <p> <b>IPv4 address</b>: four values between 0 and 255, separated by periods (.), for example, <code>192.0.2.44</code>.</p> </li> <li> <p> <b>IPv6 address</b>: eight groups of four hexadecimal values, separated by colons (:), for example, <code>2001:0db8:85a3:0000:0000:abcd:0001:2345</code>. You can also shorten IPv6 addresses as described in RFC 5952, for example, <code>2001:db8:85a3::abcd:1:2345</code>.</p> </li> </ul> <p>If the endpoint is an EC2 instance, we recommend that you create an Elastic IP address, associate it with your EC2 instance, and specify the Elastic IP address for <code>IPAddress</code>. This ensures that the IP address of your instance will never change.</p> <p>For more information, see <a>HealthCheckConfig$FullyQualifiedDomainName</a>.</p> <p>Constraints: Amazon Route 53 can't check the health of endpoints for which the IP address is in local, private, non-routable, or multicast ranges. For more information about IP addresses for which you can't create health checks, see the following documents:</p> <ul> <li> <p> <a href="https://tools.ietf.org/html/rfc5735">RFC 5735, Special Use IPv4 Addresses</a> </p> </li> <li> <p> <a href="https://tools.ietf.org/html/rfc6598">RFC 6598, IANA-Reserved IPv4 Prefix for Shared Address Space</a> </p> </li> <li> <p> <a href="https://tools.ietf.org/html/rfc5156">RFC 5156, Special-Use IPv6 Addresses</a> </p> </li> </ul> <p>When the value of <code>Type</code> is <code>CALCULATED</code> or <code>CLOUDWATCH_METRIC</code>, omit <code>IPAddress</code>.</p>
    pub ip_address: Option<String>,
    /// <p><p>When CloudWatch has insufficient data about the metric to determine the alarm state, the status that you want Amazon Route 53 to assign to the health check:</p> <ul> <li> <p> <code>Healthy</code>: Amazon Route 53 considers the health check to be healthy.</p> </li> <li> <p> <code>Unhealthy</code>: Amazon Route 53 considers the health check to be unhealthy.</p> </li> <li> <p> <code>LastKnownStatus</code>: Amazon Route 53 uses the status of the health check from the last time that CloudWatch had sufficient data to determine the alarm state. For new health checks that have no last known status, the default status for the health check is healthy.</p> </li> </ul></p>
    pub insufficient_data_health_status: Option<String>,
    /// <p>Specify whether you want Amazon Route 53 to invert the status of a health check, for example, to consider a health check unhealthy when it otherwise would be considered healthy.</p>
    pub inverted: Option<bool>,
    /// <p><p>Specify whether you want Amazon Route 53 to measure the latency between health checkers in multiple AWS regions and your endpoint, and to display CloudWatch latency graphs on the <b>Health Checks</b> page in the Amazon Route 53 console.</p> <important> <p>You can&#39;t change the value of <code>MeasureLatency</code> after you create a health check.</p> </important></p>
    pub measure_latency: Option<bool>,
    /// <p>The port on the endpoint on which you want Amazon Route 53 to perform health checks. Specify a value for <code>Port</code> only when you specify a value for <code>IPAddress</code>.</p>
    pub port: Option<i64>,
    /// <p>A complex type that contains one <code>Region</code> element for each region from which you want Amazon Route 53 health checkers to check the specified endpoint.</p> <p>If you don't specify any regions, Amazon Route 53 health checkers automatically performs checks from all of the regions that are listed under <b>Valid Values</b>.</p> <p>If you update a health check to remove a region that has been performing health checks, Amazon Route 53 will briefly continue to perform checks from that region to ensure that some health checkers are always checking the endpoint (for example, if you replace three regions with four different regions). </p>
    pub regions: Option<Vec<String>>,
    /// <p>The number of seconds between the time that Amazon Route 53 gets a response from your endpoint and the time that it sends the next health check request. Each Amazon Route 53 health checker makes requests at this interval.</p> <important> <p>You can't change the value of <code>RequestInterval</code> after you create a health check.</p> </important> <p>If you don't specify a value for <code>RequestInterval</code>, the default value is <code>30</code> seconds.</p>
    pub request_interval: Option<i64>,
    /// <p>The path, if any, that you want Amazon Route 53 to request when performing health checks. The path can be any value for which your endpoint will return an HTTP status code of 2xx or 3xx when the endpoint is healthy, for example, the file /docs/route53-health-check.html. </p>
    pub resource_path: Option<String>,
    /// <p>If the value of Type is <code>HTTP_STR_MATCH</code> or <code>HTTP_STR_MATCH</code>, the string that you want Amazon Route 53 to search for in the response body from the specified resource. If the string appears in the response body, Amazon Route 53 considers the resource healthy.</p> <p>Amazon Route 53 considers case when searching for <code>SearchString</code> in the response body. </p>
    pub search_string: Option<String>,
    /// <p>The type of health check that you want to create, which indicates how Amazon Route 53 determines whether an endpoint is healthy.</p> <important> <p>You can't change the value of <code>Type</code> after you create a health check.</p> </important> <p>You can create the following types of health checks:</p> <ul> <li> <p> <b>HTTP</b>: Amazon Route 53 tries to establish a TCP connection. If successful, Amazon Route 53 submits an HTTP request and waits for an HTTP status code of 200 or greater and less than 400.</p> </li> <li> <p> <b>HTTPS</b>: Amazon Route 53 tries to establish a TCP connection. If successful, Amazon Route 53 submits an HTTPS request and waits for an HTTP status code of 200 or greater and less than 400.</p> <important> <p>If you specify <code>HTTPS</code> for the value of <code>Type</code>, the endpoint must support TLS v1.0 or later.</p> </important> </li> <li> <p> <b>HTTP_STR_MATCH</b>: Amazon Route 53 tries to establish a TCP connection. If successful, Amazon Route 53 submits an HTTP request and searches the first 5,120 bytes of the response body for the string that you specify in <code>SearchString</code>.</p> </li> <li> <p> <b>HTTPS_STR_MATCH</b>: Amazon Route 53 tries to establish a TCP connection. If successful, Amazon Route 53 submits an <code>HTTPS</code> request and searches the first 5,120 bytes of the response body for the string that you specify in <code>SearchString</code>.</p> </li> <li> <p> <b>TCP</b>: Amazon Route 53 tries to establish a TCP connection.</p> </li> <li> <p> <b>CLOUDWATCH_METRIC</b>: The health check is associated with a CloudWatch alarm. If the state of the alarm is <code>OK</code>, the health check is considered healthy. If the state is <code>ALARM</code>, the health check is considered unhealthy. If CloudWatch doesn't have sufficient data to determine whether the state is <code>OK</code> or <code>ALARM</code>, the health check status depends on the setting for <code>InsufficientDataHealthStatus</code>: <code>Healthy</code>, <code>Unhealthy</code>, or <code>LastKnownStatus</code>. </p> </li> <li> <p> <b>CALCULATED</b>: For health checks that monitor the status of other health checks, Amazon Route 53 adds up the number of health checks that Amazon Route 53 health checkers consider to be healthy and compares that number with the value of <code>HealthThreshold</code>. </p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html">How Amazon Route 53 Determines Whether an Endpoint Is Healthy</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    pub type_: String,
}

struct HealthCheckConfigDeserializer;
impl HealthCheckConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HealthCheckConfig, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = HealthCheckConfig::default();

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
                    "AlarmIdentifier" => {
                        obj.alarm_identifier = Some(try!(
                            AlarmIdentifierDeserializer::deserialize("AlarmIdentifier", stack)
                        ));
                    }
                    "ChildHealthChecks" => {
                        obj.child_health_checks =
                            Some(try!(ChildHealthCheckListDeserializer::deserialize(
                                "ChildHealthChecks",
                                stack
                            )));
                    }
                    "EnableSNI" => {
                        obj.enable_sni =
                            Some(try!(EnableSNIDeserializer::deserialize("EnableSNI", stack)));
                    }
                    "FailureThreshold" => {
                        obj.failure_threshold = Some(try!(
                            FailureThresholdDeserializer::deserialize("FailureThreshold", stack)
                        ));
                    }
                    "FullyQualifiedDomainName" => {
                        obj.fully_qualified_domain_name =
                            Some(try!(FullyQualifiedDomainNameDeserializer::deserialize(
                                "FullyQualifiedDomainName",
                                stack
                            )));
                    }
                    "HealthThreshold" => {
                        obj.health_threshold = Some(try!(
                            HealthThresholdDeserializer::deserialize("HealthThreshold", stack)
                        ));
                    }
                    "IPAddress" => {
                        obj.ip_address =
                            Some(try!(IPAddressDeserializer::deserialize("IPAddress", stack)));
                    }
                    "InsufficientDataHealthStatus" => {
                        obj.insufficient_data_health_status =
                            Some(try!(InsufficientDataHealthStatusDeserializer::deserialize(
                                "InsufficientDataHealthStatus",
                                stack
                            )));
                    }
                    "Inverted" => {
                        obj.inverted =
                            Some(try!(InvertedDeserializer::deserialize("Inverted", stack)));
                    }
                    "MeasureLatency" => {
                        obj.measure_latency = Some(try!(MeasureLatencyDeserializer::deserialize(
                            "MeasureLatency",
                            stack
                        )));
                    }
                    "Port" => {
                        obj.port = Some(try!(PortDeserializer::deserialize("Port", stack)));
                    }
                    "Regions" => {
                        obj.regions = Some(try!(HealthCheckRegionListDeserializer::deserialize(
                            "Regions", stack
                        )));
                    }
                    "RequestInterval" => {
                        obj.request_interval = Some(try!(
                            RequestIntervalDeserializer::deserialize("RequestInterval", stack)
                        ));
                    }
                    "ResourcePath" => {
                        obj.resource_path = Some(try!(ResourcePathDeserializer::deserialize(
                            "ResourcePath",
                            stack
                        )));
                    }
                    "SearchString" => {
                        obj.search_string = Some(try!(SearchStringDeserializer::deserialize(
                            "SearchString",
                            stack
                        )));
                    }
                    "Type" => {
                        obj.type_ = try!(HealthCheckTypeDeserializer::deserialize("Type", stack));
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

pub struct HealthCheckConfigSerializer;
impl HealthCheckConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &HealthCheckConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.alarm_identifier {
            &AlarmIdentifierSerializer::serialize(&mut writer, "AlarmIdentifier", value)?;
        }
        if let Some(ref value) = obj.child_health_checks {
            &ChildHealthCheckListSerializer::serialize(&mut writer, "ChildHealthChecks", value)?;
        }
        if let Some(ref value) = obj.enable_sni {
            writer.write(xml::writer::XmlEvent::start_element("EnableSNI"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.failure_threshold {
            writer.write(xml::writer::XmlEvent::start_element("FailureThreshold"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.fully_qualified_domain_name {
            writer.write(xml::writer::XmlEvent::start_element(
                "FullyQualifiedDomainName",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.health_threshold {
            writer.write(xml::writer::XmlEvent::start_element("HealthThreshold"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.ip_address {
            writer.write(xml::writer::XmlEvent::start_element("IPAddress"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.insufficient_data_health_status {
            writer.write(xml::writer::XmlEvent::start_element(
                "InsufficientDataHealthStatus",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.inverted {
            writer.write(xml::writer::XmlEvent::start_element("Inverted"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.measure_latency {
            writer.write(xml::writer::XmlEvent::start_element("MeasureLatency"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.port {
            writer.write(xml::writer::XmlEvent::start_element("Port"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.regions {
            &HealthCheckRegionListSerializer::serialize(&mut writer, "Regions", value)?;
        }
        if let Some(ref value) = obj.request_interval {
            writer.write(xml::writer::XmlEvent::start_element("RequestInterval"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.resource_path {
            writer.write(xml::writer::XmlEvent::start_element("ResourcePath"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.search_string {
            writer.write(xml::writer::XmlEvent::start_element("SearchString"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Type"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.type_
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct HealthCheckCountDeserializer;
impl HealthCheckCountDeserializer {
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
struct HealthCheckIdDeserializer;
impl HealthCheckIdDeserializer {
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

pub struct HealthCheckIdSerializer;
impl HealthCheckIdSerializer {
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

struct HealthCheckNonceDeserializer;
impl HealthCheckNonceDeserializer {
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

pub struct HealthCheckNonceSerializer;
impl HealthCheckNonceSerializer {
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

/// <p>A complex type that contains the last failure reason as reported by one Amazon Route 53 health checker.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HealthCheckObservation {
    /// <p>The IP address of the Amazon Route 53 health checker that provided the failure reason in <code>StatusReport</code>.</p>
    pub ip_address: Option<String>,
    /// <p>The region of the Amazon Route 53 health checker that provided the status in <code>StatusReport</code>.</p>
    pub region: Option<String>,
    /// <p>A complex type that contains the last failure reason as reported by one Amazon Route 53 health checker and the time of the failed health check.</p>
    pub status_report: Option<StatusReport>,
}

struct HealthCheckObservationDeserializer;
impl HealthCheckObservationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HealthCheckObservation, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = HealthCheckObservation::default();

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
                    "IPAddress" => {
                        obj.ip_address =
                            Some(try!(IPAddressDeserializer::deserialize("IPAddress", stack)));
                    }
                    "Region" => {
                        obj.region = Some(try!(HealthCheckRegionDeserializer::deserialize(
                            "Region", stack
                        )));
                    }
                    "StatusReport" => {
                        obj.status_report = Some(try!(StatusReportDeserializer::deserialize(
                            "StatusReport",
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
struct HealthCheckObservationsDeserializer;
impl HealthCheckObservationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<HealthCheckObservation>, XmlParseError> {
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
                    if name == "HealthCheckObservation" {
                        obj.push(try!(HealthCheckObservationDeserializer::deserialize(
                            "HealthCheckObservation",
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
struct HealthCheckRegionDeserializer;
impl HealthCheckRegionDeserializer {
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

pub struct HealthCheckRegionSerializer;
impl HealthCheckRegionSerializer {
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

struct HealthCheckRegionListDeserializer;
impl HealthCheckRegionListDeserializer {
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
                    if name == "Region" {
                        obj.push(try!(HealthCheckRegionDeserializer::deserialize(
                            "Region", stack
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

pub struct HealthCheckRegionListSerializer;
impl HealthCheckRegionListSerializer {
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
            HealthCheckRegionSerializer::serialize(writer, "Region", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

struct HealthCheckTypeDeserializer;
impl HealthCheckTypeDeserializer {
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

pub struct HealthCheckTypeSerializer;
impl HealthCheckTypeSerializer {
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

struct HealthCheckVersionDeserializer;
impl HealthCheckVersionDeserializer {
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

pub struct HealthCheckVersionSerializer;
impl HealthCheckVersionSerializer {
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

struct HealthChecksDeserializer;
impl HealthChecksDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<HealthCheck>, XmlParseError> {
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
                    if name == "HealthCheck" {
                        obj.push(try!(HealthCheckDeserializer::deserialize(
                            "HealthCheck",
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
struct HealthThresholdDeserializer;
impl HealthThresholdDeserializer {
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

pub struct HealthThresholdSerializer;
impl HealthThresholdSerializer {
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

/// <p>A complex type that contains general information about the hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HostedZone {
    /// <p>The value that you specified for <code>CallerReference</code> when you created the hosted zone.</p>
    pub caller_reference: String,
    /// <p>A complex type that includes the <code>Comment</code> and <code>PrivateZone</code> elements. If you omitted the <code>HostedZoneConfig</code> and <code>Comment</code> elements from the request, the <code>Config</code> and <code>Comment</code> elements don't appear in the response.</p>
    pub config: Option<HostedZoneConfig>,
    /// <p>The ID that Amazon Route 53 assigned to the hosted zone when you created it.</p>
    pub id: String,
    /// <p>If the hosted zone was created by another service, the service that created the hosted zone. When a hosted zone is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    pub linked_service: Option<LinkedService>,
    /// <p>The name of the domain. For public hosted zones, this is the name that you have registered with your DNS registrar.</p> <p>For information about how to specify characters other than <code>a-z</code>, <code>0-9</code>, and <code>-</code> (hyphen) and how to specify internationalized domain names, see <a>CreateHostedZone</a>.</p>
    pub name: String,
    /// <p>The number of resource record sets in the hosted zone.</p>
    pub resource_record_set_count: Option<i64>,
}

struct HostedZoneDeserializer;
impl HostedZoneDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HostedZone, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = HostedZone::default();

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
                            try!(NonceDeserializer::deserialize("CallerReference", stack));
                    }
                    "Config" => {
                        obj.config = Some(try!(HostedZoneConfigDeserializer::deserialize(
                            "Config", stack
                        )));
                    }
                    "Id" => {
                        obj.id = try!(ResourceIdDeserializer::deserialize("Id", stack));
                    }
                    "LinkedService" => {
                        obj.linked_service = Some(try!(LinkedServiceDeserializer::deserialize(
                            "LinkedService",
                            stack
                        )));
                    }
                    "Name" => {
                        obj.name = try!(DNSNameDeserializer::deserialize("Name", stack));
                    }
                    "ResourceRecordSetCount" => {
                        obj.resource_record_set_count =
                            Some(try!(HostedZoneRRSetCountDeserializer::deserialize(
                                "ResourceRecordSetCount",
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
/// <p>A complex type that contains an optional comment about your hosted zone. If you don't want to specify a comment, omit both the <code>HostedZoneConfig</code> and <code>Comment</code> elements.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HostedZoneConfig {
    /// <p>Any comments that you want to include about the hosted zone.</p>
    pub comment: Option<String>,
    /// <p>A value that indicates whether this is a private hosted zone.</p>
    pub private_zone: Option<bool>,
}

struct HostedZoneConfigDeserializer;
impl HostedZoneConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HostedZoneConfig, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = HostedZoneConfig::default();

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
                        obj.comment = Some(try!(ResourceDescriptionDeserializer::deserialize(
                            "Comment", stack
                        )));
                    }
                    "PrivateZone" => {
                        obj.private_zone = Some(try!(IsPrivateZoneDeserializer::deserialize(
                            "PrivateZone",
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

pub struct HostedZoneConfigSerializer;
impl HostedZoneConfigSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &HostedZoneConfig,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.comment {
            writer.write(xml::writer::XmlEvent::start_element("Comment"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.private_zone {
            writer.write(xml::writer::XmlEvent::start_element("PrivateZone"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct HostedZoneCountDeserializer;
impl HostedZoneCountDeserializer {
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
/// <p>A complex type that contains the type of limit that you specified in the request and the current value for that limit.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HostedZoneLimit {
    /// <p><p>The limit that you requested. Valid values include the following:</p> <ul> <li> <p> <b>MAX<em>RRSETS</em>BY<em>ZONE</b>: The maximum number of records that you can create in the specified hosted zone.</p> </li> <li> <p> <b>MAX</em>VPCS<em>ASSOCIATED</em>BY_ZONE</b>: The maximum number of Amazon VPCs that you can associate with the specified private hosted zone.</p> </li> </ul></p>
    pub type_: String,
    /// <p>The current value for the limit that is specified by <code>Type</code>.</p>
    pub value: i64,
}

struct HostedZoneLimitDeserializer;
impl HostedZoneLimitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HostedZoneLimit, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = HostedZoneLimit::default();

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
                    "Type" => {
                        obj.type_ =
                            try!(HostedZoneLimitTypeDeserializer::deserialize("Type", stack));
                    }
                    "Value" => {
                        obj.value = try!(LimitValueDeserializer::deserialize("Value", stack));
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
struct HostedZoneLimitTypeDeserializer;
impl HostedZoneLimitTypeDeserializer {
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

pub struct HostedZoneLimitTypeSerializer;
impl HostedZoneLimitTypeSerializer {
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

struct HostedZoneRRSetCountDeserializer;
impl HostedZoneRRSetCountDeserializer {
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
struct HostedZonesDeserializer;
impl HostedZonesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<HostedZone>, XmlParseError> {
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
                    if name == "HostedZone" {
                        obj.push(try!(HostedZoneDeserializer::deserialize(
                            "HostedZone",
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
struct IPAddressDeserializer;
impl IPAddressDeserializer {
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

pub struct IPAddressSerializer;
impl IPAddressSerializer {
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

struct IPAddressCidrDeserializer;
impl IPAddressCidrDeserializer {
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
struct InsufficientDataHealthStatusDeserializer;
impl InsufficientDataHealthStatusDeserializer {
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

pub struct InsufficientDataHealthStatusSerializer;
impl InsufficientDataHealthStatusSerializer {
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

struct InvertedDeserializer;
impl InvertedDeserializer {
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

pub struct InvertedSerializer;
impl InvertedSerializer {
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

struct IsPrivateZoneDeserializer;
impl IsPrivateZoneDeserializer {
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

pub struct IsPrivateZoneSerializer;
impl IsPrivateZoneSerializer {
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

struct LimitValueDeserializer;
impl LimitValueDeserializer {
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
/// <p>If a health check or hosted zone was created by another service, <code>LinkedService</code> is a complex type that describes the service that created the resource. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LinkedService {
    /// <p>If the health check or hosted zone was created by another service, an optional description that can be provided by the other service. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    pub description: Option<String>,
    /// <p>If the health check or hosted zone was created by another service, the service that created the resource. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    pub service_principal: Option<String>,
}

struct LinkedServiceDeserializer;
impl LinkedServiceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LinkedService, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LinkedService::default();

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
                    "Description" => {
                        obj.description = Some(try!(ResourceDescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "ServicePrincipal" => {
                        obj.service_principal = Some(try!(
                            ServicePrincipalDeserializer::deserialize("ServicePrincipal", stack)
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
/// <p>A request to get a list of geographic locations that Amazon Route 53 supports for geolocation resource record sets. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListGeoLocationsRequest {
    /// <p>(Optional) The maximum number of geolocations to be included in the response body for this request. If more than <code>MaxItems</code> geolocations remain to be listed, then the value of the <code>IsTruncated</code> element in the response is <code>true</code>.</p>
    pub max_items: Option<String>,
    /// <p>The code for the continent with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Amazon Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is true, and if <code>NextContinentCode</code> from the previous response has a value, enter that value in <code>StartContinentCode</code> to return the next page of results.</p> <p>Include <code>StartContinentCode</code> only if you want to list continents. Don't include <code>StartContinentCode</code> when you're listing countries or countries with their subdivisions.</p>
    pub start_continent_code: Option<String>,
    /// <p>The code for the country with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Amazon Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextCountryCode</code> from the previous response has a value, enter that value in <code>StartCountryCode</code> to return the next page of results.</p> <p>Amazon Route 53 uses the two-letter country codes that are specified in <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO standard 3166-1 alpha-2</a>.</p>
    pub start_country_code: Option<String>,
    /// <p>The code for the subdivision (for example, state or province) with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Amazon Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextSubdivisionCode</code> from the previous response has a value, enter that value in <code>StartSubdivisionCode</code> to return the next page of results.</p> <p>To list subdivisions of a country, you must include both <code>StartCountryCode</code> and <code>StartSubdivisionCode</code>.</p>
    pub start_subdivision_code: Option<String>,
}

/// <p>A complex type containing the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListGeoLocationsResponse {
    /// <p>A complex type that contains one <code>GeoLocationDetails</code> element for each location that Amazon Route 53 supports for geolocation.</p>
    pub geo_location_details_list: Vec<GeoLocationDetails>,
    /// <p>A value that indicates whether more locations remain to be listed after the last location in this response. If so, the value of <code>IsTruncated</code> is <code>true</code>. To get more values, submit another request and include the values of <code>NextContinentCode</code>, <code>NextCountryCode</code>, and <code>NextSubdivisionCode</code> in the <code>StartContinentCode</code>, <code>StartCountryCode</code>, and <code>StartSubdivisionCode</code>, as applicable.</p>
    pub is_truncated: bool,
    /// <p>The value that you specified for <code>MaxItems</code> in the request.</p>
    pub max_items: String,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, you can make a follow-up request to display more locations. Enter the value of <code>NextContinentCode</code> in the <code>StartContinentCode</code> parameter in another <code>ListGeoLocations</code> request.</p>
    pub next_continent_code: Option<String>,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, you can make a follow-up request to display more locations. Enter the value of <code>NextCountryCode</code> in the <code>StartCountryCode</code> parameter in another <code>ListGeoLocations</code> request.</p>
    pub next_country_code: Option<String>,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, you can make a follow-up request to display more locations. Enter the value of <code>NextSubdivisionCode</code> in the <code>StartSubdivisionCode</code> parameter in another <code>ListGeoLocations</code> request.</p>
    pub next_subdivision_code: Option<String>,
}

struct ListGeoLocationsResponseDeserializer;
impl ListGeoLocationsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListGeoLocationsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListGeoLocationsResponse::default();

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
                    "GeoLocationDetailsList" => {
                        obj.geo_location_details_list =
                            try!(GeoLocationDetailsListDeserializer::deserialize(
                                "GeoLocationDetailsList",
                                stack
                            ));
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
                    }
                    "MaxItems" => {
                        obj.max_items =
                            try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
                    }
                    "NextContinentCode" => {
                        obj.next_continent_code =
                            Some(try!(GeoLocationContinentCodeDeserializer::deserialize(
                                "NextContinentCode",
                                stack
                            )));
                    }
                    "NextCountryCode" => {
                        obj.next_country_code =
                            Some(try!(GeoLocationCountryCodeDeserializer::deserialize(
                                "NextCountryCode",
                                stack
                            )));
                    }
                    "NextSubdivisionCode" => {
                        obj.next_subdivision_code =
                            Some(try!(GeoLocationSubdivisionCodeDeserializer::deserialize(
                                "NextSubdivisionCode",
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
/// <p>A request to retrieve a list of the health checks that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListHealthChecksRequest {
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more health checks. To get another group, submit another <code>ListHealthChecks</code> request. </p> <p>For the value of <code>marker</code>, specify the value of <code>NextMarker</code> from the previous response, which is the ID of the first health check that Amazon Route 53 will return if you submit another request.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more health checks to get.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of health checks that you want <code>ListHealthChecks</code> to return in response to the current request. Amazon Route 53 returns a maximum of 100 items. If you set <code>MaxItems</code> to a value greater than 100, Amazon Route 53 returns only the first 100 health checks. </p>
    pub max_items: Option<String>,
}

/// <p>A complex type that contains the response to a <code>ListHealthChecks</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListHealthChecksResponse {
    /// <p>A complex type that contains one <code>HealthCheck</code> element for each health check that is associated with the current AWS account.</p>
    pub health_checks: Vec<HealthCheck>,
    /// <p>A flag that indicates whether there are more health checks to be listed. If the response was truncated, you can get the next group of health checks by submitting another <code>ListHealthChecks</code> request and specifying the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub is_truncated: bool,
    /// <p>For the second and subsequent calls to <code>ListHealthChecks</code>, <code>Marker</code> is the value that you specified for the <code>marker</code> parameter in the previous request.</p>
    pub marker: String,
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHealthChecks</code> that produced the current response.</p>
    pub max_items: String,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the first health check that Amazon Route 53 returns if you submit another <code>ListHealthChecks</code> request and specify the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub next_marker: Option<String>,
}

struct ListHealthChecksResponseDeserializer;
impl ListHealthChecksResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListHealthChecksResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListHealthChecksResponse::default();

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
                    "HealthChecks" => {
                        obj.health_checks =
                            try!(HealthChecksDeserializer::deserialize("HealthChecks", stack));
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
                    }
                    "Marker" => {
                        obj.marker = try!(PageMarkerDeserializer::deserialize("Marker", stack));
                    }
                    "MaxItems" => {
                        obj.max_items =
                            try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
                    }
                    "NextMarker" => {
                        obj.next_marker = Some(try!(PageMarkerDeserializer::deserialize(
                            "NextMarker",
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
/// <p>Retrieves a list of the public and private hosted zones that are associated with the current AWS account in ASCII order by domain name. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListHostedZonesByNameRequest {
    /// <p>(Optional) For your first request to <code>ListHostedZonesByName</code>, include the <code>dnsname</code> parameter only if you want to specify the name of the first hosted zone in the response. If you don't include the <code>dnsname</code> parameter, Amazon Route 53 returns all of the hosted zones that were created by the current AWS account, in ASCII order. For subsequent requests, include both <code>dnsname</code> and <code>hostedzoneid</code> parameters. For <code>dnsname</code>, specify the value of <code>NextDNSName</code> from the previous response.</p>
    pub dns_name: Option<String>,
    /// <p>(Optional) For your first request to <code>ListHostedZonesByName</code>, do not include the <code>hostedzoneid</code> parameter.</p> <p>If you have more hosted zones than the value of <code>maxitems</code>, <code>ListHostedZonesByName</code> returns only the first <code>maxitems</code> hosted zones. To get the next group of <code>maxitems</code> hosted zones, submit another request to <code>ListHostedZonesByName</code> and include both <code>dnsname</code> and <code>hostedzoneid</code> parameters. For the value of <code>hostedzoneid</code>, specify the value of the <code>NextHostedZoneId</code> element from the previous response.</p>
    pub hosted_zone_id: Option<String>,
    /// <p>The maximum number of hosted zones to be included in the response body for this request. If you have more than <code>maxitems</code> hosted zones, then the value of the <code>IsTruncated</code> element in the response is true, and the values of <code>NextDNSName</code> and <code>NextHostedZoneId</code> specify the first hosted zone in the next group of <code>maxitems</code> hosted zones. </p>
    pub max_items: Option<String>,
}

/// <p>A complex type that contains the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListHostedZonesByNameResponse {
    /// <p>For the second and subsequent calls to <code>ListHostedZonesByName</code>, <code>DNSName</code> is the value that you specified for the <code>dnsname</code> parameter in the request that produced the current response.</p>
    pub dns_name: Option<String>,
    /// <p>The ID that Amazon Route 53 assigned to the hosted zone when you created it.</p>
    pub hosted_zone_id: Option<String>,
    /// <p>A complex type that contains general information about the hosted zone.</p>
    pub hosted_zones: Vec<HostedZone>,
    /// <p>A flag that indicates whether there are more hosted zones to be listed. If the response was truncated, you can get the next group of <code>maxitems</code> hosted zones by calling <code>ListHostedZonesByName</code> again and specifying the values of <code>NextDNSName</code> and <code>NextHostedZoneId</code> elements in the <code>dnsname</code> and <code>hostedzoneid</code> parameters.</p>
    pub is_truncated: bool,
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHostedZonesByName</code> that produced the current response.</p>
    pub max_items: String,
    /// <p>If <code>IsTruncated</code> is true, the value of <code>NextDNSName</code> is the name of the first hosted zone in the next group of <code>maxitems</code> hosted zones. Call <code>ListHostedZonesByName</code> again and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p> <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    pub next_dns_name: Option<String>,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextHostedZoneId</code> identifies the first hosted zone in the next group of <code>maxitems</code> hosted zones. Call <code>ListHostedZonesByName</code> again and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p> <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    pub next_hosted_zone_id: Option<String>,
}

struct ListHostedZonesByNameResponseDeserializer;
impl ListHostedZonesByNameResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListHostedZonesByNameResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListHostedZonesByNameResponse::default();

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
                    "DNSName" => {
                        obj.dns_name =
                            Some(try!(DNSNameDeserializer::deserialize("DNSName", stack)));
                    }
                    "HostedZoneId" => {
                        obj.hosted_zone_id = Some(try!(ResourceIdDeserializer::deserialize(
                            "HostedZoneId",
                            stack
                        )));
                    }
                    "HostedZones" => {
                        obj.hosted_zones =
                            try!(HostedZonesDeserializer::deserialize("HostedZones", stack));
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
                    }
                    "MaxItems" => {
                        obj.max_items =
                            try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
                    }
                    "NextDNSName" => {
                        obj.next_dns_name =
                            Some(try!(DNSNameDeserializer::deserialize("NextDNSName", stack)));
                    }
                    "NextHostedZoneId" => {
                        obj.next_hosted_zone_id = Some(try!(ResourceIdDeserializer::deserialize(
                            "NextHostedZoneId",
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
/// <p>A request to retrieve a list of the public and private hosted zones that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListHostedZonesRequest {
    /// <p>If you're using reusable delegation sets and you want to list all of the hosted zones that are associated with a reusable delegation set, specify the ID of that reusable delegation set. </p>
    pub delegation_set_id: Option<String>,
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more hosted zones. To get more hosted zones, submit another <code>ListHostedZones</code> request. </p> <p>For the value of <code>marker</code>, specify the value of <code>NextMarker</code> from the previous response, which is the ID of the first hosted zone that Amazon Route 53 will return if you submit another request.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more hosted zones to get.</p>
    pub marker: Option<String>,
    /// <p>(Optional) The maximum number of hosted zones that you want Amazon Route 53 to return. If you have more than <code>maxitems</code> hosted zones, the value of <code>IsTruncated</code> in the response is <code>true</code>, and the value of <code>NextMarker</code> is the hosted zone ID of the first hosted zone that Amazon Route 53 will return if you submit another request.</p>
    pub max_items: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListHostedZonesResponse {
    /// <p>A complex type that contains general information about the hosted zone.</p>
    pub hosted_zones: Vec<HostedZone>,
    /// <p>A flag indicating whether there are more hosted zones to be listed. If the response was truncated, you can get more hosted zones by submitting another <code>ListHostedZones</code> request and specifying the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub is_truncated: bool,
    /// <p>For the second and subsequent calls to <code>ListHostedZones</code>, <code>Marker</code> is the value that you specified for the <code>marker</code> parameter in the request that produced the current response.</p>
    pub marker: String,
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHostedZones</code> that produced the current response.</p>
    pub max_items: String,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the first hosted zone in the next group of hosted zones. Submit another <code>ListHostedZones</code> request, and specify the value of <code>NextMarker</code> from the response in the <code>marker</code> parameter.</p> <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    pub next_marker: Option<String>,
}

struct ListHostedZonesResponseDeserializer;
impl ListHostedZonesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListHostedZonesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListHostedZonesResponse::default();

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
                    "HostedZones" => {
                        obj.hosted_zones =
                            try!(HostedZonesDeserializer::deserialize("HostedZones", stack));
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
                    }
                    "Marker" => {
                        obj.marker = try!(PageMarkerDeserializer::deserialize("Marker", stack));
                    }
                    "MaxItems" => {
                        obj.max_items =
                            try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
                    }
                    "NextMarker" => {
                        obj.next_marker = Some(try!(PageMarkerDeserializer::deserialize(
                            "NextMarker",
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListQueryLoggingConfigsRequest {
    /// <p>(Optional) If you want to list the query logging configuration that is associated with a hosted zone, specify the ID in <code>HostedZoneId</code>. </p> <p>If you don't specify a hosted zone ID, <code>ListQueryLoggingConfigs</code> returns all of the configurations that are associated with the current AWS account.</p>
    pub hosted_zone_id: Option<String>,
    /// <p>(Optional) The maximum number of query logging configurations that you want Amazon Route 53 to return in response to the current request. If the current AWS account has more than <code>MaxResults</code> configurations, use the value of <a>ListQueryLoggingConfigsResponse$NextToken</a> in the response to get the next page of results.</p> <p>If you don't specify a value for <code>MaxResults</code>, Amazon Route 53 returns up to 100 configurations.</p>
    pub max_results: Option<String>,
    /// <p>(Optional) If the current AWS account has more than <code>MaxResults</code> query logging configurations, use <code>NextToken</code> to get the second and subsequent pages of results.</p> <p>For the first <code>ListQueryLoggingConfigs</code> request, omit this value.</p> <p>For the second and subsequent requests, get the value of <code>NextToken</code> from the previous response and specify that value for <code>NextToken</code> in the request.</p>
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListQueryLoggingConfigsResponse {
    /// <p>If a response includes the last of the query logging configurations that are associated with the current AWS account, <code>NextToken</code> doesn't appear in the response.</p> <p>If a response doesn't include the last of the configurations, you can get more configurations by submitting another <a>ListQueryLoggingConfigs</a> request. Get the value of <code>NextToken</code> that Amazon Route 53 returned in the previous response and include it in <code>NextToken</code> in the next request.</p>
    pub next_token: Option<String>,
    /// <p>An array that contains one <a>QueryLoggingConfig</a> element for each configuration for DNS query logging that is associated with the current AWS account.</p>
    pub query_logging_configs: Vec<QueryLoggingConfig>,
}

struct ListQueryLoggingConfigsResponseDeserializer;
impl ListQueryLoggingConfigsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListQueryLoggingConfigsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListQueryLoggingConfigsResponse::default();

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
                    "NextToken" => {
                        obj.next_token = Some(try!(PaginationTokenDeserializer::deserialize(
                            "NextToken",
                            stack
                        )));
                    }
                    "QueryLoggingConfigs" => {
                        obj.query_logging_configs =
                            try!(QueryLoggingConfigsDeserializer::deserialize(
                                "QueryLoggingConfigs",
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
/// <p>A request for the resource record sets that are associated with a specified hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListResourceRecordSetsRequest {
    /// <p>The ID of the hosted zone that contains the resource record sets that you want to list.</p>
    pub hosted_zone_id: String,
    /// <p>(Optional) The maximum number of resource records sets to include in the response body for this request. If the response includes more than <code>maxitems</code> resource record sets, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of the <code>NextRecordName</code> and <code>NextRecordType</code> elements in the response identify the first resource record set in the next group of <code>maxitems</code> resource record sets.</p>
    pub max_items: Option<String>,
    /// <p> <i>Weighted resource record sets only:</i> If results were truncated for a given DNS name and type, specify the value of <code>NextRecordIdentifier</code> from the previous response to get the next resource record set that has the current DNS name and type.</p>
    pub start_record_identifier: Option<String>,
    /// <p>The first name in the lexicographic ordering of resource record sets that you want to list.</p>
    pub start_record_name: Option<String>,
    /// <p>The type of resource record set to begin the record listing from.</p> <p>Valid values for basic resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>NS</code> | <code>PTR</code> | <code>SOA</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code> </p> <p>Values for weighted, latency, geo, and failover resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>PTR</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code> </p> <p>Values for alias resource record sets: </p> <ul> <li> <p> <b>CloudFront distribution</b>: A or AAAA</p> </li> <li> <p> <b>Elastic Beanstalk environment that has a regionalized subdomain</b>: A</p> </li> <li> <p> <b>ELB load balancer</b>: A | AAAA</p> </li> <li> <p> <b>Amazon S3 bucket</b>: A</p> </li> <li> <p> <b>Another resource record set in this hosted zone:</b> The type of the resource record set that the alias references.</p> </li> </ul> <p>Constraint: Specifying <code>type</code> without specifying <code>name</code> returns an <code>InvalidInput</code> error.</p>
    pub start_record_type: Option<String>,
}

/// <p>A complex type that contains list information for the resource record set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListResourceRecordSetsResponse {
    /// <p>A flag that indicates whether more resource record sets remain to be listed. If your results were truncated, you can make a follow-up pagination request by using the <code>NextRecordName</code> element.</p>
    pub is_truncated: bool,
    /// <p>The maximum number of records you requested.</p>
    pub max_items: String,
    /// <p> <i>Weighted, latency, geolocation, and failover resource record sets only</i>: If results were truncated for a given DNS name and type, the value of <code>SetIdentifier</code> for the next resource record set that has the current DNS name and type.</p>
    pub next_record_identifier: Option<String>,
    /// <p>If the results were truncated, the name of the next record in the list.</p> <p>This element is present only if <code>IsTruncated</code> is true. </p>
    pub next_record_name: Option<String>,
    /// <p>If the results were truncated, the type of the next record in the list.</p> <p>This element is present only if <code>IsTruncated</code> is true. </p>
    pub next_record_type: Option<String>,
    /// <p>Information about multiple resource record sets.</p>
    pub resource_record_sets: Vec<ResourceRecordSet>,
}

struct ListResourceRecordSetsResponseDeserializer;
impl ListResourceRecordSetsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListResourceRecordSetsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListResourceRecordSetsResponse::default();

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
                            try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
                    }
                    "MaxItems" => {
                        obj.max_items =
                            try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
                    }
                    "NextRecordIdentifier" => {
                        obj.next_record_identifier =
                            Some(try!(ResourceRecordSetIdentifierDeserializer::deserialize(
                                "NextRecordIdentifier",
                                stack
                            )));
                    }
                    "NextRecordName" => {
                        obj.next_record_name = Some(try!(DNSNameDeserializer::deserialize(
                            "NextRecordName",
                            stack
                        )));
                    }
                    "NextRecordType" => {
                        obj.next_record_type = Some(try!(RRTypeDeserializer::deserialize(
                            "NextRecordType",
                            stack
                        )));
                    }
                    "ResourceRecordSets" => {
                        obj.resource_record_sets =
                            try!(ResourceRecordSetsDeserializer::deserialize(
                                "ResourceRecordSets",
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
/// <p>A request to get a list of the reusable delegation sets that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListReusableDelegationSetsRequest {
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more reusable delegation sets. To get another group, submit another <code>ListReusableDelegationSets</code> request. </p> <p>For the value of <code>marker</code>, specify the value of <code>NextMarker</code> from the previous response, which is the ID of the first reusable delegation set that Amazon Route 53 will return if you submit another request.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more reusable delegation sets to get.</p>
    pub marker: Option<String>,
    /// <p>The number of reusable delegation sets that you want Amazon Route 53 to return in the response to this request. If you specify a value greater than 100, Amazon Route 53 returns only the first 100 reusable delegation sets.</p>
    pub max_items: Option<String>,
}

/// <p>A complex type that contains information about the reusable delegation sets that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListReusableDelegationSetsResponse {
    /// <p>A complex type that contains one <code>DelegationSet</code> element for each reusable delegation set that was created by the current AWS account.</p>
    pub delegation_sets: Vec<DelegationSet>,
    /// <p>A flag that indicates whether there are more reusable delegation sets to be listed.</p>
    pub is_truncated: bool,
    /// <p>For the second and subsequent calls to <code>ListReusableDelegationSets</code>, <code>Marker</code> is the value that you specified for the <code>marker</code> parameter in the request that produced the current response.</p>
    pub marker: String,
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListReusableDelegationSets</code> that produced the current response.</p>
    pub max_items: String,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the next reusable delegation set that Amazon Route 53 will return if you submit another <code>ListReusableDelegationSets</code> request and specify the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub next_marker: Option<String>,
}

struct ListReusableDelegationSetsResponseDeserializer;
impl ListReusableDelegationSetsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListReusableDelegationSetsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListReusableDelegationSetsResponse::default();

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
                    "DelegationSets" => {
                        obj.delegation_sets = try!(DelegationSetsDeserializer::deserialize(
                            "DelegationSets",
                            stack
                        ));
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
                    }
                    "Marker" => {
                        obj.marker = try!(PageMarkerDeserializer::deserialize("Marker", stack));
                    }
                    "MaxItems" => {
                        obj.max_items =
                            try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
                    }
                    "NextMarker" => {
                        obj.next_marker = Some(try!(PageMarkerDeserializer::deserialize(
                            "NextMarker",
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
/// <p>A complex type containing information about a request for a list of the tags that are associated with an individual resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTagsForResourceRequest {
    /// <p>The ID of the resource for which you want to retrieve tags.</p>
    pub resource_id: String,
    /// <p><p>The type of the resource.</p> <ul> <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li> <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li> </ul></p>
    pub resource_type: String,
}

/// <p>A complex type that contains information about the health checks or hosted zones for which you want to list tags.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTagsForResourceResponse {
    /// <p>A <code>ResourceTagSet</code> containing tags associated with the specified resource.</p>
    pub resource_tag_set: ResourceTagSet,
}

struct ListTagsForResourceResponseDeserializer;
impl ListTagsForResourceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTagsForResourceResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListTagsForResourceResponse::default();

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
                    "ResourceTagSet" => {
                        obj.resource_tag_set = try!(ResourceTagSetDeserializer::deserialize(
                            "ResourceTagSet",
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
/// <p>A complex type that contains information about the health checks or hosted zones for which you want to list tags.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTagsForResourcesRequest {
    /// <p>A complex type that contains the ResourceId element for each resource for which you want to get a list of tags.</p>
    pub resource_ids: Vec<String>,
    /// <p><p>The type of the resources.</p> <ul> <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li> <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li> </ul></p>
    pub resource_type: String,
}

pub struct ListTagsForResourcesRequestSerializer;
impl ListTagsForResourcesRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ListTagsForResourcesRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        TagResourceIdListSerializer::serialize(&mut writer, "ResourceIds", &obj.resource_ids)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>A complex type containing tags for the specified resources.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTagsForResourcesResponse {
    /// <p>A list of <code>ResourceTagSet</code>s containing tags associated with the specified resources.</p>
    pub resource_tag_sets: Vec<ResourceTagSet>,
}

struct ListTagsForResourcesResponseDeserializer;
impl ListTagsForResourcesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTagsForResourcesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListTagsForResourcesResponse::default();

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
                    "ResourceTagSets" => {
                        obj.resource_tag_sets = try!(ResourceTagSetListDeserializer::deserialize(
                            "ResourceTagSets",
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
/// <p>A complex type that contains the information about the request to list the traffic policies that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTrafficPoliciesRequest {
    /// <p>(Optional) The maximum number of traffic policies that you want Amazon Route 53 to return in response to this request. If you have more than <code>MaxItems</code> traffic policies, the value of <code>IsTruncated</code> in the response is <code>true</code>, and the value of <code>TrafficPolicyIdMarker</code> is the ID of the first traffic policy that Amazon Route 53 will return if you submit another request.</p>
    pub max_items: Option<String>,
    /// <p>(Conditional) For your first request to <code>ListTrafficPolicies</code>, don't include the <code>TrafficPolicyIdMarker</code> parameter.</p> <p>If you have more traffic policies than the value of <code>MaxItems</code>, <code>ListTrafficPolicies</code> returns only the first <code>MaxItems</code> traffic policies. To get the next group of policies, submit another request to <code>ListTrafficPolicies</code>. For the value of <code>TrafficPolicyIdMarker</code>, specify the value of <code>TrafficPolicyIdMarker</code> that was returned in the previous response.</p>
    pub traffic_policy_id_marker: Option<String>,
}

/// <p>A complex type that contains the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTrafficPoliciesResponse {
    /// <p>A flag that indicates whether there are more traffic policies to be listed. If the response was truncated, you can get the next group of traffic policies by submitting another <code>ListTrafficPolicies</code> request and specifying the value of <code>TrafficPolicyIdMarker</code> in the <code>TrafficPolicyIdMarker</code> request parameter.</p>
    pub is_truncated: bool,
    /// <p>The value that you specified for the <code>MaxItems</code> parameter in the <code>ListTrafficPolicies</code> request that produced the current response.</p>
    pub max_items: String,
    /// <p>If the value of <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyIdMarker</code> is the ID of the first traffic policy in the next group of <code>MaxItems</code> traffic policies.</p>
    pub traffic_policy_id_marker: String,
    /// <p>A list that contains one <code>TrafficPolicySummary</code> element for each traffic policy that was created by the current AWS account.</p>
    pub traffic_policy_summaries: Vec<TrafficPolicySummary>,
}

struct ListTrafficPoliciesResponseDeserializer;
impl ListTrafficPoliciesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTrafficPoliciesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListTrafficPoliciesResponse::default();

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
                            try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
                    }
                    "MaxItems" => {
                        obj.max_items =
                            try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
                    }
                    "TrafficPolicyIdMarker" => {
                        obj.traffic_policy_id_marker =
                            try!(TrafficPolicyIdDeserializer::deserialize(
                                "TrafficPolicyIdMarker",
                                stack
                            ));
                    }
                    "TrafficPolicySummaries" => {
                        obj.traffic_policy_summaries =
                            try!(TrafficPolicySummariesDeserializer::deserialize(
                                "TrafficPolicySummaries",
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
/// <p>A request for the traffic policy instances that you created in a specified hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTrafficPolicyInstancesByHostedZoneRequest {
    /// <p>The ID of the hosted zone that you want to list traffic policy instances for.</p>
    pub hosted_zone_id: String,
    /// <p>The maximum number of traffic policy instances to be included in the response body for this request. If you have more than <code>MaxItems</code> traffic policy instances, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> represent the first traffic policy instance that Amazon Route 53 will return if you submit another request.</p>
    pub max_items: Option<String>,
    /// <p>If the value of <code>IsTruncated</code> in the previous response is true, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>trafficpolicyinstancename</code>, specify the value of <code>TrafficPolicyInstanceNameMarker</code> from the previous response, which is the name of the first traffic policy instance in the next group of traffic policy instances.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub traffic_policy_instance_name_marker: Option<String>,
    /// <p>If the value of <code>IsTruncated</code> in the previous response is true, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>trafficpolicyinstancetype</code>, specify the value of <code>TrafficPolicyInstanceTypeMarker</code> from the previous response, which is the type of the first traffic policy instance in the next group of traffic policy instances.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub traffic_policy_instance_type_marker: Option<String>,
}

/// <p>A complex type that contains the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTrafficPolicyInstancesByHostedZoneResponse {
    /// <p>A flag that indicates whether there are more traffic policy instances to be listed. If the response was truncated, you can get the next group of traffic policy instances by submitting another <code>ListTrafficPolicyInstancesByHostedZone</code> request and specifying the values of <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> in the corresponding request parameters.</p>
    pub is_truncated: bool,
    /// <p>The value that you specified for the <code>MaxItems</code> parameter in the <code>ListTrafficPolicyInstancesByHostedZone</code> request that produced the current response.</p>
    pub max_items: String,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyInstanceNameMarker</code> is the name of the first traffic policy instance in the next group of traffic policy instances.</p>
    pub traffic_policy_instance_name_marker: Option<String>,
    /// <p>If <code>IsTruncated</code> is true, <code>TrafficPolicyInstanceTypeMarker</code> is the DNS type of the resource record sets that are associated with the first traffic policy instance in the next group of traffic policy instances.</p>
    pub traffic_policy_instance_type_marker: Option<String>,
    /// <p>A list that contains one <code>TrafficPolicyInstance</code> element for each traffic policy instance that matches the elements in the request. </p>
    pub traffic_policy_instances: Vec<TrafficPolicyInstance>,
}

struct ListTrafficPolicyInstancesByHostedZoneResponseDeserializer;
impl ListTrafficPolicyInstancesByHostedZoneResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTrafficPolicyInstancesByHostedZoneResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListTrafficPolicyInstancesByHostedZoneResponse::default();

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
                            try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
                    }
                    "MaxItems" => {
                        obj.max_items =
                            try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
                    }
                    "TrafficPolicyInstanceNameMarker" => {
                        obj.traffic_policy_instance_name_marker =
                            Some(try!(DNSNameDeserializer::deserialize(
                                "TrafficPolicyInstanceNameMarker",
                                stack
                            )));
                    }
                    "TrafficPolicyInstanceTypeMarker" => {
                        obj.traffic_policy_instance_type_marker =
                            Some(try!(RRTypeDeserializer::deserialize(
                                "TrafficPolicyInstanceTypeMarker",
                                stack
                            )));
                    }
                    "TrafficPolicyInstances" => {
                        obj.traffic_policy_instances =
                            try!(TrafficPolicyInstancesDeserializer::deserialize(
                                "TrafficPolicyInstances",
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
/// <p>A complex type that contains the information about the request to list your traffic policy instances.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTrafficPolicyInstancesByPolicyRequest {
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstancesByPolicy</code> request. </p> <p>For the value of <code>hostedzoneid</code>, specify the value of <code>HostedZoneIdMarker</code> from the previous response, which is the hosted zone ID of the first traffic policy instance that Amazon Route 53 will return if you submit another request.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub hosted_zone_id_marker: Option<String>,
    /// <p>The maximum number of traffic policy instances to be included in the response body for this request. If you have more than <code>MaxItems</code> traffic policy instances, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> represent the first traffic policy instance that Amazon Route 53 will return if you submit another request.</p>
    pub max_items: Option<String>,
    /// <p>The ID of the traffic policy for which you want to list traffic policy instances.</p>
    pub traffic_policy_id: String,
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstancesByPolicy</code> request.</p> <p>For the value of <code>trafficpolicyinstancename</code>, specify the value of <code>TrafficPolicyInstanceNameMarker</code> from the previous response, which is the name of the first traffic policy instance that Amazon Route 53 will return if you submit another request.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub traffic_policy_instance_name_marker: Option<String>,
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstancesByPolicy</code> request.</p> <p>For the value of <code>trafficpolicyinstancetype</code>, specify the value of <code>TrafficPolicyInstanceTypeMarker</code> from the previous response, which is the name of the first traffic policy instance that Amazon Route 53 will return if you submit another request.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub traffic_policy_instance_type_marker: Option<String>,
    /// <p>The version of the traffic policy for which you want to list traffic policy instances. The version must be associated with the traffic policy that is specified by <code>TrafficPolicyId</code>.</p>
    pub traffic_policy_version: i64,
}

/// <p>A complex type that contains the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTrafficPolicyInstancesByPolicyResponse {
    /// <p>If <code>IsTruncated</code> is <code>true</code>, <code>HostedZoneIdMarker</code> is the ID of the hosted zone of the first traffic policy instance in the next group of traffic policy instances.</p>
    pub hosted_zone_id_marker: Option<String>,
    /// <p>A flag that indicates whether there are more traffic policy instances to be listed. If the response was truncated, you can get the next group of traffic policy instances by calling <code>ListTrafficPolicyInstancesByPolicy</code> again and specifying the values of the <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> elements in the corresponding request parameters.</p>
    pub is_truncated: bool,
    /// <p>The value that you specified for the <code>MaxItems</code> parameter in the call to <code>ListTrafficPolicyInstancesByPolicy</code> that produced the current response.</p>
    pub max_items: String,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyInstanceNameMarker</code> is the name of the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>
    pub traffic_policy_instance_name_marker: Option<String>,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyInstanceTypeMarker</code> is the DNS type of the resource record sets that are associated with the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>
    pub traffic_policy_instance_type_marker: Option<String>,
    /// <p>A list that contains one <code>TrafficPolicyInstance</code> element for each traffic policy instance that matches the elements in the request.</p>
    pub traffic_policy_instances: Vec<TrafficPolicyInstance>,
}

struct ListTrafficPolicyInstancesByPolicyResponseDeserializer;
impl ListTrafficPolicyInstancesByPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTrafficPolicyInstancesByPolicyResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListTrafficPolicyInstancesByPolicyResponse::default();

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
                    "HostedZoneIdMarker" => {
                        obj.hosted_zone_id_marker = Some(try!(
                            ResourceIdDeserializer::deserialize("HostedZoneIdMarker", stack)
                        ));
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
                    }
                    "MaxItems" => {
                        obj.max_items =
                            try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
                    }
                    "TrafficPolicyInstanceNameMarker" => {
                        obj.traffic_policy_instance_name_marker =
                            Some(try!(DNSNameDeserializer::deserialize(
                                "TrafficPolicyInstanceNameMarker",
                                stack
                            )));
                    }
                    "TrafficPolicyInstanceTypeMarker" => {
                        obj.traffic_policy_instance_type_marker =
                            Some(try!(RRTypeDeserializer::deserialize(
                                "TrafficPolicyInstanceTypeMarker",
                                stack
                            )));
                    }
                    "TrafficPolicyInstances" => {
                        obj.traffic_policy_instances =
                            try!(TrafficPolicyInstancesDeserializer::deserialize(
                                "TrafficPolicyInstances",
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
/// <p>A request to get information about the traffic policy instances that you created by using the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTrafficPolicyInstancesRequest {
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>HostedZoneId</code>, specify the value of <code>HostedZoneIdMarker</code> from the previous response, which is the hosted zone ID of the first traffic policy instance in the next group of traffic policy instances.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub hosted_zone_id_marker: Option<String>,
    /// <p>The maximum number of traffic policy instances that you want Amazon Route 53 to return in response to a <code>ListTrafficPolicyInstances</code> request. If you have more than <code>MaxItems</code> traffic policy instances, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> represent the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>
    pub max_items: Option<String>,
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>trafficpolicyinstancename</code>, specify the value of <code>TrafficPolicyInstanceNameMarker</code> from the previous response, which is the name of the first traffic policy instance in the next group of traffic policy instances.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub traffic_policy_instance_name_marker: Option<String>,
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>trafficpolicyinstancetype</code>, specify the value of <code>TrafficPolicyInstanceTypeMarker</code> from the previous response, which is the type of the first traffic policy instance in the next group of traffic policy instances.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    pub traffic_policy_instance_type_marker: Option<String>,
}

/// <p>A complex type that contains the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTrafficPolicyInstancesResponse {
    /// <p>If <code>IsTruncated</code> is <code>true</code>, <code>HostedZoneIdMarker</code> is the ID of the hosted zone of the first traffic policy instance that Amazon Route 53 will return if you submit another <code>ListTrafficPolicyInstances</code> request. </p>
    pub hosted_zone_id_marker: Option<String>,
    /// <p>A flag that indicates whether there are more traffic policy instances to be listed. If the response was truncated, you can get more traffic policy instances by calling <code>ListTrafficPolicyInstances</code> again and specifying the values of the <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> in the corresponding request parameters.</p>
    pub is_truncated: bool,
    /// <p>The value that you specified for the <code>MaxItems</code> parameter in the call to <code>ListTrafficPolicyInstances</code> that produced the current response.</p>
    pub max_items: String,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyInstanceNameMarker</code> is the name of the first traffic policy instance that Amazon Route 53 will return if you submit another <code>ListTrafficPolicyInstances</code> request. </p>
    pub traffic_policy_instance_name_marker: Option<String>,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyInstanceTypeMarker</code> is the DNS type of the resource record sets that are associated with the first traffic policy instance that Amazon Route 53 will return if you submit another <code>ListTrafficPolicyInstances</code> request. </p>
    pub traffic_policy_instance_type_marker: Option<String>,
    /// <p>A list that contains one <code>TrafficPolicyInstance</code> element for each traffic policy instance that matches the elements in the request.</p>
    pub traffic_policy_instances: Vec<TrafficPolicyInstance>,
}

struct ListTrafficPolicyInstancesResponseDeserializer;
impl ListTrafficPolicyInstancesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTrafficPolicyInstancesResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListTrafficPolicyInstancesResponse::default();

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
                    "HostedZoneIdMarker" => {
                        obj.hosted_zone_id_marker = Some(try!(
                            ResourceIdDeserializer::deserialize("HostedZoneIdMarker", stack)
                        ));
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
                    }
                    "MaxItems" => {
                        obj.max_items =
                            try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
                    }
                    "TrafficPolicyInstanceNameMarker" => {
                        obj.traffic_policy_instance_name_marker =
                            Some(try!(DNSNameDeserializer::deserialize(
                                "TrafficPolicyInstanceNameMarker",
                                stack
                            )));
                    }
                    "TrafficPolicyInstanceTypeMarker" => {
                        obj.traffic_policy_instance_type_marker =
                            Some(try!(RRTypeDeserializer::deserialize(
                                "TrafficPolicyInstanceTypeMarker",
                                stack
                            )));
                    }
                    "TrafficPolicyInstances" => {
                        obj.traffic_policy_instances =
                            try!(TrafficPolicyInstancesDeserializer::deserialize(
                                "TrafficPolicyInstances",
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
/// <p>A complex type that contains the information about the request to list your traffic policies.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTrafficPolicyVersionsRequest {
    /// <p>Specify the value of <code>Id</code> of the traffic policy for which you want to list all versions.</p>
    pub id: String,
    /// <p>The maximum number of traffic policy versions that you want Amazon Route 53 to include in the response body for this request. If the specified traffic policy has more than <code>MaxItems</code> versions, the value of <code>IsTruncated</code> in the response is <code>true</code>, and the value of the <code>TrafficPolicyVersionMarker</code> element is the ID of the first version that Amazon Route 53 will return if you submit another request.</p>
    pub max_items: Option<String>,
    /// <p>For your first request to <code>ListTrafficPolicyVersions</code>, don't include the <code>TrafficPolicyVersionMarker</code> parameter.</p> <p>If you have more traffic policy versions than the value of <code>MaxItems</code>, <code>ListTrafficPolicyVersions</code> returns only the first group of <code>MaxItems</code> versions. To get more traffic policy versions, submit another <code>ListTrafficPolicyVersions</code> request. For the value of <code>TrafficPolicyVersionMarker</code>, specify the value of <code>TrafficPolicyVersionMarker</code> in the previous response.</p>
    pub traffic_policy_version_marker: Option<String>,
}

/// <p>A complex type that contains the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTrafficPolicyVersionsResponse {
    /// <p>A flag that indicates whether there are more traffic policies to be listed. If the response was truncated, you can get the next group of traffic policies by submitting another <code>ListTrafficPolicyVersions</code> request and specifying the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub is_truncated: bool,
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the <code>ListTrafficPolicyVersions</code> request that produced the current response.</p>
    pub max_items: String,
    /// <p>A list that contains one <code>TrafficPolicy</code> element for each traffic policy version that is associated with the specified traffic policy.</p>
    pub traffic_policies: Vec<TrafficPolicy>,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>TrafficPolicyVersionMarker</code> identifies the first traffic policy that Amazon Route 53 will return if you submit another request. Call <code>ListTrafficPolicyVersions</code> again and specify the value of <code>TrafficPolicyVersionMarker</code> in the <code>TrafficPolicyVersionMarker</code> request parameter.</p> <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>
    pub traffic_policy_version_marker: String,
}

struct ListTrafficPolicyVersionsResponseDeserializer;
impl ListTrafficPolicyVersionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTrafficPolicyVersionsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListTrafficPolicyVersionsResponse::default();

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
                            try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
                    }
                    "MaxItems" => {
                        obj.max_items =
                            try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
                    }
                    "TrafficPolicies" => {
                        obj.traffic_policies = try!(TrafficPoliciesDeserializer::deserialize(
                            "TrafficPolicies",
                            stack
                        ));
                    }
                    "TrafficPolicyVersionMarker" => {
                        obj.traffic_policy_version_marker =
                            try!(TrafficPolicyVersionMarkerDeserializer::deserialize(
                                "TrafficPolicyVersionMarker",
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
/// <p>A complex type that contains information about that can be associated with your hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListVPCAssociationAuthorizationsRequest {
    /// <p>The ID of the hosted zone for which you want a list of VPCs that can be associated with the hosted zone.</p>
    pub hosted_zone_id: String,
    /// <p> <i>Optional</i>: An integer that specifies the maximum number of VPCs that you want Amazon Route 53 to return. If you don't specify a value for <code>MaxResults</code>, Amazon Route 53 returns up to 50 VPCs per page.</p>
    pub max_results: Option<String>,
    /// <p> <i>Optional</i>: If a response includes a <code>NextToken</code> element, there are more VPCs that can be associated with the specified hosted zone. To get the next page of results, submit another request, and include the value of <code>NextToken</code> from the response in the <code>nexttoken</code> parameter in another <code>ListVPCAssociationAuthorizations</code> request.</p>
    pub next_token: Option<String>,
}

/// <p>A complex type that contains the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListVPCAssociationAuthorizationsResponse {
    /// <p>The ID of the hosted zone that you can associate the listed VPCs with.</p>
    pub hosted_zone_id: String,
    /// <p>When the response includes a <code>NextToken</code> element, there are more VPCs that can be associated with the specified hosted zone. To get the next page of VPCs, submit another <code>ListVPCAssociationAuthorizations</code> request, and include the value of the <code>NextToken</code> element from the response in the <code>nexttoken</code> request parameter.</p>
    pub next_token: Option<String>,
    /// <p>The list of VPCs that are authorized to be associated with the specified hosted zone.</p>
    pub vp_cs: Vec<VPC>,
}

struct ListVPCAssociationAuthorizationsResponseDeserializer;
impl ListVPCAssociationAuthorizationsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListVPCAssociationAuthorizationsResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListVPCAssociationAuthorizationsResponse::default();

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
                    "HostedZoneId" => {
                        obj.hosted_zone_id =
                            try!(ResourceIdDeserializer::deserialize("HostedZoneId", stack));
                    }
                    "NextToken" => {
                        obj.next_token = Some(try!(PaginationTokenDeserializer::deserialize(
                            "NextToken",
                            stack
                        )));
                    }
                    "VPCs" => {
                        obj.vp_cs = try!(VPCsDeserializer::deserialize("VPCs", stack));
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

pub struct MaxResultsSerializer;
impl MaxResultsSerializer {
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

struct MeasureLatencyDeserializer;
impl MeasureLatencyDeserializer {
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

pub struct MeasureLatencySerializer;
impl MeasureLatencySerializer {
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

struct MessageDeserializer;
impl MessageDeserializer {
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
struct MetricNameDeserializer;
impl MetricNameDeserializer {
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
struct NameserverDeserializer;
impl NameserverDeserializer {
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
struct NamespaceDeserializer;
impl NamespaceDeserializer {
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
struct NonceDeserializer;
impl NonceDeserializer {
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

pub struct NonceSerializer;
impl NonceSerializer {
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

struct PageMarkerDeserializer;
impl PageMarkerDeserializer {
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

pub struct PageMarkerSerializer;
impl PageMarkerSerializer {
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

struct PageMaxItemsDeserializer;
impl PageMaxItemsDeserializer {
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

pub struct PageMaxItemsSerializer;
impl PageMaxItemsSerializer {
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

struct PageTruncatedDeserializer;
impl PageTruncatedDeserializer {
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
struct PaginationTokenDeserializer;
impl PaginationTokenDeserializer {
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

pub struct PaginationTokenSerializer;
impl PaginationTokenSerializer {
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

struct PeriodDeserializer;
impl PeriodDeserializer {
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
struct PortDeserializer;
impl PortDeserializer {
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

pub struct PortSerializer;
impl PortSerializer {
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

/// <p>A complex type that contains information about a configuration for DNS query logging.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct QueryLoggingConfig {
    /// <p>The Amazon Resource Name (ARN) of the CloudWatch Logs log group that Amazon Route 53 is publishing logs to.</p>
    pub cloud_watch_logs_log_group_arn: String,
    /// <p>The ID of the hosted zone that CloudWatch Logs is logging queries for. </p>
    pub hosted_zone_id: String,
    /// <p>The ID for a configuration for DNS query logging.</p>
    pub id: String,
}

struct QueryLoggingConfigDeserializer;
impl QueryLoggingConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueryLoggingConfig, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = QueryLoggingConfig::default();

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
                    "CloudWatchLogsLogGroupArn" => {
                        obj.cloud_watch_logs_log_group_arn =
                            try!(CloudWatchLogsLogGroupArnDeserializer::deserialize(
                                "CloudWatchLogsLogGroupArn",
                                stack
                            ));
                    }
                    "HostedZoneId" => {
                        obj.hosted_zone_id =
                            try!(ResourceIdDeserializer::deserialize("HostedZoneId", stack));
                    }
                    "Id" => {
                        obj.id = try!(QueryLoggingConfigIdDeserializer::deserialize("Id", stack));
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
struct QueryLoggingConfigIdDeserializer;
impl QueryLoggingConfigIdDeserializer {
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

pub struct QueryLoggingConfigIdSerializer;
impl QueryLoggingConfigIdSerializer {
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

struct QueryLoggingConfigsDeserializer;
impl QueryLoggingConfigsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<QueryLoggingConfig>, XmlParseError> {
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
                    if name == "QueryLoggingConfig" {
                        obj.push(try!(QueryLoggingConfigDeserializer::deserialize(
                            "QueryLoggingConfig",
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
struct RDataDeserializer;
impl RDataDeserializer {
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

pub struct RDataSerializer;
impl RDataSerializer {
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

struct RRTypeDeserializer;
impl RRTypeDeserializer {
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

pub struct RRTypeSerializer;
impl RRTypeSerializer {
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

struct RecordDataDeserializer;
impl RecordDataDeserializer {
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
                    if name == "RecordDataEntry" {
                        obj.push(try!(RecordDataEntryDeserializer::deserialize(
                            "RecordDataEntry",
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
struct RecordDataEntryDeserializer;
impl RecordDataEntryDeserializer {
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
struct RequestIntervalDeserializer;
impl RequestIntervalDeserializer {
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

pub struct RequestIntervalSerializer;
impl RequestIntervalSerializer {
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

pub struct ResettableElementNameSerializer;
impl ResettableElementNameSerializer {
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

pub struct ResettableElementNameListSerializer;
impl ResettableElementNameListSerializer {
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
            ResettableElementNameSerializer::serialize(writer, "ResettableElementName", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

struct ResourceDescriptionDeserializer;
impl ResourceDescriptionDeserializer {
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

pub struct ResourceDescriptionSerializer;
impl ResourceDescriptionSerializer {
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

struct ResourceIdDeserializer;
impl ResourceIdDeserializer {
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

pub struct ResourceIdSerializer;
impl ResourceIdSerializer {
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

struct ResourcePathDeserializer;
impl ResourcePathDeserializer {
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

pub struct ResourcePathSerializer;
impl ResourcePathSerializer {
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

/// <p><p>Information specific to the resource record.</p> <note> <p>If you&#39;re creating an alias resource record set, omit <code>ResourceRecord</code>.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResourceRecord {
    /// <p><p>The current or new DNS record value, not to exceed 4,000 characters. In the case of a <code>DELETE</code> action, if the current value does not match the actual value, an error is returned. For descriptions about how to format <code>Value</code> for different record types, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/ResourceRecordTypes.html">Supported DNS Resource Record Types</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>You can specify more than one value for all record types except <code>CNAME</code> and <code>SOA</code>. </p> <note> <p>If you&#39;re creating an alias resource record set, omit <code>Value</code>.</p> </note></p>
    pub value: String,
}

struct ResourceRecordDeserializer;
impl ResourceRecordDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceRecord, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ResourceRecord::default();

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
                    "Value" => {
                        obj.value = try!(RDataDeserializer::deserialize("Value", stack));
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

pub struct ResourceRecordSerializer;
impl ResourceRecordSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ResourceRecord,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Value"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.value
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Information about the resource record set to create or delete.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResourceRecordSet {
    /// <p><p> <i>Alias resource record sets only:</i> Information about the CloudFront distribution, AWS Elastic Beanstalk environment, ELB load balancer, Amazon S3 bucket, or Amazon Route 53 resource record set to which you&#39;re redirecting queries. The AWS Elastic Beanstalk environment must have a regionalized subdomain.</p> <p>If you&#39;re creating resource records sets for a private hosted zone, note the following:</p> <ul> <li> <p>You can&#39;t create alias resource record sets for CloudFront distributions in a private hosted zone.</p> </li> <li> <p>Creating geolocation alias resource record sets or latency alias resource record sets in a private hosted zone is unsupported.</p> </li> <li> <p>For information about creating failover resource record sets in a private hosted zone, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-private-hosted-zones.html">Configuring Failover in a Private Hosted Zone</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </li> </ul></p>
    pub alias_target: Option<AliasTarget>,
    /// <p><p> <i>Failover resource record sets only:</i> To configure failover, you add the <code>Failover</code> element to two resource record sets. For one resource record set, you specify <code>PRIMARY</code> as the value for <code>Failover</code>; for the other resource record set, you specify <code>SECONDARY</code>. In addition, you include the <code>HealthCheckId</code> element and specify the health check that you want Amazon Route 53 to perform for each resource record set.</p> <p>Except where noted, the following failover behaviors assume that you have included the <code>HealthCheckId</code> element in both resource record sets:</p> <ul> <li> <p>When the primary resource record set is healthy, Amazon Route 53 responds to DNS queries with the applicable value from the primary resource record set regardless of the health of the secondary resource record set.</p> </li> <li> <p>When the primary resource record set is unhealthy and the secondary resource record set is healthy, Amazon Route 53 responds to DNS queries with the applicable value from the secondary resource record set.</p> </li> <li> <p>When the secondary resource record set is unhealthy, Amazon Route 53 responds to DNS queries with the applicable value from the primary resource record set regardless of the health of the primary resource record set.</p> </li> <li> <p>If you omit the <code>HealthCheckId</code> element for the secondary resource record set, and if the primary resource record set is unhealthy, Amazon Route 53 always responds to DNS queries with the applicable value from the secondary resource record set. This is true regardless of the health of the associated endpoint.</p> </li> </ul> <p>You can&#39;t create non-failover resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements as failover resource record sets.</p> <p>For failover alias resource record sets, you must also include the <code>EvaluateTargetHealth</code> element and set the value to true.</p> <p>For more information about configuring failover for Amazon Route 53, see the following topics in the <i>Amazon Route 53 Developer Guide</i>: </p> <ul> <li> <p> <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover.html">Amazon Route 53 Health Checks and DNS Failover</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-private-hosted-zones.html">Configuring Failover in a Private Hosted Zone</a> </p> </li> </ul></p>
    pub failover: Option<String>,
    /// <p> <i>Geo location resource record sets only:</i> A complex type that lets you control how Amazon Route 53 responds to DNS queries based on the geographic origin of the query. For example, if you want all queries from Africa to be routed to a web server with an IP address of <code>192.0.2.111</code>, create a resource record set with a <code>Type</code> of <code>A</code> and a <code>ContinentCode</code> of <code>AF</code>.</p> <note> <p>Creating geolocation and geolocation alias resource record sets in private hosted zones is not supported.</p> </note> <p>If you create separate resource record sets for overlapping geographic regions (for example, one resource record set for a continent and one for a country on the same continent), priority goes to the smallest geographic region. This allows you to route most queries for a continent to one resource and to route queries for a country on that continent to a different resource.</p> <p>You can't create two geolocation resource record sets that specify the same geographic location.</p> <p>The value <code>*</code> in the <code>CountryCode</code> element matches all geographic locations that aren't specified in other geolocation resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements.</p> <important> <p>Geolocation works by mapping IP addresses to locations. However, some IP addresses aren't mapped to geographic locations, so even if you create geolocation resource record sets that cover all seven continents, Amazon Route 53 will receive some DNS queries from locations that it can't identify. We recommend that you create a resource record set for which the value of <code>CountryCode</code> is <code>*</code>, which handles both queries that come from locations for which you haven't created geolocation resource record sets and queries from IP addresses that aren't mapped to a location. If you don't create a <code>*</code> resource record set, Amazon Route 53 returns a "no answer" response for queries from those locations.</p> </important> <p>You can't create non-geolocation resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements as geolocation resource record sets.</p>
    pub geo_location: Option<GeoLocation>,
    /// <p><p>If you want Amazon Route 53 to return this resource record set in response to a DNS query only when a health check is passing, include the <code>HealthCheckId</code> element and specify the ID of the applicable health check.</p> <p>Amazon Route 53 determines whether a resource record set is healthy based on one of the following:</p> <ul> <li> <p>By periodically sending a request to the endpoint that is specified in the health check</p> </li> <li> <p>By aggregating the status of a specified group of health checks (calculated health checks)</p> </li> <li> <p>By determining the current state of a CloudWatch alarm (CloudWatch metric health checks)</p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html">How Amazon Route 53 Determines Whether an Endpoint Is Healthy</a>.</p> <p>The <code>HealthCheckId</code> element is only useful when Amazon Route 53 is choosing between two or more resource record sets to respond to a DNS query, and you want Amazon Route 53 to base the choice in part on the status of a health check. Configuring health checks only makes sense in the following configurations:</p> <ul> <li> <p>You&#39;re checking the health of the resource record sets in a group of weighted, latency, geolocation, or failover resource record sets, and you specify health check IDs for all of the resource record sets. If the health check for one resource record set specifies an endpoint that is not healthy, Amazon Route 53 stops responding to queries using the value for that resource record set.</p> </li> <li> <p>You set <code>EvaluateTargetHealth</code> to true for the resource record sets in a group of alias, weighted alias, latency alias, geolocation alias, or failover alias resource record sets, and you specify health check IDs for all of the resource record sets that are referenced by the alias resource record sets.</p> </li> </ul> <important> <p>Amazon Route 53 doesn&#39;t check the health of the endpoint specified in the resource record set, for example, the endpoint specified by the IP address in the <code>Value</code> element. When you add a <code>HealthCheckId</code> element to a resource record set, Amazon Route 53 checks the health of the endpoint that you specified in the health check. </p> </important> <p>For geolocation resource record sets, if an endpoint is unhealthy, Amazon Route 53 looks for a resource record set for the larger, associated geographic region. For example, suppose you have resource record sets for a state in the United States, for the United States, for North America, and for all locations. If the endpoint for the state resource record set is unhealthy, Amazon Route 53 checks the resource record sets for the United States, for North America, and for all locations (a resource record set for which the value of <code>CountryCode</code> is <code>*</code>), in that order, until it finds a resource record set for which the endpoint is healthy. </p> <p>If your health checks specify the endpoint only by domain name, we recommend that you create a separate health check for each endpoint. For example, create a health check for each <code>HTTP</code> server that is serving content for <code>www.example.com</code>. For the value of <code>FullyQualifiedDomainName</code>, specify the domain name of the server (such as <code>us-east-2-www.example.com</code>), not the name of the resource record sets (example.com).</p> <important> <p>n this configuration, if you create a health check for which the value of <code>FullyQualifiedDomainName</code> matches the name of the resource record sets and then associate the health check with those resource record sets, health check results will be unpredictable.</p> </important> <p>For more information, see the following topics in the <i>Amazon Route 53 Developer Guide</i>:</p> <ul> <li> <p> <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover.html">Amazon Route 53 Health Checks and DNS Failover</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-private-hosted-zones.html">Configuring Failover in a Private Hosted Zone</a> </p> </li> </ul></p>
    pub health_check_id: Option<String>,
    /// <p> <i>Multivalue answer resource record sets only</i>: To route traffic approximately randomly to multiple resources, such as web servers, create one multivalue answer record for each resource and specify <code>true</code> for <code>MultiValueAnswer</code>. Note the following:</p> <ul> <li> <p>If you associate a health check with a multivalue answer resource record set, Amazon Route 53 responds to DNS queries with the corresponding IP address only when the health check is healthy.</p> </li> <li> <p>If you don't associate a health check with a multivalue answer record, Amazon Route 53 always considers the record to be healthy.</p> </li> <li> <p>Amazon Route 53 responds to DNS queries with up to eight healthy records; if you have eight or fewer healthy records, Amazon Route 53 responds to all DNS queries with all the healthy records.</p> </li> <li> <p>If you have more than eight healthy records, Amazon Route 53 responds to different DNS resolvers with different combinations of healthy records.</p> </li> <li> <p>When all records are unhealthy, Amazon Route 53 responds to DNS queries with up to eight unhealthy records.</p> </li> <li> <p>If a resource becomes unavailable after a resolver caches a response, client software typically tries another of the IP addresses in the response.</p> </li> </ul> <p>You can't create multivalue answer alias records.</p>
    pub multi_value_answer: Option<bool>,
    /// <p>The name of the domain you want to perform the action on.</p> <p>Enter a fully qualified domain name, for example, <code>www.example.com</code>. You can optionally include a trailing dot. If you omit the trailing dot, Amazon Route 53 still assumes that the domain name that you specify is fully qualified. This means that Amazon Route 53 treats <code>www.example.com</code> (without a trailing dot) and <code>www.example.com.</code> (with a trailing dot) as identical.</p> <p>For information about how to specify characters other than <code>a-z</code>, <code>0-9</code>, and <code>-</code> (hyphen) and how to specify internationalized domain names, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DomainNameFormat.html">DNS Domain Name Format</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>You can use the asterisk (*) wildcard to replace the leftmost label in a domain name, for example, <code>*.example.com</code>. Note the following:</p> <ul> <li> <p>The * must replace the entire label. For example, you can't specify <code>*prod.example.com</code> or <code>prod*.example.com</code>.</p> </li> <li> <p>The * can't replace any of the middle labels, for example, marketing.*.example.com.</p> </li> <li> <p>If you include * in any position other than the leftmost label in a domain name, DNS treats it as an * character (ASCII 42), not as a wildcard.</p> <important> <p>You can't use the * wildcard for resource records sets that have a type of NS.</p> </important> </li> </ul> <p>You can use the * wildcard as the leftmost label in a domain name, for example, <code>*.example.com</code>. You can't use an * for one of the middle labels, for example, <code>marketing.*.example.com</code>. In addition, the * must replace the entire label; for example, you can't specify <code>prod*.example.com</code>.</p>
    pub name: String,
    /// <p><p> <i>Latency-based resource record sets only:</i> The Amazon EC2 Region where you created the resource that this resource record set refers to. The resource typically is an AWS resource, such as an EC2 instance or an ELB load balancer, and is referred to by an IP address or a DNS domain name, depending on the record type.</p> <note> <p>Creating latency and latency alias resource record sets in private hosted zones is not supported.</p> </note> <p>When Amazon Route 53 receives a DNS query for a domain name and type for which you have created latency resource record sets, Amazon Route 53 selects the latency resource record set that has the lowest latency between the end user and the associated Amazon EC2 Region. Amazon Route 53 then returns the value that is associated with the selected resource record set.</p> <p>Note the following:</p> <ul> <li> <p>You can only specify one <code>ResourceRecord</code> per latency resource record set.</p> </li> <li> <p>You can only create one latency resource record set for each Amazon EC2 Region.</p> </li> <li> <p>You aren&#39;t required to create latency resource record sets for all Amazon EC2 Regions. Amazon Route 53 will choose the region with the best latency from among the regions that you create latency resource record sets for.</p> </li> <li> <p>You can&#39;t create non-latency resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements as latency resource record sets.</p> </li> </ul></p>
    pub region: Option<String>,
    /// <p><p>Information about the resource records to act upon.</p> <note> <p>If you&#39;re creating an alias resource record set, omit <code>ResourceRecords</code>.</p> </note></p>
    pub resource_records: Option<Vec<ResourceRecord>>,
    /// <p> <i>Weighted, Latency, Geo, and Failover resource record sets only:</i> An identifier that differentiates among multiple resource record sets that have the same combination of DNS name and type. The value of <code>SetIdentifier</code> must be unique for each resource record set that has the same combination of DNS name and type. Omit <code>SetIdentifier</code> for any other types of record sets.</p>
    pub set_identifier: Option<String>,
    /// <p><p>The resource record cache time to live (TTL), in seconds. Note the following:</p> <ul> <li> <p>If you&#39;re creating or updating an alias resource record set, omit <code>TTL</code>. Amazon Route 53 uses the value of <code>TTL</code> for the alias target. </p> </li> <li> <p>If you&#39;re associating this resource record set with a health check (if you&#39;re adding a <code>HealthCheckId</code> element), we recommend that you specify a <code>TTL</code> of 60 seconds or less so clients respond quickly to changes in health status.</p> </li> <li> <p>All of the resource record sets in a group of weighted resource record sets must have the same value for <code>TTL</code>.</p> </li> <li> <p>If a group of weighted resource record sets includes one or more weighted alias resource record sets for which the alias target is an ELB load balancer, we recommend that you specify a <code>TTL</code> of 60 seconds for all of the non-alias weighted resource record sets that have the same name and type. Values other than 60 seconds (the TTL for load balancers) will change the effect of the values that you specify for <code>Weight</code>.</p> </li> </ul></p>
    pub ttl: Option<i64>,
    /// <p><p>When you create a traffic policy instance, Amazon Route 53 automatically creates a resource record set. <code>TrafficPolicyInstanceId</code> is the ID of the traffic policy instance that Amazon Route 53 created this resource record set for.</p> <important> <p>To delete the resource record set that is associated with a traffic policy instance, use <code>DeleteTrafficPolicyInstance</code>. Amazon Route 53 will delete the resource record set automatically. If you delete the resource record set by using <code>ChangeResourceRecordSets</code>, Amazon Route 53 doesn&#39;t automatically delete the traffic policy instance, and you&#39;ll continue to be charged for it even though it&#39;s no longer in use. </p> </important></p>
    pub traffic_policy_instance_id: Option<String>,
    /// <p><p>The DNS record type. For information about different record types and how data is encoded for them, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/ResourceRecordTypes.html">Supported DNS Resource Record Types</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>Valid values for basic resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>NS</code> | <code>PTR</code> | <code>SOA</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code> </p> <p>Values for weighted, latency, geolocation, and failover resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>PTR</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code>. When creating a group of weighted, latency, geolocation, or failover resource record sets, specify the same value for all of the resource record sets in the group.</p> <p>Valid values for multivalue answer resource record sets: <code>A</code> | <code>AAAA</code> | <code>MX</code> | <code>NAPTR</code> | <code>PTR</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code> </p> <note> <p>SPF records were formerly used to verify the identity of the sender of email messages. However, we no longer recommend that you create resource record sets for which the value of <code>Type</code> is <code>SPF</code>. RFC 7208, <i>Sender Policy Framework (SPF) for Authorizing Use of Domains in Email, Version 1</i>, has been updated to say, &quot;...[I]ts existence and mechanism defined in [RFC4408] have led to some interoperability issues. Accordingly, its use is no longer appropriate for SPF version 1; implementations are not to use it.&quot; In RFC 7208, see section 14.1, <a href="http://tools.ietf.org/html/rfc7208#section-14.1">The SPF DNS Record Type</a>.</p> </note> <p>Values for alias resource record sets:</p> <ul> <li> <p> <b>CloudFront distributions:</b> <code>A</code> </p> <p>If IPv6 is enabled for the distribution, create two resource record sets to route traffic to your distribution, one with a value of <code>A</code> and one with a value of <code>AAAA</code>. </p> </li> <li> <p> <b>AWS Elastic Beanstalk environment that has a regionalized subdomain</b>: <code>A</code> </p> </li> <li> <p> <b>ELB load balancers:</b> <code>A</code> | <code>AAAA</code> </p> </li> <li> <p> <b>Amazon S3 buckets:</b> <code>A</code> </p> </li> <li> <p> <b>Another resource record set in this hosted zone:</b> Specify the type of the resource record set that you&#39;re creating the alias for. All values are supported except <code>NS</code> and <code>SOA</code>.</p> </li> </ul></p>
    pub type_: String,
    /// <p><p> <i>Weighted resource record sets only:</i> Among resource record sets that have the same combination of DNS name and type, a value that determines the proportion of DNS queries that Amazon Route 53 responds to using the current resource record set. Amazon Route 53 calculates the sum of the weights for the resource record sets that have the same combination of DNS name and type. Amazon Route 53 then responds to queries based on the ratio of a resource&#39;s weight to the total. Note the following:</p> <ul> <li> <p>You must specify a value for the <code>Weight</code> element for every weighted resource record set.</p> </li> <li> <p>You can only specify one <code>ResourceRecord</code> per weighted resource record set.</p> </li> <li> <p>You can&#39;t create latency, failover, or geolocation resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements as weighted resource record sets.</p> </li> <li> <p>You can create a maximum of 100 weighted resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements.</p> </li> <li> <p>For weighted (but not weighted alias) resource record sets, if you set <code>Weight</code> to <code>0</code> for a resource record set, Amazon Route 53 never responds to queries with the applicable value for that resource record set. However, if you set <code>Weight</code> to <code>0</code> for all resource record sets that have the same combination of DNS name and type, traffic is routed to all resources with equal probability.</p> <p>The effect of setting <code>Weight</code> to <code>0</code> is different when you associate health checks with weighted resource record sets. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-configuring-options.html">Options for Configuring Amazon Route 53 Active-Active and Active-Passive Failover</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </li> </ul></p>
    pub weight: Option<i64>,
}

struct ResourceRecordSetDeserializer;
impl ResourceRecordSetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceRecordSet, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ResourceRecordSet::default();

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
                    "AliasTarget" => {
                        obj.alias_target = Some(try!(AliasTargetDeserializer::deserialize(
                            "AliasTarget",
                            stack
                        )));
                    }
                    "Failover" => {
                        obj.failover = Some(try!(
                            ResourceRecordSetFailoverDeserializer::deserialize("Failover", stack)
                        ));
                    }
                    "GeoLocation" => {
                        obj.geo_location = Some(try!(GeoLocationDeserializer::deserialize(
                            "GeoLocation",
                            stack
                        )));
                    }
                    "HealthCheckId" => {
                        obj.health_check_id = Some(try!(HealthCheckIdDeserializer::deserialize(
                            "HealthCheckId",
                            stack
                        )));
                    }
                    "MultiValueAnswer" => {
                        obj.multi_value_answer = Some(try!(
                            ResourceRecordSetMultiValueAnswerDeserializer::deserialize(
                                "MultiValueAnswer",
                                stack
                            )
                        ));
                    }
                    "Name" => {
                        obj.name = try!(DNSNameDeserializer::deserialize("Name", stack));
                    }
                    "Region" => {
                        obj.region = Some(try!(ResourceRecordSetRegionDeserializer::deserialize(
                            "Region", stack
                        )));
                    }
                    "ResourceRecords" => {
                        obj.resource_records = Some(try!(
                            ResourceRecordsDeserializer::deserialize("ResourceRecords", stack)
                        ));
                    }
                    "SetIdentifier" => {
                        obj.set_identifier =
                            Some(try!(ResourceRecordSetIdentifierDeserializer::deserialize(
                                "SetIdentifier",
                                stack
                            )));
                    }
                    "TTL" => {
                        obj.ttl = Some(try!(TTLDeserializer::deserialize("TTL", stack)));
                    }
                    "TrafficPolicyInstanceId" => {
                        obj.traffic_policy_instance_id =
                            Some(try!(TrafficPolicyInstanceIdDeserializer::deserialize(
                                "TrafficPolicyInstanceId",
                                stack
                            )));
                    }
                    "Type" => {
                        obj.type_ = try!(RRTypeDeserializer::deserialize("Type", stack));
                    }
                    "Weight" => {
                        obj.weight = Some(try!(ResourceRecordSetWeightDeserializer::deserialize(
                            "Weight", stack
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

pub struct ResourceRecordSetSerializer;
impl ResourceRecordSetSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ResourceRecordSet,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.alias_target {
            &AliasTargetSerializer::serialize(&mut writer, "AliasTarget", value)?;
        }
        if let Some(ref value) = obj.failover {
            writer.write(xml::writer::XmlEvent::start_element("Failover"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.geo_location {
            &GeoLocationSerializer::serialize(&mut writer, "GeoLocation", value)?;
        }
        if let Some(ref value) = obj.health_check_id {
            writer.write(xml::writer::XmlEvent::start_element("HealthCheckId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.multi_value_answer {
            writer.write(xml::writer::XmlEvent::start_element("MultiValueAnswer"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Name"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.name
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.region {
            writer.write(xml::writer::XmlEvent::start_element("Region"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.resource_records {
            &ResourceRecordsSerializer::serialize(&mut writer, "ResourceRecords", value)?;
        }
        if let Some(ref value) = obj.set_identifier {
            writer.write(xml::writer::XmlEvent::start_element("SetIdentifier"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.ttl {
            writer.write(xml::writer::XmlEvent::start_element("TTL"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.traffic_policy_instance_id {
            writer.write(xml::writer::XmlEvent::start_element(
                "TrafficPolicyInstanceId",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Type"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.type_
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.weight {
            writer.write(xml::writer::XmlEvent::start_element("Weight"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ResourceRecordSetFailoverDeserializer;
impl ResourceRecordSetFailoverDeserializer {
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

pub struct ResourceRecordSetFailoverSerializer;
impl ResourceRecordSetFailoverSerializer {
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

struct ResourceRecordSetIdentifierDeserializer;
impl ResourceRecordSetIdentifierDeserializer {
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

pub struct ResourceRecordSetIdentifierSerializer;
impl ResourceRecordSetIdentifierSerializer {
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

struct ResourceRecordSetMultiValueAnswerDeserializer;
impl ResourceRecordSetMultiValueAnswerDeserializer {
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

pub struct ResourceRecordSetMultiValueAnswerSerializer;
impl ResourceRecordSetMultiValueAnswerSerializer {
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

struct ResourceRecordSetRegionDeserializer;
impl ResourceRecordSetRegionDeserializer {
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

pub struct ResourceRecordSetRegionSerializer;
impl ResourceRecordSetRegionSerializer {
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

struct ResourceRecordSetWeightDeserializer;
impl ResourceRecordSetWeightDeserializer {
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

pub struct ResourceRecordSetWeightSerializer;
impl ResourceRecordSetWeightSerializer {
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

struct ResourceRecordSetsDeserializer;
impl ResourceRecordSetsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ResourceRecordSet>, XmlParseError> {
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
                    if name == "ResourceRecordSet" {
                        obj.push(try!(ResourceRecordSetDeserializer::deserialize(
                            "ResourceRecordSet",
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
struct ResourceRecordsDeserializer;
impl ResourceRecordsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ResourceRecord>, XmlParseError> {
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
                    if name == "ResourceRecord" {
                        obj.push(try!(ResourceRecordDeserializer::deserialize(
                            "ResourceRecord",
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

pub struct ResourceRecordsSerializer;
impl ResourceRecordsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<ResourceRecord>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            ResourceRecordSerializer::serialize(writer, "ResourceRecord", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p>A complex type containing a resource and its associated tags.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResourceTagSet {
    /// <p>The ID for the specified resource.</p>
    pub resource_id: Option<String>,
    /// <p><p>The type of the resource.</p> <ul> <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li> <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li> </ul></p>
    pub resource_type: Option<String>,
    /// <p>The tags associated with the specified resource.</p>
    pub tags: Option<Vec<Tag>>,
}

struct ResourceTagSetDeserializer;
impl ResourceTagSetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceTagSet, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ResourceTagSet::default();

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
                    "ResourceId" => {
                        obj.resource_id = Some(try!(TagResourceIdDeserializer::deserialize(
                            "ResourceId",
                            stack
                        )));
                    }
                    "ResourceType" => {
                        obj.resource_type = Some(try!(TagResourceTypeDeserializer::deserialize(
                            "ResourceType",
                            stack
                        )));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
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
struct ResourceTagSetListDeserializer;
impl ResourceTagSetListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ResourceTagSet>, XmlParseError> {
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
                    if name == "ResourceTagSet" {
                        obj.push(try!(ResourceTagSetDeserializer::deserialize(
                            "ResourceTagSet",
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
/// <p>A complex type that contains the type of limit that you specified in the request and the current value for that limit.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReusableDelegationSetLimit {
    /// <p>The limit that you requested: <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code>, the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p>
    pub type_: String,
    /// <p>The current value for the <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code> limit.</p>
    pub value: i64,
}

struct ReusableDelegationSetLimitDeserializer;
impl ReusableDelegationSetLimitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReusableDelegationSetLimit, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReusableDelegationSetLimit::default();

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
                    "Type" => {
                        obj.type_ = try!(ReusableDelegationSetLimitTypeDeserializer::deserialize(
                            "Type", stack
                        ));
                    }
                    "Value" => {
                        obj.value = try!(LimitValueDeserializer::deserialize("Value", stack));
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
struct ReusableDelegationSetLimitTypeDeserializer;
impl ReusableDelegationSetLimitTypeDeserializer {
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

pub struct ReusableDelegationSetLimitTypeSerializer;
impl ReusableDelegationSetLimitTypeSerializer {
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

struct SearchStringDeserializer;
impl SearchStringDeserializer {
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

pub struct SearchStringSerializer;
impl SearchStringSerializer {
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

struct ServicePrincipalDeserializer;
impl ServicePrincipalDeserializer {
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
struct StatisticDeserializer;
impl StatisticDeserializer {
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
struct StatusDeserializer;
impl StatusDeserializer {
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
/// <p>A complex type that contains the status that one Amazon Route 53 health checker reports and the time of the health check.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StatusReport {
    /// <p>The date and time that the health checker performed the health check in <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601 format</a> and Coordinated Universal Time (UTC). For example, the value <code>2017-03-27T17:48:16.751Z</code> represents March 27, 2017 at 17:48:16.751 UTC.</p>
    pub checked_time: Option<String>,
    /// <p>A description of the status of the health check endpoint as reported by one of the Amazon Route 53 health checkers.</p>
    pub status: Option<String>,
}

struct StatusReportDeserializer;
impl StatusReportDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StatusReport, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StatusReport::default();

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
                    "CheckedTime" => {
                        obj.checked_time = Some(try!(TimeStampDeserializer::deserialize(
                            "CheckedTime",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(StatusDeserializer::deserialize("Status", stack)));
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

pub struct SubnetMaskSerializer;
impl SubnetMaskSerializer {
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

struct TTLDeserializer;
impl TTLDeserializer {
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

pub struct TTLSerializer;
impl TTLSerializer {
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

/// <p>A complex type that contains information about a tag that you want to add or edit for the specified health check or hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tag {
    /// <p><p>The value of <code>Key</code> depends on the operation that you want to perform:</p> <ul> <li> <p> <b>Add a tag to a health check or hosted zone</b>: <code>Key</code> is the name that you want to give the new tag.</p> </li> <li> <p> <b>Edit a tag</b>: <code>Key</code> is the name of the tag that you want to change the <code>Value</code> for.</p> </li> <li> <p> <b> Delete a key</b>: <code>Key</code> is the name of the tag you want to remove.</p> </li> <li> <p> <b>Give a name to a health check</b>: Edit the default <code>Name</code> tag. In the Amazon Route 53 console, the list of your health checks includes a <b>Name</b> column that lets you see the name that you&#39;ve given to each health check.</p> </li> </ul></p>
    pub key: Option<String>,
    /// <p><p>The value of <code>Value</code> depends on the operation that you want to perform:</p> <ul> <li> <p> <b>Add a tag to a health check or hosted zone</b>: <code>Value</code> is the value that you want to give the new tag.</p> </li> <li> <p> <b>Edit a tag</b>: <code>Value</code> is the new value that you want to assign the tag.</p> </li> </ul></p>
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
                        obj.key = Some(try!(TagKeyDeserializer::deserialize("Key", stack)));
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
        if let Some(ref value) = obj.key {
            writer.write(xml::writer::XmlEvent::start_element("Key"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
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

struct TagResourceIdDeserializer;
impl TagResourceIdDeserializer {
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

pub struct TagResourceIdSerializer;
impl TagResourceIdSerializer {
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

pub struct TagResourceIdListSerializer;
impl TagResourceIdListSerializer {
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
            TagResourceIdSerializer::serialize(writer, "ResourceId", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

struct TagResourceTypeDeserializer;
impl TagResourceTypeDeserializer {
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

pub struct TagResourceTypeSerializer;
impl TagResourceTypeSerializer {
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

/// <p>Gets the value that Amazon Route 53 returns in response to a DNS request for a specified record name and type. You can optionally specify the IP address of a DNS resolver, an EDNS0 client subnet IP address, and a subnet mask. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TestDNSAnswerRequest {
    /// <p>If the resolver that you specified for resolverip supports EDNS0, specify the IPv4 or IPv6 address of a client in the applicable location, for example, <code>192.0.2.44</code> or <code>2001:db8:85a3::8a2e:370:7334</code>.</p>
    pub edns0_client_subnet_ip: Option<String>,
    /// <p>If you specify an IP address for <code>edns0clientsubnetip</code>, you can optionally specify the number of bits of the IP address that you want the checking tool to include in the DNS query. For example, if you specify <code>192.0.2.44</code> for <code>edns0clientsubnetip</code> and <code>24</code> for <code>edns0clientsubnetmask</code>, the checking tool will simulate a request from 192.0.2.0/24. The default value is 24 bits for IPv4 addresses and 64 bits for IPv6 addresses.</p>
    pub edns0_client_subnet_mask: Option<String>,
    /// <p>The ID of the hosted zone that you want Amazon Route 53 to simulate a query for.</p>
    pub hosted_zone_id: String,
    /// <p>The name of the resource record set that you want Amazon Route 53 to simulate a query for.</p>
    pub record_name: String,
    /// <p>The type of the resource record set.</p>
    pub record_type: String,
    /// <p>If you want to simulate a request from a specific DNS resolver, specify the IP address for that resolver. If you omit this value, <code>TestDnsAnswer</code> uses the IP address of a DNS resolver in the AWS US East (N. Virginia) Region (<code>us-east-1</code>).</p>
    pub resolver_ip: Option<String>,
}

/// <p>A complex type that contains the response to a <code>TestDNSAnswer</code> request. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TestDNSAnswerResponse {
    /// <p>The Amazon Route 53 name server used to respond to the request.</p>
    pub nameserver: String,
    /// <p>The protocol that Amazon Route 53 used to respond to the request, either <code>UDP</code> or <code>TCP</code>. </p>
    pub protocol: String,
    /// <p>A list that contains values that Amazon Route 53 returned for this resource record set.</p>
    pub record_data: Vec<String>,
    /// <p>The name of the resource record set that you submitted a request for.</p>
    pub record_name: String,
    /// <p>The type of the resource record set that you submitted a request for.</p>
    pub record_type: String,
    /// <p>A code that indicates whether the request is valid or not. The most common response code is <code>NOERROR</code>, meaning that the request is valid. If the response is not valid, Amazon Route 53 returns a response code that describes the error. For a list of possible response codes, see <a href="http://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-6">DNS RCODES</a> on the IANA website. </p>
    pub response_code: String,
}

struct TestDNSAnswerResponseDeserializer;
impl TestDNSAnswerResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TestDNSAnswerResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TestDNSAnswerResponse::default();

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
                    "Nameserver" => {
                        obj.nameserver =
                            try!(NameserverDeserializer::deserialize("Nameserver", stack));
                    }
                    "Protocol" => {
                        obj.protocol = try!(TransportProtocolDeserializer::deserialize(
                            "Protocol", stack
                        ));
                    }
                    "RecordData" => {
                        obj.record_data =
                            try!(RecordDataDeserializer::deserialize("RecordData", stack));
                    }
                    "RecordName" => {
                        obj.record_name =
                            try!(DNSNameDeserializer::deserialize("RecordName", stack));
                    }
                    "RecordType" => {
                        obj.record_type =
                            try!(RRTypeDeserializer::deserialize("RecordType", stack));
                    }
                    "ResponseCode" => {
                        obj.response_code =
                            try!(DNSRCodeDeserializer::deserialize("ResponseCode", stack));
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
struct ThresholdDeserializer;
impl ThresholdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<f64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct TimeStampDeserializer;
impl TimeStampDeserializer {
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
struct TrafficPoliciesDeserializer;
impl TrafficPoliciesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TrafficPolicy>, XmlParseError> {
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
                    if name == "TrafficPolicy" {
                        obj.push(try!(TrafficPolicyDeserializer::deserialize(
                            "TrafficPolicy",
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
/// <p>A complex type that contains settings for a traffic policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TrafficPolicy {
    /// <p>The comment that you specify in the <code>CreateTrafficPolicy</code> request, if any.</p>
    pub comment: Option<String>,
    /// <p>The definition of a traffic policy in JSON format. You specify the JSON document to use for a new traffic policy in the <code>CreateTrafficPolicy</code> request. For more information about the JSON format, see <a href="http://docs.aws.amazon.com/Route53/latest/APIReference/api-policies-traffic-policy-document-format.html">Traffic Policy Document Format</a>.</p>
    pub document: String,
    /// <p>The ID that Amazon Route 53 assigned to a traffic policy when you created it.</p>
    pub id: String,
    /// <p>The name that you specified when you created the traffic policy.</p>
    pub name: String,
    /// <p>The DNS type of the resource record sets that Amazon Route 53 creates when you use a traffic policy to create a traffic policy instance.</p>
    pub type_: String,
    /// <p>The version number that Amazon Route 53 assigns to a traffic policy. For a new traffic policy, the value of <code>Version</code> is always 1.</p>
    pub version: i64,
}

struct TrafficPolicyDeserializer;
impl TrafficPolicyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TrafficPolicy, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TrafficPolicy::default();

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
                        obj.comment = Some(try!(TrafficPolicyCommentDeserializer::deserialize(
                            "Comment", stack
                        )));
                    }
                    "Document" => {
                        obj.document = try!(TrafficPolicyDocumentDeserializer::deserialize(
                            "Document", stack
                        ));
                    }
                    "Id" => {
                        obj.id = try!(TrafficPolicyIdDeserializer::deserialize("Id", stack));
                    }
                    "Name" => {
                        obj.name = try!(TrafficPolicyNameDeserializer::deserialize("Name", stack));
                    }
                    "Type" => {
                        obj.type_ = try!(RRTypeDeserializer::deserialize("Type", stack));
                    }
                    "Version" => {
                        obj.version = try!(TrafficPolicyVersionDeserializer::deserialize(
                            "Version", stack
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
struct TrafficPolicyCommentDeserializer;
impl TrafficPolicyCommentDeserializer {
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

pub struct TrafficPolicyCommentSerializer;
impl TrafficPolicyCommentSerializer {
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

struct TrafficPolicyDocumentDeserializer;
impl TrafficPolicyDocumentDeserializer {
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

pub struct TrafficPolicyDocumentSerializer;
impl TrafficPolicyDocumentSerializer {
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

struct TrafficPolicyIdDeserializer;
impl TrafficPolicyIdDeserializer {
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

pub struct TrafficPolicyIdSerializer;
impl TrafficPolicyIdSerializer {
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

/// <p>A complex type that contains settings for the new traffic policy instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TrafficPolicyInstance {
    /// <p>The ID of the hosted zone that Amazon Route 53 created resource record sets in.</p>
    pub hosted_zone_id: String,
    /// <p>The ID that Amazon Route 53 assigned to the new traffic policy instance.</p>
    pub id: String,
    /// <p>If <code>State</code> is <code>Failed</code>, an explanation of the reason for the failure. If <code>State</code> is another value, <code>Message</code> is empty.</p>
    pub message: String,
    /// <p>The DNS name, such as www.example.com, for which Amazon Route 53 responds to queries by using the resource record sets that are associated with this traffic policy instance. </p>
    pub name: String,
    /// <p><p>The value of <code>State</code> is one of the following values:</p> <dl> <dt>Applied</dt> <dd> <p>Amazon Route 53 has finished creating resource record sets, and changes have propagated to all Amazon Route 53 edge locations.</p> </dd> <dt>Creating</dt> <dd> <p>Amazon Route 53 is creating the resource record sets. Use <code>GetTrafficPolicyInstance</code> to confirm that the <code>CreateTrafficPolicyInstance</code> request completed successfully.</p> </dd> <dt>Failed</dt> <dd> <p>Amazon Route 53 wasn&#39;t able to create or update the resource record sets. When the value of <code>State</code> is <code>Failed</code>, see <code>Message</code> for an explanation of what caused the request to fail.</p> </dd> </dl></p>
    pub state: String,
    /// <p>The TTL that Amazon Route 53 assigned to all of the resource record sets that it created in the specified hosted zone.</p>
    pub ttl: i64,
    /// <p>The ID of the traffic policy that Amazon Route 53 used to create resource record sets in the specified hosted zone.</p>
    pub traffic_policy_id: String,
    /// <p>The DNS type that Amazon Route 53 assigned to all of the resource record sets that it created for this traffic policy instance. </p>
    pub traffic_policy_type: String,
    /// <p>The version of the traffic policy that Amazon Route 53 used to create resource record sets in the specified hosted zone.</p>
    pub traffic_policy_version: i64,
}

struct TrafficPolicyInstanceDeserializer;
impl TrafficPolicyInstanceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TrafficPolicyInstance, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TrafficPolicyInstance::default();

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
                    "HostedZoneId" => {
                        obj.hosted_zone_id =
                            try!(ResourceIdDeserializer::deserialize("HostedZoneId", stack));
                    }
                    "Id" => {
                        obj.id = try!(TrafficPolicyInstanceIdDeserializer::deserialize(
                            "Id", stack
                        ));
                    }
                    "Message" => {
                        obj.message = try!(MessageDeserializer::deserialize("Message", stack));
                    }
                    "Name" => {
                        obj.name = try!(DNSNameDeserializer::deserialize("Name", stack));
                    }
                    "State" => {
                        obj.state = try!(TrafficPolicyInstanceStateDeserializer::deserialize(
                            "State", stack
                        ));
                    }
                    "TTL" => {
                        obj.ttl = try!(TTLDeserializer::deserialize("TTL", stack));
                    }
                    "TrafficPolicyId" => {
                        obj.traffic_policy_id = try!(TrafficPolicyIdDeserializer::deserialize(
                            "TrafficPolicyId",
                            stack
                        ));
                    }
                    "TrafficPolicyType" => {
                        obj.traffic_policy_type =
                            try!(RRTypeDeserializer::deserialize("TrafficPolicyType", stack));
                    }
                    "TrafficPolicyVersion" => {
                        obj.traffic_policy_version =
                            try!(TrafficPolicyVersionDeserializer::deserialize(
                                "TrafficPolicyVersion",
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
struct TrafficPolicyInstanceCountDeserializer;
impl TrafficPolicyInstanceCountDeserializer {
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
struct TrafficPolicyInstanceIdDeserializer;
impl TrafficPolicyInstanceIdDeserializer {
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

pub struct TrafficPolicyInstanceIdSerializer;
impl TrafficPolicyInstanceIdSerializer {
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

struct TrafficPolicyInstanceStateDeserializer;
impl TrafficPolicyInstanceStateDeserializer {
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
struct TrafficPolicyInstancesDeserializer;
impl TrafficPolicyInstancesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TrafficPolicyInstance>, XmlParseError> {
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
                    if name == "TrafficPolicyInstance" {
                        obj.push(try!(TrafficPolicyInstanceDeserializer::deserialize(
                            "TrafficPolicyInstance",
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
struct TrafficPolicyNameDeserializer;
impl TrafficPolicyNameDeserializer {
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

pub struct TrafficPolicyNameSerializer;
impl TrafficPolicyNameSerializer {
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

struct TrafficPolicySummariesDeserializer;
impl TrafficPolicySummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TrafficPolicySummary>, XmlParseError> {
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
                    if name == "TrafficPolicySummary" {
                        obj.push(try!(TrafficPolicySummaryDeserializer::deserialize(
                            "TrafficPolicySummary",
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
/// <p>A complex type that contains information about the latest version of one traffic policy that is associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TrafficPolicySummary {
    /// <p>The ID that Amazon Route 53 assigned to the traffic policy when you created it.</p>
    pub id: String,
    /// <p>The version number of the latest version of the traffic policy.</p>
    pub latest_version: i64,
    /// <p>The name that you specified for the traffic policy when you created it.</p>
    pub name: String,
    /// <p>The number of traffic policies that are associated with the current AWS account.</p>
    pub traffic_policy_count: i64,
    /// <p>The DNS type of the resource record sets that Amazon Route 53 creates when you use a traffic policy to create a traffic policy instance.</p>
    pub type_: String,
}

struct TrafficPolicySummaryDeserializer;
impl TrafficPolicySummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TrafficPolicySummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TrafficPolicySummary::default();

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
                    "Id" => {
                        obj.id = try!(TrafficPolicyIdDeserializer::deserialize("Id", stack));
                    }
                    "LatestVersion" => {
                        obj.latest_version = try!(TrafficPolicyVersionDeserializer::deserialize(
                            "LatestVersion",
                            stack
                        ));
                    }
                    "Name" => {
                        obj.name = try!(TrafficPolicyNameDeserializer::deserialize("Name", stack));
                    }
                    "TrafficPolicyCount" => {
                        obj.traffic_policy_count =
                            try!(TrafficPolicyVersionDeserializer::deserialize(
                                "TrafficPolicyCount",
                                stack
                            ));
                    }
                    "Type" => {
                        obj.type_ = try!(RRTypeDeserializer::deserialize("Type", stack));
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
struct TrafficPolicyVersionDeserializer;
impl TrafficPolicyVersionDeserializer {
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

pub struct TrafficPolicyVersionSerializer;
impl TrafficPolicyVersionSerializer {
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

struct TrafficPolicyVersionMarkerDeserializer;
impl TrafficPolicyVersionMarkerDeserializer {
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

pub struct TrafficPolicyVersionMarkerSerializer;
impl TrafficPolicyVersionMarkerSerializer {
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

struct TransportProtocolDeserializer;
impl TransportProtocolDeserializer {
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
/// <p>A complex type that contains information about a request to update a health check.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateHealthCheckRequest {
    pub alarm_identifier: Option<AlarmIdentifier>,
    /// <p>A complex type that contains one <code>ChildHealthCheck</code> element for each health check that you want to associate with a <code>CALCULATED</code> health check.</p>
    pub child_health_checks: Option<Vec<String>>,
    /// <p>Specify whether you want Amazon Route 53 to send the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>client_hello</code> message during <code>TLS</code> negotiation. This allows the endpoint to respond to <code>HTTPS</code> health check requests with the applicable SSL/TLS certificate.</p> <p>Some endpoints require that HTTPS requests include the host name in the <code>client_hello</code> message. If you don't enable SNI, the status of the health check will be SSL alert <code>handshake_failure</code>. A health check can also have that status for other reasons. If SNI is enabled and you're still getting the error, check the SSL/TLS configuration on your endpoint and confirm that your certificate is valid.</p> <p>The SSL/TLS certificate on your endpoint includes a domain name in the <code>Common Name</code> field and possibly several more in the <code>Subject Alternative Names</code> field. One of the domain names in the certificate should match the value that you specify for <code>FullyQualifiedDomainName</code>. If the endpoint responds to the <code>client_hello</code> message with a certificate that does not include the domain name that you specified in <code>FullyQualifiedDomainName</code>, a health checker will retry the handshake. In the second attempt, the health checker will omit <code>FullyQualifiedDomainName</code> from the <code>client_hello</code> message.</p>
    pub enable_sni: Option<bool>,
    /// <p>The number of consecutive health checks that an endpoint must pass or fail for Amazon Route 53 to change the current status of the endpoint from unhealthy to healthy or vice versa. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html">How Amazon Route 53 Determines Whether an Endpoint Is Healthy</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If you don't specify a value for <code>FailureThreshold</code>, the default value is three health checks.</p>
    pub failure_threshold: Option<i64>,
    /// <p>Amazon Route 53 behavior depends on whether you specify a value for <code>IPAddress</code>.</p> <note> <p>If a health check already has a value for <code>IPAddress</code>, you can change the value. However, you can't update an existing health check to add or remove the value of <code>IPAddress</code>. </p> </note> <p> <b>If you specify a value for</b> <code>IPAddress</code>:</p> <p>Amazon Route 53 sends health check requests to the specified IPv4 or IPv6 address and passes the value of <code>FullyQualifiedDomainName</code> in the <code>Host</code> header for all health checks except TCP health checks. This is typically the fully qualified DNS name of the endpoint on which you want Amazon Route 53 to perform health checks.</p> <p>When Amazon Route 53 checks the health of an endpoint, here is how it constructs the <code>Host</code> header:</p> <ul> <li> <p>If you specify a value of <code>80</code> for <code>Port</code> and <code>HTTP</code> or <code>HTTP_STR_MATCH</code> for <code>Type</code>, Amazon Route 53 passes the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>Host</code> header.</p> </li> <li> <p>If you specify a value of <code>443</code> for <code>Port</code> and <code>HTTPS</code> or <code>HTTPS_STR_MATCH</code> for <code>Type</code>, Amazon Route 53 passes the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>Host</code> header.</p> </li> <li> <p>If you specify another value for <code>Port</code> and any value except <code>TCP</code> for <code>Type</code>, Amazon Route 53 passes <i> <code>FullyQualifiedDomainName</code>:<code>Port</code> </i> to the endpoint in the <code>Host</code> header.</p> </li> </ul> <p>If you don't specify a value for <code>FullyQualifiedDomainName</code>, Amazon Route 53 substitutes the value of <code>IPAddress</code> in the <code>Host</code> header in each of the above cases.</p> <p> <b>If you don't specify a value for</b> <code>IPAddress</code>:</p> <p>If you don't specify a value for <code>IPAddress</code>, Amazon Route 53 sends a DNS request to the domain that you specify in <code>FullyQualifiedDomainName</code> at the interval you specify in <code>RequestInterval</code>. Using an IPv4 address that is returned by DNS, Amazon Route 53 then checks the health of the endpoint.</p> <note> <p>If you don't specify a value for <code>IPAddress</code>, Amazon Route 53 uses only IPv4 to send health checks to the endpoint. If there's no resource record set with a type of A for the name that you specify for <code>FullyQualifiedDomainName</code>, the health check fails with a "DNS resolution failed" error.</p> </note> <p>If you want to check the health of weighted, latency, or failover resource record sets and you choose to specify the endpoint only by <code>FullyQualifiedDomainName</code>, we recommend that you create a separate health check for each endpoint. For example, create a health check for each HTTP server that is serving content for www.example.com. For the value of <code>FullyQualifiedDomainName</code>, specify the domain name of the server (such as <code>us-east-2-www.example.com</code>), not the name of the resource record sets (www.example.com).</p> <important> <p>In this configuration, if the value of <code>FullyQualifiedDomainName</code> matches the name of the resource record sets and you then associate the health check with those resource record sets, health check results will be unpredictable.</p> </important> <p>In addition, if the value of <code>Type</code> is <code>HTTP</code>, <code>HTTPS</code>, <code>HTTP_STR_MATCH</code>, or <code>HTTPS_STR_MATCH</code>, Amazon Route 53 passes the value of <code>FullyQualifiedDomainName</code> in the <code>Host</code> header, as it does when you specify a value for <code>IPAddress</code>. If the value of <code>Type</code> is <code>TCP</code>, Amazon Route 53 doesn't pass a <code>Host</code> header.</p>
    pub fully_qualified_domain_name: Option<String>,
    /// <p>The ID for the health check for which you want detailed information. When you created the health check, <code>CreateHealthCheck</code> returned the ID in the response, in the <code>HealthCheckId</code> element.</p>
    pub health_check_id: String,
    /// <p><p>A sequential counter that Amazon Route 53 sets to <code>1</code> when you create a health check and increments by 1 each time you update settings for the health check.</p> <p>We recommend that you use <code>GetHealthCheck</code> or <code>ListHealthChecks</code> to get the current value of <code>HealthCheckVersion</code> for the health check that you want to update, and that you include that value in your <code>UpdateHealthCheck</code> request. This prevents Amazon Route 53 from overwriting an intervening update:</p> <ul> <li> <p>If the value in the <code>UpdateHealthCheck</code> request matches the value of <code>HealthCheckVersion</code> in the health check, Amazon Route 53 updates the health check with the new settings.</p> </li> <li> <p>If the value of <code>HealthCheckVersion</code> in the health check is greater, the health check was changed after you got the version number. Amazon Route 53 does not update the health check, and it returns a <code>HealthCheckVersionMismatch</code> error.</p> </li> </ul></p>
    pub health_check_version: Option<i64>,
    /// <p><p>The number of child health checks that are associated with a <code>CALCULATED</code> health that Amazon Route 53 must consider healthy for the <code>CALCULATED</code> health check to be considered healthy. To specify the child health checks that you want to associate with a <code>CALCULATED</code> health check, use the <code>ChildHealthChecks</code> and <code>ChildHealthCheck</code> elements.</p> <p>Note the following:</p> <ul> <li> <p>If you specify a number greater than the number of child health checks, Amazon Route 53 always considers this health check to be unhealthy.</p> </li> <li> <p>If you specify <code>0</code>, Amazon Route 53 always considers this health check to be healthy.</p> </li> </ul></p>
    pub health_threshold: Option<i64>,
    /// <p><p>The IPv4 or IPv6 IP address for the endpoint that you want Amazon Route 53 to perform health checks on. If you don&#39;t specify a value for <code>IPAddress</code>, Amazon Route 53 sends a DNS request to resolve the domain name that you specify in <code>FullyQualifiedDomainName</code> at the interval that you specify in <code>RequestInterval</code>. Using an IP address that is returned by DNS, Amazon Route 53 then checks the health of the endpoint.</p> <p>Use one of the following formats for the value of <code>IPAddress</code>: </p> <ul> <li> <p> <b>IPv4 address</b>: four values between 0 and 255, separated by periods (.), for example, <code>192.0.2.44</code>.</p> </li> <li> <p> <b>IPv6 address</b>: eight groups of four hexadecimal values, separated by colons (:), for example, <code>2001:0db8:85a3:0000:0000:abcd:0001:2345</code>. You can also shorten IPv6 addresses as described in RFC 5952, for example, <code>2001:db8:85a3::abcd:1:2345</code>.</p> </li> </ul> <p>If the endpoint is an EC2 instance, we recommend that you create an Elastic IP address, associate it with your EC2 instance, and specify the Elastic IP address for <code>IPAddress</code>. This ensures that the IP address of your instance never changes. For more information, see the applicable documentation:</p> <ul> <li> <p>Linux: <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html">Elastic IP Addresses (EIP)</a> in the <i>Amazon EC2 User Guide for Linux Instances</i> </p> </li> <li> <p>Windows: <a href="http://docs.aws.amazon.com/AWSEC2/latest/WindowsGuide/elastic-ip-addresses-eip.html">Elastic IP Addresses (EIP)</a> in the <i>Amazon EC2 User Guide for Windows Instances</i> </p> </li> </ul> <note> <p>If a health check already has a value for <code>IPAddress</code>, you can change the value. However, you can&#39;t update an existing health check to add or remove the value of <code>IPAddress</code>. </p> </note> <p>For more information, see <a>UpdateHealthCheckRequest$FullyQualifiedDomainName</a>.</p> <p>Constraints: Amazon Route 53 can&#39;t check the health of endpoints for which the IP address is in local, private, non-routable, or multicast ranges. For more information about IP addresses for which you can&#39;t create health checks, see the following documents:</p> <ul> <li> <p> <a href="https://tools.ietf.org/html/rfc5735">RFC 5735, Special Use IPv4 Addresses</a> </p> </li> <li> <p> <a href="https://tools.ietf.org/html/rfc6598">RFC 6598, IANA-Reserved IPv4 Prefix for Shared Address Space</a> </p> </li> <li> <p> <a href="https://tools.ietf.org/html/rfc5156">RFC 5156, Special-Use IPv6 Addresses</a> </p> </li> </ul></p>
    pub ip_address: Option<String>,
    /// <p><p>When CloudWatch has insufficient data about the metric to determine the alarm state, the status that you want Amazon Route 53 to assign to the health check:</p> <ul> <li> <p> <code>Healthy</code>: Amazon Route 53 considers the health check to be healthy.</p> </li> <li> <p> <code>Unhealthy</code>: Amazon Route 53 considers the health check to be unhealthy.</p> </li> <li> <p> <code>LastKnownStatus</code>: Amazon Route 53 uses the status of the health check from the last time CloudWatch had sufficient data to determine the alarm state. For new health checks that have no last known status, the default status for the health check is healthy.</p> </li> </ul></p>
    pub insufficient_data_health_status: Option<String>,
    /// <p>Specify whether you want Amazon Route 53 to invert the status of a health check, for example, to consider a health check unhealthy when it otherwise would be considered healthy.</p>
    pub inverted: Option<bool>,
    /// <p>The port on the endpoint on which you want Amazon Route 53 to perform health checks.</p>
    pub port: Option<i64>,
    /// <p>A complex type that contains one <code>Region</code> element for each region that you want Amazon Route 53 health checkers to check the specified endpoint from.</p>
    pub regions: Option<Vec<String>>,
    /// <p><p>A complex type that contains one <code>ResettableElementName</code> element for each element that you want to reset to the default value. Valid values for <code>ResettableElementName</code> include the following:</p> <ul> <li> <p> <code>ChildHealthChecks</code>: Amazon Route 53 resets <a>HealthCheckConfig$ChildHealthChecks</a> to null.</p> </li> <li> <p> <code>FullyQualifiedDomainName</code>: Amazon Route 53 resets <a>HealthCheckConfig$FullyQualifiedDomainName</a> to null.</p> </li> <li> <p> <code>Regions</code>: Amazon Route 53 resets the <a>HealthCheckConfig$Regions</a> list to the default set of regions. </p> </li> <li> <p> <code>ResourcePath</code>: Amazon Route 53 resets <a>HealthCheckConfig$ResourcePath</a> to null.</p> </li> </ul></p>
    pub reset_elements: Option<Vec<String>>,
    /// <p>The path that you want Amazon Route 53 to request when performing health checks. The path can be any value for which your endpoint will return an HTTP status code of 2xx or 3xx when the endpoint is healthy, for example the file /docs/route53-health-check.html. </p> <p>Specify this value only if you want to change it.</p>
    pub resource_path: Option<String>,
    /// <p>If the value of <code>Type</code> is <code>HTTP_STR_MATCH</code> or <code>HTTP_STR_MATCH</code>, the string that you want Amazon Route 53 to search for in the response body from the specified resource. If the string appears in the response body, Amazon Route 53 considers the resource healthy. (You can't change the value of <code>Type</code> when you update a health check.)</p>
    pub search_string: Option<String>,
}

pub struct UpdateHealthCheckRequestSerializer;
impl UpdateHealthCheckRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &UpdateHealthCheckRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        if let Some(ref value) = obj.alarm_identifier {
            &AlarmIdentifierSerializer::serialize(&mut writer, "AlarmIdentifier", value)?;
        }
        if let Some(ref value) = obj.child_health_checks {
            &ChildHealthCheckListSerializer::serialize(&mut writer, "ChildHealthChecks", value)?;
        }
        if let Some(ref value) = obj.enable_sni {
            &EnableSNISerializer::serialize(&mut writer, "EnableSNI", value)?;
        }
        if let Some(ref value) = obj.failure_threshold {
            &FailureThresholdSerializer::serialize(&mut writer, "FailureThreshold", value)?;
        }
        if let Some(ref value) = obj.fully_qualified_domain_name {
            &FullyQualifiedDomainNameSerializer::serialize(
                &mut writer,
                "FullyQualifiedDomainName",
                value,
            )?;
        }
        if let Some(ref value) = obj.health_check_version {
            &HealthCheckVersionSerializer::serialize(&mut writer, "HealthCheckVersion", value)?;
        }
        if let Some(ref value) = obj.health_threshold {
            &HealthThresholdSerializer::serialize(&mut writer, "HealthThreshold", value)?;
        }
        if let Some(ref value) = obj.ip_address {
            &IPAddressSerializer::serialize(&mut writer, "IPAddress", value)?;
        }
        if let Some(ref value) = obj.insufficient_data_health_status {
            &InsufficientDataHealthStatusSerializer::serialize(
                &mut writer,
                "InsufficientDataHealthStatus",
                value,
            )?;
        }
        if let Some(ref value) = obj.inverted {
            &InvertedSerializer::serialize(&mut writer, "Inverted", value)?;
        }
        if let Some(ref value) = obj.port {
            &PortSerializer::serialize(&mut writer, "Port", value)?;
        }
        if let Some(ref value) = obj.regions {
            &HealthCheckRegionListSerializer::serialize(&mut writer, "Regions", value)?;
        }
        if let Some(ref value) = obj.reset_elements {
            &ResettableElementNameListSerializer::serialize(&mut writer, "ResetElements", value)?;
        }
        if let Some(ref value) = obj.resource_path {
            &ResourcePathSerializer::serialize(&mut writer, "ResourcePath", value)?;
        }
        if let Some(ref value) = obj.search_string {
            &SearchStringSerializer::serialize(&mut writer, "SearchString", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateHealthCheckResponse {
    pub health_check: HealthCheck,
}

struct UpdateHealthCheckResponseDeserializer;
impl UpdateHealthCheckResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateHealthCheckResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateHealthCheckResponse::default();

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
                    "HealthCheck" => {
                        obj.health_check =
                            try!(HealthCheckDeserializer::deserialize("HealthCheck", stack));
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
/// <p>A request to update the comment for a hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateHostedZoneCommentRequest {
    /// <p>The new comment for the hosted zone. If you don't specify a value for <code>Comment</code>, Amazon Route 53 deletes the existing value of the <code>Comment</code> element, if any.</p>
    pub comment: Option<String>,
    /// <p>The ID for the hosted zone that you want to update the comment for.</p>
    pub id: String,
}

pub struct UpdateHostedZoneCommentRequestSerializer;
impl UpdateHostedZoneCommentRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &UpdateHostedZoneCommentRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        if let Some(ref value) = obj.comment {
            &ResourceDescriptionSerializer::serialize(&mut writer, "Comment", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>A complex type that contains the response to the <code>UpdateHostedZoneComment</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateHostedZoneCommentResponse {
    pub hosted_zone: HostedZone,
}

struct UpdateHostedZoneCommentResponseDeserializer;
impl UpdateHostedZoneCommentResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateHostedZoneCommentResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateHostedZoneCommentResponse::default();

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
                    "HostedZone" => {
                        obj.hosted_zone =
                            try!(HostedZoneDeserializer::deserialize("HostedZone", stack));
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
/// <p>A complex type that contains information about the traffic policy that you want to update the comment for.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateTrafficPolicyCommentRequest {
    /// <p>The new comment for the specified traffic policy and version.</p>
    pub comment: String,
    /// <p>The value of <code>Id</code> for the traffic policy that you want to update the comment for.</p>
    pub id: String,
    /// <p>The value of <code>Version</code> for the traffic policy that you want to update the comment for.</p>
    pub version: i64,
}

pub struct UpdateTrafficPolicyCommentRequestSerializer;
impl UpdateTrafficPolicyCommentRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &UpdateTrafficPolicyCommentRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        TrafficPolicyCommentSerializer::serialize(&mut writer, "Comment", &obj.comment)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>A complex type that contains the response information for the traffic policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateTrafficPolicyCommentResponse {
    /// <p>A complex type that contains settings for the specified traffic policy.</p>
    pub traffic_policy: TrafficPolicy,
}

struct UpdateTrafficPolicyCommentResponseDeserializer;
impl UpdateTrafficPolicyCommentResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateTrafficPolicyCommentResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateTrafficPolicyCommentResponse::default();

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
                    "TrafficPolicy" => {
                        obj.traffic_policy = try!(TrafficPolicyDeserializer::deserialize(
                            "TrafficPolicy",
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
/// <p>A complex type that contains information about the resource record sets that you want to update based on a specified traffic policy instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateTrafficPolicyInstanceRequest {
    /// <p>The ID of the traffic policy instance that you want to update.</p>
    pub id: String,
    /// <p>The TTL that you want Amazon Route 53 to assign to all of the updated resource record sets.</p>
    pub ttl: i64,
    /// <p>The ID of the traffic policy that you want Amazon Route 53 to use to update resource record sets for the specified traffic policy instance.</p>
    pub traffic_policy_id: String,
    /// <p>The version of the traffic policy that you want Amazon Route 53 to use to update resource record sets for the specified traffic policy instance.</p>
    pub traffic_policy_version: i64,
}

pub struct UpdateTrafficPolicyInstanceRequestSerializer;
impl UpdateTrafficPolicyInstanceRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &UpdateTrafficPolicyInstanceRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        TTLSerializer::serialize(&mut writer, "TTL", &obj.ttl)?;
        TrafficPolicyIdSerializer::serialize(
            &mut writer,
            "TrafficPolicyId",
            &obj.traffic_policy_id,
        )?;
        TrafficPolicyVersionSerializer::serialize(
            &mut writer,
            "TrafficPolicyVersion",
            &obj.traffic_policy_version,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
/// <p>A complex type that contains information about the resource record sets that Amazon Route 53 created based on a specified traffic policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateTrafficPolicyInstanceResponse {
    /// <p>A complex type that contains settings for the updated traffic policy instance.</p>
    pub traffic_policy_instance: TrafficPolicyInstance,
}

struct UpdateTrafficPolicyInstanceResponseDeserializer;
impl UpdateTrafficPolicyInstanceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateTrafficPolicyInstanceResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateTrafficPolicyInstanceResponse::default();

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
                    "TrafficPolicyInstance" => {
                        obj.traffic_policy_instance =
                            try!(TrafficPolicyInstanceDeserializer::deserialize(
                                "TrafficPolicyInstance",
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
struct UsageCountDeserializer;
impl UsageCountDeserializer {
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
/// <p>(Private hosted zones only) A complex type that contains information about an Amazon VPC.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VPC {
    pub vpc_id: Option<String>,
    /// <p>(Private hosted zones only) The region in which you created an Amazon VPC.</p>
    pub vpc_region: Option<String>,
}

struct VPCDeserializer;
impl VPCDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<VPC, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = VPC::default();

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
                    "VPCId" => {
                        obj.vpc_id = Some(try!(VPCIdDeserializer::deserialize("VPCId", stack)));
                    }
                    "VPCRegion" => {
                        obj.vpc_region =
                            Some(try!(VPCRegionDeserializer::deserialize("VPCRegion", stack)));
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

pub struct VPCSerializer;
impl VPCSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &VPC,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.vpc_id {
            writer.write(xml::writer::XmlEvent::start_element("VPCId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.vpc_region {
            writer.write(xml::writer::XmlEvent::start_element("VPCRegion"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct VPCIdDeserializer;
impl VPCIdDeserializer {
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

pub struct VPCIdSerializer;
impl VPCIdSerializer {
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

struct VPCRegionDeserializer;
impl VPCRegionDeserializer {
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

pub struct VPCRegionSerializer;
impl VPCRegionSerializer {
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

struct VPCsDeserializer;
impl VPCsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<VPC>, XmlParseError> {
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
                    if name == "VPC" {
                        obj.push(try!(VPCDeserializer::deserialize("VPC", stack)));
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
/// Errors returned by AssociateVPCWithHostedZone
#[derive(Debug, PartialEq)]
pub enum AssociateVPCWithHostedZoneError {
    /// <p><p>The cause of this error depends on whether you&#39;re trying to create a public or a private hosted zone:</p> <ul> <li> <p> <b>Public hosted zone:</b> Two hosted zones that have the same name or that have a parent/child relationship (example.com and test.example.com) can&#39;t have any common name servers. You tried to create a hosted zone that has the same name as an existing hosted zone or that&#39;s the parent or child of an existing hosted zone, and you specified a delegation set that shares one or more name servers with the existing hosted zone. For more information, see <a>CreateReusableDelegationSet</a>.</p> </li> <li> <p> <b>Private hosted zone:</b> You specified an Amazon VPC that you&#39;re already using for another hosted zone, and the domain that you specified for one of the hosted zones is a subdomain of the domain that you specified for the other hosted zone. For example, you can&#39;t use the same Amazon VPC for the hosted zones for example.com and test.example.com.</p> </li> </ul></p>
    ConflictingDomainExists(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC.</p>
    InvalidVPCId(String),
    /// <p>This operation can't be completed either because the current account has reached the limit on reusable delegation sets that it can create or because you've reached the limit on the number of Amazon VPCs that you can associate with a private hosted zone. To get the current limit on the number of reusable delegation sets, see <a>GetAccountLimit</a>. To get the current limit on the number of Amazon VPCs that you can associate with a private hosted zone, see <a>GetHostedZoneLimit</a>. To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p>
    LimitsExceeded(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>Associating the specified VPC with the specified hosted zone has not been authorized.</p>
    NotAuthorized(String),
    /// <p>You're trying to associate a VPC with a public hosted zone. Amazon Route 53 doesn't support associating a VPC with a public hosted zone.</p>
    PublicZoneVPCAssociation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AssociateVPCWithHostedZoneError {
    pub fn from_body(body: &str) -> AssociateVPCWithHostedZoneError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConflictingDomainExists" => {
                    AssociateVPCWithHostedZoneError::ConflictingDomainExists(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidInput" => AssociateVPCWithHostedZoneError::InvalidInput(String::from(
                    parsed_error.message,
                )),
                "InvalidVPCId" => AssociateVPCWithHostedZoneError::InvalidVPCId(String::from(
                    parsed_error.message,
                )),
                "LimitsExceeded" => AssociateVPCWithHostedZoneError::LimitsExceeded(String::from(
                    parsed_error.message,
                )),
                "NoSuchHostedZone" => AssociateVPCWithHostedZoneError::NoSuchHostedZone(
                    String::from(parsed_error.message),
                ),
                "NotAuthorizedException" => AssociateVPCWithHostedZoneError::NotAuthorized(
                    String::from(parsed_error.message),
                ),
                "PublicZoneVPCAssociation" => {
                    AssociateVPCWithHostedZoneError::PublicZoneVPCAssociation(String::from(
                        parsed_error.message,
                    ))
                }
                _ => AssociateVPCWithHostedZoneError::Unknown(String::from(body)),
            },
            Err(_) => AssociateVPCWithHostedZoneError::Unknown(body.to_string()),
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

impl From<XmlParseError> for AssociateVPCWithHostedZoneError {
    fn from(err: XmlParseError) -> AssociateVPCWithHostedZoneError {
        let XmlParseError(message) = err;
        AssociateVPCWithHostedZoneError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AssociateVPCWithHostedZoneError {
    fn from(err: CredentialsError) -> AssociateVPCWithHostedZoneError {
        AssociateVPCWithHostedZoneError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateVPCWithHostedZoneError {
    fn from(err: HttpDispatchError) -> AssociateVPCWithHostedZoneError {
        AssociateVPCWithHostedZoneError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateVPCWithHostedZoneError {
    fn from(err: io::Error) -> AssociateVPCWithHostedZoneError {
        AssociateVPCWithHostedZoneError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateVPCWithHostedZoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateVPCWithHostedZoneError {
    fn description(&self) -> &str {
        match *self {
            AssociateVPCWithHostedZoneError::ConflictingDomainExists(ref cause) => cause,
            AssociateVPCWithHostedZoneError::InvalidInput(ref cause) => cause,
            AssociateVPCWithHostedZoneError::InvalidVPCId(ref cause) => cause,
            AssociateVPCWithHostedZoneError::LimitsExceeded(ref cause) => cause,
            AssociateVPCWithHostedZoneError::NoSuchHostedZone(ref cause) => cause,
            AssociateVPCWithHostedZoneError::NotAuthorized(ref cause) => cause,
            AssociateVPCWithHostedZoneError::PublicZoneVPCAssociation(ref cause) => cause,
            AssociateVPCWithHostedZoneError::Validation(ref cause) => cause,
            AssociateVPCWithHostedZoneError::Credentials(ref err) => err.description(),
            AssociateVPCWithHostedZoneError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateVPCWithHostedZoneError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ChangeResourceRecordSets
#[derive(Debug, PartialEq)]
pub enum ChangeResourceRecordSetsError {
    /// <p>This exception contains a list of messages that might contain one or more error messages. Each error message indicates one error in the change batch.</p>
    InvalidChangeBatch(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
    NoSuchHealthCheck(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ChangeResourceRecordSetsError {
    pub fn from_body(body: &str) -> ChangeResourceRecordSetsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidChangeBatch" => ChangeResourceRecordSetsError::InvalidChangeBatch(
                    String::from(parsed_error.message),
                ),
                "InvalidInput" => {
                    ChangeResourceRecordSetsError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHealthCheck" => ChangeResourceRecordSetsError::NoSuchHealthCheck(
                    String::from(parsed_error.message),
                ),
                "NoSuchHostedZone" => ChangeResourceRecordSetsError::NoSuchHostedZone(
                    String::from(parsed_error.message),
                ),
                "PriorRequestNotComplete" => {
                    ChangeResourceRecordSetsError::PriorRequestNotComplete(String::from(
                        parsed_error.message,
                    ))
                }
                _ => ChangeResourceRecordSetsError::Unknown(String::from(body)),
            },
            Err(_) => ChangeResourceRecordSetsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ChangeResourceRecordSetsError {
    fn from(err: XmlParseError) -> ChangeResourceRecordSetsError {
        let XmlParseError(message) = err;
        ChangeResourceRecordSetsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ChangeResourceRecordSetsError {
    fn from(err: CredentialsError) -> ChangeResourceRecordSetsError {
        ChangeResourceRecordSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ChangeResourceRecordSetsError {
    fn from(err: HttpDispatchError) -> ChangeResourceRecordSetsError {
        ChangeResourceRecordSetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ChangeResourceRecordSetsError {
    fn from(err: io::Error) -> ChangeResourceRecordSetsError {
        ChangeResourceRecordSetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ChangeResourceRecordSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ChangeResourceRecordSetsError {
    fn description(&self) -> &str {
        match *self {
            ChangeResourceRecordSetsError::InvalidChangeBatch(ref cause) => cause,
            ChangeResourceRecordSetsError::InvalidInput(ref cause) => cause,
            ChangeResourceRecordSetsError::NoSuchHealthCheck(ref cause) => cause,
            ChangeResourceRecordSetsError::NoSuchHostedZone(ref cause) => cause,
            ChangeResourceRecordSetsError::PriorRequestNotComplete(ref cause) => cause,
            ChangeResourceRecordSetsError::Validation(ref cause) => cause,
            ChangeResourceRecordSetsError::Credentials(ref err) => err.description(),
            ChangeResourceRecordSetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ChangeResourceRecordSetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ChangeTagsForResource
#[derive(Debug, PartialEq)]
pub enum ChangeTagsForResourceError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
    NoSuchHealthCheck(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ChangeTagsForResourceError {
    pub fn from_body(body: &str) -> ChangeTagsForResourceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    ChangeTagsForResourceError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHealthCheck" => ChangeTagsForResourceError::NoSuchHealthCheck(String::from(
                    parsed_error.message,
                )),
                "NoSuchHostedZone" => {
                    ChangeTagsForResourceError::NoSuchHostedZone(String::from(parsed_error.message))
                }
                "PriorRequestNotComplete" => ChangeTagsForResourceError::PriorRequestNotComplete(
                    String::from(parsed_error.message),
                ),
                "ThrottlingException" => {
                    ChangeTagsForResourceError::Throttling(String::from(parsed_error.message))
                }
                _ => ChangeTagsForResourceError::Unknown(String::from(body)),
            },
            Err(_) => ChangeTagsForResourceError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ChangeTagsForResourceError {
    fn from(err: XmlParseError) -> ChangeTagsForResourceError {
        let XmlParseError(message) = err;
        ChangeTagsForResourceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ChangeTagsForResourceError {
    fn from(err: CredentialsError) -> ChangeTagsForResourceError {
        ChangeTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ChangeTagsForResourceError {
    fn from(err: HttpDispatchError) -> ChangeTagsForResourceError {
        ChangeTagsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ChangeTagsForResourceError {
    fn from(err: io::Error) -> ChangeTagsForResourceError {
        ChangeTagsForResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ChangeTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ChangeTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ChangeTagsForResourceError::InvalidInput(ref cause) => cause,
            ChangeTagsForResourceError::NoSuchHealthCheck(ref cause) => cause,
            ChangeTagsForResourceError::NoSuchHostedZone(ref cause) => cause,
            ChangeTagsForResourceError::PriorRequestNotComplete(ref cause) => cause,
            ChangeTagsForResourceError::Throttling(ref cause) => cause,
            ChangeTagsForResourceError::Validation(ref cause) => cause,
            ChangeTagsForResourceError::Credentials(ref err) => err.description(),
            ChangeTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ChangeTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateHealthCheck
#[derive(Debug, PartialEq)]
pub enum CreateHealthCheckError {
    /// <p><p> The health check you&#39;re attempting to create already exists. Amazon Route 53 returns this error when you submit a request that has the following values:</p> <ul> <li> <p>The same value for <code>CallerReference</code> as an existing health check, and one or more values that differ from the existing health check that has the same caller reference.</p> </li> <li> <p>The same value for <code>CallerReference</code> as a health check that you created and later deleted, regardless of the other settings in the request.</p> </li> </ul></p>
    HealthCheckAlreadyExists(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>This health check can't be created because the current account has reached the limit on the number of active health checks.</p> <p>For information about default limits, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>For information about how to get the current limit for an account, see <a>GetAccountLimit</a>. To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p> <p>You have reached the maximum number of active health checks for an AWS account. To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p>
    TooManyHealthChecks(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateHealthCheckError {
    pub fn from_body(body: &str) -> CreateHealthCheckError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "HealthCheckAlreadyExists" => CreateHealthCheckError::HealthCheckAlreadyExists(
                    String::from(parsed_error.message),
                ),
                "InvalidInput" => {
                    CreateHealthCheckError::InvalidInput(String::from(parsed_error.message))
                }
                "TooManyHealthChecks" => {
                    CreateHealthCheckError::TooManyHealthChecks(String::from(parsed_error.message))
                }
                _ => CreateHealthCheckError::Unknown(String::from(body)),
            },
            Err(_) => CreateHealthCheckError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateHealthCheckError {
    fn from(err: XmlParseError) -> CreateHealthCheckError {
        let XmlParseError(message) = err;
        CreateHealthCheckError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateHealthCheckError {
    fn from(err: CredentialsError) -> CreateHealthCheckError {
        CreateHealthCheckError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateHealthCheckError {
    fn from(err: HttpDispatchError) -> CreateHealthCheckError {
        CreateHealthCheckError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateHealthCheckError {
    fn from(err: io::Error) -> CreateHealthCheckError {
        CreateHealthCheckError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateHealthCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateHealthCheckError {
    fn description(&self) -> &str {
        match *self {
            CreateHealthCheckError::HealthCheckAlreadyExists(ref cause) => cause,
            CreateHealthCheckError::InvalidInput(ref cause) => cause,
            CreateHealthCheckError::TooManyHealthChecks(ref cause) => cause,
            CreateHealthCheckError::Validation(ref cause) => cause,
            CreateHealthCheckError::Credentials(ref err) => err.description(),
            CreateHealthCheckError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateHealthCheckError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateHostedZone
#[derive(Debug, PartialEq)]
pub enum CreateHostedZoneError {
    /// <p><p>The cause of this error depends on whether you&#39;re trying to create a public or a private hosted zone:</p> <ul> <li> <p> <b>Public hosted zone:</b> Two hosted zones that have the same name or that have a parent/child relationship (example.com and test.example.com) can&#39;t have any common name servers. You tried to create a hosted zone that has the same name as an existing hosted zone or that&#39;s the parent or child of an existing hosted zone, and you specified a delegation set that shares one or more name servers with the existing hosted zone. For more information, see <a>CreateReusableDelegationSet</a>.</p> </li> <li> <p> <b>Private hosted zone:</b> You specified an Amazon VPC that you&#39;re already using for another hosted zone, and the domain that you specified for one of the hosted zones is a subdomain of the domain that you specified for the other hosted zone. For example, you can&#39;t use the same Amazon VPC for the hosted zones for example.com and test.example.com.</p> </li> </ul></p>
    ConflictingDomainExists(String),
    /// <p>You can create a hosted zone that has the same name as an existing hosted zone (example.com is common), but there is a limit to the number of hosted zones that have the same name. If you get this error, Amazon Route 53 has reached that limit. If you own the domain name and Amazon Route 53 generates this error, contact Customer Support.</p>
    DelegationSetNotAvailable(String),
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    DelegationSetNotReusable(String),
    /// <p>The hosted zone you're trying to create already exists. Amazon Route 53 returns this error when a hosted zone has already been created with the specified <code>CallerReference</code>.</p>
    HostedZoneAlreadyExists(String),
    /// <p>The specified domain name is not valid.</p>
    InvalidDomainName(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC.</p>
    InvalidVPCId(String),
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    NoSuchDelegationSet(String),
    /// <p>This operation can't be completed either because the current account has reached the limit on the number of hosted zones or because you've reached the limit on the number of hosted zones that can be associated with a reusable delegation set.</p> <p>For information about default limits, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>To get the current limit on hosted zones that can be created by an account, see <a>GetAccountLimit</a>.</p> <p>To get the current limit on hosted zones that can be associated with a reusable delegation set, see <a>GetReusableDelegationSetLimit</a>.</p> <p>To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p>
    TooManyHostedZones(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateHostedZoneError {
    pub fn from_body(body: &str) -> CreateHostedZoneError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConflictingDomainExists" => CreateHostedZoneError::ConflictingDomainExists(
                    String::from(parsed_error.message),
                ),
                "DelegationSetNotAvailable" => CreateHostedZoneError::DelegationSetNotAvailable(
                    String::from(parsed_error.message),
                ),
                "DelegationSetNotReusable" => CreateHostedZoneError::DelegationSetNotReusable(
                    String::from(parsed_error.message),
                ),
                "HostedZoneAlreadyExists" => CreateHostedZoneError::HostedZoneAlreadyExists(
                    String::from(parsed_error.message),
                ),
                "InvalidDomainName" => {
                    CreateHostedZoneError::InvalidDomainName(String::from(parsed_error.message))
                }
                "InvalidInput" => {
                    CreateHostedZoneError::InvalidInput(String::from(parsed_error.message))
                }
                "InvalidVPCId" => {
                    CreateHostedZoneError::InvalidVPCId(String::from(parsed_error.message))
                }
                "NoSuchDelegationSet" => {
                    CreateHostedZoneError::NoSuchDelegationSet(String::from(parsed_error.message))
                }
                "TooManyHostedZones" => {
                    CreateHostedZoneError::TooManyHostedZones(String::from(parsed_error.message))
                }
                _ => CreateHostedZoneError::Unknown(String::from(body)),
            },
            Err(_) => CreateHostedZoneError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateHostedZoneError {
    fn from(err: XmlParseError) -> CreateHostedZoneError {
        let XmlParseError(message) = err;
        CreateHostedZoneError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateHostedZoneError {
    fn from(err: CredentialsError) -> CreateHostedZoneError {
        CreateHostedZoneError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateHostedZoneError {
    fn from(err: HttpDispatchError) -> CreateHostedZoneError {
        CreateHostedZoneError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateHostedZoneError {
    fn from(err: io::Error) -> CreateHostedZoneError {
        CreateHostedZoneError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateHostedZoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateHostedZoneError {
    fn description(&self) -> &str {
        match *self {
            CreateHostedZoneError::ConflictingDomainExists(ref cause) => cause,
            CreateHostedZoneError::DelegationSetNotAvailable(ref cause) => cause,
            CreateHostedZoneError::DelegationSetNotReusable(ref cause) => cause,
            CreateHostedZoneError::HostedZoneAlreadyExists(ref cause) => cause,
            CreateHostedZoneError::InvalidDomainName(ref cause) => cause,
            CreateHostedZoneError::InvalidInput(ref cause) => cause,
            CreateHostedZoneError::InvalidVPCId(ref cause) => cause,
            CreateHostedZoneError::NoSuchDelegationSet(ref cause) => cause,
            CreateHostedZoneError::TooManyHostedZones(ref cause) => cause,
            CreateHostedZoneError::Validation(ref cause) => cause,
            CreateHostedZoneError::Credentials(ref err) => err.description(),
            CreateHostedZoneError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateHostedZoneError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateQueryLoggingConfig
#[derive(Debug, PartialEq)]
pub enum CreateQueryLoggingConfigError {
    /// <p>Another user submitted a request to create, update, or delete the object at the same time that you did. Retry the request. </p>
    ConcurrentModification(String),
    /// <p><p>Amazon Route 53 doesn&#39;t have the permissions required to create log streams and send query logs to log streams. Possible causes include the following:</p> <ul> <li> <p>There is no resource policy that specifies the log group ARN in the value for <code>Resource</code>.</p> </li> <li> <p>The resource policy that includes the log group ARN in the value for <code>Resource</code> doesn&#39;t have the necessary permissions.</p> </li> <li> <p>The resource policy hasn&#39;t finished propagating yet.</p> </li> </ul></p>
    InsufficientCloudWatchLogsResourcePolicy(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>There is no CloudWatch Logs log group with the specified ARN.</p>
    NoSuchCloudWatchLogsLogGroup(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>You can create only one query logging configuration for a hosted zone, and a query logging configuration already exists for this hosted zone.</p>
    QueryLoggingConfigAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateQueryLoggingConfigError {
    pub fn from_body(body: &str) -> CreateQueryLoggingConfigError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConcurrentModification" => CreateQueryLoggingConfigError::ConcurrentModification(
                    String::from(parsed_error.message),
                ),
                "InsufficientCloudWatchLogsResourcePolicy" => {
                    CreateQueryLoggingConfigError::InsufficientCloudWatchLogsResourcePolicy(
                        String::from(parsed_error.message),
                    )
                }
                "InvalidInput" => {
                    CreateQueryLoggingConfigError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchCloudWatchLogsLogGroup" => {
                    CreateQueryLoggingConfigError::NoSuchCloudWatchLogsLogGroup(String::from(
                        parsed_error.message,
                    ))
                }
                "NoSuchHostedZone" => CreateQueryLoggingConfigError::NoSuchHostedZone(
                    String::from(parsed_error.message),
                ),
                "QueryLoggingConfigAlreadyExists" => {
                    CreateQueryLoggingConfigError::QueryLoggingConfigAlreadyExists(String::from(
                        parsed_error.message,
                    ))
                }
                _ => CreateQueryLoggingConfigError::Unknown(String::from(body)),
            },
            Err(_) => CreateQueryLoggingConfigError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateQueryLoggingConfigError {
    fn from(err: XmlParseError) -> CreateQueryLoggingConfigError {
        let XmlParseError(message) = err;
        CreateQueryLoggingConfigError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateQueryLoggingConfigError {
    fn from(err: CredentialsError) -> CreateQueryLoggingConfigError {
        CreateQueryLoggingConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateQueryLoggingConfigError {
    fn from(err: HttpDispatchError) -> CreateQueryLoggingConfigError {
        CreateQueryLoggingConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateQueryLoggingConfigError {
    fn from(err: io::Error) -> CreateQueryLoggingConfigError {
        CreateQueryLoggingConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateQueryLoggingConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateQueryLoggingConfigError {
    fn description(&self) -> &str {
        match *self {
            CreateQueryLoggingConfigError::ConcurrentModification(ref cause) => cause,
            CreateQueryLoggingConfigError::InsufficientCloudWatchLogsResourcePolicy(ref cause) => {
                cause
            }
            CreateQueryLoggingConfigError::InvalidInput(ref cause) => cause,
            CreateQueryLoggingConfigError::NoSuchCloudWatchLogsLogGroup(ref cause) => cause,
            CreateQueryLoggingConfigError::NoSuchHostedZone(ref cause) => cause,
            CreateQueryLoggingConfigError::QueryLoggingConfigAlreadyExists(ref cause) => cause,
            CreateQueryLoggingConfigError::Validation(ref cause) => cause,
            CreateQueryLoggingConfigError::Credentials(ref err) => err.description(),
            CreateQueryLoggingConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateQueryLoggingConfigError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateReusableDelegationSet
#[derive(Debug, PartialEq)]
pub enum CreateReusableDelegationSetError {
    /// <p>A delegation set with the same owner and caller reference combination has already been created.</p>
    DelegationSetAlreadyCreated(String),
    /// <p>The specified delegation set has already been marked as reusable.</p>
    DelegationSetAlreadyReusable(String),
    /// <p>You can create a hosted zone that has the same name as an existing hosted zone (example.com is common), but there is a limit to the number of hosted zones that have the same name. If you get this error, Amazon Route 53 has reached that limit. If you own the domain name and Amazon Route 53 generates this error, contact Customer Support.</p>
    DelegationSetNotAvailable(String),
    /// <p>The specified HostedZone can't be found.</p>
    HostedZoneNotFound(String),
    /// <p>Parameter name is invalid.</p>
    InvalidArgument(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>This operation can't be completed either because the current account has reached the limit on reusable delegation sets that it can create or because you've reached the limit on the number of Amazon VPCs that you can associate with a private hosted zone. To get the current limit on the number of reusable delegation sets, see <a>GetAccountLimit</a>. To get the current limit on the number of Amazon VPCs that you can associate with a private hosted zone, see <a>GetHostedZoneLimit</a>. To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p>
    LimitsExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateReusableDelegationSetError {
    pub fn from_body(body: &str) -> CreateReusableDelegationSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "DelegationSetAlreadyCreated" => {
                    CreateReusableDelegationSetError::DelegationSetAlreadyCreated(String::from(
                        parsed_error.message,
                    ))
                }
                "DelegationSetAlreadyReusable" => {
                    CreateReusableDelegationSetError::DelegationSetAlreadyReusable(String::from(
                        parsed_error.message,
                    ))
                }
                "DelegationSetNotAvailable" => {
                    CreateReusableDelegationSetError::DelegationSetNotAvailable(String::from(
                        parsed_error.message,
                    ))
                }
                "HostedZoneNotFound" => CreateReusableDelegationSetError::HostedZoneNotFound(
                    String::from(parsed_error.message),
                ),
                "InvalidArgument" => CreateReusableDelegationSetError::InvalidArgument(
                    String::from(parsed_error.message),
                ),
                "InvalidInput" => CreateReusableDelegationSetError::InvalidInput(String::from(
                    parsed_error.message,
                )),
                "LimitsExceeded" => CreateReusableDelegationSetError::LimitsExceeded(String::from(
                    parsed_error.message,
                )),
                _ => CreateReusableDelegationSetError::Unknown(String::from(body)),
            },
            Err(_) => CreateReusableDelegationSetError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateReusableDelegationSetError {
    fn from(err: XmlParseError) -> CreateReusableDelegationSetError {
        let XmlParseError(message) = err;
        CreateReusableDelegationSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateReusableDelegationSetError {
    fn from(err: CredentialsError) -> CreateReusableDelegationSetError {
        CreateReusableDelegationSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateReusableDelegationSetError {
    fn from(err: HttpDispatchError) -> CreateReusableDelegationSetError {
        CreateReusableDelegationSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateReusableDelegationSetError {
    fn from(err: io::Error) -> CreateReusableDelegationSetError {
        CreateReusableDelegationSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateReusableDelegationSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReusableDelegationSetError {
    fn description(&self) -> &str {
        match *self {
            CreateReusableDelegationSetError::DelegationSetAlreadyCreated(ref cause) => cause,
            CreateReusableDelegationSetError::DelegationSetAlreadyReusable(ref cause) => cause,
            CreateReusableDelegationSetError::DelegationSetNotAvailable(ref cause) => cause,
            CreateReusableDelegationSetError::HostedZoneNotFound(ref cause) => cause,
            CreateReusableDelegationSetError::InvalidArgument(ref cause) => cause,
            CreateReusableDelegationSetError::InvalidInput(ref cause) => cause,
            CreateReusableDelegationSetError::LimitsExceeded(ref cause) => cause,
            CreateReusableDelegationSetError::Validation(ref cause) => cause,
            CreateReusableDelegationSetError::Credentials(ref err) => err.description(),
            CreateReusableDelegationSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateReusableDelegationSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTrafficPolicy
#[derive(Debug, PartialEq)]
pub enum CreateTrafficPolicyError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>The format of the traffic policy document that you specified in the <code>Document</code> element is invalid.</p>
    InvalidTrafficPolicyDocument(String),
    /// <p>This traffic policy can't be created because the current account has reached the limit on the number of traffic policies.</p> <p>For information about default limits, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>To get the current limit for an account, see <a>GetAccountLimit</a>. </p> <p>To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p>
    TooManyTrafficPolicies(String),
    /// <p>A traffic policy that has the same value for <code>Name</code> already exists.</p>
    TrafficPolicyAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTrafficPolicyError {
    pub fn from_body(body: &str) -> CreateTrafficPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    CreateTrafficPolicyError::InvalidInput(String::from(parsed_error.message))
                }
                "InvalidTrafficPolicyDocument" => {
                    CreateTrafficPolicyError::InvalidTrafficPolicyDocument(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyTrafficPolicies" => CreateTrafficPolicyError::TooManyTrafficPolicies(
                    String::from(parsed_error.message),
                ),
                "TrafficPolicyAlreadyExists" => {
                    CreateTrafficPolicyError::TrafficPolicyAlreadyExists(String::from(
                        parsed_error.message,
                    ))
                }
                _ => CreateTrafficPolicyError::Unknown(String::from(body)),
            },
            Err(_) => CreateTrafficPolicyError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateTrafficPolicyError {
    fn from(err: XmlParseError) -> CreateTrafficPolicyError {
        let XmlParseError(message) = err;
        CreateTrafficPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateTrafficPolicyError {
    fn from(err: CredentialsError) -> CreateTrafficPolicyError {
        CreateTrafficPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTrafficPolicyError {
    fn from(err: HttpDispatchError) -> CreateTrafficPolicyError {
        CreateTrafficPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTrafficPolicyError {
    fn from(err: io::Error) -> CreateTrafficPolicyError {
        CreateTrafficPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTrafficPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTrafficPolicyError {
    fn description(&self) -> &str {
        match *self {
            CreateTrafficPolicyError::InvalidInput(ref cause) => cause,
            CreateTrafficPolicyError::InvalidTrafficPolicyDocument(ref cause) => cause,
            CreateTrafficPolicyError::TooManyTrafficPolicies(ref cause) => cause,
            CreateTrafficPolicyError::TrafficPolicyAlreadyExists(ref cause) => cause,
            CreateTrafficPolicyError::Validation(ref cause) => cause,
            CreateTrafficPolicyError::Credentials(ref err) => err.description(),
            CreateTrafficPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateTrafficPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTrafficPolicyInstance
#[derive(Debug, PartialEq)]
pub enum CreateTrafficPolicyInstanceError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>No traffic policy exists with the specified ID.</p>
    NoSuchTrafficPolicy(String),
    /// <p>This traffic policy instance can't be created because the current account has reached the limit on the number of traffic policy instances.</p> <p>For information about default limits, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>For information about how to get the current limit for an account, see <a>GetAccountLimit</a>.</p> <p>To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p>
    TooManyTrafficPolicyInstances(String),
    /// <p>There is already a traffic policy instance with the specified ID.</p>
    TrafficPolicyInstanceAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTrafficPolicyInstanceError {
    pub fn from_body(body: &str) -> CreateTrafficPolicyInstanceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => CreateTrafficPolicyInstanceError::InvalidInput(String::from(
                    parsed_error.message,
                )),
                "NoSuchHostedZone" => CreateTrafficPolicyInstanceError::NoSuchHostedZone(
                    String::from(parsed_error.message),
                ),
                "NoSuchTrafficPolicy" => CreateTrafficPolicyInstanceError::NoSuchTrafficPolicy(
                    String::from(parsed_error.message),
                ),
                "TooManyTrafficPolicyInstances" => {
                    CreateTrafficPolicyInstanceError::TooManyTrafficPolicyInstances(String::from(
                        parsed_error.message,
                    ))
                }
                "TrafficPolicyInstanceAlreadyExists" => {
                    CreateTrafficPolicyInstanceError::TrafficPolicyInstanceAlreadyExists(
                        String::from(parsed_error.message),
                    )
                }
                _ => CreateTrafficPolicyInstanceError::Unknown(String::from(body)),
            },
            Err(_) => CreateTrafficPolicyInstanceError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateTrafficPolicyInstanceError {
    fn from(err: XmlParseError) -> CreateTrafficPolicyInstanceError {
        let XmlParseError(message) = err;
        CreateTrafficPolicyInstanceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateTrafficPolicyInstanceError {
    fn from(err: CredentialsError) -> CreateTrafficPolicyInstanceError {
        CreateTrafficPolicyInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTrafficPolicyInstanceError {
    fn from(err: HttpDispatchError) -> CreateTrafficPolicyInstanceError {
        CreateTrafficPolicyInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTrafficPolicyInstanceError {
    fn from(err: io::Error) -> CreateTrafficPolicyInstanceError {
        CreateTrafficPolicyInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTrafficPolicyInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTrafficPolicyInstanceError {
    fn description(&self) -> &str {
        match *self {
            CreateTrafficPolicyInstanceError::InvalidInput(ref cause) => cause,
            CreateTrafficPolicyInstanceError::NoSuchHostedZone(ref cause) => cause,
            CreateTrafficPolicyInstanceError::NoSuchTrafficPolicy(ref cause) => cause,
            CreateTrafficPolicyInstanceError::TooManyTrafficPolicyInstances(ref cause) => cause,
            CreateTrafficPolicyInstanceError::TrafficPolicyInstanceAlreadyExists(ref cause) => {
                cause
            }
            CreateTrafficPolicyInstanceError::Validation(ref cause) => cause,
            CreateTrafficPolicyInstanceError::Credentials(ref err) => err.description(),
            CreateTrafficPolicyInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateTrafficPolicyInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTrafficPolicyVersion
#[derive(Debug, PartialEq)]
pub enum CreateTrafficPolicyVersionError {
    /// <p>Another user submitted a request to create, update, or delete the object at the same time that you did. Retry the request. </p>
    ConcurrentModification(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>The format of the traffic policy document that you specified in the <code>Document</code> element is invalid.</p>
    InvalidTrafficPolicyDocument(String),
    /// <p>No traffic policy exists with the specified ID.</p>
    NoSuchTrafficPolicy(String),
    /// <p>This traffic policy version can't be created because you've reached the limit of 1000 on the number of versions that you can create for the current traffic policy.</p> <p>To create more traffic policy versions, you can use <a>GetTrafficPolicy</a> to get the traffic policy document for a specified traffic policy version, and then use <a>CreateTrafficPolicy</a> to create a new traffic policy using the traffic policy document.</p>
    TooManyTrafficPolicyVersionsForCurrentPolicy(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTrafficPolicyVersionError {
    pub fn from_body(body: &str) -> CreateTrafficPolicyVersionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConcurrentModification" => {
                    CreateTrafficPolicyVersionError::ConcurrentModification(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidInput" => CreateTrafficPolicyVersionError::InvalidInput(String::from(
                    parsed_error.message,
                )),
                "InvalidTrafficPolicyDocument" => {
                    CreateTrafficPolicyVersionError::InvalidTrafficPolicyDocument(String::from(
                        parsed_error.message,
                    ))
                }
                "NoSuchTrafficPolicy" => CreateTrafficPolicyVersionError::NoSuchTrafficPolicy(
                    String::from(parsed_error.message),
                ),
                "TooManyTrafficPolicyVersionsForCurrentPolicy" => {
                    CreateTrafficPolicyVersionError::TooManyTrafficPolicyVersionsForCurrentPolicy(
                        String::from(parsed_error.message),
                    )
                }
                _ => CreateTrafficPolicyVersionError::Unknown(String::from(body)),
            },
            Err(_) => CreateTrafficPolicyVersionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateTrafficPolicyVersionError {
    fn from(err: XmlParseError) -> CreateTrafficPolicyVersionError {
        let XmlParseError(message) = err;
        CreateTrafficPolicyVersionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateTrafficPolicyVersionError {
    fn from(err: CredentialsError) -> CreateTrafficPolicyVersionError {
        CreateTrafficPolicyVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTrafficPolicyVersionError {
    fn from(err: HttpDispatchError) -> CreateTrafficPolicyVersionError {
        CreateTrafficPolicyVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTrafficPolicyVersionError {
    fn from(err: io::Error) -> CreateTrafficPolicyVersionError {
        CreateTrafficPolicyVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTrafficPolicyVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTrafficPolicyVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateTrafficPolicyVersionError::ConcurrentModification(ref cause) => cause,
            CreateTrafficPolicyVersionError::InvalidInput(ref cause) => cause,
            CreateTrafficPolicyVersionError::InvalidTrafficPolicyDocument(ref cause) => cause,
            CreateTrafficPolicyVersionError::NoSuchTrafficPolicy(ref cause) => cause,
            CreateTrafficPolicyVersionError::TooManyTrafficPolicyVersionsForCurrentPolicy(
                ref cause,
            ) => cause,
            CreateTrafficPolicyVersionError::Validation(ref cause) => cause,
            CreateTrafficPolicyVersionError::Credentials(ref err) => err.description(),
            CreateTrafficPolicyVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateTrafficPolicyVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateVPCAssociationAuthorization
#[derive(Debug, PartialEq)]
pub enum CreateVPCAssociationAuthorizationError {
    /// <p>Another user submitted a request to create, update, or delete the object at the same time that you did. Retry the request. </p>
    ConcurrentModification(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC.</p>
    InvalidVPCId(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>You've created the maximum number of authorizations that can be created for the specified hosted zone. To authorize another VPC to be associated with the hosted zone, submit a <code>DeleteVPCAssociationAuthorization</code> request to remove an existing authorization. To get a list of existing authorizations, submit a <code>ListVPCAssociationAuthorizations</code> request.</p>
    TooManyVPCAssociationAuthorizations(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateVPCAssociationAuthorizationError {
    pub fn from_body(body: &str) -> CreateVPCAssociationAuthorizationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConcurrentModification" => {
                    CreateVPCAssociationAuthorizationError::ConcurrentModification(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidInput" => CreateVPCAssociationAuthorizationError::InvalidInput(
                    String::from(parsed_error.message),
                ),
                "InvalidVPCId" => CreateVPCAssociationAuthorizationError::InvalidVPCId(
                    String::from(parsed_error.message),
                ),
                "NoSuchHostedZone" => CreateVPCAssociationAuthorizationError::NoSuchHostedZone(
                    String::from(parsed_error.message),
                ),
                "TooManyVPCAssociationAuthorizations" => {
                    CreateVPCAssociationAuthorizationError::TooManyVPCAssociationAuthorizations(
                        String::from(parsed_error.message),
                    )
                }
                _ => CreateVPCAssociationAuthorizationError::Unknown(String::from(body)),
            },
            Err(_) => CreateVPCAssociationAuthorizationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateVPCAssociationAuthorizationError {
    fn from(err: XmlParseError) -> CreateVPCAssociationAuthorizationError {
        let XmlParseError(message) = err;
        CreateVPCAssociationAuthorizationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateVPCAssociationAuthorizationError {
    fn from(err: CredentialsError) -> CreateVPCAssociationAuthorizationError {
        CreateVPCAssociationAuthorizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateVPCAssociationAuthorizationError {
    fn from(err: HttpDispatchError) -> CreateVPCAssociationAuthorizationError {
        CreateVPCAssociationAuthorizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateVPCAssociationAuthorizationError {
    fn from(err: io::Error) -> CreateVPCAssociationAuthorizationError {
        CreateVPCAssociationAuthorizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateVPCAssociationAuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateVPCAssociationAuthorizationError {
    fn description(&self) -> &str {
        match *self {
            CreateVPCAssociationAuthorizationError::ConcurrentModification(ref cause) => cause,
            CreateVPCAssociationAuthorizationError::InvalidInput(ref cause) => cause,
            CreateVPCAssociationAuthorizationError::InvalidVPCId(ref cause) => cause,
            CreateVPCAssociationAuthorizationError::NoSuchHostedZone(ref cause) => cause,
            CreateVPCAssociationAuthorizationError::TooManyVPCAssociationAuthorizations(
                ref cause,
            ) => cause,
            CreateVPCAssociationAuthorizationError::Validation(ref cause) => cause,
            CreateVPCAssociationAuthorizationError::Credentials(ref err) => err.description(),
            CreateVPCAssociationAuthorizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateVPCAssociationAuthorizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteHealthCheck
#[derive(Debug, PartialEq)]
pub enum DeleteHealthCheckError {
    /// <p>This error code is not in use.</p>
    HealthCheckInUse(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
    NoSuchHealthCheck(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteHealthCheckError {
    pub fn from_body(body: &str) -> DeleteHealthCheckError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "HealthCheckInUse" => {
                    DeleteHealthCheckError::HealthCheckInUse(String::from(parsed_error.message))
                }
                "InvalidInput" => {
                    DeleteHealthCheckError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHealthCheck" => {
                    DeleteHealthCheckError::NoSuchHealthCheck(String::from(parsed_error.message))
                }
                _ => DeleteHealthCheckError::Unknown(String::from(body)),
            },
            Err(_) => DeleteHealthCheckError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteHealthCheckError {
    fn from(err: XmlParseError) -> DeleteHealthCheckError {
        let XmlParseError(message) = err;
        DeleteHealthCheckError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteHealthCheckError {
    fn from(err: CredentialsError) -> DeleteHealthCheckError {
        DeleteHealthCheckError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteHealthCheckError {
    fn from(err: HttpDispatchError) -> DeleteHealthCheckError {
        DeleteHealthCheckError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteHealthCheckError {
    fn from(err: io::Error) -> DeleteHealthCheckError {
        DeleteHealthCheckError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteHealthCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteHealthCheckError {
    fn description(&self) -> &str {
        match *self {
            DeleteHealthCheckError::HealthCheckInUse(ref cause) => cause,
            DeleteHealthCheckError::InvalidInput(ref cause) => cause,
            DeleteHealthCheckError::NoSuchHealthCheck(ref cause) => cause,
            DeleteHealthCheckError::Validation(ref cause) => cause,
            DeleteHealthCheckError::Credentials(ref err) => err.description(),
            DeleteHealthCheckError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteHealthCheckError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteHostedZone
#[derive(Debug, PartialEq)]
pub enum DeleteHostedZoneError {
    /// <p>The hosted zone contains resource records that are not SOA or NS records.</p>
    HostedZoneNotEmpty(String),
    /// <p>The specified domain name is not valid.</p>
    InvalidDomainName(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteHostedZoneError {
    pub fn from_body(body: &str) -> DeleteHostedZoneError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "HostedZoneNotEmpty" => {
                    DeleteHostedZoneError::HostedZoneNotEmpty(String::from(parsed_error.message))
                }
                "InvalidDomainName" => {
                    DeleteHostedZoneError::InvalidDomainName(String::from(parsed_error.message))
                }
                "InvalidInput" => {
                    DeleteHostedZoneError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHostedZone" => {
                    DeleteHostedZoneError::NoSuchHostedZone(String::from(parsed_error.message))
                }
                "PriorRequestNotComplete" => DeleteHostedZoneError::PriorRequestNotComplete(
                    String::from(parsed_error.message),
                ),
                _ => DeleteHostedZoneError::Unknown(String::from(body)),
            },
            Err(_) => DeleteHostedZoneError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteHostedZoneError {
    fn from(err: XmlParseError) -> DeleteHostedZoneError {
        let XmlParseError(message) = err;
        DeleteHostedZoneError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteHostedZoneError {
    fn from(err: CredentialsError) -> DeleteHostedZoneError {
        DeleteHostedZoneError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteHostedZoneError {
    fn from(err: HttpDispatchError) -> DeleteHostedZoneError {
        DeleteHostedZoneError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteHostedZoneError {
    fn from(err: io::Error) -> DeleteHostedZoneError {
        DeleteHostedZoneError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteHostedZoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteHostedZoneError {
    fn description(&self) -> &str {
        match *self {
            DeleteHostedZoneError::HostedZoneNotEmpty(ref cause) => cause,
            DeleteHostedZoneError::InvalidDomainName(ref cause) => cause,
            DeleteHostedZoneError::InvalidInput(ref cause) => cause,
            DeleteHostedZoneError::NoSuchHostedZone(ref cause) => cause,
            DeleteHostedZoneError::PriorRequestNotComplete(ref cause) => cause,
            DeleteHostedZoneError::Validation(ref cause) => cause,
            DeleteHostedZoneError::Credentials(ref err) => err.description(),
            DeleteHostedZoneError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteHostedZoneError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteQueryLoggingConfig
#[derive(Debug, PartialEq)]
pub enum DeleteQueryLoggingConfigError {
    /// <p>Another user submitted a request to create, update, or delete the object at the same time that you did. Retry the request. </p>
    ConcurrentModification(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>There is no DNS query logging configuration with the specified ID.</p>
    NoSuchQueryLoggingConfig(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteQueryLoggingConfigError {
    pub fn from_body(body: &str) -> DeleteQueryLoggingConfigError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConcurrentModification" => DeleteQueryLoggingConfigError::ConcurrentModification(
                    String::from(parsed_error.message),
                ),
                "InvalidInput" => {
                    DeleteQueryLoggingConfigError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchQueryLoggingConfig" => {
                    DeleteQueryLoggingConfigError::NoSuchQueryLoggingConfig(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteQueryLoggingConfigError::Unknown(String::from(body)),
            },
            Err(_) => DeleteQueryLoggingConfigError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteQueryLoggingConfigError {
    fn from(err: XmlParseError) -> DeleteQueryLoggingConfigError {
        let XmlParseError(message) = err;
        DeleteQueryLoggingConfigError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteQueryLoggingConfigError {
    fn from(err: CredentialsError) -> DeleteQueryLoggingConfigError {
        DeleteQueryLoggingConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteQueryLoggingConfigError {
    fn from(err: HttpDispatchError) -> DeleteQueryLoggingConfigError {
        DeleteQueryLoggingConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteQueryLoggingConfigError {
    fn from(err: io::Error) -> DeleteQueryLoggingConfigError {
        DeleteQueryLoggingConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteQueryLoggingConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteQueryLoggingConfigError {
    fn description(&self) -> &str {
        match *self {
            DeleteQueryLoggingConfigError::ConcurrentModification(ref cause) => cause,
            DeleteQueryLoggingConfigError::InvalidInput(ref cause) => cause,
            DeleteQueryLoggingConfigError::NoSuchQueryLoggingConfig(ref cause) => cause,
            DeleteQueryLoggingConfigError::Validation(ref cause) => cause,
            DeleteQueryLoggingConfigError::Credentials(ref err) => err.description(),
            DeleteQueryLoggingConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteQueryLoggingConfigError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteReusableDelegationSet
#[derive(Debug, PartialEq)]
pub enum DeleteReusableDelegationSetError {
    /// <p>The specified delegation contains associated hosted zones which must be deleted before the reusable delegation set can be deleted.</p>
    DelegationSetInUse(String),
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    DelegationSetNotReusable(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    NoSuchDelegationSet(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteReusableDelegationSetError {
    pub fn from_body(body: &str) -> DeleteReusableDelegationSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "DelegationSetInUse" => DeleteReusableDelegationSetError::DelegationSetInUse(
                    String::from(parsed_error.message),
                ),
                "DelegationSetNotReusable" => {
                    DeleteReusableDelegationSetError::DelegationSetNotReusable(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidInput" => DeleteReusableDelegationSetError::InvalidInput(String::from(
                    parsed_error.message,
                )),
                "NoSuchDelegationSet" => DeleteReusableDelegationSetError::NoSuchDelegationSet(
                    String::from(parsed_error.message),
                ),
                _ => DeleteReusableDelegationSetError::Unknown(String::from(body)),
            },
            Err(_) => DeleteReusableDelegationSetError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteReusableDelegationSetError {
    fn from(err: XmlParseError) -> DeleteReusableDelegationSetError {
        let XmlParseError(message) = err;
        DeleteReusableDelegationSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteReusableDelegationSetError {
    fn from(err: CredentialsError) -> DeleteReusableDelegationSetError {
        DeleteReusableDelegationSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteReusableDelegationSetError {
    fn from(err: HttpDispatchError) -> DeleteReusableDelegationSetError {
        DeleteReusableDelegationSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteReusableDelegationSetError {
    fn from(err: io::Error) -> DeleteReusableDelegationSetError {
        DeleteReusableDelegationSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteReusableDelegationSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReusableDelegationSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteReusableDelegationSetError::DelegationSetInUse(ref cause) => cause,
            DeleteReusableDelegationSetError::DelegationSetNotReusable(ref cause) => cause,
            DeleteReusableDelegationSetError::InvalidInput(ref cause) => cause,
            DeleteReusableDelegationSetError::NoSuchDelegationSet(ref cause) => cause,
            DeleteReusableDelegationSetError::Validation(ref cause) => cause,
            DeleteReusableDelegationSetError::Credentials(ref err) => err.description(),
            DeleteReusableDelegationSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteReusableDelegationSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTrafficPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteTrafficPolicyError {
    /// <p>Another user submitted a request to create, update, or delete the object at the same time that you did. Retry the request. </p>
    ConcurrentModification(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy exists with the specified ID.</p>
    NoSuchTrafficPolicy(String),
    /// <p>One or more traffic policy instances were created by using the specified traffic policy.</p>
    TrafficPolicyInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTrafficPolicyError {
    pub fn from_body(body: &str) -> DeleteTrafficPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConcurrentModification" => DeleteTrafficPolicyError::ConcurrentModification(
                    String::from(parsed_error.message),
                ),
                "InvalidInput" => {
                    DeleteTrafficPolicyError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchTrafficPolicy" => DeleteTrafficPolicyError::NoSuchTrafficPolicy(
                    String::from(parsed_error.message),
                ),
                "TrafficPolicyInUse" => {
                    DeleteTrafficPolicyError::TrafficPolicyInUse(String::from(parsed_error.message))
                }
                _ => DeleteTrafficPolicyError::Unknown(String::from(body)),
            },
            Err(_) => DeleteTrafficPolicyError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteTrafficPolicyError {
    fn from(err: XmlParseError) -> DeleteTrafficPolicyError {
        let XmlParseError(message) = err;
        DeleteTrafficPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteTrafficPolicyError {
    fn from(err: CredentialsError) -> DeleteTrafficPolicyError {
        DeleteTrafficPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTrafficPolicyError {
    fn from(err: HttpDispatchError) -> DeleteTrafficPolicyError {
        DeleteTrafficPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTrafficPolicyError {
    fn from(err: io::Error) -> DeleteTrafficPolicyError {
        DeleteTrafficPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTrafficPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTrafficPolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteTrafficPolicyError::ConcurrentModification(ref cause) => cause,
            DeleteTrafficPolicyError::InvalidInput(ref cause) => cause,
            DeleteTrafficPolicyError::NoSuchTrafficPolicy(ref cause) => cause,
            DeleteTrafficPolicyError::TrafficPolicyInUse(ref cause) => cause,
            DeleteTrafficPolicyError::Validation(ref cause) => cause,
            DeleteTrafficPolicyError::Credentials(ref err) => err.description(),
            DeleteTrafficPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteTrafficPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTrafficPolicyInstance
#[derive(Debug, PartialEq)]
pub enum DeleteTrafficPolicyInstanceError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy instance exists with the specified ID.</p>
    NoSuchTrafficPolicyInstance(String),
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTrafficPolicyInstanceError {
    pub fn from_body(body: &str) -> DeleteTrafficPolicyInstanceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => DeleteTrafficPolicyInstanceError::InvalidInput(String::from(
                    parsed_error.message,
                )),
                "NoSuchTrafficPolicyInstance" => {
                    DeleteTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(String::from(
                        parsed_error.message,
                    ))
                }
                "PriorRequestNotComplete" => {
                    DeleteTrafficPolicyInstanceError::PriorRequestNotComplete(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteTrafficPolicyInstanceError::Unknown(String::from(body)),
            },
            Err(_) => DeleteTrafficPolicyInstanceError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteTrafficPolicyInstanceError {
    fn from(err: XmlParseError) -> DeleteTrafficPolicyInstanceError {
        let XmlParseError(message) = err;
        DeleteTrafficPolicyInstanceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteTrafficPolicyInstanceError {
    fn from(err: CredentialsError) -> DeleteTrafficPolicyInstanceError {
        DeleteTrafficPolicyInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTrafficPolicyInstanceError {
    fn from(err: HttpDispatchError) -> DeleteTrafficPolicyInstanceError {
        DeleteTrafficPolicyInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTrafficPolicyInstanceError {
    fn from(err: io::Error) -> DeleteTrafficPolicyInstanceError {
        DeleteTrafficPolicyInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTrafficPolicyInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTrafficPolicyInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeleteTrafficPolicyInstanceError::InvalidInput(ref cause) => cause,
            DeleteTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(ref cause) => cause,
            DeleteTrafficPolicyInstanceError::PriorRequestNotComplete(ref cause) => cause,
            DeleteTrafficPolicyInstanceError::Validation(ref cause) => cause,
            DeleteTrafficPolicyInstanceError::Credentials(ref err) => err.description(),
            DeleteTrafficPolicyInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteTrafficPolicyInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVPCAssociationAuthorization
#[derive(Debug, PartialEq)]
pub enum DeleteVPCAssociationAuthorizationError {
    /// <p>Another user submitted a request to create, update, or delete the object at the same time that you did. Retry the request. </p>
    ConcurrentModification(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC.</p>
    InvalidVPCId(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>The VPC that you specified is not authorized to be associated with the hosted zone.</p>
    VPCAssociationAuthorizationNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteVPCAssociationAuthorizationError {
    pub fn from_body(body: &str) -> DeleteVPCAssociationAuthorizationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConcurrentModification" => {
                    DeleteVPCAssociationAuthorizationError::ConcurrentModification(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidInput" => DeleteVPCAssociationAuthorizationError::InvalidInput(
                    String::from(parsed_error.message),
                ),
                "InvalidVPCId" => DeleteVPCAssociationAuthorizationError::InvalidVPCId(
                    String::from(parsed_error.message),
                ),
                "NoSuchHostedZone" => DeleteVPCAssociationAuthorizationError::NoSuchHostedZone(
                    String::from(parsed_error.message),
                ),
                "VPCAssociationAuthorizationNotFound" => {
                    DeleteVPCAssociationAuthorizationError::VPCAssociationAuthorizationNotFound(
                        String::from(parsed_error.message),
                    )
                }
                _ => DeleteVPCAssociationAuthorizationError::Unknown(String::from(body)),
            },
            Err(_) => DeleteVPCAssociationAuthorizationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteVPCAssociationAuthorizationError {
    fn from(err: XmlParseError) -> DeleteVPCAssociationAuthorizationError {
        let XmlParseError(message) = err;
        DeleteVPCAssociationAuthorizationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteVPCAssociationAuthorizationError {
    fn from(err: CredentialsError) -> DeleteVPCAssociationAuthorizationError {
        DeleteVPCAssociationAuthorizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteVPCAssociationAuthorizationError {
    fn from(err: HttpDispatchError) -> DeleteVPCAssociationAuthorizationError {
        DeleteVPCAssociationAuthorizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteVPCAssociationAuthorizationError {
    fn from(err: io::Error) -> DeleteVPCAssociationAuthorizationError {
        DeleteVPCAssociationAuthorizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteVPCAssociationAuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVPCAssociationAuthorizationError {
    fn description(&self) -> &str {
        match *self {
            DeleteVPCAssociationAuthorizationError::ConcurrentModification(ref cause) => cause,
            DeleteVPCAssociationAuthorizationError::InvalidInput(ref cause) => cause,
            DeleteVPCAssociationAuthorizationError::InvalidVPCId(ref cause) => cause,
            DeleteVPCAssociationAuthorizationError::NoSuchHostedZone(ref cause) => cause,
            DeleteVPCAssociationAuthorizationError::VPCAssociationAuthorizationNotFound(
                ref cause,
            ) => cause,
            DeleteVPCAssociationAuthorizationError::Validation(ref cause) => cause,
            DeleteVPCAssociationAuthorizationError::Credentials(ref err) => err.description(),
            DeleteVPCAssociationAuthorizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteVPCAssociationAuthorizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateVPCFromHostedZone
#[derive(Debug, PartialEq)]
pub enum DisassociateVPCFromHostedZoneError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC.</p>
    InvalidVPCId(String),
    /// <p>The VPC that you're trying to disassociate from the private hosted zone is the last VPC that is associated with the hosted zone. Amazon Route 53 doesn't support disassociating the last VPC from a hosted zone.</p>
    LastVPCAssociation(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>The specified VPC and hosted zone are not currently associated.</p>
    VPCAssociationNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisassociateVPCFromHostedZoneError {
    pub fn from_body(body: &str) -> DisassociateVPCFromHostedZoneError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => DisassociateVPCFromHostedZoneError::InvalidInput(String::from(
                    parsed_error.message,
                )),
                "InvalidVPCId" => DisassociateVPCFromHostedZoneError::InvalidVPCId(String::from(
                    parsed_error.message,
                )),
                "LastVPCAssociation" => DisassociateVPCFromHostedZoneError::LastVPCAssociation(
                    String::from(parsed_error.message),
                ),
                "NoSuchHostedZone" => DisassociateVPCFromHostedZoneError::NoSuchHostedZone(
                    String::from(parsed_error.message),
                ),
                "VPCAssociationNotFound" => {
                    DisassociateVPCFromHostedZoneError::VPCAssociationNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DisassociateVPCFromHostedZoneError::Unknown(String::from(body)),
            },
            Err(_) => DisassociateVPCFromHostedZoneError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DisassociateVPCFromHostedZoneError {
    fn from(err: XmlParseError) -> DisassociateVPCFromHostedZoneError {
        let XmlParseError(message) = err;
        DisassociateVPCFromHostedZoneError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DisassociateVPCFromHostedZoneError {
    fn from(err: CredentialsError) -> DisassociateVPCFromHostedZoneError {
        DisassociateVPCFromHostedZoneError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateVPCFromHostedZoneError {
    fn from(err: HttpDispatchError) -> DisassociateVPCFromHostedZoneError {
        DisassociateVPCFromHostedZoneError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateVPCFromHostedZoneError {
    fn from(err: io::Error) -> DisassociateVPCFromHostedZoneError {
        DisassociateVPCFromHostedZoneError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateVPCFromHostedZoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateVPCFromHostedZoneError {
    fn description(&self) -> &str {
        match *self {
            DisassociateVPCFromHostedZoneError::InvalidInput(ref cause) => cause,
            DisassociateVPCFromHostedZoneError::InvalidVPCId(ref cause) => cause,
            DisassociateVPCFromHostedZoneError::LastVPCAssociation(ref cause) => cause,
            DisassociateVPCFromHostedZoneError::NoSuchHostedZone(ref cause) => cause,
            DisassociateVPCFromHostedZoneError::VPCAssociationNotFound(ref cause) => cause,
            DisassociateVPCFromHostedZoneError::Validation(ref cause) => cause,
            DisassociateVPCFromHostedZoneError::Credentials(ref err) => err.description(),
            DisassociateVPCFromHostedZoneError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateVPCFromHostedZoneError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAccountLimit
#[derive(Debug, PartialEq)]
pub enum GetAccountLimitError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAccountLimitError {
    pub fn from_body(body: &str) -> GetAccountLimitError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    GetAccountLimitError::InvalidInput(String::from(parsed_error.message))
                }
                _ => GetAccountLimitError::Unknown(String::from(body)),
            },
            Err(_) => GetAccountLimitError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetAccountLimitError {
    fn from(err: XmlParseError) -> GetAccountLimitError {
        let XmlParseError(message) = err;
        GetAccountLimitError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetAccountLimitError {
    fn from(err: CredentialsError) -> GetAccountLimitError {
        GetAccountLimitError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAccountLimitError {
    fn from(err: HttpDispatchError) -> GetAccountLimitError {
        GetAccountLimitError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAccountLimitError {
    fn from(err: io::Error) -> GetAccountLimitError {
        GetAccountLimitError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAccountLimitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAccountLimitError {
    fn description(&self) -> &str {
        match *self {
            GetAccountLimitError::InvalidInput(ref cause) => cause,
            GetAccountLimitError::Validation(ref cause) => cause,
            GetAccountLimitError::Credentials(ref err) => err.description(),
            GetAccountLimitError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAccountLimitError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetChange
#[derive(Debug, PartialEq)]
pub enum GetChangeError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>A change with the specified change ID does not exist.</p>
    NoSuchChange(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetChangeError {
    pub fn from_body(body: &str) -> GetChangeError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => GetChangeError::InvalidInput(String::from(parsed_error.message)),
                "NoSuchChange" => GetChangeError::NoSuchChange(String::from(parsed_error.message)),
                _ => GetChangeError::Unknown(String::from(body)),
            },
            Err(_) => GetChangeError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetChangeError {
    fn from(err: XmlParseError) -> GetChangeError {
        let XmlParseError(message) = err;
        GetChangeError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetChangeError {
    fn from(err: CredentialsError) -> GetChangeError {
        GetChangeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetChangeError {
    fn from(err: HttpDispatchError) -> GetChangeError {
        GetChangeError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetChangeError {
    fn from(err: io::Error) -> GetChangeError {
        GetChangeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetChangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetChangeError {
    fn description(&self) -> &str {
        match *self {
            GetChangeError::InvalidInput(ref cause) => cause,
            GetChangeError::NoSuchChange(ref cause) => cause,
            GetChangeError::Validation(ref cause) => cause,
            GetChangeError::Credentials(ref err) => err.description(),
            GetChangeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetChangeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCheckerIpRanges
#[derive(Debug, PartialEq)]
pub enum GetCheckerIpRangesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetCheckerIpRangesError {
    pub fn from_body(body: &str) -> GetCheckerIpRangesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetCheckerIpRangesError::Unknown(String::from(body)),
            },
            Err(_) => GetCheckerIpRangesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetCheckerIpRangesError {
    fn from(err: XmlParseError) -> GetCheckerIpRangesError {
        let XmlParseError(message) = err;
        GetCheckerIpRangesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetCheckerIpRangesError {
    fn from(err: CredentialsError) -> GetCheckerIpRangesError {
        GetCheckerIpRangesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCheckerIpRangesError {
    fn from(err: HttpDispatchError) -> GetCheckerIpRangesError {
        GetCheckerIpRangesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCheckerIpRangesError {
    fn from(err: io::Error) -> GetCheckerIpRangesError {
        GetCheckerIpRangesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCheckerIpRangesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCheckerIpRangesError {
    fn description(&self) -> &str {
        match *self {
            GetCheckerIpRangesError::Validation(ref cause) => cause,
            GetCheckerIpRangesError::Credentials(ref err) => err.description(),
            GetCheckerIpRangesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCheckerIpRangesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGeoLocation
#[derive(Debug, PartialEq)]
pub enum GetGeoLocationError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>Amazon Route 53 doesn't support the specified geolocation.</p>
    NoSuchGeoLocation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetGeoLocationError {
    pub fn from_body(body: &str) -> GetGeoLocationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    GetGeoLocationError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchGeoLocation" => {
                    GetGeoLocationError::NoSuchGeoLocation(String::from(parsed_error.message))
                }
                _ => GetGeoLocationError::Unknown(String::from(body)),
            },
            Err(_) => GetGeoLocationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetGeoLocationError {
    fn from(err: XmlParseError) -> GetGeoLocationError {
        let XmlParseError(message) = err;
        GetGeoLocationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetGeoLocationError {
    fn from(err: CredentialsError) -> GetGeoLocationError {
        GetGeoLocationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGeoLocationError {
    fn from(err: HttpDispatchError) -> GetGeoLocationError {
        GetGeoLocationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGeoLocationError {
    fn from(err: io::Error) -> GetGeoLocationError {
        GetGeoLocationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGeoLocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGeoLocationError {
    fn description(&self) -> &str {
        match *self {
            GetGeoLocationError::InvalidInput(ref cause) => cause,
            GetGeoLocationError::NoSuchGeoLocation(ref cause) => cause,
            GetGeoLocationError::Validation(ref cause) => cause,
            GetGeoLocationError::Credentials(ref err) => err.description(),
            GetGeoLocationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetGeoLocationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetHealthCheck
#[derive(Debug, PartialEq)]
pub enum GetHealthCheckError {
    /// <p>The resource you're trying to access is unsupported on this Amazon Route 53 endpoint.</p>
    IncompatibleVersion(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
    NoSuchHealthCheck(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetHealthCheckError {
    pub fn from_body(body: &str) -> GetHealthCheckError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "IncompatibleVersion" => {
                    GetHealthCheckError::IncompatibleVersion(String::from(parsed_error.message))
                }
                "InvalidInput" => {
                    GetHealthCheckError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHealthCheck" => {
                    GetHealthCheckError::NoSuchHealthCheck(String::from(parsed_error.message))
                }
                _ => GetHealthCheckError::Unknown(String::from(body)),
            },
            Err(_) => GetHealthCheckError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetHealthCheckError {
    fn from(err: XmlParseError) -> GetHealthCheckError {
        let XmlParseError(message) = err;
        GetHealthCheckError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetHealthCheckError {
    fn from(err: CredentialsError) -> GetHealthCheckError {
        GetHealthCheckError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetHealthCheckError {
    fn from(err: HttpDispatchError) -> GetHealthCheckError {
        GetHealthCheckError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetHealthCheckError {
    fn from(err: io::Error) -> GetHealthCheckError {
        GetHealthCheckError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetHealthCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetHealthCheckError {
    fn description(&self) -> &str {
        match *self {
            GetHealthCheckError::IncompatibleVersion(ref cause) => cause,
            GetHealthCheckError::InvalidInput(ref cause) => cause,
            GetHealthCheckError::NoSuchHealthCheck(ref cause) => cause,
            GetHealthCheckError::Validation(ref cause) => cause,
            GetHealthCheckError::Credentials(ref err) => err.description(),
            GetHealthCheckError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetHealthCheckError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetHealthCheckCount
#[derive(Debug, PartialEq)]
pub enum GetHealthCheckCountError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetHealthCheckCountError {
    pub fn from_body(body: &str) -> GetHealthCheckCountError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetHealthCheckCountError::Unknown(String::from(body)),
            },
            Err(_) => GetHealthCheckCountError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetHealthCheckCountError {
    fn from(err: XmlParseError) -> GetHealthCheckCountError {
        let XmlParseError(message) = err;
        GetHealthCheckCountError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetHealthCheckCountError {
    fn from(err: CredentialsError) -> GetHealthCheckCountError {
        GetHealthCheckCountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetHealthCheckCountError {
    fn from(err: HttpDispatchError) -> GetHealthCheckCountError {
        GetHealthCheckCountError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetHealthCheckCountError {
    fn from(err: io::Error) -> GetHealthCheckCountError {
        GetHealthCheckCountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetHealthCheckCountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetHealthCheckCountError {
    fn description(&self) -> &str {
        match *self {
            GetHealthCheckCountError::Validation(ref cause) => cause,
            GetHealthCheckCountError::Credentials(ref err) => err.description(),
            GetHealthCheckCountError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetHealthCheckCountError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetHealthCheckLastFailureReason
#[derive(Debug, PartialEq)]
pub enum GetHealthCheckLastFailureReasonError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
    NoSuchHealthCheck(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetHealthCheckLastFailureReasonError {
    pub fn from_body(body: &str) -> GetHealthCheckLastFailureReasonError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => GetHealthCheckLastFailureReasonError::InvalidInput(String::from(
                    parsed_error.message,
                )),
                "NoSuchHealthCheck" => GetHealthCheckLastFailureReasonError::NoSuchHealthCheck(
                    String::from(parsed_error.message),
                ),
                _ => GetHealthCheckLastFailureReasonError::Unknown(String::from(body)),
            },
            Err(_) => GetHealthCheckLastFailureReasonError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetHealthCheckLastFailureReasonError {
    fn from(err: XmlParseError) -> GetHealthCheckLastFailureReasonError {
        let XmlParseError(message) = err;
        GetHealthCheckLastFailureReasonError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetHealthCheckLastFailureReasonError {
    fn from(err: CredentialsError) -> GetHealthCheckLastFailureReasonError {
        GetHealthCheckLastFailureReasonError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetHealthCheckLastFailureReasonError {
    fn from(err: HttpDispatchError) -> GetHealthCheckLastFailureReasonError {
        GetHealthCheckLastFailureReasonError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetHealthCheckLastFailureReasonError {
    fn from(err: io::Error) -> GetHealthCheckLastFailureReasonError {
        GetHealthCheckLastFailureReasonError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetHealthCheckLastFailureReasonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetHealthCheckLastFailureReasonError {
    fn description(&self) -> &str {
        match *self {
            GetHealthCheckLastFailureReasonError::InvalidInput(ref cause) => cause,
            GetHealthCheckLastFailureReasonError::NoSuchHealthCheck(ref cause) => cause,
            GetHealthCheckLastFailureReasonError::Validation(ref cause) => cause,
            GetHealthCheckLastFailureReasonError::Credentials(ref err) => err.description(),
            GetHealthCheckLastFailureReasonError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetHealthCheckLastFailureReasonError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetHealthCheckStatus
#[derive(Debug, PartialEq)]
pub enum GetHealthCheckStatusError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
    NoSuchHealthCheck(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetHealthCheckStatusError {
    pub fn from_body(body: &str) -> GetHealthCheckStatusError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    GetHealthCheckStatusError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHealthCheck" => {
                    GetHealthCheckStatusError::NoSuchHealthCheck(String::from(parsed_error.message))
                }
                _ => GetHealthCheckStatusError::Unknown(String::from(body)),
            },
            Err(_) => GetHealthCheckStatusError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetHealthCheckStatusError {
    fn from(err: XmlParseError) -> GetHealthCheckStatusError {
        let XmlParseError(message) = err;
        GetHealthCheckStatusError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetHealthCheckStatusError {
    fn from(err: CredentialsError) -> GetHealthCheckStatusError {
        GetHealthCheckStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetHealthCheckStatusError {
    fn from(err: HttpDispatchError) -> GetHealthCheckStatusError {
        GetHealthCheckStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetHealthCheckStatusError {
    fn from(err: io::Error) -> GetHealthCheckStatusError {
        GetHealthCheckStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetHealthCheckStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetHealthCheckStatusError {
    fn description(&self) -> &str {
        match *self {
            GetHealthCheckStatusError::InvalidInput(ref cause) => cause,
            GetHealthCheckStatusError::NoSuchHealthCheck(ref cause) => cause,
            GetHealthCheckStatusError::Validation(ref cause) => cause,
            GetHealthCheckStatusError::Credentials(ref err) => err.description(),
            GetHealthCheckStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetHealthCheckStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetHostedZone
#[derive(Debug, PartialEq)]
pub enum GetHostedZoneError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetHostedZoneError {
    pub fn from_body(body: &str) -> GetHostedZoneError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    GetHostedZoneError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHostedZone" => {
                    GetHostedZoneError::NoSuchHostedZone(String::from(parsed_error.message))
                }
                _ => GetHostedZoneError::Unknown(String::from(body)),
            },
            Err(_) => GetHostedZoneError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetHostedZoneError {
    fn from(err: XmlParseError) -> GetHostedZoneError {
        let XmlParseError(message) = err;
        GetHostedZoneError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetHostedZoneError {
    fn from(err: CredentialsError) -> GetHostedZoneError {
        GetHostedZoneError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetHostedZoneError {
    fn from(err: HttpDispatchError) -> GetHostedZoneError {
        GetHostedZoneError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetHostedZoneError {
    fn from(err: io::Error) -> GetHostedZoneError {
        GetHostedZoneError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetHostedZoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetHostedZoneError {
    fn description(&self) -> &str {
        match *self {
            GetHostedZoneError::InvalidInput(ref cause) => cause,
            GetHostedZoneError::NoSuchHostedZone(ref cause) => cause,
            GetHostedZoneError::Validation(ref cause) => cause,
            GetHostedZoneError::Credentials(ref err) => err.description(),
            GetHostedZoneError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetHostedZoneError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetHostedZoneCount
#[derive(Debug, PartialEq)]
pub enum GetHostedZoneCountError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetHostedZoneCountError {
    pub fn from_body(body: &str) -> GetHostedZoneCountError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    GetHostedZoneCountError::InvalidInput(String::from(parsed_error.message))
                }
                _ => GetHostedZoneCountError::Unknown(String::from(body)),
            },
            Err(_) => GetHostedZoneCountError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetHostedZoneCountError {
    fn from(err: XmlParseError) -> GetHostedZoneCountError {
        let XmlParseError(message) = err;
        GetHostedZoneCountError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetHostedZoneCountError {
    fn from(err: CredentialsError) -> GetHostedZoneCountError {
        GetHostedZoneCountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetHostedZoneCountError {
    fn from(err: HttpDispatchError) -> GetHostedZoneCountError {
        GetHostedZoneCountError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetHostedZoneCountError {
    fn from(err: io::Error) -> GetHostedZoneCountError {
        GetHostedZoneCountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetHostedZoneCountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetHostedZoneCountError {
    fn description(&self) -> &str {
        match *self {
            GetHostedZoneCountError::InvalidInput(ref cause) => cause,
            GetHostedZoneCountError::Validation(ref cause) => cause,
            GetHostedZoneCountError::Credentials(ref err) => err.description(),
            GetHostedZoneCountError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetHostedZoneCountError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetHostedZoneLimit
#[derive(Debug, PartialEq)]
pub enum GetHostedZoneLimitError {
    /// <p>The specified hosted zone is a public hosted zone, not a private hosted zone.</p>
    HostedZoneNotPrivate(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetHostedZoneLimitError {
    pub fn from_body(body: &str) -> GetHostedZoneLimitError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "HostedZoneNotPrivate" => GetHostedZoneLimitError::HostedZoneNotPrivate(
                    String::from(parsed_error.message),
                ),
                "InvalidInput" => {
                    GetHostedZoneLimitError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHostedZone" => {
                    GetHostedZoneLimitError::NoSuchHostedZone(String::from(parsed_error.message))
                }
                _ => GetHostedZoneLimitError::Unknown(String::from(body)),
            },
            Err(_) => GetHostedZoneLimitError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetHostedZoneLimitError {
    fn from(err: XmlParseError) -> GetHostedZoneLimitError {
        let XmlParseError(message) = err;
        GetHostedZoneLimitError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetHostedZoneLimitError {
    fn from(err: CredentialsError) -> GetHostedZoneLimitError {
        GetHostedZoneLimitError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetHostedZoneLimitError {
    fn from(err: HttpDispatchError) -> GetHostedZoneLimitError {
        GetHostedZoneLimitError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetHostedZoneLimitError {
    fn from(err: io::Error) -> GetHostedZoneLimitError {
        GetHostedZoneLimitError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetHostedZoneLimitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetHostedZoneLimitError {
    fn description(&self) -> &str {
        match *self {
            GetHostedZoneLimitError::HostedZoneNotPrivate(ref cause) => cause,
            GetHostedZoneLimitError::InvalidInput(ref cause) => cause,
            GetHostedZoneLimitError::NoSuchHostedZone(ref cause) => cause,
            GetHostedZoneLimitError::Validation(ref cause) => cause,
            GetHostedZoneLimitError::Credentials(ref err) => err.description(),
            GetHostedZoneLimitError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetHostedZoneLimitError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetQueryLoggingConfig
#[derive(Debug, PartialEq)]
pub enum GetQueryLoggingConfigError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>There is no DNS query logging configuration with the specified ID.</p>
    NoSuchQueryLoggingConfig(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetQueryLoggingConfigError {
    pub fn from_body(body: &str) -> GetQueryLoggingConfigError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    GetQueryLoggingConfigError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchQueryLoggingConfig" => GetQueryLoggingConfigError::NoSuchQueryLoggingConfig(
                    String::from(parsed_error.message),
                ),
                _ => GetQueryLoggingConfigError::Unknown(String::from(body)),
            },
            Err(_) => GetQueryLoggingConfigError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetQueryLoggingConfigError {
    fn from(err: XmlParseError) -> GetQueryLoggingConfigError {
        let XmlParseError(message) = err;
        GetQueryLoggingConfigError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetQueryLoggingConfigError {
    fn from(err: CredentialsError) -> GetQueryLoggingConfigError {
        GetQueryLoggingConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetQueryLoggingConfigError {
    fn from(err: HttpDispatchError) -> GetQueryLoggingConfigError {
        GetQueryLoggingConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetQueryLoggingConfigError {
    fn from(err: io::Error) -> GetQueryLoggingConfigError {
        GetQueryLoggingConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetQueryLoggingConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetQueryLoggingConfigError {
    fn description(&self) -> &str {
        match *self {
            GetQueryLoggingConfigError::InvalidInput(ref cause) => cause,
            GetQueryLoggingConfigError::NoSuchQueryLoggingConfig(ref cause) => cause,
            GetQueryLoggingConfigError::Validation(ref cause) => cause,
            GetQueryLoggingConfigError::Credentials(ref err) => err.description(),
            GetQueryLoggingConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetQueryLoggingConfigError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetReusableDelegationSet
#[derive(Debug, PartialEq)]
pub enum GetReusableDelegationSetError {
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    DelegationSetNotReusable(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    NoSuchDelegationSet(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetReusableDelegationSetError {
    pub fn from_body(body: &str) -> GetReusableDelegationSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "DelegationSetNotReusable" => {
                    GetReusableDelegationSetError::DelegationSetNotReusable(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidInput" => {
                    GetReusableDelegationSetError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchDelegationSet" => GetReusableDelegationSetError::NoSuchDelegationSet(
                    String::from(parsed_error.message),
                ),
                _ => GetReusableDelegationSetError::Unknown(String::from(body)),
            },
            Err(_) => GetReusableDelegationSetError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetReusableDelegationSetError {
    fn from(err: XmlParseError) -> GetReusableDelegationSetError {
        let XmlParseError(message) = err;
        GetReusableDelegationSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetReusableDelegationSetError {
    fn from(err: CredentialsError) -> GetReusableDelegationSetError {
        GetReusableDelegationSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetReusableDelegationSetError {
    fn from(err: HttpDispatchError) -> GetReusableDelegationSetError {
        GetReusableDelegationSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetReusableDelegationSetError {
    fn from(err: io::Error) -> GetReusableDelegationSetError {
        GetReusableDelegationSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetReusableDelegationSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetReusableDelegationSetError {
    fn description(&self) -> &str {
        match *self {
            GetReusableDelegationSetError::DelegationSetNotReusable(ref cause) => cause,
            GetReusableDelegationSetError::InvalidInput(ref cause) => cause,
            GetReusableDelegationSetError::NoSuchDelegationSet(ref cause) => cause,
            GetReusableDelegationSetError::Validation(ref cause) => cause,
            GetReusableDelegationSetError::Credentials(ref err) => err.description(),
            GetReusableDelegationSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetReusableDelegationSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetReusableDelegationSetLimit
#[derive(Debug, PartialEq)]
pub enum GetReusableDelegationSetLimitError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    NoSuchDelegationSet(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetReusableDelegationSetLimitError {
    pub fn from_body(body: &str) -> GetReusableDelegationSetLimitError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => GetReusableDelegationSetLimitError::InvalidInput(String::from(
                    parsed_error.message,
                )),
                "NoSuchDelegationSet" => GetReusableDelegationSetLimitError::NoSuchDelegationSet(
                    String::from(parsed_error.message),
                ),
                _ => GetReusableDelegationSetLimitError::Unknown(String::from(body)),
            },
            Err(_) => GetReusableDelegationSetLimitError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetReusableDelegationSetLimitError {
    fn from(err: XmlParseError) -> GetReusableDelegationSetLimitError {
        let XmlParseError(message) = err;
        GetReusableDelegationSetLimitError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetReusableDelegationSetLimitError {
    fn from(err: CredentialsError) -> GetReusableDelegationSetLimitError {
        GetReusableDelegationSetLimitError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetReusableDelegationSetLimitError {
    fn from(err: HttpDispatchError) -> GetReusableDelegationSetLimitError {
        GetReusableDelegationSetLimitError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetReusableDelegationSetLimitError {
    fn from(err: io::Error) -> GetReusableDelegationSetLimitError {
        GetReusableDelegationSetLimitError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetReusableDelegationSetLimitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetReusableDelegationSetLimitError {
    fn description(&self) -> &str {
        match *self {
            GetReusableDelegationSetLimitError::InvalidInput(ref cause) => cause,
            GetReusableDelegationSetLimitError::NoSuchDelegationSet(ref cause) => cause,
            GetReusableDelegationSetLimitError::Validation(ref cause) => cause,
            GetReusableDelegationSetLimitError::Credentials(ref err) => err.description(),
            GetReusableDelegationSetLimitError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetReusableDelegationSetLimitError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTrafficPolicy
#[derive(Debug, PartialEq)]
pub enum GetTrafficPolicyError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy exists with the specified ID.</p>
    NoSuchTrafficPolicy(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetTrafficPolicyError {
    pub fn from_body(body: &str) -> GetTrafficPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    GetTrafficPolicyError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchTrafficPolicy" => {
                    GetTrafficPolicyError::NoSuchTrafficPolicy(String::from(parsed_error.message))
                }
                _ => GetTrafficPolicyError::Unknown(String::from(body)),
            },
            Err(_) => GetTrafficPolicyError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetTrafficPolicyError {
    fn from(err: XmlParseError) -> GetTrafficPolicyError {
        let XmlParseError(message) = err;
        GetTrafficPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetTrafficPolicyError {
    fn from(err: CredentialsError) -> GetTrafficPolicyError {
        GetTrafficPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTrafficPolicyError {
    fn from(err: HttpDispatchError) -> GetTrafficPolicyError {
        GetTrafficPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTrafficPolicyError {
    fn from(err: io::Error) -> GetTrafficPolicyError {
        GetTrafficPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTrafficPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTrafficPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetTrafficPolicyError::InvalidInput(ref cause) => cause,
            GetTrafficPolicyError::NoSuchTrafficPolicy(ref cause) => cause,
            GetTrafficPolicyError::Validation(ref cause) => cause,
            GetTrafficPolicyError::Credentials(ref err) => err.description(),
            GetTrafficPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTrafficPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTrafficPolicyInstance
#[derive(Debug, PartialEq)]
pub enum GetTrafficPolicyInstanceError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy instance exists with the specified ID.</p>
    NoSuchTrafficPolicyInstance(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetTrafficPolicyInstanceError {
    pub fn from_body(body: &str) -> GetTrafficPolicyInstanceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    GetTrafficPolicyInstanceError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchTrafficPolicyInstance" => {
                    GetTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(String::from(
                        parsed_error.message,
                    ))
                }
                _ => GetTrafficPolicyInstanceError::Unknown(String::from(body)),
            },
            Err(_) => GetTrafficPolicyInstanceError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetTrafficPolicyInstanceError {
    fn from(err: XmlParseError) -> GetTrafficPolicyInstanceError {
        let XmlParseError(message) = err;
        GetTrafficPolicyInstanceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetTrafficPolicyInstanceError {
    fn from(err: CredentialsError) -> GetTrafficPolicyInstanceError {
        GetTrafficPolicyInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTrafficPolicyInstanceError {
    fn from(err: HttpDispatchError) -> GetTrafficPolicyInstanceError {
        GetTrafficPolicyInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTrafficPolicyInstanceError {
    fn from(err: io::Error) -> GetTrafficPolicyInstanceError {
        GetTrafficPolicyInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTrafficPolicyInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTrafficPolicyInstanceError {
    fn description(&self) -> &str {
        match *self {
            GetTrafficPolicyInstanceError::InvalidInput(ref cause) => cause,
            GetTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(ref cause) => cause,
            GetTrafficPolicyInstanceError::Validation(ref cause) => cause,
            GetTrafficPolicyInstanceError::Credentials(ref err) => err.description(),
            GetTrafficPolicyInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetTrafficPolicyInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTrafficPolicyInstanceCount
#[derive(Debug, PartialEq)]
pub enum GetTrafficPolicyInstanceCountError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetTrafficPolicyInstanceCountError {
    pub fn from_body(body: &str) -> GetTrafficPolicyInstanceCountError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetTrafficPolicyInstanceCountError::Unknown(String::from(body)),
            },
            Err(_) => GetTrafficPolicyInstanceCountError::Unknown(body.to_string()),
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

impl From<XmlParseError> for GetTrafficPolicyInstanceCountError {
    fn from(err: XmlParseError) -> GetTrafficPolicyInstanceCountError {
        let XmlParseError(message) = err;
        GetTrafficPolicyInstanceCountError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetTrafficPolicyInstanceCountError {
    fn from(err: CredentialsError) -> GetTrafficPolicyInstanceCountError {
        GetTrafficPolicyInstanceCountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTrafficPolicyInstanceCountError {
    fn from(err: HttpDispatchError) -> GetTrafficPolicyInstanceCountError {
        GetTrafficPolicyInstanceCountError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTrafficPolicyInstanceCountError {
    fn from(err: io::Error) -> GetTrafficPolicyInstanceCountError {
        GetTrafficPolicyInstanceCountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTrafficPolicyInstanceCountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTrafficPolicyInstanceCountError {
    fn description(&self) -> &str {
        match *self {
            GetTrafficPolicyInstanceCountError::Validation(ref cause) => cause,
            GetTrafficPolicyInstanceCountError::Credentials(ref err) => err.description(),
            GetTrafficPolicyInstanceCountError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetTrafficPolicyInstanceCountError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListGeoLocations
#[derive(Debug, PartialEq)]
pub enum ListGeoLocationsError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListGeoLocationsError {
    pub fn from_body(body: &str) -> ListGeoLocationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    ListGeoLocationsError::InvalidInput(String::from(parsed_error.message))
                }
                _ => ListGeoLocationsError::Unknown(String::from(body)),
            },
            Err(_) => ListGeoLocationsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListGeoLocationsError {
    fn from(err: XmlParseError) -> ListGeoLocationsError {
        let XmlParseError(message) = err;
        ListGeoLocationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListGeoLocationsError {
    fn from(err: CredentialsError) -> ListGeoLocationsError {
        ListGeoLocationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListGeoLocationsError {
    fn from(err: HttpDispatchError) -> ListGeoLocationsError {
        ListGeoLocationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListGeoLocationsError {
    fn from(err: io::Error) -> ListGeoLocationsError {
        ListGeoLocationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListGeoLocationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGeoLocationsError {
    fn description(&self) -> &str {
        match *self {
            ListGeoLocationsError::InvalidInput(ref cause) => cause,
            ListGeoLocationsError::Validation(ref cause) => cause,
            ListGeoLocationsError::Credentials(ref err) => err.description(),
            ListGeoLocationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListGeoLocationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListHealthChecks
#[derive(Debug, PartialEq)]
pub enum ListHealthChecksError {
    /// <p>The resource you're trying to access is unsupported on this Amazon Route 53 endpoint.</p>
    IncompatibleVersion(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListHealthChecksError {
    pub fn from_body(body: &str) -> ListHealthChecksError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "IncompatibleVersion" => {
                    ListHealthChecksError::IncompatibleVersion(String::from(parsed_error.message))
                }
                "InvalidInput" => {
                    ListHealthChecksError::InvalidInput(String::from(parsed_error.message))
                }
                _ => ListHealthChecksError::Unknown(String::from(body)),
            },
            Err(_) => ListHealthChecksError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListHealthChecksError {
    fn from(err: XmlParseError) -> ListHealthChecksError {
        let XmlParseError(message) = err;
        ListHealthChecksError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListHealthChecksError {
    fn from(err: CredentialsError) -> ListHealthChecksError {
        ListHealthChecksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListHealthChecksError {
    fn from(err: HttpDispatchError) -> ListHealthChecksError {
        ListHealthChecksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListHealthChecksError {
    fn from(err: io::Error) -> ListHealthChecksError {
        ListHealthChecksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListHealthChecksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListHealthChecksError {
    fn description(&self) -> &str {
        match *self {
            ListHealthChecksError::IncompatibleVersion(ref cause) => cause,
            ListHealthChecksError::InvalidInput(ref cause) => cause,
            ListHealthChecksError::Validation(ref cause) => cause,
            ListHealthChecksError::Credentials(ref err) => err.description(),
            ListHealthChecksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListHealthChecksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListHostedZones
#[derive(Debug, PartialEq)]
pub enum ListHostedZonesError {
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    DelegationSetNotReusable(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    NoSuchDelegationSet(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListHostedZonesError {
    pub fn from_body(body: &str) -> ListHostedZonesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "DelegationSetNotReusable" => ListHostedZonesError::DelegationSetNotReusable(
                    String::from(parsed_error.message),
                ),
                "InvalidInput" => {
                    ListHostedZonesError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchDelegationSet" => {
                    ListHostedZonesError::NoSuchDelegationSet(String::from(parsed_error.message))
                }
                _ => ListHostedZonesError::Unknown(String::from(body)),
            },
            Err(_) => ListHostedZonesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListHostedZonesError {
    fn from(err: XmlParseError) -> ListHostedZonesError {
        let XmlParseError(message) = err;
        ListHostedZonesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListHostedZonesError {
    fn from(err: CredentialsError) -> ListHostedZonesError {
        ListHostedZonesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListHostedZonesError {
    fn from(err: HttpDispatchError) -> ListHostedZonesError {
        ListHostedZonesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListHostedZonesError {
    fn from(err: io::Error) -> ListHostedZonesError {
        ListHostedZonesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListHostedZonesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListHostedZonesError {
    fn description(&self) -> &str {
        match *self {
            ListHostedZonesError::DelegationSetNotReusable(ref cause) => cause,
            ListHostedZonesError::InvalidInput(ref cause) => cause,
            ListHostedZonesError::NoSuchDelegationSet(ref cause) => cause,
            ListHostedZonesError::Validation(ref cause) => cause,
            ListHostedZonesError::Credentials(ref err) => err.description(),
            ListHostedZonesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListHostedZonesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListHostedZonesByName
#[derive(Debug, PartialEq)]
pub enum ListHostedZonesByNameError {
    /// <p>The specified domain name is not valid.</p>
    InvalidDomainName(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListHostedZonesByNameError {
    pub fn from_body(body: &str) -> ListHostedZonesByNameError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidDomainName" => ListHostedZonesByNameError::InvalidDomainName(String::from(
                    parsed_error.message,
                )),
                "InvalidInput" => {
                    ListHostedZonesByNameError::InvalidInput(String::from(parsed_error.message))
                }
                _ => ListHostedZonesByNameError::Unknown(String::from(body)),
            },
            Err(_) => ListHostedZonesByNameError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListHostedZonesByNameError {
    fn from(err: XmlParseError) -> ListHostedZonesByNameError {
        let XmlParseError(message) = err;
        ListHostedZonesByNameError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListHostedZonesByNameError {
    fn from(err: CredentialsError) -> ListHostedZonesByNameError {
        ListHostedZonesByNameError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListHostedZonesByNameError {
    fn from(err: HttpDispatchError) -> ListHostedZonesByNameError {
        ListHostedZonesByNameError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListHostedZonesByNameError {
    fn from(err: io::Error) -> ListHostedZonesByNameError {
        ListHostedZonesByNameError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListHostedZonesByNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListHostedZonesByNameError {
    fn description(&self) -> &str {
        match *self {
            ListHostedZonesByNameError::InvalidDomainName(ref cause) => cause,
            ListHostedZonesByNameError::InvalidInput(ref cause) => cause,
            ListHostedZonesByNameError::Validation(ref cause) => cause,
            ListHostedZonesByNameError::Credentials(ref err) => err.description(),
            ListHostedZonesByNameError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListHostedZonesByNameError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListQueryLoggingConfigs
#[derive(Debug, PartialEq)]
pub enum ListQueryLoggingConfigsError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>The value that you specified to get the second or subsequent page of results is invalid.</p>
    InvalidPaginationToken(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListQueryLoggingConfigsError {
    pub fn from_body(body: &str) -> ListQueryLoggingConfigsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    ListQueryLoggingConfigsError::InvalidInput(String::from(parsed_error.message))
                }
                "InvalidPaginationToken" => ListQueryLoggingConfigsError::InvalidPaginationToken(
                    String::from(parsed_error.message),
                ),
                "NoSuchHostedZone" => ListQueryLoggingConfigsError::NoSuchHostedZone(String::from(
                    parsed_error.message,
                )),
                _ => ListQueryLoggingConfigsError::Unknown(String::from(body)),
            },
            Err(_) => ListQueryLoggingConfigsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListQueryLoggingConfigsError {
    fn from(err: XmlParseError) -> ListQueryLoggingConfigsError {
        let XmlParseError(message) = err;
        ListQueryLoggingConfigsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListQueryLoggingConfigsError {
    fn from(err: CredentialsError) -> ListQueryLoggingConfigsError {
        ListQueryLoggingConfigsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListQueryLoggingConfigsError {
    fn from(err: HttpDispatchError) -> ListQueryLoggingConfigsError {
        ListQueryLoggingConfigsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListQueryLoggingConfigsError {
    fn from(err: io::Error) -> ListQueryLoggingConfigsError {
        ListQueryLoggingConfigsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListQueryLoggingConfigsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListQueryLoggingConfigsError {
    fn description(&self) -> &str {
        match *self {
            ListQueryLoggingConfigsError::InvalidInput(ref cause) => cause,
            ListQueryLoggingConfigsError::InvalidPaginationToken(ref cause) => cause,
            ListQueryLoggingConfigsError::NoSuchHostedZone(ref cause) => cause,
            ListQueryLoggingConfigsError::Validation(ref cause) => cause,
            ListQueryLoggingConfigsError::Credentials(ref err) => err.description(),
            ListQueryLoggingConfigsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListQueryLoggingConfigsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResourceRecordSets
#[derive(Debug, PartialEq)]
pub enum ListResourceRecordSetsError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListResourceRecordSetsError {
    pub fn from_body(body: &str) -> ListResourceRecordSetsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    ListResourceRecordSetsError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHostedZone" => ListResourceRecordSetsError::NoSuchHostedZone(String::from(
                    parsed_error.message,
                )),
                _ => ListResourceRecordSetsError::Unknown(String::from(body)),
            },
            Err(_) => ListResourceRecordSetsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListResourceRecordSetsError {
    fn from(err: XmlParseError) -> ListResourceRecordSetsError {
        let XmlParseError(message) = err;
        ListResourceRecordSetsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListResourceRecordSetsError {
    fn from(err: CredentialsError) -> ListResourceRecordSetsError {
        ListResourceRecordSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListResourceRecordSetsError {
    fn from(err: HttpDispatchError) -> ListResourceRecordSetsError {
        ListResourceRecordSetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListResourceRecordSetsError {
    fn from(err: io::Error) -> ListResourceRecordSetsError {
        ListResourceRecordSetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListResourceRecordSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceRecordSetsError {
    fn description(&self) -> &str {
        match *self {
            ListResourceRecordSetsError::InvalidInput(ref cause) => cause,
            ListResourceRecordSetsError::NoSuchHostedZone(ref cause) => cause,
            ListResourceRecordSetsError::Validation(ref cause) => cause,
            ListResourceRecordSetsError::Credentials(ref err) => err.description(),
            ListResourceRecordSetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListResourceRecordSetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListReusableDelegationSets
#[derive(Debug, PartialEq)]
pub enum ListReusableDelegationSetsError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListReusableDelegationSetsError {
    pub fn from_body(body: &str) -> ListReusableDelegationSetsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => ListReusableDelegationSetsError::InvalidInput(String::from(
                    parsed_error.message,
                )),
                _ => ListReusableDelegationSetsError::Unknown(String::from(body)),
            },
            Err(_) => ListReusableDelegationSetsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListReusableDelegationSetsError {
    fn from(err: XmlParseError) -> ListReusableDelegationSetsError {
        let XmlParseError(message) = err;
        ListReusableDelegationSetsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListReusableDelegationSetsError {
    fn from(err: CredentialsError) -> ListReusableDelegationSetsError {
        ListReusableDelegationSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListReusableDelegationSetsError {
    fn from(err: HttpDispatchError) -> ListReusableDelegationSetsError {
        ListReusableDelegationSetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListReusableDelegationSetsError {
    fn from(err: io::Error) -> ListReusableDelegationSetsError {
        ListReusableDelegationSetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListReusableDelegationSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListReusableDelegationSetsError {
    fn description(&self) -> &str {
        match *self {
            ListReusableDelegationSetsError::InvalidInput(ref cause) => cause,
            ListReusableDelegationSetsError::Validation(ref cause) => cause,
            ListReusableDelegationSetsError::Credentials(ref err) => err.description(),
            ListReusableDelegationSetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListReusableDelegationSetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
    NoSuchHealthCheck(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
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
                "InvalidInput" => {
                    ListTagsForResourceError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHealthCheck" => {
                    ListTagsForResourceError::NoSuchHealthCheck(String::from(parsed_error.message))
                }
                "NoSuchHostedZone" => {
                    ListTagsForResourceError::NoSuchHostedZone(String::from(parsed_error.message))
                }
                "PriorRequestNotComplete" => ListTagsForResourceError::PriorRequestNotComplete(
                    String::from(parsed_error.message),
                ),
                "ThrottlingException" => {
                    ListTagsForResourceError::Throttling(String::from(parsed_error.message))
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
            ListTagsForResourceError::InvalidInput(ref cause) => cause,
            ListTagsForResourceError::NoSuchHealthCheck(ref cause) => cause,
            ListTagsForResourceError::NoSuchHostedZone(ref cause) => cause,
            ListTagsForResourceError::PriorRequestNotComplete(ref cause) => cause,
            ListTagsForResourceError::Throttling(ref cause) => cause,
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResources
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourcesError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
    NoSuchHealthCheck(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsForResourcesError {
    pub fn from_body(body: &str) -> ListTagsForResourcesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    ListTagsForResourcesError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHealthCheck" => {
                    ListTagsForResourcesError::NoSuchHealthCheck(String::from(parsed_error.message))
                }
                "NoSuchHostedZone" => {
                    ListTagsForResourcesError::NoSuchHostedZone(String::from(parsed_error.message))
                }
                "PriorRequestNotComplete" => ListTagsForResourcesError::PriorRequestNotComplete(
                    String::from(parsed_error.message),
                ),
                "ThrottlingException" => {
                    ListTagsForResourcesError::Throttling(String::from(parsed_error.message))
                }
                _ => ListTagsForResourcesError::Unknown(String::from(body)),
            },
            Err(_) => ListTagsForResourcesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListTagsForResourcesError {
    fn from(err: XmlParseError) -> ListTagsForResourcesError {
        let XmlParseError(message) = err;
        ListTagsForResourcesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListTagsForResourcesError {
    fn from(err: CredentialsError) -> ListTagsForResourcesError {
        ListTagsForResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForResourcesError {
    fn from(err: HttpDispatchError) -> ListTagsForResourcesError {
        ListTagsForResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForResourcesError {
    fn from(err: io::Error) -> ListTagsForResourcesError {
        ListTagsForResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourcesError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourcesError::InvalidInput(ref cause) => cause,
            ListTagsForResourcesError::NoSuchHealthCheck(ref cause) => cause,
            ListTagsForResourcesError::NoSuchHostedZone(ref cause) => cause,
            ListTagsForResourcesError::PriorRequestNotComplete(ref cause) => cause,
            ListTagsForResourcesError::Throttling(ref cause) => cause,
            ListTagsForResourcesError::Validation(ref cause) => cause,
            ListTagsForResourcesError::Credentials(ref err) => err.description(),
            ListTagsForResourcesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourcesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTrafficPolicies
#[derive(Debug, PartialEq)]
pub enum ListTrafficPoliciesError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTrafficPoliciesError {
    pub fn from_body(body: &str) -> ListTrafficPoliciesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    ListTrafficPoliciesError::InvalidInput(String::from(parsed_error.message))
                }
                _ => ListTrafficPoliciesError::Unknown(String::from(body)),
            },
            Err(_) => ListTrafficPoliciesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListTrafficPoliciesError {
    fn from(err: XmlParseError) -> ListTrafficPoliciesError {
        let XmlParseError(message) = err;
        ListTrafficPoliciesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListTrafficPoliciesError {
    fn from(err: CredentialsError) -> ListTrafficPoliciesError {
        ListTrafficPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTrafficPoliciesError {
    fn from(err: HttpDispatchError) -> ListTrafficPoliciesError {
        ListTrafficPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTrafficPoliciesError {
    fn from(err: io::Error) -> ListTrafficPoliciesError {
        ListTrafficPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTrafficPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTrafficPoliciesError {
    fn description(&self) -> &str {
        match *self {
            ListTrafficPoliciesError::InvalidInput(ref cause) => cause,
            ListTrafficPoliciesError::Validation(ref cause) => cause,
            ListTrafficPoliciesError::Credentials(ref err) => err.description(),
            ListTrafficPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTrafficPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTrafficPolicyInstances
#[derive(Debug, PartialEq)]
pub enum ListTrafficPolicyInstancesError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy instance exists with the specified ID.</p>
    NoSuchTrafficPolicyInstance(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTrafficPolicyInstancesError {
    pub fn from_body(body: &str) -> ListTrafficPolicyInstancesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => ListTrafficPolicyInstancesError::InvalidInput(String::from(
                    parsed_error.message,
                )),
                "NoSuchTrafficPolicyInstance" => {
                    ListTrafficPolicyInstancesError::NoSuchTrafficPolicyInstance(String::from(
                        parsed_error.message,
                    ))
                }
                _ => ListTrafficPolicyInstancesError::Unknown(String::from(body)),
            },
            Err(_) => ListTrafficPolicyInstancesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListTrafficPolicyInstancesError {
    fn from(err: XmlParseError) -> ListTrafficPolicyInstancesError {
        let XmlParseError(message) = err;
        ListTrafficPolicyInstancesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListTrafficPolicyInstancesError {
    fn from(err: CredentialsError) -> ListTrafficPolicyInstancesError {
        ListTrafficPolicyInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTrafficPolicyInstancesError {
    fn from(err: HttpDispatchError) -> ListTrafficPolicyInstancesError {
        ListTrafficPolicyInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTrafficPolicyInstancesError {
    fn from(err: io::Error) -> ListTrafficPolicyInstancesError {
        ListTrafficPolicyInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTrafficPolicyInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTrafficPolicyInstancesError {
    fn description(&self) -> &str {
        match *self {
            ListTrafficPolicyInstancesError::InvalidInput(ref cause) => cause,
            ListTrafficPolicyInstancesError::NoSuchTrafficPolicyInstance(ref cause) => cause,
            ListTrafficPolicyInstancesError::Validation(ref cause) => cause,
            ListTrafficPolicyInstancesError::Credentials(ref err) => err.description(),
            ListTrafficPolicyInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTrafficPolicyInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTrafficPolicyInstancesByHostedZone
#[derive(Debug, PartialEq)]
pub enum ListTrafficPolicyInstancesByHostedZoneError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>No traffic policy instance exists with the specified ID.</p>
    NoSuchTrafficPolicyInstance(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTrafficPolicyInstancesByHostedZoneError {
    pub fn from_body(body: &str) -> ListTrafficPolicyInstancesByHostedZoneError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => ListTrafficPolicyInstancesByHostedZoneError::InvalidInput(
                    String::from(parsed_error.message),
                ),
                "NoSuchHostedZone" => {
                    ListTrafficPolicyInstancesByHostedZoneError::NoSuchHostedZone(String::from(
                        parsed_error.message,
                    ))
                }
                "NoSuchTrafficPolicyInstance" => {
                    ListTrafficPolicyInstancesByHostedZoneError::NoSuchTrafficPolicyInstance(
                        String::from(parsed_error.message),
                    )
                }
                _ => ListTrafficPolicyInstancesByHostedZoneError::Unknown(String::from(body)),
            },
            Err(_) => ListTrafficPolicyInstancesByHostedZoneError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListTrafficPolicyInstancesByHostedZoneError {
    fn from(err: XmlParseError) -> ListTrafficPolicyInstancesByHostedZoneError {
        let XmlParseError(message) = err;
        ListTrafficPolicyInstancesByHostedZoneError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListTrafficPolicyInstancesByHostedZoneError {
    fn from(err: CredentialsError) -> ListTrafficPolicyInstancesByHostedZoneError {
        ListTrafficPolicyInstancesByHostedZoneError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTrafficPolicyInstancesByHostedZoneError {
    fn from(err: HttpDispatchError) -> ListTrafficPolicyInstancesByHostedZoneError {
        ListTrafficPolicyInstancesByHostedZoneError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTrafficPolicyInstancesByHostedZoneError {
    fn from(err: io::Error) -> ListTrafficPolicyInstancesByHostedZoneError {
        ListTrafficPolicyInstancesByHostedZoneError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTrafficPolicyInstancesByHostedZoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTrafficPolicyInstancesByHostedZoneError {
    fn description(&self) -> &str {
        match *self {
            ListTrafficPolicyInstancesByHostedZoneError::InvalidInput(ref cause) => cause,
            ListTrafficPolicyInstancesByHostedZoneError::NoSuchHostedZone(ref cause) => cause,
            ListTrafficPolicyInstancesByHostedZoneError::NoSuchTrafficPolicyInstance(ref cause) => {
                cause
            }
            ListTrafficPolicyInstancesByHostedZoneError::Validation(ref cause) => cause,
            ListTrafficPolicyInstancesByHostedZoneError::Credentials(ref err) => err.description(),
            ListTrafficPolicyInstancesByHostedZoneError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTrafficPolicyInstancesByHostedZoneError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTrafficPolicyInstancesByPolicy
#[derive(Debug, PartialEq)]
pub enum ListTrafficPolicyInstancesByPolicyError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy exists with the specified ID.</p>
    NoSuchTrafficPolicy(String),
    /// <p>No traffic policy instance exists with the specified ID.</p>
    NoSuchTrafficPolicyInstance(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTrafficPolicyInstancesByPolicyError {
    pub fn from_body(body: &str) -> ListTrafficPolicyInstancesByPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => ListTrafficPolicyInstancesByPolicyError::InvalidInput(
                    String::from(parsed_error.message),
                ),
                "NoSuchTrafficPolicy" => {
                    ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicy(String::from(
                        parsed_error.message,
                    ))
                }
                "NoSuchTrafficPolicyInstance" => {
                    ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicyInstance(
                        String::from(parsed_error.message),
                    )
                }
                _ => ListTrafficPolicyInstancesByPolicyError::Unknown(String::from(body)),
            },
            Err(_) => ListTrafficPolicyInstancesByPolicyError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListTrafficPolicyInstancesByPolicyError {
    fn from(err: XmlParseError) -> ListTrafficPolicyInstancesByPolicyError {
        let XmlParseError(message) = err;
        ListTrafficPolicyInstancesByPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListTrafficPolicyInstancesByPolicyError {
    fn from(err: CredentialsError) -> ListTrafficPolicyInstancesByPolicyError {
        ListTrafficPolicyInstancesByPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTrafficPolicyInstancesByPolicyError {
    fn from(err: HttpDispatchError) -> ListTrafficPolicyInstancesByPolicyError {
        ListTrafficPolicyInstancesByPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTrafficPolicyInstancesByPolicyError {
    fn from(err: io::Error) -> ListTrafficPolicyInstancesByPolicyError {
        ListTrafficPolicyInstancesByPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTrafficPolicyInstancesByPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTrafficPolicyInstancesByPolicyError {
    fn description(&self) -> &str {
        match *self {
            ListTrafficPolicyInstancesByPolicyError::InvalidInput(ref cause) => cause,
            ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicy(ref cause) => cause,
            ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicyInstance(ref cause) => {
                cause
            }
            ListTrafficPolicyInstancesByPolicyError::Validation(ref cause) => cause,
            ListTrafficPolicyInstancesByPolicyError::Credentials(ref err) => err.description(),
            ListTrafficPolicyInstancesByPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTrafficPolicyInstancesByPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTrafficPolicyVersions
#[derive(Debug, PartialEq)]
pub enum ListTrafficPolicyVersionsError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy exists with the specified ID.</p>
    NoSuchTrafficPolicy(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTrafficPolicyVersionsError {
    pub fn from_body(body: &str) -> ListTrafficPolicyVersionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    ListTrafficPolicyVersionsError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchTrafficPolicy" => ListTrafficPolicyVersionsError::NoSuchTrafficPolicy(
                    String::from(parsed_error.message),
                ),
                _ => ListTrafficPolicyVersionsError::Unknown(String::from(body)),
            },
            Err(_) => ListTrafficPolicyVersionsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListTrafficPolicyVersionsError {
    fn from(err: XmlParseError) -> ListTrafficPolicyVersionsError {
        let XmlParseError(message) = err;
        ListTrafficPolicyVersionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListTrafficPolicyVersionsError {
    fn from(err: CredentialsError) -> ListTrafficPolicyVersionsError {
        ListTrafficPolicyVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTrafficPolicyVersionsError {
    fn from(err: HttpDispatchError) -> ListTrafficPolicyVersionsError {
        ListTrafficPolicyVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTrafficPolicyVersionsError {
    fn from(err: io::Error) -> ListTrafficPolicyVersionsError {
        ListTrafficPolicyVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTrafficPolicyVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTrafficPolicyVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListTrafficPolicyVersionsError::InvalidInput(ref cause) => cause,
            ListTrafficPolicyVersionsError::NoSuchTrafficPolicy(ref cause) => cause,
            ListTrafficPolicyVersionsError::Validation(ref cause) => cause,
            ListTrafficPolicyVersionsError::Credentials(ref err) => err.description(),
            ListTrafficPolicyVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTrafficPolicyVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListVPCAssociationAuthorizations
#[derive(Debug, PartialEq)]
pub enum ListVPCAssociationAuthorizationsError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>The value that you specified to get the second or subsequent page of results is invalid.</p>
    InvalidPaginationToken(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListVPCAssociationAuthorizationsError {
    pub fn from_body(body: &str) -> ListVPCAssociationAuthorizationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => ListVPCAssociationAuthorizationsError::InvalidInput(
                    String::from(parsed_error.message),
                ),
                "InvalidPaginationToken" => {
                    ListVPCAssociationAuthorizationsError::InvalidPaginationToken(String::from(
                        parsed_error.message,
                    ))
                }
                "NoSuchHostedZone" => ListVPCAssociationAuthorizationsError::NoSuchHostedZone(
                    String::from(parsed_error.message),
                ),
                _ => ListVPCAssociationAuthorizationsError::Unknown(String::from(body)),
            },
            Err(_) => ListVPCAssociationAuthorizationsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListVPCAssociationAuthorizationsError {
    fn from(err: XmlParseError) -> ListVPCAssociationAuthorizationsError {
        let XmlParseError(message) = err;
        ListVPCAssociationAuthorizationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListVPCAssociationAuthorizationsError {
    fn from(err: CredentialsError) -> ListVPCAssociationAuthorizationsError {
        ListVPCAssociationAuthorizationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListVPCAssociationAuthorizationsError {
    fn from(err: HttpDispatchError) -> ListVPCAssociationAuthorizationsError {
        ListVPCAssociationAuthorizationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListVPCAssociationAuthorizationsError {
    fn from(err: io::Error) -> ListVPCAssociationAuthorizationsError {
        ListVPCAssociationAuthorizationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListVPCAssociationAuthorizationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListVPCAssociationAuthorizationsError {
    fn description(&self) -> &str {
        match *self {
            ListVPCAssociationAuthorizationsError::InvalidInput(ref cause) => cause,
            ListVPCAssociationAuthorizationsError::InvalidPaginationToken(ref cause) => cause,
            ListVPCAssociationAuthorizationsError::NoSuchHostedZone(ref cause) => cause,
            ListVPCAssociationAuthorizationsError::Validation(ref cause) => cause,
            ListVPCAssociationAuthorizationsError::Credentials(ref err) => err.description(),
            ListVPCAssociationAuthorizationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListVPCAssociationAuthorizationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TestDNSAnswer
#[derive(Debug, PartialEq)]
pub enum TestDNSAnswerError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TestDNSAnswerError {
    pub fn from_body(body: &str) -> TestDNSAnswerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    TestDNSAnswerError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHostedZone" => {
                    TestDNSAnswerError::NoSuchHostedZone(String::from(parsed_error.message))
                }
                _ => TestDNSAnswerError::Unknown(String::from(body)),
            },
            Err(_) => TestDNSAnswerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for TestDNSAnswerError {
    fn from(err: XmlParseError) -> TestDNSAnswerError {
        let XmlParseError(message) = err;
        TestDNSAnswerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for TestDNSAnswerError {
    fn from(err: CredentialsError) -> TestDNSAnswerError {
        TestDNSAnswerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TestDNSAnswerError {
    fn from(err: HttpDispatchError) -> TestDNSAnswerError {
        TestDNSAnswerError::HttpDispatch(err)
    }
}
impl From<io::Error> for TestDNSAnswerError {
    fn from(err: io::Error) -> TestDNSAnswerError {
        TestDNSAnswerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TestDNSAnswerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestDNSAnswerError {
    fn description(&self) -> &str {
        match *self {
            TestDNSAnswerError::InvalidInput(ref cause) => cause,
            TestDNSAnswerError::NoSuchHostedZone(ref cause) => cause,
            TestDNSAnswerError::Validation(ref cause) => cause,
            TestDNSAnswerError::Credentials(ref err) => err.description(),
            TestDNSAnswerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TestDNSAnswerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateHealthCheck
#[derive(Debug, PartialEq)]
pub enum UpdateHealthCheckError {
    /// <p>The value of <code>HealthCheckVersion</code> in the request doesn't match the value of <code>HealthCheckVersion</code> in the health check.</p>
    HealthCheckVersionMismatch(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
    NoSuchHealthCheck(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateHealthCheckError {
    pub fn from_body(body: &str) -> UpdateHealthCheckError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "HealthCheckVersionMismatch" => UpdateHealthCheckError::HealthCheckVersionMismatch(
                    String::from(parsed_error.message),
                ),
                "InvalidInput" => {
                    UpdateHealthCheckError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHealthCheck" => {
                    UpdateHealthCheckError::NoSuchHealthCheck(String::from(parsed_error.message))
                }
                _ => UpdateHealthCheckError::Unknown(String::from(body)),
            },
            Err(_) => UpdateHealthCheckError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateHealthCheckError {
    fn from(err: XmlParseError) -> UpdateHealthCheckError {
        let XmlParseError(message) = err;
        UpdateHealthCheckError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateHealthCheckError {
    fn from(err: CredentialsError) -> UpdateHealthCheckError {
        UpdateHealthCheckError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateHealthCheckError {
    fn from(err: HttpDispatchError) -> UpdateHealthCheckError {
        UpdateHealthCheckError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateHealthCheckError {
    fn from(err: io::Error) -> UpdateHealthCheckError {
        UpdateHealthCheckError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateHealthCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateHealthCheckError {
    fn description(&self) -> &str {
        match *self {
            UpdateHealthCheckError::HealthCheckVersionMismatch(ref cause) => cause,
            UpdateHealthCheckError::InvalidInput(ref cause) => cause,
            UpdateHealthCheckError::NoSuchHealthCheck(ref cause) => cause,
            UpdateHealthCheckError::Validation(ref cause) => cause,
            UpdateHealthCheckError::Credentials(ref err) => err.description(),
            UpdateHealthCheckError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateHealthCheckError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateHostedZoneComment
#[derive(Debug, PartialEq)]
pub enum UpdateHostedZoneCommentError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateHostedZoneCommentError {
    pub fn from_body(body: &str) -> UpdateHostedZoneCommentError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidInput" => {
                    UpdateHostedZoneCommentError::InvalidInput(String::from(parsed_error.message))
                }
                "NoSuchHostedZone" => UpdateHostedZoneCommentError::NoSuchHostedZone(String::from(
                    parsed_error.message,
                )),
                _ => UpdateHostedZoneCommentError::Unknown(String::from(body)),
            },
            Err(_) => UpdateHostedZoneCommentError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateHostedZoneCommentError {
    fn from(err: XmlParseError) -> UpdateHostedZoneCommentError {
        let XmlParseError(message) = err;
        UpdateHostedZoneCommentError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateHostedZoneCommentError {
    fn from(err: CredentialsError) -> UpdateHostedZoneCommentError {
        UpdateHostedZoneCommentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateHostedZoneCommentError {
    fn from(err: HttpDispatchError) -> UpdateHostedZoneCommentError {
        UpdateHostedZoneCommentError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateHostedZoneCommentError {
    fn from(err: io::Error) -> UpdateHostedZoneCommentError {
        UpdateHostedZoneCommentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateHostedZoneCommentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateHostedZoneCommentError {
    fn description(&self) -> &str {
        match *self {
            UpdateHostedZoneCommentError::InvalidInput(ref cause) => cause,
            UpdateHostedZoneCommentError::NoSuchHostedZone(ref cause) => cause,
            UpdateHostedZoneCommentError::Validation(ref cause) => cause,
            UpdateHostedZoneCommentError::Credentials(ref err) => err.description(),
            UpdateHostedZoneCommentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateHostedZoneCommentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTrafficPolicyComment
#[derive(Debug, PartialEq)]
pub enum UpdateTrafficPolicyCommentError {
    /// <p>Another user submitted a request to create, update, or delete the object at the same time that you did. Retry the request. </p>
    ConcurrentModification(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy exists with the specified ID.</p>
    NoSuchTrafficPolicy(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateTrafficPolicyCommentError {
    pub fn from_body(body: &str) -> UpdateTrafficPolicyCommentError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConcurrentModification" => {
                    UpdateTrafficPolicyCommentError::ConcurrentModification(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidInput" => UpdateTrafficPolicyCommentError::InvalidInput(String::from(
                    parsed_error.message,
                )),
                "NoSuchTrafficPolicy" => UpdateTrafficPolicyCommentError::NoSuchTrafficPolicy(
                    String::from(parsed_error.message),
                ),
                _ => UpdateTrafficPolicyCommentError::Unknown(String::from(body)),
            },
            Err(_) => UpdateTrafficPolicyCommentError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateTrafficPolicyCommentError {
    fn from(err: XmlParseError) -> UpdateTrafficPolicyCommentError {
        let XmlParseError(message) = err;
        UpdateTrafficPolicyCommentError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateTrafficPolicyCommentError {
    fn from(err: CredentialsError) -> UpdateTrafficPolicyCommentError {
        UpdateTrafficPolicyCommentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTrafficPolicyCommentError {
    fn from(err: HttpDispatchError) -> UpdateTrafficPolicyCommentError {
        UpdateTrafficPolicyCommentError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTrafficPolicyCommentError {
    fn from(err: io::Error) -> UpdateTrafficPolicyCommentError {
        UpdateTrafficPolicyCommentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateTrafficPolicyCommentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTrafficPolicyCommentError {
    fn description(&self) -> &str {
        match *self {
            UpdateTrafficPolicyCommentError::ConcurrentModification(ref cause) => cause,
            UpdateTrafficPolicyCommentError::InvalidInput(ref cause) => cause,
            UpdateTrafficPolicyCommentError::NoSuchTrafficPolicy(ref cause) => cause,
            UpdateTrafficPolicyCommentError::Validation(ref cause) => cause,
            UpdateTrafficPolicyCommentError::Credentials(ref err) => err.description(),
            UpdateTrafficPolicyCommentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateTrafficPolicyCommentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTrafficPolicyInstance
#[derive(Debug, PartialEq)]
pub enum UpdateTrafficPolicyInstanceError {
    /// <p>You tried to update a traffic policy instance by using a traffic policy version that has a different DNS type than the current type for the instance. You specified the type in the JSON document in the <code>CreateTrafficPolicy</code> or <code>CreateTrafficPolicyVersion</code>request. </p>
    ConflictingTypes(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy exists with the specified ID.</p>
    NoSuchTrafficPolicy(String),
    /// <p>No traffic policy instance exists with the specified ID.</p>
    NoSuchTrafficPolicyInstance(String),
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateTrafficPolicyInstanceError {
    pub fn from_body(body: &str) -> UpdateTrafficPolicyInstanceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ConflictingTypes" => UpdateTrafficPolicyInstanceError::ConflictingTypes(
                    String::from(parsed_error.message),
                ),
                "InvalidInput" => UpdateTrafficPolicyInstanceError::InvalidInput(String::from(
                    parsed_error.message,
                )),
                "NoSuchTrafficPolicy" => UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicy(
                    String::from(parsed_error.message),
                ),
                "NoSuchTrafficPolicyInstance" => {
                    UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(String::from(
                        parsed_error.message,
                    ))
                }
                "PriorRequestNotComplete" => {
                    UpdateTrafficPolicyInstanceError::PriorRequestNotComplete(String::from(
                        parsed_error.message,
                    ))
                }
                _ => UpdateTrafficPolicyInstanceError::Unknown(String::from(body)),
            },
            Err(_) => UpdateTrafficPolicyInstanceError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateTrafficPolicyInstanceError {
    fn from(err: XmlParseError) -> UpdateTrafficPolicyInstanceError {
        let XmlParseError(message) = err;
        UpdateTrafficPolicyInstanceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateTrafficPolicyInstanceError {
    fn from(err: CredentialsError) -> UpdateTrafficPolicyInstanceError {
        UpdateTrafficPolicyInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTrafficPolicyInstanceError {
    fn from(err: HttpDispatchError) -> UpdateTrafficPolicyInstanceError {
        UpdateTrafficPolicyInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTrafficPolicyInstanceError {
    fn from(err: io::Error) -> UpdateTrafficPolicyInstanceError {
        UpdateTrafficPolicyInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateTrafficPolicyInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTrafficPolicyInstanceError {
    fn description(&self) -> &str {
        match *self {
            UpdateTrafficPolicyInstanceError::ConflictingTypes(ref cause) => cause,
            UpdateTrafficPolicyInstanceError::InvalidInput(ref cause) => cause,
            UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicy(ref cause) => cause,
            UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(ref cause) => cause,
            UpdateTrafficPolicyInstanceError::PriorRequestNotComplete(ref cause) => cause,
            UpdateTrafficPolicyInstanceError::Validation(ref cause) => cause,
            UpdateTrafficPolicyInstanceError::Credentials(ref err) => err.description(),
            UpdateTrafficPolicyInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateTrafficPolicyInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Route 53 API. Route 53 clients implement this trait.
pub trait Route53 {
    /// <p><p>Associates an Amazon VPC with a private hosted zone. </p> <important> <p>To perform the association, the VPC and the private hosted zone must already exist. You can&#39;t convert a public hosted zone into a private hosted zone.</p> </important> <note> <p>If you want to associate a VPC that was created by using one AWS account with a private hosted zone that was created by using a different account, the AWS account that created the private hosted zone must first submit a <code>CreateVPCAssociationAuthorization</code> request. Then the account that created the VPC must submit an <code>AssociateVPCWithHostedZone</code> request.</p> </note></p>
    fn associate_vpc_with_hosted_zone(
        &self,
        input: AssociateVPCWithHostedZoneRequest,
    ) -> RusotoFuture<AssociateVPCWithHostedZoneResponse, AssociateVPCWithHostedZoneError>;

    /// <p>Creates, changes, or deletes a resource record set, which contains authoritative DNS information for a specified domain name or subdomain name. For example, you can use <code>ChangeResourceRecordSets</code> to create a resource record set that routes traffic for test.example.com to a web server that has an IP address of 192.0.2.44.</p> <p> <b>Change Batches and Transactional Changes</b> </p> <p>The request body must include a document with a <code>ChangeResourceRecordSetsRequest</code> element. The request body contains a list of change items, known as a change batch. Change batches are considered transactional changes. When using the Amazon Route 53 API to change resource record sets, Amazon Route 53 either makes all or none of the changes in a change batch request. This ensures that Amazon Route 53 never partially implements the intended changes to the resource record sets in a hosted zone. </p> <p>For example, a change batch request that deletes the <code>CNAME</code> record for www.example.com and creates an alias resource record set for www.example.com. Amazon Route 53 deletes the first resource record set and creates the second resource record set in a single operation. If either the <code>DELETE</code> or the <code>CREATE</code> action fails, then both changes (plus any other changes in the batch) fail, and the original <code>CNAME</code> record continues to exist.</p> <important> <p>Due to the nature of transactional changes, you can't delete the same resource record set more than once in a single change batch. If you attempt to delete the same change batch more than once, Amazon Route 53 returns an <code>InvalidChangeBatch</code> error.</p> </important> <p> <b>Traffic Flow</b> </p> <p>To create resource record sets for complex routing configurations, use either the traffic flow visual editor in the Amazon Route 53 console or the API actions for traffic policies and traffic policy instances. Save the configuration as a traffic policy, then associate the traffic policy with one or more domain names (such as example.com) or subdomain names (such as www.example.com), in the same hosted zone or in multiple hosted zones. You can roll back the updates if the new configuration isn't performing as expected. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/traffic-flow.html">Using Traffic Flow to Route DNS Traffic</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p> <b>Create, Delete, and Upsert</b> </p> <p>Use <code>ChangeResourceRecordsSetsRequest</code> to perform the following actions:</p> <ul> <li> <p> <code>CREATE</code>: Creates a resource record set that has the specified values.</p> </li> <li> <p> <code>DELETE</code>: Deletes an existing resource record set that has the specified values.</p> </li> <li> <p> <code>UPSERT</code>: If a resource record set does not already exist, AWS creates it. If a resource set does exist, Amazon Route 53 updates it with the values in the request. </p> </li> </ul> <p> <b>Syntaxes for Creating, Updating, and Deleting Resource Record Sets</b> </p> <p>The syntax for a request depends on the type of resource record set that you want to create, delete, or update, such as weighted, alias, or failover. The XML elements in your request must appear in the order listed in the syntax. </p> <p>For an example for each type of resource record set, see "Examples."</p> <p>Don't refer to the syntax in the "Parameter Syntax" section, which includes all of the elements for every kind of resource record set that you can create, delete, or update by using <code>ChangeResourceRecordSets</code>. </p> <p> <b>Change Propagation to Amazon Route 53 DNS Servers</b> </p> <p>When you submit a <code>ChangeResourceRecordSets</code> request, Amazon Route 53 propagates your changes to all of the Amazon Route 53 authoritative DNS servers. While your changes are propagating, <code>GetChange</code> returns a status of <code>PENDING</code>. When propagation is complete, <code>GetChange</code> returns a status of <code>INSYNC</code>. Changes generally propagate to all Amazon Route 53 name servers within 60 seconds. For more information, see <a>GetChange</a>.</p> <p> <b>Limits on ChangeResourceRecordSets Requests</b> </p> <p>For information about the limits on a <code>ChangeResourceRecordSets</code> request, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    fn change_resource_record_sets(
        &self,
        input: ChangeResourceRecordSetsRequest,
    ) -> RusotoFuture<ChangeResourceRecordSetsResponse, ChangeResourceRecordSetsError>;

    /// <p>Adds, edits, or deletes tags for a health check or a hosted zone.</p> <p>For information about using tags for cost allocation, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    fn change_tags_for_resource(
        &self,
        input: ChangeTagsForResourceRequest,
    ) -> RusotoFuture<ChangeTagsForResourceResponse, ChangeTagsForResourceError>;

    /// <p><p>Creates a new health check.</p> <p>For information about adding health checks to resource record sets, see <a>ResourceRecordSet$HealthCheckId</a> in <a>ChangeResourceRecordSets</a>. </p> <p> <b>ELB Load Balancers</b> </p> <p>If you&#39;re registering EC2 instances with an Elastic Load Balancing (ELB) load balancer, do not create Amazon Route 53 health checks for the EC2 instances. When you register an EC2 instance with a load balancer, you configure settings for an ELB health check, which performs a similar function to an Amazon Route 53 health check.</p> <p> <b>Private Hosted Zones</b> </p> <p>You can associate health checks with failover resource record sets in a private hosted zone. Note the following:</p> <ul> <li> <p>Amazon Route 53 health checkers are outside the VPC. To check the health of an endpoint within a VPC by IP address, you must assign a public IP address to the instance in the VPC.</p> </li> <li> <p>You can configure a health checker to check the health of an external resource that the instance relies on, such as a database server.</p> </li> <li> <p>You can create a CloudWatch metric, associate an alarm with the metric, and then create a health check that is based on the state of the alarm. For example, you might create a CloudWatch metric that checks the status of the Amazon EC2 <code>StatusCheckFailed</code> metric, add an alarm to the metric, and then create a health check that is based on the state of the alarm. For information about creating CloudWatch metrics and alarms by using the CloudWatch console, see the <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/WhatIsCloudWatch.html">Amazon CloudWatch User Guide</a>.</p> </li> </ul></p>
    fn create_health_check(
        &self,
        input: CreateHealthCheckRequest,
    ) -> RusotoFuture<CreateHealthCheckResponse, CreateHealthCheckError>;

    /// <p>Creates a new public hosted zone, which you use to specify how the Domain Name System (DNS) routes traffic on the Internet for a domain, such as example.com, and its subdomains. </p> <important> <p>You can't convert a public hosted zones to a private hosted zone or vice versa. Instead, you must create a new hosted zone with the same name and create new resource record sets.</p> </important> <p>For more information about charges for hosted zones, see <a href="http://aws.amazon.com/route53/pricing/">Amazon Route 53 Pricing</a>.</p> <p>Note the following:</p> <ul> <li> <p>You can't create a hosted zone for a top-level domain (TLD).</p> </li> <li> <p>Amazon Route 53 automatically creates a default SOA record and four NS records for the zone. For more information about SOA and NS records, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/SOA-NSrecords.html">NS and SOA Records that Amazon Route 53 Creates for a Hosted Zone</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If you want to use the same name servers for multiple hosted zones, you can optionally associate a reusable delegation set with the hosted zone. See the <code>DelegationSetId</code> element.</p> </li> <li> <p>If your domain is registered with a registrar other than Amazon Route 53, you must update the name servers with your registrar to make Amazon Route 53 your DNS service. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/creating-migrating.html">Configuring Amazon Route 53 as your DNS Service</a> in the <i>Amazon Route 53 Developer Guide</i>. </p> </li> </ul> <p>When you submit a <code>CreateHostedZone</code> request, the initial status of the hosted zone is <code>PENDING</code>. This means that the NS and SOA records are not yet available on all Amazon Route 53 DNS servers. When the NS and SOA records are available, the status of the zone changes to <code>INSYNC</code>.</p>
    fn create_hosted_zone(
        &self,
        input: CreateHostedZoneRequest,
    ) -> RusotoFuture<CreateHostedZoneResponse, CreateHostedZoneError>;

    /// <p><p>Creates a configuration for DNS query logging. After you create a query logging configuration, Amazon Route 53 begins to publish log data to an Amazon CloudWatch Logs log group.</p> <p>DNS query logs contain information about the queries that Amazon Route 53 receives for a specified public hosted zone, such as the following:</p> <ul> <li> <p>Amazon Route 53 edge location that responded to the DNS query</p> </li> <li> <p>Domain or subdomain that was requested</p> </li> <li> <p>DNS record type, such as A or AAAA</p> </li> <li> <p>DNS response code, such as <code>NoError</code> or <code>ServFail</code> </p> </li> </ul> <dl> <dt>Log Group and Resource Policy</dt> <dd> <p>Before you create a query logging configuration, perform the following operations.</p> <note> <p>If you create a query logging configuration using the Amazon Route 53 console, Amazon Route 53 performs these operations automatically.</p> </note> <ol> <li> <p>Create a CloudWatch Logs log group, and make note of the ARN, which you specify when you create a query logging configuration. Note the following:</p> <ul> <li> <p>You must create the log group in the us-east-1 region.</p> </li> <li> <p>You must use the same AWS account to create the log group and the hosted zone that you want to configure query logging for.</p> </li> <li> <p>When you create log groups for query logging, we recommend that you use a consistent prefix, for example:</p> <p> <code>/aws/route53/<i>hosted zone name</i> </code> </p> <p>In the next step, you&#39;ll create a resource policy, which controls access to one or more log groups and the associated AWS resources, such as Amazon Route 53 hosted zones. There&#39;s a limit on the number of resource policies that you can create, so we recommend that you use a consistent prefix so you can use the same resource policy for all the log groups that you create for query logging.</p> </li> </ul> </li> <li> <p>Create a CloudWatch Logs resource policy, and give it the permissions that Amazon Route 53 needs to create log streams and to send query logs to log streams. For the value of <code>Resource</code>, specify the ARN for the log group that you created in the previous step. To use the same resource policy for all the CloudWatch Logs log groups that you created for query logging configurations, replace the hosted zone name with <code><em></code>, for example:</p> <p> <code>arn:aws:logs:us-east-1:123412341234:log-group:/aws/route53/</em></code> </p> <note> <p>You can&#39;t use the CloudWatch console to create or edit a resource policy. You must use the CloudWatch API, one of the AWS SDKs, or the AWS CLI.</p> </note> </li> </ol> </dd> <dt>Log Streams and Edge Locations</dt> <dd> <p>When Amazon Route 53 finishes creating the configuration for DNS query logging, it does the following:</p> <ul> <li> <p>Creates a log stream for an edge location the first time that the edge location responds to DNS queries for the specified hosted zone. That log stream is used to log all queries that Amazon Route 53 responds to for that edge location.</p> </li> <li> <p>Begins to send query logs to the applicable log stream.</p> </li> </ul> <p>The name of each log stream is in the following format:</p> <p> <code> <i>hosted zone ID</i>/<i>edge location code</i> </code> </p> <p>The edge location code is a three-letter code and an arbitrarily assigned number, for example, DFW3. The three-letter code typically corresponds with the International Air Transport Association airport code for an airport near the edge location. (These abbreviations might change in the future.) For a list of edge locations, see &quot;The Amazon Route 53 Global Network&quot; on the <a href="http://aws.amazon.com/route53/details/">Amazon Route 53 Product Details</a> page.</p> </dd> <dt>Queries That Are Logged</dt> <dd> <p>Query logs contain only the queries that DNS resolvers forward to Amazon Route 53. If a DNS resolver has already cached the response to a query (such as the IP address for a load balancer for example.com), the resolver will continue to return the cached response. It doesn&#39;t forward another query to Amazon Route 53 until the TTL for the corresponding resource record set expires. Depending on how many DNS queries are submitted for a resource record set, and depending on the TTL for that resource record set, query logs might contain information about only one query out of every several thousand queries that are submitted to DNS. For more information about how DNS works, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/welcome-dns-service.html">Routing Internet Traffic to Your Website or Web Application</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </dd> <dt>Log File Format</dt> <dd> <p>For a list of the values in each query log and the format of each value, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html">Logging DNS Queries</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </dd> <dt>Pricing</dt> <dd> <p>For information about charges for query logs, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </dd> <dt>How to Stop Logging</dt> <dd> <p>If you want Amazon Route 53 to stop sending query logs to CloudWatch Logs, delete the query logging configuration. For more information, see <a>DeleteQueryLoggingConfig</a>.</p> </dd> </dl></p>
    fn create_query_logging_config(
        &self,
        input: CreateQueryLoggingConfigRequest,
    ) -> RusotoFuture<CreateQueryLoggingConfigResponse, CreateQueryLoggingConfigError>;

    /// <p><p>Creates a delegation set (a group of four name servers) that can be reused by multiple hosted zones. If a hosted zoned ID is specified, <code>CreateReusableDelegationSet</code> marks the delegation set associated with that zone as reusable.</p> <note> <p>You can&#39;t associate a reusable delegation set with a private hosted zone.</p> </note> <p>For information about using a reusable delegation set to configure white label name servers, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/white-label-name-servers.html">Configuring White Label Name Servers</a>.</p> <p>The process for migrating existing hosted zones to use a reusable delegation set is comparable to the process for configuring white label name servers. You need to perform the following steps:</p> <ol> <li> <p>Create a reusable delegation set.</p> </li> <li> <p>Recreate hosted zones, and reduce the TTL to 60 seconds or less.</p> </li> <li> <p>Recreate resource record sets in the new hosted zones.</p> </li> <li> <p>Change the registrar&#39;s name servers to use the name servers for the new hosted zones.</p> </li> <li> <p>Monitor traffic for the website or application.</p> </li> <li> <p>Change TTLs back to their original values.</p> </li> </ol> <p>If you want to migrate existing hosted zones to use a reusable delegation set, the existing hosted zones can&#39;t use any of the name servers that are assigned to the reusable delegation set. If one or more hosted zones do use one or more name servers that are assigned to the reusable delegation set, you can do one of the following:</p> <ul> <li> <p>For small numbers of hosted zones—up to a few hundred—it&#39;s relatively easy to create reusable delegation sets until you get one that has four name servers that don&#39;t overlap with any of the name servers in your hosted zones.</p> </li> <li> <p>For larger numbers of hosted zones, the easiest solution is to use more than one reusable delegation set.</p> </li> <li> <p>For larger numbers of hosted zones, you can also migrate hosted zones that have overlapping name servers to hosted zones that don&#39;t have overlapping name servers, then migrate the hosted zones again to use the reusable delegation set.</p> </li> </ul></p>
    fn create_reusable_delegation_set(
        &self,
        input: CreateReusableDelegationSetRequest,
    ) -> RusotoFuture<CreateReusableDelegationSetResponse, CreateReusableDelegationSetError>;

    /// <p>Creates a traffic policy, which you use to create multiple DNS resource record sets for one domain name (such as example.com) or one subdomain name (such as www.example.com).</p>
    fn create_traffic_policy(
        &self,
        input: CreateTrafficPolicyRequest,
    ) -> RusotoFuture<CreateTrafficPolicyResponse, CreateTrafficPolicyError>;

    /// <p>Creates resource record sets in a specified hosted zone based on the settings in a specified traffic policy version. In addition, <code>CreateTrafficPolicyInstance</code> associates the resource record sets with a specified domain name (such as example.com) or subdomain name (such as www.example.com). Amazon Route 53 responds to DNS queries for the domain or subdomain name by using the resource record sets that <code>CreateTrafficPolicyInstance</code> created.</p>
    fn create_traffic_policy_instance(
        &self,
        input: CreateTrafficPolicyInstanceRequest,
    ) -> RusotoFuture<CreateTrafficPolicyInstanceResponse, CreateTrafficPolicyInstanceError>;

    /// <p>Creates a new version of an existing traffic policy. When you create a new version of a traffic policy, you specify the ID of the traffic policy that you want to update and a JSON-formatted document that describes the new version. You use traffic policies to create multiple DNS resource record sets for one domain name (such as example.com) or one subdomain name (such as www.example.com). You can create a maximum of 1000 versions of a traffic policy. If you reach the limit and need to create another version, you'll need to start a new traffic policy.</p>
    fn create_traffic_policy_version(
        &self,
        input: CreateTrafficPolicyVersionRequest,
    ) -> RusotoFuture<CreateTrafficPolicyVersionResponse, CreateTrafficPolicyVersionError>;

    /// <p><p>Authorizes the AWS account that created a specified VPC to submit an <code>AssociateVPCWithHostedZone</code> request to associate the VPC with a specified hosted zone that was created by a different account. To submit a <code>CreateVPCAssociationAuthorization</code> request, you must use the account that created the hosted zone. After you authorize the association, use the account that created the VPC to submit an <code>AssociateVPCWithHostedZone</code> request.</p> <note> <p>If you want to associate multiple VPCs that you created by using one account with a hosted zone that you created by using a different account, you must submit one authorization request for each VPC.</p> </note></p>
    fn create_vpc_association_authorization(
        &self,
        input: CreateVPCAssociationAuthorizationRequest,
    ) -> RusotoFuture<
        CreateVPCAssociationAuthorizationResponse,
        CreateVPCAssociationAuthorizationError,
    >;

    /// <p><p>Deletes a health check.</p> <important> <p>Amazon Route 53 does not prevent you from deleting a health check even if the health check is associated with one or more resource record sets. If you delete a health check and you don&#39;t update the associated resource record sets, the future status of the health check can&#39;t be predicted and may change. This will affect the routing of DNS queries for your DNS failover configuration. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/health-checks-creating-deleting.html#health-checks-deleting.html">Replacing and Deleting Health Checks</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </important></p>
    fn delete_health_check(
        &self,
        input: DeleteHealthCheckRequest,
    ) -> RusotoFuture<DeleteHealthCheckResponse, DeleteHealthCheckError>;

    /// <p><p>Deletes a hosted zone.</p> <important> <p>If the name servers for the hosted zone are associated with a domain and if you want to make the domain unavailable on the Internet, we recommend that you delete the name servers from the domain to prevent future DNS queries from possibly being misrouted. If the domain is registered with Amazon Route 53, see <code>UpdateDomainNameservers</code>. If the domain is registered with another registrar, use the method provided by the registrar to delete name servers for the domain.</p> <p>Some domain registries don&#39;t allow you to remove all of the name servers for a domain. If the registry for your domain requires one or more name servers, we recommend that you delete the hosted zone only if you transfer DNS service to another service provider, and you replace the name servers for the domain with name servers from the new provider.</p> </important> <p>You can delete a hosted zone only if it contains only the default SOA record and NS resource record sets. If the hosted zone contains other resource record sets, you must delete them before you can delete the hosted zone. If you try to delete a hosted zone that contains other resource record sets, the request fails, and Amazon Route 53 returns a <code>HostedZoneNotEmpty</code> error. For information about deleting records from your hosted zone, see <a>ChangeResourceRecordSets</a>.</p> <p>To verify that the hosted zone has been deleted, do one of the following:</p> <ul> <li> <p>Use the <code>GetHostedZone</code> action to request information about the hosted zone.</p> </li> <li> <p>Use the <code>ListHostedZones</code> action to get a list of the hosted zones associated with the current AWS account.</p> </li> </ul></p>
    fn delete_hosted_zone(
        &self,
        input: DeleteHostedZoneRequest,
    ) -> RusotoFuture<DeleteHostedZoneResponse, DeleteHostedZoneError>;

    /// <p>Deletes a configuration for DNS query logging. If you delete a configuration, Amazon Route 53 stops sending query logs to CloudWatch Logs. Amazon Route 53 doesn't delete any logs that are already in CloudWatch Logs.</p> <p>For more information about DNS query logs, see <a>CreateQueryLoggingConfig</a>.</p>
    fn delete_query_logging_config(
        &self,
        input: DeleteQueryLoggingConfigRequest,
    ) -> RusotoFuture<DeleteQueryLoggingConfigResponse, DeleteQueryLoggingConfigError>;

    /// <p>Deletes a reusable delegation set.</p> <important> <p>You can delete a reusable delegation set only if it isn't associated with any hosted zones.</p> </important> <p>To verify that the reusable delegation set is not associated with any hosted zones, submit a <a>GetReusableDelegationSet</a> request and specify the ID of the reusable delegation set that you want to delete.</p>
    fn delete_reusable_delegation_set(
        &self,
        input: DeleteReusableDelegationSetRequest,
    ) -> RusotoFuture<DeleteReusableDelegationSetResponse, DeleteReusableDelegationSetError>;

    /// <p>Deletes a traffic policy.</p>
    fn delete_traffic_policy(
        &self,
        input: DeleteTrafficPolicyRequest,
    ) -> RusotoFuture<DeleteTrafficPolicyResponse, DeleteTrafficPolicyError>;

    /// <p><p>Deletes a traffic policy instance and all of the resource record sets that Amazon Route 53 created when you created the instance.</p> <note> <p>In the Amazon Route 53 console, traffic policy instances are known as policy records.</p> </note></p>
    fn delete_traffic_policy_instance(
        &self,
        input: DeleteTrafficPolicyInstanceRequest,
    ) -> RusotoFuture<DeleteTrafficPolicyInstanceResponse, DeleteTrafficPolicyInstanceError>;

    /// <p><p>Removes authorization to submit an <code>AssociateVPCWithHostedZone</code> request to associate a specified VPC with a hosted zone that was created by a different account. You must use the account that created the hosted zone to submit a <code>DeleteVPCAssociationAuthorization</code> request.</p> <important> <p>Sending this request only prevents the AWS account that created the VPC from associating the VPC with the Amazon Route 53 hosted zone in the future. If the VPC is already associated with the hosted zone, <code>DeleteVPCAssociationAuthorization</code> won&#39;t disassociate the VPC from the hosted zone. If you want to delete an existing association, use <code>DisassociateVPCFromHostedZone</code>.</p> </important></p>
    fn delete_vpc_association_authorization(
        &self,
        input: DeleteVPCAssociationAuthorizationRequest,
    ) -> RusotoFuture<
        DeleteVPCAssociationAuthorizationResponse,
        DeleteVPCAssociationAuthorizationError,
    >;

    /// <p><p>Disassociates a VPC from a Amazon Route 53 private hosted zone. </p> <note> <p>You can&#39;t disassociate the last VPC from a private hosted zone.</p> </note> <important> <p>You can&#39;t disassociate a VPC from a private hosted zone when only one VPC is associated with the hosted zone. You also can&#39;t convert a private hosted zone into a public hosted zone.</p> </important></p>
    fn disassociate_vpc_from_hosted_zone(
        &self,
        input: DisassociateVPCFromHostedZoneRequest,
    ) -> RusotoFuture<DisassociateVPCFromHostedZoneResponse, DisassociateVPCFromHostedZoneError>;

    /// <p>Gets the specified limit for the current account, for example, the maximum number of health checks that you can create using the account.</p> <p>For the default limit, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>. To request a higher limit, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-route53">open a case</a>.</p>
    fn get_account_limit(
        &self,
        input: GetAccountLimitRequest,
    ) -> RusotoFuture<GetAccountLimitResponse, GetAccountLimitError>;

    /// <p><p>Returns the current status of a change batch request. The status is one of the following values:</p> <ul> <li> <p> <code>PENDING</code> indicates that the changes in this request have not propagated to all Amazon Route 53 DNS servers. This is the initial status of all change batch requests.</p> </li> <li> <p> <code>INSYNC</code> indicates that the changes have propagated to all Amazon Route 53 DNS servers. </p> </li> </ul></p>
    fn get_change(
        &self,
        input: GetChangeRequest,
    ) -> RusotoFuture<GetChangeResponse, GetChangeError>;

    /// <p> <code>GetCheckerIpRanges</code> still works, but we recommend that you download ip-ranges.json, which includes IP address ranges for all AWS services. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/route-53-ip-addresses.html">IP Address Ranges of Amazon Route 53 Servers</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    fn get_checker_ip_ranges(
        &self,
        input: GetCheckerIpRangesRequest,
    ) -> RusotoFuture<GetCheckerIpRangesResponse, GetCheckerIpRangesError>;

    /// <p>Gets information about whether a specified geographic location is supported for Amazon Route 53 geolocation resource record sets.</p> <p>Use the following syntax to determine whether a continent is supported for geolocation:</p> <p> <code>GET /2013-04-01/geolocation?ContinentCode=<i>two-letter abbreviation for a continent</i> </code> </p> <p>Use the following syntax to determine whether a country is supported for geolocation:</p> <p> <code>GET /2013-04-01/geolocation?CountryCode=<i>two-character country code</i> </code> </p> <p>Use the following syntax to determine whether a subdivision of a country is supported for geolocation:</p> <p> <code>GET /2013-04-01/geolocation?CountryCode=<i>two-character country code</i>&amp;SubdivisionCode=<i>subdivision code</i> </code> </p>
    fn get_geo_location(
        &self,
        input: GetGeoLocationRequest,
    ) -> RusotoFuture<GetGeoLocationResponse, GetGeoLocationError>;

    /// <p>Gets information about a specified health check.</p>
    fn get_health_check(
        &self,
        input: GetHealthCheckRequest,
    ) -> RusotoFuture<GetHealthCheckResponse, GetHealthCheckError>;

    /// <p>Retrieves the number of health checks that are associated with the current AWS account.</p>
    fn get_health_check_count(
        &self,
        input: GetHealthCheckCountRequest,
    ) -> RusotoFuture<GetHealthCheckCountResponse, GetHealthCheckCountError>;

    /// <p>Gets the reason that a specified health check failed most recently.</p>
    fn get_health_check_last_failure_reason(
        &self,
        input: GetHealthCheckLastFailureReasonRequest,
    ) -> RusotoFuture<GetHealthCheckLastFailureReasonResponse, GetHealthCheckLastFailureReasonError>;

    /// <p>Gets status of a specified health check. </p>
    fn get_health_check_status(
        &self,
        input: GetHealthCheckStatusRequest,
    ) -> RusotoFuture<GetHealthCheckStatusResponse, GetHealthCheckStatusError>;

    /// <p>Gets information about a specified hosted zone including the four name servers assigned to the hosted zone.</p>
    fn get_hosted_zone(
        &self,
        input: GetHostedZoneRequest,
    ) -> RusotoFuture<GetHostedZoneResponse, GetHostedZoneError>;

    /// <p>Retrieves the number of hosted zones that are associated with the current AWS account.</p>
    fn get_hosted_zone_count(
        &self,
        input: GetHostedZoneCountRequest,
    ) -> RusotoFuture<GetHostedZoneCountResponse, GetHostedZoneCountError>;

    /// <p>Gets the specified limit for a specified hosted zone, for example, the maximum number of records that you can create in the hosted zone. </p> <p>For the default limit, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>. To request a higher limit, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-route53">open a case</a>.</p>
    fn get_hosted_zone_limit(
        &self,
        input: GetHostedZoneLimitRequest,
    ) -> RusotoFuture<GetHostedZoneLimitResponse, GetHostedZoneLimitError>;

    /// <p>Gets information about a specified configuration for DNS query logging.</p> <p>For more information about DNS query logs, see <a>CreateQueryLoggingConfig</a> and <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html">Logging DNS Queries</a>.</p>
    fn get_query_logging_config(
        &self,
        input: GetQueryLoggingConfigRequest,
    ) -> RusotoFuture<GetQueryLoggingConfigResponse, GetQueryLoggingConfigError>;

    /// <p>Retrieves information about a specified reusable delegation set, including the four name servers that are assigned to the delegation set.</p>
    fn get_reusable_delegation_set(
        &self,
        input: GetReusableDelegationSetRequest,
    ) -> RusotoFuture<GetReusableDelegationSetResponse, GetReusableDelegationSetError>;

    /// <p>Gets the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p> <p>For the default limit, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>. To request a higher limit, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-route53">open a case</a>.</p>
    fn get_reusable_delegation_set_limit(
        &self,
        input: GetReusableDelegationSetLimitRequest,
    ) -> RusotoFuture<GetReusableDelegationSetLimitResponse, GetReusableDelegationSetLimitError>;

    /// <p>Gets information about a specific traffic policy version.</p>
    fn get_traffic_policy(
        &self,
        input: GetTrafficPolicyRequest,
    ) -> RusotoFuture<GetTrafficPolicyResponse, GetTrafficPolicyError>;

    /// <p><p>Gets information about a specified traffic policy instance.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there&#39;s a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <note> <p>In the Amazon Route 53 console, traffic policy instances are known as policy records.</p> </note></p>
    fn get_traffic_policy_instance(
        &self,
        input: GetTrafficPolicyInstanceRequest,
    ) -> RusotoFuture<GetTrafficPolicyInstanceResponse, GetTrafficPolicyInstanceError>;

    /// <p>Gets the number of traffic policy instances that are associated with the current AWS account.</p>
    fn get_traffic_policy_instance_count(
        &self,
        input: GetTrafficPolicyInstanceCountRequest,
    ) -> RusotoFuture<GetTrafficPolicyInstanceCountResponse, GetTrafficPolicyInstanceCountError>;

    /// <p>Retrieves a list of supported geo locations.</p> <p>Countries are listed first, and continents are listed last. If Amazon Route 53 supports subdivisions for a country (for example, states or provinces), the subdivisions for that country are listed in alphabetical order immediately after the corresponding country.</p>
    fn list_geo_locations(
        &self,
        input: ListGeoLocationsRequest,
    ) -> RusotoFuture<ListGeoLocationsResponse, ListGeoLocationsError>;

    /// <p>Retrieve a list of the health checks that are associated with the current AWS account. </p>
    fn list_health_checks(
        &self,
        input: ListHealthChecksRequest,
    ) -> RusotoFuture<ListHealthChecksResponse, ListHealthChecksError>;

    /// <p>Retrieves a list of the public and private hosted zones that are associated with the current AWS account. The response includes a <code>HostedZones</code> child element for each hosted zone.</p> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of hosted zones, you can use the <code>maxitems</code> parameter to list them in groups of up to 100.</p>
    fn list_hosted_zones(
        &self,
        input: ListHostedZonesRequest,
    ) -> RusotoFuture<ListHostedZonesResponse, ListHostedZonesError>;

    /// <p><p>Retrieves a list of your hosted zones in lexicographic order. The response includes a <code>HostedZones</code> child element for each hosted zone created by the current AWS account. </p> <p> <code>ListHostedZonesByName</code> sorts hosted zones by name with the labels reversed. For example:</p> <p> <code>com.example.www.</code> </p> <p>Note the trailing dot, which can change the sort order in some circumstances.</p> <p>If the domain name includes escape characters or Punycode, <code>ListHostedZonesByName</code> alphabetizes the domain name using the escaped or Punycoded value, which is the format that Amazon Route 53 saves in its database. For example, to create a hosted zone for exämple.com, you specify ex\344mple.com for the domain name. <code>ListHostedZonesByName</code> alphabetizes it as:</p> <p> <code>com.ex\344mple.</code> </p> <p>The labels are reversed and alphabetized using the escaped value. For more information about valid domain name formats, including internationalized domain names, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DomainNameFormat.html">DNS Domain Name Format</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>Amazon Route 53 returns up to 100 items in each response. If you have a lot of hosted zones, use the <code>MaxItems</code> parameter to list them in groups of up to 100. The response includes values that help navigate from one group of <code>MaxItems</code> hosted zones to the next:</p> <ul> <li> <p>The <code>DNSName</code> and <code>HostedZoneId</code> elements in the response contain the values, if any, specified for the <code>dnsname</code> and <code>hostedzoneid</code> parameters in the request that produced the current response.</p> </li> <li> <p>The <code>MaxItems</code> element in the response contains the value, if any, that you specified for the <code>maxitems</code> parameter in the request that produced the current response.</p> </li> <li> <p>If the value of <code>IsTruncated</code> in the response is true, there are more hosted zones associated with the current AWS account. </p> <p>If <code>IsTruncated</code> is false, this response includes the last hosted zone that is associated with the current account. The <code>NextDNSName</code> element and <code>NextHostedZoneId</code> elements are omitted from the response.</p> </li> <li> <p>The <code>NextDNSName</code> and <code>NextHostedZoneId</code> elements in the response contain the domain name and the hosted zone ID of the next hosted zone that is associated with the current AWS account. If you want to list more hosted zones, make another call to <code>ListHostedZonesByName</code>, and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p> </li> </ul></p>
    fn list_hosted_zones_by_name(
        &self,
        input: ListHostedZonesByNameRequest,
    ) -> RusotoFuture<ListHostedZonesByNameResponse, ListHostedZonesByNameError>;

    /// <p>Lists the configurations for DNS query logging that are associated with the current AWS account or the configuration that is associated with a specified hosted zone.</p> <p>For more information about DNS query logs, see <a>CreateQueryLoggingConfig</a>. Additional information, including the format of DNS query logs, appears in <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html">Logging DNS Queries</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    fn list_query_logging_configs(
        &self,
        input: ListQueryLoggingConfigsRequest,
    ) -> RusotoFuture<ListQueryLoggingConfigsResponse, ListQueryLoggingConfigsError>;

    /// <p>Lists the resource record sets in a specified hosted zone.</p> <p> <code>ListResourceRecordSets</code> returns up to 100 resource record sets at a time in ASCII order, beginning at a position specified by the <code>name</code> and <code>type</code> elements. The action sorts results first by DNS name with the labels reversed, for example:</p> <p> <code>com.example.www.</code> </p> <p>Note the trailing dot, which can change the sort order in some circumstances.</p> <p>When multiple records have the same DNS name, the action sorts results by the record type.</p> <p>You can use the name and type elements to adjust the beginning position of the list of resource record sets returned:</p> <dl> <dt>If you do not specify Name or Type</dt> <dd> <p>The results begin with the first resource record set that the hosted zone contains.</p> </dd> <dt>If you specify Name but not Type</dt> <dd> <p>The results begin with the first resource record set in the list whose name is greater than or equal to <code>Name</code>.</p> </dd> <dt>If you specify Type but not Name</dt> <dd> <p>Amazon Route 53 returns the <code>InvalidInput</code> error.</p> </dd> <dt>If you specify both Name and Type</dt> <dd> <p>The results begin with the first resource record set in the list whose name is greater than or equal to <code>Name</code>, and whose type is greater than or equal to <code>Type</code>.</p> </dd> </dl> <p>This action returns the most current version of the records. This includes records that are <code>PENDING</code>, and that are not yet available on all Amazon Route 53 DNS servers.</p> <p>To ensure that you get an accurate listing of the resource record sets for a hosted zone at a point in time, do not submit a <code>ChangeResourceRecordSets</code> request while you're paging through the results of a <code>ListResourceRecordSets</code> request. If you do, some pages may display results without the latest changes while other pages display results with the latest changes.</p>
    fn list_resource_record_sets(
        &self,
        input: ListResourceRecordSetsRequest,
    ) -> RusotoFuture<ListResourceRecordSetsResponse, ListResourceRecordSetsError>;

    /// <p>Retrieves a list of the reusable delegation sets that are associated with the current AWS account.</p>
    fn list_reusable_delegation_sets(
        &self,
        input: ListReusableDelegationSetsRequest,
    ) -> RusotoFuture<ListReusableDelegationSetsResponse, ListReusableDelegationSetsError>;

    /// <p>Lists tags for one health check or hosted zone. </p> <p>For information about using tags for cost allocation, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Lists tags for up to 10 health checks or hosted zones.</p> <p>For information about using tags for cost allocation, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    fn list_tags_for_resources(
        &self,
        input: ListTagsForResourcesRequest,
    ) -> RusotoFuture<ListTagsForResourcesResponse, ListTagsForResourcesError>;

    /// <p>Gets information about the latest version for every traffic policy that is associated with the current AWS account. Policies are listed in the order in which they were created. </p>
    fn list_traffic_policies(
        &self,
        input: ListTrafficPoliciesRequest,
    ) -> RusotoFuture<ListTrafficPoliciesResponse, ListTrafficPoliciesError>;

    /// <p>Gets information about the traffic policy instances that you created by using the current AWS account.</p> <note> <p>After you submit an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p>
    fn list_traffic_policy_instances(
        &self,
        input: ListTrafficPolicyInstancesRequest,
    ) -> RusotoFuture<ListTrafficPolicyInstancesResponse, ListTrafficPolicyInstancesError>;

    /// <p>Gets information about the traffic policy instances that you created in a specified hosted zone.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p>
    fn list_traffic_policy_instances_by_hosted_zone(
        &self,
        input: ListTrafficPolicyInstancesByHostedZoneRequest,
    ) -> RusotoFuture<
        ListTrafficPolicyInstancesByHostedZoneResponse,
        ListTrafficPolicyInstancesByHostedZoneError,
    >;

    /// <p>Gets information about the traffic policy instances that you created by using a specify traffic policy version.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p>
    fn list_traffic_policy_instances_by_policy(
        &self,
        input: ListTrafficPolicyInstancesByPolicyRequest,
    ) -> RusotoFuture<
        ListTrafficPolicyInstancesByPolicyResponse,
        ListTrafficPolicyInstancesByPolicyError,
    >;

    /// <p>Gets information about all of the versions for a specified traffic policy.</p> <p>Traffic policy versions are listed in numerical order by <code>VersionNumber</code>.</p>
    fn list_traffic_policy_versions(
        &self,
        input: ListTrafficPolicyVersionsRequest,
    ) -> RusotoFuture<ListTrafficPolicyVersionsResponse, ListTrafficPolicyVersionsError>;

    /// <p>Gets a list of the VPCs that were created by other accounts and that can be associated with a specified hosted zone because you've submitted one or more <code>CreateVPCAssociationAuthorization</code> requests. </p> <p>The response includes a <code>VPCs</code> element with a <code>VPC</code> child element for each VPC that can be associated with the hosted zone.</p>
    fn list_vpc_association_authorizations(
        &self,
        input: ListVPCAssociationAuthorizationsRequest,
    ) -> RusotoFuture<ListVPCAssociationAuthorizationsResponse, ListVPCAssociationAuthorizationsError>;

    /// <p>Gets the value that Amazon Route 53 returns in response to a DNS request for a specified record name and type. You can optionally specify the IP address of a DNS resolver, an EDNS0 client subnet IP address, and a subnet mask. </p>
    fn test_dns_answer(
        &self,
        input: TestDNSAnswerRequest,
    ) -> RusotoFuture<TestDNSAnswerResponse, TestDNSAnswerError>;

    /// <p>Updates an existing health check. Note that some values can't be updated. </p> <p>For more information about updating health checks, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/health-checks-creating-deleting.html">Creating, Updating, and Deleting Health Checks</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    fn update_health_check(
        &self,
        input: UpdateHealthCheckRequest,
    ) -> RusotoFuture<UpdateHealthCheckResponse, UpdateHealthCheckError>;

    /// <p>Updates the comment for a specified hosted zone.</p>
    fn update_hosted_zone_comment(
        &self,
        input: UpdateHostedZoneCommentRequest,
    ) -> RusotoFuture<UpdateHostedZoneCommentResponse, UpdateHostedZoneCommentError>;

    /// <p>Updates the comment for a specified traffic policy version.</p>
    fn update_traffic_policy_comment(
        &self,
        input: UpdateTrafficPolicyCommentRequest,
    ) -> RusotoFuture<UpdateTrafficPolicyCommentResponse, UpdateTrafficPolicyCommentError>;

    /// <p><p>Updates the resource record sets in a specified hosted zone that were created based on the settings in a specified traffic policy version.</p> <p>When you update a traffic policy instance, Amazon Route 53 continues to respond to DNS queries for the root resource record set name (such as example.com) while it replaces one group of resource record sets with another. Amazon Route 53 performs the following operations:</p> <ol> <li> <p>Amazon Route 53 creates a new group of resource record sets based on the specified traffic policy. This is true regardless of how significant the differences are between the existing resource record sets and the new resource record sets. </p> </li> <li> <p>When all of the new resource record sets have been created, Amazon Route 53 starts to respond to DNS queries for the root resource record set name (such as example.com) by using the new resource record sets.</p> </li> <li> <p>Amazon Route 53 deletes the old group of resource record sets that are associated with the root resource record set name.</p> </li> </ol></p>
    fn update_traffic_policy_instance(
        &self,
        input: UpdateTrafficPolicyInstanceRequest,
    ) -> RusotoFuture<UpdateTrafficPolicyInstanceResponse, UpdateTrafficPolicyInstanceError>;
}
/// A client for the Route 53 API.
pub struct Route53Client<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl Route53Client {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> Route53Client {
        Route53Client::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> Route53Client<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        Route53Client {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Route53 for Route53Client<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p><p>Associates an Amazon VPC with a private hosted zone. </p> <important> <p>To perform the association, the VPC and the private hosted zone must already exist. You can&#39;t convert a public hosted zone into a private hosted zone.</p> </important> <note> <p>If you want to associate a VPC that was created by using one AWS account with a private hosted zone that was created by using a different account, the AWS account that created the private hosted zone must first submit a <code>CreateVPCAssociationAuthorization</code> request. Then the account that created the VPC must submit an <code>AssociateVPCWithHostedZone</code> request.</p> </note></p>
    #[allow(unused_variables, warnings)]
    fn associate_vpc_with_hosted_zone(
        &self,
        input: AssociateVPCWithHostedZoneRequest,
    ) -> RusotoFuture<AssociateVPCWithHostedZoneResponse, AssociateVPCWithHostedZoneError> {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/associatevpc",
            id = input.hosted_zone_id
        );

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        AssociateVPCWithHostedZoneRequestSerializer::serialize(
            &mut writer,
            "AssociateVPCWithHostedZoneRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AssociateVPCWithHostedZoneError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = AssociateVPCWithHostedZoneResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(AssociateVPCWithHostedZoneResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates, changes, or deletes a resource record set, which contains authoritative DNS information for a specified domain name or subdomain name. For example, you can use <code>ChangeResourceRecordSets</code> to create a resource record set that routes traffic for test.example.com to a web server that has an IP address of 192.0.2.44.</p> <p> <b>Change Batches and Transactional Changes</b> </p> <p>The request body must include a document with a <code>ChangeResourceRecordSetsRequest</code> element. The request body contains a list of change items, known as a change batch. Change batches are considered transactional changes. When using the Amazon Route 53 API to change resource record sets, Amazon Route 53 either makes all or none of the changes in a change batch request. This ensures that Amazon Route 53 never partially implements the intended changes to the resource record sets in a hosted zone. </p> <p>For example, a change batch request that deletes the <code>CNAME</code> record for www.example.com and creates an alias resource record set for www.example.com. Amazon Route 53 deletes the first resource record set and creates the second resource record set in a single operation. If either the <code>DELETE</code> or the <code>CREATE</code> action fails, then both changes (plus any other changes in the batch) fail, and the original <code>CNAME</code> record continues to exist.</p> <important> <p>Due to the nature of transactional changes, you can't delete the same resource record set more than once in a single change batch. If you attempt to delete the same change batch more than once, Amazon Route 53 returns an <code>InvalidChangeBatch</code> error.</p> </important> <p> <b>Traffic Flow</b> </p> <p>To create resource record sets for complex routing configurations, use either the traffic flow visual editor in the Amazon Route 53 console or the API actions for traffic policies and traffic policy instances. Save the configuration as a traffic policy, then associate the traffic policy with one or more domain names (such as example.com) or subdomain names (such as www.example.com), in the same hosted zone or in multiple hosted zones. You can roll back the updates if the new configuration isn't performing as expected. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/traffic-flow.html">Using Traffic Flow to Route DNS Traffic</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p> <b>Create, Delete, and Upsert</b> </p> <p>Use <code>ChangeResourceRecordsSetsRequest</code> to perform the following actions:</p> <ul> <li> <p> <code>CREATE</code>: Creates a resource record set that has the specified values.</p> </li> <li> <p> <code>DELETE</code>: Deletes an existing resource record set that has the specified values.</p> </li> <li> <p> <code>UPSERT</code>: If a resource record set does not already exist, AWS creates it. If a resource set does exist, Amazon Route 53 updates it with the values in the request. </p> </li> </ul> <p> <b>Syntaxes for Creating, Updating, and Deleting Resource Record Sets</b> </p> <p>The syntax for a request depends on the type of resource record set that you want to create, delete, or update, such as weighted, alias, or failover. The XML elements in your request must appear in the order listed in the syntax. </p> <p>For an example for each type of resource record set, see "Examples."</p> <p>Don't refer to the syntax in the "Parameter Syntax" section, which includes all of the elements for every kind of resource record set that you can create, delete, or update by using <code>ChangeResourceRecordSets</code>. </p> <p> <b>Change Propagation to Amazon Route 53 DNS Servers</b> </p> <p>When you submit a <code>ChangeResourceRecordSets</code> request, Amazon Route 53 propagates your changes to all of the Amazon Route 53 authoritative DNS servers. While your changes are propagating, <code>GetChange</code> returns a status of <code>PENDING</code>. When propagation is complete, <code>GetChange</code> returns a status of <code>INSYNC</code>. Changes generally propagate to all Amazon Route 53 name servers within 60 seconds. For more information, see <a>GetChange</a>.</p> <p> <b>Limits on ChangeResourceRecordSets Requests</b> </p> <p>For information about the limits on a <code>ChangeResourceRecordSets</code> request, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    fn change_resource_record_sets(
        &self,
        input: ChangeResourceRecordSetsRequest,
    ) -> RusotoFuture<ChangeResourceRecordSetsResponse, ChangeResourceRecordSetsError> {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/rrset/",
            id = input.hosted_zone_id
        );

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        ChangeResourceRecordSetsRequestSerializer::serialize(
            &mut writer,
            "ChangeResourceRecordSetsRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ChangeResourceRecordSetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ChangeResourceRecordSetsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ChangeResourceRecordSetsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds, edits, or deletes tags for a health check or a hosted zone.</p> <p>For information about using tags for cost allocation, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    fn change_tags_for_resource(
        &self,
        input: ChangeTagsForResourceRequest,
    ) -> RusotoFuture<ChangeTagsForResourceResponse, ChangeTagsForResourceError> {
        let request_uri = format!(
            "/2013-04-01/tags/{resource_type}/{resource_id}",
            resource_id = input.resource_id,
            resource_type = input.resource_type
        );

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        ChangeTagsForResourceRequestSerializer::serialize(
            &mut writer,
            "ChangeTagsForResourceRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ChangeTagsForResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ChangeTagsForResourceResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ChangeTagsForResourceResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a new health check.</p> <p>For information about adding health checks to resource record sets, see <a>ResourceRecordSet$HealthCheckId</a> in <a>ChangeResourceRecordSets</a>. </p> <p> <b>ELB Load Balancers</b> </p> <p>If you&#39;re registering EC2 instances with an Elastic Load Balancing (ELB) load balancer, do not create Amazon Route 53 health checks for the EC2 instances. When you register an EC2 instance with a load balancer, you configure settings for an ELB health check, which performs a similar function to an Amazon Route 53 health check.</p> <p> <b>Private Hosted Zones</b> </p> <p>You can associate health checks with failover resource record sets in a private hosted zone. Note the following:</p> <ul> <li> <p>Amazon Route 53 health checkers are outside the VPC. To check the health of an endpoint within a VPC by IP address, you must assign a public IP address to the instance in the VPC.</p> </li> <li> <p>You can configure a health checker to check the health of an external resource that the instance relies on, such as a database server.</p> </li> <li> <p>You can create a CloudWatch metric, associate an alarm with the metric, and then create a health check that is based on the state of the alarm. For example, you might create a CloudWatch metric that checks the status of the Amazon EC2 <code>StatusCheckFailed</code> metric, add an alarm to the metric, and then create a health check that is based on the state of the alarm. For information about creating CloudWatch metrics and alarms by using the CloudWatch console, see the <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/WhatIsCloudWatch.html">Amazon CloudWatch User Guide</a>.</p> </li> </ul></p>
    #[allow(unused_variables, warnings)]
    fn create_health_check(
        &self,
        input: CreateHealthCheckRequest,
    ) -> RusotoFuture<CreateHealthCheckResponse, CreateHealthCheckError> {
        let request_uri = "/2013-04-01/healthcheck";

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CreateHealthCheckRequestSerializer::serialize(
            &mut writer,
            "CreateHealthCheckRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateHealthCheckError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateHealthCheckResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CreateHealthCheckResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                let value = response.headers.get("Location").unwrap().to_owned();
                result.location = value;
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new public hosted zone, which you use to specify how the Domain Name System (DNS) routes traffic on the Internet for a domain, such as example.com, and its subdomains. </p> <important> <p>You can't convert a public hosted zones to a private hosted zone or vice versa. Instead, you must create a new hosted zone with the same name and create new resource record sets.</p> </important> <p>For more information about charges for hosted zones, see <a href="http://aws.amazon.com/route53/pricing/">Amazon Route 53 Pricing</a>.</p> <p>Note the following:</p> <ul> <li> <p>You can't create a hosted zone for a top-level domain (TLD).</p> </li> <li> <p>Amazon Route 53 automatically creates a default SOA record and four NS records for the zone. For more information about SOA and NS records, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/SOA-NSrecords.html">NS and SOA Records that Amazon Route 53 Creates for a Hosted Zone</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If you want to use the same name servers for multiple hosted zones, you can optionally associate a reusable delegation set with the hosted zone. See the <code>DelegationSetId</code> element.</p> </li> <li> <p>If your domain is registered with a registrar other than Amazon Route 53, you must update the name servers with your registrar to make Amazon Route 53 your DNS service. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/creating-migrating.html">Configuring Amazon Route 53 as your DNS Service</a> in the <i>Amazon Route 53 Developer Guide</i>. </p> </li> </ul> <p>When you submit a <code>CreateHostedZone</code> request, the initial status of the hosted zone is <code>PENDING</code>. This means that the NS and SOA records are not yet available on all Amazon Route 53 DNS servers. When the NS and SOA records are available, the status of the zone changes to <code>INSYNC</code>.</p>
    #[allow(unused_variables, warnings)]
    fn create_hosted_zone(
        &self,
        input: CreateHostedZoneRequest,
    ) -> RusotoFuture<CreateHostedZoneResponse, CreateHostedZoneError> {
        let request_uri = "/2013-04-01/hostedzone";

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CreateHostedZoneRequestSerializer::serialize(
            &mut writer,
            "CreateHostedZoneRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateHostedZoneError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateHostedZoneResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CreateHostedZoneResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                let value = response.headers.get("Location").unwrap().to_owned();
                result.location = value;
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a configuration for DNS query logging. After you create a query logging configuration, Amazon Route 53 begins to publish log data to an Amazon CloudWatch Logs log group.</p> <p>DNS query logs contain information about the queries that Amazon Route 53 receives for a specified public hosted zone, such as the following:</p> <ul> <li> <p>Amazon Route 53 edge location that responded to the DNS query</p> </li> <li> <p>Domain or subdomain that was requested</p> </li> <li> <p>DNS record type, such as A or AAAA</p> </li> <li> <p>DNS response code, such as <code>NoError</code> or <code>ServFail</code> </p> </li> </ul> <dl> <dt>Log Group and Resource Policy</dt> <dd> <p>Before you create a query logging configuration, perform the following operations.</p> <note> <p>If you create a query logging configuration using the Amazon Route 53 console, Amazon Route 53 performs these operations automatically.</p> </note> <ol> <li> <p>Create a CloudWatch Logs log group, and make note of the ARN, which you specify when you create a query logging configuration. Note the following:</p> <ul> <li> <p>You must create the log group in the us-east-1 region.</p> </li> <li> <p>You must use the same AWS account to create the log group and the hosted zone that you want to configure query logging for.</p> </li> <li> <p>When you create log groups for query logging, we recommend that you use a consistent prefix, for example:</p> <p> <code>/aws/route53/<i>hosted zone name</i> </code> </p> <p>In the next step, you&#39;ll create a resource policy, which controls access to one or more log groups and the associated AWS resources, such as Amazon Route 53 hosted zones. There&#39;s a limit on the number of resource policies that you can create, so we recommend that you use a consistent prefix so you can use the same resource policy for all the log groups that you create for query logging.</p> </li> </ul> </li> <li> <p>Create a CloudWatch Logs resource policy, and give it the permissions that Amazon Route 53 needs to create log streams and to send query logs to log streams. For the value of <code>Resource</code>, specify the ARN for the log group that you created in the previous step. To use the same resource policy for all the CloudWatch Logs log groups that you created for query logging configurations, replace the hosted zone name with <code><em></code>, for example:</p> <p> <code>arn:aws:logs:us-east-1:123412341234:log-group:/aws/route53/</em></code> </p> <note> <p>You can&#39;t use the CloudWatch console to create or edit a resource policy. You must use the CloudWatch API, one of the AWS SDKs, or the AWS CLI.</p> </note> </li> </ol> </dd> <dt>Log Streams and Edge Locations</dt> <dd> <p>When Amazon Route 53 finishes creating the configuration for DNS query logging, it does the following:</p> <ul> <li> <p>Creates a log stream for an edge location the first time that the edge location responds to DNS queries for the specified hosted zone. That log stream is used to log all queries that Amazon Route 53 responds to for that edge location.</p> </li> <li> <p>Begins to send query logs to the applicable log stream.</p> </li> </ul> <p>The name of each log stream is in the following format:</p> <p> <code> <i>hosted zone ID</i>/<i>edge location code</i> </code> </p> <p>The edge location code is a three-letter code and an arbitrarily assigned number, for example, DFW3. The three-letter code typically corresponds with the International Air Transport Association airport code for an airport near the edge location. (These abbreviations might change in the future.) For a list of edge locations, see &quot;The Amazon Route 53 Global Network&quot; on the <a href="http://aws.amazon.com/route53/details/">Amazon Route 53 Product Details</a> page.</p> </dd> <dt>Queries That Are Logged</dt> <dd> <p>Query logs contain only the queries that DNS resolvers forward to Amazon Route 53. If a DNS resolver has already cached the response to a query (such as the IP address for a load balancer for example.com), the resolver will continue to return the cached response. It doesn&#39;t forward another query to Amazon Route 53 until the TTL for the corresponding resource record set expires. Depending on how many DNS queries are submitted for a resource record set, and depending on the TTL for that resource record set, query logs might contain information about only one query out of every several thousand queries that are submitted to DNS. For more information about how DNS works, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/welcome-dns-service.html">Routing Internet Traffic to Your Website or Web Application</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </dd> <dt>Log File Format</dt> <dd> <p>For a list of the values in each query log and the format of each value, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html">Logging DNS Queries</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </dd> <dt>Pricing</dt> <dd> <p>For information about charges for query logs, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </dd> <dt>How to Stop Logging</dt> <dd> <p>If you want Amazon Route 53 to stop sending query logs to CloudWatch Logs, delete the query logging configuration. For more information, see <a>DeleteQueryLoggingConfig</a>.</p> </dd> </dl></p>
    #[allow(unused_variables, warnings)]
    fn create_query_logging_config(
        &self,
        input: CreateQueryLoggingConfigRequest,
    ) -> RusotoFuture<CreateQueryLoggingConfigResponse, CreateQueryLoggingConfigError> {
        let request_uri = "/2013-04-01/queryloggingconfig";

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CreateQueryLoggingConfigRequestSerializer::serialize(
            &mut writer,
            "CreateQueryLoggingConfigRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateQueryLoggingConfigError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateQueryLoggingConfigResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CreateQueryLoggingConfigResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                let value = response.headers.get("Location").unwrap().to_owned();
                result.location = value;
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a delegation set (a group of four name servers) that can be reused by multiple hosted zones. If a hosted zoned ID is specified, <code>CreateReusableDelegationSet</code> marks the delegation set associated with that zone as reusable.</p> <note> <p>You can&#39;t associate a reusable delegation set with a private hosted zone.</p> </note> <p>For information about using a reusable delegation set to configure white label name servers, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/white-label-name-servers.html">Configuring White Label Name Servers</a>.</p> <p>The process for migrating existing hosted zones to use a reusable delegation set is comparable to the process for configuring white label name servers. You need to perform the following steps:</p> <ol> <li> <p>Create a reusable delegation set.</p> </li> <li> <p>Recreate hosted zones, and reduce the TTL to 60 seconds or less.</p> </li> <li> <p>Recreate resource record sets in the new hosted zones.</p> </li> <li> <p>Change the registrar&#39;s name servers to use the name servers for the new hosted zones.</p> </li> <li> <p>Monitor traffic for the website or application.</p> </li> <li> <p>Change TTLs back to their original values.</p> </li> </ol> <p>If you want to migrate existing hosted zones to use a reusable delegation set, the existing hosted zones can&#39;t use any of the name servers that are assigned to the reusable delegation set. If one or more hosted zones do use one or more name servers that are assigned to the reusable delegation set, you can do one of the following:</p> <ul> <li> <p>For small numbers of hosted zones—up to a few hundred—it&#39;s relatively easy to create reusable delegation sets until you get one that has four name servers that don&#39;t overlap with any of the name servers in your hosted zones.</p> </li> <li> <p>For larger numbers of hosted zones, the easiest solution is to use more than one reusable delegation set.</p> </li> <li> <p>For larger numbers of hosted zones, you can also migrate hosted zones that have overlapping name servers to hosted zones that don&#39;t have overlapping name servers, then migrate the hosted zones again to use the reusable delegation set.</p> </li> </ul></p>
    #[allow(unused_variables, warnings)]
    fn create_reusable_delegation_set(
        &self,
        input: CreateReusableDelegationSetRequest,
    ) -> RusotoFuture<CreateReusableDelegationSetResponse, CreateReusableDelegationSetError> {
        let request_uri = "/2013-04-01/delegationset";

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CreateReusableDelegationSetRequestSerializer::serialize(
            &mut writer,
            "CreateReusableDelegationSetRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateReusableDelegationSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateReusableDelegationSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        CreateReusableDelegationSetResponseDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                let value = response.headers.get("Location").unwrap().to_owned();
                result.location = value;
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a traffic policy, which you use to create multiple DNS resource record sets for one domain name (such as example.com) or one subdomain name (such as www.example.com).</p>
    #[allow(unused_variables, warnings)]
    fn create_traffic_policy(
        &self,
        input: CreateTrafficPolicyRequest,
    ) -> RusotoFuture<CreateTrafficPolicyResponse, CreateTrafficPolicyError> {
        let request_uri = "/2013-04-01/trafficpolicy";

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CreateTrafficPolicyRequestSerializer::serialize(
            &mut writer,
            "CreateTrafficPolicyRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateTrafficPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateTrafficPolicyResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CreateTrafficPolicyResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                let value = response.headers.get("Location").unwrap().to_owned();
                result.location = value;
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates resource record sets in a specified hosted zone based on the settings in a specified traffic policy version. In addition, <code>CreateTrafficPolicyInstance</code> associates the resource record sets with a specified domain name (such as example.com) or subdomain name (such as www.example.com). Amazon Route 53 responds to DNS queries for the domain or subdomain name by using the resource record sets that <code>CreateTrafficPolicyInstance</code> created.</p>
    #[allow(unused_variables, warnings)]
    fn create_traffic_policy_instance(
        &self,
        input: CreateTrafficPolicyInstanceRequest,
    ) -> RusotoFuture<CreateTrafficPolicyInstanceResponse, CreateTrafficPolicyInstanceError> {
        let request_uri = "/2013-04-01/trafficpolicyinstance";

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CreateTrafficPolicyInstanceRequestSerializer::serialize(
            &mut writer,
            "CreateTrafficPolicyInstanceRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateTrafficPolicyInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateTrafficPolicyInstanceResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        CreateTrafficPolicyInstanceResponseDeserializer::deserialize(
                            &actual_tag_name,
                            &mut stack
                        )
                    );
                }

                let value = response.headers.get("Location").unwrap().to_owned();
                result.location = value;
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new version of an existing traffic policy. When you create a new version of a traffic policy, you specify the ID of the traffic policy that you want to update and a JSON-formatted document that describes the new version. You use traffic policies to create multiple DNS resource record sets for one domain name (such as example.com) or one subdomain name (such as www.example.com). You can create a maximum of 1000 versions of a traffic policy. If you reach the limit and need to create another version, you'll need to start a new traffic policy.</p>
    #[allow(unused_variables, warnings)]
    fn create_traffic_policy_version(
        &self,
        input: CreateTrafficPolicyVersionRequest,
    ) -> RusotoFuture<CreateTrafficPolicyVersionResponse, CreateTrafficPolicyVersionError> {
        let request_uri = format!("/2013-04-01/trafficpolicy/{id}", id = input.id);

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CreateTrafficPolicyVersionRequestSerializer::serialize(
            &mut writer,
            "CreateTrafficPolicyVersionRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateTrafficPolicyVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateTrafficPolicyVersionResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CreateTrafficPolicyVersionResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                let value = response.headers.get("Location").unwrap().to_owned();
                result.location = value;
                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Authorizes the AWS account that created a specified VPC to submit an <code>AssociateVPCWithHostedZone</code> request to associate the VPC with a specified hosted zone that was created by a different account. To submit a <code>CreateVPCAssociationAuthorization</code> request, you must use the account that created the hosted zone. After you authorize the association, use the account that created the VPC to submit an <code>AssociateVPCWithHostedZone</code> request.</p> <note> <p>If you want to associate multiple VPCs that you created by using one account with a hosted zone that you created by using a different account, you must submit one authorization request for each VPC.</p> </note></p>
    #[allow(unused_variables, warnings)]
    fn create_vpc_association_authorization(
        &self,
        input: CreateVPCAssociationAuthorizationRequest,
    ) -> RusotoFuture<
        CreateVPCAssociationAuthorizationResponse,
        CreateVPCAssociationAuthorizationError,
    > {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/authorizevpcassociation",
            id = input.hosted_zone_id
        );

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CreateVPCAssociationAuthorizationRequestSerializer::serialize(
            &mut writer,
            "CreateVPCAssociationAuthorizationRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateVPCAssociationAuthorizationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateVPCAssociationAuthorizationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        CreateVPCAssociationAuthorizationResponseDeserializer::deserialize(
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

    /// <p><p>Deletes a health check.</p> <important> <p>Amazon Route 53 does not prevent you from deleting a health check even if the health check is associated with one or more resource record sets. If you delete a health check and you don&#39;t update the associated resource record sets, the future status of the health check can&#39;t be predicted and may change. This will affect the routing of DNS queries for your DNS failover configuration. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/health-checks-creating-deleting.html#health-checks-deleting.html">Replacing and Deleting Health Checks</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </important></p>
    #[allow(unused_variables, warnings)]
    fn delete_health_check(
        &self,
        input: DeleteHealthCheckRequest,
    ) -> RusotoFuture<DeleteHealthCheckResponse, DeleteHealthCheckError> {
        let request_uri = format!(
            "/2013-04-01/healthcheck/{health_check_id}",
            health_check_id = input.health_check_id
        );

        let mut request = SignedRequest::new("DELETE", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteHealthCheckError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteHealthCheckResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(DeleteHealthCheckResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes a hosted zone.</p> <important> <p>If the name servers for the hosted zone are associated with a domain and if you want to make the domain unavailable on the Internet, we recommend that you delete the name servers from the domain to prevent future DNS queries from possibly being misrouted. If the domain is registered with Amazon Route 53, see <code>UpdateDomainNameservers</code>. If the domain is registered with another registrar, use the method provided by the registrar to delete name servers for the domain.</p> <p>Some domain registries don&#39;t allow you to remove all of the name servers for a domain. If the registry for your domain requires one or more name servers, we recommend that you delete the hosted zone only if you transfer DNS service to another service provider, and you replace the name servers for the domain with name servers from the new provider.</p> </important> <p>You can delete a hosted zone only if it contains only the default SOA record and NS resource record sets. If the hosted zone contains other resource record sets, you must delete them before you can delete the hosted zone. If you try to delete a hosted zone that contains other resource record sets, the request fails, and Amazon Route 53 returns a <code>HostedZoneNotEmpty</code> error. For information about deleting records from your hosted zone, see <a>ChangeResourceRecordSets</a>.</p> <p>To verify that the hosted zone has been deleted, do one of the following:</p> <ul> <li> <p>Use the <code>GetHostedZone</code> action to request information about the hosted zone.</p> </li> <li> <p>Use the <code>ListHostedZones</code> action to get a list of the hosted zones associated with the current AWS account.</p> </li> </ul></p>
    #[allow(unused_variables, warnings)]
    fn delete_hosted_zone(
        &self,
        input: DeleteHostedZoneRequest,
    ) -> RusotoFuture<DeleteHostedZoneResponse, DeleteHostedZoneError> {
        let request_uri = format!("/2013-04-01/hostedzone/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteHostedZoneError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteHostedZoneResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(DeleteHostedZoneResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a configuration for DNS query logging. If you delete a configuration, Amazon Route 53 stops sending query logs to CloudWatch Logs. Amazon Route 53 doesn't delete any logs that are already in CloudWatch Logs.</p> <p>For more information about DNS query logs, see <a>CreateQueryLoggingConfig</a>.</p>
    #[allow(unused_variables, warnings)]
    fn delete_query_logging_config(
        &self,
        input: DeleteQueryLoggingConfigRequest,
    ) -> RusotoFuture<DeleteQueryLoggingConfigResponse, DeleteQueryLoggingConfigError> {
        let request_uri = format!("/2013-04-01/queryloggingconfig/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteQueryLoggingConfigError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteQueryLoggingConfigResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(DeleteQueryLoggingConfigResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a reusable delegation set.</p> <important> <p>You can delete a reusable delegation set only if it isn't associated with any hosted zones.</p> </important> <p>To verify that the reusable delegation set is not associated with any hosted zones, submit a <a>GetReusableDelegationSet</a> request and specify the ID of the reusable delegation set that you want to delete.</p>
    #[allow(unused_variables, warnings)]
    fn delete_reusable_delegation_set(
        &self,
        input: DeleteReusableDelegationSetRequest,
    ) -> RusotoFuture<DeleteReusableDelegationSetResponse, DeleteReusableDelegationSetError> {
        let request_uri = format!("/2013-04-01/delegationset/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteReusableDelegationSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteReusableDelegationSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        DeleteReusableDelegationSetResponseDeserializer::deserialize(
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

    /// <p>Deletes a traffic policy.</p>
    #[allow(unused_variables, warnings)]
    fn delete_traffic_policy(
        &self,
        input: DeleteTrafficPolicyRequest,
    ) -> RusotoFuture<DeleteTrafficPolicyResponse, DeleteTrafficPolicyError> {
        let request_uri = format!(
            "/2013-04-01/trafficpolicy/{id}/{version}",
            id = input.id,
            version = input.version
        );

        let mut request = SignedRequest::new("DELETE", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTrafficPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteTrafficPolicyResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(DeleteTrafficPolicyResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes a traffic policy instance and all of the resource record sets that Amazon Route 53 created when you created the instance.</p> <note> <p>In the Amazon Route 53 console, traffic policy instances are known as policy records.</p> </note></p>
    #[allow(unused_variables, warnings)]
    fn delete_traffic_policy_instance(
        &self,
        input: DeleteTrafficPolicyInstanceRequest,
    ) -> RusotoFuture<DeleteTrafficPolicyInstanceResponse, DeleteTrafficPolicyInstanceError> {
        let request_uri = format!("/2013-04-01/trafficpolicyinstance/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTrafficPolicyInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteTrafficPolicyInstanceResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        DeleteTrafficPolicyInstanceResponseDeserializer::deserialize(
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

    /// <p><p>Removes authorization to submit an <code>AssociateVPCWithHostedZone</code> request to associate a specified VPC with a hosted zone that was created by a different account. You must use the account that created the hosted zone to submit a <code>DeleteVPCAssociationAuthorization</code> request.</p> <important> <p>Sending this request only prevents the AWS account that created the VPC from associating the VPC with the Amazon Route 53 hosted zone in the future. If the VPC is already associated with the hosted zone, <code>DeleteVPCAssociationAuthorization</code> won&#39;t disassociate the VPC from the hosted zone. If you want to delete an existing association, use <code>DisassociateVPCFromHostedZone</code>.</p> </important></p>
    #[allow(unused_variables, warnings)]
    fn delete_vpc_association_authorization(
        &self,
        input: DeleteVPCAssociationAuthorizationRequest,
    ) -> RusotoFuture<
        DeleteVPCAssociationAuthorizationResponse,
        DeleteVPCAssociationAuthorizationError,
    > {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/deauthorizevpcassociation",
            id = input.hosted_zone_id
        );

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        DeleteVPCAssociationAuthorizationRequestSerializer::serialize(
            &mut writer,
            "DeleteVPCAssociationAuthorizationRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVPCAssociationAuthorizationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteVPCAssociationAuthorizationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        DeleteVPCAssociationAuthorizationResponseDeserializer::deserialize(
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

    /// <p><p>Disassociates a VPC from a Amazon Route 53 private hosted zone. </p> <note> <p>You can&#39;t disassociate the last VPC from a private hosted zone.</p> </note> <important> <p>You can&#39;t disassociate a VPC from a private hosted zone when only one VPC is associated with the hosted zone. You also can&#39;t convert a private hosted zone into a public hosted zone.</p> </important></p>
    #[allow(unused_variables, warnings)]
    fn disassociate_vpc_from_hosted_zone(
        &self,
        input: DisassociateVPCFromHostedZoneRequest,
    ) -> RusotoFuture<DisassociateVPCFromHostedZoneResponse, DisassociateVPCFromHostedZoneError>
    {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/disassociatevpc",
            id = input.hosted_zone_id
        );

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        DisassociateVPCFromHostedZoneRequestSerializer::serialize(
            &mut writer,
            "DisassociateVPCFromHostedZoneRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateVPCFromHostedZoneError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DisassociateVPCFromHostedZoneResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        DisassociateVPCFromHostedZoneResponseDeserializer::deserialize(
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

    /// <p>Gets the specified limit for the current account, for example, the maximum number of health checks that you can create using the account.</p> <p>For the default limit, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>. To request a higher limit, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-route53">open a case</a>.</p>
    #[allow(unused_variables, warnings)]
    fn get_account_limit(
        &self,
        input: GetAccountLimitRequest,
    ) -> RusotoFuture<GetAccountLimitResponse, GetAccountLimitError> {
        let request_uri = format!("/2013-04-01/accountlimit/{type}", type = input.type_);

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetAccountLimitError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetAccountLimitResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetAccountLimitResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns the current status of a change batch request. The status is one of the following values:</p> <ul> <li> <p> <code>PENDING</code> indicates that the changes in this request have not propagated to all Amazon Route 53 DNS servers. This is the initial status of all change batch requests.</p> </li> <li> <p> <code>INSYNC</code> indicates that the changes have propagated to all Amazon Route 53 DNS servers. </p> </li> </ul></p>
    #[allow(unused_variables, warnings)]
    fn get_change(
        &self,
        input: GetChangeRequest,
    ) -> RusotoFuture<GetChangeResponse, GetChangeError> {
        let request_uri = format!("/2013-04-01/change/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetChangeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetChangeResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetChangeResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p> <code>GetCheckerIpRanges</code> still works, but we recommend that you download ip-ranges.json, which includes IP address ranges for all AWS services. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/route-53-ip-addresses.html">IP Address Ranges of Amazon Route 53 Servers</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    fn get_checker_ip_ranges(
        &self,
        input: GetCheckerIpRangesRequest,
    ) -> RusotoFuture<GetCheckerIpRangesResponse, GetCheckerIpRangesError> {
        let request_uri = "/2013-04-01/checkeripranges";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetCheckerIpRangesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetCheckerIpRangesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetCheckerIpRangesResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about whether a specified geographic location is supported for Amazon Route 53 geolocation resource record sets.</p> <p>Use the following syntax to determine whether a continent is supported for geolocation:</p> <p> <code>GET /2013-04-01/geolocation?ContinentCode=<i>two-letter abbreviation for a continent</i> </code> </p> <p>Use the following syntax to determine whether a country is supported for geolocation:</p> <p> <code>GET /2013-04-01/geolocation?CountryCode=<i>two-character country code</i> </code> </p> <p>Use the following syntax to determine whether a subdivision of a country is supported for geolocation:</p> <p> <code>GET /2013-04-01/geolocation?CountryCode=<i>two-character country code</i>&amp;SubdivisionCode=<i>subdivision code</i> </code> </p>
    #[allow(unused_variables, warnings)]
    fn get_geo_location(
        &self,
        input: GetGeoLocationRequest,
    ) -> RusotoFuture<GetGeoLocationResponse, GetGeoLocationError> {
        let request_uri = "/2013-04-01/geolocation";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.continent_code {
            params.put("continentcode", x);
        }
        if let Some(ref x) = input.country_code {
            params.put("countrycode", x);
        }
        if let Some(ref x) = input.subdivision_code {
            params.put("subdivisioncode", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetGeoLocationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetGeoLocationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetGeoLocationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about a specified health check.</p>
    #[allow(unused_variables, warnings)]
    fn get_health_check(
        &self,
        input: GetHealthCheckRequest,
    ) -> RusotoFuture<GetHealthCheckResponse, GetHealthCheckError> {
        let request_uri = format!(
            "/2013-04-01/healthcheck/{health_check_id}",
            health_check_id = input.health_check_id
        );

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetHealthCheckError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetHealthCheckResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetHealthCheckResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the number of health checks that are associated with the current AWS account.</p>
    #[allow(unused_variables, warnings)]
    fn get_health_check_count(
        &self,
        input: GetHealthCheckCountRequest,
    ) -> RusotoFuture<GetHealthCheckCountResponse, GetHealthCheckCountError> {
        let request_uri = "/2013-04-01/healthcheckcount";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetHealthCheckCountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetHealthCheckCountResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetHealthCheckCountResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the reason that a specified health check failed most recently.</p>
    #[allow(unused_variables, warnings)]
    fn get_health_check_last_failure_reason(
        &self,
        input: GetHealthCheckLastFailureReasonRequest,
    ) -> RusotoFuture<GetHealthCheckLastFailureReasonResponse, GetHealthCheckLastFailureReasonError>
    {
        let request_uri = format!(
            "/2013-04-01/healthcheck/{health_check_id}/lastfailurereason",
            health_check_id = input.health_check_id
        );

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetHealthCheckLastFailureReasonError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetHealthCheckLastFailureReasonResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        GetHealthCheckLastFailureReasonResponseDeserializer::deserialize(
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

    /// <p>Gets status of a specified health check. </p>
    #[allow(unused_variables, warnings)]
    fn get_health_check_status(
        &self,
        input: GetHealthCheckStatusRequest,
    ) -> RusotoFuture<GetHealthCheckStatusResponse, GetHealthCheckStatusError> {
        let request_uri = format!(
            "/2013-04-01/healthcheck/{health_check_id}/status",
            health_check_id = input.health_check_id
        );

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetHealthCheckStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetHealthCheckStatusResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetHealthCheckStatusResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about a specified hosted zone including the four name servers assigned to the hosted zone.</p>
    #[allow(unused_variables, warnings)]
    fn get_hosted_zone(
        &self,
        input: GetHostedZoneRequest,
    ) -> RusotoFuture<GetHostedZoneResponse, GetHostedZoneError> {
        let request_uri = format!("/2013-04-01/hostedzone/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetHostedZoneError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetHostedZoneResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetHostedZoneResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the number of hosted zones that are associated with the current AWS account.</p>
    #[allow(unused_variables, warnings)]
    fn get_hosted_zone_count(
        &self,
        input: GetHostedZoneCountRequest,
    ) -> RusotoFuture<GetHostedZoneCountResponse, GetHostedZoneCountError> {
        let request_uri = "/2013-04-01/hostedzonecount";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetHostedZoneCountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetHostedZoneCountResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetHostedZoneCountResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the specified limit for a specified hosted zone, for example, the maximum number of records that you can create in the hosted zone. </p> <p>For the default limit, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>. To request a higher limit, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-route53">open a case</a>.</p>
    #[allow(unused_variables, warnings)]
    fn get_hosted_zone_limit(
        &self,
        input: GetHostedZoneLimitRequest,
    ) -> RusotoFuture<GetHostedZoneLimitResponse, GetHostedZoneLimitError> {
        let request_uri = format!("/2013-04-01/hostedzonelimit/{id}/{type}", id = input.hosted_zone_id, type = input.type_);

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetHostedZoneLimitError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetHostedZoneLimitResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetHostedZoneLimitResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about a specified configuration for DNS query logging.</p> <p>For more information about DNS query logs, see <a>CreateQueryLoggingConfig</a> and <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html">Logging DNS Queries</a>.</p>
    #[allow(unused_variables, warnings)]
    fn get_query_logging_config(
        &self,
        input: GetQueryLoggingConfigRequest,
    ) -> RusotoFuture<GetQueryLoggingConfigResponse, GetQueryLoggingConfigError> {
        let request_uri = format!("/2013-04-01/queryloggingconfig/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetQueryLoggingConfigError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetQueryLoggingConfigResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetQueryLoggingConfigResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves information about a specified reusable delegation set, including the four name servers that are assigned to the delegation set.</p>
    #[allow(unused_variables, warnings)]
    fn get_reusable_delegation_set(
        &self,
        input: GetReusableDelegationSetRequest,
    ) -> RusotoFuture<GetReusableDelegationSetResponse, GetReusableDelegationSetError> {
        let request_uri = format!("/2013-04-01/delegationset/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetReusableDelegationSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetReusableDelegationSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetReusableDelegationSetResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p> <p>For the default limit, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>. To request a higher limit, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-route53">open a case</a>.</p>
    #[allow(unused_variables, warnings)]
    fn get_reusable_delegation_set_limit(
        &self,
        input: GetReusableDelegationSetLimitRequest,
    ) -> RusotoFuture<GetReusableDelegationSetLimitResponse, GetReusableDelegationSetLimitError>
    {
        let request_uri = format!("/2013-04-01/reusabledelegationsetlimit/{id}/{type}", id = input.delegation_set_id, type = input.type_);

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetReusableDelegationSetLimitError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetReusableDelegationSetLimitResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        GetReusableDelegationSetLimitResponseDeserializer::deserialize(
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

    /// <p>Gets information about a specific traffic policy version.</p>
    #[allow(unused_variables, warnings)]
    fn get_traffic_policy(
        &self,
        input: GetTrafficPolicyRequest,
    ) -> RusotoFuture<GetTrafficPolicyResponse, GetTrafficPolicyError> {
        let request_uri = format!(
            "/2013-04-01/trafficpolicy/{id}/{version}",
            id = input.id,
            version = input.version
        );

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetTrafficPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetTrafficPolicyResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetTrafficPolicyResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Gets information about a specified traffic policy instance.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there&#39;s a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <note> <p>In the Amazon Route 53 console, traffic policy instances are known as policy records.</p> </note></p>
    #[allow(unused_variables, warnings)]
    fn get_traffic_policy_instance(
        &self,
        input: GetTrafficPolicyInstanceRequest,
    ) -> RusotoFuture<GetTrafficPolicyInstanceResponse, GetTrafficPolicyInstanceError> {
        let request_uri = format!("/2013-04-01/trafficpolicyinstance/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetTrafficPolicyInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetTrafficPolicyInstanceResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetTrafficPolicyInstanceResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the number of traffic policy instances that are associated with the current AWS account.</p>
    #[allow(unused_variables, warnings)]
    fn get_traffic_policy_instance_count(
        &self,
        input: GetTrafficPolicyInstanceCountRequest,
    ) -> RusotoFuture<GetTrafficPolicyInstanceCountResponse, GetTrafficPolicyInstanceCountError>
    {
        let request_uri = "/2013-04-01/trafficpolicyinstancecount";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetTrafficPolicyInstanceCountError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetTrafficPolicyInstanceCountResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        GetTrafficPolicyInstanceCountResponseDeserializer::deserialize(
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

    /// <p>Retrieves a list of supported geo locations.</p> <p>Countries are listed first, and continents are listed last. If Amazon Route 53 supports subdivisions for a country (for example, states or provinces), the subdivisions for that country are listed in alphabetical order immediately after the corresponding country.</p>
    #[allow(unused_variables, warnings)]
    fn list_geo_locations(
        &self,
        input: ListGeoLocationsRequest,
    ) -> RusotoFuture<ListGeoLocationsResponse, ListGeoLocationsError> {
        let request_uri = "/2013-04-01/geolocations";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.max_items {
            params.put("maxitems", x);
        }
        if let Some(ref x) = input.start_continent_code {
            params.put("startcontinentcode", x);
        }
        if let Some(ref x) = input.start_country_code {
            params.put("startcountrycode", x);
        }
        if let Some(ref x) = input.start_subdivision_code {
            params.put("startsubdivisioncode", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListGeoLocationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListGeoLocationsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListGeoLocationsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieve a list of the health checks that are associated with the current AWS account. </p>
    #[allow(unused_variables, warnings)]
    fn list_health_checks(
        &self,
        input: ListHealthChecksRequest,
    ) -> RusotoFuture<ListHealthChecksResponse, ListHealthChecksError> {
        let request_uri = "/2013-04-01/healthcheck";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("maxitems", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListHealthChecksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListHealthChecksResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListHealthChecksResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves a list of the public and private hosted zones that are associated with the current AWS account. The response includes a <code>HostedZones</code> child element for each hosted zone.</p> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of hosted zones, you can use the <code>maxitems</code> parameter to list them in groups of up to 100.</p>
    #[allow(unused_variables, warnings)]
    fn list_hosted_zones(
        &self,
        input: ListHostedZonesRequest,
    ) -> RusotoFuture<ListHostedZonesResponse, ListHostedZonesError> {
        let request_uri = "/2013-04-01/hostedzone";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.delegation_set_id {
            params.put("delegationsetid", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("maxitems", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListHostedZonesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListHostedZonesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListHostedZonesResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Retrieves a list of your hosted zones in lexicographic order. The response includes a <code>HostedZones</code> child element for each hosted zone created by the current AWS account. </p> <p> <code>ListHostedZonesByName</code> sorts hosted zones by name with the labels reversed. For example:</p> <p> <code>com.example.www.</code> </p> <p>Note the trailing dot, which can change the sort order in some circumstances.</p> <p>If the domain name includes escape characters or Punycode, <code>ListHostedZonesByName</code> alphabetizes the domain name using the escaped or Punycoded value, which is the format that Amazon Route 53 saves in its database. For example, to create a hosted zone for exämple.com, you specify ex\344mple.com for the domain name. <code>ListHostedZonesByName</code> alphabetizes it as:</p> <p> <code>com.ex\344mple.</code> </p> <p>The labels are reversed and alphabetized using the escaped value. For more information about valid domain name formats, including internationalized domain names, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DomainNameFormat.html">DNS Domain Name Format</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>Amazon Route 53 returns up to 100 items in each response. If you have a lot of hosted zones, use the <code>MaxItems</code> parameter to list them in groups of up to 100. The response includes values that help navigate from one group of <code>MaxItems</code> hosted zones to the next:</p> <ul> <li> <p>The <code>DNSName</code> and <code>HostedZoneId</code> elements in the response contain the values, if any, specified for the <code>dnsname</code> and <code>hostedzoneid</code> parameters in the request that produced the current response.</p> </li> <li> <p>The <code>MaxItems</code> element in the response contains the value, if any, that you specified for the <code>maxitems</code> parameter in the request that produced the current response.</p> </li> <li> <p>If the value of <code>IsTruncated</code> in the response is true, there are more hosted zones associated with the current AWS account. </p> <p>If <code>IsTruncated</code> is false, this response includes the last hosted zone that is associated with the current account. The <code>NextDNSName</code> element and <code>NextHostedZoneId</code> elements are omitted from the response.</p> </li> <li> <p>The <code>NextDNSName</code> and <code>NextHostedZoneId</code> elements in the response contain the domain name and the hosted zone ID of the next hosted zone that is associated with the current AWS account. If you want to list more hosted zones, make another call to <code>ListHostedZonesByName</code>, and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p> </li> </ul></p>
    #[allow(unused_variables, warnings)]
    fn list_hosted_zones_by_name(
        &self,
        input: ListHostedZonesByNameRequest,
    ) -> RusotoFuture<ListHostedZonesByNameResponse, ListHostedZonesByNameError> {
        let request_uri = "/2013-04-01/hostedzonesbyname";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.dns_name {
            params.put("dnsname", x);
        }
        if let Some(ref x) = input.hosted_zone_id {
            params.put("hostedzoneid", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("maxitems", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListHostedZonesByNameError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListHostedZonesByNameResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListHostedZonesByNameResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the configurations for DNS query logging that are associated with the current AWS account or the configuration that is associated with a specified hosted zone.</p> <p>For more information about DNS query logs, see <a>CreateQueryLoggingConfig</a>. Additional information, including the format of DNS query logs, appears in <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html">Logging DNS Queries</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    fn list_query_logging_configs(
        &self,
        input: ListQueryLoggingConfigsRequest,
    ) -> RusotoFuture<ListQueryLoggingConfigsResponse, ListQueryLoggingConfigsError> {
        let request_uri = "/2013-04-01/queryloggingconfig";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.hosted_zone_id {
            params.put("hostedzoneid", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxresults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nexttoken", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListQueryLoggingConfigsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListQueryLoggingConfigsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListQueryLoggingConfigsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the resource record sets in a specified hosted zone.</p> <p> <code>ListResourceRecordSets</code> returns up to 100 resource record sets at a time in ASCII order, beginning at a position specified by the <code>name</code> and <code>type</code> elements. The action sorts results first by DNS name with the labels reversed, for example:</p> <p> <code>com.example.www.</code> </p> <p>Note the trailing dot, which can change the sort order in some circumstances.</p> <p>When multiple records have the same DNS name, the action sorts results by the record type.</p> <p>You can use the name and type elements to adjust the beginning position of the list of resource record sets returned:</p> <dl> <dt>If you do not specify Name or Type</dt> <dd> <p>The results begin with the first resource record set that the hosted zone contains.</p> </dd> <dt>If you specify Name but not Type</dt> <dd> <p>The results begin with the first resource record set in the list whose name is greater than or equal to <code>Name</code>.</p> </dd> <dt>If you specify Type but not Name</dt> <dd> <p>Amazon Route 53 returns the <code>InvalidInput</code> error.</p> </dd> <dt>If you specify both Name and Type</dt> <dd> <p>The results begin with the first resource record set in the list whose name is greater than or equal to <code>Name</code>, and whose type is greater than or equal to <code>Type</code>.</p> </dd> </dl> <p>This action returns the most current version of the records. This includes records that are <code>PENDING</code>, and that are not yet available on all Amazon Route 53 DNS servers.</p> <p>To ensure that you get an accurate listing of the resource record sets for a hosted zone at a point in time, do not submit a <code>ChangeResourceRecordSets</code> request while you're paging through the results of a <code>ListResourceRecordSets</code> request. If you do, some pages may display results without the latest changes while other pages display results with the latest changes.</p>
    #[allow(unused_variables, warnings)]
    fn list_resource_record_sets(
        &self,
        input: ListResourceRecordSetsRequest,
    ) -> RusotoFuture<ListResourceRecordSetsResponse, ListResourceRecordSetsError> {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/rrset",
            id = input.hosted_zone_id
        );

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.max_items {
            params.put("maxitems", x);
        }
        if let Some(ref x) = input.start_record_identifier {
            params.put("identifier", x);
        }
        if let Some(ref x) = input.start_record_name {
            params.put("name", x);
        }
        if let Some(ref x) = input.start_record_type {
            params.put("type", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListResourceRecordSetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListResourceRecordSetsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListResourceRecordSetsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves a list of the reusable delegation sets that are associated with the current AWS account.</p>
    #[allow(unused_variables, warnings)]
    fn list_reusable_delegation_sets(
        &self,
        input: ListReusableDelegationSetsRequest,
    ) -> RusotoFuture<ListReusableDelegationSetsResponse, ListReusableDelegationSetsError> {
        let request_uri = "/2013-04-01/delegationset";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("maxitems", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListReusableDelegationSetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListReusableDelegationSetsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListReusableDelegationSetsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists tags for one health check or hosted zone. </p> <p>For information about using tags for cost allocation, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let request_uri = format!(
            "/2013-04-01/tags/{resource_type}/{resource_id}",
            resource_id = input.resource_id,
            resource_type = input.resource_type
        );

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

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
                    result = ListTagsForResourceResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListTagsForResourceResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists tags for up to 10 health checks or hosted zones.</p> <p>For information about using tags for cost allocation, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    fn list_tags_for_resources(
        &self,
        input: ListTagsForResourcesRequest,
    ) -> RusotoFuture<ListTagsForResourcesResponse, ListTagsForResourcesError> {
        let request_uri = format!(
            "/2013-04-01/tags/{resource_type}",
            resource_type = input.resource_type
        );

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        ListTagsForResourcesRequestSerializer::serialize(
            &mut writer,
            "ListTagsForResourcesRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsForResourcesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListTagsForResourcesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListTagsForResourcesResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the latest version for every traffic policy that is associated with the current AWS account. Policies are listed in the order in which they were created. </p>
    #[allow(unused_variables, warnings)]
    fn list_traffic_policies(
        &self,
        input: ListTrafficPoliciesRequest,
    ) -> RusotoFuture<ListTrafficPoliciesResponse, ListTrafficPoliciesError> {
        let request_uri = "/2013-04-01/trafficpolicies";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.max_items {
            params.put("maxitems", x);
        }
        if let Some(ref x) = input.traffic_policy_id_marker {
            params.put("trafficpolicyid", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTrafficPoliciesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListTrafficPoliciesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListTrafficPoliciesResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the traffic policy instances that you created by using the current AWS account.</p> <note> <p>After you submit an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p>
    #[allow(unused_variables, warnings)]
    fn list_traffic_policy_instances(
        &self,
        input: ListTrafficPolicyInstancesRequest,
    ) -> RusotoFuture<ListTrafficPolicyInstancesResponse, ListTrafficPolicyInstancesError> {
        let request_uri = "/2013-04-01/trafficpolicyinstances";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.hosted_zone_id_marker {
            params.put("hostedzoneid", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("maxitems", x);
        }
        if let Some(ref x) = input.traffic_policy_instance_name_marker {
            params.put("trafficpolicyinstancename", x);
        }
        if let Some(ref x) = input.traffic_policy_instance_type_marker {
            params.put("trafficpolicyinstancetype", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTrafficPolicyInstancesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListTrafficPolicyInstancesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListTrafficPolicyInstancesResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the traffic policy instances that you created in a specified hosted zone.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p>
    #[allow(unused_variables, warnings)]
    fn list_traffic_policy_instances_by_hosted_zone(
        &self,
        input: ListTrafficPolicyInstancesByHostedZoneRequest,
    ) -> RusotoFuture<
        ListTrafficPolicyInstancesByHostedZoneResponse,
        ListTrafficPolicyInstancesByHostedZoneError,
    > {
        let request_uri = "/2013-04-01/trafficpolicyinstances/hostedzone";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.hosted_zone_id);
        if let Some(ref x) = input.max_items {
            params.put("maxitems", x);
        }
        if let Some(ref x) = input.traffic_policy_instance_name_marker {
            params.put("trafficpolicyinstancename", x);
        }
        if let Some(ref x) = input.traffic_policy_instance_type_marker {
            params.put("trafficpolicyinstancetype", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTrafficPolicyInstancesByHostedZoneError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListTrafficPolicyInstancesByHostedZoneResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        ListTrafficPolicyInstancesByHostedZoneResponseDeserializer::deserialize(
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

    /// <p>Gets information about the traffic policy instances that you created by using a specify traffic policy version.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p>
    #[allow(unused_variables, warnings)]
    fn list_traffic_policy_instances_by_policy(
        &self,
        input: ListTrafficPolicyInstancesByPolicyRequest,
    ) -> RusotoFuture<
        ListTrafficPolicyInstancesByPolicyResponse,
        ListTrafficPolicyInstancesByPolicyError,
    > {
        let request_uri = "/2013-04-01/trafficpolicyinstances/trafficpolicy";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.hosted_zone_id_marker {
            params.put("hostedzoneid", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("maxitems", x);
        }
        params.put("id", &input.traffic_policy_id);
        if let Some(ref x) = input.traffic_policy_instance_name_marker {
            params.put("trafficpolicyinstancename", x);
        }
        if let Some(ref x) = input.traffic_policy_instance_type_marker {
            params.put("trafficpolicyinstancetype", x);
        }
        params.put("version", &input.traffic_policy_version);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTrafficPolicyInstancesByPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListTrafficPolicyInstancesByPolicyResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        ListTrafficPolicyInstancesByPolicyResponseDeserializer::deserialize(
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

    /// <p>Gets information about all of the versions for a specified traffic policy.</p> <p>Traffic policy versions are listed in numerical order by <code>VersionNumber</code>.</p>
    #[allow(unused_variables, warnings)]
    fn list_traffic_policy_versions(
        &self,
        input: ListTrafficPolicyVersionsRequest,
    ) -> RusotoFuture<ListTrafficPolicyVersionsResponse, ListTrafficPolicyVersionsError> {
        let request_uri = format!("/2013-04-01/trafficpolicies/{id}/versions", id = input.id);

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.max_items {
            params.put("maxitems", x);
        }
        if let Some(ref x) = input.traffic_policy_version_marker {
            params.put("trafficpolicyversion", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTrafficPolicyVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListTrafficPolicyVersionsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListTrafficPolicyVersionsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets a list of the VPCs that were created by other accounts and that can be associated with a specified hosted zone because you've submitted one or more <code>CreateVPCAssociationAuthorization</code> requests. </p> <p>The response includes a <code>VPCs</code> element with a <code>VPC</code> child element for each VPC that can be associated with the hosted zone.</p>
    #[allow(unused_variables, warnings)]
    fn list_vpc_association_authorizations(
        &self,
        input: ListVPCAssociationAuthorizationsRequest,
    ) -> RusotoFuture<ListVPCAssociationAuthorizationsResponse, ListVPCAssociationAuthorizationsError>
    {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/authorizevpcassociation",
            id = input.hosted_zone_id
        );

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxresults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nexttoken", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListVPCAssociationAuthorizationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListVPCAssociationAuthorizationsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        ListVPCAssociationAuthorizationsResponseDeserializer::deserialize(
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

    /// <p>Gets the value that Amazon Route 53 returns in response to a DNS request for a specified record name and type. You can optionally specify the IP address of a DNS resolver, an EDNS0 client subnet IP address, and a subnet mask. </p>
    #[allow(unused_variables, warnings)]
    fn test_dns_answer(
        &self,
        input: TestDNSAnswerRequest,
    ) -> RusotoFuture<TestDNSAnswerResponse, TestDNSAnswerError> {
        let request_uri = "/2013-04-01/testdnsanswer";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.edns0_client_subnet_ip {
            params.put("edns0clientsubnetip", x);
        }
        if let Some(ref x) = input.edns0_client_subnet_mask {
            params.put("edns0clientsubnetmask", x);
        }
        params.put("hostedzoneid", &input.hosted_zone_id);
        params.put("recordname", &input.record_name);
        params.put("recordtype", &input.record_type);
        if let Some(ref x) = input.resolver_ip {
            params.put("resolverip", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TestDNSAnswerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = TestDNSAnswerResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(TestDNSAnswerResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates an existing health check. Note that some values can't be updated. </p> <p>For more information about updating health checks, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/health-checks-creating-deleting.html">Creating, Updating, and Deleting Health Checks</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    fn update_health_check(
        &self,
        input: UpdateHealthCheckRequest,
    ) -> RusotoFuture<UpdateHealthCheckResponse, UpdateHealthCheckError> {
        let request_uri = format!(
            "/2013-04-01/healthcheck/{health_check_id}",
            health_check_id = input.health_check_id
        );

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        UpdateHealthCheckRequestSerializer::serialize(
            &mut writer,
            "UpdateHealthCheckRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateHealthCheckError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = UpdateHealthCheckResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(UpdateHealthCheckResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the comment for a specified hosted zone.</p>
    #[allow(unused_variables, warnings)]
    fn update_hosted_zone_comment(
        &self,
        input: UpdateHostedZoneCommentRequest,
    ) -> RusotoFuture<UpdateHostedZoneCommentResponse, UpdateHostedZoneCommentError> {
        let request_uri = format!("/2013-04-01/hostedzone/{id}", id = input.id);

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        UpdateHostedZoneCommentRequestSerializer::serialize(
            &mut writer,
            "UpdateHostedZoneCommentRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateHostedZoneCommentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = UpdateHostedZoneCommentResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(UpdateHostedZoneCommentResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the comment for a specified traffic policy version.</p>
    #[allow(unused_variables, warnings)]
    fn update_traffic_policy_comment(
        &self,
        input: UpdateTrafficPolicyCommentRequest,
    ) -> RusotoFuture<UpdateTrafficPolicyCommentResponse, UpdateTrafficPolicyCommentError> {
        let request_uri = format!(
            "/2013-04-01/trafficpolicy/{id}/{version}",
            id = input.id,
            version = input.version
        );

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        UpdateTrafficPolicyCommentRequestSerializer::serialize(
            &mut writer,
            "UpdateTrafficPolicyCommentRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateTrafficPolicyCommentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = UpdateTrafficPolicyCommentResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(UpdateTrafficPolicyCommentResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack
                    ));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Updates the resource record sets in a specified hosted zone that were created based on the settings in a specified traffic policy version.</p> <p>When you update a traffic policy instance, Amazon Route 53 continues to respond to DNS queries for the root resource record set name (such as example.com) while it replaces one group of resource record sets with another. Amazon Route 53 performs the following operations:</p> <ol> <li> <p>Amazon Route 53 creates a new group of resource record sets based on the specified traffic policy. This is true regardless of how significant the differences are between the existing resource record sets and the new resource record sets. </p> </li> <li> <p>When all of the new resource record sets have been created, Amazon Route 53 starts to respond to DNS queries for the root resource record set name (such as example.com) by using the new resource record sets.</p> </li> <li> <p>Amazon Route 53 deletes the old group of resource record sets that are associated with the root resource record set name.</p> </li> </ol></p>
    #[allow(unused_variables, warnings)]
    fn update_traffic_policy_instance(
        &self,
        input: UpdateTrafficPolicyInstanceRequest,
    ) -> RusotoFuture<UpdateTrafficPolicyInstanceResponse, UpdateTrafficPolicyInstanceError> {
        let request_uri = format!("/2013-04-01/trafficpolicyinstance/{id}", id = input.id);

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        UpdateTrafficPolicyInstanceRequestSerializer::serialize(
            &mut writer,
            "UpdateTrafficPolicyInstanceRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK
                && response.status != StatusCode::NO_CONTENT
                && response.status != StatusCode::PARTIAL_CONTENT
            {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateTrafficPolicyInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = UpdateTrafficPolicyInstanceResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(
                        UpdateTrafficPolicyInstanceResponseDeserializer::deserialize(
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
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[test]
    fn test_parse_error_route_53_get_hosted_zone() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "route53-get-hosted-zone.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = Route53Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetHostedZoneRequest::default();
        let result = client.get_hosted_zone(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

}
