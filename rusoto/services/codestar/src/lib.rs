
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

//! <fullname>AWS CodeStar</fullname> <p>This is the API reference for AWS CodeStar. This reference provides descriptions of the operations and data types for the AWS CodeStar API along with usage examples.</p> <p>You can use the AWS CodeStar API to work with:</p> <p>Projects and their resources, by calling the following:</p> <ul> <li> <p> <a>DeleteProject</a>, which deletes a project in AWS CodeStar.</p> </li> <li> <p> <a>DescribeProject</a>, which lists the attributes of a project.</p> </li> <li> <p> <a>ListProjects</a>, which lists all AWS CodeStar projects associated with your AWS account.</p> </li> <li> <p> <a>ListResources</a>, which lists the resources associated with an AWS CodeStar project.</p> </li> <li> <p> <a>UpdateProject</a>, which updates the attributes of an AWS CodeStar project.</p> </li> </ul> <p>Teams and team members, by calling the following:</p> <ul> <li> <p> <a>AssociateTeamMember</a>, which adds an IAM user to the team for an AWS CodeStar project.</p> </li> <li> <p> <a>DisassociateTeamMember</a>, which removes an IAM user from the team for an AWS CodeStar project.</p> </li> <li> <p> <a>ListTeamMembers</a>, which lists all the IAM users in the team for an AWS CodeStar project, including their roles and attributes.</p> </li> </ul> <p>Users, by calling the following:</p> <ul> <li> <p> <a>CreateUserProfile</a>, which creates a user profile that contains data associated with the user across all AWS CodeStar projects.</p> </li> <li> <p> <a>DeleteUserProfile</a>, which deletes all user profile information across all AWS CodeStar projects.</p> </li> <li> <p> <a>DescribeUserProfile</a>, which describes the profile of a user.</p> </li> <li> <p> <a>ListUserProfiles</a>, which lists all AWS CodeStar user profiles.</p> </li> <li> <p> <a>UpdateUserProfile</a>, which updates the profile for an AWS CodeStar user. </p> </li> </ul>
//!
//! If you're using the service, you're probably looking for [CodeStarClient](struct.CodeStarClient.html) and [CodeStar](trait.CodeStar.html).

extern crate futures;
extern crate hyper;
extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            
