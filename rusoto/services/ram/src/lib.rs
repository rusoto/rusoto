
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
//! <p>Use AWS Resource Access Manager to share AWS resources between AWS accounts. To share a resource, you create a resource share, associate the resource with the resource share, and specify the principals that can access the resource. The following principals are supported:</p> <ul> <li> <p>The ID of an AWS account</p> </li> <li> <p>The Amazon Resource Name (ARN) of an OU from AWS Organizations</p> </li> <li> <p>The Amazon Resource Name (ARN) of an organization from AWS Organizations</p> </li> </ul> <p>If you specify an AWS account that doesn't exist in the same organization as the account that owns the resource share, the owner of the specified account receives an invitation to accept the resource share. After the owner accepts the invitation, they can access the resources in the resource share. An administrator of the specified account can use IAM policies to restrict access resources in the resource share.</p>
//!
//! If you're using the service, you're probably looking for [RamClient](struct.RamClient.html) and [Ram](trait.Ram.html).

extern crate bytes;
extern crate futures;
#[macro_use]
extern crate log;
extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            
