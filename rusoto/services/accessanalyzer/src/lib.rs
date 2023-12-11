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
//! <p>AWS IAM Access Analyzer helps identify potential resource-access risks by enabling you to identify any policies that grant access to an external principal. It does this by using logic-based reasoning to analyze resource-based policies in your AWS environment. An external principal can be another AWS account, a root user, an IAM user or role, a federated user, an AWS service, or an anonymous user. You can also use Access Analyzer to preview and validate public and cross-account access to your resources before deploying permissions changes. This guide describes the AWS IAM Access Analyzer operations that you can call programmatically. For general information about Access Analyzer, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/what-is-access-analyzer.html">AWS IAM Access Analyzer</a> in the <b>IAM User Guide</b>.</p> <p>To start using Access Analyzer, you first need to create an analyzer.</p>
//!
//! If you're using the service, you're probably looking for [AccessAnalyzerClient](struct.AccessAnalyzerClient.html) and [AccessAnalyzer](trait.AccessAnalyzer.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
