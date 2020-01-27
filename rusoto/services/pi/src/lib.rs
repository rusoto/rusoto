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
//! <p>AWS Performance Insights enables you to monitor and explore different dimensions of database load based on data captured from a running RDS instance. The guide provides detailed information about Performance Insights data types, parameters and errors. For more information about Performance Insights capabilities see <a href="http://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PerfInsights.html">Using Amazon RDS Performance Insights </a> in the <i>Amazon RDS User Guide</i>.</p> <p> The AWS Performance Insights API provides visibility into the performance of your RDS instance, when Performance Insights is enabled for supported engine types. While Amazon CloudWatch provides the authoritative source for AWS service vended monitoring metrics, AWS Performance Insights offers a domain-specific view of database load measured as Average Active Sessions and provided to API consumers as a 2-dimensional time-series dataset. The time dimension of the data provides DB load data for each time point in the queried time range, and each time point decomposes overall load in relation to the requested dimensions, such as SQL, Wait-event, User or Host, measured at that time point.</p>
//!
//! If you're using the service, you're probably looking for [PerformanceInsightsClient](struct.PerformanceInsightsClient.html) and [PerformanceInsights](trait.PerformanceInsights.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
