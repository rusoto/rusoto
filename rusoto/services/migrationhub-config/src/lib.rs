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
//! <p>The AWS Migration Hub home region APIs are available specifically for working with your Migration Hub home region. You can use these APIs to determine a home region, as well as to create and work with controls that describe the home region.</p> <ul> <li> <p>You must make API calls for write actions (create, notify, associate, disassociate, import, or put) while in your home region, or a <code>HomeRegionNotSetException</code> error is returned.</p> </li> <li> <p>API calls for read actions (list, describe, stop, and delete) are permitted outside of your home region.</p> </li> <li> <p>If you call a write API outside the home region, an <code>InvalidInputException</code> is returned.</p> </li> <li> <p>You can call <code>GetHomeRegion</code> action to obtain the account's Migration Hub home region.</p> </li> </ul> <p>For specific API usage, see the sections that follow in this AWS Migration Hub Home Region API reference. </p>
//!
//! If you're using the service, you're probably looking for [MigrationHubConfigClient](struct.MigrationHubConfigClient.html) and [MigrationHubConfig](trait.MigrationHubConfig.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
