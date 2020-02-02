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
//! <p> Amazon Elastic Compute Cloud Image Builder provides a one-stop-shop to automate the image management processes. You configure an automated pipeline that creates images for use on AWS. As software updates become available, Image Builder automatically produces a new image based on a customizable schedule and distributes it to stipulated AWS Regions after running tests on it. With the Image Builder, organizations can capture their internal or industry-specific compliance policies as a vetted template that can be consistently applied to every new image. Built-in integration with AWS Organizations provides customers with a centralized way to enforce image distribution and access policies across their AWS accounts and Regions. Image Builder supports multiple image format AMIs on AWS.</p>
//!
//! If you're using the service, you're probably looking for [ImageBuilderClient](struct.ImageBuilderClient.html) and [ImageBuilder](trait.ImageBuilder.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
