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
//! <p><fullname>Amazon RDS Performance Insights</fullname> <p>Amazon RDS Performance Insights enables you to monitor and explore different dimensions of database load based on data captured from a running DB instance. The guide provides detailed information about Performance Insights data types, parameters and errors.</p> <p>When Performance Insights is enabled, the Amazon RDS Performance Insights API provides visibility into the performance of your DB instance. Amazon CloudWatch provides the authoritative source for AWS service-vended monitoring metrics. Performance Insights offers a domain-specific view of DB load. </p> <p>DB load is measured as Average Active Sessions. Performance Insights provides the data to API consumers as a two-dimensional time-series dataset. The time dimension provides DB load data for each time point in the queried time range. Each time point decomposes overall load in relation to the requested dimensions, measured at that time point. Examples include SQL, Wait event, User, and Host.</p> <ul> <li> <p>To learn more about Performance Insights and Amazon Aurora DB instances, go to the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/USER_PerfInsights.html">Amazon Aurora User Guide</a>.</p> </li> <li> <p>To learn more about Performance Insights and Amazon RDS DB instances, go to the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PerfInsights.html">Amazon RDS User Guide</a>.</p> </li> </ul></p>
//!
//! If you're using the service, you're probably looking for [PerformanceInsightsClient](struct.PerformanceInsightsClient.html) and [PerformanceInsights](trait.PerformanceInsights.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
