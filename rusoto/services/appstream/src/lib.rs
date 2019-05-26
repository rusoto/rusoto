
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
//! <p><fullname>Amazon AppStream 2.0</fullname> <p>This is the <i>Amazon AppStream 2.0 API Reference</i>. It provides descriptions and syntax for each of the actions and data types in AppStream 2.0. AppStream 2.0 is a fully managed application streaming service. You centrally manage your desktop applications on AppStream 2.0 and securely deliver them to any computer. AppStream 2.0 manages the AWS resources required to host and run your applications, scales automatically, and provides access to your users on demand.</p> <p>To learn more about AppStream 2.0, see the following resources:</p> <ul> <li> <p> <a href="http://aws.amazon.com/appstream2">Amazon AppStream 2.0 product page</a> </p> </li> <li> <p> <a href="http://aws.amazon.com/documentation/appstream2">Amazon AppStream 2.0 documentation</a> </p> </li> </ul></p>
//!
//! If you're using the service, you're probably looking for [AppStreamClient](struct.AppStreamClient.html) and [AppStream](trait.AppStream.html).

extern crate bytes;
extern crate futures;
extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use crate::generated::*;
pub use crate::custom::*;
            
