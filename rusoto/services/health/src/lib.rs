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
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
//! <p><fullname>AWS Health</fullname> <p>The AWS Health API provides programmatic access to the AWS Health information that appears in the <a href="https://phd.aws.amazon.com/phd/home#/">AWS Personal Health Dashboard</a>. You can use the API operations to get information about AWS Health events that affect your AWS services and resources.</p> <note> <ul> <li> <p>You must have a Business or Enterprise Support plan from <a href="http://aws.amazon.com/premiumsupport/">AWS Support</a> to use the AWS Health API. If you call the AWS Health API from an AWS account that doesn&#39;t have a Business or Enterprise Support plan, you receive a <code>SubscriptionRequiredException</code> error.</p> </li> <li> <p>You can use the AWS Health endpoint health.us-east-1.amazonaws.com (HTTPS) to call the AWS Health API operations. AWS Health supports a multi-Region application architecture and has two regional endpoints in an active-passive configuration. You can use the high availability endpoint example to determine which AWS Region is active, so that you can get the latest information from the API. For more information, see <a href="https://docs.aws.amazon.com/health/latest/ug/health-api.html">Accessing the AWS Health API</a> in the <i>AWS Health User Guide</i>.</p> </li> </ul> </note> <p>For authentication of requests, AWS Health uses the <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 Signing Process</a>.</p> <p>If your AWS account is part of AWS Organizations, you can use the AWS Health organizational view feature. This feature provides a centralized view of AWS Health events across all accounts in your organization. You can aggregate AWS Health events in real time to identify accounts in your organization that are affected by an operational event or get notified of security vulnerabilities. Use the organizational view API operations to enable this feature and return event information. For more information, see <a href="https://docs.aws.amazon.com/health/latest/ug/aggregate-events.html">Aggregating AWS Health events</a> in the <i>AWS Health User Guide</i>.</p> <note> <p>When you use the AWS Health API operations to return AWS Health events, see the following recommendations:</p> <ul> <li> <p>Use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html#AWSHealth-Type-Event-eventScopeCode">eventScopeCode</a> parameter to specify whether to return AWS Health events that are public or account-specific.</p> </li> <li> <p>Use pagination to view all events from the response. For example, if you call the <code>DescribeEventsForOrganization</code> operation to get all events in your organization, you might receive several page results. Specify the <code>nextToken</code> in the next request to return more results.</p> </li> </ul> </note></p>
//!
//! If you're using the service, you're probably looking for [AWSHealthClient](struct.AWSHealthClient.html) and [AWSHealth](trait.AWSHealth.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
