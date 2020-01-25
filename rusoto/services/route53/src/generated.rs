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

/// <p>A complex type that contains the type of limit that you specified in the request and the current value for that limit.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AccountLimit {
    /// <p><p>The limit that you requested. Valid values include the following:</p> <ul> <li> <p> <b>MAX<em>HEALTH</em>CHECKS<em>BY</em>OWNER</b>: The maximum number of health checks that you can create using the current account.</p> </li> <li> <p> <b>MAX<em>HOSTED</em>ZONES<em>BY</em>OWNER</b>: The maximum number of hosted zones that you can create using the current account.</p> </li> <li> <p> <b>MAX<em>REUSABLE</em>DELEGATION<em>SETS</em>BY<em>OWNER</b>: The maximum number of reusable delegation sets that you can create using the current account.</p> </li> <li> <p> <b>MAX</em>TRAFFIC<em>POLICIES</em>BY<em>OWNER</b>: The maximum number of traffic policies that you can create using the current account.</p> </li> <li> <p> <b>MAX</em>TRAFFIC<em>POLICY</em>INSTANCES<em>BY</em>OWNER</b>: The maximum number of traffic policy instances that you can create using the current account. (Traffic policy instances are referred to as traffic flow policy records in the Amazon Route 53 console.)</p> </li> </ul></p>
    pub type_: String,
    /// <p>The current value for the limit that is specified by <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_AccountLimit.html#Route53-Type-AccountLimit-Type">Type</a>.</p>
    pub value: i64,
}

struct AccountLimitDeserializer;
impl AccountLimitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccountLimit, XmlParseError> {
        deserialize_elements::<_, AccountLimit, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Type" => {
                    obj.type_ = AccountLimitTypeDeserializer::deserialize("Type", stack)?;
                }
                "Value" => {
                    obj.value = LimitValueDeserializer::deserialize("Value", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct AccountLimitTypeDeserializer;
impl AccountLimitTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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

/// <p>A complex type that identifies the CloudWatch alarm that you want Amazon Route 53 health checkers to use to determine whether the specified health check is healthy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AlarmIdentifier {
    /// <p><p>The name of the CloudWatch alarm that you want Amazon Route 53 health checkers to use to determine whether this health check is healthy.</p> <note> <p>Route 53 supports CloudWatch alarms with the following features:</p> <ul> <li> <p>Standard-resolution metrics. High-resolution metrics aren&#39;t supported. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/publishingMetrics.html#high-resolution-metrics">High-Resolution Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p> </li> <li> <p>Statistics: Average, Minimum, Maximum, Sum, and SampleCount. Extended statistics aren&#39;t supported.</p> </li> </ul> </note></p>
    pub name: String,
    /// <p>For the CloudWatch alarm that you want Route 53 health checkers to use to determine whether this health check is healthy, the region that the alarm was created in.</p> <p>For the current list of CloudWatch regions, see <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#cw_region">Amazon CloudWatch</a> in the <i>AWS Regions and Endpoints</i> chapter of the <i>Amazon Web Services General Reference</i>.</p>
    pub region: String,
}

struct AlarmIdentifierDeserializer;
impl AlarmIdentifierDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AlarmIdentifier, XmlParseError> {
        deserialize_elements::<_, AlarmIdentifier, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Name" => {
                    obj.name = AlarmNameDeserializer::deserialize("Name", stack)?;
                }
                "Region" => {
                    obj.region = CloudWatchRegionDeserializer::deserialize("Region", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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

/// <p><p> <i>Alias resource record sets only:</i> Information about the AWS resource, such as a CloudFront distribution or an Amazon S3 bucket, that you want to route traffic to.</p> <p>When creating resource record sets for a private hosted zone, note the following:</p> <ul> <li> <p>Creating geolocation alias resource record sets or latency alias resource record sets in a private hosted zone is unsupported.</p> </li> <li> <p>For information about creating failover resource record sets in a private hosted zone, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-private-hosted-zones.html">Configuring Failover in a Private Hosted Zone</a>.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AliasTarget {
    /// <p><p> <i>Alias resource record sets only:</i> The value that you specify depends on where you want to route queries:</p> <dl> <dt>Amazon API Gateway custom regional APIs and edge-optimized APIs</dt> <dd> <p>Specify the applicable domain name for your API. You can get the applicable value using the AWS CLI command <a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-domain-names.html">get-domain-names</a>:</p> <ul> <li> <p>For regional APIs, specify the value of <code>regionalDomainName</code>.</p> </li> <li> <p>For edge-optimized APIs, specify the value of <code>distributionDomainName</code>. This is the name of the associated CloudFront distribution, such as <code>da1b2c3d4e5.cloudfront.net</code>.</p> </li> </ul> <note> <p>The name of the record that you&#39;re creating must match a custom domain name for your API, such as <code>api.example.com</code>.</p> </note> </dd> <dt>Amazon Virtual Private Cloud interface VPC endpoint</dt> <dd> <p>Enter the API endpoint for the interface endpoint, such as <code>vpce-123456789abcdef01-example-us-east-1a.elasticloadbalancing.us-east-1.vpce.amazonaws.com</code>. For edge-optimized APIs, this is the domain name for the corresponding CloudFront distribution. You can get the value of <code>DnsName</code> using the AWS CLI command <a href="https://docs.aws.amazon.com/cli/latest/reference/ec2/describe-vpc-endpoints.html">describe-vpc-endpoints</a>.</p> </dd> <dt>CloudFront distribution</dt> <dd> <p>Specify the domain name that CloudFront assigned when you created your distribution.</p> <p>Your CloudFront distribution must include an alternate domain name that matches the name of the resource record set. For example, if the name of the resource record set is <i>acme.example.com</i>, your CloudFront distribution must include <i>acme.example.com</i> as one of the alternate domain names. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/CNAMEs.html">Using Alternate Domain Names (CNAMEs)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> <p>You can&#39;t create a resource record set in a private hosted zone to route traffic to a CloudFront distribution.</p> <note> <p>For failover alias records, you can&#39;t specify a CloudFront distribution for both the primary and secondary records. A distribution must include an alternate domain name that matches the name of the record. However, the primary and secondary records have the same name, and you can&#39;t include the same alternate domain name in more than one distribution. </p> </note> </dd> <dt>Elastic Beanstalk environment</dt> <dd> <p>If the domain name for your Elastic Beanstalk environment includes the region that you deployed the environment in, you can create an alias record that routes traffic to the environment. For example, the domain name <code>my-environment.<i>us-west-2</i>.elasticbeanstalk.com</code> is a regionalized domain name. </p> <important> <p>For environments that were created before early 2016, the domain name doesn&#39;t include the region. To route traffic to these environments, you must create a CNAME record instead of an alias record. Note that you can&#39;t create a CNAME record for the root domain name. For example, if your domain name is example.com, you can create a record that routes traffic for acme.example.com to your Elastic Beanstalk environment, but you can&#39;t create a record that routes traffic for example.com to your Elastic Beanstalk environment.</p> </important> <p>For Elastic Beanstalk environments that have regionalized subdomains, specify the <code>CNAME</code> attribute for the environment. You can use the following methods to get the value of the CNAME attribute:</p> <ul> <li> <p> <i>AWS Management Console</i>: For information about how to get the value by using the console, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/customdomains.html">Using Custom Domains with AWS Elastic Beanstalk</a> in the <i>AWS Elastic Beanstalk Developer Guide</i>.</p> </li> <li> <p> <i>Elastic Beanstalk API</i>: Use the <code>DescribeEnvironments</code> action to get the value of the <code>CNAME</code> attribute. For more information, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/api/API_DescribeEnvironments.html">DescribeEnvironments</a> in the <i>AWS Elastic Beanstalk API Reference</i>.</p> </li> <li> <p> <i>AWS CLI</i>: Use the <code>describe-environments</code> command to get the value of the <code>CNAME</code> attribute. For more information, see <a href="http://docs.aws.amazon.com/cli/latest/reference/elasticbeanstalk/describe-environments.html">describe-environments</a> in the <i>AWS Command Line Interface Reference</i>.</p> </li> </ul> </dd> <dt>ELB load balancer</dt> <dd> <p>Specify the DNS name that is associated with the load balancer. Get the DNS name by using the AWS Management Console, the ELB API, or the AWS CLI. </p> <ul> <li> <p> <b>AWS Management Console</b>: Go to the EC2 page, choose <b>Load Balancers</b> in the navigation pane, choose the load balancer, choose the <b>Description</b> tab, and get the value of the <b>DNS name</b> field. </p> <p>If you&#39;re routing traffic to a Classic Load Balancer, get the value that begins with <b>dualstack</b>. If you&#39;re routing traffic to another type of load balancer, get the value that applies to the record type, A or AAAA.</p> </li> <li> <p> <b>Elastic Load Balancing API</b>: Use <code>DescribeLoadBalancers</code> to get the value of <code>DNSName</code>. For more information, see the applicable guide:</p> <ul> <li> <p>Classic Load Balancers: <a href="http://docs.aws.amazon.com/elasticloadbalancing/2012-06-01/APIReference/API_DescribeLoadBalancers.html">DescribeLoadBalancers</a> </p> </li> <li> <p>Application and Network Load Balancers: <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DescribeLoadBalancers.html">DescribeLoadBalancers</a> </p> </li> </ul> </li> <li> <p> <b>AWS CLI</b>: Use <code>describe-load-balancers</code> to get the value of <code>DNSName</code>. For more information, see the applicable guide:</p> <ul> <li> <p>Classic Load Balancers: <a href="http://docs.aws.amazon.com/cli/latest/reference/elb/describe-load-balancers.html">describe-load-balancers</a> </p> </li> <li> <p>Application and Network Load Balancers: <a href="http://docs.aws.amazon.com/cli/latest/reference/elbv2/describe-load-balancers.html">describe-load-balancers</a> </p> </li> </ul> </li> </ul> </dd> <dt>Amazon S3 bucket that is configured as a static website</dt> <dd> <p>Specify the domain name of the Amazon S3 website endpoint that you created the bucket in, for example, <code>s3-website.us-east-2.amazonaws.com</code>. For more information about valid values, see the table <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region">Amazon Simple Storage Service (S3) Website Endpoints</a> in the <i>Amazon Web Services General Reference</i>. For more information about using S3 buckets for websites, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/getting-started.html">Getting Started with Amazon Route 53</a> in the <i>Amazon Route 53 Developer Guide.</i> </p> </dd> <dt>Another Route 53 resource record set</dt> <dd> <p>Specify the value of the <code>Name</code> element for a resource record set in the current hosted zone.</p> <note> <p>If you&#39;re creating an alias record that has the same name as the hosted zone (known as the zone apex), you can&#39;t specify the domain name for a record for which the value of <code>Type</code> is <code>CNAME</code>. This is because the alias record must have the same type as the record that you&#39;re routing traffic to, and creating a CNAME record for the zone apex isn&#39;t supported even for an alias record.</p> </note> </dd> </dl></p>
    pub dns_name: String,
    /// <p> <i>Applies only to alias, failover alias, geolocation alias, latency alias, and weighted alias resource record sets:</i> When <code>EvaluateTargetHealth</code> is <code>true</code>, an alias resource record set inherits the health of the referenced AWS resource, such as an ELB load balancer or another resource record set in the hosted zone.</p> <p>Note the following:</p> <dl> <dt>CloudFront distributions</dt> <dd> <p>You can't set <code>EvaluateTargetHealth</code> to <code>true</code> when the alias target is a CloudFront distribution.</p> </dd> <dt>Elastic Beanstalk environments that have regionalized subdomains</dt> <dd> <p>If you specify an Elastic Beanstalk environment in <code>DNSName</code> and the environment contains an ELB load balancer, Elastic Load Balancing routes queries only to the healthy Amazon EC2 instances that are registered with the load balancer. (An environment automatically contains an ELB load balancer if it includes more than one Amazon EC2 instance.) If you set <code>EvaluateTargetHealth</code> to <code>true</code> and either no Amazon EC2 instances are healthy or the load balancer itself is unhealthy, Route 53 routes queries to other available resources that are healthy, if any. </p> <p>If the environment contains a single Amazon EC2 instance, there are no special requirements.</p> </dd> <dt>ELB load balancers</dt> <dd> <p>Health checking behavior depends on the type of load balancer:</p> <ul> <li> <p> <b>Classic Load Balancers</b>: If you specify an ELB Classic Load Balancer in <code>DNSName</code>, Elastic Load Balancing routes queries only to the healthy Amazon EC2 instances that are registered with the load balancer. If you set <code>EvaluateTargetHealth</code> to <code>true</code> and either no EC2 instances are healthy or the load balancer itself is unhealthy, Route 53 routes queries to other resources.</p> </li> <li> <p> <b>Application and Network Load Balancers</b>: If you specify an ELB Application or Network Load Balancer and you set <code>EvaluateTargetHealth</code> to <code>true</code>, Route 53 routes queries to the load balancer based on the health of the target groups that are associated with the load balancer:</p> <ul> <li> <p>For an Application or Network Load Balancer to be considered healthy, every target group that contains targets must contain at least one healthy target. If any target group contains only unhealthy targets, the load balancer is considered unhealthy, and Route 53 routes queries to other resources.</p> </li> <li> <p>A target group that has no registered targets is considered unhealthy.</p> </li> </ul> </li> </ul> <note> <p>When you create a load balancer, you configure settings for Elastic Load Balancing health checks; they're not Route 53 health checks, but they perform a similar function. Do not create Route 53 health checks for the EC2 instances that you register with an ELB load balancer. </p> </note> </dd> <dt>S3 buckets</dt> <dd> <p>There are no special requirements for setting <code>EvaluateTargetHealth</code> to <code>true</code> when the alias target is an S3 bucket.</p> </dd> <dt>Other records in the same hosted zone</dt> <dd> <p>If the AWS resource that you specify in <code>DNSName</code> is a record or a group of records (for example, a group of weighted records) but is not another alias record, we recommend that you associate a health check with all of the records in the alias target. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-complex-configs.html#dns-failover-complex-configs-hc-omitting">What Happens When You Omit Health Checks?</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </dd> </dl> <p>For more information and examples, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover.html">Amazon Route 53 Health Checks and DNS Failover</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    pub evaluate_target_health: bool,
    /// <p><p> <i>Alias resource records sets only</i>: The value used depends on where you want to route traffic:</p> <dl> <dt>Amazon API Gateway custom regional APIs and edge-optimized APIs</dt> <dd> <p>Specify the hosted zone ID for your API. You can get the applicable value using the AWS CLI command <a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-domain-names.html">get-domain-names</a>:</p> <ul> <li> <p>For regional APIs, specify the value of <code>regionalHostedZoneId</code>.</p> </li> <li> <p>For edge-optimized APIs, specify the value of <code>distributionHostedZoneId</code>.</p> </li> </ul> </dd> <dt>Amazon Virtual Private Cloud interface VPC endpoint</dt> <dd> <p>Specify the hosted zone ID for your interface endpoint. You can get the value of <code>HostedZoneId</code> using the AWS CLI command <a href="https://docs.aws.amazon.com/cli/latest/reference/ec2/describe-vpc-endpoints.html">describe-vpc-endpoints</a>.</p> </dd> <dt>CloudFront distribution</dt> <dd> <p>Specify <code>Z2FDTNDATAQYW2</code>.</p> <note> <p>Alias resource record sets for CloudFront can&#39;t be created in a private zone.</p> </note> </dd> <dt>Elastic Beanstalk environment</dt> <dd> <p>Specify the hosted zone ID for the region that you created the environment in. The environment must have a regionalized subdomain. For a list of regions and the corresponding hosted zone IDs, see <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#elasticbeanstalk_region">AWS Elastic Beanstalk</a> in the &quot;AWS Regions and Endpoints&quot; chapter of the <i>Amazon Web Services General Reference</i>.</p> </dd> <dt>ELB load balancer</dt> <dd> <p>Specify the value of the hosted zone ID for the load balancer. Use the following methods to get the hosted zone ID:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html#elb_region">Elastic Load Balancing</a> table in the &quot;AWS Regions and Endpoints&quot; chapter of the <i>Amazon Web Services General Reference</i>: Use the value that corresponds with the region that you created your load balancer in. Note that there are separate columns for Application and Classic Load Balancers and for Network Load Balancers.</p> </li> <li> <p> <b>AWS Management Console</b>: Go to the Amazon EC2 page, choose <b>Load Balancers</b> in the navigation pane, select the load balancer, and get the value of the <b>Hosted zone</b> field on the <b>Description</b> tab.</p> </li> <li> <p> <b>Elastic Load Balancing API</b>: Use <code>DescribeLoadBalancers</code> to get the applicable value. For more information, see the applicable guide:</p> <ul> <li> <p>Classic Load Balancers: Use <a href="http://docs.aws.amazon.com/elasticloadbalancing/2012-06-01/APIReference/API_DescribeLoadBalancers.html">DescribeLoadBalancers</a> to get the value of <code>CanonicalHostedZoneNameId</code>.</p> </li> <li> <p>Application and Network Load Balancers: Use <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DescribeLoadBalancers.html">DescribeLoadBalancers</a> to get the value of <code>CanonicalHostedZoneId</code>.</p> </li> </ul> </li> <li> <p> <b>AWS CLI</b>: Use <code>describe-load-balancers</code> to get the applicable value. For more information, see the applicable guide:</p> <ul> <li> <p>Classic Load Balancers: Use <a href="http://docs.aws.amazon.com/cli/latest/reference/elb/describe-load-balancers.html">describe-load-balancers</a> to get the value of <code>CanonicalHostedZoneNameId</code>.</p> </li> <li> <p>Application and Network Load Balancers: Use <a href="http://docs.aws.amazon.com/cli/latest/reference/elbv2/describe-load-balancers.html">describe-load-balancers</a> to get the value of <code>CanonicalHostedZoneId</code>.</p> </li> </ul> </li> </ul> </dd> <dt>An Amazon S3 bucket configured as a static website</dt> <dd> <p>Specify the hosted zone ID for the region that you created the bucket in. For more information about valid values, see the <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region">Amazon Simple Storage Service Website Endpoints</a> table in the &quot;AWS Regions and Endpoints&quot; chapter of the <i>Amazon Web Services General Reference</i>.</p> </dd> <dt>Another Route 53 resource record set in your hosted zone</dt> <dd> <p>Specify the hosted zone ID of your hosted zone. (An alias resource record set can&#39;t reference a resource record set in a different hosted zone.)</p> </dd> </dl></p>
    pub hosted_zone_id: String,
}

struct AliasTargetDeserializer;
impl AliasTargetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AliasTarget, XmlParseError> {
        deserialize_elements::<_, AliasTarget, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DNSName" => {
                    obj.dns_name = DNSNameDeserializer::deserialize("DNSName", stack)?;
                }
                "EvaluateTargetHealth" => {
                    obj.evaluate_target_health =
                        AliasHealthEnabledDeserializer::deserialize("EvaluateTargetHealth", stack)?;
                }
                "HostedZoneId" => {
                    obj.hosted_zone_id =
                        ResourceIdDeserializer::deserialize("HostedZoneId", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AssociateVPCWithHostedZoneResponse {
    /// <p>A complex type that describes the changes made to your hosted zone.</p>
    pub change_info: ChangeInfo,
}

struct AssociateVPCWithHostedZoneResponseDeserializer;
impl AssociateVPCWithHostedZoneResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AssociateVPCWithHostedZoneResponse, XmlParseError> {
        deserialize_elements::<_, AssociateVPCWithHostedZoneResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ChangeInfo" => {
                        obj.change_info = ChangeInfoDeserializer::deserialize("ChangeInfo", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>The information for each resource record set that you want to change.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Change {
    /// <p><p>The action to perform:</p> <ul> <li> <p> <code>CREATE</code>: Creates a resource record set that has the specified values.</p> </li> <li> <p> <code>DELETE</code>: Deletes a existing resource record set.</p> <important> <p>To delete the resource record set that is associated with a traffic policy instance, use <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_DeleteTrafficPolicyInstance.html">DeleteTrafficPolicyInstance</a>. Amazon Route 53 will delete the resource record set automatically. If you delete the resource record set by using <code>ChangeResourceRecordSets</code>, Route 53 doesn&#39;t automatically delete the traffic policy instance, and you&#39;ll continue to be charged for it even though it&#39;s no longer in use. </p> </important> </li> <li> <p> <code>UPSERT</code>: If a resource record set doesn&#39;t already exist, Route 53 creates it. If a resource record set does exist, Route 53 updates it with the values in the request.</p> </li> </ul></p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ChangeInfo {
    /// <p>A complex type that describes change information about changes made to your hosted zone.</p> <p>This element contains an ID that you use when performing a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetChange.html">GetChange</a> action to get detailed information about the change.</p>
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ChangeInfo, XmlParseError> {
        deserialize_elements::<_, ChangeInfo, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Comment" => {
                    obj.comment = Some(ResourceDescriptionDeserializer::deserialize(
                        "Comment", stack,
                    )?);
                }
                "Id" => {
                    obj.id = ResourceIdDeserializer::deserialize("Id", stack)?;
                }
                "Status" => {
                    obj.status = ChangeStatusDeserializer::deserialize("Status", stack)?;
                }
                "SubmittedAt" => {
                    obj.submitted_at = TimeStampDeserializer::deserialize("SubmittedAt", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A complex type that contains change information for the resource record set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ChangeResourceRecordSetsResponse {
    /// <p>A complex type that contains information about changes made to your hosted zone.</p> <p>This element contains an ID that you use when performing a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetChange.html">GetChange</a> action to get detailed information about the change.</p>
    pub change_info: ChangeInfo,
}

struct ChangeResourceRecordSetsResponseDeserializer;
impl ChangeResourceRecordSetsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ChangeResourceRecordSetsResponse, XmlParseError> {
        deserialize_elements::<_, ChangeResourceRecordSetsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ChangeInfo" => {
                        obj.change_info = ChangeInfoDeserializer::deserialize("ChangeInfo", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct ChangeStatusDeserializer;
impl ChangeStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A complex type that contains information about the tags that you want to add, edit, or delete.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ChangeTagsForResourceResponse {}

struct ChangeTagsForResourceResponseDeserializer;
impl ChangeTagsForResourceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ChangeTagsForResourceResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = ChangeTagsForResourceResponse::default();

        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(IPAddressCidrDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ChildHealthCheckListDeserializer;
impl ChildHealthCheckListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ChildHealthCheck" {
                obj.push(HealthCheckIdDeserializer::deserialize(
                    "ChildHealthCheck",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudWatchAlarmConfiguration, XmlParseError> {
        deserialize_elements::<_, CloudWatchAlarmConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ComparisonOperator" => {
                        obj.comparison_operator = ComparisonOperatorDeserializer::deserialize(
                            "ComparisonOperator",
                            stack,
                        )?;
                    }
                    "Dimensions" => {
                        obj.dimensions
                            .get_or_insert(vec![])
                            .extend(DimensionListDeserializer::deserialize("Dimensions", stack)?);
                    }
                    "EvaluationPeriods" => {
                        obj.evaluation_periods =
                            EvaluationPeriodsDeserializer::deserialize("EvaluationPeriods", stack)?;
                    }
                    "MetricName" => {
                        obj.metric_name = MetricNameDeserializer::deserialize("MetricName", stack)?;
                    }
                    "Namespace" => {
                        obj.namespace = NamespaceDeserializer::deserialize("Namespace", stack)?;
                    }
                    "Period" => {
                        obj.period = PeriodDeserializer::deserialize("Period", stack)?;
                    }
                    "Statistic" => {
                        obj.statistic = StatisticDeserializer::deserialize("Statistic", stack)?;
                    }
                    "Threshold" => {
                        obj.threshold = ThresholdDeserializer::deserialize("Threshold", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct CloudWatchLogsLogGroupArnDeserializer;
impl CloudWatchLogsLogGroupArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A complex type that contains the health check request information.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateHealthCheckRequest {
    /// <p><p>A unique string that identifies the request and that allows you to retry a failed <code>CreateHealthCheck</code> request without the risk of creating two identical health checks:</p> <ul> <li> <p>If you send a <code>CreateHealthCheck</code> request with the same <code>CallerReference</code> and settings as a previous request, and if the health check doesn&#39;t exist, Amazon Route 53 creates the health check. If the health check does exist, Route 53 returns the settings for the existing health check.</p> </li> <li> <p>If you send a <code>CreateHealthCheck</code> request with the same <code>CallerReference</code> as a deleted health check, regardless of the settings, Route 53 returns a <code>HealthCheckAlreadyExists</code> error.</p> </li> <li> <p>If you send a <code>CreateHealthCheck</code> request with the same <code>CallerReference</code> as an existing health check but with different settings, Route 53 returns a <code>HealthCheckAlreadyExists</code> error.</p> </li> <li> <p>If you send a <code>CreateHealthCheck</code> request with a unique <code>CallerReference</code> but settings identical to an existing health check, Route 53 creates the health check.</p> </li> </ul></p>
    pub caller_reference: String,
    /// <p>A complex type that contains settings for a new health check.</p>
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateHealthCheckResponse {
    /// <p>A complex type that contains identifying information about the health check.</p>
    pub health_check: HealthCheck,
    /// <p>The unique URL representing the new health check.</p>
    pub location: String,
}

struct CreateHealthCheckResponseDeserializer;
impl CreateHealthCheckResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateHealthCheckResponse, XmlParseError> {
        deserialize_elements::<_, CreateHealthCheckResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HealthCheck" => {
                        obj.health_check =
                            HealthCheckDeserializer::deserialize("HealthCheck", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A complex type that contains information about the request to create a public or private hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateHostedZoneRequest {
    /// <p>A unique string that identifies the request and that allows failed <code>CreateHostedZone</code> requests to be retried without the risk of executing the operation twice. You must use a unique <code>CallerReference</code> string every time you submit a <code>CreateHostedZone</code> request. <code>CallerReference</code> can be any unique string, for example, a date/time stamp.</p>
    pub caller_reference: String,
    /// <p>If you want to associate a reusable delegation set with this hosted zone, the ID that Amazon Route 53 assigned to the reusable delegation set when you created it. For more information about reusable delegation sets, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateReusableDelegationSet.html">CreateReusableDelegationSet</a>.</p>
    pub delegation_set_id: Option<String>,
    /// <p>(Optional) A complex type that contains the following optional values:</p> <ul> <li> <p>For public and private hosted zones, an optional comment</p> </li> <li> <p>For private hosted zones, an optional <code>PrivateZone</code> element</p> </li> </ul> <p>If you don't specify a comment or the <code>PrivateZone</code> element, omit <code>HostedZoneConfig</code> and the other elements.</p>
    pub hosted_zone_config: Option<HostedZoneConfig>,
    /// <p>The name of the domain. Specify a fully qualified domain name, for example, <i>www.example.com</i>. The trailing dot is optional; Amazon Route 53 assumes that the domain name is fully qualified. This means that Route 53 treats <i>www.example.com</i> (without a trailing dot) and <i>www.example.com.</i> (with a trailing dot) as identical.</p> <p>If you're creating a public hosted zone, this is the name you have registered with your DNS registrar. If your domain name is registered with a registrar other than Route 53, change the name servers for your domain to the set of <code>NameServers</code> that <code>CreateHostedZone</code> returns in <code>DelegationSet</code>.</p>
    pub name: String,
    /// <p>(Private hosted zones only) A complex type that contains information about the Amazon VPC that you're associating with this hosted zone.</p> <p>You can specify only one Amazon VPC when you create a private hosted zone. To associate additional Amazon VPCs with the hosted zone, use <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_AssociateVPCWithHostedZone.html">AssociateVPCWithHostedZone</a> after you create a hosted zone.</p>
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateHostedZoneResponse, XmlParseError> {
        deserialize_elements::<_, CreateHostedZoneResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ChangeInfo" => {
                        obj.change_info = ChangeInfoDeserializer::deserialize("ChangeInfo", stack)?;
                    }
                    "DelegationSet" => {
                        obj.delegation_set =
                            DelegationSetDeserializer::deserialize("DelegationSet", stack)?;
                    }
                    "HostedZone" => {
                        obj.hosted_zone = HostedZoneDeserializer::deserialize("HostedZone", stack)?;
                    }
                    "VPC" => {
                        obj.vpc = Some(VPCDeserializer::deserialize("VPC", stack)?);
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
pub struct CreateQueryLoggingConfigRequest {
    /// <p>The Amazon Resource Name (ARN) for the log group that you want to Amazon Route 53 to send query logs to. This is the format of the ARN:</p> <p>arn:aws:logs:<i>region</i>:<i>account-id</i>:log-group:<i>log_group_name</i> </p> <p>To get the ARN for a log group, you can use the CloudWatch console, the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogGroups.html">DescribeLogGroups</a> API action, the <a href="https://docs.aws.amazon.com/cli/latest/reference/logs/describe-log-groups.html">describe-log-groups</a> command, or the applicable command in one of the AWS SDKs.</p>
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateQueryLoggingConfigResponse {
    /// <p>The unique URL representing the new query logging configuration.</p>
    pub location: String,
    /// <p>A complex type that contains the ID for a query logging configuration, the ID of the hosted zone that you want to log queries for, and the ARN for the log group that you want Amazon Route 53 to send query logs to.</p>
    pub query_logging_config: QueryLoggingConfig,
}

struct CreateQueryLoggingConfigResponseDeserializer;
impl CreateQueryLoggingConfigResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateQueryLoggingConfigResponse, XmlParseError> {
        deserialize_elements::<_, CreateQueryLoggingConfigResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "QueryLoggingConfig" => {
                        obj.query_logging_config = QueryLoggingConfigDeserializer::deserialize(
                            "QueryLoggingConfig",
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
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateReusableDelegationSetResponse {
    /// <p>A complex type that contains name server information.</p>
    pub delegation_set: DelegationSet,
    /// <p>The unique URL representing the new reusable delegation set.</p>
    pub location: String,
}

struct CreateReusableDelegationSetResponseDeserializer;
impl CreateReusableDelegationSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateReusableDelegationSetResponse, XmlParseError> {
        deserialize_elements::<_, CreateReusableDelegationSetResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DelegationSet" => {
                        obj.delegation_set =
                            DelegationSetDeserializer::deserialize("DelegationSet", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A complex type that contains information about the resource record sets that you want to create based on a specified traffic policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTrafficPolicyInstanceRequest {
    /// <p>The ID of the hosted zone that you want Amazon Route 53 to create resource record sets in by using the configuration in a traffic policy.</p>
    pub hosted_zone_id: String,
    /// <p>The domain name (such as example.com) or subdomain name (such as www.example.com) for which Amazon Route 53 responds to DNS queries by using the resource record sets that Route 53 creates for this traffic policy instance.</p>
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateTrafficPolicyInstanceResponse {
    /// <p>A unique URL that represents a new traffic policy instance.</p>
    pub location: String,
    /// <p>A complex type that contains settings for the new traffic policy instance.</p>
    pub traffic_policy_instance: TrafficPolicyInstance,
}

struct CreateTrafficPolicyInstanceResponseDeserializer;
impl CreateTrafficPolicyInstanceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateTrafficPolicyInstanceResponse, XmlParseError> {
        deserialize_elements::<_, CreateTrafficPolicyInstanceResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TrafficPolicyInstance" => {
                        obj.traffic_policy_instance =
                            TrafficPolicyInstanceDeserializer::deserialize(
                                "TrafficPolicyInstance",
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
/// <p>A complex type that contains information about the traffic policy that you want to create.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTrafficPolicyRequest {
    /// <p>(Optional) Any comments that you want to include about the traffic policy.</p>
    pub comment: Option<String>,
    /// <p>The definition of this traffic policy in JSON format. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/api-policies-traffic-policy-document-format.html">Traffic Policy Document Format</a>.</p>
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateTrafficPolicyResponse {
    /// <p>A unique URL that represents a new traffic policy.</p>
    pub location: String,
    /// <p>A complex type that contains settings for the new traffic policy.</p>
    pub traffic_policy: TrafficPolicy,
}

struct CreateTrafficPolicyResponseDeserializer;
impl CreateTrafficPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateTrafficPolicyResponse, XmlParseError> {
        deserialize_elements::<_, CreateTrafficPolicyResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TrafficPolicy" => {
                        obj.traffic_policy =
                            TrafficPolicyDeserializer::deserialize("TrafficPolicy", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A complex type that contains information about the traffic policy that you want to create a new version for.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTrafficPolicyVersionRequest {
    /// <p>The comment that you specified in the <code>CreateTrafficPolicyVersion</code> request, if any.</p>
    pub comment: Option<String>,
    /// <p>The definition of this version of the traffic policy, in JSON format. You specified the JSON in the <code>CreateTrafficPolicyVersion</code> request. For more information about the JSON format, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateTrafficPolicy.html">CreateTrafficPolicy</a>.</p>
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateTrafficPolicyVersionResponse {
    /// <p>A unique URL that represents a new traffic policy version.</p>
    pub location: String,
    /// <p>A complex type that contains settings for the new version of the traffic policy.</p>
    pub traffic_policy: TrafficPolicy,
}

struct CreateTrafficPolicyVersionResponseDeserializer;
impl CreateTrafficPolicyVersionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateTrafficPolicyVersionResponse, XmlParseError> {
        deserialize_elements::<_, CreateTrafficPolicyVersionResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TrafficPolicy" => {
                        obj.traffic_policy =
                            TrafficPolicyDeserializer::deserialize("TrafficPolicy", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A complex type that contains information about the request to authorize associating a VPC with your private hosted zone. Authorization is only required when a private hosted zone and a VPC were created by using different accounts.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateVPCAssociationAuthorizationResponse {
    /// <p>The ID of the hosted zone that you authorized associating a VPC with.</p>
    pub hosted_zone_id: String,
    /// <p>The VPC that you authorized associating with a hosted zone.</p>
    pub vpc: VPC,
}

struct CreateVPCAssociationAuthorizationResponseDeserializer;
impl CreateVPCAssociationAuthorizationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateVPCAssociationAuthorizationResponse, XmlParseError> {
        deserialize_elements::<_, CreateVPCAssociationAuthorizationResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HostedZoneId" => {
                        obj.hosted_zone_id =
                            ResourceIdDeserializer::deserialize("HostedZoneId", stack)?;
                    }
                    "VPC" => {
                        obj.vpc = VPCDeserializer::deserialize("VPC", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A complex type that lists the name servers in a delegation set, as well as the <code>CallerReference</code> and the <code>ID</code> for the delegation set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DelegationSet, XmlParseError> {
        deserialize_elements::<_, DelegationSet, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CallerReference" => {
                    obj.caller_reference =
                        Some(NonceDeserializer::deserialize("CallerReference", stack)?);
                }
                "Id" => {
                    obj.id = Some(ResourceIdDeserializer::deserialize("Id", stack)?);
                }
                "NameServers" => {
                    obj.name_servers
                        .extend(DelegationSetNameServersDeserializer::deserialize(
                            "NameServers",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DelegationSetNameServersDeserializer;
impl DelegationSetNameServersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "NameServer" {
                obj.push(DNSNameDeserializer::deserialize("NameServer", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct DelegationSetsDeserializer;
impl DelegationSetsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DelegationSet>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DelegationSet" {
                obj.push(DelegationSetDeserializer::deserialize(
                    "DelegationSet",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>This action deletes a health check.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteHealthCheckRequest {
    /// <p>The ID of the health check that you want to delete.</p>
    pub health_check_id: String,
}

/// <p>An empty element.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteHealthCheckResponse {}

struct DeleteHealthCheckResponseDeserializer;
impl DeleteHealthCheckResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteHealthCheckResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteHealthCheckResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A request to delete a hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteHostedZoneRequest {
    /// <p>The ID of the hosted zone you want to delete.</p>
    pub id: String,
}

/// <p>A complex type that contains the response to a <code>DeleteHostedZone</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteHostedZoneResponse {
    /// <p>A complex type that contains the ID, the status, and the date and time of a request to delete a hosted zone.</p>
    pub change_info: ChangeInfo,
}

struct DeleteHostedZoneResponseDeserializer;
impl DeleteHostedZoneResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteHostedZoneResponse, XmlParseError> {
        deserialize_elements::<_, DeleteHostedZoneResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ChangeInfo" => {
                        obj.change_info = ChangeInfoDeserializer::deserialize("ChangeInfo", stack)?;
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
pub struct DeleteQueryLoggingConfigRequest {
    /// <p>The ID of the configuration that you want to delete. </p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteQueryLoggingConfigResponse {}

struct DeleteQueryLoggingConfigResponseDeserializer;
impl DeleteQueryLoggingConfigResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteQueryLoggingConfigResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteQueryLoggingConfigResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A request to delete a reusable delegation set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReusableDelegationSetRequest {
    /// <p>The ID of the reusable delegation set that you want to delete.</p>
    pub id: String,
}

/// <p>An empty element.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteReusableDelegationSetResponse {}

struct DeleteReusableDelegationSetResponseDeserializer;
impl DeleteReusableDelegationSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteReusableDelegationSetResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteReusableDelegationSetResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A request to delete a specified traffic policy instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTrafficPolicyInstanceRequest {
    /// <p><p>The ID of the traffic policy instance that you want to delete. </p> <important> <p>When you delete a traffic policy instance, Amazon Route 53 also deletes all of the resource record sets that were created when you created the traffic policy instance.</p> </important></p>
    pub id: String,
}

/// <p>An empty element.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteTrafficPolicyInstanceResponse {}

struct DeleteTrafficPolicyInstanceResponseDeserializer;
impl DeleteTrafficPolicyInstanceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteTrafficPolicyInstanceResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteTrafficPolicyInstanceResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A request to delete a specified traffic policy version.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTrafficPolicyRequest {
    /// <p>The ID of the traffic policy that you want to delete.</p>
    pub id: String,
    /// <p>The version number of the traffic policy that you want to delete.</p>
    pub version: i64,
}

/// <p>An empty element.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteTrafficPolicyResponse {}

struct DeleteTrafficPolicyResponseDeserializer;
impl DeleteTrafficPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteTrafficPolicyResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteTrafficPolicyResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A complex type that contains information about the request to remove authorization to associate a VPC that was created by one AWS account with a hosted zone that was created with a different AWS account. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteVPCAssociationAuthorizationResponse {}

struct DeleteVPCAssociationAuthorizationResponseDeserializer;
impl DeleteVPCAssociationAuthorizationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteVPCAssociationAuthorizationResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteVPCAssociationAuthorizationResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>For the metric that the CloudWatch alarm is associated with, a complex type that contains information about one dimension.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Dimension {
    /// <p>For the metric that the CloudWatch alarm is associated with, the name of one dimension.</p>
    pub name: String,
    /// <p>For the metric that the CloudWatch alarm is associated with, the value of one dimension.</p>
    pub value: String,
}

struct DimensionDeserializer;
impl DimensionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Dimension, XmlParseError> {
        deserialize_elements::<_, Dimension, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Name" => {
                    obj.name = DimensionFieldDeserializer::deserialize("Name", stack)?;
                }
                "Value" => {
                    obj.value = DimensionFieldDeserializer::deserialize("Value", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DimensionFieldDeserializer;
impl DimensionFieldDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DimensionListDeserializer;
impl DimensionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Dimension>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Dimension" {
                obj.push(DimensionDeserializer::deserialize("Dimension", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct DisabledDeserializer;
impl DisabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct DisabledSerializer;
impl DisabledSerializer {
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DisassociateVPCFromHostedZoneResponse {
    /// <p>A complex type that describes the changes made to the specified private hosted zone.</p>
    pub change_info: ChangeInfo,
}

struct DisassociateVPCFromHostedZoneResponseDeserializer;
impl DisassociateVPCFromHostedZoneResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DisassociateVPCFromHostedZoneResponse, XmlParseError> {
        deserialize_elements::<_, DisassociateVPCFromHostedZoneResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ChangeInfo" => {
                        obj.change_info = ChangeInfoDeserializer::deserialize("ChangeInfo", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct EnableSNIDeserializer;
impl EnableSNIDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct FailureThresholdDeserializer;
impl FailureThresholdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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

/// <p>A complex type that contains information about a geographic location.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GeoLocation {
    /// <p>The two-letter code for the continent.</p> <p>Valid values: <code>AF</code> | <code>AN</code> | <code>AS</code> | <code>EU</code> | <code>OC</code> | <code>NA</code> | <code>SA</code> </p> <p>Constraint: Specifying <code>ContinentCode</code> with either <code>CountryCode</code> or <code>SubdivisionCode</code> returns an <code>InvalidInput</code> error.</p>
    pub continent_code: Option<String>,
    /// <p>The two-letter code for the country.</p>
    pub country_code: Option<String>,
    /// <p>The code for the subdivision. Route 53 currently supports only states in the United States.</p>
    pub subdivision_code: Option<String>,
}

struct GeoLocationDeserializer;
impl GeoLocationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GeoLocation, XmlParseError> {
        deserialize_elements::<_, GeoLocation, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ContinentCode" => {
                    obj.continent_code = Some(GeoLocationContinentCodeDeserializer::deserialize(
                        "ContinentCode",
                        stack,
                    )?);
                }
                "CountryCode" => {
                    obj.country_code = Some(GeoLocationCountryCodeDeserializer::deserialize(
                        "CountryCode",
                        stack,
                    )?);
                }
                "SubdivisionCode" => {
                    obj.subdivision_code =
                        Some(GeoLocationSubdivisionCodeDeserializer::deserialize(
                            "SubdivisionCode",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct GeoLocationCountryCodeDeserializer;
impl GeoLocationCountryCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A complex type that contains the codes and full continent, country, and subdivision names for the specified <code>geolocation</code> code.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GeoLocationDetails {
    /// <p>The two-letter code for the continent.</p>
    pub continent_code: Option<String>,
    /// <p>The full name of the continent.</p>
    pub continent_name: Option<String>,
    /// <p>The two-letter code for the country.</p>
    pub country_code: Option<String>,
    /// <p>The name of the country.</p>
    pub country_name: Option<String>,
    /// <p>The code for the subdivision. Route 53 currently supports only states in the United States.</p>
    pub subdivision_code: Option<String>,
    /// <p>The full name of the subdivision. Route 53 currently supports only states in the United States.</p>
    pub subdivision_name: Option<String>,
}

struct GeoLocationDetailsDeserializer;
impl GeoLocationDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GeoLocationDetails, XmlParseError> {
        deserialize_elements::<_, GeoLocationDetails, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ContinentCode" => {
                    obj.continent_code = Some(GeoLocationContinentCodeDeserializer::deserialize(
                        "ContinentCode",
                        stack,
                    )?);
                }
                "ContinentName" => {
                    obj.continent_name = Some(GeoLocationContinentNameDeserializer::deserialize(
                        "ContinentName",
                        stack,
                    )?);
                }
                "CountryCode" => {
                    obj.country_code = Some(GeoLocationCountryCodeDeserializer::deserialize(
                        "CountryCode",
                        stack,
                    )?);
                }
                "CountryName" => {
                    obj.country_name = Some(GeoLocationCountryNameDeserializer::deserialize(
                        "CountryName",
                        stack,
                    )?);
                }
                "SubdivisionCode" => {
                    obj.subdivision_code =
                        Some(GeoLocationSubdivisionCodeDeserializer::deserialize(
                            "SubdivisionCode",
                            stack,
                        )?);
                }
                "SubdivisionName" => {
                    obj.subdivision_name =
                        Some(GeoLocationSubdivisionNameDeserializer::deserialize(
                            "SubdivisionName",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct GeoLocationDetailsListDeserializer;
impl GeoLocationDetailsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<GeoLocationDetails>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "GeoLocationDetails" {
                obj.push(GeoLocationDetailsDeserializer::deserialize(
                    "GeoLocationDetails",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct GeoLocationSubdivisionCodeDeserializer;
impl GeoLocationSubdivisionCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A complex type that contains information about the request to create a hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAccountLimitRequest {
    /// <p><p>The limit that you want to get. Valid values include the following:</p> <ul> <li> <p> <b>MAX<em>HEALTH</em>CHECKS<em>BY</em>OWNER</b>: The maximum number of health checks that you can create using the current account.</p> </li> <li> <p> <b>MAX<em>HOSTED</em>ZONES<em>BY</em>OWNER</b>: The maximum number of hosted zones that you can create using the current account.</p> </li> <li> <p> <b>MAX<em>REUSABLE</em>DELEGATION<em>SETS</em>BY<em>OWNER</b>: The maximum number of reusable delegation sets that you can create using the current account.</p> </li> <li> <p> <b>MAX</em>TRAFFIC<em>POLICIES</em>BY<em>OWNER</b>: The maximum number of traffic policies that you can create using the current account.</p> </li> <li> <p> <b>MAX</em>TRAFFIC<em>POLICY</em>INSTANCES<em>BY</em>OWNER</b>: The maximum number of traffic policy instances that you can create using the current account. (Traffic policy instances are referred to as traffic flow policy records in the Amazon Route 53 console.)</p> </li> </ul></p>
    pub type_: String,
}

/// <p>A complex type that contains the requested limit. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetAccountLimitResponse {
    /// <p>The current number of entities that you have created of the specified type. For example, if you specified <code>MAX_HEALTH_CHECKS_BY_OWNER</code> for the value of <code>Type</code> in the request, the value of <code>Count</code> is the current number of health checks that you have created using the current account.</p>
    pub count: i64,
    /// <p>The current setting for the specified limit. For example, if you specified <code>MAX_HEALTH_CHECKS_BY_OWNER</code> for the value of <code>Type</code> in the request, the value of <code>Limit</code> is the maximum number of health checks that you can create using the current account.</p>
    pub limit: AccountLimit,
}

struct GetAccountLimitResponseDeserializer;
impl GetAccountLimitResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetAccountLimitResponse, XmlParseError> {
        deserialize_elements::<_, GetAccountLimitResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Count" => {
                        obj.count = UsageCountDeserializer::deserialize("Count", stack)?;
                    }
                    "Limit" => {
                        obj.limit = AccountLimitDeserializer::deserialize("Limit", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>The input for a GetChange request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetChangeRequest {
    /// <p>The ID of the change batch request. The value that you specify here is the value that <code>ChangeResourceRecordSets</code> returned in the <code>Id</code> element when you submitted the request.</p>
    pub id: String,
}

/// <p>A complex type that contains the <code>ChangeInfo</code> element.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetChangeResponse {
    /// <p>A complex type that contains information about the specified change batch.</p>
    pub change_info: ChangeInfo,
}

struct GetChangeResponseDeserializer;
impl GetChangeResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetChangeResponse, XmlParseError> {
        deserialize_elements::<_, GetChangeResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ChangeInfo" => {
                    obj.change_info = ChangeInfoDeserializer::deserialize("ChangeInfo", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Empty request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCheckerIpRangesRequest {}

/// <p>A complex type that contains the <code>CheckerIpRanges</code> element.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetCheckerIpRangesResponse {
    /// <p>A complex type that contains sorted list of IP ranges in CIDR format for Amazon Route 53 health checkers.</p>
    pub checker_ip_ranges: Vec<String>,
}

struct GetCheckerIpRangesResponseDeserializer;
impl GetCheckerIpRangesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetCheckerIpRangesResponse, XmlParseError> {
        deserialize_elements::<_, GetCheckerIpRangesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CheckerIpRanges" => {
                        obj.checker_ip_ranges
                            .extend(CheckerIpRangesDeserializer::deserialize(
                                "CheckerIpRanges",
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
/// <p>A request for information about whether a specified geographic location is supported for Amazon Route 53 geolocation resource record sets.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGeoLocationRequest {
    /// <p><p>Amazon Route 53 supports the following continent codes:</p> <ul> <li> <p> <b>AF</b>: Africa</p> </li> <li> <p> <b>AN</b>: Antarctica</p> </li> <li> <p> <b>AS</b>: Asia</p> </li> <li> <p> <b>EU</b>: Europe</p> </li> <li> <p> <b>OC</b>: Oceania</p> </li> <li> <p> <b>NA</b>: North America</p> </li> <li> <p> <b>SA</b>: South America</p> </li> </ul></p>
    pub continent_code: Option<String>,
    /// <p>Amazon Route 53 uses the two-letter country codes that are specified in <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO standard 3166-1 alpha-2</a>.</p>
    pub country_code: Option<String>,
    /// <p>Amazon Route 53 uses the one- to three-letter subdivision codes that are specified in <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO standard 3166-1 alpha-2</a>. Route 53 doesn't support subdivision codes for all countries. If you specify <code>subdivisioncode</code>, you must also specify <code>countrycode</code>. </p>
    pub subdivision_code: Option<String>,
}

/// <p>A complex type that contains the response information for the specified geolocation code.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetGeoLocationResponse {
    /// <p>A complex type that contains the codes and full continent, country, and subdivision names for the specified geolocation code.</p>
    pub geo_location_details: GeoLocationDetails,
}

struct GetGeoLocationResponseDeserializer;
impl GetGeoLocationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetGeoLocationResponse, XmlParseError> {
        deserialize_elements::<_, GetGeoLocationResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "GeoLocationDetails" => {
                    obj.geo_location_details =
                        GeoLocationDetailsDeserializer::deserialize("GeoLocationDetails", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A request for the number of health checks that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetHealthCheckCountRequest {}

/// <p>A complex type that contains the response to a <code>GetHealthCheckCount</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetHealthCheckCountResponse {
    /// <p>The number of health checks associated with the current AWS account.</p>
    pub health_check_count: i64,
}

struct GetHealthCheckCountResponseDeserializer;
impl GetHealthCheckCountResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHealthCheckCountResponse, XmlParseError> {
        deserialize_elements::<_, GetHealthCheckCountResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HealthCheckCount" => {
                        obj.health_check_count =
                            HealthCheckCountDeserializer::deserialize("HealthCheckCount", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A request for the reason that a health check failed most recently.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetHealthCheckLastFailureReasonRequest {
    /// <p><p>The ID for the health check for which you want the last failure reason. When you created the health check, <code>CreateHealthCheck</code> returned the ID in the response, in the <code>HealthCheckId</code> element.</p> <note> <p>If you want to get the last failure reason for a calculated health check, you must use the Amazon Route 53 console or the CloudWatch console. You can&#39;t use <code>GetHealthCheckLastFailureReason</code> for a calculated health check.</p> </note></p>
    pub health_check_id: String,
}

/// <p>A complex type that contains the response to a <code>GetHealthCheckLastFailureReason</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetHealthCheckLastFailureReasonResponse {
    /// <p>A list that contains one <code>Observation</code> element for each Amazon Route 53 health checker that is reporting a last failure reason. </p>
    pub health_check_observations: Vec<HealthCheckObservation>,
}

struct GetHealthCheckLastFailureReasonResponseDeserializer;
impl GetHealthCheckLastFailureReasonResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHealthCheckLastFailureReasonResponse, XmlParseError> {
        deserialize_elements::<_, GetHealthCheckLastFailureReasonResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HealthCheckObservations" => {
                        obj.health_check_observations.extend(
                            HealthCheckObservationsDeserializer::deserialize(
                                "HealthCheckObservations",
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
/// <p>A request to get information about a specified health check. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetHealthCheckRequest {
    /// <p>The identifier that Amazon Route 53 assigned to the health check when you created it. When you add or update a resource record set, you use this value to specify which health check to use. The value can be up to 64 characters long.</p>
    pub health_check_id: String,
}

/// <p>A complex type that contains the response to a <code>GetHealthCheck</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetHealthCheckResponse {
    /// <p>A complex type that contains information about one health check that is associated with the current AWS account.</p>
    pub health_check: HealthCheck,
}

struct GetHealthCheckResponseDeserializer;
impl GetHealthCheckResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHealthCheckResponse, XmlParseError> {
        deserialize_elements::<_, GetHealthCheckResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "HealthCheck" => {
                    obj.health_check = HealthCheckDeserializer::deserialize("HealthCheck", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A request to get the status for a health check.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetHealthCheckStatusRequest {
    /// <p><p>The ID for the health check that you want the current status for. When you created the health check, <code>CreateHealthCheck</code> returned the ID in the response, in the <code>HealthCheckId</code> element.</p> <note> <p>If you want to check the status of a calculated health check, you must use the Amazon Route 53 console or the CloudWatch console. You can&#39;t use <code>GetHealthCheckStatus</code> to get the status of a calculated health check.</p> </note></p>
    pub health_check_id: String,
}

/// <p>A complex type that contains the response to a <code>GetHealthCheck</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetHealthCheckStatusResponse {
    /// <p>A list that contains one <code>HealthCheckObservation</code> element for each Amazon Route 53 health checker that is reporting a status about the health check endpoint.</p>
    pub health_check_observations: Vec<HealthCheckObservation>,
}

struct GetHealthCheckStatusResponseDeserializer;
impl GetHealthCheckStatusResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHealthCheckStatusResponse, XmlParseError> {
        deserialize_elements::<_, GetHealthCheckStatusResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HealthCheckObservations" => {
                        obj.health_check_observations.extend(
                            HealthCheckObservationsDeserializer::deserialize(
                                "HealthCheckObservations",
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
/// <p>A request to retrieve a count of all the hosted zones that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetHostedZoneCountRequest {}

/// <p>A complex type that contains the response to a <code>GetHostedZoneCount</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetHostedZoneCountResponse {
    /// <p>The total number of public and private hosted zones that are associated with the current AWS account.</p>
    pub hosted_zone_count: i64,
}

struct GetHostedZoneCountResponseDeserializer;
impl GetHostedZoneCountResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHostedZoneCountResponse, XmlParseError> {
        deserialize_elements::<_, GetHostedZoneCountResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HostedZoneCount" => {
                        obj.hosted_zone_count =
                            HostedZoneCountDeserializer::deserialize("HostedZoneCount", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A complex type that contains information about the request to create a hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetHostedZoneLimitRequest {
    /// <p>The ID of the hosted zone that you want to get a limit for.</p>
    pub hosted_zone_id: String,
    /// <p><p>The limit that you want to get. Valid values include the following:</p> <ul> <li> <p> <b>MAX<em>RRSETS</em>BY<em>ZONE</b>: The maximum number of records that you can create in the specified hosted zone.</p> </li> <li> <p> <b>MAX</em>VPCS<em>ASSOCIATED</em>BY_ZONE</b>: The maximum number of Amazon VPCs that you can associate with the specified private hosted zone.</p> </li> </ul></p>
    pub type_: String,
}

/// <p>A complex type that contains the requested limit. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetHostedZoneLimitResponse {
    /// <p>The current number of entities that you have created of the specified type. For example, if you specified <code>MAX_RRSETS_BY_ZONE</code> for the value of <code>Type</code> in the request, the value of <code>Count</code> is the current number of records that you have created in the specified hosted zone.</p>
    pub count: i64,
    /// <p>The current setting for the specified limit. For example, if you specified <code>MAX_RRSETS_BY_ZONE</code> for the value of <code>Type</code> in the request, the value of <code>Limit</code> is the maximum number of records that you can create in the specified hosted zone.</p>
    pub limit: HostedZoneLimit,
}

struct GetHostedZoneLimitResponseDeserializer;
impl GetHostedZoneLimitResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHostedZoneLimitResponse, XmlParseError> {
        deserialize_elements::<_, GetHostedZoneLimitResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Count" => {
                        obj.count = UsageCountDeserializer::deserialize("Count", stack)?;
                    }
                    "Limit" => {
                        obj.limit = HostedZoneLimitDeserializer::deserialize("Limit", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A request to get information about a specified hosted zone. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetHostedZoneRequest {
    /// <p>The ID of the hosted zone that you want to get information about.</p>
    pub id: String,
}

/// <p>A complex type that contain the response to a <code>GetHostedZone</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetHostedZoneResponse, XmlParseError> {
        deserialize_elements::<_, GetHostedZoneResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DelegationSet" => {
                    obj.delegation_set = Some(DelegationSetDeserializer::deserialize(
                        "DelegationSet",
                        stack,
                    )?);
                }
                "HostedZone" => {
                    obj.hosted_zone = HostedZoneDeserializer::deserialize("HostedZone", stack)?;
                }
                "VPCs" => {
                    obj.vp_cs
                        .get_or_insert(vec![])
                        .extend(VPCsDeserializer::deserialize("VPCs", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetQueryLoggingConfigRequest {
    /// <p>The ID of the configuration for DNS query logging that you want to get information about.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetQueryLoggingConfigResponse {
    /// <p>A complex type that contains information about the query logging configuration that you specified in a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetQueryLoggingConfig.html">GetQueryLoggingConfig</a> request.</p>
    pub query_logging_config: QueryLoggingConfig,
}

struct GetQueryLoggingConfigResponseDeserializer;
impl GetQueryLoggingConfigResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetQueryLoggingConfigResponse, XmlParseError> {
        deserialize_elements::<_, GetQueryLoggingConfigResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "QueryLoggingConfig" => {
                        obj.query_logging_config = QueryLoggingConfigDeserializer::deserialize(
                            "QueryLoggingConfig",
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
/// <p>A complex type that contains information about the request to create a hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetReusableDelegationSetLimitRequest {
    /// <p>The ID of the delegation set that you want to get the limit for.</p>
    pub delegation_set_id: String,
    /// <p>Specify <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code> to get the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p>
    pub type_: String,
}

/// <p>A complex type that contains the requested limit. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetReusableDelegationSetLimitResponse {
    /// <p>The current number of hosted zones that you can associate with the specified reusable delegation set.</p>
    pub count: i64,
    /// <p>The current setting for the limit on hosted zones that you can associate with the specified reusable delegation set.</p>
    pub limit: ReusableDelegationSetLimit,
}

struct GetReusableDelegationSetLimitResponseDeserializer;
impl GetReusableDelegationSetLimitResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetReusableDelegationSetLimitResponse, XmlParseError> {
        deserialize_elements::<_, GetReusableDelegationSetLimitResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Count" => {
                        obj.count = UsageCountDeserializer::deserialize("Count", stack)?;
                    }
                    "Limit" => {
                        obj.limit =
                            ReusableDelegationSetLimitDeserializer::deserialize("Limit", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A request to get information about a specified reusable delegation set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetReusableDelegationSetRequest {
    /// <p>The ID of the reusable delegation set that you want to get a list of name servers for.</p>
    pub id: String,
}

/// <p>A complex type that contains the response to the <code>GetReusableDelegationSet</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetReusableDelegationSetResponse {
    /// <p>A complex type that contains information about the reusable delegation set.</p>
    pub delegation_set: DelegationSet,
}

struct GetReusableDelegationSetResponseDeserializer;
impl GetReusableDelegationSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetReusableDelegationSetResponse, XmlParseError> {
        deserialize_elements::<_, GetReusableDelegationSetResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DelegationSet" => {
                        obj.delegation_set =
                            DelegationSetDeserializer::deserialize("DelegationSet", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Request to get the number of traffic policy instances that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTrafficPolicyInstanceCountRequest {}

/// <p>A complex type that contains information about the resource record sets that Amazon Route 53 created based on a specified traffic policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetTrafficPolicyInstanceCountResponse {
    /// <p>The number of traffic policy instances that are associated with the current AWS account.</p>
    pub traffic_policy_instance_count: i64,
}

struct GetTrafficPolicyInstanceCountResponseDeserializer;
impl GetTrafficPolicyInstanceCountResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTrafficPolicyInstanceCountResponse, XmlParseError> {
        deserialize_elements::<_, GetTrafficPolicyInstanceCountResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TrafficPolicyInstanceCount" => {
                        obj.traffic_policy_instance_count =
                            TrafficPolicyInstanceCountDeserializer::deserialize(
                                "TrafficPolicyInstanceCount",
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
/// <p>Gets information about a specified traffic policy instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTrafficPolicyInstanceRequest {
    /// <p>The ID of the traffic policy instance that you want to get information about.</p>
    pub id: String,
}

/// <p>A complex type that contains information about the resource record sets that Amazon Route 53 created based on a specified traffic policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetTrafficPolicyInstanceResponse {
    /// <p>A complex type that contains settings for the traffic policy instance.</p>
    pub traffic_policy_instance: TrafficPolicyInstance,
}

struct GetTrafficPolicyInstanceResponseDeserializer;
impl GetTrafficPolicyInstanceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTrafficPolicyInstanceResponse, XmlParseError> {
        deserialize_elements::<_, GetTrafficPolicyInstanceResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TrafficPolicyInstance" => {
                        obj.traffic_policy_instance =
                            TrafficPolicyInstanceDeserializer::deserialize(
                                "TrafficPolicyInstance",
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
/// <p>Gets information about a specific traffic policy version.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTrafficPolicyRequest {
    /// <p>The ID of the traffic policy that you want to get information about.</p>
    pub id: String,
    /// <p>The version number of the traffic policy that you want to get information about.</p>
    pub version: i64,
}

/// <p>A complex type that contains the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetTrafficPolicyResponse {
    /// <p>A complex type that contains settings for the specified traffic policy.</p>
    pub traffic_policy: TrafficPolicy,
}

struct GetTrafficPolicyResponseDeserializer;
impl GetTrafficPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTrafficPolicyResponse, XmlParseError> {
        deserialize_elements::<_, GetTrafficPolicyResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TrafficPolicy" => {
                        obj.traffic_policy =
                            TrafficPolicyDeserializer::deserialize("TrafficPolicy", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A complex type that contains information about one health check that is associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HealthCheck, XmlParseError> {
        deserialize_elements::<_, HealthCheck, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CallerReference" => {
                    obj.caller_reference =
                        HealthCheckNonceDeserializer::deserialize("CallerReference", stack)?;
                }
                "CloudWatchAlarmConfiguration" => {
                    obj.cloud_watch_alarm_configuration =
                        Some(CloudWatchAlarmConfigurationDeserializer::deserialize(
                            "CloudWatchAlarmConfiguration",
                            stack,
                        )?);
                }
                "HealthCheckConfig" => {
                    obj.health_check_config =
                        HealthCheckConfigDeserializer::deserialize("HealthCheckConfig", stack)?;
                }
                "HealthCheckVersion" => {
                    obj.health_check_version =
                        HealthCheckVersionDeserializer::deserialize("HealthCheckVersion", stack)?;
                }
                "Id" => {
                    obj.id = HealthCheckIdDeserializer::deserialize("Id", stack)?;
                }
                "LinkedService" => {
                    obj.linked_service = Some(LinkedServiceDeserializer::deserialize(
                        "LinkedService",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A complex type that contains information about the health check.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HealthCheckConfig {
    /// <p>A complex type that identifies the CloudWatch alarm that you want Amazon Route 53 health checkers to use to determine whether the specified health check is healthy.</p>
    pub alarm_identifier: Option<AlarmIdentifier>,
    /// <p>(CALCULATED Health Checks Only) A complex type that contains one <code>ChildHealthCheck</code> element for each health check that you want to associate with a <code>CALCULATED</code> health check.</p>
    pub child_health_checks: Option<Vec<String>>,
    /// <p>Stops Route 53 from performing health checks. When you disable a health check, here's what happens:</p> <ul> <li> <p> <b>Health checks that check the health of endpoints:</b> Route 53 stops submitting requests to your application, server, or other resource.</p> </li> <li> <p> <b>Calculated health checks:</b> Route 53 stops aggregating the status of the referenced health checks.</p> </li> <li> <p> <b>Health checks that monitor CloudWatch alarms:</b> Route 53 stops monitoring the corresponding CloudWatch metrics.</p> </li> </ul> <p>After you disable a health check, Route 53 considers the status of the health check to always be healthy. If you configured DNS failover, Route 53 continues to route traffic to the corresponding resources. If you want to stop routing traffic to a resource, change the value of <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_UpdateHealthCheck.html#Route53-UpdateHealthCheck-request-Inverted">Inverted</a>. </p> <p>Charges for a health check still apply when the health check is disabled. For more information, see <a href="http://aws.amazon.com/route53/pricing/">Amazon Route 53 Pricing</a>.</p>
    pub disabled: Option<bool>,
    /// <p>Specify whether you want Amazon Route 53 to send the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>client_hello</code> message during TLS negotiation. This allows the endpoint to respond to <code>HTTPS</code> health check requests with the applicable SSL/TLS certificate.</p> <p>Some endpoints require that <code>HTTPS</code> requests include the host name in the <code>client_hello</code> message. If you don't enable SNI, the status of the health check will be <code>SSL alert handshake_failure</code>. A health check can also have that status for other reasons. If SNI is enabled and you're still getting the error, check the SSL/TLS configuration on your endpoint and confirm that your certificate is valid.</p> <p>The SSL/TLS certificate on your endpoint includes a domain name in the <code>Common Name</code> field and possibly several more in the <code>Subject Alternative Names</code> field. One of the domain names in the certificate should match the value that you specify for <code>FullyQualifiedDomainName</code>. If the endpoint responds to the <code>client_hello</code> message with a certificate that does not include the domain name that you specified in <code>FullyQualifiedDomainName</code>, a health checker will retry the handshake. In the second attempt, the health checker will omit <code>FullyQualifiedDomainName</code> from the <code>client_hello</code> message.</p>
    pub enable_sni: Option<bool>,
    /// <p>The number of consecutive health checks that an endpoint must pass or fail for Amazon Route 53 to change the current status of the endpoint from unhealthy to healthy or vice versa. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html">How Amazon Route 53 Determines Whether an Endpoint Is Healthy</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If you don't specify a value for <code>FailureThreshold</code>, the default value is three health checks.</p>
    pub failure_threshold: Option<i64>,
    /// <p>Amazon Route 53 behavior depends on whether you specify a value for <code>IPAddress</code>.</p> <p> <b>If you specify a value for</b> <code>IPAddress</code>:</p> <p>Amazon Route 53 sends health check requests to the specified IPv4 or IPv6 address and passes the value of <code>FullyQualifiedDomainName</code> in the <code>Host</code> header for all health checks except TCP health checks. This is typically the fully qualified DNS name of the endpoint on which you want Route 53 to perform health checks.</p> <p>When Route 53 checks the health of an endpoint, here is how it constructs the <code>Host</code> header:</p> <ul> <li> <p>If you specify a value of <code>80</code> for <code>Port</code> and <code>HTTP</code> or <code>HTTP_STR_MATCH</code> for <code>Type</code>, Route 53 passes the value of <code>FullyQualifiedDomainName</code> to the endpoint in the Host header. </p> </li> <li> <p>If you specify a value of <code>443</code> for <code>Port</code> and <code>HTTPS</code> or <code>HTTPS_STR_MATCH</code> for <code>Type</code>, Route 53 passes the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>Host</code> header.</p> </li> <li> <p>If you specify another value for <code>Port</code> and any value except <code>TCP</code> for <code>Type</code>, Route 53 passes <code>FullyQualifiedDomainName:Port</code> to the endpoint in the <code>Host</code> header.</p> </li> </ul> <p>If you don't specify a value for <code>FullyQualifiedDomainName</code>, Route 53 substitutes the value of <code>IPAddress</code> in the <code>Host</code> header in each of the preceding cases.</p> <p> <b>If you don't specify a value for <code>IPAddress</code> </b>:</p> <p>Route 53 sends a DNS request to the domain that you specify for <code>FullyQualifiedDomainName</code> at the interval that you specify for <code>RequestInterval</code>. Using an IPv4 address that DNS returns, Route 53 then checks the health of the endpoint.</p> <note> <p>If you don't specify a value for <code>IPAddress</code>, Route 53 uses only IPv4 to send health checks to the endpoint. If there's no resource record set with a type of A for the name that you specify for <code>FullyQualifiedDomainName</code>, the health check fails with a "DNS resolution failed" error.</p> </note> <p>If you want to check the health of weighted, latency, or failover resource record sets and you choose to specify the endpoint only by <code>FullyQualifiedDomainName</code>, we recommend that you create a separate health check for each endpoint. For example, create a health check for each HTTP server that is serving content for www.example.com. For the value of <code>FullyQualifiedDomainName</code>, specify the domain name of the server (such as us-east-2-www.example.com), not the name of the resource record sets (www.example.com).</p> <important> <p>In this configuration, if you create a health check for which the value of <code>FullyQualifiedDomainName</code> matches the name of the resource record sets and you then associate the health check with those resource record sets, health check results will be unpredictable.</p> </important> <p>In addition, if the value that you specify for <code>Type</code> is <code>HTTP</code>, <code>HTTPS</code>, <code>HTTP_STR_MATCH</code>, or <code>HTTPS_STR_MATCH</code>, Route 53 passes the value of <code>FullyQualifiedDomainName</code> in the <code>Host</code> header, as it does when you specify a value for <code>IPAddress</code>. If the value of <code>Type</code> is <code>TCP</code>, Route 53 doesn't pass a <code>Host</code> header.</p>
    pub fully_qualified_domain_name: Option<String>,
    /// <p><p>The number of child health checks that are associated with a <code>CALCULATED</code> health check that Amazon Route 53 must consider healthy for the <code>CALCULATED</code> health check to be considered healthy. To specify the child health checks that you want to associate with a <code>CALCULATED</code> health check, use the <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_UpdateHealthCheck.html#Route53-UpdateHealthCheck-request-ChildHealthChecks">ChildHealthChecks</a> element.</p> <p>Note the following:</p> <ul> <li> <p>If you specify a number greater than the number of child health checks, Route 53 always considers this health check to be unhealthy.</p> </li> <li> <p>If you specify <code>0</code>, Route 53 always considers this health check to be healthy.</p> </li> </ul></p>
    pub health_threshold: Option<i64>,
    /// <p>The IPv4 or IPv6 IP address of the endpoint that you want Amazon Route 53 to perform health checks on. If you don't specify a value for <code>IPAddress</code>, Route 53 sends a DNS request to resolve the domain name that you specify in <code>FullyQualifiedDomainName</code> at the interval that you specify in <code>RequestInterval</code>. Using an IP address returned by DNS, Route 53 then checks the health of the endpoint.</p> <p>Use one of the following formats for the value of <code>IPAddress</code>: </p> <ul> <li> <p> <b>IPv4 address</b>: four values between 0 and 255, separated by periods (.), for example, <code>192.0.2.44</code>.</p> </li> <li> <p> <b>IPv6 address</b>: eight groups of four hexadecimal values, separated by colons (:), for example, <code>2001:0db8:85a3:0000:0000:abcd:0001:2345</code>. You can also shorten IPv6 addresses as described in RFC 5952, for example, <code>2001:db8:85a3::abcd:1:2345</code>.</p> </li> </ul> <p>If the endpoint is an EC2 instance, we recommend that you create an Elastic IP address, associate it with your EC2 instance, and specify the Elastic IP address for <code>IPAddress</code>. This ensures that the IP address of your instance will never change.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_UpdateHealthCheck.html#Route53-UpdateHealthCheck-request-FullyQualifiedDomainName">FullyQualifiedDomainName</a>. </p> <p>Constraints: Route 53 can't check the health of endpoints for which the IP address is in local, private, non-routable, or multicast ranges. For more information about IP addresses for which you can't create health checks, see the following documents:</p> <ul> <li> <p> <a href="https://tools.ietf.org/html/rfc5735">RFC 5735, Special Use IPv4 Addresses</a> </p> </li> <li> <p> <a href="https://tools.ietf.org/html/rfc6598">RFC 6598, IANA-Reserved IPv4 Prefix for Shared Address Space</a> </p> </li> <li> <p> <a href="https://tools.ietf.org/html/rfc5156">RFC 5156, Special-Use IPv6 Addresses</a> </p> </li> </ul> <p>When the value of <code>Type</code> is <code>CALCULATED</code> or <code>CLOUDWATCH_METRIC</code>, omit <code>IPAddress</code>.</p>
    pub ip_address: Option<String>,
    /// <p><p>When CloudWatch has insufficient data about the metric to determine the alarm state, the status that you want Amazon Route 53 to assign to the health check:</p> <ul> <li> <p> <code>Healthy</code>: Route 53 considers the health check to be healthy.</p> </li> <li> <p> <code>Unhealthy</code>: Route 53 considers the health check to be unhealthy.</p> </li> <li> <p> <code>LastKnownStatus</code>: Route 53 uses the status of the health check from the last time that CloudWatch had sufficient data to determine the alarm state. For new health checks that have no last known status, the default status for the health check is healthy.</p> </li> </ul></p>
    pub insufficient_data_health_status: Option<String>,
    /// <p>Specify whether you want Amazon Route 53 to invert the status of a health check, for example, to consider a health check unhealthy when it otherwise would be considered healthy.</p>
    pub inverted: Option<bool>,
    /// <p><p>Specify whether you want Amazon Route 53 to measure the latency between health checkers in multiple AWS regions and your endpoint, and to display CloudWatch latency graphs on the <b>Health Checks</b> page in the Route 53 console.</p> <important> <p>You can&#39;t change the value of <code>MeasureLatency</code> after you create a health check.</p> </important></p>
    pub measure_latency: Option<bool>,
    /// <p>The port on the endpoint on which you want Amazon Route 53 to perform health checks. Specify a value for <code>Port</code> only when you specify a value for <code>IPAddress</code>.</p>
    pub port: Option<i64>,
    /// <p>A complex type that contains one <code>Region</code> element for each region from which you want Amazon Route 53 health checkers to check the specified endpoint.</p> <p>If you don't specify any regions, Route 53 health checkers automatically performs checks from all of the regions that are listed under <b>Valid Values</b>.</p> <p>If you update a health check to remove a region that has been performing health checks, Route 53 will briefly continue to perform checks from that region to ensure that some health checkers are always checking the endpoint (for example, if you replace three regions with four different regions). </p>
    pub regions: Option<Vec<String>>,
    /// <p>The number of seconds between the time that Amazon Route 53 gets a response from your endpoint and the time that it sends the next health check request. Each Route 53 health checker makes requests at this interval.</p> <important> <p>You can't change the value of <code>RequestInterval</code> after you create a health check.</p> </important> <p>If you don't specify a value for <code>RequestInterval</code>, the default value is <code>30</code> seconds.</p>
    pub request_interval: Option<i64>,
    /// <p>The path, if any, that you want Amazon Route 53 to request when performing health checks. The path can be any value for which your endpoint will return an HTTP status code of 2xx or 3xx when the endpoint is healthy, for example, the file /docs/route53-health-check.html. You can also include query string parameters, for example, <code>/welcome.html?language=jp&amp;login=y</code>. </p>
    pub resource_path: Option<String>,
    /// <p>If the value of Type is <code>HTTP_STR_MATCH</code> or <code>HTTP_STR_MATCH</code>, the string that you want Amazon Route 53 to search for in the response body from the specified resource. If the string appears in the response body, Route 53 considers the resource healthy.</p> <p>Route 53 considers case when searching for <code>SearchString</code> in the response body. </p>
    pub search_string: Option<String>,
    /// <p>The type of health check that you want to create, which indicates how Amazon Route 53 determines whether an endpoint is healthy.</p> <important> <p>You can't change the value of <code>Type</code> after you create a health check.</p> </important> <p>You can create the following types of health checks:</p> <ul> <li> <p> <b>HTTP</b>: Route 53 tries to establish a TCP connection. If successful, Route 53 submits an HTTP request and waits for an HTTP status code of 200 or greater and less than 400.</p> </li> <li> <p> <b>HTTPS</b>: Route 53 tries to establish a TCP connection. If successful, Route 53 submits an HTTPS request and waits for an HTTP status code of 200 or greater and less than 400.</p> <important> <p>If you specify <code>HTTPS</code> for the value of <code>Type</code>, the endpoint must support TLS v1.0 or later.</p> </important> </li> <li> <p> <b>HTTP_STR_MATCH</b>: Route 53 tries to establish a TCP connection. If successful, Route 53 submits an HTTP request and searches the first 5,120 bytes of the response body for the string that you specify in <code>SearchString</code>.</p> </li> <li> <p> <b>HTTPS_STR_MATCH</b>: Route 53 tries to establish a TCP connection. If successful, Route 53 submits an <code>HTTPS</code> request and searches the first 5,120 bytes of the response body for the string that you specify in <code>SearchString</code>.</p> </li> <li> <p> <b>TCP</b>: Route 53 tries to establish a TCP connection.</p> </li> <li> <p> <b>CLOUDWATCH_METRIC</b>: The health check is associated with a CloudWatch alarm. If the state of the alarm is <code>OK</code>, the health check is considered healthy. If the state is <code>ALARM</code>, the health check is considered unhealthy. If CloudWatch doesn't have sufficient data to determine whether the state is <code>OK</code> or <code>ALARM</code>, the health check status depends on the setting for <code>InsufficientDataHealthStatus</code>: <code>Healthy</code>, <code>Unhealthy</code>, or <code>LastKnownStatus</code>. </p> </li> <li> <p> <b>CALCULATED</b>: For health checks that monitor the status of other health checks, Route 53 adds up the number of health checks that Route 53 health checkers consider to be healthy and compares that number with the value of <code>HealthThreshold</code>. </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html">How Route 53 Determines Whether an Endpoint Is Healthy</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    pub type_: String,
}

struct HealthCheckConfigDeserializer;
impl HealthCheckConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HealthCheckConfig, XmlParseError> {
        deserialize_elements::<_, HealthCheckConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AlarmIdentifier" => {
                    obj.alarm_identifier = Some(AlarmIdentifierDeserializer::deserialize(
                        "AlarmIdentifier",
                        stack,
                    )?);
                }
                "ChildHealthChecks" => {
                    obj.child_health_checks.get_or_insert(vec![]).extend(
                        ChildHealthCheckListDeserializer::deserialize("ChildHealthChecks", stack)?,
                    );
                }
                "Disabled" => {
                    obj.disabled = Some(DisabledDeserializer::deserialize("Disabled", stack)?);
                }
                "EnableSNI" => {
                    obj.enable_sni = Some(EnableSNIDeserializer::deserialize("EnableSNI", stack)?);
                }
                "FailureThreshold" => {
                    obj.failure_threshold = Some(FailureThresholdDeserializer::deserialize(
                        "FailureThreshold",
                        stack,
                    )?);
                }
                "FullyQualifiedDomainName" => {
                    obj.fully_qualified_domain_name =
                        Some(FullyQualifiedDomainNameDeserializer::deserialize(
                            "FullyQualifiedDomainName",
                            stack,
                        )?);
                }
                "HealthThreshold" => {
                    obj.health_threshold = Some(HealthThresholdDeserializer::deserialize(
                        "HealthThreshold",
                        stack,
                    )?);
                }
                "IPAddress" => {
                    obj.ip_address = Some(IPAddressDeserializer::deserialize("IPAddress", stack)?);
                }
                "InsufficientDataHealthStatus" => {
                    obj.insufficient_data_health_status =
                        Some(InsufficientDataHealthStatusDeserializer::deserialize(
                            "InsufficientDataHealthStatus",
                            stack,
                        )?);
                }
                "Inverted" => {
                    obj.inverted = Some(InvertedDeserializer::deserialize("Inverted", stack)?);
                }
                "MeasureLatency" => {
                    obj.measure_latency = Some(MeasureLatencyDeserializer::deserialize(
                        "MeasureLatency",
                        stack,
                    )?);
                }
                "Port" => {
                    obj.port = Some(PortDeserializer::deserialize("Port", stack)?);
                }
                "Regions" => {
                    obj.regions.get_or_insert(vec![]).extend(
                        HealthCheckRegionListDeserializer::deserialize("Regions", stack)?,
                    );
                }
                "RequestInterval" => {
                    obj.request_interval = Some(RequestIntervalDeserializer::deserialize(
                        "RequestInterval",
                        stack,
                    )?);
                }
                "ResourcePath" => {
                    obj.resource_path = Some(ResourcePathDeserializer::deserialize(
                        "ResourcePath",
                        stack,
                    )?);
                }
                "SearchString" => {
                    obj.search_string = Some(SearchStringDeserializer::deserialize(
                        "SearchString",
                        stack,
                    )?);
                }
                "Type" => {
                    obj.type_ = HealthCheckTypeDeserializer::deserialize("Type", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
        if let Some(ref value) = obj.disabled {
            writer.write(xml::writer::XmlEvent::start_element("Disabled"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct HealthCheckIdDeserializer;
impl HealthCheckIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HealthCheckObservation, XmlParseError> {
        deserialize_elements::<_, HealthCheckObservation, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "IPAddress" => {
                    obj.ip_address = Some(IPAddressDeserializer::deserialize("IPAddress", stack)?);
                }
                "Region" => {
                    obj.region = Some(HealthCheckRegionDeserializer::deserialize("Region", stack)?);
                }
                "StatusReport" => {
                    obj.status_report = Some(StatusReportDeserializer::deserialize(
                        "StatusReport",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct HealthCheckObservationsDeserializer;
impl HealthCheckObservationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<HealthCheckObservation>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "HealthCheckObservation" {
                obj.push(HealthCheckObservationDeserializer::deserialize(
                    "HealthCheckObservation",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct HealthCheckRegionDeserializer;
impl HealthCheckRegionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Region" {
                obj.push(HealthCheckRegionDeserializer::deserialize("Region", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<HealthCheck>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "HealthCheck" {
                obj.push(HealthCheckDeserializer::deserialize("HealthCheck", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct HealthThresholdDeserializer;
impl HealthThresholdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct HostedZone {
    /// <p>The value that you specified for <code>CallerReference</code> when you created the hosted zone.</p>
    pub caller_reference: String,
    /// <p>A complex type that includes the <code>Comment</code> and <code>PrivateZone</code> elements. If you omitted the <code>HostedZoneConfig</code> and <code>Comment</code> elements from the request, the <code>Config</code> and <code>Comment</code> elements don't appear in the response.</p>
    pub config: Option<HostedZoneConfig>,
    /// <p>The ID that Amazon Route 53 assigned to the hosted zone when you created it.</p>
    pub id: String,
    /// <p>If the hosted zone was created by another service, the service that created the hosted zone. When a hosted zone is created by another service, you can't edit or delete it using Route 53. </p>
    pub linked_service: Option<LinkedService>,
    /// <p>The name of the domain. For public hosted zones, this is the name that you have registered with your DNS registrar.</p> <p>For information about how to specify characters other than <code>a-z</code>, <code>0-9</code>, and <code>-</code> (hyphen) and how to specify internationalized domain names, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateHostedZone.html">CreateHostedZone</a>.</p>
    pub name: String,
    /// <p>The number of resource record sets in the hosted zone.</p>
    pub resource_record_set_count: Option<i64>,
}

struct HostedZoneDeserializer;
impl HostedZoneDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HostedZone, XmlParseError> {
        deserialize_elements::<_, HostedZone, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CallerReference" => {
                    obj.caller_reference =
                        NonceDeserializer::deserialize("CallerReference", stack)?;
                }
                "Config" => {
                    obj.config = Some(HostedZoneConfigDeserializer::deserialize("Config", stack)?);
                }
                "Id" => {
                    obj.id = ResourceIdDeserializer::deserialize("Id", stack)?;
                }
                "LinkedService" => {
                    obj.linked_service = Some(LinkedServiceDeserializer::deserialize(
                        "LinkedService",
                        stack,
                    )?);
                }
                "Name" => {
                    obj.name = DNSNameDeserializer::deserialize("Name", stack)?;
                }
                "ResourceRecordSetCount" => {
                    obj.resource_record_set_count =
                        Some(HostedZoneRRSetCountDeserializer::deserialize(
                            "ResourceRecordSetCount",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A complex type that contains an optional comment about your hosted zone. If you don't want to specify a comment, omit both the <code>HostedZoneConfig</code> and <code>Comment</code> elements.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HostedZoneConfig {
    /// <p>Any comments that you want to include about the hosted zone.</p>
    pub comment: Option<String>,
    /// <p>A value that indicates whether this is a private hosted zone.</p>
    pub private_zone: Option<bool>,
}

struct HostedZoneConfigDeserializer;
impl HostedZoneConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HostedZoneConfig, XmlParseError> {
        deserialize_elements::<_, HostedZoneConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Comment" => {
                    obj.comment = Some(ResourceDescriptionDeserializer::deserialize(
                        "Comment", stack,
                    )?);
                }
                "PrivateZone" => {
                    obj.private_zone = Some(IsPrivateZoneDeserializer::deserialize(
                        "PrivateZone",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A complex type that contains the type of limit that you specified in the request and the current value for that limit.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct HostedZoneLimit {
    /// <p><p>The limit that you requested. Valid values include the following:</p> <ul> <li> <p> <b>MAX<em>RRSETS</em>BY<em>ZONE</b>: The maximum number of records that you can create in the specified hosted zone.</p> </li> <li> <p> <b>MAX</em>VPCS<em>ASSOCIATED</em>BY_ZONE</b>: The maximum number of Amazon VPCs that you can associate with the specified private hosted zone.</p> </li> </ul></p>
    pub type_: String,
    /// <p>The current value for the limit that is specified by <code>Type</code>.</p>
    pub value: i64,
}

struct HostedZoneLimitDeserializer;
impl HostedZoneLimitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HostedZoneLimit, XmlParseError> {
        deserialize_elements::<_, HostedZoneLimit, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Type" => {
                    obj.type_ = HostedZoneLimitTypeDeserializer::deserialize("Type", stack)?;
                }
                "Value" => {
                    obj.value = LimitValueDeserializer::deserialize("Value", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct HostedZoneLimitTypeDeserializer;
impl HostedZoneLimitTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct HostedZonesDeserializer;
impl HostedZonesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<HostedZone>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "HostedZone" {
                obj.push(HostedZoneDeserializer::deserialize("HostedZone", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct IPAddressDeserializer;
impl IPAddressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct InsufficientDataHealthStatusDeserializer;
impl InsufficientDataHealthStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>If a health check or hosted zone was created by another service, <code>LinkedService</code> is a complex type that describes the service that created the resource. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct LinkedService {
    /// <p>If the health check or hosted zone was created by another service, an optional description that can be provided by the other service. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    pub description: Option<String>,
    /// <p>If the health check or hosted zone was created by another service, the service that created the resource. When a resource is created by another service, you can't edit or delete it using Amazon Route 53. </p>
    pub service_principal: Option<String>,
}

struct LinkedServiceDeserializer;
impl LinkedServiceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LinkedService, XmlParseError> {
        deserialize_elements::<_, LinkedService, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Description" => {
                    obj.description = Some(ResourceDescriptionDeserializer::deserialize(
                        "Description",
                        stack,
                    )?);
                }
                "ServicePrincipal" => {
                    obj.service_principal = Some(ServicePrincipalDeserializer::deserialize(
                        "ServicePrincipal",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>A request to get a list of geographic locations that Amazon Route 53 supports for geolocation resource record sets. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGeoLocationsRequest {
    /// <p>(Optional) The maximum number of geolocations to be included in the response body for this request. If more than <code>maxitems</code> geolocations remain to be listed, then the value of the <code>IsTruncated</code> element in the response is <code>true</code>.</p>
    pub max_items: Option<String>,
    /// <p>The code for the continent with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is true, and if <code>NextContinentCode</code> from the previous response has a value, enter that value in <code>startcontinentcode</code> to return the next page of results.</p> <p>Include <code>startcontinentcode</code> only if you want to list continents. Don't include <code>startcontinentcode</code> when you're listing countries or countries with their subdivisions.</p>
    pub start_continent_code: Option<String>,
    /// <p>The code for the country with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextCountryCode</code> from the previous response has a value, enter that value in <code>startcountrycode</code> to return the next page of results.</p> <p>Route 53 uses the two-letter country codes that are specified in <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO standard 3166-1 alpha-2</a>.</p>
    pub start_country_code: Option<String>,
    /// <p>The code for the subdivision (for example, state or province) with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextSubdivisionCode</code> from the previous response has a value, enter that value in <code>startsubdivisioncode</code> to return the next page of results.</p> <p>To list subdivisions of a country, you must include both <code>startcountrycode</code> and <code>startsubdivisioncode</code>.</p>
    pub start_subdivision_code: Option<String>,
}

/// <p>A complex type containing the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListGeoLocationsResponse {
    /// <p>A complex type that contains one <code>GeoLocationDetails</code> element for each location that Amazon Route 53 supports for geolocation.</p>
    pub geo_location_details_list: Vec<GeoLocationDetails>,
    /// <p>A value that indicates whether more locations remain to be listed after the last location in this response. If so, the value of <code>IsTruncated</code> is <code>true</code>. To get more values, submit another request and include the values of <code>NextContinentCode</code>, <code>NextCountryCode</code>, and <code>NextSubdivisionCode</code> in the <code>startcontinentcode</code>, <code>startcountrycode</code>, and <code>startsubdivisioncode</code>, as applicable.</p>
    pub is_truncated: bool,
    /// <p>The value that you specified for <code>MaxItems</code> in the request.</p>
    pub max_items: String,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, you can make a follow-up request to display more locations. Enter the value of <code>NextContinentCode</code> in the <code>startcontinentcode</code> parameter in another <code>ListGeoLocations</code> request.</p>
    pub next_continent_code: Option<String>,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, you can make a follow-up request to display more locations. Enter the value of <code>NextCountryCode</code> in the <code>startcountrycode</code> parameter in another <code>ListGeoLocations</code> request.</p>
    pub next_country_code: Option<String>,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, you can make a follow-up request to display more locations. Enter the value of <code>NextSubdivisionCode</code> in the <code>startsubdivisioncode</code> parameter in another <code>ListGeoLocations</code> request.</p>
    pub next_subdivision_code: Option<String>,
}

struct ListGeoLocationsResponseDeserializer;
impl ListGeoLocationsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListGeoLocationsResponse, XmlParseError> {
        deserialize_elements::<_, ListGeoLocationsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "GeoLocationDetailsList" => {
                        obj.geo_location_details_list.extend(
                            GeoLocationDetailsListDeserializer::deserialize(
                                "GeoLocationDetailsList",
                                stack,
                            )?,
                        );
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            PageTruncatedDeserializer::deserialize("IsTruncated", stack)?;
                    }
                    "MaxItems" => {
                        obj.max_items = PageMaxItemsDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "NextContinentCode" => {
                        obj.next_continent_code =
                            Some(GeoLocationContinentCodeDeserializer::deserialize(
                                "NextContinentCode",
                                stack,
                            )?);
                    }
                    "NextCountryCode" => {
                        obj.next_country_code =
                            Some(GeoLocationCountryCodeDeserializer::deserialize(
                                "NextCountryCode",
                                stack,
                            )?);
                    }
                    "NextSubdivisionCode" => {
                        obj.next_subdivision_code =
                            Some(GeoLocationSubdivisionCodeDeserializer::deserialize(
                                "NextSubdivisionCode",
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
/// <p>A request to retrieve a list of the health checks that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListHealthChecksRequest {
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more health checks. To get another group, submit another <code>ListHealthChecks</code> request. </p> <p>For the value of <code>marker</code>, specify the value of <code>NextMarker</code> from the previous response, which is the ID of the first health check that Amazon Route 53 will return if you submit another request.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more health checks to get.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of health checks that you want <code>ListHealthChecks</code> to return in response to the current request. Amazon Route 53 returns a maximum of 100 items. If you set <code>MaxItems</code> to a value greater than 100, Route 53 returns only the first 100 health checks. </p>
    pub max_items: Option<String>,
}

/// <p>A complex type that contains the response to a <code>ListHealthChecks</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListHealthChecksResponse, XmlParseError> {
        deserialize_elements::<_, ListHealthChecksResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HealthChecks" => {
                        obj.health_checks
                            .extend(HealthChecksDeserializer::deserialize(
                                "HealthChecks",
                                stack,
                            )?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            PageTruncatedDeserializer::deserialize("IsTruncated", stack)?;
                    }
                    "Marker" => {
                        obj.marker = PageMarkerDeserializer::deserialize("Marker", stack)?;
                    }
                    "MaxItems" => {
                        obj.max_items = PageMaxItemsDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(PageMarkerDeserializer::deserialize("NextMarker", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Retrieves a list of the public and private hosted zones that are associated with the current AWS account in ASCII order by domain name. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListHostedZonesByNameResponse, XmlParseError> {
        deserialize_elements::<_, ListHostedZonesByNameResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DNSName" => {
                        obj.dns_name = Some(DNSNameDeserializer::deserialize("DNSName", stack)?);
                    }
                    "HostedZoneId" => {
                        obj.hosted_zone_id =
                            Some(ResourceIdDeserializer::deserialize("HostedZoneId", stack)?);
                    }
                    "HostedZones" => {
                        obj.hosted_zones
                            .extend(HostedZonesDeserializer::deserialize("HostedZones", stack)?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            PageTruncatedDeserializer::deserialize("IsTruncated", stack)?;
                    }
                    "MaxItems" => {
                        obj.max_items = PageMaxItemsDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "NextDNSName" => {
                        obj.next_dns_name =
                            Some(DNSNameDeserializer::deserialize("NextDNSName", stack)?);
                    }
                    "NextHostedZoneId" => {
                        obj.next_hosted_zone_id = Some(ResourceIdDeserializer::deserialize(
                            "NextHostedZoneId",
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
/// <p>A request to retrieve a list of the public and private hosted zones that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListHostedZonesRequest {
    /// <p>If you're using reusable delegation sets and you want to list all of the hosted zones that are associated with a reusable delegation set, specify the ID of that reusable delegation set. </p>
    pub delegation_set_id: Option<String>,
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more hosted zones. To get more hosted zones, submit another <code>ListHostedZones</code> request. </p> <p>For the value of <code>marker</code>, specify the value of <code>NextMarker</code> from the previous response, which is the ID of the first hosted zone that Amazon Route 53 will return if you submit another request.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more hosted zones to get.</p>
    pub marker: Option<String>,
    /// <p>(Optional) The maximum number of hosted zones that you want Amazon Route 53 to return. If you have more than <code>maxitems</code> hosted zones, the value of <code>IsTruncated</code> in the response is <code>true</code>, and the value of <code>NextMarker</code> is the hosted zone ID of the first hosted zone that Route 53 will return if you submit another request.</p>
    pub max_items: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListHostedZonesResponse, XmlParseError> {
        deserialize_elements::<_, ListHostedZonesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HostedZones" => {
                        obj.hosted_zones
                            .extend(HostedZonesDeserializer::deserialize("HostedZones", stack)?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            PageTruncatedDeserializer::deserialize("IsTruncated", stack)?;
                    }
                    "Marker" => {
                        obj.marker = PageMarkerDeserializer::deserialize("Marker", stack)?;
                    }
                    "MaxItems" => {
                        obj.max_items = PageMaxItemsDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(PageMarkerDeserializer::deserialize("NextMarker", stack)?);
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
pub struct ListQueryLoggingConfigsRequest {
    /// <p>(Optional) If you want to list the query logging configuration that is associated with a hosted zone, specify the ID in <code>HostedZoneId</code>. </p> <p>If you don't specify a hosted zone ID, <code>ListQueryLoggingConfigs</code> returns all of the configurations that are associated with the current AWS account.</p>
    pub hosted_zone_id: Option<String>,
    /// <p>(Optional) The maximum number of query logging configurations that you want Amazon Route 53 to return in response to the current request. If the current AWS account has more than <code>MaxResults</code> configurations, use the value of <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ListQueryLoggingConfigs.html#API_ListQueryLoggingConfigs_RequestSyntax">NextToken</a> in the response to get the next page of results.</p> <p>If you don't specify a value for <code>MaxResults</code>, Route 53 returns up to 100 configurations.</p>
    pub max_results: Option<String>,
    /// <p>(Optional) If the current AWS account has more than <code>MaxResults</code> query logging configurations, use <code>NextToken</code> to get the second and subsequent pages of results.</p> <p>For the first <code>ListQueryLoggingConfigs</code> request, omit this value.</p> <p>For the second and subsequent requests, get the value of <code>NextToken</code> from the previous response and specify that value for <code>NextToken</code> in the request.</p>
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListQueryLoggingConfigsResponse {
    /// <p>If a response includes the last of the query logging configurations that are associated with the current AWS account, <code>NextToken</code> doesn't appear in the response.</p> <p>If a response doesn't include the last of the configurations, you can get more configurations by submitting another <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ListQueryLoggingConfigs.html">ListQueryLoggingConfigs</a> request. Get the value of <code>NextToken</code> that Amazon Route 53 returned in the previous response and include it in <code>NextToken</code> in the next request.</p>
    pub next_token: Option<String>,
    /// <p>An array that contains one <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_QueryLoggingConfig.html">QueryLoggingConfig</a> element for each configuration for DNS query logging that is associated with the current AWS account.</p>
    pub query_logging_configs: Vec<QueryLoggingConfig>,
}

struct ListQueryLoggingConfigsResponseDeserializer;
impl ListQueryLoggingConfigsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListQueryLoggingConfigsResponse, XmlParseError> {
        deserialize_elements::<_, ListQueryLoggingConfigsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextToken" => {
                        obj.next_token = Some(PaginationTokenDeserializer::deserialize(
                            "NextToken",
                            stack,
                        )?);
                    }
                    "QueryLoggingConfigs" => {
                        obj.query_logging_configs.extend(
                            QueryLoggingConfigsDeserializer::deserialize(
                                "QueryLoggingConfigs",
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
/// <p>A request for the resource record sets that are associated with a specified hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListResourceRecordSetsRequest {
    /// <p>The ID of the hosted zone that contains the resource record sets that you want to list.</p>
    pub hosted_zone_id: String,
    /// <p>(Optional) The maximum number of resource records sets to include in the response body for this request. If the response includes more than <code>maxitems</code> resource record sets, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of the <code>NextRecordName</code> and <code>NextRecordType</code> elements in the response identify the first resource record set in the next group of <code>maxitems</code> resource record sets.</p>
    pub max_items: Option<String>,
    /// <p> <i>Resource record sets that have a routing policy other than simple:</i> If results were truncated for a given DNS name and type, specify the value of <code>NextRecordIdentifier</code> from the previous response to get the next resource record set that has the current DNS name and type.</p>
    pub start_record_identifier: Option<String>,
    /// <p>The first name in the lexicographic ordering of resource record sets that you want to list.</p>
    pub start_record_name: Option<String>,
    /// <p>The type of resource record set to begin the record listing from.</p> <p>Valid values for basic resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>NS</code> | <code>PTR</code> | <code>SOA</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code> </p> <p>Values for weighted, latency, geolocation, and failover resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>PTR</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code> </p> <p>Values for alias resource record sets: </p> <ul> <li> <p> <b>API Gateway custom regional API or edge-optimized API</b>: A</p> </li> <li> <p> <b>CloudFront distribution</b>: A or AAAA</p> </li> <li> <p> <b>Elastic Beanstalk environment that has a regionalized subdomain</b>: A</p> </li> <li> <p> <b>Elastic Load Balancing load balancer</b>: A | AAAA</p> </li> <li> <p> <b>Amazon S3 bucket</b>: A</p> </li> <li> <p> <b>Amazon VPC interface VPC endpoint</b>: A</p> </li> <li> <p> <b>Another resource record set in this hosted zone:</b> The type of the resource record set that the alias references.</p> </li> </ul> <p>Constraint: Specifying <code>type</code> without specifying <code>name</code> returns an <code>InvalidInput</code> error.</p>
    pub start_record_type: Option<String>,
}

/// <p>A complex type that contains list information for the resource record set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListResourceRecordSetsResponse {
    /// <p>A flag that indicates whether more resource record sets remain to be listed. If your results were truncated, you can make a follow-up pagination request by using the <code>NextRecordName</code> element.</p>
    pub is_truncated: bool,
    /// <p>The maximum number of records you requested.</p>
    pub max_items: String,
    /// <p> <i>Resource record sets that have a routing policy other than simple:</i> If results were truncated for a given DNS name and type, the value of <code>SetIdentifier</code> for the next resource record set that has the current DNS name and type.</p> <p>For information about routing policies, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy.html">Choosing a Routing Policy</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListResourceRecordSetsResponse, XmlParseError> {
        deserialize_elements::<_, ListResourceRecordSetsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IsTruncated" => {
                        obj.is_truncated =
                            PageTruncatedDeserializer::deserialize("IsTruncated", stack)?;
                    }
                    "MaxItems" => {
                        obj.max_items = PageMaxItemsDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "NextRecordIdentifier" => {
                        obj.next_record_identifier =
                            Some(ResourceRecordSetIdentifierDeserializer::deserialize(
                                "NextRecordIdentifier",
                                stack,
                            )?);
                    }
                    "NextRecordName" => {
                        obj.next_record_name =
                            Some(DNSNameDeserializer::deserialize("NextRecordName", stack)?);
                    }
                    "NextRecordType" => {
                        obj.next_record_type =
                            Some(RRTypeDeserializer::deserialize("NextRecordType", stack)?);
                    }
                    "ResourceRecordSets" => {
                        obj.resource_record_sets.extend(
                            ResourceRecordSetsDeserializer::deserialize(
                                "ResourceRecordSets",
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
/// <p>A request to get a list of the reusable delegation sets that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListReusableDelegationSetsRequest {
    /// <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more reusable delegation sets. To get another group, submit another <code>ListReusableDelegationSets</code> request. </p> <p>For the value of <code>marker</code>, specify the value of <code>NextMarker</code> from the previous response, which is the ID of the first reusable delegation set that Amazon Route 53 will return if you submit another request.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more reusable delegation sets to get.</p>
    pub marker: Option<String>,
    /// <p>The number of reusable delegation sets that you want Amazon Route 53 to return in the response to this request. If you specify a value greater than 100, Route 53 returns only the first 100 reusable delegation sets.</p>
    pub max_items: Option<String>,
}

/// <p>A complex type that contains information about the reusable delegation sets that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListReusableDelegationSetsResponse, XmlParseError> {
        deserialize_elements::<_, ListReusableDelegationSetsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DelegationSets" => {
                        obj.delegation_sets
                            .extend(DelegationSetsDeserializer::deserialize(
                                "DelegationSets",
                                stack,
                            )?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            PageTruncatedDeserializer::deserialize("IsTruncated", stack)?;
                    }
                    "Marker" => {
                        obj.marker = PageMarkerDeserializer::deserialize("Marker", stack)?;
                    }
                    "MaxItems" => {
                        obj.max_items = PageMaxItemsDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "NextMarker" => {
                        obj.next_marker =
                            Some(PageMarkerDeserializer::deserialize("NextMarker", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A complex type containing information about a request for a list of the tags that are associated with an individual resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The ID of the resource for which you want to retrieve tags.</p>
    pub resource_id: String,
    /// <p><p>The type of the resource.</p> <ul> <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li> <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li> </ul></p>
    pub resource_type: String,
}

/// <p>A complex type that contains information about the health checks or hosted zones for which you want to list tags.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A <code>ResourceTagSet</code> containing tags associated with the specified resource.</p>
    pub resource_tag_set: ResourceTagSet,
}

struct ListTagsForResourceResponseDeserializer;
impl ListTagsForResourceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTagsForResourceResponse, XmlParseError> {
        deserialize_elements::<_, ListTagsForResourceResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ResourceTagSet" => {
                        obj.resource_tag_set =
                            ResourceTagSetDeserializer::deserialize("ResourceTagSet", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A complex type that contains information about the health checks or hosted zones for which you want to list tags.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListTagsForResourcesResponse {
    /// <p>A list of <code>ResourceTagSet</code>s containing tags associated with the specified resources.</p>
    pub resource_tag_sets: Vec<ResourceTagSet>,
}

struct ListTagsForResourcesResponseDeserializer;
impl ListTagsForResourcesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTagsForResourcesResponse, XmlParseError> {
        deserialize_elements::<_, ListTagsForResourcesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ResourceTagSets" => {
                        obj.resource_tag_sets
                            .extend(ResourceTagSetListDeserializer::deserialize(
                                "ResourceTagSets",
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
/// <p>A complex type that contains the information about the request to list the traffic policies that are associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTrafficPoliciesRequest {
    /// <p>(Optional) The maximum number of traffic policies that you want Amazon Route 53 to return in response to this request. If you have more than <code>MaxItems</code> traffic policies, the value of <code>IsTruncated</code> in the response is <code>true</code>, and the value of <code>TrafficPolicyIdMarker</code> is the ID of the first traffic policy that Route 53 will return if you submit another request.</p>
    pub max_items: Option<String>,
    /// <p>(Conditional) For your first request to <code>ListTrafficPolicies</code>, don't include the <code>TrafficPolicyIdMarker</code> parameter.</p> <p>If you have more traffic policies than the value of <code>MaxItems</code>, <code>ListTrafficPolicies</code> returns only the first <code>MaxItems</code> traffic policies. To get the next group of policies, submit another request to <code>ListTrafficPolicies</code>. For the value of <code>TrafficPolicyIdMarker</code>, specify the value of <code>TrafficPolicyIdMarker</code> that was returned in the previous response.</p>
    pub traffic_policy_id_marker: Option<String>,
}

/// <p>A complex type that contains the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTrafficPoliciesResponse, XmlParseError> {
        deserialize_elements::<_, ListTrafficPoliciesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IsTruncated" => {
                        obj.is_truncated =
                            PageTruncatedDeserializer::deserialize("IsTruncated", stack)?;
                    }
                    "MaxItems" => {
                        obj.max_items = PageMaxItemsDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "TrafficPolicyIdMarker" => {
                        obj.traffic_policy_id_marker = TrafficPolicyIdDeserializer::deserialize(
                            "TrafficPolicyIdMarker",
                            stack,
                        )?;
                    }
                    "TrafficPolicySummaries" => {
                        obj.traffic_policy_summaries.extend(
                            TrafficPolicySummariesDeserializer::deserialize(
                                "TrafficPolicySummaries",
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
/// <p>A request for the traffic policy instances that you created in a specified hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTrafficPolicyInstancesByHostedZoneResponse, XmlParseError> {
        deserialize_elements::<_, ListTrafficPolicyInstancesByHostedZoneResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IsTruncated" => {
                        obj.is_truncated =
                            PageTruncatedDeserializer::deserialize("IsTruncated", stack)?;
                    }
                    "MaxItems" => {
                        obj.max_items = PageMaxItemsDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "TrafficPolicyInstanceNameMarker" => {
                        obj.traffic_policy_instance_name_marker =
                            Some(DNSNameDeserializer::deserialize(
                                "TrafficPolicyInstanceNameMarker",
                                stack,
                            )?);
                    }
                    "TrafficPolicyInstanceTypeMarker" => {
                        obj.traffic_policy_instance_type_marker =
                            Some(RRTypeDeserializer::deserialize(
                                "TrafficPolicyInstanceTypeMarker",
                                stack,
                            )?);
                    }
                    "TrafficPolicyInstances" => {
                        obj.traffic_policy_instances.extend(
                            TrafficPolicyInstancesDeserializer::deserialize(
                                "TrafficPolicyInstances",
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
/// <p>A complex type that contains the information about the request to list your traffic policy instances.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTrafficPolicyInstancesByPolicyResponse, XmlParseError> {
        deserialize_elements::<_, ListTrafficPolicyInstancesByPolicyResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HostedZoneIdMarker" => {
                        obj.hosted_zone_id_marker = Some(ResourceIdDeserializer::deserialize(
                            "HostedZoneIdMarker",
                            stack,
                        )?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            PageTruncatedDeserializer::deserialize("IsTruncated", stack)?;
                    }
                    "MaxItems" => {
                        obj.max_items = PageMaxItemsDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "TrafficPolicyInstanceNameMarker" => {
                        obj.traffic_policy_instance_name_marker =
                            Some(DNSNameDeserializer::deserialize(
                                "TrafficPolicyInstanceNameMarker",
                                stack,
                            )?);
                    }
                    "TrafficPolicyInstanceTypeMarker" => {
                        obj.traffic_policy_instance_type_marker =
                            Some(RRTypeDeserializer::deserialize(
                                "TrafficPolicyInstanceTypeMarker",
                                stack,
                            )?);
                    }
                    "TrafficPolicyInstances" => {
                        obj.traffic_policy_instances.extend(
                            TrafficPolicyInstancesDeserializer::deserialize(
                                "TrafficPolicyInstances",
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
/// <p>A request to get information about the traffic policy instances that you created by using the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListTrafficPolicyInstancesResponse {
    /// <p>If <code>IsTruncated</code> is <code>true</code>, <code>HostedZoneIdMarker</code> is the ID of the hosted zone of the first traffic policy instance that Route 53 will return if you submit another <code>ListTrafficPolicyInstances</code> request. </p>
    pub hosted_zone_id_marker: Option<String>,
    /// <p>A flag that indicates whether there are more traffic policy instances to be listed. If the response was truncated, you can get more traffic policy instances by calling <code>ListTrafficPolicyInstances</code> again and specifying the values of the <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> in the corresponding request parameters.</p>
    pub is_truncated: bool,
    /// <p>The value that you specified for the <code>MaxItems</code> parameter in the call to <code>ListTrafficPolicyInstances</code> that produced the current response.</p>
    pub max_items: String,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyInstanceNameMarker</code> is the name of the first traffic policy instance that Route 53 will return if you submit another <code>ListTrafficPolicyInstances</code> request. </p>
    pub traffic_policy_instance_name_marker: Option<String>,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyInstanceTypeMarker</code> is the DNS type of the resource record sets that are associated with the first traffic policy instance that Amazon Route 53 will return if you submit another <code>ListTrafficPolicyInstances</code> request. </p>
    pub traffic_policy_instance_type_marker: Option<String>,
    /// <p>A list that contains one <code>TrafficPolicyInstance</code> element for each traffic policy instance that matches the elements in the request.</p>
    pub traffic_policy_instances: Vec<TrafficPolicyInstance>,
}

struct ListTrafficPolicyInstancesResponseDeserializer;
impl ListTrafficPolicyInstancesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTrafficPolicyInstancesResponse, XmlParseError> {
        deserialize_elements::<_, ListTrafficPolicyInstancesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HostedZoneIdMarker" => {
                        obj.hosted_zone_id_marker = Some(ResourceIdDeserializer::deserialize(
                            "HostedZoneIdMarker",
                            stack,
                        )?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            PageTruncatedDeserializer::deserialize("IsTruncated", stack)?;
                    }
                    "MaxItems" => {
                        obj.max_items = PageMaxItemsDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "TrafficPolicyInstanceNameMarker" => {
                        obj.traffic_policy_instance_name_marker =
                            Some(DNSNameDeserializer::deserialize(
                                "TrafficPolicyInstanceNameMarker",
                                stack,
                            )?);
                    }
                    "TrafficPolicyInstanceTypeMarker" => {
                        obj.traffic_policy_instance_type_marker =
                            Some(RRTypeDeserializer::deserialize(
                                "TrafficPolicyInstanceTypeMarker",
                                stack,
                            )?);
                    }
                    "TrafficPolicyInstances" => {
                        obj.traffic_policy_instances.extend(
                            TrafficPolicyInstancesDeserializer::deserialize(
                                "TrafficPolicyInstances",
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
/// <p>A complex type that contains the information about the request to list your traffic policies.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTrafficPolicyVersionsRequest {
    /// <p>Specify the value of <code>Id</code> of the traffic policy for which you want to list all versions.</p>
    pub id: String,
    /// <p>The maximum number of traffic policy versions that you want Amazon Route 53 to include in the response body for this request. If the specified traffic policy has more than <code>MaxItems</code> versions, the value of <code>IsTruncated</code> in the response is <code>true</code>, and the value of the <code>TrafficPolicyVersionMarker</code> element is the ID of the first version that Route 53 will return if you submit another request.</p>
    pub max_items: Option<String>,
    /// <p>For your first request to <code>ListTrafficPolicyVersions</code>, don't include the <code>TrafficPolicyVersionMarker</code> parameter.</p> <p>If you have more traffic policy versions than the value of <code>MaxItems</code>, <code>ListTrafficPolicyVersions</code> returns only the first group of <code>MaxItems</code> versions. To get more traffic policy versions, submit another <code>ListTrafficPolicyVersions</code> request. For the value of <code>TrafficPolicyVersionMarker</code>, specify the value of <code>TrafficPolicyVersionMarker</code> in the previous response.</p>
    pub traffic_policy_version_marker: Option<String>,
}

/// <p>A complex type that contains the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTrafficPolicyVersionsResponse, XmlParseError> {
        deserialize_elements::<_, ListTrafficPolicyVersionsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IsTruncated" => {
                        obj.is_truncated =
                            PageTruncatedDeserializer::deserialize("IsTruncated", stack)?;
                    }
                    "MaxItems" => {
                        obj.max_items = PageMaxItemsDeserializer::deserialize("MaxItems", stack)?;
                    }
                    "TrafficPolicies" => {
                        obj.traffic_policies
                            .extend(TrafficPoliciesDeserializer::deserialize(
                                "TrafficPolicies",
                                stack,
                            )?);
                    }
                    "TrafficPolicyVersionMarker" => {
                        obj.traffic_policy_version_marker =
                            TrafficPolicyVersionMarkerDeserializer::deserialize(
                                "TrafficPolicyVersionMarker",
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
/// <p>A complex type that contains information about that can be associated with your hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVPCAssociationAuthorizationsRequest {
    /// <p>The ID of the hosted zone for which you want a list of VPCs that can be associated with the hosted zone.</p>
    pub hosted_zone_id: String,
    /// <p> <i>Optional</i>: An integer that specifies the maximum number of VPCs that you want Amazon Route 53 to return. If you don't specify a value for <code>MaxResults</code>, Route 53 returns up to 50 VPCs per page.</p>
    pub max_results: Option<String>,
    /// <p> <i>Optional</i>: If a response includes a <code>NextToken</code> element, there are more VPCs that can be associated with the specified hosted zone. To get the next page of results, submit another request, and include the value of <code>NextToken</code> from the response in the <code>nexttoken</code> parameter in another <code>ListVPCAssociationAuthorizations</code> request.</p>
    pub next_token: Option<String>,
}

/// <p>A complex type that contains the response information for the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListVPCAssociationAuthorizationsResponse, XmlParseError> {
        deserialize_elements::<_, ListVPCAssociationAuthorizationsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HostedZoneId" => {
                        obj.hosted_zone_id =
                            ResourceIdDeserializer::deserialize("HostedZoneId", stack)?;
                    }
                    "NextToken" => {
                        obj.next_token = Some(PaginationTokenDeserializer::deserialize(
                            "NextToken",
                            stack,
                        )?);
                    }
                    "VPCs" => {
                        obj.vp_cs
                            .extend(VPCsDeserializer::deserialize("VPCs", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MetricNameDeserializer;
impl MetricNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NameserverDeserializer;
impl NameserverDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NamespaceDeserializer;
impl NamespaceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NonceDeserializer;
impl NonceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PaginationTokenDeserializer;
impl PaginationTokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueryLoggingConfig, XmlParseError> {
        deserialize_elements::<_, QueryLoggingConfig, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CloudWatchLogsLogGroupArn" => {
                    obj.cloud_watch_logs_log_group_arn =
                        CloudWatchLogsLogGroupArnDeserializer::deserialize(
                            "CloudWatchLogsLogGroupArn",
                            stack,
                        )?;
                }
                "HostedZoneId" => {
                    obj.hosted_zone_id =
                        ResourceIdDeserializer::deserialize("HostedZoneId", stack)?;
                }
                "Id" => {
                    obj.id = QueryLoggingConfigIdDeserializer::deserialize("Id", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct QueryLoggingConfigIdDeserializer;
impl QueryLoggingConfigIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<QueryLoggingConfig>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "QueryLoggingConfig" {
                obj.push(QueryLoggingConfigDeserializer::deserialize(
                    "QueryLoggingConfig",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct RDataDeserializer;
impl RDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "RecordDataEntry" {
                obj.push(RecordDataEntryDeserializer::deserialize(
                    "RecordDataEntry",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct RecordDataEntryDeserializer;
impl RecordDataEntryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct RequestIntervalDeserializer;
impl RequestIntervalDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResourceRecord {
    /// <p><p>The current or new DNS record value, not to exceed 4,000 characters. In the case of a <code>DELETE</code> action, if the current value does not match the actual value, an error is returned. For descriptions about how to format <code>Value</code> for different record types, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/ResourceRecordTypes.html">Supported DNS Resource Record Types</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>You can specify more than one value for all record types except <code>CNAME</code> and <code>SOA</code>. </p> <note> <p>If you&#39;re creating an alias resource record set, omit <code>Value</code>.</p> </note></p>
    pub value: String,
}

struct ResourceRecordDeserializer;
impl ResourceRecordDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceRecord, XmlParseError> {
        deserialize_elements::<_, ResourceRecord, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Value" => {
                    obj.value = RDataDeserializer::deserialize("Value", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResourceRecordSet {
    /// <p><p> <i>Alias resource record sets only:</i> Information about the AWS resource, such as a CloudFront distribution or an Amazon S3 bucket, that you want to route traffic to. </p> <p>If you&#39;re creating resource records sets for a private hosted zone, note the following:</p> <ul> <li> <p>You can&#39;t create an alias resource record set in a private hosted zone to route traffic to a CloudFront distribution.</p> </li> <li> <p>Creating geolocation alias resource record sets or latency alias resource record sets in a private hosted zone is unsupported.</p> </li> <li> <p>For information about creating failover resource record sets in a private hosted zone, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-private-hosted-zones.html">Configuring Failover in a Private Hosted Zone</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </li> </ul></p>
    pub alias_target: Option<AliasTarget>,
    /// <p><p> <i>Failover resource record sets only:</i> To configure failover, you add the <code>Failover</code> element to two resource record sets. For one resource record set, you specify <code>PRIMARY</code> as the value for <code>Failover</code>; for the other resource record set, you specify <code>SECONDARY</code>. In addition, you include the <code>HealthCheckId</code> element and specify the health check that you want Amazon Route 53 to perform for each resource record set.</p> <p>Except where noted, the following failover behaviors assume that you have included the <code>HealthCheckId</code> element in both resource record sets:</p> <ul> <li> <p>When the primary resource record set is healthy, Route 53 responds to DNS queries with the applicable value from the primary resource record set regardless of the health of the secondary resource record set.</p> </li> <li> <p>When the primary resource record set is unhealthy and the secondary resource record set is healthy, Route 53 responds to DNS queries with the applicable value from the secondary resource record set.</p> </li> <li> <p>When the secondary resource record set is unhealthy, Route 53 responds to DNS queries with the applicable value from the primary resource record set regardless of the health of the primary resource record set.</p> </li> <li> <p>If you omit the <code>HealthCheckId</code> element for the secondary resource record set, and if the primary resource record set is unhealthy, Route 53 always responds to DNS queries with the applicable value from the secondary resource record set. This is true regardless of the health of the associated endpoint.</p> </li> </ul> <p>You can&#39;t create non-failover resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements as failover resource record sets.</p> <p>For failover alias resource record sets, you must also include the <code>EvaluateTargetHealth</code> element and set the value to true.</p> <p>For more information about configuring failover for Route 53, see the following topics in the <i>Amazon Route 53 Developer Guide</i>: </p> <ul> <li> <p> <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover.html">Route 53 Health Checks and DNS Failover</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-private-hosted-zones.html">Configuring Failover in a Private Hosted Zone</a> </p> </li> </ul></p>
    pub failover: Option<String>,
    /// <p> <i>Geolocation resource record sets only:</i> A complex type that lets you control how Amazon Route 53 responds to DNS queries based on the geographic origin of the query. For example, if you want all queries from Africa to be routed to a web server with an IP address of <code>192.0.2.111</code>, create a resource record set with a <code>Type</code> of <code>A</code> and a <code>ContinentCode</code> of <code>AF</code>.</p> <note> <p>Creating geolocation and geolocation alias resource record sets in private hosted zones is not supported.</p> </note> <p>If you create separate resource record sets for overlapping geographic regions (for example, one resource record set for a continent and one for a country on the same continent), priority goes to the smallest geographic region. This allows you to route most queries for a continent to one resource and to route queries for a country on that continent to a different resource.</p> <p>You can't create two geolocation resource record sets that specify the same geographic location.</p> <p>The value <code>*</code> in the <code>CountryCode</code> element matches all geographic locations that aren't specified in other geolocation resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements.</p> <important> <p>Geolocation works by mapping IP addresses to locations. However, some IP addresses aren't mapped to geographic locations, so even if you create geolocation resource record sets that cover all seven continents, Route 53 will receive some DNS queries from locations that it can't identify. We recommend that you create a resource record set for which the value of <code>CountryCode</code> is <code>*</code>, which handles both queries that come from locations for which you haven't created geolocation resource record sets and queries from IP addresses that aren't mapped to a location. If you don't create a <code>*</code> resource record set, Route 53 returns a "no answer" response for queries from those locations.</p> </important> <p>You can't create non-geolocation resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements as geolocation resource record sets.</p>
    pub geo_location: Option<GeoLocation>,
    /// <p><p>If you want Amazon Route 53 to return this resource record set in response to a DNS query only when the status of a health check is healthy, include the <code>HealthCheckId</code> element and specify the ID of the applicable health check.</p> <p>Route 53 determines whether a resource record set is healthy based on one of the following:</p> <ul> <li> <p>By periodically sending a request to the endpoint that is specified in the health check</p> </li> <li> <p>By aggregating the status of a specified group of health checks (calculated health checks)</p> </li> <li> <p>By determining the current state of a CloudWatch alarm (CloudWatch metric health checks)</p> </li> </ul> <important> <p>Route 53 doesn&#39;t check the health of the endpoint that is specified in the resource record set, for example, the endpoint specified by the IP address in the <code>Value</code> element. When you add a <code>HealthCheckId</code> element to a resource record set, Route 53 checks the health of the endpoint that you specified in the health check. </p> </important> <p>For more information, see the following topics in the <i>Amazon Route 53 Developer Guide</i>:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html">How Amazon Route 53 Determines Whether an Endpoint Is Healthy</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover.html">Route 53 Health Checks and DNS Failover</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-private-hosted-zones.html">Configuring Failover in a Private Hosted Zone</a> </p> </li> </ul> <p> <b>When to Specify HealthCheckId</b> </p> <p>Specifying a value for <code>HealthCheckId</code> is useful only when Route 53 is choosing between two or more resource record sets to respond to a DNS query, and you want Route 53 to base the choice in part on the status of a health check. Configuring health checks makes sense only in the following configurations:</p> <ul> <li> <p> <b>Non-alias resource record sets</b>: You&#39;re checking the health of a group of non-alias resource record sets that have the same routing policy, name, and type (such as multiple weighted records named www.example.com with a type of A) and you specify health check IDs for all the resource record sets. </p> <p>If the health check status for a resource record set is healthy, Route 53 includes the record among the records that it responds to DNS queries with.</p> <p>If the health check status for a resource record set is unhealthy, Route 53 stops responding to DNS queries using the value for that resource record set.</p> <p>If the health check status for all resource record sets in the group is unhealthy, Route 53 considers all resource record sets in the group healthy and responds to DNS queries accordingly. </p> </li> <li> <p> <b>Alias resource record sets</b>: You specify the following settings:</p> <ul> <li> <p>You set <code>EvaluateTargetHealth</code> to true for an alias resource record set in a group of resource record sets that have the same routing policy, name, and type (such as multiple weighted records named www.example.com with a type of A). </p> </li> <li> <p>You configure the alias resource record set to route traffic to a non-alias resource record set in the same hosted zone.</p> </li> <li> <p>You specify a health check ID for the non-alias resource record set. </p> </li> </ul> <p>If the health check status is healthy, Route 53 considers the alias resource record set to be healthy and includes the alias record among the records that it responds to DNS queries with.</p> <p>If the health check status is unhealthy, Route 53 stops responding to DNS queries using the alias resource record set.</p> <note> <p>The alias resource record set can also route traffic to a <i>group</i> of non-alias resource record sets that have the same routing policy, name, and type. In that configuration, associate health checks with all of the resource record sets in the group of non-alias resource record sets.</p> </note> </li> </ul> <p> <b>Geolocation Routing</b> </p> <p>For geolocation resource record sets, if an endpoint is unhealthy, Route 53 looks for a resource record set for the larger, associated geographic region. For example, suppose you have resource record sets for a state in the United States, for the entire United States, for North America, and a resource record set that has <code><em></code> for <code>CountryCode</code> is <code></em></code>, which applies to all locations. If the endpoint for the state resource record set is unhealthy, Route 53 checks for healthy resource record sets in the following order until it finds a resource record set for which the endpoint is healthy:</p> <ul> <li> <p>The United States</p> </li> <li> <p>North America</p> </li> <li> <p>The default resource record set</p> </li> </ul> <p> <b>Specifying the Health Check Endpoint by Domain Name</b> </p> <p>If your health checks specify the endpoint only by domain name, we recommend that you create a separate health check for each endpoint. For example, create a health check for each <code>HTTP</code> server that is serving content for <code>www.example.com</code>. For the value of <code>FullyQualifiedDomainName</code>, specify the domain name of the server (such as <code>us-east-2-www.example.com</code>), not the name of the resource record sets (<code>www.example.com</code>).</p> <important> <p>Health check results will be unpredictable if you do the following:</p> <ul> <li> <p>Create a health check that has the same value for <code>FullyQualifiedDomainName</code> as the name of a resource record set.</p> </li> <li> <p>Associate that health check with the resource record set.</p> </li> </ul> </important></p>
    pub health_check_id: Option<String>,
    /// <p> <i>Multivalue answer resource record sets only</i>: To route traffic approximately randomly to multiple resources, such as web servers, create one multivalue answer record for each resource and specify <code>true</code> for <code>MultiValueAnswer</code>. Note the following:</p> <ul> <li> <p>If you associate a health check with a multivalue answer resource record set, Amazon Route 53 responds to DNS queries with the corresponding IP address only when the health check is healthy.</p> </li> <li> <p>If you don't associate a health check with a multivalue answer record, Route 53 always considers the record to be healthy.</p> </li> <li> <p>Route 53 responds to DNS queries with up to eight healthy records; if you have eight or fewer healthy records, Route 53 responds to all DNS queries with all the healthy records.</p> </li> <li> <p>If you have more than eight healthy records, Route 53 responds to different DNS resolvers with different combinations of healthy records.</p> </li> <li> <p>When all records are unhealthy, Route 53 responds to DNS queries with up to eight unhealthy records.</p> </li> <li> <p>If a resource becomes unavailable after a resolver caches a response, client software typically tries another of the IP addresses in the response.</p> </li> </ul> <p>You can't create multivalue answer alias records.</p>
    pub multi_value_answer: Option<bool>,
    /// <p>For <code>ChangeResourceRecordSets</code> requests, the name of the record that you want to create, update, or delete. For <code>ListResourceRecordSets</code> responses, the name of a record in the specified hosted zone.</p> <p> <b>ChangeResourceRecordSets Only</b> </p> <p>Enter a fully qualified domain name, for example, <code>www.example.com</code>. You can optionally include a trailing dot. If you omit the trailing dot, Amazon Route 53 assumes that the domain name that you specify is fully qualified. This means that Route 53 treats <code>www.example.com</code> (without a trailing dot) and <code>www.example.com.</code> (with a trailing dot) as identical.</p> <p>For information about how to specify characters other than <code>a-z</code>, <code>0-9</code>, and <code>-</code> (hyphen) and how to specify internationalized domain names, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DomainNameFormat.html">DNS Domain Name Format</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>You can use the asterisk (*) wildcard to replace the leftmost label in a domain name, for example, <code>*.example.com</code>. Note the following:</p> <ul> <li> <p>The * must replace the entire label. For example, you can't specify <code>*prod.example.com</code> or <code>prod*.example.com</code>.</p> </li> <li> <p>The * can't replace any of the middle labels, for example, marketing.*.example.com.</p> </li> <li> <p>If you include * in any position other than the leftmost label in a domain name, DNS treats it as an * character (ASCII 42), not as a wildcard.</p> <important> <p>You can't use the * wildcard for resource records sets that have a type of NS.</p> </important> </li> </ul> <p>You can use the * wildcard as the leftmost label in a domain name, for example, <code>*.example.com</code>. You can't use an * for one of the middle labels, for example, <code>marketing.*.example.com</code>. In addition, the * must replace the entire label; for example, you can't specify <code>prod*.example.com</code>.</p>
    pub name: String,
    /// <p><p> <i>Latency-based resource record sets only:</i> The Amazon EC2 Region where you created the resource that this resource record set refers to. The resource typically is an AWS resource, such as an EC2 instance or an ELB load balancer, and is referred to by an IP address or a DNS domain name, depending on the record type.</p> <note> <p>Creating latency and latency alias resource record sets in private hosted zones is not supported.</p> </note> <p>When Amazon Route 53 receives a DNS query for a domain name and type for which you have created latency resource record sets, Route 53 selects the latency resource record set that has the lowest latency between the end user and the associated Amazon EC2 Region. Route 53 then returns the value that is associated with the selected resource record set.</p> <p>Note the following:</p> <ul> <li> <p>You can only specify one <code>ResourceRecord</code> per latency resource record set.</p> </li> <li> <p>You can only create one latency resource record set for each Amazon EC2 Region.</p> </li> <li> <p>You aren&#39;t required to create latency resource record sets for all Amazon EC2 Regions. Route 53 will choose the region with the best latency from among the regions that you create latency resource record sets for.</p> </li> <li> <p>You can&#39;t create non-latency resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements as latency resource record sets.</p> </li> </ul></p>
    pub region: Option<String>,
    /// <p><p>Information about the resource records to act upon.</p> <note> <p>If you&#39;re creating an alias resource record set, omit <code>ResourceRecords</code>.</p> </note></p>
    pub resource_records: Option<Vec<ResourceRecord>>,
    /// <p> <i>Resource record sets that have a routing policy other than simple:</i> An identifier that differentiates among multiple resource record sets that have the same combination of name and type, such as multiple weighted resource record sets named acme.example.com that have a type of A. In a group of resource record sets that have the same name and type, the value of <code>SetIdentifier</code> must be unique for each resource record set. </p> <p>For information about routing policies, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy.html">Choosing a Routing Policy</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    pub set_identifier: Option<String>,
    /// <p><p>The resource record cache time to live (TTL), in seconds. Note the following:</p> <ul> <li> <p>If you&#39;re creating or updating an alias resource record set, omit <code>TTL</code>. Amazon Route 53 uses the value of <code>TTL</code> for the alias target. </p> </li> <li> <p>If you&#39;re associating this resource record set with a health check (if you&#39;re adding a <code>HealthCheckId</code> element), we recommend that you specify a <code>TTL</code> of 60 seconds or less so clients respond quickly to changes in health status.</p> </li> <li> <p>All of the resource record sets in a group of weighted resource record sets must have the same value for <code>TTL</code>.</p> </li> <li> <p>If a group of weighted resource record sets includes one or more weighted alias resource record sets for which the alias target is an ELB load balancer, we recommend that you specify a <code>TTL</code> of 60 seconds for all of the non-alias weighted resource record sets that have the same name and type. Values other than 60 seconds (the TTL for load balancers) will change the effect of the values that you specify for <code>Weight</code>.</p> </li> </ul></p>
    pub ttl: Option<i64>,
    /// <p><p>When you create a traffic policy instance, Amazon Route 53 automatically creates a resource record set. <code>TrafficPolicyInstanceId</code> is the ID of the traffic policy instance that Route 53 created this resource record set for.</p> <important> <p>To delete the resource record set that is associated with a traffic policy instance, use <code>DeleteTrafficPolicyInstance</code>. Route 53 will delete the resource record set automatically. If you delete the resource record set by using <code>ChangeResourceRecordSets</code>, Route 53 doesn&#39;t automatically delete the traffic policy instance, and you&#39;ll continue to be charged for it even though it&#39;s no longer in use. </p> </important></p>
    pub traffic_policy_instance_id: Option<String>,
    /// <p><p>The DNS record type. For information about different record types and how data is encoded for them, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/ResourceRecordTypes.html">Supported DNS Resource Record Types</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>Valid values for basic resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>NS</code> | <code>PTR</code> | <code>SOA</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code> </p> <p>Values for weighted, latency, geolocation, and failover resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>PTR</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code>. When creating a group of weighted, latency, geolocation, or failover resource record sets, specify the same value for all of the resource record sets in the group.</p> <p>Valid values for multivalue answer resource record sets: <code>A</code> | <code>AAAA</code> | <code>MX</code> | <code>NAPTR</code> | <code>PTR</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code> </p> <note> <p>SPF records were formerly used to verify the identity of the sender of email messages. However, we no longer recommend that you create resource record sets for which the value of <code>Type</code> is <code>SPF</code>. RFC 7208, <i>Sender Policy Framework (SPF) for Authorizing Use of Domains in Email, Version 1</i>, has been updated to say, &quot;...[I]ts existence and mechanism defined in [RFC4408] have led to some interoperability issues. Accordingly, its use is no longer appropriate for SPF version 1; implementations are not to use it.&quot; In RFC 7208, see section 14.1, <a href="http://tools.ietf.org/html/rfc7208#section-14.1">The SPF DNS Record Type</a>.</p> </note> <p>Values for alias resource record sets:</p> <ul> <li> <p> <b>Amazon API Gateway custom regional APIs and edge-optimized APIs:</b> <code>A</code> </p> </li> <li> <p> <b>CloudFront distributions:</b> <code>A</code> </p> <p>If IPv6 is enabled for the distribution, create two resource record sets to route traffic to your distribution, one with a value of <code>A</code> and one with a value of <code>AAAA</code>. </p> </li> <li> <p> <b>AWS Elastic Beanstalk environment that has a regionalized subdomain</b>: <code>A</code> </p> </li> <li> <p> <b>ELB load balancers:</b> <code>A</code> | <code>AAAA</code> </p> </li> <li> <p> <b>Amazon S3 buckets:</b> <code>A</code> </p> </li> <li> <p> <b>Amazon Virtual Private Cloud interface VPC endpoints</b> <code>A</code> </p> </li> <li> <p> <b>Another resource record set in this hosted zone:</b> Specify the type of the resource record set that you&#39;re creating the alias for. All values are supported except <code>NS</code> and <code>SOA</code>.</p> <note> <p>If you&#39;re creating an alias record that has the same name as the hosted zone (known as the zone apex), you can&#39;t route traffic to a record for which the value of <code>Type</code> is <code>CNAME</code>. This is because the alias record must have the same type as the record you&#39;re routing traffic to, and creating a CNAME record for the zone apex isn&#39;t supported even for an alias record.</p> </note> </li> </ul></p>
    pub type_: String,
    /// <p><p> <i>Weighted resource record sets only:</i> Among resource record sets that have the same combination of DNS name and type, a value that determines the proportion of DNS queries that Amazon Route 53 responds to using the current resource record set. Route 53 calculates the sum of the weights for the resource record sets that have the same combination of DNS name and type. Route 53 then responds to queries based on the ratio of a resource&#39;s weight to the total. Note the following:</p> <ul> <li> <p>You must specify a value for the <code>Weight</code> element for every weighted resource record set.</p> </li> <li> <p>You can only specify one <code>ResourceRecord</code> per weighted resource record set.</p> </li> <li> <p>You can&#39;t create latency, failover, or geolocation resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements as weighted resource record sets.</p> </li> <li> <p>You can create a maximum of 100 weighted resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements.</p> </li> <li> <p>For weighted (but not weighted alias) resource record sets, if you set <code>Weight</code> to <code>0</code> for a resource record set, Route 53 never responds to queries with the applicable value for that resource record set. However, if you set <code>Weight</code> to <code>0</code> for all resource record sets that have the same combination of DNS name and type, traffic is routed to all resources with equal probability.</p> <p>The effect of setting <code>Weight</code> to <code>0</code> is different when you associate health checks with weighted resource record sets. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-configuring-options.html">Options for Configuring Route 53 Active-Active and Active-Passive Failover</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </li> </ul></p>
    pub weight: Option<i64>,
}

struct ResourceRecordSetDeserializer;
impl ResourceRecordSetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceRecordSet, XmlParseError> {
        deserialize_elements::<_, ResourceRecordSet, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AliasTarget" => {
                    obj.alias_target =
                        Some(AliasTargetDeserializer::deserialize("AliasTarget", stack)?);
                }
                "Failover" => {
                    obj.failover = Some(ResourceRecordSetFailoverDeserializer::deserialize(
                        "Failover", stack,
                    )?);
                }
                "GeoLocation" => {
                    obj.geo_location =
                        Some(GeoLocationDeserializer::deserialize("GeoLocation", stack)?);
                }
                "HealthCheckId" => {
                    obj.health_check_id = Some(HealthCheckIdDeserializer::deserialize(
                        "HealthCheckId",
                        stack,
                    )?);
                }
                "MultiValueAnswer" => {
                    obj.multi_value_answer =
                        Some(ResourceRecordSetMultiValueAnswerDeserializer::deserialize(
                            "MultiValueAnswer",
                            stack,
                        )?);
                }
                "Name" => {
                    obj.name = DNSNameDeserializer::deserialize("Name", stack)?;
                }
                "Region" => {
                    obj.region = Some(ResourceRecordSetRegionDeserializer::deserialize(
                        "Region", stack,
                    )?);
                }
                "ResourceRecords" => {
                    obj.resource_records.get_or_insert(vec![]).extend(
                        ResourceRecordsDeserializer::deserialize("ResourceRecords", stack)?,
                    );
                }
                "SetIdentifier" => {
                    obj.set_identifier =
                        Some(ResourceRecordSetIdentifierDeserializer::deserialize(
                            "SetIdentifier",
                            stack,
                        )?);
                }
                "TTL" => {
                    obj.ttl = Some(TTLDeserializer::deserialize("TTL", stack)?);
                }
                "TrafficPolicyInstanceId" => {
                    obj.traffic_policy_instance_id =
                        Some(TrafficPolicyInstanceIdDeserializer::deserialize(
                            "TrafficPolicyInstanceId",
                            stack,
                        )?);
                }
                "Type" => {
                    obj.type_ = RRTypeDeserializer::deserialize("Type", stack)?;
                }
                "Weight" => {
                    obj.weight = Some(ResourceRecordSetWeightDeserializer::deserialize(
                        "Weight", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ResourceRecordSet>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ResourceRecordSet" {
                obj.push(ResourceRecordSetDeserializer::deserialize(
                    "ResourceRecordSet",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ResourceRecordsDeserializer;
impl ResourceRecordsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ResourceRecord>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ResourceRecord" {
                obj.push(ResourceRecordDeserializer::deserialize(
                    "ResourceRecord",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceTagSet, XmlParseError> {
        deserialize_elements::<_, ResourceTagSet, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ResourceId" => {
                    obj.resource_id =
                        Some(TagResourceIdDeserializer::deserialize("ResourceId", stack)?);
                }
                "ResourceType" => {
                    obj.resource_type = Some(TagResourceTypeDeserializer::deserialize(
                        "ResourceType",
                        stack,
                    )?);
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
struct ResourceTagSetListDeserializer;
impl ResourceTagSetListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ResourceTagSet>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ResourceTagSet" {
                obj.push(ResourceTagSetDeserializer::deserialize(
                    "ResourceTagSet",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A complex type that contains the type of limit that you specified in the request and the current value for that limit.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ReusableDelegationSetLimit {
    /// <p>The limit that you requested: <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code>, the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p>
    pub type_: String,
    /// <p>The current value for the <code>MAX_ZONES_BY_REUSABLE_DELEGATION_SET</code> limit.</p>
    pub value: i64,
}

struct ReusableDelegationSetLimitDeserializer;
impl ReusableDelegationSetLimitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReusableDelegationSetLimit, XmlParseError> {
        deserialize_elements::<_, ReusableDelegationSetLimit, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Type" => {
                        obj.type_ =
                            ReusableDelegationSetLimitTypeDeserializer::deserialize("Type", stack)?;
                    }
                    "Value" => {
                        obj.value = LimitValueDeserializer::deserialize("Value", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct ReusableDelegationSetLimitTypeDeserializer;
impl ReusableDelegationSetLimitTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StatisticDeserializer;
impl StatisticDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StatusDeserializer;
impl StatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A complex type that contains the status that one Amazon Route 53 health checker reports and the time of the health check.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct StatusReport {
    /// <p>The date and time that the health checker performed the health check in <a href="https://en.wikipedia.org/wiki/ISO_8601">ISO 8601 format</a> and Coordinated Universal Time (UTC). For example, the value <code>2017-03-27T17:48:16.751Z</code> represents March 27, 2017 at 17:48:16.751 UTC.</p>
    pub checked_time: Option<String>,
    /// <p>A description of the status of the health check endpoint as reported by one of the Amazon Route 53 health checkers.</p>
    pub status: Option<String>,
}

struct StatusReportDeserializer;
impl StatusReportDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StatusReport, XmlParseError> {
        deserialize_elements::<_, StatusReport, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CheckedTime" => {
                    obj.checked_time =
                        Some(TimeStampDeserializer::deserialize("CheckedTime", stack)?);
                }
                "Status" => {
                    obj.status = Some(StatusDeserializer::deserialize("Status", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tag {
    /// <p><p>The value of <code>Key</code> depends on the operation that you want to perform:</p> <ul> <li> <p> <b>Add a tag to a health check or hosted zone</b>: <code>Key</code> is the name that you want to give the new tag.</p> </li> <li> <p> <b>Edit a tag</b>: <code>Key</code> is the name of the tag that you want to change the <code>Value</code> for.</p> </li> <li> <p> <b> Delete a key</b>: <code>Key</code> is the name of the tag you want to remove.</p> </li> <li> <p> <b>Give a name to a health check</b>: Edit the default <code>Name</code> tag. In the Amazon Route 53 console, the list of your health checks includes a <b>Name</b> column that lets you see the name that you&#39;ve given to each health check.</p> </li> </ul></p>
    pub key: Option<String>,
    /// <p><p>The value of <code>Value</code> depends on the operation that you want to perform:</p> <ul> <li> <p> <b>Add a tag to a health check or hosted zone</b>: <code>Value</code> is the value that you want to give the new tag.</p> </li> <li> <p> <b>Edit a tag</b>: <code>Value</code> is the new value that you want to assign the tag.</p> </li> </ul></p>
    pub value: Option<String>,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Tag, XmlParseError> {
        deserialize_elements::<_, Tag, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Key" => {
                    obj.key = Some(TagKeyDeserializer::deserialize("Key", stack)?);
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

struct TagResourceIdDeserializer;
impl TagResourceIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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

/// <p>Gets the value that Amazon Route 53 returns in response to a DNS request for a specified record name and type. You can optionally specify the IP address of a DNS resolver, an EDNS0 client subnet IP address, and a subnet mask. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TestDNSAnswerRequest {
    /// <p>If the resolver that you specified for resolverip supports EDNS0, specify the IPv4 or IPv6 address of a client in the applicable location, for example, <code>192.0.2.44</code> or <code>2001:db8:85a3::8a2e:370:7334</code>.</p>
    pub edns0_client_subnet_ip: Option<String>,
    /// <p><p>If you specify an IP address for <code>edns0clientsubnetip</code>, you can optionally specify the number of bits of the IP address that you want the checking tool to include in the DNS query. For example, if you specify <code>192.0.2.44</code> for <code>edns0clientsubnetip</code> and <code>24</code> for <code>edns0clientsubnetmask</code>, the checking tool will simulate a request from 192.0.2.0/24. The default value is 24 bits for IPv4 addresses and 64 bits for IPv6 addresses.</p> <p>The range of valid values depends on whether <code>edns0clientsubnetip</code> is an IPv4 or an IPv6 address:</p> <ul> <li> <p> <b>IPv4</b>: Specify a value between 0 and 32</p> </li> <li> <p> <b>IPv6</b>: Specify a value between 0 and 128</p> </li> </ul></p>
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TestDNSAnswerResponse, XmlParseError> {
        deserialize_elements::<_, TestDNSAnswerResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Nameserver" => {
                    obj.nameserver = NameserverDeserializer::deserialize("Nameserver", stack)?;
                }
                "Protocol" => {
                    obj.protocol = TransportProtocolDeserializer::deserialize("Protocol", stack)?;
                }
                "RecordData" => {
                    obj.record_data
                        .extend(RecordDataDeserializer::deserialize("RecordData", stack)?);
                }
                "RecordName" => {
                    obj.record_name = DNSNameDeserializer::deserialize("RecordName", stack)?;
                }
                "RecordType" => {
                    obj.record_type = RRTypeDeserializer::deserialize("RecordType", stack)?;
                }
                "ResponseCode" => {
                    obj.response_code = DNSRCodeDeserializer::deserialize("ResponseCode", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ThresholdDeserializer;
impl ThresholdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<f64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = f64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TimeStampDeserializer;
impl TimeStampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TrafficPoliciesDeserializer;
impl TrafficPoliciesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TrafficPolicy>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "TrafficPolicy" {
                obj.push(TrafficPolicyDeserializer::deserialize(
                    "TrafficPolicy",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A complex type that contains settings for a traffic policy.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TrafficPolicy {
    /// <p>The comment that you specify in the <code>CreateTrafficPolicy</code> request, if any.</p>
    pub comment: Option<String>,
    /// <p>The definition of a traffic policy in JSON format. You specify the JSON document to use for a new traffic policy in the <code>CreateTrafficPolicy</code> request. For more information about the JSON format, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/api-policies-traffic-policy-document-format.html">Traffic Policy Document Format</a>.</p>
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TrafficPolicy, XmlParseError> {
        deserialize_elements::<_, TrafficPolicy, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Comment" => {
                    obj.comment = Some(TrafficPolicyCommentDeserializer::deserialize(
                        "Comment", stack,
                    )?);
                }
                "Document" => {
                    obj.document =
                        TrafficPolicyDocumentDeserializer::deserialize("Document", stack)?;
                }
                "Id" => {
                    obj.id = TrafficPolicyIdDeserializer::deserialize("Id", stack)?;
                }
                "Name" => {
                    obj.name = TrafficPolicyNameDeserializer::deserialize("Name", stack)?;
                }
                "Type" => {
                    obj.type_ = RRTypeDeserializer::deserialize("Type", stack)?;
                }
                "Version" => {
                    obj.version = TrafficPolicyVersionDeserializer::deserialize("Version", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct TrafficPolicyCommentDeserializer;
impl TrafficPolicyCommentDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TrafficPolicyInstance {
    /// <p>The ID of the hosted zone that Amazon Route 53 created resource record sets in.</p>
    pub hosted_zone_id: String,
    /// <p>The ID that Amazon Route 53 assigned to the new traffic policy instance.</p>
    pub id: String,
    /// <p>If <code>State</code> is <code>Failed</code>, an explanation of the reason for the failure. If <code>State</code> is another value, <code>Message</code> is empty.</p>
    pub message: String,
    /// <p>The DNS name, such as www.example.com, for which Amazon Route 53 responds to queries by using the resource record sets that are associated with this traffic policy instance. </p>
    pub name: String,
    /// <p><p>The value of <code>State</code> is one of the following values:</p> <dl> <dt>Applied</dt> <dd> <p>Amazon Route 53 has finished creating resource record sets, and changes have propagated to all Route 53 edge locations.</p> </dd> <dt>Creating</dt> <dd> <p>Route 53 is creating the resource record sets. Use <code>GetTrafficPolicyInstance</code> to confirm that the <code>CreateTrafficPolicyInstance</code> request completed successfully.</p> </dd> <dt>Failed</dt> <dd> <p>Route 53 wasn&#39;t able to create or update the resource record sets. When the value of <code>State</code> is <code>Failed</code>, see <code>Message</code> for an explanation of what caused the request to fail.</p> </dd> </dl></p>
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TrafficPolicyInstance, XmlParseError> {
        deserialize_elements::<_, TrafficPolicyInstance, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "HostedZoneId" => {
                    obj.hosted_zone_id =
                        ResourceIdDeserializer::deserialize("HostedZoneId", stack)?;
                }
                "Id" => {
                    obj.id = TrafficPolicyInstanceIdDeserializer::deserialize("Id", stack)?;
                }
                "Message" => {
                    obj.message = MessageDeserializer::deserialize("Message", stack)?;
                }
                "Name" => {
                    obj.name = DNSNameDeserializer::deserialize("Name", stack)?;
                }
                "State" => {
                    obj.state =
                        TrafficPolicyInstanceStateDeserializer::deserialize("State", stack)?;
                }
                "TTL" => {
                    obj.ttl = TTLDeserializer::deserialize("TTL", stack)?;
                }
                "TrafficPolicyId" => {
                    obj.traffic_policy_id =
                        TrafficPolicyIdDeserializer::deserialize("TrafficPolicyId", stack)?;
                }
                "TrafficPolicyType" => {
                    obj.traffic_policy_type =
                        RRTypeDeserializer::deserialize("TrafficPolicyType", stack)?;
                }
                "TrafficPolicyVersion" => {
                    obj.traffic_policy_version = TrafficPolicyVersionDeserializer::deserialize(
                        "TrafficPolicyVersion",
                        stack,
                    )?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct TrafficPolicyInstanceCountDeserializer;
impl TrafficPolicyInstanceCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TrafficPolicyInstanceIdDeserializer;
impl TrafficPolicyInstanceIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TrafficPolicyInstancesDeserializer;
impl TrafficPolicyInstancesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TrafficPolicyInstance>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "TrafficPolicyInstance" {
                obj.push(TrafficPolicyInstanceDeserializer::deserialize(
                    "TrafficPolicyInstance",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct TrafficPolicyNameDeserializer;
impl TrafficPolicyNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TrafficPolicySummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "TrafficPolicySummary" {
                obj.push(TrafficPolicySummaryDeserializer::deserialize(
                    "TrafficPolicySummary",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A complex type that contains information about the latest version of one traffic policy that is associated with the current AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TrafficPolicySummary, XmlParseError> {
        deserialize_elements::<_, TrafficPolicySummary, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Id" => {
                    obj.id = TrafficPolicyIdDeserializer::deserialize("Id", stack)?;
                }
                "LatestVersion" => {
                    obj.latest_version =
                        TrafficPolicyVersionDeserializer::deserialize("LatestVersion", stack)?;
                }
                "Name" => {
                    obj.name = TrafficPolicyNameDeserializer::deserialize("Name", stack)?;
                }
                "TrafficPolicyCount" => {
                    obj.traffic_policy_count =
                        TrafficPolicyVersionDeserializer::deserialize("TrafficPolicyCount", stack)?;
                }
                "Type" => {
                    obj.type_ = RRTypeDeserializer::deserialize("Type", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct TrafficPolicyVersionDeserializer;
impl TrafficPolicyVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A complex type that contains information about a request to update a health check.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateHealthCheckRequest {
    /// <p>A complex type that identifies the CloudWatch alarm that you want Amazon Route 53 health checkers to use to determine whether the specified health check is healthy.</p>
    pub alarm_identifier: Option<AlarmIdentifier>,
    /// <p>A complex type that contains one <code>ChildHealthCheck</code> element for each health check that you want to associate with a <code>CALCULATED</code> health check.</p>
    pub child_health_checks: Option<Vec<String>>,
    /// <p>Stops Route 53 from performing health checks. When you disable a health check, here's what happens:</p> <ul> <li> <p> <b>Health checks that check the health of endpoints:</b> Route 53 stops submitting requests to your application, server, or other resource.</p> </li> <li> <p> <b>Calculated health checks:</b> Route 53 stops aggregating the status of the referenced health checks.</p> </li> <li> <p> <b>Health checks that monitor CloudWatch alarms:</b> Route 53 stops monitoring the corresponding CloudWatch metrics.</p> </li> </ul> <p>After you disable a health check, Route 53 considers the status of the health check to always be healthy. If you configured DNS failover, Route 53 continues to route traffic to the corresponding resources. If you want to stop routing traffic to a resource, change the value of <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_UpdateHealthCheck.html#Route53-UpdateHealthCheck-request-Inverted">Inverted</a>. </p> <p>Charges for a health check still apply when the health check is disabled. For more information, see <a href="http://aws.amazon.com/route53/pricing/">Amazon Route 53 Pricing</a>.</p>
    pub disabled: Option<bool>,
    /// <p>Specify whether you want Amazon Route 53 to send the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>client_hello</code> message during <code>TLS</code> negotiation. This allows the endpoint to respond to <code>HTTPS</code> health check requests with the applicable SSL/TLS certificate.</p> <p>Some endpoints require that HTTPS requests include the host name in the <code>client_hello</code> message. If you don't enable SNI, the status of the health check will be SSL alert <code>handshake_failure</code>. A health check can also have that status for other reasons. If SNI is enabled and you're still getting the error, check the SSL/TLS configuration on your endpoint and confirm that your certificate is valid.</p> <p>The SSL/TLS certificate on your endpoint includes a domain name in the <code>Common Name</code> field and possibly several more in the <code>Subject Alternative Names</code> field. One of the domain names in the certificate should match the value that you specify for <code>FullyQualifiedDomainName</code>. If the endpoint responds to the <code>client_hello</code> message with a certificate that does not include the domain name that you specified in <code>FullyQualifiedDomainName</code>, a health checker will retry the handshake. In the second attempt, the health checker will omit <code>FullyQualifiedDomainName</code> from the <code>client_hello</code> message.</p>
    pub enable_sni: Option<bool>,
    /// <p>The number of consecutive health checks that an endpoint must pass or fail for Amazon Route 53 to change the current status of the endpoint from unhealthy to healthy or vice versa. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html">How Amazon Route 53 Determines Whether an Endpoint Is Healthy</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If you don't specify a value for <code>FailureThreshold</code>, the default value is three health checks.</p>
    pub failure_threshold: Option<i64>,
    /// <p>Amazon Route 53 behavior depends on whether you specify a value for <code>IPAddress</code>.</p> <note> <p>If a health check already has a value for <code>IPAddress</code>, you can change the value. However, you can't update an existing health check to add or remove the value of <code>IPAddress</code>. </p> </note> <p> <b>If you specify a value for</b> <code>IPAddress</code>:</p> <p>Route 53 sends health check requests to the specified IPv4 or IPv6 address and passes the value of <code>FullyQualifiedDomainName</code> in the <code>Host</code> header for all health checks except TCP health checks. This is typically the fully qualified DNS name of the endpoint on which you want Route 53 to perform health checks.</p> <p>When Route 53 checks the health of an endpoint, here is how it constructs the <code>Host</code> header:</p> <ul> <li> <p>If you specify a value of <code>80</code> for <code>Port</code> and <code>HTTP</code> or <code>HTTP_STR_MATCH</code> for <code>Type</code>, Route 53 passes the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>Host</code> header.</p> </li> <li> <p>If you specify a value of <code>443</code> for <code>Port</code> and <code>HTTPS</code> or <code>HTTPS_STR_MATCH</code> for <code>Type</code>, Route 53 passes the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>Host</code> header.</p> </li> <li> <p>If you specify another value for <code>Port</code> and any value except <code>TCP</code> for <code>Type</code>, Route 53 passes <i> <code>FullyQualifiedDomainName</code>:<code>Port</code> </i> to the endpoint in the <code>Host</code> header.</p> </li> </ul> <p>If you don't specify a value for <code>FullyQualifiedDomainName</code>, Route 53 substitutes the value of <code>IPAddress</code> in the <code>Host</code> header in each of the above cases.</p> <p> <b>If you don't specify a value for</b> <code>IPAddress</code>:</p> <p>If you don't specify a value for <code>IPAddress</code>, Route 53 sends a DNS request to the domain that you specify in <code>FullyQualifiedDomainName</code> at the interval you specify in <code>RequestInterval</code>. Using an IPv4 address that is returned by DNS, Route 53 then checks the health of the endpoint.</p> <note> <p>If you don't specify a value for <code>IPAddress</code>, Route 53 uses only IPv4 to send health checks to the endpoint. If there's no resource record set with a type of A for the name that you specify for <code>FullyQualifiedDomainName</code>, the health check fails with a "DNS resolution failed" error.</p> </note> <p>If you want to check the health of weighted, latency, or failover resource record sets and you choose to specify the endpoint only by <code>FullyQualifiedDomainName</code>, we recommend that you create a separate health check for each endpoint. For example, create a health check for each HTTP server that is serving content for www.example.com. For the value of <code>FullyQualifiedDomainName</code>, specify the domain name of the server (such as <code>us-east-2-www.example.com</code>), not the name of the resource record sets (www.example.com).</p> <important> <p>In this configuration, if the value of <code>FullyQualifiedDomainName</code> matches the name of the resource record sets and you then associate the health check with those resource record sets, health check results will be unpredictable.</p> </important> <p>In addition, if the value of <code>Type</code> is <code>HTTP</code>, <code>HTTPS</code>, <code>HTTP_STR_MATCH</code>, or <code>HTTPS_STR_MATCH</code>, Route 53 passes the value of <code>FullyQualifiedDomainName</code> in the <code>Host</code> header, as it does when you specify a value for <code>IPAddress</code>. If the value of <code>Type</code> is <code>TCP</code>, Route 53 doesn't pass a <code>Host</code> header.</p>
    pub fully_qualified_domain_name: Option<String>,
    /// <p>The ID for the health check for which you want detailed information. When you created the health check, <code>CreateHealthCheck</code> returned the ID in the response, in the <code>HealthCheckId</code> element.</p>
    pub health_check_id: String,
    /// <p><p>A sequential counter that Amazon Route 53 sets to <code>1</code> when you create a health check and increments by 1 each time you update settings for the health check.</p> <p>We recommend that you use <code>GetHealthCheck</code> or <code>ListHealthChecks</code> to get the current value of <code>HealthCheckVersion</code> for the health check that you want to update, and that you include that value in your <code>UpdateHealthCheck</code> request. This prevents Route 53 from overwriting an intervening update:</p> <ul> <li> <p>If the value in the <code>UpdateHealthCheck</code> request matches the value of <code>HealthCheckVersion</code> in the health check, Route 53 updates the health check with the new settings.</p> </li> <li> <p>If the value of <code>HealthCheckVersion</code> in the health check is greater, the health check was changed after you got the version number. Route 53 does not update the health check, and it returns a <code>HealthCheckVersionMismatch</code> error.</p> </li> </ul></p>
    pub health_check_version: Option<i64>,
    /// <p><p>The number of child health checks that are associated with a <code>CALCULATED</code> health that Amazon Route 53 must consider healthy for the <code>CALCULATED</code> health check to be considered healthy. To specify the child health checks that you want to associate with a <code>CALCULATED</code> health check, use the <code>ChildHealthChecks</code> and <code>ChildHealthCheck</code> elements.</p> <p>Note the following:</p> <ul> <li> <p>If you specify a number greater than the number of child health checks, Route 53 always considers this health check to be unhealthy.</p> </li> <li> <p>If you specify <code>0</code>, Route 53 always considers this health check to be healthy.</p> </li> </ul></p>
    pub health_threshold: Option<i64>,
    /// <p><p>The IPv4 or IPv6 IP address for the endpoint that you want Amazon Route 53 to perform health checks on. If you don&#39;t specify a value for <code>IPAddress</code>, Route 53 sends a DNS request to resolve the domain name that you specify in <code>FullyQualifiedDomainName</code> at the interval that you specify in <code>RequestInterval</code>. Using an IP address that is returned by DNS, Route 53 then checks the health of the endpoint.</p> <p>Use one of the following formats for the value of <code>IPAddress</code>: </p> <ul> <li> <p> <b>IPv4 address</b>: four values between 0 and 255, separated by periods (.), for example, <code>192.0.2.44</code>.</p> </li> <li> <p> <b>IPv6 address</b>: eight groups of four hexadecimal values, separated by colons (:), for example, <code>2001:0db8:85a3:0000:0000:abcd:0001:2345</code>. You can also shorten IPv6 addresses as described in RFC 5952, for example, <code>2001:db8:85a3::abcd:1:2345</code>.</p> </li> </ul> <p>If the endpoint is an EC2 instance, we recommend that you create an Elastic IP address, associate it with your EC2 instance, and specify the Elastic IP address for <code>IPAddress</code>. This ensures that the IP address of your instance never changes. For more information, see the applicable documentation:</p> <ul> <li> <p>Linux: <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html">Elastic IP Addresses (EIP)</a> in the <i>Amazon EC2 User Guide for Linux Instances</i> </p> </li> <li> <p>Windows: <a href="https://docs.aws.amazon.com/AWSEC2/latest/WindowsGuide/elastic-ip-addresses-eip.html">Elastic IP Addresses (EIP)</a> in the <i>Amazon EC2 User Guide for Windows Instances</i> </p> </li> </ul> <note> <p>If a health check already has a value for <code>IPAddress</code>, you can change the value. However, you can&#39;t update an existing health check to add or remove the value of <code>IPAddress</code>. </p> </note> <p>For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_UpdateHealthCheck.html#Route53-UpdateHealthCheck-request-FullyQualifiedDomainName">FullyQualifiedDomainName</a>. </p> <p>Constraints: Route 53 can&#39;t check the health of endpoints for which the IP address is in local, private, non-routable, or multicast ranges. For more information about IP addresses for which you can&#39;t create health checks, see the following documents:</p> <ul> <li> <p> <a href="https://tools.ietf.org/html/rfc5735">RFC 5735, Special Use IPv4 Addresses</a> </p> </li> <li> <p> <a href="https://tools.ietf.org/html/rfc6598">RFC 6598, IANA-Reserved IPv4 Prefix for Shared Address Space</a> </p> </li> <li> <p> <a href="https://tools.ietf.org/html/rfc5156">RFC 5156, Special-Use IPv6 Addresses</a> </p> </li> </ul></p>
    pub ip_address: Option<String>,
    /// <p><p>When CloudWatch has insufficient data about the metric to determine the alarm state, the status that you want Amazon Route 53 to assign to the health check:</p> <ul> <li> <p> <code>Healthy</code>: Route 53 considers the health check to be healthy.</p> </li> <li> <p> <code>Unhealthy</code>: Route 53 considers the health check to be unhealthy.</p> </li> <li> <p> <code>LastKnownStatus</code>: Route 53 uses the status of the health check from the last time CloudWatch had sufficient data to determine the alarm state. For new health checks that have no last known status, the default status for the health check is healthy.</p> </li> </ul></p>
    pub insufficient_data_health_status: Option<String>,
    /// <p>Specify whether you want Amazon Route 53 to invert the status of a health check, for example, to consider a health check unhealthy when it otherwise would be considered healthy.</p>
    pub inverted: Option<bool>,
    /// <p>The port on the endpoint on which you want Amazon Route 53 to perform health checks.</p>
    pub port: Option<i64>,
    /// <p>A complex type that contains one <code>Region</code> element for each region that you want Amazon Route 53 health checkers to check the specified endpoint from.</p>
    pub regions: Option<Vec<String>>,
    /// <p><p>A complex type that contains one <code>ResettableElementName</code> element for each element that you want to reset to the default value. Valid values for <code>ResettableElementName</code> include the following:</p> <ul> <li> <p> <code>ChildHealthChecks</code>: Amazon Route 53 resets <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_HealthCheckConfig.html#Route53-Type-HealthCheckConfig-ChildHealthChecks">ChildHealthChecks</a> to null.</p> </li> <li> <p> <code>FullyQualifiedDomainName</code>: Route 53 resets <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_UpdateHealthCheck.html#Route53-UpdateHealthCheck-request-FullyQualifiedDomainName">FullyQualifiedDomainName</a>. to null.</p> </li> <li> <p> <code>Regions</code>: Route 53 resets the <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_HealthCheckConfig.html#Route53-Type-HealthCheckConfig-Regions">Regions</a> list to the default set of regions. </p> </li> <li> <p> <code>ResourcePath</code>: Route 53 resets <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_HealthCheckConfig.html#Route53-Type-HealthCheckConfig-ResourcePath">ResourcePath</a> to null.</p> </li> </ul></p>
    pub reset_elements: Option<Vec<String>>,
    /// <p>The path that you want Amazon Route 53 to request when performing health checks. The path can be any value for which your endpoint will return an HTTP status code of 2xx or 3xx when the endpoint is healthy, for example the file /docs/route53-health-check.html. You can also include query string parameters, for example, <code>/welcome.html?language=jp&amp;login=y</code>. </p> <p>Specify this value only if you want to change it.</p>
    pub resource_path: Option<String>,
    /// <p>If the value of <code>Type</code> is <code>HTTP_STR_MATCH</code> or <code>HTTP_STR_MATCH</code>, the string that you want Amazon Route 53 to search for in the response body from the specified resource. If the string appears in the response body, Route 53 considers the resource healthy. (You can't change the value of <code>Type</code> when you update a health check.)</p>
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
        if let Some(ref value) = obj.disabled {
            &DisabledSerializer::serialize(&mut writer, "Disabled", value)?;
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
/// <p>A complex type that contains the response to the <code>UpdateHealthCheck</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateHealthCheckResponse {
    /// <p>A complex type that contains the response to an <code>UpdateHealthCheck</code> request.</p>
    pub health_check: HealthCheck,
}

struct UpdateHealthCheckResponseDeserializer;
impl UpdateHealthCheckResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateHealthCheckResponse, XmlParseError> {
        deserialize_elements::<_, UpdateHealthCheckResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HealthCheck" => {
                        obj.health_check =
                            HealthCheckDeserializer::deserialize("HealthCheck", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A request to update the comment for a hosted zone.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateHostedZoneCommentResponse {
    /// <p>A complex type that contains the response to the <code>UpdateHostedZoneComment</code> request.</p>
    pub hosted_zone: HostedZone,
}

struct UpdateHostedZoneCommentResponseDeserializer;
impl UpdateHostedZoneCommentResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateHostedZoneCommentResponse, XmlParseError> {
        deserialize_elements::<_, UpdateHostedZoneCommentResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HostedZone" => {
                        obj.hosted_zone = HostedZoneDeserializer::deserialize("HostedZone", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A complex type that contains information about the traffic policy that you want to update the comment for.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateTrafficPolicyCommentResponse {
    /// <p>A complex type that contains settings for the specified traffic policy.</p>
    pub traffic_policy: TrafficPolicy,
}

struct UpdateTrafficPolicyCommentResponseDeserializer;
impl UpdateTrafficPolicyCommentResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateTrafficPolicyCommentResponse, XmlParseError> {
        deserialize_elements::<_, UpdateTrafficPolicyCommentResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TrafficPolicy" => {
                        obj.traffic_policy =
                            TrafficPolicyDeserializer::deserialize("TrafficPolicy", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>A complex type that contains information about the resource record sets that you want to update based on a specified traffic policy instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateTrafficPolicyInstanceResponse {
    /// <p>A complex type that contains settings for the updated traffic policy instance.</p>
    pub traffic_policy_instance: TrafficPolicyInstance,
}

struct UpdateTrafficPolicyInstanceResponseDeserializer;
impl UpdateTrafficPolicyInstanceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateTrafficPolicyInstanceResponse, XmlParseError> {
        deserialize_elements::<_, UpdateTrafficPolicyInstanceResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TrafficPolicyInstance" => {
                        obj.traffic_policy_instance =
                            TrafficPolicyInstanceDeserializer::deserialize(
                                "TrafficPolicyInstance",
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
struct UsageCountDeserializer;
impl UsageCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>(Private hosted zones only) A complex type that contains information about an Amazon VPC.</p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VPC {
    pub vpc_id: Option<String>,
    /// <p>(Private hosted zones only) The region that an Amazon VPC was created in.</p>
    pub vpc_region: Option<String>,
}

struct VPCDeserializer;
impl VPCDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<VPC, XmlParseError> {
        deserialize_elements::<_, VPC, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "VPCId" => {
                    obj.vpc_id = Some(VPCIdDeserializer::deserialize("VPCId", stack)?);
                }
                "VPCRegion" => {
                    obj.vpc_region = Some(VPCRegionDeserializer::deserialize("VPCRegion", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<VPC>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "VPC" {
                obj.push(VPCDeserializer::deserialize("VPC", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// Errors returned by AssociateVPCWithHostedZone
#[derive(Debug, PartialEq)]
pub enum AssociateVPCWithHostedZoneError {
    /// <p><p>The cause of this error depends on whether you&#39;re trying to create a public or a private hosted zone:</p> <ul> <li> <p> <b>Public hosted zone:</b> Two hosted zones that have the same name or that have a parent/child relationship (example.com and test.example.com) can&#39;t have any common name servers. You tried to create a hosted zone that has the same name as an existing hosted zone or that&#39;s the parent or child of an existing hosted zone, and you specified a delegation set that shares one or more name servers with the existing hosted zone. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateReusableDelegationSet.html">CreateReusableDelegationSet</a>.</p> </li> <li> <p> <b>Private hosted zone:</b> You specified an Amazon VPC that you&#39;re already using for another hosted zone, and the domain that you specified for one of the hosted zones is a subdomain of the domain that you specified for the other hosted zone. For example, you can&#39;t use the same Amazon VPC for the hosted zones for example.com and test.example.com.</p> </li> </ul></p>
    ConflictingDomainExists(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC.</p>
    InvalidVPCId(String),
    /// <p>This operation can't be completed either because the current account has reached the limit on reusable delegation sets that it can create or because you've reached the limit on the number of Amazon VPCs that you can associate with a private hosted zone. To get the current limit on the number of reusable delegation sets, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetAccountLimit.html">GetAccountLimit</a>. To get the current limit on the number of Amazon VPCs that you can associate with a private hosted zone, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetHostedZoneLimit.html">GetHostedZoneLimit</a>. To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p>
    LimitsExceeded(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>Associating the specified VPC with the specified hosted zone has not been authorized.</p>
    NotAuthorized(String),
    /// <p>You're trying to associate a VPC with a public hosted zone. Amazon Route 53 doesn't support associating a VPC with a public hosted zone.</p>
    PublicZoneVPCAssociation(String),
}

impl AssociateVPCWithHostedZoneError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateVPCWithHostedZoneError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConflictingDomainExists" => {
                        return RusotoError::Service(
                            AssociateVPCWithHostedZoneError::ConflictingDomainExists(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(AssociateVPCWithHostedZoneError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVPCId" => {
                        return RusotoError::Service(AssociateVPCWithHostedZoneError::InvalidVPCId(
                            parsed_error.message,
                        ))
                    }
                    "LimitsExceeded" => {
                        return RusotoError::Service(
                            AssociateVPCWithHostedZoneError::LimitsExceeded(parsed_error.message),
                        )
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(
                            AssociateVPCWithHostedZoneError::NoSuchHostedZone(parsed_error.message),
                        )
                    }
                    "NotAuthorizedException" => {
                        return RusotoError::Service(
                            AssociateVPCWithHostedZoneError::NotAuthorized(parsed_error.message),
                        )
                    }
                    "PublicZoneVPCAssociation" => {
                        return RusotoError::Service(
                            AssociateVPCWithHostedZoneError::PublicZoneVPCAssociation(
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
impl fmt::Display for AssociateVPCWithHostedZoneError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateVPCWithHostedZoneError::ConflictingDomainExists(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateVPCWithHostedZoneError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AssociateVPCWithHostedZoneError::InvalidVPCId(ref cause) => write!(f, "{}", cause),
            AssociateVPCWithHostedZoneError::LimitsExceeded(ref cause) => write!(f, "{}", cause),
            AssociateVPCWithHostedZoneError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
            AssociateVPCWithHostedZoneError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AssociateVPCWithHostedZoneError::PublicZoneVPCAssociation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateVPCWithHostedZoneError {}
/// Errors returned by ChangeResourceRecordSets
#[derive(Debug, PartialEq)]
pub enum ChangeResourceRecordSetsError {
    /// <p>This exception contains a list of messages that might contain one or more error messages. Each error message indicates one error in the change batch.</p>
    InvalidChangeBatch(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the specified ID.</p>
    NoSuchHealthCheck(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
}

impl ChangeResourceRecordSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ChangeResourceRecordSetsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidChangeBatch" => {
                        return RusotoError::Service(
                            ChangeResourceRecordSetsError::InvalidChangeBatch(parsed_error.message),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(ChangeResourceRecordSetsError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHealthCheck" => {
                        return RusotoError::Service(
                            ChangeResourceRecordSetsError::NoSuchHealthCheck(parsed_error.message),
                        )
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(
                            ChangeResourceRecordSetsError::NoSuchHostedZone(parsed_error.message),
                        )
                    }
                    "PriorRequestNotComplete" => {
                        return RusotoError::Service(
                            ChangeResourceRecordSetsError::PriorRequestNotComplete(
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
impl fmt::Display for ChangeResourceRecordSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChangeResourceRecordSetsError::InvalidChangeBatch(ref cause) => write!(f, "{}", cause),
            ChangeResourceRecordSetsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ChangeResourceRecordSetsError::NoSuchHealthCheck(ref cause) => write!(f, "{}", cause),
            ChangeResourceRecordSetsError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
            ChangeResourceRecordSetsError::PriorRequestNotComplete(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ChangeResourceRecordSetsError {}
/// Errors returned by ChangeTagsForResource
#[derive(Debug, PartialEq)]
pub enum ChangeTagsForResourceError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the specified ID.</p>
    NoSuchHealthCheck(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl ChangeTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ChangeTagsForResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(ChangeTagsForResourceError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHealthCheck" => {
                        return RusotoError::Service(ChangeTagsForResourceError::NoSuchHealthCheck(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(ChangeTagsForResourceError::NoSuchHostedZone(
                            parsed_error.message,
                        ))
                    }
                    "PriorRequestNotComplete" => {
                        return RusotoError::Service(
                            ChangeTagsForResourceError::PriorRequestNotComplete(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ThrottlingException" => {
                        return RusotoError::Service(ChangeTagsForResourceError::Throttling(
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
impl fmt::Display for ChangeTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChangeTagsForResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ChangeTagsForResourceError::NoSuchHealthCheck(ref cause) => write!(f, "{}", cause),
            ChangeTagsForResourceError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
            ChangeTagsForResourceError::PriorRequestNotComplete(ref cause) => {
                write!(f, "{}", cause)
            }
            ChangeTagsForResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ChangeTagsForResourceError {}
/// Errors returned by CreateHealthCheck
#[derive(Debug, PartialEq)]
pub enum CreateHealthCheckError {
    /// <p><p> The health check you&#39;re attempting to create already exists. Amazon Route 53 returns this error when you submit a request that has the following values:</p> <ul> <li> <p>The same value for <code>CallerReference</code> as an existing health check, and one or more values that differ from the existing health check that has the same caller reference.</p> </li> <li> <p>The same value for <code>CallerReference</code> as a health check that you created and later deleted, regardless of the other settings in the request.</p> </li> </ul></p>
    HealthCheckAlreadyExists(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>This health check can't be created because the current account has reached the limit on the number of active health checks.</p> <p>For information about default limits, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>For information about how to get the current limit for an account, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetAccountLimit.html">GetAccountLimit</a>. To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p> <p>You have reached the maximum number of active health checks for an AWS account. To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p>
    TooManyHealthChecks(String),
}

impl CreateHealthCheckError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHealthCheckError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "HealthCheckAlreadyExists" => {
                        return RusotoError::Service(
                            CreateHealthCheckError::HealthCheckAlreadyExists(parsed_error.message),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(CreateHealthCheckError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "TooManyHealthChecks" => {
                        return RusotoError::Service(CreateHealthCheckError::TooManyHealthChecks(
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
impl fmt::Display for CreateHealthCheckError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateHealthCheckError::HealthCheckAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateHealthCheckError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateHealthCheckError::TooManyHealthChecks(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateHealthCheckError {}
/// Errors returned by CreateHostedZone
#[derive(Debug, PartialEq)]
pub enum CreateHostedZoneError {
    /// <p><p>The cause of this error depends on whether you&#39;re trying to create a public or a private hosted zone:</p> <ul> <li> <p> <b>Public hosted zone:</b> Two hosted zones that have the same name or that have a parent/child relationship (example.com and test.example.com) can&#39;t have any common name servers. You tried to create a hosted zone that has the same name as an existing hosted zone or that&#39;s the parent or child of an existing hosted zone, and you specified a delegation set that shares one or more name servers with the existing hosted zone. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateReusableDelegationSet.html">CreateReusableDelegationSet</a>.</p> </li> <li> <p> <b>Private hosted zone:</b> You specified an Amazon VPC that you&#39;re already using for another hosted zone, and the domain that you specified for one of the hosted zones is a subdomain of the domain that you specified for the other hosted zone. For example, you can&#39;t use the same Amazon VPC for the hosted zones for example.com and test.example.com.</p> </li> </ul></p>
    ConflictingDomainExists(String),
    /// <p>You can create a hosted zone that has the same name as an existing hosted zone (example.com is common), but there is a limit to the number of hosted zones that have the same name. If you get this error, Amazon Route 53 has reached that limit. If you own the domain name and Route 53 generates this error, contact Customer Support.</p>
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
    /// <p>This operation can't be completed either because the current account has reached the limit on the number of hosted zones or because you've reached the limit on the number of hosted zones that can be associated with a reusable delegation set.</p> <p>For information about default limits, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>To get the current limit on hosted zones that can be created by an account, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetAccountLimit.html">GetAccountLimit</a>.</p> <p>To get the current limit on hosted zones that can be associated with a reusable delegation set, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetReusableDelegationSetLimit.html">GetReusableDelegationSetLimit</a>.</p> <p>To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p>
    TooManyHostedZones(String),
}

impl CreateHostedZoneError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHostedZoneError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConflictingDomainExists" => {
                        return RusotoError::Service(
                            CreateHostedZoneError::ConflictingDomainExists(parsed_error.message),
                        )
                    }
                    "DelegationSetNotAvailable" => {
                        return RusotoError::Service(
                            CreateHostedZoneError::DelegationSetNotAvailable(parsed_error.message),
                        )
                    }
                    "DelegationSetNotReusable" => {
                        return RusotoError::Service(
                            CreateHostedZoneError::DelegationSetNotReusable(parsed_error.message),
                        )
                    }
                    "HostedZoneAlreadyExists" => {
                        return RusotoError::Service(
                            CreateHostedZoneError::HostedZoneAlreadyExists(parsed_error.message),
                        )
                    }
                    "InvalidDomainName" => {
                        return RusotoError::Service(CreateHostedZoneError::InvalidDomainName(
                            parsed_error.message,
                        ))
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(CreateHostedZoneError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "InvalidVPCId" => {
                        return RusotoError::Service(CreateHostedZoneError::InvalidVPCId(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDelegationSet" => {
                        return RusotoError::Service(CreateHostedZoneError::NoSuchDelegationSet(
                            parsed_error.message,
                        ))
                    }
                    "TooManyHostedZones" => {
                        return RusotoError::Service(CreateHostedZoneError::TooManyHostedZones(
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
impl fmt::Display for CreateHostedZoneError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateHostedZoneError::ConflictingDomainExists(ref cause) => write!(f, "{}", cause),
            CreateHostedZoneError::DelegationSetNotAvailable(ref cause) => write!(f, "{}", cause),
            CreateHostedZoneError::DelegationSetNotReusable(ref cause) => write!(f, "{}", cause),
            CreateHostedZoneError::HostedZoneAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateHostedZoneError::InvalidDomainName(ref cause) => write!(f, "{}", cause),
            CreateHostedZoneError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateHostedZoneError::InvalidVPCId(ref cause) => write!(f, "{}", cause),
            CreateHostedZoneError::NoSuchDelegationSet(ref cause) => write!(f, "{}", cause),
            CreateHostedZoneError::TooManyHostedZones(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateHostedZoneError {}
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
}

impl CreateQueryLoggingConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateQueryLoggingConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConcurrentModification" => {
                        return RusotoError::Service(
                            CreateQueryLoggingConfigError::ConcurrentModification(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InsufficientCloudWatchLogsResourcePolicy" => {
                        return RusotoError::Service(
                            CreateQueryLoggingConfigError::InsufficientCloudWatchLogsResourcePolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(CreateQueryLoggingConfigError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchCloudWatchLogsLogGroup" => {
                        return RusotoError::Service(
                            CreateQueryLoggingConfigError::NoSuchCloudWatchLogsLogGroup(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(
                            CreateQueryLoggingConfigError::NoSuchHostedZone(parsed_error.message),
                        )
                    }
                    "QueryLoggingConfigAlreadyExists" => {
                        return RusotoError::Service(
                            CreateQueryLoggingConfigError::QueryLoggingConfigAlreadyExists(
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
impl fmt::Display for CreateQueryLoggingConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateQueryLoggingConfigError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateQueryLoggingConfigError::InsufficientCloudWatchLogsResourcePolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateQueryLoggingConfigError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateQueryLoggingConfigError::NoSuchCloudWatchLogsLogGroup(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateQueryLoggingConfigError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
            CreateQueryLoggingConfigError::QueryLoggingConfigAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateQueryLoggingConfigError {}
/// Errors returned by CreateReusableDelegationSet
#[derive(Debug, PartialEq)]
pub enum CreateReusableDelegationSetError {
    /// <p>A delegation set with the same owner and caller reference combination has already been created.</p>
    DelegationSetAlreadyCreated(String),
    /// <p>The specified delegation set has already been marked as reusable.</p>
    DelegationSetAlreadyReusable(String),
    /// <p>You can create a hosted zone that has the same name as an existing hosted zone (example.com is common), but there is a limit to the number of hosted zones that have the same name. If you get this error, Amazon Route 53 has reached that limit. If you own the domain name and Route 53 generates this error, contact Customer Support.</p>
    DelegationSetNotAvailable(String),
    /// <p>The specified HostedZone can't be found.</p>
    HostedZoneNotFound(String),
    /// <p>Parameter name is invalid.</p>
    InvalidArgument(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>This operation can't be completed either because the current account has reached the limit on reusable delegation sets that it can create or because you've reached the limit on the number of Amazon VPCs that you can associate with a private hosted zone. To get the current limit on the number of reusable delegation sets, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetAccountLimit.html">GetAccountLimit</a>. To get the current limit on the number of Amazon VPCs that you can associate with a private hosted zone, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetHostedZoneLimit.html">GetHostedZoneLimit</a>. To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p>
    LimitsExceeded(String),
}

impl CreateReusableDelegationSetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateReusableDelegationSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DelegationSetAlreadyCreated" => {
                        return RusotoError::Service(
                            CreateReusableDelegationSetError::DelegationSetAlreadyCreated(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DelegationSetAlreadyReusable" => {
                        return RusotoError::Service(
                            CreateReusableDelegationSetError::DelegationSetAlreadyReusable(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DelegationSetNotAvailable" => {
                        return RusotoError::Service(
                            CreateReusableDelegationSetError::DelegationSetNotAvailable(
                                parsed_error.message,
                            ),
                        )
                    }
                    "HostedZoneNotFound" => {
                        return RusotoError::Service(
                            CreateReusableDelegationSetError::HostedZoneNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidArgument" => {
                        return RusotoError::Service(
                            CreateReusableDelegationSetError::InvalidArgument(parsed_error.message),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(
                            CreateReusableDelegationSetError::InvalidInput(parsed_error.message),
                        )
                    }
                    "LimitsExceeded" => {
                        return RusotoError::Service(
                            CreateReusableDelegationSetError::LimitsExceeded(parsed_error.message),
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
impl fmt::Display for CreateReusableDelegationSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateReusableDelegationSetError::DelegationSetAlreadyCreated(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReusableDelegationSetError::DelegationSetAlreadyReusable(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReusableDelegationSetError::DelegationSetNotAvailable(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReusableDelegationSetError::HostedZoneNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReusableDelegationSetError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateReusableDelegationSetError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateReusableDelegationSetError::LimitsExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateReusableDelegationSetError {}
/// Errors returned by CreateTrafficPolicy
#[derive(Debug, PartialEq)]
pub enum CreateTrafficPolicyError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>The format of the traffic policy document that you specified in the <code>Document</code> element is invalid.</p>
    InvalidTrafficPolicyDocument(String),
    /// <p>This traffic policy can't be created because the current account has reached the limit on the number of traffic policies.</p> <p>For information about default limits, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>To get the current limit for an account, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetAccountLimit.html">GetAccountLimit</a>. </p> <p>To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p>
    TooManyTrafficPolicies(String),
    /// <p>A traffic policy that has the same value for <code>Name</code> already exists.</p>
    TrafficPolicyAlreadyExists(String),
}

impl CreateTrafficPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTrafficPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(CreateTrafficPolicyError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "InvalidTrafficPolicyDocument" => {
                        return RusotoError::Service(
                            CreateTrafficPolicyError::InvalidTrafficPolicyDocument(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyTrafficPolicies" => {
                        return RusotoError::Service(
                            CreateTrafficPolicyError::TooManyTrafficPolicies(parsed_error.message),
                        )
                    }
                    "TrafficPolicyAlreadyExists" => {
                        return RusotoError::Service(
                            CreateTrafficPolicyError::TrafficPolicyAlreadyExists(
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
impl fmt::Display for CreateTrafficPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTrafficPolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateTrafficPolicyError::InvalidTrafficPolicyDocument(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateTrafficPolicyError::TooManyTrafficPolicies(ref cause) => write!(f, "{}", cause),
            CreateTrafficPolicyError::TrafficPolicyAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateTrafficPolicyError {}
/// Errors returned by CreateTrafficPolicyInstance
#[derive(Debug, PartialEq)]
pub enum CreateTrafficPolicyInstanceError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>No traffic policy exists with the specified ID.</p>
    NoSuchTrafficPolicy(String),
    /// <p>This traffic policy instance can't be created because the current account has reached the limit on the number of traffic policy instances.</p> <p>For information about default limits, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>For information about how to get the current limit for an account, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetAccountLimit.html">GetAccountLimit</a>.</p> <p>To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p>
    TooManyTrafficPolicyInstances(String),
    /// <p>There is already a traffic policy instance with the specified ID.</p>
    TrafficPolicyInstanceAlreadyExists(String),
}

impl CreateTrafficPolicyInstanceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateTrafficPolicyInstanceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(
                            CreateTrafficPolicyInstanceError::InvalidInput(parsed_error.message),
                        )
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(
                            CreateTrafficPolicyInstanceError::NoSuchHostedZone(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchTrafficPolicy" => {
                        return RusotoError::Service(
                            CreateTrafficPolicyInstanceError::NoSuchTrafficPolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyTrafficPolicyInstances" => {
                        return RusotoError::Service(
                            CreateTrafficPolicyInstanceError::TooManyTrafficPolicyInstances(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TrafficPolicyInstanceAlreadyExists" => {
                        return RusotoError::Service(
                            CreateTrafficPolicyInstanceError::TrafficPolicyInstanceAlreadyExists(
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
impl fmt::Display for CreateTrafficPolicyInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTrafficPolicyInstanceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateTrafficPolicyInstanceError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
            CreateTrafficPolicyInstanceError::NoSuchTrafficPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateTrafficPolicyInstanceError::TooManyTrafficPolicyInstances(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateTrafficPolicyInstanceError::TrafficPolicyInstanceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateTrafficPolicyInstanceError {}
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
    /// <p>This traffic policy version can't be created because you've reached the limit of 1000 on the number of versions that you can create for the current traffic policy.</p> <p>To create more traffic policy versions, you can use <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetTrafficPolicy.html">GetTrafficPolicy</a> to get the traffic policy document for a specified traffic policy version, and then use <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateTrafficPolicy.html">CreateTrafficPolicy</a> to create a new traffic policy using the traffic policy document.</p>
    TooManyTrafficPolicyVersionsForCurrentPolicy(String),
}

impl CreateTrafficPolicyVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateTrafficPolicyVersionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "ConcurrentModification" => return RusotoError::Service(CreateTrafficPolicyVersionError::ConcurrentModification(parsed_error.message)),"InvalidInput" => return RusotoError::Service(CreateTrafficPolicyVersionError::InvalidInput(parsed_error.message)),"InvalidTrafficPolicyDocument" => return RusotoError::Service(CreateTrafficPolicyVersionError::InvalidTrafficPolicyDocument(parsed_error.message)),"NoSuchTrafficPolicy" => return RusotoError::Service(CreateTrafficPolicyVersionError::NoSuchTrafficPolicy(parsed_error.message)),"TooManyTrafficPolicyVersionsForCurrentPolicy" => return RusotoError::Service(CreateTrafficPolicyVersionError::TooManyTrafficPolicyVersionsForCurrentPolicy(parsed_error.message)),_ => {}
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
impl fmt::Display for CreateTrafficPolicyVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTrafficPolicyVersionError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateTrafficPolicyVersionError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateTrafficPolicyVersionError::InvalidTrafficPolicyDocument(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateTrafficPolicyVersionError::NoSuchTrafficPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateTrafficPolicyVersionError::TooManyTrafficPolicyVersionsForCurrentPolicy(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTrafficPolicyVersionError {}
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
}

impl CreateVPCAssociationAuthorizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateVPCAssociationAuthorizationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConcurrentModification" => {
                        return RusotoError::Service(
                            CreateVPCAssociationAuthorizationError::ConcurrentModification(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(
                            CreateVPCAssociationAuthorizationError::InvalidInput(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidVPCId" => {
                        return RusotoError::Service(
                            CreateVPCAssociationAuthorizationError::InvalidVPCId(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(
                            CreateVPCAssociationAuthorizationError::NoSuchHostedZone(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TooManyVPCAssociationAuthorizations" => return RusotoError::Service(
                        CreateVPCAssociationAuthorizationError::TooManyVPCAssociationAuthorizations(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateVPCAssociationAuthorizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVPCAssociationAuthorizationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateVPCAssociationAuthorizationError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateVPCAssociationAuthorizationError::InvalidVPCId(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateVPCAssociationAuthorizationError::NoSuchHostedZone(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateVPCAssociationAuthorizationError::TooManyVPCAssociationAuthorizations(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVPCAssociationAuthorizationError {}
/// Errors returned by DeleteHealthCheck
#[derive(Debug, PartialEq)]
pub enum DeleteHealthCheckError {
    /// <p>This error code is not in use.</p>
    HealthCheckInUse(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the specified ID.</p>
    NoSuchHealthCheck(String),
}

impl DeleteHealthCheckError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteHealthCheckError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "HealthCheckInUse" => {
                        return RusotoError::Service(DeleteHealthCheckError::HealthCheckInUse(
                            parsed_error.message,
                        ))
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(DeleteHealthCheckError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHealthCheck" => {
                        return RusotoError::Service(DeleteHealthCheckError::NoSuchHealthCheck(
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
impl fmt::Display for DeleteHealthCheckError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteHealthCheckError::HealthCheckInUse(ref cause) => write!(f, "{}", cause),
            DeleteHealthCheckError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteHealthCheckError::NoSuchHealthCheck(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteHealthCheckError {}
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
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
}

impl DeleteHostedZoneError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteHostedZoneError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "HostedZoneNotEmpty" => {
                        return RusotoError::Service(DeleteHostedZoneError::HostedZoneNotEmpty(
                            parsed_error.message,
                        ))
                    }
                    "InvalidDomainName" => {
                        return RusotoError::Service(DeleteHostedZoneError::InvalidDomainName(
                            parsed_error.message,
                        ))
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(DeleteHostedZoneError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(DeleteHostedZoneError::NoSuchHostedZone(
                            parsed_error.message,
                        ))
                    }
                    "PriorRequestNotComplete" => {
                        return RusotoError::Service(
                            DeleteHostedZoneError::PriorRequestNotComplete(parsed_error.message),
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
impl fmt::Display for DeleteHostedZoneError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteHostedZoneError::HostedZoneNotEmpty(ref cause) => write!(f, "{}", cause),
            DeleteHostedZoneError::InvalidDomainName(ref cause) => write!(f, "{}", cause),
            DeleteHostedZoneError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteHostedZoneError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
            DeleteHostedZoneError::PriorRequestNotComplete(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteHostedZoneError {}
/// Errors returned by DeleteQueryLoggingConfig
#[derive(Debug, PartialEq)]
pub enum DeleteQueryLoggingConfigError {
    /// <p>Another user submitted a request to create, update, or delete the object at the same time that you did. Retry the request. </p>
    ConcurrentModification(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>There is no DNS query logging configuration with the specified ID.</p>
    NoSuchQueryLoggingConfig(String),
}

impl DeleteQueryLoggingConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteQueryLoggingConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConcurrentModification" => {
                        return RusotoError::Service(
                            DeleteQueryLoggingConfigError::ConcurrentModification(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(DeleteQueryLoggingConfigError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchQueryLoggingConfig" => {
                        return RusotoError::Service(
                            DeleteQueryLoggingConfigError::NoSuchQueryLoggingConfig(
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
impl fmt::Display for DeleteQueryLoggingConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteQueryLoggingConfigError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteQueryLoggingConfigError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteQueryLoggingConfigError::NoSuchQueryLoggingConfig(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteQueryLoggingConfigError {}
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
}

impl DeleteReusableDelegationSetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteReusableDelegationSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DelegationSetInUse" => {
                        return RusotoError::Service(
                            DeleteReusableDelegationSetError::DelegationSetInUse(
                                parsed_error.message,
                            ),
                        )
                    }
                    "DelegationSetNotReusable" => {
                        return RusotoError::Service(
                            DeleteReusableDelegationSetError::DelegationSetNotReusable(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(
                            DeleteReusableDelegationSetError::InvalidInput(parsed_error.message),
                        )
                    }
                    "NoSuchDelegationSet" => {
                        return RusotoError::Service(
                            DeleteReusableDelegationSetError::NoSuchDelegationSet(
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
impl fmt::Display for DeleteReusableDelegationSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReusableDelegationSetError::DelegationSetInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteReusableDelegationSetError::DelegationSetNotReusable(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteReusableDelegationSetError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteReusableDelegationSetError::NoSuchDelegationSet(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteReusableDelegationSetError {}
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
}

impl DeleteTrafficPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTrafficPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConcurrentModification" => {
                        return RusotoError::Service(
                            DeleteTrafficPolicyError::ConcurrentModification(parsed_error.message),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(DeleteTrafficPolicyError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchTrafficPolicy" => {
                        return RusotoError::Service(DeleteTrafficPolicyError::NoSuchTrafficPolicy(
                            parsed_error.message,
                        ))
                    }
                    "TrafficPolicyInUse" => {
                        return RusotoError::Service(DeleteTrafficPolicyError::TrafficPolicyInUse(
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
impl fmt::Display for DeleteTrafficPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTrafficPolicyError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteTrafficPolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteTrafficPolicyError::NoSuchTrafficPolicy(ref cause) => write!(f, "{}", cause),
            DeleteTrafficPolicyError::TrafficPolicyInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTrafficPolicyError {}
/// Errors returned by DeleteTrafficPolicyInstance
#[derive(Debug, PartialEq)]
pub enum DeleteTrafficPolicyInstanceError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy instance exists with the specified ID.</p>
    NoSuchTrafficPolicyInstance(String),
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
}

impl DeleteTrafficPolicyInstanceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteTrafficPolicyInstanceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(
                            DeleteTrafficPolicyInstanceError::InvalidInput(parsed_error.message),
                        )
                    }
                    "NoSuchTrafficPolicyInstance" => {
                        return RusotoError::Service(
                            DeleteTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(
                                parsed_error.message,
                            ),
                        )
                    }
                    "PriorRequestNotComplete" => {
                        return RusotoError::Service(
                            DeleteTrafficPolicyInstanceError::PriorRequestNotComplete(
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
impl fmt::Display for DeleteTrafficPolicyInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTrafficPolicyInstanceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteTrafficPolicyInstanceError::PriorRequestNotComplete(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteTrafficPolicyInstanceError {}
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
}

impl DeleteVPCAssociationAuthorizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteVPCAssociationAuthorizationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConcurrentModification" => {
                        return RusotoError::Service(
                            DeleteVPCAssociationAuthorizationError::ConcurrentModification(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(
                            DeleteVPCAssociationAuthorizationError::InvalidInput(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidVPCId" => {
                        return RusotoError::Service(
                            DeleteVPCAssociationAuthorizationError::InvalidVPCId(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(
                            DeleteVPCAssociationAuthorizationError::NoSuchHostedZone(
                                parsed_error.message,
                            ),
                        )
                    }
                    "VPCAssociationAuthorizationNotFound" => return RusotoError::Service(
                        DeleteVPCAssociationAuthorizationError::VPCAssociationAuthorizationNotFound(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteVPCAssociationAuthorizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVPCAssociationAuthorizationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVPCAssociationAuthorizationError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVPCAssociationAuthorizationError::InvalidVPCId(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVPCAssociationAuthorizationError::NoSuchHostedZone(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVPCAssociationAuthorizationError::VPCAssociationAuthorizationNotFound(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVPCAssociationAuthorizationError {}
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
}

impl DisassociateVPCFromHostedZoneError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateVPCFromHostedZoneError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(
                            DisassociateVPCFromHostedZoneError::InvalidInput(parsed_error.message),
                        )
                    }
                    "InvalidVPCId" => {
                        return RusotoError::Service(
                            DisassociateVPCFromHostedZoneError::InvalidVPCId(parsed_error.message),
                        )
                    }
                    "LastVPCAssociation" => {
                        return RusotoError::Service(
                            DisassociateVPCFromHostedZoneError::LastVPCAssociation(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(
                            DisassociateVPCFromHostedZoneError::NoSuchHostedZone(
                                parsed_error.message,
                            ),
                        )
                    }
                    "VPCAssociationNotFound" => {
                        return RusotoError::Service(
                            DisassociateVPCFromHostedZoneError::VPCAssociationNotFound(
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
impl fmt::Display for DisassociateVPCFromHostedZoneError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateVPCFromHostedZoneError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DisassociateVPCFromHostedZoneError::InvalidVPCId(ref cause) => write!(f, "{}", cause),
            DisassociateVPCFromHostedZoneError::LastVPCAssociation(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateVPCFromHostedZoneError::NoSuchHostedZone(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateVPCFromHostedZoneError::VPCAssociationNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateVPCFromHostedZoneError {}
/// Errors returned by GetAccountLimit
#[derive(Debug, PartialEq)]
pub enum GetAccountLimitError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
}

impl GetAccountLimitError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAccountLimitError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(GetAccountLimitError::InvalidInput(
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
impl fmt::Display for GetAccountLimitError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAccountLimitError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAccountLimitError {}
/// Errors returned by GetChange
#[derive(Debug, PartialEq)]
pub enum GetChangeError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>A change with the specified change ID does not exist.</p>
    NoSuchChange(String),
}

impl GetChangeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetChangeError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(GetChangeError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchChange" => {
                        return RusotoError::Service(GetChangeError::NoSuchChange(
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
impl fmt::Display for GetChangeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetChangeError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetChangeError::NoSuchChange(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetChangeError {}
/// Errors returned by GetCheckerIpRanges
#[derive(Debug, PartialEq)]
pub enum GetCheckerIpRangesError {}

impl GetCheckerIpRangesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCheckerIpRangesError> {
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
impl fmt::Display for GetCheckerIpRangesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetCheckerIpRangesError {}
/// Errors returned by GetGeoLocation
#[derive(Debug, PartialEq)]
pub enum GetGeoLocationError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>Amazon Route 53 doesn't support the specified geographic location.</p>
    NoSuchGeoLocation(String),
}

impl GetGeoLocationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGeoLocationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(GetGeoLocationError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchGeoLocation" => {
                        return RusotoError::Service(GetGeoLocationError::NoSuchGeoLocation(
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
impl fmt::Display for GetGeoLocationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGeoLocationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetGeoLocationError::NoSuchGeoLocation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGeoLocationError {}
/// Errors returned by GetHealthCheck
#[derive(Debug, PartialEq)]
pub enum GetHealthCheckError {
    /// <p>The resource you're trying to access is unsupported on this Amazon Route 53 endpoint.</p>
    IncompatibleVersion(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the specified ID.</p>
    NoSuchHealthCheck(String),
}

impl GetHealthCheckError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetHealthCheckError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "IncompatibleVersion" => {
                        return RusotoError::Service(GetHealthCheckError::IncompatibleVersion(
                            parsed_error.message,
                        ))
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(GetHealthCheckError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHealthCheck" => {
                        return RusotoError::Service(GetHealthCheckError::NoSuchHealthCheck(
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
impl fmt::Display for GetHealthCheckError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetHealthCheckError::IncompatibleVersion(ref cause) => write!(f, "{}", cause),
            GetHealthCheckError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetHealthCheckError::NoSuchHealthCheck(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetHealthCheckError {}
/// Errors returned by GetHealthCheckCount
#[derive(Debug, PartialEq)]
pub enum GetHealthCheckCountError {}

impl GetHealthCheckCountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetHealthCheckCountError> {
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
impl fmt::Display for GetHealthCheckCountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetHealthCheckCountError {}
/// Errors returned by GetHealthCheckLastFailureReason
#[derive(Debug, PartialEq)]
pub enum GetHealthCheckLastFailureReasonError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the specified ID.</p>
    NoSuchHealthCheck(String),
}

impl GetHealthCheckLastFailureReasonError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetHealthCheckLastFailureReasonError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(
                            GetHealthCheckLastFailureReasonError::InvalidInput(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchHealthCheck" => {
                        return RusotoError::Service(
                            GetHealthCheckLastFailureReasonError::NoSuchHealthCheck(
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
impl fmt::Display for GetHealthCheckLastFailureReasonError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetHealthCheckLastFailureReasonError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetHealthCheckLastFailureReasonError::NoSuchHealthCheck(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetHealthCheckLastFailureReasonError {}
/// Errors returned by GetHealthCheckStatus
#[derive(Debug, PartialEq)]
pub enum GetHealthCheckStatusError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the specified ID.</p>
    NoSuchHealthCheck(String),
}

impl GetHealthCheckStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetHealthCheckStatusError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(GetHealthCheckStatusError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHealthCheck" => {
                        return RusotoError::Service(GetHealthCheckStatusError::NoSuchHealthCheck(
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
impl fmt::Display for GetHealthCheckStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetHealthCheckStatusError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetHealthCheckStatusError::NoSuchHealthCheck(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetHealthCheckStatusError {}
/// Errors returned by GetHostedZone
#[derive(Debug, PartialEq)]
pub enum GetHostedZoneError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
}

impl GetHostedZoneError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetHostedZoneError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(GetHostedZoneError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(GetHostedZoneError::NoSuchHostedZone(
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
impl fmt::Display for GetHostedZoneError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetHostedZoneError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetHostedZoneError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetHostedZoneError {}
/// Errors returned by GetHostedZoneCount
#[derive(Debug, PartialEq)]
pub enum GetHostedZoneCountError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
}

impl GetHostedZoneCountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetHostedZoneCountError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(GetHostedZoneCountError::InvalidInput(
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
impl fmt::Display for GetHostedZoneCountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetHostedZoneCountError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetHostedZoneCountError {}
/// Errors returned by GetHostedZoneLimit
#[derive(Debug, PartialEq)]
pub enum GetHostedZoneLimitError {
    /// <p>The specified hosted zone is a public hosted zone, not a private hosted zone.</p>
    HostedZoneNotPrivate(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
}

impl GetHostedZoneLimitError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetHostedZoneLimitError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "HostedZoneNotPrivate" => {
                        return RusotoError::Service(GetHostedZoneLimitError::HostedZoneNotPrivate(
                            parsed_error.message,
                        ))
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(GetHostedZoneLimitError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(GetHostedZoneLimitError::NoSuchHostedZone(
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
impl fmt::Display for GetHostedZoneLimitError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetHostedZoneLimitError::HostedZoneNotPrivate(ref cause) => write!(f, "{}", cause),
            GetHostedZoneLimitError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetHostedZoneLimitError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetHostedZoneLimitError {}
/// Errors returned by GetQueryLoggingConfig
#[derive(Debug, PartialEq)]
pub enum GetQueryLoggingConfigError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>There is no DNS query logging configuration with the specified ID.</p>
    NoSuchQueryLoggingConfig(String),
}

impl GetQueryLoggingConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetQueryLoggingConfigError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(GetQueryLoggingConfigError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchQueryLoggingConfig" => {
                        return RusotoError::Service(
                            GetQueryLoggingConfigError::NoSuchQueryLoggingConfig(
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
impl fmt::Display for GetQueryLoggingConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetQueryLoggingConfigError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetQueryLoggingConfigError::NoSuchQueryLoggingConfig(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetQueryLoggingConfigError {}
/// Errors returned by GetReusableDelegationSet
#[derive(Debug, PartialEq)]
pub enum GetReusableDelegationSetError {
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    DelegationSetNotReusable(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    NoSuchDelegationSet(String),
}

impl GetReusableDelegationSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetReusableDelegationSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DelegationSetNotReusable" => {
                        return RusotoError::Service(
                            GetReusableDelegationSetError::DelegationSetNotReusable(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(GetReusableDelegationSetError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDelegationSet" => {
                        return RusotoError::Service(
                            GetReusableDelegationSetError::NoSuchDelegationSet(
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
impl fmt::Display for GetReusableDelegationSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetReusableDelegationSetError::DelegationSetNotReusable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetReusableDelegationSetError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetReusableDelegationSetError::NoSuchDelegationSet(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetReusableDelegationSetError {}
/// Errors returned by GetReusableDelegationSetLimit
#[derive(Debug, PartialEq)]
pub enum GetReusableDelegationSetLimitError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    NoSuchDelegationSet(String),
}

impl GetReusableDelegationSetLimitError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetReusableDelegationSetLimitError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(
                            GetReusableDelegationSetLimitError::InvalidInput(parsed_error.message),
                        )
                    }
                    "NoSuchDelegationSet" => {
                        return RusotoError::Service(
                            GetReusableDelegationSetLimitError::NoSuchDelegationSet(
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
impl fmt::Display for GetReusableDelegationSetLimitError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetReusableDelegationSetLimitError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetReusableDelegationSetLimitError::NoSuchDelegationSet(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetReusableDelegationSetLimitError {}
/// Errors returned by GetTrafficPolicy
#[derive(Debug, PartialEq)]
pub enum GetTrafficPolicyError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy exists with the specified ID.</p>
    NoSuchTrafficPolicy(String),
}

impl GetTrafficPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTrafficPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(GetTrafficPolicyError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchTrafficPolicy" => {
                        return RusotoError::Service(GetTrafficPolicyError::NoSuchTrafficPolicy(
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
impl fmt::Display for GetTrafficPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTrafficPolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetTrafficPolicyError::NoSuchTrafficPolicy(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTrafficPolicyError {}
/// Errors returned by GetTrafficPolicyInstance
#[derive(Debug, PartialEq)]
pub enum GetTrafficPolicyInstanceError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy instance exists with the specified ID.</p>
    NoSuchTrafficPolicyInstance(String),
}

impl GetTrafficPolicyInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTrafficPolicyInstanceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(GetTrafficPolicyInstanceError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchTrafficPolicyInstance" => {
                        return RusotoError::Service(
                            GetTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(
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
impl fmt::Display for GetTrafficPolicyInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTrafficPolicyInstanceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetTrafficPolicyInstanceError {}
/// Errors returned by GetTrafficPolicyInstanceCount
#[derive(Debug, PartialEq)]
pub enum GetTrafficPolicyInstanceCountError {}

impl GetTrafficPolicyInstanceCountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetTrafficPolicyInstanceCountError> {
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
impl fmt::Display for GetTrafficPolicyInstanceCountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetTrafficPolicyInstanceCountError {}
/// Errors returned by ListGeoLocations
#[derive(Debug, PartialEq)]
pub enum ListGeoLocationsError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
}

impl ListGeoLocationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGeoLocationsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(ListGeoLocationsError::InvalidInput(
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
impl fmt::Display for ListGeoLocationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGeoLocationsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGeoLocationsError {}
/// Errors returned by ListHealthChecks
#[derive(Debug, PartialEq)]
pub enum ListHealthChecksError {
    /// <p>The resource you're trying to access is unsupported on this Amazon Route 53 endpoint.</p>
    IncompatibleVersion(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
}

impl ListHealthChecksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHealthChecksError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "IncompatibleVersion" => {
                        return RusotoError::Service(ListHealthChecksError::IncompatibleVersion(
                            parsed_error.message,
                        ))
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(ListHealthChecksError::InvalidInput(
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
impl fmt::Display for ListHealthChecksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListHealthChecksError::IncompatibleVersion(ref cause) => write!(f, "{}", cause),
            ListHealthChecksError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListHealthChecksError {}
/// Errors returned by ListHostedZones
#[derive(Debug, PartialEq)]
pub enum ListHostedZonesError {
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    DelegationSetNotReusable(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    NoSuchDelegationSet(String),
}

impl ListHostedZonesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHostedZonesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DelegationSetNotReusable" => {
                        return RusotoError::Service(
                            ListHostedZonesError::DelegationSetNotReusable(parsed_error.message),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(ListHostedZonesError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDelegationSet" => {
                        return RusotoError::Service(ListHostedZonesError::NoSuchDelegationSet(
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
impl fmt::Display for ListHostedZonesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListHostedZonesError::DelegationSetNotReusable(ref cause) => write!(f, "{}", cause),
            ListHostedZonesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListHostedZonesError::NoSuchDelegationSet(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListHostedZonesError {}
/// Errors returned by ListHostedZonesByName
#[derive(Debug, PartialEq)]
pub enum ListHostedZonesByNameError {
    /// <p>The specified domain name is not valid.</p>
    InvalidDomainName(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
}

impl ListHostedZonesByNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHostedZonesByNameError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidDomainName" => {
                        return RusotoError::Service(ListHostedZonesByNameError::InvalidDomainName(
                            parsed_error.message,
                        ))
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(ListHostedZonesByNameError::InvalidInput(
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
impl fmt::Display for ListHostedZonesByNameError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListHostedZonesByNameError::InvalidDomainName(ref cause) => write!(f, "{}", cause),
            ListHostedZonesByNameError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListHostedZonesByNameError {}
/// Errors returned by ListQueryLoggingConfigs
#[derive(Debug, PartialEq)]
pub enum ListQueryLoggingConfigsError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>The value that you specified to get the second or subsequent page of results is invalid.</p>
    InvalidPaginationToken(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
}

impl ListQueryLoggingConfigsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListQueryLoggingConfigsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(ListQueryLoggingConfigsError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "InvalidPaginationToken" => {
                        return RusotoError::Service(
                            ListQueryLoggingConfigsError::InvalidPaginationToken(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(
                            ListQueryLoggingConfigsError::NoSuchHostedZone(parsed_error.message),
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
impl fmt::Display for ListQueryLoggingConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListQueryLoggingConfigsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListQueryLoggingConfigsError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListQueryLoggingConfigsError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListQueryLoggingConfigsError {}
/// Errors returned by ListResourceRecordSets
#[derive(Debug, PartialEq)]
pub enum ListResourceRecordSetsError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
}

impl ListResourceRecordSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourceRecordSetsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(ListResourceRecordSetsError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(ListResourceRecordSetsError::NoSuchHostedZone(
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
impl fmt::Display for ListResourceRecordSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourceRecordSetsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListResourceRecordSetsError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResourceRecordSetsError {}
/// Errors returned by ListReusableDelegationSets
#[derive(Debug, PartialEq)]
pub enum ListReusableDelegationSetsError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
}

impl ListReusableDelegationSetsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListReusableDelegationSetsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(ListReusableDelegationSetsError::InvalidInput(
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
impl fmt::Display for ListReusableDelegationSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListReusableDelegationSetsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListReusableDelegationSetsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the specified ID.</p>
    NoSuchHealthCheck(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(ListTagsForResourceError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHealthCheck" => {
                        return RusotoError::Service(ListTagsForResourceError::NoSuchHealthCheck(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(ListTagsForResourceError::NoSuchHostedZone(
                            parsed_error.message,
                        ))
                    }
                    "PriorRequestNotComplete" => {
                        return RusotoError::Service(
                            ListTagsForResourceError::PriorRequestNotComplete(parsed_error.message),
                        )
                    }
                    "ThrottlingException" => {
                        return RusotoError::Service(ListTagsForResourceError::Throttling(
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
            ListTagsForResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NoSuchHealthCheck(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::PriorRequestNotComplete(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTagsForResources
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourcesError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the specified ID.</p>
    NoSuchHealthCheck(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl ListTagsForResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourcesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(ListTagsForResourcesError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHealthCheck" => {
                        return RusotoError::Service(ListTagsForResourcesError::NoSuchHealthCheck(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(ListTagsForResourcesError::NoSuchHostedZone(
                            parsed_error.message,
                        ))
                    }
                    "PriorRequestNotComplete" => {
                        return RusotoError::Service(
                            ListTagsForResourcesError::PriorRequestNotComplete(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ThrottlingException" => {
                        return RusotoError::Service(ListTagsForResourcesError::Throttling(
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
impl fmt::Display for ListTagsForResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourcesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListTagsForResourcesError::NoSuchHealthCheck(ref cause) => write!(f, "{}", cause),
            ListTagsForResourcesError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
            ListTagsForResourcesError::PriorRequestNotComplete(ref cause) => write!(f, "{}", cause),
            ListTagsForResourcesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourcesError {}
/// Errors returned by ListTrafficPolicies
#[derive(Debug, PartialEq)]
pub enum ListTrafficPoliciesError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
}

impl ListTrafficPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTrafficPoliciesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(ListTrafficPoliciesError::InvalidInput(
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
impl fmt::Display for ListTrafficPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTrafficPoliciesError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTrafficPoliciesError {}
/// Errors returned by ListTrafficPolicyInstances
#[derive(Debug, PartialEq)]
pub enum ListTrafficPolicyInstancesError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy instance exists with the specified ID.</p>
    NoSuchTrafficPolicyInstance(String),
}

impl ListTrafficPolicyInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListTrafficPolicyInstancesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(ListTrafficPolicyInstancesError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchTrafficPolicyInstance" => {
                        return RusotoError::Service(
                            ListTrafficPolicyInstancesError::NoSuchTrafficPolicyInstance(
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
impl fmt::Display for ListTrafficPolicyInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTrafficPolicyInstancesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListTrafficPolicyInstancesError::NoSuchTrafficPolicyInstance(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListTrafficPolicyInstancesError {}
/// Errors returned by ListTrafficPolicyInstancesByHostedZone
#[derive(Debug, PartialEq)]
pub enum ListTrafficPolicyInstancesByHostedZoneError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
    /// <p>No traffic policy instance exists with the specified ID.</p>
    NoSuchTrafficPolicyInstance(String),
}

impl ListTrafficPolicyInstancesByHostedZoneError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListTrafficPolicyInstancesByHostedZoneError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(
                            ListTrafficPolicyInstancesByHostedZoneError::InvalidInput(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(
                            ListTrafficPolicyInstancesByHostedZoneError::NoSuchHostedZone(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchTrafficPolicyInstance" => return RusotoError::Service(
                        ListTrafficPolicyInstancesByHostedZoneError::NoSuchTrafficPolicyInstance(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListTrafficPolicyInstancesByHostedZoneError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTrafficPolicyInstancesByHostedZoneError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            ListTrafficPolicyInstancesByHostedZoneError::NoSuchHostedZone(ref cause) => {
                write!(f, "{}", cause)
            }
            ListTrafficPolicyInstancesByHostedZoneError::NoSuchTrafficPolicyInstance(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListTrafficPolicyInstancesByHostedZoneError {}
/// Errors returned by ListTrafficPolicyInstancesByPolicy
#[derive(Debug, PartialEq)]
pub enum ListTrafficPolicyInstancesByPolicyError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy exists with the specified ID.</p>
    NoSuchTrafficPolicy(String),
    /// <p>No traffic policy instance exists with the specified ID.</p>
    NoSuchTrafficPolicyInstance(String),
}

impl ListTrafficPolicyInstancesByPolicyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListTrafficPolicyInstancesByPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(
                            ListTrafficPolicyInstancesByPolicyError::InvalidInput(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchTrafficPolicy" => {
                        return RusotoError::Service(
                            ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchTrafficPolicyInstance" => {
                        return RusotoError::Service(
                            ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicyInstance(
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
impl fmt::Display for ListTrafficPolicyInstancesByPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTrafficPolicyInstancesByPolicyError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicyInstance(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListTrafficPolicyInstancesByPolicyError {}
/// Errors returned by ListTrafficPolicyVersions
#[derive(Debug, PartialEq)]
pub enum ListTrafficPolicyVersionsError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy exists with the specified ID.</p>
    NoSuchTrafficPolicy(String),
}

impl ListTrafficPolicyVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTrafficPolicyVersionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(ListTrafficPolicyVersionsError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchTrafficPolicy" => {
                        return RusotoError::Service(
                            ListTrafficPolicyVersionsError::NoSuchTrafficPolicy(
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
impl fmt::Display for ListTrafficPolicyVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTrafficPolicyVersionsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListTrafficPolicyVersionsError::NoSuchTrafficPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListTrafficPolicyVersionsError {}
/// Errors returned by ListVPCAssociationAuthorizations
#[derive(Debug, PartialEq)]
pub enum ListVPCAssociationAuthorizationsError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>The value that you specified to get the second or subsequent page of results is invalid.</p>
    InvalidPaginationToken(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
}

impl ListVPCAssociationAuthorizationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListVPCAssociationAuthorizationsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(
                            ListVPCAssociationAuthorizationsError::InvalidInput(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidPaginationToken" => {
                        return RusotoError::Service(
                            ListVPCAssociationAuthorizationsError::InvalidPaginationToken(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(
                            ListVPCAssociationAuthorizationsError::NoSuchHostedZone(
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
impl fmt::Display for ListVPCAssociationAuthorizationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVPCAssociationAuthorizationsError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            ListVPCAssociationAuthorizationsError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListVPCAssociationAuthorizationsError::NoSuchHostedZone(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListVPCAssociationAuthorizationsError {}
/// Errors returned by TestDNSAnswer
#[derive(Debug, PartialEq)]
pub enum TestDNSAnswerError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
}

impl TestDNSAnswerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TestDNSAnswerError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(TestDNSAnswerError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(TestDNSAnswerError::NoSuchHostedZone(
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
impl fmt::Display for TestDNSAnswerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TestDNSAnswerError::InvalidInput(ref cause) => write!(f, "{}", cause),
            TestDNSAnswerError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TestDNSAnswerError {}
/// Errors returned by UpdateHealthCheck
#[derive(Debug, PartialEq)]
pub enum UpdateHealthCheckError {
    /// <p>The value of <code>HealthCheckVersion</code> in the request doesn't match the value of <code>HealthCheckVersion</code> in the health check.</p>
    HealthCheckVersionMismatch(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No health check exists with the specified ID.</p>
    NoSuchHealthCheck(String),
}

impl UpdateHealthCheckError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateHealthCheckError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "HealthCheckVersionMismatch" => {
                        return RusotoError::Service(
                            UpdateHealthCheckError::HealthCheckVersionMismatch(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(UpdateHealthCheckError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHealthCheck" => {
                        return RusotoError::Service(UpdateHealthCheckError::NoSuchHealthCheck(
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
impl fmt::Display for UpdateHealthCheckError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateHealthCheckError::HealthCheckVersionMismatch(ref cause) => write!(f, "{}", cause),
            UpdateHealthCheckError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateHealthCheckError::NoSuchHealthCheck(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateHealthCheckError {}
/// Errors returned by UpdateHostedZoneComment
#[derive(Debug, PartialEq)]
pub enum UpdateHostedZoneCommentError {
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No hosted zone exists with the ID that you specified.</p>
    NoSuchHostedZone(String),
}

impl UpdateHostedZoneCommentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateHostedZoneCommentError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidInput" => {
                        return RusotoError::Service(UpdateHostedZoneCommentError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchHostedZone" => {
                        return RusotoError::Service(
                            UpdateHostedZoneCommentError::NoSuchHostedZone(parsed_error.message),
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
impl fmt::Display for UpdateHostedZoneCommentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateHostedZoneCommentError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateHostedZoneCommentError::NoSuchHostedZone(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateHostedZoneCommentError {}
/// Errors returned by UpdateTrafficPolicyComment
#[derive(Debug, PartialEq)]
pub enum UpdateTrafficPolicyCommentError {
    /// <p>Another user submitted a request to create, update, or delete the object at the same time that you did. Retry the request. </p>
    ConcurrentModification(String),
    /// <p>The input is not valid.</p>
    InvalidInput(String),
    /// <p>No traffic policy exists with the specified ID.</p>
    NoSuchTrafficPolicy(String),
}

impl UpdateTrafficPolicyCommentError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateTrafficPolicyCommentError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConcurrentModification" => {
                        return RusotoError::Service(
                            UpdateTrafficPolicyCommentError::ConcurrentModification(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(UpdateTrafficPolicyCommentError::InvalidInput(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchTrafficPolicy" => {
                        return RusotoError::Service(
                            UpdateTrafficPolicyCommentError::NoSuchTrafficPolicy(
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
impl fmt::Display for UpdateTrafficPolicyCommentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTrafficPolicyCommentError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateTrafficPolicyCommentError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateTrafficPolicyCommentError::NoSuchTrafficPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateTrafficPolicyCommentError {}
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
    /// <p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
    PriorRequestNotComplete(String),
}

impl UpdateTrafficPolicyInstanceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateTrafficPolicyInstanceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConflictingTypes" => {
                        return RusotoError::Service(
                            UpdateTrafficPolicyInstanceError::ConflictingTypes(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidInput" => {
                        return RusotoError::Service(
                            UpdateTrafficPolicyInstanceError::InvalidInput(parsed_error.message),
                        )
                    }
                    "NoSuchTrafficPolicy" => {
                        return RusotoError::Service(
                            UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicy(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoSuchTrafficPolicyInstance" => {
                        return RusotoError::Service(
                            UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(
                                parsed_error.message,
                            ),
                        )
                    }
                    "PriorRequestNotComplete" => {
                        return RusotoError::Service(
                            UpdateTrafficPolicyInstanceError::PriorRequestNotComplete(
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
impl fmt::Display for UpdateTrafficPolicyInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTrafficPolicyInstanceError::ConflictingTypes(ref cause) => write!(f, "{}", cause),
            UpdateTrafficPolicyInstanceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateTrafficPolicyInstanceError::PriorRequestNotComplete(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateTrafficPolicyInstanceError {}
/// Trait representing the capabilities of the Route 53 API. Route 53 clients implement this trait.
#[async_trait]
pub trait Route53 {
    /// <p><p>Associates an Amazon VPC with a private hosted zone. </p> <important> <p>To perform the association, the VPC and the private hosted zone must already exist. You can&#39;t convert a public hosted zone into a private hosted zone.</p> </important> <note> <p>If you want to associate a VPC that was created by using one AWS account with a private hosted zone that was created by using a different account, the AWS account that created the private hosted zone must first submit a <code>CreateVPCAssociationAuthorization</code> request. Then the account that created the VPC must submit an <code>AssociateVPCWithHostedZone</code> request.</p> </note></p>
    async fn associate_vpc_with_hosted_zone(
        &self,
        input: AssociateVPCWithHostedZoneRequest,
    ) -> Result<AssociateVPCWithHostedZoneResponse, RusotoError<AssociateVPCWithHostedZoneError>>;

    /// For TXT records, see <a href="./util/fn.quote_txt_record.html">util::quote_txt_record</a>
    /// <p>Creates, changes, or deletes a resource record set, which contains authoritative DNS information for a specified domain name or subdomain name. For example, you can use <code>ChangeResourceRecordSets</code> to create a resource record set that routes traffic for test.example.com to a web server that has an IP address of 192.0.2.44.</p> <p> <b>Change Batches and Transactional Changes</b> </p> <p>The request body must include a document with a <code>ChangeResourceRecordSetsRequest</code> element. The request body contains a list of change items, known as a change batch. Change batches are considered transactional changes. When using the Amazon Route 53 API to change resource record sets, Route 53 either makes all or none of the changes in a change batch request. This ensures that Route 53 never partially implements the intended changes to the resource record sets in a hosted zone. </p> <p>For example, a change batch request that deletes the <code>CNAME</code> record for www.example.com and creates an alias resource record set for www.example.com. Route 53 deletes the first resource record set and creates the second resource record set in a single operation. If either the <code>DELETE</code> or the <code>CREATE</code> action fails, then both changes (plus any other changes in the batch) fail, and the original <code>CNAME</code> record continues to exist.</p> <important> <p>Due to the nature of transactional changes, you can't delete the same resource record set more than once in a single change batch. If you attempt to delete the same change batch more than once, Route 53 returns an <code>InvalidChangeBatch</code> error.</p> </important> <p> <b>Traffic Flow</b> </p> <p>To create resource record sets for complex routing configurations, use either the traffic flow visual editor in the Route 53 console or the API actions for traffic policies and traffic policy instances. Save the configuration as a traffic policy, then associate the traffic policy with one or more domain names (such as example.com) or subdomain names (such as www.example.com), in the same hosted zone or in multiple hosted zones. You can roll back the updates if the new configuration isn't performing as expected. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/traffic-flow.html">Using Traffic Flow to Route DNS Traffic</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p> <b>Create, Delete, and Upsert</b> </p> <p>Use <code>ChangeResourceRecordsSetsRequest</code> to perform the following actions:</p> <ul> <li> <p> <code>CREATE</code>: Creates a resource record set that has the specified values.</p> </li> <li> <p> <code>DELETE</code>: Deletes an existing resource record set that has the specified values.</p> </li> <li> <p> <code>UPSERT</code>: If a resource record set does not already exist, AWS creates it. If a resource set does exist, Route 53 updates it with the values in the request. </p> </li> </ul> <p> <b>Syntaxes for Creating, Updating, and Deleting Resource Record Sets</b> </p> <p>The syntax for a request depends on the type of resource record set that you want to create, delete, or update, such as weighted, alias, or failover. The XML elements in your request must appear in the order listed in the syntax. </p> <p>For an example for each type of resource record set, see "Examples."</p> <p>Don't refer to the syntax in the "Parameter Syntax" section, which includes all of the elements for every kind of resource record set that you can create, delete, or update by using <code>ChangeResourceRecordSets</code>. </p> <p> <b>Change Propagation to Route 53 DNS Servers</b> </p> <p>When you submit a <code>ChangeResourceRecordSets</code> request, Route 53 propagates your changes to all of the Route 53 authoritative DNS servers. While your changes are propagating, <code>GetChange</code> returns a status of <code>PENDING</code>. When propagation is complete, <code>GetChange</code> returns a status of <code>INSYNC</code>. Changes generally propagate to all Route 53 name servers within 60 seconds. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetChange.html">GetChange</a>.</p> <p> <b>Limits on ChangeResourceRecordSets Requests</b> </p> <p>For information about the limits on a <code>ChangeResourceRecordSets</code> request, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    async fn change_resource_record_sets(
        &self,
        input: ChangeResourceRecordSetsRequest,
    ) -> Result<ChangeResourceRecordSetsResponse, RusotoError<ChangeResourceRecordSetsError>>;

    /// <p>Adds, edits, or deletes tags for a health check or a hosted zone.</p> <p>For information about using tags for cost allocation, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    async fn change_tags_for_resource(
        &self,
        input: ChangeTagsForResourceRequest,
    ) -> Result<ChangeTagsForResourceResponse, RusotoError<ChangeTagsForResourceError>>;

    /// <p><p>Creates a new health check.</p> <p>For information about adding health checks to resource record sets, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ResourceRecordSet.html#Route53-Type-ResourceRecordSet-HealthCheckId">HealthCheckId</a> in <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ChangeResourceRecordSets.html">ChangeResourceRecordSets</a>. </p> <p> <b>ELB Load Balancers</b> </p> <p>If you&#39;re registering EC2 instances with an Elastic Load Balancing (ELB) load balancer, do not create Amazon Route 53 health checks for the EC2 instances. When you register an EC2 instance with a load balancer, you configure settings for an ELB health check, which performs a similar function to a Route 53 health check.</p> <p> <b>Private Hosted Zones</b> </p> <p>You can associate health checks with failover resource record sets in a private hosted zone. Note the following:</p> <ul> <li> <p>Route 53 health checkers are outside the VPC. To check the health of an endpoint within a VPC by IP address, you must assign a public IP address to the instance in the VPC.</p> </li> <li> <p>You can configure a health checker to check the health of an external resource that the instance relies on, such as a database server.</p> </li> <li> <p>You can create a CloudWatch metric, associate an alarm with the metric, and then create a health check that is based on the state of the alarm. For example, you might create a CloudWatch metric that checks the status of the Amazon EC2 <code>StatusCheckFailed</code> metric, add an alarm to the metric, and then create a health check that is based on the state of the alarm. For information about creating CloudWatch metrics and alarms by using the CloudWatch console, see the <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/WhatIsCloudWatch.html">Amazon CloudWatch User Guide</a>.</p> </li> </ul></p>
    async fn create_health_check(
        &self,
        input: CreateHealthCheckRequest,
    ) -> Result<CreateHealthCheckResponse, RusotoError<CreateHealthCheckError>>;

    /// <p>Creates a new public or private hosted zone. You create records in a public hosted zone to define how you want to route traffic on the internet for a domain, such as example.com, and its subdomains (apex.example.com, acme.example.com). You create records in a private hosted zone to define how you want to route traffic for a domain and its subdomains within one or more Amazon Virtual Private Clouds (Amazon VPCs). </p> <important> <p>You can't convert a public hosted zone to a private hosted zone or vice versa. Instead, you must create a new hosted zone with the same name and create new resource record sets.</p> </important> <p>For more information about charges for hosted zones, see <a href="http://aws.amazon.com/route53/pricing/">Amazon Route 53 Pricing</a>.</p> <p>Note the following:</p> <ul> <li> <p>You can't create a hosted zone for a top-level domain (TLD) such as .com.</p> </li> <li> <p>For public hosted zones, Amazon Route 53 automatically creates a default SOA record and four NS records for the zone. For more information about SOA and NS records, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/SOA-NSrecords.html">NS and SOA Records that Route 53 Creates for a Hosted Zone</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If you want to use the same name servers for multiple public hosted zones, you can optionally associate a reusable delegation set with the hosted zone. See the <code>DelegationSetId</code> element.</p> </li> <li> <p>If your domain is registered with a registrar other than Route 53, you must update the name servers with your registrar to make Route 53 the DNS service for the domain. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/MigratingDNS.html">Migrating DNS Service for an Existing Domain to Amazon Route 53</a> in the <i>Amazon Route 53 Developer Guide</i>. </p> </li> </ul> <p>When you submit a <code>CreateHostedZone</code> request, the initial status of the hosted zone is <code>PENDING</code>. For public hosted zones, this means that the NS and SOA records are not yet available on all Route 53 DNS servers. When the NS and SOA records are available, the status of the zone changes to <code>INSYNC</code>.</p>
    async fn create_hosted_zone(
        &self,
        input: CreateHostedZoneRequest,
    ) -> Result<CreateHostedZoneResponse, RusotoError<CreateHostedZoneError>>;

    /// <p><p>Creates a configuration for DNS query logging. After you create a query logging configuration, Amazon Route 53 begins to publish log data to an Amazon CloudWatch Logs log group.</p> <p>DNS query logs contain information about the queries that Route 53 receives for a specified public hosted zone, such as the following:</p> <ul> <li> <p>Route 53 edge location that responded to the DNS query</p> </li> <li> <p>Domain or subdomain that was requested</p> </li> <li> <p>DNS record type, such as A or AAAA</p> </li> <li> <p>DNS response code, such as <code>NoError</code> or <code>ServFail</code> </p> </li> </ul> <dl> <dt>Log Group and Resource Policy</dt> <dd> <p>Before you create a query logging configuration, perform the following operations.</p> <note> <p>If you create a query logging configuration using the Route 53 console, Route 53 performs these operations automatically.</p> </note> <ol> <li> <p>Create a CloudWatch Logs log group, and make note of the ARN, which you specify when you create a query logging configuration. Note the following:</p> <ul> <li> <p>You must create the log group in the us-east-1 region.</p> </li> <li> <p>You must use the same AWS account to create the log group and the hosted zone that you want to configure query logging for.</p> </li> <li> <p>When you create log groups for query logging, we recommend that you use a consistent prefix, for example:</p> <p> <code>/aws/route53/<i>hosted zone name</i> </code> </p> <p>In the next step, you&#39;ll create a resource policy, which controls access to one or more log groups and the associated AWS resources, such as Route 53 hosted zones. There&#39;s a limit on the number of resource policies that you can create, so we recommend that you use a consistent prefix so you can use the same resource policy for all the log groups that you create for query logging.</p> </li> </ul> </li> <li> <p>Create a CloudWatch Logs resource policy, and give it the permissions that Route 53 needs to create log streams and to send query logs to log streams. For the value of <code>Resource</code>, specify the ARN for the log group that you created in the previous step. To use the same resource policy for all the CloudWatch Logs log groups that you created for query logging configurations, replace the hosted zone name with <code><em></code>, for example:</p> <p> <code>arn:aws:logs:us-east-1:123412341234:log-group:/aws/route53/</em></code> </p> <note> <p>You can&#39;t use the CloudWatch console to create or edit a resource policy. You must use the CloudWatch API, one of the AWS SDKs, or the AWS CLI.</p> </note> </li> </ol> </dd> <dt>Log Streams and Edge Locations</dt> <dd> <p>When Route 53 finishes creating the configuration for DNS query logging, it does the following:</p> <ul> <li> <p>Creates a log stream for an edge location the first time that the edge location responds to DNS queries for the specified hosted zone. That log stream is used to log all queries that Route 53 responds to for that edge location.</p> </li> <li> <p>Begins to send query logs to the applicable log stream.</p> </li> </ul> <p>The name of each log stream is in the following format:</p> <p> <code> <i>hosted zone ID</i>/<i>edge location code</i> </code> </p> <p>The edge location code is a three-letter code and an arbitrarily assigned number, for example, DFW3. The three-letter code typically corresponds with the International Air Transport Association airport code for an airport near the edge location. (These abbreviations might change in the future.) For a list of edge locations, see &quot;The Route 53 Global Network&quot; on the <a href="http://aws.amazon.com/route53/details/">Route 53 Product Details</a> page.</p> </dd> <dt>Queries That Are Logged</dt> <dd> <p>Query logs contain only the queries that DNS resolvers forward to Route 53. If a DNS resolver has already cached the response to a query (such as the IP address for a load balancer for example.com), the resolver will continue to return the cached response. It doesn&#39;t forward another query to Route 53 until the TTL for the corresponding resource record set expires. Depending on how many DNS queries are submitted for a resource record set, and depending on the TTL for that resource record set, query logs might contain information about only one query out of every several thousand queries that are submitted to DNS. For more information about how DNS works, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/welcome-dns-service.html">Routing Internet Traffic to Your Website or Web Application</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </dd> <dt>Log File Format</dt> <dd> <p>For a list of the values in each query log and the format of each value, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html">Logging DNS Queries</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </dd> <dt>Pricing</dt> <dd> <p>For information about charges for query logs, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </dd> <dt>How to Stop Logging</dt> <dd> <p>If you want Route 53 to stop sending query logs to CloudWatch Logs, delete the query logging configuration. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_DeleteQueryLoggingConfig.html">DeleteQueryLoggingConfig</a>.</p> </dd> </dl></p>
    async fn create_query_logging_config(
        &self,
        input: CreateQueryLoggingConfigRequest,
    ) -> Result<CreateQueryLoggingConfigResponse, RusotoError<CreateQueryLoggingConfigError>>;

    /// <p><p>Creates a delegation set (a group of four name servers) that can be reused by multiple hosted zones. If a hosted zoned ID is specified, <code>CreateReusableDelegationSet</code> marks the delegation set associated with that zone as reusable.</p> <note> <p>You can&#39;t associate a reusable delegation set with a private hosted zone.</p> </note> <p>For information about using a reusable delegation set to configure white label name servers, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/white-label-name-servers.html">Configuring White Label Name Servers</a>.</p> <p>The process for migrating existing hosted zones to use a reusable delegation set is comparable to the process for configuring white label name servers. You need to perform the following steps:</p> <ol> <li> <p>Create a reusable delegation set.</p> </li> <li> <p>Recreate hosted zones, and reduce the TTL to 60 seconds or less.</p> </li> <li> <p>Recreate resource record sets in the new hosted zones.</p> </li> <li> <p>Change the registrar&#39;s name servers to use the name servers for the new hosted zones.</p> </li> <li> <p>Monitor traffic for the website or application.</p> </li> <li> <p>Change TTLs back to their original values.</p> </li> </ol> <p>If you want to migrate existing hosted zones to use a reusable delegation set, the existing hosted zones can&#39;t use any of the name servers that are assigned to the reusable delegation set. If one or more hosted zones do use one or more name servers that are assigned to the reusable delegation set, you can do one of the following:</p> <ul> <li> <p>For small numbers of hosted zones—up to a few hundred—it&#39;s relatively easy to create reusable delegation sets until you get one that has four name servers that don&#39;t overlap with any of the name servers in your hosted zones.</p> </li> <li> <p>For larger numbers of hosted zones, the easiest solution is to use more than one reusable delegation set.</p> </li> <li> <p>For larger numbers of hosted zones, you can also migrate hosted zones that have overlapping name servers to hosted zones that don&#39;t have overlapping name servers, then migrate the hosted zones again to use the reusable delegation set.</p> </li> </ul></p>
    async fn create_reusable_delegation_set(
        &self,
        input: CreateReusableDelegationSetRequest,
    ) -> Result<CreateReusableDelegationSetResponse, RusotoError<CreateReusableDelegationSetError>>;

    /// <p>Creates a traffic policy, which you use to create multiple DNS resource record sets for one domain name (such as example.com) or one subdomain name (such as www.example.com).</p>
    async fn create_traffic_policy(
        &self,
        input: CreateTrafficPolicyRequest,
    ) -> Result<CreateTrafficPolicyResponse, RusotoError<CreateTrafficPolicyError>>;

    /// <p>Creates resource record sets in a specified hosted zone based on the settings in a specified traffic policy version. In addition, <code>CreateTrafficPolicyInstance</code> associates the resource record sets with a specified domain name (such as example.com) or subdomain name (such as www.example.com). Amazon Route 53 responds to DNS queries for the domain or subdomain name by using the resource record sets that <code>CreateTrafficPolicyInstance</code> created.</p>
    async fn create_traffic_policy_instance(
        &self,
        input: CreateTrafficPolicyInstanceRequest,
    ) -> Result<CreateTrafficPolicyInstanceResponse, RusotoError<CreateTrafficPolicyInstanceError>>;

    /// <p>Creates a new version of an existing traffic policy. When you create a new version of a traffic policy, you specify the ID of the traffic policy that you want to update and a JSON-formatted document that describes the new version. You use traffic policies to create multiple DNS resource record sets for one domain name (such as example.com) or one subdomain name (such as www.example.com). You can create a maximum of 1000 versions of a traffic policy. If you reach the limit and need to create another version, you'll need to start a new traffic policy.</p>
    async fn create_traffic_policy_version(
        &self,
        input: CreateTrafficPolicyVersionRequest,
    ) -> Result<CreateTrafficPolicyVersionResponse, RusotoError<CreateTrafficPolicyVersionError>>;

    /// <p><p>Authorizes the AWS account that created a specified VPC to submit an <code>AssociateVPCWithHostedZone</code> request to associate the VPC with a specified hosted zone that was created by a different account. To submit a <code>CreateVPCAssociationAuthorization</code> request, you must use the account that created the hosted zone. After you authorize the association, use the account that created the VPC to submit an <code>AssociateVPCWithHostedZone</code> request.</p> <note> <p>If you want to associate multiple VPCs that you created by using one account with a hosted zone that you created by using a different account, you must submit one authorization request for each VPC.</p> </note></p>
    async fn create_vpc_association_authorization(
        &self,
        input: CreateVPCAssociationAuthorizationRequest,
    ) -> Result<
        CreateVPCAssociationAuthorizationResponse,
        RusotoError<CreateVPCAssociationAuthorizationError>,
    >;

    /// <p><p>Deletes a health check.</p> <important> <p>Amazon Route 53 does not prevent you from deleting a health check even if the health check is associated with one or more resource record sets. If you delete a health check and you don&#39;t update the associated resource record sets, the future status of the health check can&#39;t be predicted and may change. This will affect the routing of DNS queries for your DNS failover configuration. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/health-checks-creating-deleting.html#health-checks-deleting.html">Replacing and Deleting Health Checks</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </important></p>
    async fn delete_health_check(
        &self,
        input: DeleteHealthCheckRequest,
    ) -> Result<DeleteHealthCheckResponse, RusotoError<DeleteHealthCheckError>>;

    /// <p><p>Deletes a hosted zone.</p> <p>If the hosted zone was created by another service, such as AWS Cloud Map, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DeleteHostedZone.html#delete-public-hosted-zone-created-by-another-service">Deleting Public Hosted Zones That Were Created by Another Service</a> in the <i>Amazon Route 53 Developer Guide</i> for information about how to delete it. (The process is the same for public and private hosted zones that were created by another service.)</p> <p>If you want to keep your domain registration but you want to stop routing internet traffic to your website or web application, we recommend that you delete resource record sets in the hosted zone instead of deleting the hosted zone.</p> <important> <p>If you delete a hosted zone, you can&#39;t undelete it. You must create a new hosted zone and update the name servers for your domain registration, which can require up to 48 hours to take effect. (If you delegated responsibility for a subdomain to a hosted zone and you delete the child hosted zone, you must update the name servers in the parent hosted zone.) In addition, if you delete a hosted zone, someone could hijack the domain and route traffic to their own resources using your domain name.</p> </important> <p>If you want to avoid the monthly charge for the hosted zone, you can transfer DNS service for the domain to a free DNS service. When you transfer DNS service, you have to update the name servers for the domain registration. If the domain is registered with Route 53, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_UpdateDomainNameservers.html">UpdateDomainNameservers</a> for information about how to replace Route 53 name servers with name servers for the new DNS service. If the domain is registered with another registrar, use the method provided by the registrar to update name servers for the domain registration. For more information, perform an internet search on &quot;free DNS service.&quot;</p> <p>You can delete a hosted zone only if it contains only the default SOA record and NS resource record sets. If the hosted zone contains other resource record sets, you must delete them before you can delete the hosted zone. If you try to delete a hosted zone that contains other resource record sets, the request fails, and Route 53 returns a <code>HostedZoneNotEmpty</code> error. For information about deleting records from your hosted zone, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ChangeResourceRecordSets.html">ChangeResourceRecordSets</a>.</p> <p>To verify that the hosted zone has been deleted, do one of the following:</p> <ul> <li> <p>Use the <code>GetHostedZone</code> action to request information about the hosted zone.</p> </li> <li> <p>Use the <code>ListHostedZones</code> action to get a list of the hosted zones associated with the current AWS account.</p> </li> </ul></p>
    async fn delete_hosted_zone(
        &self,
        input: DeleteHostedZoneRequest,
    ) -> Result<DeleteHostedZoneResponse, RusotoError<DeleteHostedZoneError>>;

    /// <p>Deletes a configuration for DNS query logging. If you delete a configuration, Amazon Route 53 stops sending query logs to CloudWatch Logs. Route 53 doesn't delete any logs that are already in CloudWatch Logs.</p> <p>For more information about DNS query logs, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateQueryLoggingConfig.html">CreateQueryLoggingConfig</a>.</p>
    async fn delete_query_logging_config(
        &self,
        input: DeleteQueryLoggingConfigRequest,
    ) -> Result<DeleteQueryLoggingConfigResponse, RusotoError<DeleteQueryLoggingConfigError>>;

    /// <p>Deletes a reusable delegation set.</p> <important> <p>You can delete a reusable delegation set only if it isn't associated with any hosted zones.</p> </important> <p>To verify that the reusable delegation set is not associated with any hosted zones, submit a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetReusableDelegationSet.html">GetReusableDelegationSet</a> request and specify the ID of the reusable delegation set that you want to delete.</p>
    async fn delete_reusable_delegation_set(
        &self,
        input: DeleteReusableDelegationSetRequest,
    ) -> Result<DeleteReusableDelegationSetResponse, RusotoError<DeleteReusableDelegationSetError>>;

    /// <p>Deletes a traffic policy.</p>
    async fn delete_traffic_policy(
        &self,
        input: DeleteTrafficPolicyRequest,
    ) -> Result<DeleteTrafficPolicyResponse, RusotoError<DeleteTrafficPolicyError>>;

    /// <p><p>Deletes a traffic policy instance and all of the resource record sets that Amazon Route 53 created when you created the instance.</p> <note> <p>In the Route 53 console, traffic policy instances are known as policy records.</p> </note></p>
    async fn delete_traffic_policy_instance(
        &self,
        input: DeleteTrafficPolicyInstanceRequest,
    ) -> Result<DeleteTrafficPolicyInstanceResponse, RusotoError<DeleteTrafficPolicyInstanceError>>;

    /// <p><p>Removes authorization to submit an <code>AssociateVPCWithHostedZone</code> request to associate a specified VPC with a hosted zone that was created by a different account. You must use the account that created the hosted zone to submit a <code>DeleteVPCAssociationAuthorization</code> request.</p> <important> <p>Sending this request only prevents the AWS account that created the VPC from associating the VPC with the Amazon Route 53 hosted zone in the future. If the VPC is already associated with the hosted zone, <code>DeleteVPCAssociationAuthorization</code> won&#39;t disassociate the VPC from the hosted zone. If you want to delete an existing association, use <code>DisassociateVPCFromHostedZone</code>.</p> </important></p>
    async fn delete_vpc_association_authorization(
        &self,
        input: DeleteVPCAssociationAuthorizationRequest,
    ) -> Result<
        DeleteVPCAssociationAuthorizationResponse,
        RusotoError<DeleteVPCAssociationAuthorizationError>,
    >;

    /// <p><p>Disassociates a VPC from a Amazon Route 53 private hosted zone. Note the following:</p> <ul> <li> <p>You can&#39;t disassociate the last VPC from a private hosted zone.</p> </li> <li> <p>You can&#39;t convert a private hosted zone into a public hosted zone.</p> </li> <li> <p>You can submit a <code>DisassociateVPCFromHostedZone</code> request using either the account that created the hosted zone or the account that created the VPC.</p> </li> </ul></p>
    async fn disassociate_vpc_from_hosted_zone(
        &self,
        input: DisassociateVPCFromHostedZoneRequest,
    ) -> Result<
        DisassociateVPCFromHostedZoneResponse,
        RusotoError<DisassociateVPCFromHostedZoneError>,
    >;

    /// <p><p>Gets the specified limit for the current account, for example, the maximum number of health checks that you can create using the account.</p> <p>For the default limit, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>. To request a higher limit, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-route53">open a case</a>.</p> <note> <p>You can also view account limits in AWS Trusted Advisor. Sign in to the AWS Management Console and open the Trusted Advisor console at <a href="https://console.aws.amazon.com/trustedadvisor">https://console.aws.amazon.com/trustedadvisor/</a>. Then choose <b>Service limits</b> in the navigation pane.</p> </note></p>
    async fn get_account_limit(
        &self,
        input: GetAccountLimitRequest,
    ) -> Result<GetAccountLimitResponse, RusotoError<GetAccountLimitError>>;

    /// <p><p>Returns the current status of a change batch request. The status is one of the following values:</p> <ul> <li> <p> <code>PENDING</code> indicates that the changes in this request have not propagated to all Amazon Route 53 DNS servers. This is the initial status of all change batch requests.</p> </li> <li> <p> <code>INSYNC</code> indicates that the changes have propagated to all Route 53 DNS servers. </p> </li> </ul></p>
    async fn get_change(
        &self,
        input: GetChangeRequest,
    ) -> Result<GetChangeResponse, RusotoError<GetChangeError>>;

    /// <p><important> <p> <code>GetCheckerIpRanges</code> still works, but we recommend that you download ip-ranges.json, which includes IP address ranges for all AWS services. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/route-53-ip-addresses.html">IP Address Ranges of Amazon Route 53 Servers</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </important></p>
    async fn get_checker_ip_ranges(
        &self,
        input: GetCheckerIpRangesRequest,
    ) -> Result<GetCheckerIpRangesResponse, RusotoError<GetCheckerIpRangesError>>;

    /// <p>Gets information about whether a specified geographic location is supported for Amazon Route 53 geolocation resource record sets.</p> <p>Use the following syntax to determine whether a continent is supported for geolocation:</p> <p> <code>GET /2013-04-01/geolocation?continentcode=<i>two-letter abbreviation for a continent</i> </code> </p> <p>Use the following syntax to determine whether a country is supported for geolocation:</p> <p> <code>GET /2013-04-01/geolocation?countrycode=<i>two-character country code</i> </code> </p> <p>Use the following syntax to determine whether a subdivision of a country is supported for geolocation:</p> <p> <code>GET /2013-04-01/geolocation?countrycode=<i>two-character country code</i>&amp;subdivisioncode=<i>subdivision code</i> </code> </p>
    async fn get_geo_location(
        &self,
        input: GetGeoLocationRequest,
    ) -> Result<GetGeoLocationResponse, RusotoError<GetGeoLocationError>>;

    /// <p>Gets information about a specified health check.</p>
    async fn get_health_check(
        &self,
        input: GetHealthCheckRequest,
    ) -> Result<GetHealthCheckResponse, RusotoError<GetHealthCheckError>>;

    /// <p>Retrieves the number of health checks that are associated with the current AWS account.</p>
    async fn get_health_check_count(
        &self,
        input: GetHealthCheckCountRequest,
    ) -> Result<GetHealthCheckCountResponse, RusotoError<GetHealthCheckCountError>>;

    /// <p>Gets the reason that a specified health check failed most recently.</p>
    async fn get_health_check_last_failure_reason(
        &self,
        input: GetHealthCheckLastFailureReasonRequest,
    ) -> Result<
        GetHealthCheckLastFailureReasonResponse,
        RusotoError<GetHealthCheckLastFailureReasonError>,
    >;

    /// <p>Gets status of a specified health check. </p>
    async fn get_health_check_status(
        &self,
        input: GetHealthCheckStatusRequest,
    ) -> Result<GetHealthCheckStatusResponse, RusotoError<GetHealthCheckStatusError>>;

    /// <p>Gets information about a specified hosted zone including the four name servers assigned to the hosted zone.</p>
    async fn get_hosted_zone(
        &self,
        input: GetHostedZoneRequest,
    ) -> Result<GetHostedZoneResponse, RusotoError<GetHostedZoneError>>;

    /// <p>Retrieves the number of hosted zones that are associated with the current AWS account.</p>
    async fn get_hosted_zone_count(
        &self,
        input: GetHostedZoneCountRequest,
    ) -> Result<GetHostedZoneCountResponse, RusotoError<GetHostedZoneCountError>>;

    /// <p>Gets the specified limit for a specified hosted zone, for example, the maximum number of records that you can create in the hosted zone. </p> <p>For the default limit, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>. To request a higher limit, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-route53">open a case</a>.</p>
    async fn get_hosted_zone_limit(
        &self,
        input: GetHostedZoneLimitRequest,
    ) -> Result<GetHostedZoneLimitResponse, RusotoError<GetHostedZoneLimitError>>;

    /// <p>Gets information about a specified configuration for DNS query logging.</p> <p>For more information about DNS query logs, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateQueryLoggingConfig.html">CreateQueryLoggingConfig</a> and <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html">Logging DNS Queries</a>.</p>
    async fn get_query_logging_config(
        &self,
        input: GetQueryLoggingConfigRequest,
    ) -> Result<GetQueryLoggingConfigResponse, RusotoError<GetQueryLoggingConfigError>>;

    /// <p>Retrieves information about a specified reusable delegation set, including the four name servers that are assigned to the delegation set.</p>
    async fn get_reusable_delegation_set(
        &self,
        input: GetReusableDelegationSetRequest,
    ) -> Result<GetReusableDelegationSetResponse, RusotoError<GetReusableDelegationSetError>>;

    /// <p>Gets the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p> <p>For the default limit, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>. To request a higher limit, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-route53">open a case</a>.</p>
    async fn get_reusable_delegation_set_limit(
        &self,
        input: GetReusableDelegationSetLimitRequest,
    ) -> Result<
        GetReusableDelegationSetLimitResponse,
        RusotoError<GetReusableDelegationSetLimitError>,
    >;

    /// <p>Gets information about a specific traffic policy version.</p>
    async fn get_traffic_policy(
        &self,
        input: GetTrafficPolicyRequest,
    ) -> Result<GetTrafficPolicyResponse, RusotoError<GetTrafficPolicyError>>;

    /// <p><p>Gets information about a specified traffic policy instance.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there&#39;s a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <note> <p>In the Route 53 console, traffic policy instances are known as policy records.</p> </note></p>
    async fn get_traffic_policy_instance(
        &self,
        input: GetTrafficPolicyInstanceRequest,
    ) -> Result<GetTrafficPolicyInstanceResponse, RusotoError<GetTrafficPolicyInstanceError>>;

    /// <p>Gets the number of traffic policy instances that are associated with the current AWS account.</p>
    async fn get_traffic_policy_instance_count(
        &self,
        input: GetTrafficPolicyInstanceCountRequest,
    ) -> Result<
        GetTrafficPolicyInstanceCountResponse,
        RusotoError<GetTrafficPolicyInstanceCountError>,
    >;

    /// <p>Retrieves a list of supported geographic locations.</p> <p>Countries are listed first, and continents are listed last. If Amazon Route 53 supports subdivisions for a country (for example, states or provinces), the subdivisions for that country are listed in alphabetical order immediately after the corresponding country.</p>
    async fn list_geo_locations(
        &self,
        input: ListGeoLocationsRequest,
    ) -> Result<ListGeoLocationsResponse, RusotoError<ListGeoLocationsError>>;

    /// <p>Retrieve a list of the health checks that are associated with the current AWS account. </p>
    async fn list_health_checks(
        &self,
        input: ListHealthChecksRequest,
    ) -> Result<ListHealthChecksResponse, RusotoError<ListHealthChecksError>>;

    /// <p>Retrieves a list of the public and private hosted zones that are associated with the current AWS account. The response includes a <code>HostedZones</code> child element for each hosted zone.</p> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of hosted zones, you can use the <code>maxitems</code> parameter to list them in groups of up to 100.</p>
    async fn list_hosted_zones(
        &self,
        input: ListHostedZonesRequest,
    ) -> Result<ListHostedZonesResponse, RusotoError<ListHostedZonesError>>;

    /// <p><p>Retrieves a list of your hosted zones in lexicographic order. The response includes a <code>HostedZones</code> child element for each hosted zone created by the current AWS account. </p> <p> <code>ListHostedZonesByName</code> sorts hosted zones by name with the labels reversed. For example:</p> <p> <code>com.example.www.</code> </p> <p>Note the trailing dot, which can change the sort order in some circumstances.</p> <p>If the domain name includes escape characters or Punycode, <code>ListHostedZonesByName</code> alphabetizes the domain name using the escaped or Punycoded value, which is the format that Amazon Route 53 saves in its database. For example, to create a hosted zone for exämple.com, you specify ex\344mple.com for the domain name. <code>ListHostedZonesByName</code> alphabetizes it as:</p> <p> <code>com.ex\344mple.</code> </p> <p>The labels are reversed and alphabetized using the escaped value. For more information about valid domain name formats, including internationalized domain names, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DomainNameFormat.html">DNS Domain Name Format</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>Route 53 returns up to 100 items in each response. If you have a lot of hosted zones, use the <code>MaxItems</code> parameter to list them in groups of up to 100. The response includes values that help navigate from one group of <code>MaxItems</code> hosted zones to the next:</p> <ul> <li> <p>The <code>DNSName</code> and <code>HostedZoneId</code> elements in the response contain the values, if any, specified for the <code>dnsname</code> and <code>hostedzoneid</code> parameters in the request that produced the current response.</p> </li> <li> <p>The <code>MaxItems</code> element in the response contains the value, if any, that you specified for the <code>maxitems</code> parameter in the request that produced the current response.</p> </li> <li> <p>If the value of <code>IsTruncated</code> in the response is true, there are more hosted zones associated with the current AWS account. </p> <p>If <code>IsTruncated</code> is false, this response includes the last hosted zone that is associated with the current account. The <code>NextDNSName</code> element and <code>NextHostedZoneId</code> elements are omitted from the response.</p> </li> <li> <p>The <code>NextDNSName</code> and <code>NextHostedZoneId</code> elements in the response contain the domain name and the hosted zone ID of the next hosted zone that is associated with the current AWS account. If you want to list more hosted zones, make another call to <code>ListHostedZonesByName</code>, and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p> </li> </ul></p>
    async fn list_hosted_zones_by_name(
        &self,
        input: ListHostedZonesByNameRequest,
    ) -> Result<ListHostedZonesByNameResponse, RusotoError<ListHostedZonesByNameError>>;

    /// <p>Lists the configurations for DNS query logging that are associated with the current AWS account or the configuration that is associated with a specified hosted zone.</p> <p>For more information about DNS query logs, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateQueryLoggingConfig.html">CreateQueryLoggingConfig</a>. Additional information, including the format of DNS query logs, appears in <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html">Logging DNS Queries</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    async fn list_query_logging_configs(
        &self,
        input: ListQueryLoggingConfigsRequest,
    ) -> Result<ListQueryLoggingConfigsResponse, RusotoError<ListQueryLoggingConfigsError>>;

    /// <p>Lists the resource record sets in a specified hosted zone.</p> <p> <code>ListResourceRecordSets</code> returns up to 100 resource record sets at a time in ASCII order, beginning at a position specified by the <code>name</code> and <code>type</code> elements.</p> <p> <b>Sort order</b> </p> <p> <code>ListResourceRecordSets</code> sorts results first by DNS name with the labels reversed, for example:</p> <p> <code>com.example.www.</code> </p> <p>Note the trailing dot, which can change the sort order when the record name contains characters that appear before <code>.</code> (decimal 46) in the ASCII table. These characters include the following: <code>! " # $ % &amp; ' ( ) * + , -</code> </p> <p>When multiple records have the same DNS name, <code>ListResourceRecordSets</code> sorts results by the record type.</p> <p> <b>Specifying where to start listing records</b> </p> <p>You can use the name and type elements to specify the resource record set that the list begins with:</p> <dl> <dt>If you do not specify Name or Type</dt> <dd> <p>The results begin with the first resource record set that the hosted zone contains.</p> </dd> <dt>If you specify Name but not Type</dt> <dd> <p>The results begin with the first resource record set in the list whose name is greater than or equal to <code>Name</code>.</p> </dd> <dt>If you specify Type but not Name</dt> <dd> <p>Amazon Route 53 returns the <code>InvalidInput</code> error.</p> </dd> <dt>If you specify both Name and Type</dt> <dd> <p>The results begin with the first resource record set in the list whose name is greater than or equal to <code>Name</code>, and whose type is greater than or equal to <code>Type</code>.</p> </dd> </dl> <p> <b>Resource record sets that are PENDING</b> </p> <p>This action returns the most current version of the records. This includes records that are <code>PENDING</code>, and that are not yet available on all Route 53 DNS servers.</p> <p> <b>Changing resource record sets</b> </p> <p>To ensure that you get an accurate listing of the resource record sets for a hosted zone at a point in time, do not submit a <code>ChangeResourceRecordSets</code> request while you're paging through the results of a <code>ListResourceRecordSets</code> request. If you do, some pages may display results without the latest changes while other pages display results with the latest changes.</p> <p> <b>Displaying the next page of results</b> </p> <p>If a <code>ListResourceRecordSets</code> command returns more than one page of results, the value of <code>IsTruncated</code> is <code>true</code>. To display the next page of results, get the values of <code>NextRecordName</code>, <code>NextRecordType</code>, and <code>NextRecordIdentifier</code> (if any) from the response. Then submit another <code>ListResourceRecordSets</code> request, and specify those values for <code>StartRecordName</code>, <code>StartRecordType</code>, and <code>StartRecordIdentifier</code>.</p>
    async fn list_resource_record_sets(
        &self,
        input: ListResourceRecordSetsRequest,
    ) -> Result<ListResourceRecordSetsResponse, RusotoError<ListResourceRecordSetsError>>;

    /// <p>Retrieves a list of the reusable delegation sets that are associated with the current AWS account.</p>
    async fn list_reusable_delegation_sets(
        &self,
        input: ListReusableDelegationSetsRequest,
    ) -> Result<ListReusableDelegationSetsResponse, RusotoError<ListReusableDelegationSetsError>>;

    /// <p>Lists tags for one health check or hosted zone. </p> <p>For information about using tags for cost allocation, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists tags for up to 10 health checks or hosted zones.</p> <p>For information about using tags for cost allocation, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    async fn list_tags_for_resources(
        &self,
        input: ListTagsForResourcesRequest,
    ) -> Result<ListTagsForResourcesResponse, RusotoError<ListTagsForResourcesError>>;

    /// <p>Gets information about the latest version for every traffic policy that is associated with the current AWS account. Policies are listed in the order that they were created in. </p>
    async fn list_traffic_policies(
        &self,
        input: ListTrafficPoliciesRequest,
    ) -> Result<ListTrafficPoliciesResponse, RusotoError<ListTrafficPoliciesError>>;

    /// <p>Gets information about the traffic policy instances that you created by using the current AWS account.</p> <note> <p>After you submit an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p>
    async fn list_traffic_policy_instances(
        &self,
        input: ListTrafficPolicyInstancesRequest,
    ) -> Result<ListTrafficPolicyInstancesResponse, RusotoError<ListTrafficPolicyInstancesError>>;

    /// <p>Gets information about the traffic policy instances that you created in a specified hosted zone.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p>
    async fn list_traffic_policy_instances_by_hosted_zone(
        &self,
        input: ListTrafficPolicyInstancesByHostedZoneRequest,
    ) -> Result<
        ListTrafficPolicyInstancesByHostedZoneResponse,
        RusotoError<ListTrafficPolicyInstancesByHostedZoneError>,
    >;

    /// <p>Gets information about the traffic policy instances that you created by using a specify traffic policy version.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p>
    async fn list_traffic_policy_instances_by_policy(
        &self,
        input: ListTrafficPolicyInstancesByPolicyRequest,
    ) -> Result<
        ListTrafficPolicyInstancesByPolicyResponse,
        RusotoError<ListTrafficPolicyInstancesByPolicyError>,
    >;

    /// <p>Gets information about all of the versions for a specified traffic policy.</p> <p>Traffic policy versions are listed in numerical order by <code>VersionNumber</code>.</p>
    async fn list_traffic_policy_versions(
        &self,
        input: ListTrafficPolicyVersionsRequest,
    ) -> Result<ListTrafficPolicyVersionsResponse, RusotoError<ListTrafficPolicyVersionsError>>;

    /// <p>Gets a list of the VPCs that were created by other accounts and that can be associated with a specified hosted zone because you've submitted one or more <code>CreateVPCAssociationAuthorization</code> requests. </p> <p>The response includes a <code>VPCs</code> element with a <code>VPC</code> child element for each VPC that can be associated with the hosted zone.</p>
    async fn list_vpc_association_authorizations(
        &self,
        input: ListVPCAssociationAuthorizationsRequest,
    ) -> Result<
        ListVPCAssociationAuthorizationsResponse,
        RusotoError<ListVPCAssociationAuthorizationsError>,
    >;

    /// <p>Gets the value that Amazon Route 53 returns in response to a DNS request for a specified record name and type. You can optionally specify the IP address of a DNS resolver, an EDNS0 client subnet IP address, and a subnet mask. </p>
    async fn test_dns_answer(
        &self,
        input: TestDNSAnswerRequest,
    ) -> Result<TestDNSAnswerResponse, RusotoError<TestDNSAnswerError>>;

    /// <p>Updates an existing health check. Note that some values can't be updated. </p> <p>For more information about updating health checks, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/health-checks-creating-deleting.html">Creating, Updating, and Deleting Health Checks</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    async fn update_health_check(
        &self,
        input: UpdateHealthCheckRequest,
    ) -> Result<UpdateHealthCheckResponse, RusotoError<UpdateHealthCheckError>>;

    /// <p>Updates the comment for a specified hosted zone.</p>
    async fn update_hosted_zone_comment(
        &self,
        input: UpdateHostedZoneCommentRequest,
    ) -> Result<UpdateHostedZoneCommentResponse, RusotoError<UpdateHostedZoneCommentError>>;

    /// <p>Updates the comment for a specified traffic policy version.</p>
    async fn update_traffic_policy_comment(
        &self,
        input: UpdateTrafficPolicyCommentRequest,
    ) -> Result<UpdateTrafficPolicyCommentResponse, RusotoError<UpdateTrafficPolicyCommentError>>;

    /// <p><p>Updates the resource record sets in a specified hosted zone that were created based on the settings in a specified traffic policy version.</p> <p>When you update a traffic policy instance, Amazon Route 53 continues to respond to DNS queries for the root resource record set name (such as example.com) while it replaces one group of resource record sets with another. Route 53 performs the following operations:</p> <ol> <li> <p>Route 53 creates a new group of resource record sets based on the specified traffic policy. This is true regardless of how significant the differences are between the existing resource record sets and the new resource record sets. </p> </li> <li> <p>When all of the new resource record sets have been created, Route 53 starts to respond to DNS queries for the root resource record set name (such as example.com) by using the new resource record sets.</p> </li> <li> <p>Route 53 deletes the old group of resource record sets that are associated with the root resource record set name.</p> </li> </ol></p>
    async fn update_traffic_policy_instance(
        &self,
        input: UpdateTrafficPolicyInstanceRequest,
    ) -> Result<UpdateTrafficPolicyInstanceResponse, RusotoError<UpdateTrafficPolicyInstanceError>>;
}
/// A client for the Route 53 API.
#[derive(Clone)]
pub struct Route53Client {
    client: Client,
    region: region::Region,
}

impl Route53Client {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> Route53Client {
        Route53Client {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> Route53Client
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        Route53Client {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> Route53Client {
        Route53Client { client, region }
    }
}

#[async_trait]
impl Route53 for Route53Client {
    /// <p><p>Associates an Amazon VPC with a private hosted zone. </p> <important> <p>To perform the association, the VPC and the private hosted zone must already exist. You can&#39;t convert a public hosted zone into a private hosted zone.</p> </important> <note> <p>If you want to associate a VPC that was created by using one AWS account with a private hosted zone that was created by using a different account, the AWS account that created the private hosted zone must first submit a <code>CreateVPCAssociationAuthorization</code> request. Then the account that created the VPC must submit an <code>AssociateVPCWithHostedZone</code> request.</p> </note></p>
    #[allow(unused_variables, warnings)]
    async fn associate_vpc_with_hosted_zone(
        &self,
        input: AssociateVPCWithHostedZoneRequest,
    ) -> Result<AssociateVPCWithHostedZoneResponse, RusotoError<AssociateVPCWithHostedZoneError>>
    {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/associatevpc",
            id = input.hosted_zone_id
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        AssociateVPCWithHostedZoneRequestSerializer::serialize(
            &mut writer,
            "AssociateVPCWithHostedZoneRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(AssociateVPCWithHostedZoneError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = AssociateVPCWithHostedZoneResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = AssociateVPCWithHostedZoneResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// For TXT records, see <a href="./util/fn.quote_txt_record.html">util::quote_txt_record</a>
    /// <p>Creates, changes, or deletes a resource record set, which contains authoritative DNS information for a specified domain name or subdomain name. For example, you can use <code>ChangeResourceRecordSets</code> to create a resource record set that routes traffic for test.example.com to a web server that has an IP address of 192.0.2.44.</p> <p> <b>Change Batches and Transactional Changes</b> </p> <p>The request body must include a document with a <code>ChangeResourceRecordSetsRequest</code> element. The request body contains a list of change items, known as a change batch. Change batches are considered transactional changes. When using the Amazon Route 53 API to change resource record sets, Route 53 either makes all or none of the changes in a change batch request. This ensures that Route 53 never partially implements the intended changes to the resource record sets in a hosted zone. </p> <p>For example, a change batch request that deletes the <code>CNAME</code> record for www.example.com and creates an alias resource record set for www.example.com. Route 53 deletes the first resource record set and creates the second resource record set in a single operation. If either the <code>DELETE</code> or the <code>CREATE</code> action fails, then both changes (plus any other changes in the batch) fail, and the original <code>CNAME</code> record continues to exist.</p> <important> <p>Due to the nature of transactional changes, you can't delete the same resource record set more than once in a single change batch. If you attempt to delete the same change batch more than once, Route 53 returns an <code>InvalidChangeBatch</code> error.</p> </important> <p> <b>Traffic Flow</b> </p> <p>To create resource record sets for complex routing configurations, use either the traffic flow visual editor in the Route 53 console or the API actions for traffic policies and traffic policy instances. Save the configuration as a traffic policy, then associate the traffic policy with one or more domain names (such as example.com) or subdomain names (such as www.example.com), in the same hosted zone or in multiple hosted zones. You can roll back the updates if the new configuration isn't performing as expected. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/traffic-flow.html">Using Traffic Flow to Route DNS Traffic</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p> <b>Create, Delete, and Upsert</b> </p> <p>Use <code>ChangeResourceRecordsSetsRequest</code> to perform the following actions:</p> <ul> <li> <p> <code>CREATE</code>: Creates a resource record set that has the specified values.</p> </li> <li> <p> <code>DELETE</code>: Deletes an existing resource record set that has the specified values.</p> </li> <li> <p> <code>UPSERT</code>: If a resource record set does not already exist, AWS creates it. If a resource set does exist, Route 53 updates it with the values in the request. </p> </li> </ul> <p> <b>Syntaxes for Creating, Updating, and Deleting Resource Record Sets</b> </p> <p>The syntax for a request depends on the type of resource record set that you want to create, delete, or update, such as weighted, alias, or failover. The XML elements in your request must appear in the order listed in the syntax. </p> <p>For an example for each type of resource record set, see "Examples."</p> <p>Don't refer to the syntax in the "Parameter Syntax" section, which includes all of the elements for every kind of resource record set that you can create, delete, or update by using <code>ChangeResourceRecordSets</code>. </p> <p> <b>Change Propagation to Route 53 DNS Servers</b> </p> <p>When you submit a <code>ChangeResourceRecordSets</code> request, Route 53 propagates your changes to all of the Route 53 authoritative DNS servers. While your changes are propagating, <code>GetChange</code> returns a status of <code>PENDING</code>. When propagation is complete, <code>GetChange</code> returns a status of <code>INSYNC</code>. Changes generally propagate to all Route 53 name servers within 60 seconds. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetChange.html">GetChange</a>.</p> <p> <b>Limits on ChangeResourceRecordSets Requests</b> </p> <p>For information about the limits on a <code>ChangeResourceRecordSets</code> request, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn change_resource_record_sets(
        &self,
        input: ChangeResourceRecordSetsRequest,
    ) -> Result<ChangeResourceRecordSetsResponse, RusotoError<ChangeResourceRecordSetsError>> {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/rrset/",
            id = input.hosted_zone_id
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        ChangeResourceRecordSetsRequestSerializer::serialize(
            &mut writer,
            "ChangeResourceRecordSetsRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ChangeResourceRecordSetsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ChangeResourceRecordSetsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ChangeResourceRecordSetsResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Adds, edits, or deletes tags for a health check or a hosted zone.</p> <p>For information about using tags for cost allocation, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn change_tags_for_resource(
        &self,
        input: ChangeTagsForResourceRequest,
    ) -> Result<ChangeTagsForResourceResponse, RusotoError<ChangeTagsForResourceError>> {
        let request_uri = format!(
            "/2013-04-01/tags/{resource_type}/{resource_id}",
            resource_id = input.resource_id,
            resource_type = input.resource_type
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        ChangeTagsForResourceRequestSerializer::serialize(
            &mut writer,
            "ChangeTagsForResourceRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ChangeTagsForResourceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ChangeTagsForResourceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ChangeTagsForResourceResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Creates a new health check.</p> <p>For information about adding health checks to resource record sets, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ResourceRecordSet.html#Route53-Type-ResourceRecordSet-HealthCheckId">HealthCheckId</a> in <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ChangeResourceRecordSets.html">ChangeResourceRecordSets</a>. </p> <p> <b>ELB Load Balancers</b> </p> <p>If you&#39;re registering EC2 instances with an Elastic Load Balancing (ELB) load balancer, do not create Amazon Route 53 health checks for the EC2 instances. When you register an EC2 instance with a load balancer, you configure settings for an ELB health check, which performs a similar function to a Route 53 health check.</p> <p> <b>Private Hosted Zones</b> </p> <p>You can associate health checks with failover resource record sets in a private hosted zone. Note the following:</p> <ul> <li> <p>Route 53 health checkers are outside the VPC. To check the health of an endpoint within a VPC by IP address, you must assign a public IP address to the instance in the VPC.</p> </li> <li> <p>You can configure a health checker to check the health of an external resource that the instance relies on, such as a database server.</p> </li> <li> <p>You can create a CloudWatch metric, associate an alarm with the metric, and then create a health check that is based on the state of the alarm. For example, you might create a CloudWatch metric that checks the status of the Amazon EC2 <code>StatusCheckFailed</code> metric, add an alarm to the metric, and then create a health check that is based on the state of the alarm. For information about creating CloudWatch metrics and alarms by using the CloudWatch console, see the <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/WhatIsCloudWatch.html">Amazon CloudWatch User Guide</a>.</p> </li> </ul></p>
    #[allow(unused_variables, warnings)]
    async fn create_health_check(
        &self,
        input: CreateHealthCheckRequest,
    ) -> Result<CreateHealthCheckResponse, RusotoError<CreateHealthCheckError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateHealthCheckError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateHealthCheckResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                CreateHealthCheckResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        let value = response.headers.get("Location").unwrap().to_owned();
        result.location = value; // parse non-payload
        Ok(result)
    }

    /// <p>Creates a new public or private hosted zone. You create records in a public hosted zone to define how you want to route traffic on the internet for a domain, such as example.com, and its subdomains (apex.example.com, acme.example.com). You create records in a private hosted zone to define how you want to route traffic for a domain and its subdomains within one or more Amazon Virtual Private Clouds (Amazon VPCs). </p> <important> <p>You can't convert a public hosted zone to a private hosted zone or vice versa. Instead, you must create a new hosted zone with the same name and create new resource record sets.</p> </important> <p>For more information about charges for hosted zones, see <a href="http://aws.amazon.com/route53/pricing/">Amazon Route 53 Pricing</a>.</p> <p>Note the following:</p> <ul> <li> <p>You can't create a hosted zone for a top-level domain (TLD) such as .com.</p> </li> <li> <p>For public hosted zones, Amazon Route 53 automatically creates a default SOA record and four NS records for the zone. For more information about SOA and NS records, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/SOA-NSrecords.html">NS and SOA Records that Route 53 Creates for a Hosted Zone</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>If you want to use the same name servers for multiple public hosted zones, you can optionally associate a reusable delegation set with the hosted zone. See the <code>DelegationSetId</code> element.</p> </li> <li> <p>If your domain is registered with a registrar other than Route 53, you must update the name servers with your registrar to make Route 53 the DNS service for the domain. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/MigratingDNS.html">Migrating DNS Service for an Existing Domain to Amazon Route 53</a> in the <i>Amazon Route 53 Developer Guide</i>. </p> </li> </ul> <p>When you submit a <code>CreateHostedZone</code> request, the initial status of the hosted zone is <code>PENDING</code>. For public hosted zones, this means that the NS and SOA records are not yet available on all Route 53 DNS servers. When the NS and SOA records are available, the status of the zone changes to <code>INSYNC</code>.</p>
    #[allow(unused_variables, warnings)]
    async fn create_hosted_zone(
        &self,
        input: CreateHostedZoneRequest,
    ) -> Result<CreateHostedZoneResponse, RusotoError<CreateHostedZoneError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateHostedZoneError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateHostedZoneResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                CreateHostedZoneResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        let value = response.headers.get("Location").unwrap().to_owned();
        result.location = value; // parse non-payload
        Ok(result)
    }

    /// <p><p>Creates a configuration for DNS query logging. After you create a query logging configuration, Amazon Route 53 begins to publish log data to an Amazon CloudWatch Logs log group.</p> <p>DNS query logs contain information about the queries that Route 53 receives for a specified public hosted zone, such as the following:</p> <ul> <li> <p>Route 53 edge location that responded to the DNS query</p> </li> <li> <p>Domain or subdomain that was requested</p> </li> <li> <p>DNS record type, such as A or AAAA</p> </li> <li> <p>DNS response code, such as <code>NoError</code> or <code>ServFail</code> </p> </li> </ul> <dl> <dt>Log Group and Resource Policy</dt> <dd> <p>Before you create a query logging configuration, perform the following operations.</p> <note> <p>If you create a query logging configuration using the Route 53 console, Route 53 performs these operations automatically.</p> </note> <ol> <li> <p>Create a CloudWatch Logs log group, and make note of the ARN, which you specify when you create a query logging configuration. Note the following:</p> <ul> <li> <p>You must create the log group in the us-east-1 region.</p> </li> <li> <p>You must use the same AWS account to create the log group and the hosted zone that you want to configure query logging for.</p> </li> <li> <p>When you create log groups for query logging, we recommend that you use a consistent prefix, for example:</p> <p> <code>/aws/route53/<i>hosted zone name</i> </code> </p> <p>In the next step, you&#39;ll create a resource policy, which controls access to one or more log groups and the associated AWS resources, such as Route 53 hosted zones. There&#39;s a limit on the number of resource policies that you can create, so we recommend that you use a consistent prefix so you can use the same resource policy for all the log groups that you create for query logging.</p> </li> </ul> </li> <li> <p>Create a CloudWatch Logs resource policy, and give it the permissions that Route 53 needs to create log streams and to send query logs to log streams. For the value of <code>Resource</code>, specify the ARN for the log group that you created in the previous step. To use the same resource policy for all the CloudWatch Logs log groups that you created for query logging configurations, replace the hosted zone name with <code><em></code>, for example:</p> <p> <code>arn:aws:logs:us-east-1:123412341234:log-group:/aws/route53/</em></code> </p> <note> <p>You can&#39;t use the CloudWatch console to create or edit a resource policy. You must use the CloudWatch API, one of the AWS SDKs, or the AWS CLI.</p> </note> </li> </ol> </dd> <dt>Log Streams and Edge Locations</dt> <dd> <p>When Route 53 finishes creating the configuration for DNS query logging, it does the following:</p> <ul> <li> <p>Creates a log stream for an edge location the first time that the edge location responds to DNS queries for the specified hosted zone. That log stream is used to log all queries that Route 53 responds to for that edge location.</p> </li> <li> <p>Begins to send query logs to the applicable log stream.</p> </li> </ul> <p>The name of each log stream is in the following format:</p> <p> <code> <i>hosted zone ID</i>/<i>edge location code</i> </code> </p> <p>The edge location code is a three-letter code and an arbitrarily assigned number, for example, DFW3. The three-letter code typically corresponds with the International Air Transport Association airport code for an airport near the edge location. (These abbreviations might change in the future.) For a list of edge locations, see &quot;The Route 53 Global Network&quot; on the <a href="http://aws.amazon.com/route53/details/">Route 53 Product Details</a> page.</p> </dd> <dt>Queries That Are Logged</dt> <dd> <p>Query logs contain only the queries that DNS resolvers forward to Route 53. If a DNS resolver has already cached the response to a query (such as the IP address for a load balancer for example.com), the resolver will continue to return the cached response. It doesn&#39;t forward another query to Route 53 until the TTL for the corresponding resource record set expires. Depending on how many DNS queries are submitted for a resource record set, and depending on the TTL for that resource record set, query logs might contain information about only one query out of every several thousand queries that are submitted to DNS. For more information about how DNS works, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/welcome-dns-service.html">Routing Internet Traffic to Your Website or Web Application</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </dd> <dt>Log File Format</dt> <dd> <p>For a list of the values in each query log and the format of each value, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html">Logging DNS Queries</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </dd> <dt>Pricing</dt> <dd> <p>For information about charges for query logs, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </dd> <dt>How to Stop Logging</dt> <dd> <p>If you want Route 53 to stop sending query logs to CloudWatch Logs, delete the query logging configuration. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_DeleteQueryLoggingConfig.html">DeleteQueryLoggingConfig</a>.</p> </dd> </dl></p>
    #[allow(unused_variables, warnings)]
    async fn create_query_logging_config(
        &self,
        input: CreateQueryLoggingConfigRequest,
    ) -> Result<CreateQueryLoggingConfigResponse, RusotoError<CreateQueryLoggingConfigError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateQueryLoggingConfigError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateQueryLoggingConfigResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = CreateQueryLoggingConfigResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        let value = response.headers.get("Location").unwrap().to_owned();
        result.location = value; // parse non-payload
        Ok(result)
    }

    /// <p><p>Creates a delegation set (a group of four name servers) that can be reused by multiple hosted zones. If a hosted zoned ID is specified, <code>CreateReusableDelegationSet</code> marks the delegation set associated with that zone as reusable.</p> <note> <p>You can&#39;t associate a reusable delegation set with a private hosted zone.</p> </note> <p>For information about using a reusable delegation set to configure white label name servers, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/white-label-name-servers.html">Configuring White Label Name Servers</a>.</p> <p>The process for migrating existing hosted zones to use a reusable delegation set is comparable to the process for configuring white label name servers. You need to perform the following steps:</p> <ol> <li> <p>Create a reusable delegation set.</p> </li> <li> <p>Recreate hosted zones, and reduce the TTL to 60 seconds or less.</p> </li> <li> <p>Recreate resource record sets in the new hosted zones.</p> </li> <li> <p>Change the registrar&#39;s name servers to use the name servers for the new hosted zones.</p> </li> <li> <p>Monitor traffic for the website or application.</p> </li> <li> <p>Change TTLs back to their original values.</p> </li> </ol> <p>If you want to migrate existing hosted zones to use a reusable delegation set, the existing hosted zones can&#39;t use any of the name servers that are assigned to the reusable delegation set. If one or more hosted zones do use one or more name servers that are assigned to the reusable delegation set, you can do one of the following:</p> <ul> <li> <p>For small numbers of hosted zones—up to a few hundred—it&#39;s relatively easy to create reusable delegation sets until you get one that has four name servers that don&#39;t overlap with any of the name servers in your hosted zones.</p> </li> <li> <p>For larger numbers of hosted zones, the easiest solution is to use more than one reusable delegation set.</p> </li> <li> <p>For larger numbers of hosted zones, you can also migrate hosted zones that have overlapping name servers to hosted zones that don&#39;t have overlapping name servers, then migrate the hosted zones again to use the reusable delegation set.</p> </li> </ul></p>
    #[allow(unused_variables, warnings)]
    async fn create_reusable_delegation_set(
        &self,
        input: CreateReusableDelegationSetRequest,
    ) -> Result<CreateReusableDelegationSetResponse, RusotoError<CreateReusableDelegationSetError>>
    {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateReusableDelegationSetError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateReusableDelegationSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = CreateReusableDelegationSetResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        let value = response.headers.get("Location").unwrap().to_owned();
        result.location = value; // parse non-payload
        Ok(result)
    }

    /// <p>Creates a traffic policy, which you use to create multiple DNS resource record sets for one domain name (such as example.com) or one subdomain name (such as www.example.com).</p>
    #[allow(unused_variables, warnings)]
    async fn create_traffic_policy(
        &self,
        input: CreateTrafficPolicyRequest,
    ) -> Result<CreateTrafficPolicyResponse, RusotoError<CreateTrafficPolicyError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateTrafficPolicyError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateTrafficPolicyResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                CreateTrafficPolicyResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        let value = response.headers.get("Location").unwrap().to_owned();
        result.location = value; // parse non-payload
        Ok(result)
    }

    /// <p>Creates resource record sets in a specified hosted zone based on the settings in a specified traffic policy version. In addition, <code>CreateTrafficPolicyInstance</code> associates the resource record sets with a specified domain name (such as example.com) or subdomain name (such as www.example.com). Amazon Route 53 responds to DNS queries for the domain or subdomain name by using the resource record sets that <code>CreateTrafficPolicyInstance</code> created.</p>
    #[allow(unused_variables, warnings)]
    async fn create_traffic_policy_instance(
        &self,
        input: CreateTrafficPolicyInstanceRequest,
    ) -> Result<CreateTrafficPolicyInstanceResponse, RusotoError<CreateTrafficPolicyInstanceError>>
    {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateTrafficPolicyInstanceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateTrafficPolicyInstanceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = CreateTrafficPolicyInstanceResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        let value = response.headers.get("Location").unwrap().to_owned();
        result.location = value; // parse non-payload
        Ok(result)
    }

    /// <p>Creates a new version of an existing traffic policy. When you create a new version of a traffic policy, you specify the ID of the traffic policy that you want to update and a JSON-formatted document that describes the new version. You use traffic policies to create multiple DNS resource record sets for one domain name (such as example.com) or one subdomain name (such as www.example.com). You can create a maximum of 1000 versions of a traffic policy. If you reach the limit and need to create another version, you'll need to start a new traffic policy.</p>
    #[allow(unused_variables, warnings)]
    async fn create_traffic_policy_version(
        &self,
        input: CreateTrafficPolicyVersionRequest,
    ) -> Result<CreateTrafficPolicyVersionResponse, RusotoError<CreateTrafficPolicyVersionError>>
    {
        let request_uri = format!("/2013-04-01/trafficpolicy/{id}", id = input.id)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CreateTrafficPolicyVersionRequestSerializer::serialize(
            &mut writer,
            "CreateTrafficPolicyVersionRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateTrafficPolicyVersionError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateTrafficPolicyVersionResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = CreateTrafficPolicyVersionResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        let value = response.headers.get("Location").unwrap().to_owned();
        result.location = value; // parse non-payload
        Ok(result)
    }

    /// <p><p>Authorizes the AWS account that created a specified VPC to submit an <code>AssociateVPCWithHostedZone</code> request to associate the VPC with a specified hosted zone that was created by a different account. To submit a <code>CreateVPCAssociationAuthorization</code> request, you must use the account that created the hosted zone. After you authorize the association, use the account that created the VPC to submit an <code>AssociateVPCWithHostedZone</code> request.</p> <note> <p>If you want to associate multiple VPCs that you created by using one account with a hosted zone that you created by using a different account, you must submit one authorization request for each VPC.</p> </note></p>
    #[allow(unused_variables, warnings)]
    async fn create_vpc_association_authorization(
        &self,
        input: CreateVPCAssociationAuthorizationRequest,
    ) -> Result<
        CreateVPCAssociationAuthorizationResponse,
        RusotoError<CreateVPCAssociationAuthorizationError>,
    > {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/authorizevpcassociation",
            id = input.hosted_zone_id
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        CreateVPCAssociationAuthorizationRequestSerializer::serialize(
            &mut writer,
            "CreateVPCAssociationAuthorizationRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateVPCAssociationAuthorizationError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = CreateVPCAssociationAuthorizationResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = CreateVPCAssociationAuthorizationResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Deletes a health check.</p> <important> <p>Amazon Route 53 does not prevent you from deleting a health check even if the health check is associated with one or more resource record sets. If you delete a health check and you don&#39;t update the associated resource record sets, the future status of the health check can&#39;t be predicted and may change. This will affect the routing of DNS queries for your DNS failover configuration. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/health-checks-creating-deleting.html#health-checks-deleting.html">Replacing and Deleting Health Checks</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </important></p>
    #[allow(unused_variables, warnings)]
    async fn delete_health_check(
        &self,
        input: DeleteHealthCheckRequest,
    ) -> Result<DeleteHealthCheckResponse, RusotoError<DeleteHealthCheckError>> {
        let request_uri = format!(
            "/2013-04-01/healthcheck/{health_check_id}",
            health_check_id = input.health_check_id
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("DELETE", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteHealthCheckError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = DeleteHealthCheckResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                DeleteHealthCheckResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Deletes a hosted zone.</p> <p>If the hosted zone was created by another service, such as AWS Cloud Map, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DeleteHostedZone.html#delete-public-hosted-zone-created-by-another-service">Deleting Public Hosted Zones That Were Created by Another Service</a> in the <i>Amazon Route 53 Developer Guide</i> for information about how to delete it. (The process is the same for public and private hosted zones that were created by another service.)</p> <p>If you want to keep your domain registration but you want to stop routing internet traffic to your website or web application, we recommend that you delete resource record sets in the hosted zone instead of deleting the hosted zone.</p> <important> <p>If you delete a hosted zone, you can&#39;t undelete it. You must create a new hosted zone and update the name servers for your domain registration, which can require up to 48 hours to take effect. (If you delegated responsibility for a subdomain to a hosted zone and you delete the child hosted zone, you must update the name servers in the parent hosted zone.) In addition, if you delete a hosted zone, someone could hijack the domain and route traffic to their own resources using your domain name.</p> </important> <p>If you want to avoid the monthly charge for the hosted zone, you can transfer DNS service for the domain to a free DNS service. When you transfer DNS service, you have to update the name servers for the domain registration. If the domain is registered with Route 53, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_UpdateDomainNameservers.html">UpdateDomainNameservers</a> for information about how to replace Route 53 name servers with name servers for the new DNS service. If the domain is registered with another registrar, use the method provided by the registrar to update name servers for the domain registration. For more information, perform an internet search on &quot;free DNS service.&quot;</p> <p>You can delete a hosted zone only if it contains only the default SOA record and NS resource record sets. If the hosted zone contains other resource record sets, you must delete them before you can delete the hosted zone. If you try to delete a hosted zone that contains other resource record sets, the request fails, and Route 53 returns a <code>HostedZoneNotEmpty</code> error. For information about deleting records from your hosted zone, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_ChangeResourceRecordSets.html">ChangeResourceRecordSets</a>.</p> <p>To verify that the hosted zone has been deleted, do one of the following:</p> <ul> <li> <p>Use the <code>GetHostedZone</code> action to request information about the hosted zone.</p> </li> <li> <p>Use the <code>ListHostedZones</code> action to get a list of the hosted zones associated with the current AWS account.</p> </li> </ul></p>
    #[allow(unused_variables, warnings)]
    async fn delete_hosted_zone(
        &self,
        input: DeleteHostedZoneRequest,
    ) -> Result<DeleteHostedZoneResponse, RusotoError<DeleteHostedZoneError>> {
        let request_uri = format!("/2013-04-01/hostedzone/{id}", id = input.id)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("DELETE", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteHostedZoneError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = DeleteHostedZoneResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                DeleteHostedZoneResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes a configuration for DNS query logging. If you delete a configuration, Amazon Route 53 stops sending query logs to CloudWatch Logs. Route 53 doesn't delete any logs that are already in CloudWatch Logs.</p> <p>For more information about DNS query logs, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateQueryLoggingConfig.html">CreateQueryLoggingConfig</a>.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_query_logging_config(
        &self,
        input: DeleteQueryLoggingConfigRequest,
    ) -> Result<DeleteQueryLoggingConfigResponse, RusotoError<DeleteQueryLoggingConfigError>> {
        let request_uri = format!("/2013-04-01/queryloggingconfig/{id}", id = input.id)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("DELETE", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteQueryLoggingConfigError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = DeleteQueryLoggingConfigResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = DeleteQueryLoggingConfigResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes a reusable delegation set.</p> <important> <p>You can delete a reusable delegation set only if it isn't associated with any hosted zones.</p> </important> <p>To verify that the reusable delegation set is not associated with any hosted zones, submit a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetReusableDelegationSet.html">GetReusableDelegationSet</a> request and specify the ID of the reusable delegation set that you want to delete.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_reusable_delegation_set(
        &self,
        input: DeleteReusableDelegationSetRequest,
    ) -> Result<DeleteReusableDelegationSetResponse, RusotoError<DeleteReusableDelegationSetError>>
    {
        let request_uri = format!("/2013-04-01/delegationset/{id}", id = input.id)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("DELETE", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteReusableDelegationSetError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = DeleteReusableDelegationSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = DeleteReusableDelegationSetResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Deletes a traffic policy.</p>
    #[allow(unused_variables, warnings)]
    async fn delete_traffic_policy(
        &self,
        input: DeleteTrafficPolicyRequest,
    ) -> Result<DeleteTrafficPolicyResponse, RusotoError<DeleteTrafficPolicyError>> {
        let request_uri = format!(
            "/2013-04-01/trafficpolicy/{id}/{version}",
            id = input.id,
            version = input.version
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("DELETE", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteTrafficPolicyError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = DeleteTrafficPolicyResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                DeleteTrafficPolicyResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Deletes a traffic policy instance and all of the resource record sets that Amazon Route 53 created when you created the instance.</p> <note> <p>In the Route 53 console, traffic policy instances are known as policy records.</p> </note></p>
    #[allow(unused_variables, warnings)]
    async fn delete_traffic_policy_instance(
        &self,
        input: DeleteTrafficPolicyInstanceRequest,
    ) -> Result<DeleteTrafficPolicyInstanceResponse, RusotoError<DeleteTrafficPolicyInstanceError>>
    {
        let request_uri = format!("/2013-04-01/trafficpolicyinstance/{id}", id = input.id)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("DELETE", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteTrafficPolicyInstanceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = DeleteTrafficPolicyInstanceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = DeleteTrafficPolicyInstanceResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Removes authorization to submit an <code>AssociateVPCWithHostedZone</code> request to associate a specified VPC with a hosted zone that was created by a different account. You must use the account that created the hosted zone to submit a <code>DeleteVPCAssociationAuthorization</code> request.</p> <important> <p>Sending this request only prevents the AWS account that created the VPC from associating the VPC with the Amazon Route 53 hosted zone in the future. If the VPC is already associated with the hosted zone, <code>DeleteVPCAssociationAuthorization</code> won&#39;t disassociate the VPC from the hosted zone. If you want to delete an existing association, use <code>DisassociateVPCFromHostedZone</code>.</p> </important></p>
    #[allow(unused_variables, warnings)]
    async fn delete_vpc_association_authorization(
        &self,
        input: DeleteVPCAssociationAuthorizationRequest,
    ) -> Result<
        DeleteVPCAssociationAuthorizationResponse,
        RusotoError<DeleteVPCAssociationAuthorizationError>,
    > {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/deauthorizevpcassociation",
            id = input.hosted_zone_id
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        DeleteVPCAssociationAuthorizationRequestSerializer::serialize(
            &mut writer,
            "DeleteVPCAssociationAuthorizationRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteVPCAssociationAuthorizationError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = DeleteVPCAssociationAuthorizationResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = DeleteVPCAssociationAuthorizationResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Disassociates a VPC from a Amazon Route 53 private hosted zone. Note the following:</p> <ul> <li> <p>You can&#39;t disassociate the last VPC from a private hosted zone.</p> </li> <li> <p>You can&#39;t convert a private hosted zone into a public hosted zone.</p> </li> <li> <p>You can submit a <code>DisassociateVPCFromHostedZone</code> request using either the account that created the hosted zone or the account that created the VPC.</p> </li> </ul></p>
    #[allow(unused_variables, warnings)]
    async fn disassociate_vpc_from_hosted_zone(
        &self,
        input: DisassociateVPCFromHostedZoneRequest,
    ) -> Result<
        DisassociateVPCFromHostedZoneResponse,
        RusotoError<DisassociateVPCFromHostedZoneError>,
    > {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/disassociatevpc",
            id = input.hosted_zone_id
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        DisassociateVPCFromHostedZoneRequestSerializer::serialize(
            &mut writer,
            "DisassociateVPCFromHostedZoneRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DisassociateVPCFromHostedZoneError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = DisassociateVPCFromHostedZoneResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = DisassociateVPCFromHostedZoneResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Gets the specified limit for the current account, for example, the maximum number of health checks that you can create using the account.</p> <p>For the default limit, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>. To request a higher limit, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-route53">open a case</a>.</p> <note> <p>You can also view account limits in AWS Trusted Advisor. Sign in to the AWS Management Console and open the Trusted Advisor console at <a href="https://console.aws.amazon.com/trustedadvisor">https://console.aws.amazon.com/trustedadvisor/</a>. Then choose <b>Service limits</b> in the navigation pane.</p> </note></p>
    #[allow(unused_variables, warnings)]
    async fn get_account_limit(
        &self,
        input: GetAccountLimitRequest,
    ) -> Result<GetAccountLimitResponse, RusotoError<GetAccountLimitError>> {
        let request_uri = format!("/2013-04-01/accountlimit/{type}", type = input.type_)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetAccountLimitError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetAccountLimitResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                GetAccountLimitResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Returns the current status of a change batch request. The status is one of the following values:</p> <ul> <li> <p> <code>PENDING</code> indicates that the changes in this request have not propagated to all Amazon Route 53 DNS servers. This is the initial status of all change batch requests.</p> </li> <li> <p> <code>INSYNC</code> indicates that the changes have propagated to all Route 53 DNS servers. </p> </li> </ul></p>
    #[allow(unused_variables, warnings)]
    async fn get_change(
        &self,
        input: GetChangeRequest,
    ) -> Result<GetChangeResponse, RusotoError<GetChangeError>> {
        let request_uri = format!("/2013-04-01/change/{id}", id = input.id)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetChangeError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetChangeResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetChangeResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><important> <p> <code>GetCheckerIpRanges</code> still works, but we recommend that you download ip-ranges.json, which includes IP address ranges for all AWS services. For more information, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/route-53-ip-addresses.html">IP Address Ranges of Amazon Route 53 Servers</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </important></p>
    #[allow(unused_variables, warnings)]
    async fn get_checker_ip_ranges(
        &self,
        input: GetCheckerIpRangesRequest,
    ) -> Result<GetCheckerIpRangesResponse, RusotoError<GetCheckerIpRangesError>> {
        let request_uri = "/2013-04-01/checkeripranges";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetCheckerIpRangesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetCheckerIpRangesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                GetCheckerIpRangesResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about whether a specified geographic location is supported for Amazon Route 53 geolocation resource record sets.</p> <p>Use the following syntax to determine whether a continent is supported for geolocation:</p> <p> <code>GET /2013-04-01/geolocation?continentcode=<i>two-letter abbreviation for a continent</i> </code> </p> <p>Use the following syntax to determine whether a country is supported for geolocation:</p> <p> <code>GET /2013-04-01/geolocation?countrycode=<i>two-character country code</i> </code> </p> <p>Use the following syntax to determine whether a subdivision of a country is supported for geolocation:</p> <p> <code>GET /2013-04-01/geolocation?countrycode=<i>two-character country code</i>&amp;subdivisioncode=<i>subdivision code</i> </code> </p>
    #[allow(unused_variables, warnings)]
    async fn get_geo_location(
        &self,
        input: GetGeoLocationRequest,
    ) -> Result<GetGeoLocationResponse, RusotoError<GetGeoLocationError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetGeoLocationError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetGeoLocationResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetGeoLocationResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about a specified health check.</p>
    #[allow(unused_variables, warnings)]
    async fn get_health_check(
        &self,
        input: GetHealthCheckRequest,
    ) -> Result<GetHealthCheckResponse, RusotoError<GetHealthCheckError>> {
        let request_uri = format!(
            "/2013-04-01/healthcheck/{health_check_id}",
            health_check_id = input.health_check_id
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetHealthCheckError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetHealthCheckResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetHealthCheckResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Retrieves the number of health checks that are associated with the current AWS account.</p>
    #[allow(unused_variables, warnings)]
    async fn get_health_check_count(
        &self,
        input: GetHealthCheckCountRequest,
    ) -> Result<GetHealthCheckCountResponse, RusotoError<GetHealthCheckCountError>> {
        let request_uri = "/2013-04-01/healthcheckcount";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetHealthCheckCountError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetHealthCheckCountResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                GetHealthCheckCountResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets the reason that a specified health check failed most recently.</p>
    #[allow(unused_variables, warnings)]
    async fn get_health_check_last_failure_reason(
        &self,
        input: GetHealthCheckLastFailureReasonRequest,
    ) -> Result<
        GetHealthCheckLastFailureReasonResponse,
        RusotoError<GetHealthCheckLastFailureReasonError>,
    > {
        let request_uri = format!(
            "/2013-04-01/healthcheck/{health_check_id}/lastfailurereason",
            health_check_id = input.health_check_id
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetHealthCheckLastFailureReasonError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetHealthCheckLastFailureReasonResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetHealthCheckLastFailureReasonResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets status of a specified health check. </p>
    #[allow(unused_variables, warnings)]
    async fn get_health_check_status(
        &self,
        input: GetHealthCheckStatusRequest,
    ) -> Result<GetHealthCheckStatusResponse, RusotoError<GetHealthCheckStatusError>> {
        let request_uri = format!(
            "/2013-04-01/healthcheck/{health_check_id}/status",
            health_check_id = input.health_check_id
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetHealthCheckStatusError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetHealthCheckStatusResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetHealthCheckStatusResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about a specified hosted zone including the four name servers assigned to the hosted zone.</p>
    #[allow(unused_variables, warnings)]
    async fn get_hosted_zone(
        &self,
        input: GetHostedZoneRequest,
    ) -> Result<GetHostedZoneResponse, RusotoError<GetHostedZoneError>> {
        let request_uri = format!("/2013-04-01/hostedzone/{id}", id = input.id)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetHostedZoneError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetHostedZoneResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetHostedZoneResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Retrieves the number of hosted zones that are associated with the current AWS account.</p>
    #[allow(unused_variables, warnings)]
    async fn get_hosted_zone_count(
        &self,
        input: GetHostedZoneCountRequest,
    ) -> Result<GetHostedZoneCountResponse, RusotoError<GetHostedZoneCountError>> {
        let request_uri = "/2013-04-01/hostedzonecount";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetHostedZoneCountError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetHostedZoneCountResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                GetHostedZoneCountResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets the specified limit for a specified hosted zone, for example, the maximum number of records that you can create in the hosted zone. </p> <p>For the default limit, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>. To request a higher limit, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-route53">open a case</a>.</p>
    #[allow(unused_variables, warnings)]
    async fn get_hosted_zone_limit(
        &self,
        input: GetHostedZoneLimitRequest,
    ) -> Result<GetHostedZoneLimitResponse, RusotoError<GetHostedZoneLimitError>> {
        let request_uri = format!("/2013-04-01/hostedzonelimit/{id}/{type}", id = input.hosted_zone_id, type = input.type_).replace("/hostedzone/hostedzone/", "/hostedzone/").replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetHostedZoneLimitError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetHostedZoneLimitResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                GetHostedZoneLimitResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about a specified configuration for DNS query logging.</p> <p>For more information about DNS query logs, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateQueryLoggingConfig.html">CreateQueryLoggingConfig</a> and <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html">Logging DNS Queries</a>.</p>
    #[allow(unused_variables, warnings)]
    async fn get_query_logging_config(
        &self,
        input: GetQueryLoggingConfigRequest,
    ) -> Result<GetQueryLoggingConfigResponse, RusotoError<GetQueryLoggingConfigError>> {
        let request_uri = format!("/2013-04-01/queryloggingconfig/{id}", id = input.id)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetQueryLoggingConfigError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetQueryLoggingConfigResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetQueryLoggingConfigResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Retrieves information about a specified reusable delegation set, including the four name servers that are assigned to the delegation set.</p>
    #[allow(unused_variables, warnings)]
    async fn get_reusable_delegation_set(
        &self,
        input: GetReusableDelegationSetRequest,
    ) -> Result<GetReusableDelegationSetResponse, RusotoError<GetReusableDelegationSetError>> {
        let request_uri = format!("/2013-04-01/delegationset/{id}", id = input.id)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetReusableDelegationSetError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetReusableDelegationSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetReusableDelegationSetResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets the maximum number of hosted zones that you can associate with the specified reusable delegation set.</p> <p>For the default limit, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>. To request a higher limit, <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=service-limit-increase&amp;limitType=service-code-route53">open a case</a>.</p>
    #[allow(unused_variables, warnings)]
    async fn get_reusable_delegation_set_limit(
        &self,
        input: GetReusableDelegationSetLimitRequest,
    ) -> Result<
        GetReusableDelegationSetLimitResponse,
        RusotoError<GetReusableDelegationSetLimitError>,
    > {
        let request_uri = format!("/2013-04-01/reusabledelegationsetlimit/{id}/{type}", id = input.delegation_set_id, type = input.type_).replace("/hostedzone/hostedzone/", "/hostedzone/").replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetReusableDelegationSetLimitError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetReusableDelegationSetLimitResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetReusableDelegationSetLimitResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about a specific traffic policy version.</p>
    #[allow(unused_variables, warnings)]
    async fn get_traffic_policy(
        &self,
        input: GetTrafficPolicyRequest,
    ) -> Result<GetTrafficPolicyResponse, RusotoError<GetTrafficPolicyError>> {
        let request_uri = format!(
            "/2013-04-01/trafficpolicy/{id}/{version}",
            id = input.id,
            version = input.version
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetTrafficPolicyError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetTrafficPolicyResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                GetTrafficPolicyResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Gets information about a specified traffic policy instance.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there&#39;s a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <note> <p>In the Route 53 console, traffic policy instances are known as policy records.</p> </note></p>
    #[allow(unused_variables, warnings)]
    async fn get_traffic_policy_instance(
        &self,
        input: GetTrafficPolicyInstanceRequest,
    ) -> Result<GetTrafficPolicyInstanceResponse, RusotoError<GetTrafficPolicyInstanceError>> {
        let request_uri = format!("/2013-04-01/trafficpolicyinstance/{id}", id = input.id)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetTrafficPolicyInstanceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetTrafficPolicyInstanceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetTrafficPolicyInstanceResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets the number of traffic policy instances that are associated with the current AWS account.</p>
    #[allow(unused_variables, warnings)]
    async fn get_traffic_policy_instance_count(
        &self,
        input: GetTrafficPolicyInstanceCountRequest,
    ) -> Result<
        GetTrafficPolicyInstanceCountResponse,
        RusotoError<GetTrafficPolicyInstanceCountError>,
    > {
        let request_uri = "/2013-04-01/trafficpolicyinstancecount";

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetTrafficPolicyInstanceCountError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = GetTrafficPolicyInstanceCountResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = GetTrafficPolicyInstanceCountResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Retrieves a list of supported geographic locations.</p> <p>Countries are listed first, and continents are listed last. If Amazon Route 53 supports subdivisions for a country (for example, states or provinces), the subdivisions for that country are listed in alphabetical order immediately after the corresponding country.</p>
    #[allow(unused_variables, warnings)]
    async fn list_geo_locations(
        &self,
        input: ListGeoLocationsRequest,
    ) -> Result<ListGeoLocationsResponse, RusotoError<ListGeoLocationsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListGeoLocationsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListGeoLocationsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                ListGeoLocationsResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Retrieve a list of the health checks that are associated with the current AWS account. </p>
    #[allow(unused_variables, warnings)]
    async fn list_health_checks(
        &self,
        input: ListHealthChecksRequest,
    ) -> Result<ListHealthChecksResponse, RusotoError<ListHealthChecksError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListHealthChecksError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListHealthChecksResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                ListHealthChecksResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Retrieves a list of the public and private hosted zones that are associated with the current AWS account. The response includes a <code>HostedZones</code> child element for each hosted zone.</p> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of hosted zones, you can use the <code>maxitems</code> parameter to list them in groups of up to 100.</p>
    #[allow(unused_variables, warnings)]
    async fn list_hosted_zones(
        &self,
        input: ListHostedZonesRequest,
    ) -> Result<ListHostedZonesResponse, RusotoError<ListHostedZonesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListHostedZonesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListHostedZonesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                ListHostedZonesResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Retrieves a list of your hosted zones in lexicographic order. The response includes a <code>HostedZones</code> child element for each hosted zone created by the current AWS account. </p> <p> <code>ListHostedZonesByName</code> sorts hosted zones by name with the labels reversed. For example:</p> <p> <code>com.example.www.</code> </p> <p>Note the trailing dot, which can change the sort order in some circumstances.</p> <p>If the domain name includes escape characters or Punycode, <code>ListHostedZonesByName</code> alphabetizes the domain name using the escaped or Punycoded value, which is the format that Amazon Route 53 saves in its database. For example, to create a hosted zone for exämple.com, you specify ex\344mple.com for the domain name. <code>ListHostedZonesByName</code> alphabetizes it as:</p> <p> <code>com.ex\344mple.</code> </p> <p>The labels are reversed and alphabetized using the escaped value. For more information about valid domain name formats, including internationalized domain names, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DomainNameFormat.html">DNS Domain Name Format</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>Route 53 returns up to 100 items in each response. If you have a lot of hosted zones, use the <code>MaxItems</code> parameter to list them in groups of up to 100. The response includes values that help navigate from one group of <code>MaxItems</code> hosted zones to the next:</p> <ul> <li> <p>The <code>DNSName</code> and <code>HostedZoneId</code> elements in the response contain the values, if any, specified for the <code>dnsname</code> and <code>hostedzoneid</code> parameters in the request that produced the current response.</p> </li> <li> <p>The <code>MaxItems</code> element in the response contains the value, if any, that you specified for the <code>maxitems</code> parameter in the request that produced the current response.</p> </li> <li> <p>If the value of <code>IsTruncated</code> in the response is true, there are more hosted zones associated with the current AWS account. </p> <p>If <code>IsTruncated</code> is false, this response includes the last hosted zone that is associated with the current account. The <code>NextDNSName</code> element and <code>NextHostedZoneId</code> elements are omitted from the response.</p> </li> <li> <p>The <code>NextDNSName</code> and <code>NextHostedZoneId</code> elements in the response contain the domain name and the hosted zone ID of the next hosted zone that is associated with the current AWS account. If you want to list more hosted zones, make another call to <code>ListHostedZonesByName</code>, and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p> </li> </ul></p>
    #[allow(unused_variables, warnings)]
    async fn list_hosted_zones_by_name(
        &self,
        input: ListHostedZonesByNameRequest,
    ) -> Result<ListHostedZonesByNameResponse, RusotoError<ListHostedZonesByNameError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListHostedZonesByNameError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListHostedZonesByNameResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListHostedZonesByNameResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists the configurations for DNS query logging that are associated with the current AWS account or the configuration that is associated with a specified hosted zone.</p> <p>For more information about DNS query logs, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateQueryLoggingConfig.html">CreateQueryLoggingConfig</a>. Additional information, including the format of DNS query logs, appears in <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html">Logging DNS Queries</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn list_query_logging_configs(
        &self,
        input: ListQueryLoggingConfigsRequest,
    ) -> Result<ListQueryLoggingConfigsResponse, RusotoError<ListQueryLoggingConfigsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListQueryLoggingConfigsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListQueryLoggingConfigsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListQueryLoggingConfigsResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists the resource record sets in a specified hosted zone.</p> <p> <code>ListResourceRecordSets</code> returns up to 100 resource record sets at a time in ASCII order, beginning at a position specified by the <code>name</code> and <code>type</code> elements.</p> <p> <b>Sort order</b> </p> <p> <code>ListResourceRecordSets</code> sorts results first by DNS name with the labels reversed, for example:</p> <p> <code>com.example.www.</code> </p> <p>Note the trailing dot, which can change the sort order when the record name contains characters that appear before <code>.</code> (decimal 46) in the ASCII table. These characters include the following: <code>! " # $ % &amp; ' ( ) * + , -</code> </p> <p>When multiple records have the same DNS name, <code>ListResourceRecordSets</code> sorts results by the record type.</p> <p> <b>Specifying where to start listing records</b> </p> <p>You can use the name and type elements to specify the resource record set that the list begins with:</p> <dl> <dt>If you do not specify Name or Type</dt> <dd> <p>The results begin with the first resource record set that the hosted zone contains.</p> </dd> <dt>If you specify Name but not Type</dt> <dd> <p>The results begin with the first resource record set in the list whose name is greater than or equal to <code>Name</code>.</p> </dd> <dt>If you specify Type but not Name</dt> <dd> <p>Amazon Route 53 returns the <code>InvalidInput</code> error.</p> </dd> <dt>If you specify both Name and Type</dt> <dd> <p>The results begin with the first resource record set in the list whose name is greater than or equal to <code>Name</code>, and whose type is greater than or equal to <code>Type</code>.</p> </dd> </dl> <p> <b>Resource record sets that are PENDING</b> </p> <p>This action returns the most current version of the records. This includes records that are <code>PENDING</code>, and that are not yet available on all Route 53 DNS servers.</p> <p> <b>Changing resource record sets</b> </p> <p>To ensure that you get an accurate listing of the resource record sets for a hosted zone at a point in time, do not submit a <code>ChangeResourceRecordSets</code> request while you're paging through the results of a <code>ListResourceRecordSets</code> request. If you do, some pages may display results without the latest changes while other pages display results with the latest changes.</p> <p> <b>Displaying the next page of results</b> </p> <p>If a <code>ListResourceRecordSets</code> command returns more than one page of results, the value of <code>IsTruncated</code> is <code>true</code>. To display the next page of results, get the values of <code>NextRecordName</code>, <code>NextRecordType</code>, and <code>NextRecordIdentifier</code> (if any) from the response. Then submit another <code>ListResourceRecordSets</code> request, and specify those values for <code>StartRecordName</code>, <code>StartRecordType</code>, and <code>StartRecordIdentifier</code>.</p>
    #[allow(unused_variables, warnings)]
    async fn list_resource_record_sets(
        &self,
        input: ListResourceRecordSetsRequest,
    ) -> Result<ListResourceRecordSetsResponse, RusotoError<ListResourceRecordSetsError>> {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/rrset",
            id = input.hosted_zone_id
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListResourceRecordSetsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListResourceRecordSetsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListResourceRecordSetsResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Retrieves a list of the reusable delegation sets that are associated with the current AWS account.</p>
    #[allow(unused_variables, warnings)]
    async fn list_reusable_delegation_sets(
        &self,
        input: ListReusableDelegationSetsRequest,
    ) -> Result<ListReusableDelegationSetsResponse, RusotoError<ListReusableDelegationSetsError>>
    {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListReusableDelegationSetsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListReusableDelegationSetsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListReusableDelegationSetsResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists tags for one health check or hosted zone. </p> <p>For information about using tags for cost allocation, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!(
            "/2013-04-01/tags/{resource_type}/{resource_id}",
            resource_id = input.resource_id,
            resource_type = input.resource_type
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

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
            result = ListTagsForResourceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                ListTagsForResourceResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Lists tags for up to 10 health checks or hosted zones.</p> <p>For information about using tags for cost allocation, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn list_tags_for_resources(
        &self,
        input: ListTagsForResourcesRequest,
    ) -> Result<ListTagsForResourcesResponse, RusotoError<ListTagsForResourcesError>> {
        let request_uri = format!(
            "/2013-04-01/tags/{resource_type}",
            resource_type = input.resource_type
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        ListTagsForResourcesRequestSerializer::serialize(
            &mut writer,
            "ListTagsForResourcesRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListTagsForResourcesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListTagsForResourcesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListTagsForResourcesResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about the latest version for every traffic policy that is associated with the current AWS account. Policies are listed in the order that they were created in. </p>
    #[allow(unused_variables, warnings)]
    async fn list_traffic_policies(
        &self,
        input: ListTrafficPoliciesRequest,
    ) -> Result<ListTrafficPoliciesResponse, RusotoError<ListTrafficPoliciesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListTrafficPoliciesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListTrafficPoliciesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                ListTrafficPoliciesResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about the traffic policy instances that you created by using the current AWS account.</p> <note> <p>After you submit an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p>
    #[allow(unused_variables, warnings)]
    async fn list_traffic_policy_instances(
        &self,
        input: ListTrafficPolicyInstancesRequest,
    ) -> Result<ListTrafficPolicyInstancesResponse, RusotoError<ListTrafficPolicyInstancesError>>
    {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListTrafficPolicyInstancesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListTrafficPolicyInstancesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListTrafficPolicyInstancesResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about the traffic policy instances that you created in a specified hosted zone.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p>
    #[allow(unused_variables, warnings)]
    async fn list_traffic_policy_instances_by_hosted_zone(
        &self,
        input: ListTrafficPolicyInstancesByHostedZoneRequest,
    ) -> Result<
        ListTrafficPolicyInstancesByHostedZoneResponse,
        RusotoError<ListTrafficPolicyInstancesByHostedZoneError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListTrafficPolicyInstancesByHostedZoneError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListTrafficPolicyInstancesByHostedZoneResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListTrafficPolicyInstancesByHostedZoneResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about the traffic policy instances that you created by using a specify traffic policy version.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p>
    #[allow(unused_variables, warnings)]
    async fn list_traffic_policy_instances_by_policy(
        &self,
        input: ListTrafficPolicyInstancesByPolicyRequest,
    ) -> Result<
        ListTrafficPolicyInstancesByPolicyResponse,
        RusotoError<ListTrafficPolicyInstancesByPolicyError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListTrafficPolicyInstancesByPolicyError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListTrafficPolicyInstancesByPolicyResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListTrafficPolicyInstancesByPolicyResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets information about all of the versions for a specified traffic policy.</p> <p>Traffic policy versions are listed in numerical order by <code>VersionNumber</code>.</p>
    #[allow(unused_variables, warnings)]
    async fn list_traffic_policy_versions(
        &self,
        input: ListTrafficPolicyVersionsRequest,
    ) -> Result<ListTrafficPolicyVersionsResponse, RusotoError<ListTrafficPolicyVersionsError>>
    {
        let request_uri = format!("/2013-04-01/trafficpolicies/{id}/versions", id = input.id)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.max_items {
            params.put("maxitems", x);
        }
        if let Some(ref x) = input.traffic_policy_version_marker {
            params.put("trafficpolicyversion", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListTrafficPolicyVersionsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListTrafficPolicyVersionsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListTrafficPolicyVersionsResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets a list of the VPCs that were created by other accounts and that can be associated with a specified hosted zone because you've submitted one or more <code>CreateVPCAssociationAuthorization</code> requests. </p> <p>The response includes a <code>VPCs</code> element with a <code>VPC</code> child element for each VPC that can be associated with the hosted zone.</p>
    #[allow(unused_variables, warnings)]
    async fn list_vpc_association_authorizations(
        &self,
        input: ListVPCAssociationAuthorizationsRequest,
    ) -> Result<
        ListVPCAssociationAuthorizationsResponse,
        RusotoError<ListVPCAssociationAuthorizationsError>,
    > {
        let request_uri = format!(
            "/2013-04-01/hostedzone/{id}/authorizevpcassociation",
            id = input.hosted_zone_id
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("GET", "route53", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxresults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nexttoken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListVPCAssociationAuthorizationsError::from_response(
                response,
            ));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = ListVPCAssociationAuthorizationsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ListVPCAssociationAuthorizationsResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Gets the value that Amazon Route 53 returns in response to a DNS request for a specified record name and type. You can optionally specify the IP address of a DNS resolver, an EDNS0 client subnet IP address, and a subnet mask. </p>
    #[allow(unused_variables, warnings)]
    async fn test_dns_answer(
        &self,
        input: TestDNSAnswerRequest,
    ) -> Result<TestDNSAnswerResponse, RusotoError<TestDNSAnswerError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(TestDNSAnswerError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = TestDNSAnswerResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = TestDNSAnswerResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Updates an existing health check. Note that some values can't be updated. </p> <p>For more information about updating health checks, see <a href="http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/health-checks-creating-deleting.html">Creating, Updating, and Deleting Health Checks</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    #[allow(unused_variables, warnings)]
    async fn update_health_check(
        &self,
        input: UpdateHealthCheckRequest,
    ) -> Result<UpdateHealthCheckResponse, RusotoError<UpdateHealthCheckError>> {
        let request_uri = format!(
            "/2013-04-01/healthcheck/{health_check_id}",
            health_check_id = input.health_check_id
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        UpdateHealthCheckRequestSerializer::serialize(
            &mut writer,
            "UpdateHealthCheckRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateHealthCheckError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = UpdateHealthCheckResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                UpdateHealthCheckResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Updates the comment for a specified hosted zone.</p>
    #[allow(unused_variables, warnings)]
    async fn update_hosted_zone_comment(
        &self,
        input: UpdateHostedZoneCommentRequest,
    ) -> Result<UpdateHostedZoneCommentResponse, RusotoError<UpdateHostedZoneCommentError>> {
        let request_uri = format!("/2013-04-01/hostedzone/{id}", id = input.id)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        UpdateHostedZoneCommentRequestSerializer::serialize(
            &mut writer,
            "UpdateHostedZoneCommentRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateHostedZoneCommentError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = UpdateHostedZoneCommentResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = UpdateHostedZoneCommentResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p>Updates the comment for a specified traffic policy version.</p>
    #[allow(unused_variables, warnings)]
    async fn update_traffic_policy_comment(
        &self,
        input: UpdateTrafficPolicyCommentRequest,
    ) -> Result<UpdateTrafficPolicyCommentResponse, RusotoError<UpdateTrafficPolicyCommentError>>
    {
        let request_uri = format!(
            "/2013-04-01/trafficpolicy/{id}/{version}",
            id = input.id,
            version = input.version
        )
        .replace("/hostedzone/hostedzone/", "/hostedzone/")
        .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        UpdateTrafficPolicyCommentRequestSerializer::serialize(
            &mut writer,
            "UpdateTrafficPolicyCommentRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateTrafficPolicyCommentError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = UpdateTrafficPolicyCommentResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = UpdateTrafficPolicyCommentResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p><p>Updates the resource record sets in a specified hosted zone that were created based on the settings in a specified traffic policy version.</p> <p>When you update a traffic policy instance, Amazon Route 53 continues to respond to DNS queries for the root resource record set name (such as example.com) while it replaces one group of resource record sets with another. Route 53 performs the following operations:</p> <ol> <li> <p>Route 53 creates a new group of resource record sets based on the specified traffic policy. This is true regardless of how significant the differences are between the existing resource record sets and the new resource record sets. </p> </li> <li> <p>When all of the new resource record sets have been created, Route 53 starts to respond to DNS queries for the root resource record set name (such as example.com) by using the new resource record sets.</p> </li> <li> <p>Route 53 deletes the old group of resource record sets that are associated with the root resource record set name.</p> </li> </ol></p>
    #[allow(unused_variables, warnings)]
    async fn update_traffic_policy_instance(
        &self,
        input: UpdateTrafficPolicyInstanceRequest,
    ) -> Result<UpdateTrafficPolicyInstanceResponse, RusotoError<UpdateTrafficPolicyInstanceError>>
    {
        let request_uri = format!("/2013-04-01/trafficpolicyinstance/{id}", id = input.id)
            .replace("/hostedzone/hostedzone/", "/hostedzone/")
            .replace("/change/change/", "/change/");

        let mut request = SignedRequest::new("POST", "route53", &self.region, &request_uri);

        let mut writer = EventWriter::new(Vec::new());
        UpdateTrafficPolicyInstanceRequestSerializer::serialize(
            &mut writer,
            "UpdateTrafficPolicyInstanceRequest",
            &input,
            "https://route53.amazonaws.com/doc/2013-04-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(UpdateTrafficPolicyInstanceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let mut result;

        if xml_response.body.is_empty() {
            result = UpdateTrafficPolicyInstanceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = UpdateTrafficPolicyInstanceResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
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
    async fn test_parse_error_route_53_get_hosted_zone() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "route53-get-hosted-zone.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = Route53Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetHostedZoneRequest::default();
        let result = client.get_hosted_zone(request).await;
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }
}
