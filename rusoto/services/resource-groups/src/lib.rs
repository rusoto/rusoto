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
//! <p><fullname>AWS Resource Groups</fullname> <p>AWS Resource Groups lets you organize AWS resources such as Amazon EC2 instances, Amazon Relational Database Service databases, and Amazon S3 buckets into groups using criteria that you define as tags. A resource group is a collection of resources that match the resource types specified in a query, and share one or more tags or portions of tags. You can create a group of resources based on their roles in your cloud infrastructure, lifecycle stages, regions, application layers, or virtually any criteria. Resource groups enable you to automate management tasks, such as those in AWS Systems Manager Automation documents, on tag-related resources in AWS Systems Manager. Groups of tagged resources also let you quickly view a custom console in AWS Systems Manager that shows AWS Config compliance and other monitoring data about member resources.</p> <p>To create a resource group, build a resource query, and specify tags that identify the criteria that members of the group have in common. Tags are key-value pairs.</p> <p>For more information about Resource Groups, see the <a href="https://docs.aws.amazon.com/ARG/latest/userguide/welcome.html">AWS Resource Groups User Guide</a>.</p> <p>AWS Resource Groups uses a REST-compliant API that you can use to perform the following types of operations.</p> <ul> <li> <p>Create, Read, Update, and Delete (CRUD) operations on resource groups and resource query entities</p> </li> <li> <p>Applying, editing, and removing tags from resource groups</p> </li> <li> <p>Resolving resource group member ARNs so they can be returned as search results</p> </li> <li> <p>Getting data about resources that are members of a group</p> </li> <li> <p>Searching AWS resources based on a resource query</p> </li> </ul></p>
//!
//! If you're using the service, you're probably looking for [ResourceGroupsClient](struct.ResourceGroupsClient.html) and [ResourceGroups](trait.ResourceGroups.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
