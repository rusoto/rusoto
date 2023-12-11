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
//! <p>Using AWS Batch, you can run batch computing workloads on the AWS Cloud. Batch computing is a common means for developers, scientists, and engineers to access large amounts of compute resources. AWS Batch uses the advantages of this computing workload to remove the undifferentiated heavy lifting of configuring and managing required infrastructure. At the same time, it also adopts a familiar batch computing software approach. Given these advantages, AWS Batch can help you to efficiently provision resources in response to jobs submitted, thus effectively helping you to eliminate capacity constraints, reduce compute costs, and deliver your results more quickly.</p> <p>As a fully managed service, AWS Batch can run batch computing workloads of any scale. AWS Batch automatically provisions compute resources and optimizes workload distribution based on the quantity and scale of your specific workloads. With AWS Batch, there's no need to install or manage batch computing software. This means that you can focus your time and energy on analyzing results and solving your specific problems.</p>
//!
//! If you're using the service, you're probably looking for [BatchClient](struct.BatchClient.html) and [Batch](trait.Batch.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
