
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

#![doc(html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png")]
//! <p><fullname>Amazon Elastic Compute Cloud</fullname> <p>Amazon Elastic Compute Cloud (Amazon EC2) provides secure and resizable computing capacity in the AWS cloud. Using Amazon EC2 eliminates the need to invest in hardware up front, so you can develop and deploy applications faster.</p> <p>To learn more about Amazon EC2, Amazon EBS, and Amazon VPC, see the following resources:</p> <ul> <li> <p> <a href="http://aws.amazon.com/ec2">Amazon EC2 product page</a> </p> </li> <li> <p> <a href="http://aws.amazon.com/documentation/ec2">Amazon EC2 documentation</a> </p> </li> <li> <p> <a href="http://aws.amazon.com/ebs">Amazon EBS product page</a> </p> </li> <li> <p> <a href="http://aws.amazon.com/vpc">Amazon VPC product page</a> </p> </li> <li> <p> <a href="http://aws.amazon.com/documentation/vpc">Amazon VPC documentation</a> </p> </li> </ul></p>
//!
//! If you're using the service, you're probably looking for [Ec2Client](struct.Ec2Client.html) and [Ec2](trait.Ec2.html).

extern crate futures;
extern crate rusoto_core;
extern crate serde_urlencoded;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            
