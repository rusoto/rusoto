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
//! <p><p>Security Hub provides you with a comprehensive view of the security state of your AWS environment and resources. It also provides you with the compliance status of your environment based on CIS AWS Foundations compliance checks. Security Hub collects security data from AWS accounts, services, and integrated third-party products and helps you analyze security trends in your environment to identify the highest priority security issues. For more information about Security Hub, see the <i> <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/what-is-securityhub.html">AWS Security Hub User Guide</a> </i>.</p> <p>When you use operations in the Security Hub API, the requests are executed only in the AWS Region that is currently active or in the specific AWS Region that you specify in your request. Any configuration or settings change that results from the operation is applied only to that Region. To make the same change in other Regions, execute the same command for each Region to apply the change to. For example, if your Region is set to <code>us-west-2</code>, when you use <code>CreateMembers</code> to add a member account to Security Hub, the association of the member account with the master account is created only in the us-west-2 Region. Security Hub must be enabled for the member account in the same Region that the invite was sent from.</p> <p>The following throttling limits apply to using Security Hub API operations:</p> <ul> <li> <p> <code>GetFindings</code> - RateLimit of 3 requests per second, and a BurstLimit of 6 requests per second.</p> </li> <li> <p> <code>UpdateFindings</code> - RateLimit of 1 request per second, and a BurstLimit of 5 requests per second.</p> </li> <li> <p>All other operations - RateLimit of 10 request per second, and a BurstLimit of 30 requests per second.</p> </li> </ul></p>
//!
//! If you're using the service, you're probably looking for [SecurityHubClient](struct.SecurityHubClient.html) and [SecurityHub](trait.SecurityHub.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
