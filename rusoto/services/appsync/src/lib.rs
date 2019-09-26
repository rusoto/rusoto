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
//! <p>AWS AppSync provides API actions for creating and interacting with data sources using GraphQL from your application.</p>
//!
//! If you're using the service, you're probably looking for [AppSyncClient](struct.AppSyncClient.html) and [AppSync](trait.AppSync.html).

extern crate bytes;
extern crate futures;
extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod custom;
mod generated;

pub use crate::custom::*;
pub use crate::generated::*;
