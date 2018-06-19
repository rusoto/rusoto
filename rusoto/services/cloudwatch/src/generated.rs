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

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use rusoto_core::xmlerror::*;
use rusoto_core::xmlutil::{
    characters, end_element, find_start_element, peek_at_name, skip_tree, start_element,
};
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::reader::XmlEvent;
use xml::EventReader;

enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
struct ActionsEnabledDeserializer;
impl ActionsEnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct AlarmArnDeserializer;
impl AlarmArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct AlarmDescriptionDeserializer;
impl AlarmDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents the history of a specific alarm.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AlarmHistoryItem {
    /// <p>The descriptive name for the alarm.</p>
    pub alarm_name: Option<String>,
    /// <p>Data about the alarm, in JSON format.</p>
    pub history_data: Option<String>,
    /// <p>The type of alarm history item.</p>
    pub history_item_type: Option<String>,
    /// <p>A summary of the alarm history, in text format.</p>
    pub history_summary: Option<String>,
    /// <p>The time stamp for the alarm history item.</p>
    pub timestamp: Option<String>,
}

struct AlarmHistoryItemDeserializer;
impl AlarmHistoryItemDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AlarmHistoryItem, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AlarmHistoryItem::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AlarmName" => {
                        obj.alarm_name =
                            Some(try!(AlarmNameDeserializer::deserialize("AlarmName", stack)));
                    }
                    "HistoryData" => {
                        obj.history_data = Some(try!(HistoryDataDeserializer::deserialize(
                            "HistoryData",
                            stack
                        )));
                    }
                    "HistoryItemType" => {
                        obj.history_item_type = Some(try!(
                            HistoryItemTypeDeserializer::deserialize("HistoryItemType", stack)
                        ));
                    }
                    "HistorySummary" => {
                        obj.history_summary = Some(try!(HistorySummaryDeserializer::deserialize(
                            "HistorySummary",
                            stack
                        )));
                    }
                    "Timestamp" => {
                        obj.timestamp =
                            Some(try!(TimestampDeserializer::deserialize("Timestamp", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct AlarmHistoryItemsDeserializer;
impl AlarmHistoryItemsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AlarmHistoryItem>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(AlarmHistoryItemDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct AlarmNameDeserializer;
impl AlarmNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `AlarmNames` contents to a `SignedRequest`.
struct AlarmNamesSerializer;
impl AlarmNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct ComparisonOperatorDeserializer;
impl ComparisonOperatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DashboardArnDeserializer;
impl DashboardArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DashboardBodyDeserializer;
impl DashboardBodyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DashboardEntriesDeserializer;
impl DashboardEntriesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DashboardEntry>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(DashboardEntryDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Represents a specific dashboard.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DashboardEntry {
    /// <p>The Amazon Resource Name (ARN) of the dashboard.</p>
    pub dashboard_arn: Option<String>,
    /// <p>The name of the dashboard.</p>
    pub dashboard_name: Option<String>,
    /// <p>The time stamp of when the dashboard was last modified, either by an API call or through the console. This number is expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>
    pub last_modified: Option<String>,
    /// <p>The size of the dashboard, in bytes.</p>
    pub size: Option<i64>,
}

struct DashboardEntryDeserializer;
impl DashboardEntryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DashboardEntry, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DashboardEntry::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DashboardArn" => {
                        obj.dashboard_arn = Some(try!(DashboardArnDeserializer::deserialize(
                            "DashboardArn",
                            stack
                        )));
                    }
                    "DashboardName" => {
                        obj.dashboard_name = Some(try!(DashboardNameDeserializer::deserialize(
                            "DashboardName",
                            stack
                        )));
                    }
                    "LastModified" => {
                        obj.last_modified = Some(try!(LastModifiedDeserializer::deserialize(
                            "LastModified",
                            stack
                        )));
                    }
                    "Size" => {
                        obj.size = Some(try!(SizeDeserializer::deserialize("Size", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DashboardNameDeserializer;
impl DashboardNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `DashboardNames` contents to a `SignedRequest`.
struct DashboardNamesSerializer;
impl DashboardNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>An error or warning for the operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DashboardValidationMessage {
    /// <p>The data path related to the message.</p>
    pub data_path: Option<String>,
    /// <p>A message describing the error or warning.</p>
    pub message: Option<String>,
}

struct DashboardValidationMessageDeserializer;
impl DashboardValidationMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DashboardValidationMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DashboardValidationMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DataPath" => {
                        obj.data_path =
                            Some(try!(DataPathDeserializer::deserialize("DataPath", stack)));
                    }
                    "Message" => {
                        obj.message =
                            Some(try!(MessageDeserializer::deserialize("Message", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DashboardValidationMessagesDeserializer;
impl DashboardValidationMessagesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DashboardValidationMessage>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(DashboardValidationMessageDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct DataPathDeserializer;
impl DataPathDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Encapsulates the statistical data that CloudWatch computes from metric data.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Datapoint {
    /// <p>The average of the metric values that correspond to the data point.</p>
    pub average: Option<f64>,
    /// <p>The percentile statistic for the data point.</p>
    pub extended_statistics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>The maximum metric value for the data point.</p>
    pub maximum: Option<f64>,
    /// <p>The minimum metric value for the data point.</p>
    pub minimum: Option<f64>,
    /// <p>The number of metric values that contributed to the aggregate value of this data point.</p>
    pub sample_count: Option<f64>,
    /// <p>The sum of the metric values for the data point.</p>
    pub sum: Option<f64>,
    /// <p>The time stamp used for the data point.</p>
    pub timestamp: Option<String>,
    /// <p>The standard unit for the data point.</p>
    pub unit: Option<String>,
}

struct DatapointDeserializer;
impl DatapointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Datapoint, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Datapoint::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Average" => {
                        obj.average = Some(try!(DatapointValueDeserializer::deserialize(
                            "Average", stack
                        )));
                    }
                    "ExtendedStatistics" => {
                        obj.extended_statistics = Some(try!(
                            DatapointValueMapDeserializer::deserialize("ExtendedStatistics", stack)
                        ));
                    }
                    "Maximum" => {
                        obj.maximum = Some(try!(DatapointValueDeserializer::deserialize(
                            "Maximum", stack
                        )));
                    }
                    "Minimum" => {
                        obj.minimum = Some(try!(DatapointValueDeserializer::deserialize(
                            "Minimum", stack
                        )));
                    }
                    "SampleCount" => {
                        obj.sample_count = Some(try!(DatapointValueDeserializer::deserialize(
                            "SampleCount",
                            stack
                        )));
                    }
                    "Sum" => {
                        obj.sum = Some(try!(DatapointValueDeserializer::deserialize("Sum", stack)));
                    }
                    "Timestamp" => {
                        obj.timestamp =
                            Some(try!(TimestampDeserializer::deserialize("Timestamp", stack)));
                    }
                    "Unit" => {
                        obj.unit = Some(try!(StandardUnitDeserializer::deserialize("Unit", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DatapointValueDeserializer;
impl DatapointValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<f64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DatapointValueMapDeserializer;
impl DatapointValueMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, f64>, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "entry" {
            try!(start_element("entry", stack));
            let key = try!(ExtendedStatisticDeserializer::deserialize("key", stack));
            let value = try!(DatapointValueDeserializer::deserialize("value", stack));
            obj.insert(key, value);
            try!(end_element("entry", stack));
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
struct DatapointValuesDeserializer;
impl DatapointValuesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<f64>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(DatapointValueDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct DatapointsDeserializer;
impl DatapointsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Datapoint>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(DatapointDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct DatapointsToAlarmDeserializer;
impl DatapointsToAlarmDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteAlarmsInput {
    /// <p>The alarms to be deleted.</p>
    pub alarm_names: Vec<String>,
}

/// Serialize `DeleteAlarmsInput` contents to a `SignedRequest`.
struct DeleteAlarmsInputSerializer;
impl DeleteAlarmsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteAlarmsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AlarmNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "AlarmNames"),
            &obj.alarm_names,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDashboardsInput {
    /// <p>The dashboards to be deleted. This parameter is required.</p>
    pub dashboard_names: Vec<String>,
}

/// Serialize `DeleteDashboardsInput` contents to a `SignedRequest`.
struct DeleteDashboardsInputSerializer;
impl DeleteDashboardsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDashboardsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        DashboardNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "DashboardNames"),
            &obj.dashboard_names,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDashboardsOutput {}

struct DeleteDashboardsOutputDeserializer;
impl DeleteDashboardsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteDashboardsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteDashboardsOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAlarmHistoryInput {
    /// <p>The name of the alarm.</p>
    pub alarm_name: Option<String>,
    /// <p>The ending date to retrieve alarm history.</p>
    pub end_date: Option<String>,
    /// <p>The type of alarm histories to retrieve.</p>
    pub history_item_type: Option<String>,
    /// <p>The maximum number of alarm history records to retrieve.</p>
    pub max_records: Option<i64>,
    /// <p>The token returned by a previous call to indicate that there is more data available.</p>
    pub next_token: Option<String>,
    /// <p>The starting date to retrieve alarm history.</p>
    pub start_date: Option<String>,
}

/// Serialize `DescribeAlarmHistoryInput` contents to a `SignedRequest`.
struct DescribeAlarmHistoryInputSerializer;
impl DescribeAlarmHistoryInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeAlarmHistoryInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.alarm_name {
            params.put(
                &format!("{}{}", prefix, "AlarmName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.end_date {
            params.put(
                &format!("{}{}", prefix, "EndDate"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.history_item_type {
            params.put(
                &format!("{}{}", prefix, "HistoryItemType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.start_date {
            params.put(
                &format!("{}{}", prefix, "StartDate"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAlarmHistoryOutput {
    /// <p>The alarm histories, in JSON format.</p>
    pub alarm_history_items: Option<Vec<AlarmHistoryItem>>,
    /// <p>The token that marks the start of the next batch of returned results.</p>
    pub next_token: Option<String>,
}

struct DescribeAlarmHistoryOutputDeserializer;
impl DescribeAlarmHistoryOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAlarmHistoryOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeAlarmHistoryOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "AlarmHistoryItems" => {
                        obj.alarm_history_items = Some(try!(
                            AlarmHistoryItemsDeserializer::deserialize("AlarmHistoryItems", stack)
                        ));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAlarmsForMetricInput {
    /// <p>The dimensions associated with the metric. If the metric has any associated dimensions, you must specify them in order for the call to succeed.</p>
    pub dimensions: Option<Vec<Dimension>>,
    /// <p>The percentile statistic for the metric. Specify a value between p0.0 and p100.</p>
    pub extended_statistic: Option<String>,
    /// <p>The name of the metric.</p>
    pub metric_name: String,
    /// <p>The namespace of the metric.</p>
    pub namespace: String,
    /// <p>The period, in seconds, over which the statistic is applied.</p>
    pub period: Option<i64>,
    /// <p>The statistic for the metric, other than percentiles. For percentile statistics, use <code>ExtendedStatistics</code>.</p>
    pub statistic: Option<String>,
    /// <p>The unit for the metric.</p>
    pub unit: Option<String>,
}

/// Serialize `DescribeAlarmsForMetricInput` contents to a `SignedRequest`.
struct DescribeAlarmsForMetricInputSerializer;
impl DescribeAlarmsForMetricInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeAlarmsForMetricInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.dimensions {
            DimensionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Dimensions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.extended_statistic {
            params.put(
                &format!("{}{}", prefix, "ExtendedStatistic"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "MetricName"),
            &obj.metric_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Namespace"),
            &obj.namespace.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.period {
            params.put(
                &format!("{}{}", prefix, "Period"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.statistic {
            params.put(
                &format!("{}{}", prefix, "Statistic"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.unit {
            params.put(
                &format!("{}{}", prefix, "Unit"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAlarmsForMetricOutput {
    /// <p>The information for each alarm with the specified metric.</p>
    pub metric_alarms: Option<Vec<MetricAlarm>>,
}

struct DescribeAlarmsForMetricOutputDeserializer;
impl DescribeAlarmsForMetricOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAlarmsForMetricOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeAlarmsForMetricOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "MetricAlarms" => {
                        obj.metric_alarms = Some(try!(MetricAlarmsDeserializer::deserialize(
                            "MetricAlarms",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAlarmsInput {
    /// <p>The action name prefix.</p>
    pub action_prefix: Option<String>,
    /// <p>The alarm name prefix. If this parameter is specified, you cannot specify <code>AlarmNames</code>.</p>
    pub alarm_name_prefix: Option<String>,
    /// <p>The names of the alarms.</p>
    pub alarm_names: Option<Vec<String>>,
    /// <p>The maximum number of alarm descriptions to retrieve.</p>
    pub max_records: Option<i64>,
    /// <p>The token returned by a previous call to indicate that there is more data available.</p>
    pub next_token: Option<String>,
    /// <p>The state value to be used in matching alarms.</p>
    pub state_value: Option<String>,
}

/// Serialize `DescribeAlarmsInput` contents to a `SignedRequest`.
struct DescribeAlarmsInputSerializer;
impl DescribeAlarmsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeAlarmsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.action_prefix {
            params.put(
                &format!("{}{}", prefix, "ActionPrefix"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.alarm_name_prefix {
            params.put(
                &format!("{}{}", prefix, "AlarmNamePrefix"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.alarm_names {
            AlarmNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AlarmNames"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.state_value {
            params.put(
                &format!("{}{}", prefix, "StateValue"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAlarmsOutput {
    /// <p>The information for the specified alarms.</p>
    pub metric_alarms: Option<Vec<MetricAlarm>>,
    /// <p>The token that marks the start of the next batch of returned results.</p>
    pub next_token: Option<String>,
}

struct DescribeAlarmsOutputDeserializer;
impl DescribeAlarmsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAlarmsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeAlarmsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "MetricAlarms" => {
                        obj.metric_alarms = Some(try!(MetricAlarmsDeserializer::deserialize(
                            "MetricAlarms",
                            stack
                        )));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Expands the identity of a metric.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Dimension {
    /// <p>The name of the dimension.</p>
    pub name: String,
    /// <p>The value representing the dimension measurement.</p>
    pub value: String,
}

struct DimensionDeserializer;
impl DimensionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Dimension, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Dimension::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Name" => {
                        obj.name = try!(DimensionNameDeserializer::deserialize("Name", stack));
                    }
                    "Value" => {
                        obj.value = try!(DimensionValueDeserializer::deserialize("Value", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `Dimension` contents to a `SignedRequest`.
struct DimensionSerializer;
impl DimensionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Dimension) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "Name"),
            &obj.name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Value"),
            &obj.value.replace("+", "%2B"),
        );
    }
}

/// <p>Represents filters for a dimension.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DimensionFilter {
    /// <p>The dimension name to be matched.</p>
    pub name: String,
    /// <p>The value of the dimension to be matched.</p>
    pub value: Option<String>,
}

/// Serialize `DimensionFilter` contents to a `SignedRequest`.
struct DimensionFilterSerializer;
impl DimensionFilterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DimensionFilter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "Name"),
            &obj.name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.value {
            params.put(
                &format!("{}{}", prefix, "Value"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// Serialize `DimensionFilters` contents to a `SignedRequest`.
struct DimensionFiltersSerializer;
impl DimensionFiltersSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<DimensionFilter>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            DimensionFilterSerializer::serialize(params, &key, obj);
        }
    }
}

struct DimensionNameDeserializer;
impl DimensionNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DimensionValueDeserializer;
impl DimensionValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DimensionsDeserializer;
impl DimensionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Dimension>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(DimensionDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `Dimensions` contents to a `SignedRequest`.
struct DimensionsSerializer;
impl DimensionsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Dimension>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            DimensionSerializer::serialize(params, &key, obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DisableAlarmActionsInput {
    /// <p>The names of the alarms.</p>
    pub alarm_names: Vec<String>,
}

/// Serialize `DisableAlarmActionsInput` contents to a `SignedRequest`.
struct DisableAlarmActionsInputSerializer;
impl DisableAlarmActionsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DisableAlarmActionsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AlarmNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "AlarmNames"),
            &obj.alarm_names,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnableAlarmActionsInput {
    /// <p>The names of the alarms.</p>
    pub alarm_names: Vec<String>,
}

/// Serialize `EnableAlarmActionsInput` contents to a `SignedRequest`.
struct EnableAlarmActionsInputSerializer;
impl EnableAlarmActionsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &EnableAlarmActionsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AlarmNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "AlarmNames"),
            &obj.alarm_names,
        );
    }
}

struct EvaluateLowSampleCountPercentileDeserializer;
impl EvaluateLowSampleCountPercentileDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct EvaluationPeriodsDeserializer;
impl EvaluationPeriodsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ExtendedStatisticDeserializer;
impl ExtendedStatisticDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `ExtendedStatistics` contents to a `SignedRequest`.
struct ExtendedStatisticsSerializer;
impl ExtendedStatisticsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetDashboardInput {
    /// <p>The name of the dashboard to be described.</p>
    pub dashboard_name: String,
}

/// Serialize `GetDashboardInput` contents to a `SignedRequest`.
struct GetDashboardInputSerializer;
impl GetDashboardInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetDashboardInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DashboardName"),
            &obj.dashboard_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetDashboardOutput {
    /// <p>The Amazon Resource Name (ARN) of the dashboard.</p>
    pub dashboard_arn: Option<String>,
    /// <p>The detailed information about the dashboard, including what widgets are included and their location on the dashboard. For more information about the <code>DashboardBody</code> syntax, see <a>CloudWatch-Dashboard-Body-Structure</a>. </p>
    pub dashboard_body: Option<String>,
    /// <p>The name of the dashboard.</p>
    pub dashboard_name: Option<String>,
}

struct GetDashboardOutputDeserializer;
impl GetDashboardOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetDashboardOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetDashboardOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DashboardArn" => {
                            obj.dashboard_arn = Some(try!(DashboardArnDeserializer::deserialize(
                                "DashboardArn",
                                stack
                            )));
                        }
                        "DashboardBody" => {
                            obj.dashboard_body = Some(try!(
                                DashboardBodyDeserializer::deserialize("DashboardBody", stack)
                            ));
                        }
                        "DashboardName" => {
                            obj.dashboard_name = Some(try!(
                                DashboardNameDeserializer::deserialize("DashboardName", stack)
                            ));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetMetricDataInput {
    /// <p>The time stamp indicating the latest data to be returned.</p>
    pub end_time: String,
    /// <p>The maximum number of data points the request should return before paginating. If you omit this, the default of 100,800 is used.</p>
    pub max_datapoints: Option<i64>,
    /// <p>The metric queries to be returned. A single <code>GetMetricData</code> call can include as many as 100 <code>MetricDataQuery</code> structures. Each of these structures can specify either a metric to retrieve, or a math expression to perform on retrieved data. </p>
    pub metric_data_queries: Vec<MetricDataQuery>,
    /// <p>Include this value, if it was returned by the previous call, to get the next set of data points.</p>
    pub next_token: Option<String>,
    /// <p>The order in which data points should be returned. <code>TimestampDescending</code> returns the newest data first and paginates when the <code>MaxDatapoints</code> limit is reached. <code>TimestampAscending</code> returns the oldest data first and paginates when the <code>MaxDatapoints</code> limit is reached.</p>
    pub scan_by: Option<String>,
    /// <p>The time stamp indicating the earliest data to be returned.</p>
    pub start_time: String,
}

/// Serialize `GetMetricDataInput` contents to a `SignedRequest`.
struct GetMetricDataInputSerializer;
impl GetMetricDataInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetMetricDataInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "EndTime"),
            &obj.end_time.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.max_datapoints {
            params.put(
                &format!("{}{}", prefix, "MaxDatapoints"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        MetricDataQueriesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "MetricDataQueries"),
            &obj.metric_data_queries,
        );
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.scan_by {
            params.put(
                &format!("{}{}", prefix, "ScanBy"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "StartTime"),
            &obj.start_time.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetMetricDataOutput {
    /// <p>The metrics that are returned, including the metric name, namespace, and dimensions.</p>
    pub metric_data_results: Option<Vec<MetricDataResult>>,
    /// <p>A token that marks the next batch of returned results.</p>
    pub next_token: Option<String>,
}

struct GetMetricDataOutputDeserializer;
impl GetMetricDataOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetMetricDataOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetMetricDataOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "MetricDataResults" => {
                        obj.metric_data_results = Some(try!(
                            MetricDataResultsDeserializer::deserialize("MetricDataResults", stack)
                        ));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetMetricStatisticsInput {
    /// <p>The dimensions. If the metric contains multiple dimensions, you must include a value for each dimension. CloudWatch treats each unique combination of dimensions as a separate metric. If a specific combination of dimensions was not published, you can't retrieve statistics for it. You must specify the same dimensions that were used when the metrics were created. For an example, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#dimension-combinations">Dimension Combinations</a> in the <i>Amazon CloudWatch User Guide</i>. For more information about specifying dimensions, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
    pub dimensions: Option<Vec<Dimension>>,
    /// <p>The time stamp that determines the last data point to return.</p> <p>The value specified is exclusive; results include data points up to the specified time stamp. The time stamp must be in ISO 8601 UTC format (for example, 2016-10-10T23:00:00Z).</p>
    pub end_time: String,
    /// <p>The percentile statistics. Specify values between p0.0 and p100. When calling <code>GetMetricStatistics</code>, you must specify either <code>Statistics</code> or <code>ExtendedStatistics</code>, but not both.</p>
    pub extended_statistics: Option<Vec<String>>,
    /// <p>The name of the metric, with or without spaces.</p>
    pub metric_name: String,
    /// <p>The namespace of the metric, with or without spaces.</p>
    pub namespace: String,
    /// <p><p>The granularity, in seconds, of the returned data points. For metrics with regular resolution, a period can be as short as one minute (60 seconds) and must be a multiple of 60. For high-resolution metrics that are collected at intervals of less than one minute, the period can be 1, 5, 10, 30, 60, or any multiple of 60. High-resolution metrics are those metrics stored by a <code>PutMetricData</code> call that includes a <code>StorageResolution</code> of 1 second.</p> <p>If the <code>StartTime</code> parameter specifies a time stamp that is greater than 3 hours ago, you must specify the period as follows or no data points in that time range is returned:</p> <ul> <li> <p>Start time between 3 hours and 15 days ago - Use a multiple of 60 seconds (1 minute).</p> </li> <li> <p>Start time between 15 and 63 days ago - Use a multiple of 300 seconds (5 minutes).</p> </li> <li> <p>Start time greater than 63 days ago - Use a multiple of 3600 seconds (1 hour).</p> </li> </ul></p>
    pub period: i64,
    /// <p>The time stamp that determines the first data point to return. Start times are evaluated relative to the time that CloudWatch receives the request.</p> <p>The value specified is inclusive; results include data points with the specified time stamp. The time stamp must be in ISO 8601 UTC format (for example, 2016-10-03T23:00:00Z).</p> <p>CloudWatch rounds the specified time stamp as follows:</p> <ul> <li> <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p> </li> <li> <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p> </li> <li> <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p> </li> </ul> <p>If you set <code>Period</code> to 5, 10, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15. </p>
    pub start_time: String,
    /// <p>The metric statistics, other than percentile. For percentile statistics, use <code>ExtendedStatistics</code>. When calling <code>GetMetricStatistics</code>, you must specify either <code>Statistics</code> or <code>ExtendedStatistics</code>, but not both.</p>
    pub statistics: Option<Vec<String>>,
    /// <p>The unit for a given metric. Metrics may be reported in multiple units. Not supplying a unit results in all units being returned. If you specify only a unit that the metric does not report, the results of the call are null.</p>
    pub unit: Option<String>,
}

/// Serialize `GetMetricStatisticsInput` contents to a `SignedRequest`.
struct GetMetricStatisticsInputSerializer;
impl GetMetricStatisticsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetMetricStatisticsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.dimensions {
            DimensionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Dimensions"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "EndTime"),
            &obj.end_time.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.extended_statistics {
            ExtendedStatisticsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ExtendedStatistics"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "MetricName"),
            &obj.metric_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Namespace"),
            &obj.namespace.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Period"),
            &obj.period.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "StartTime"),
            &obj.start_time.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.statistics {
            StatisticsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Statistics"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.unit {
            params.put(
                &format!("{}{}", prefix, "Unit"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetMetricStatisticsOutput {
    /// <p>The data points for the specified metric.</p>
    pub datapoints: Option<Vec<Datapoint>>,
    /// <p>A label for the specified metric.</p>
    pub label: Option<String>,
}

struct GetMetricStatisticsOutputDeserializer;
impl GetMetricStatisticsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetMetricStatisticsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetMetricStatisticsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Datapoints" => {
                        obj.datapoints = Some(try!(DatapointsDeserializer::deserialize(
                            "Datapoints",
                            stack
                        )));
                    }
                    "Label" => {
                        obj.label =
                            Some(try!(MetricLabelDeserializer::deserialize("Label", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct HistoryDataDeserializer;
impl HistoryDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct HistoryItemTypeDeserializer;
impl HistoryItemTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct HistorySummaryDeserializer;
impl HistorySummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct LastModifiedDeserializer;
impl LastModifiedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListDashboardsInput {
    /// <p>If you specify this parameter, only the dashboards with names starting with the specified string are listed. The maximum length is 255, and valid characters are A-Z, a-z, 0-9, ".", "-", and "_". </p>
    pub dashboard_name_prefix: Option<String>,
    /// <p>The token returned by a previous call to indicate that there is more data available.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListDashboardsInput` contents to a `SignedRequest`.
struct ListDashboardsInputSerializer;
impl ListDashboardsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListDashboardsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.dashboard_name_prefix {
            params.put(
                &format!("{}{}", prefix, "DashboardNamePrefix"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListDashboardsOutput {
    /// <p>The list of matching dashboards.</p>
    pub dashboard_entries: Option<Vec<DashboardEntry>>,
    /// <p>The token that marks the start of the next batch of returned results.</p>
    pub next_token: Option<String>,
}

struct ListDashboardsOutputDeserializer;
impl ListDashboardsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDashboardsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListDashboardsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DashboardEntries" => {
                        obj.dashboard_entries = Some(try!(
                            DashboardEntriesDeserializer::deserialize("DashboardEntries", stack)
                        ));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListMetricsInput {
    /// <p>The dimensions to filter against.</p>
    pub dimensions: Option<Vec<DimensionFilter>>,
    /// <p>The name of the metric to filter against.</p>
    pub metric_name: Option<String>,
    /// <p>The namespace to filter against.</p>
    pub namespace: Option<String>,
    /// <p>The token returned by a previous call to indicate that there is more data available.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListMetricsInput` contents to a `SignedRequest`.
struct ListMetricsInputSerializer;
impl ListMetricsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListMetricsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.dimensions {
            DimensionFiltersSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Dimensions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.metric_name {
            params.put(
                &format!("{}{}", prefix, "MetricName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.namespace {
            params.put(
                &format!("{}{}", prefix, "Namespace"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListMetricsOutput {
    /// <p>The metrics.</p>
    pub metrics: Option<Vec<Metric>>,
    /// <p>The token that marks the start of the next batch of returned results.</p>
    pub next_token: Option<String>,
}

struct ListMetricsOutputDeserializer;
impl ListMetricsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListMetricsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListMetricsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Metrics" => {
                        obj.metrics =
                            Some(try!(MetricsDeserializer::deserialize("Metrics", stack)));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MessageDeserializer;
impl MessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>A message returned by the <code>GetMetricData</code>API, including a code and a description.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MessageData {
    /// <p>The error code or status code associated with the message.</p>
    pub code: Option<String>,
    /// <p>The message text.</p>
    pub value: Option<String>,
}

struct MessageDataDeserializer;
impl MessageDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MessageData, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MessageData::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Code" => {
                        obj.code = Some(try!(MessageDataCodeDeserializer::deserialize(
                            "Code", stack
                        )));
                    }
                    "Value" => {
                        obj.value = Some(try!(MessageDataValueDeserializer::deserialize(
                            "Value", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MessageDataCodeDeserializer;
impl MessageDataCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MessageDataValueDeserializer;
impl MessageDataValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a specific metric.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Metric {
    /// <p>The dimensions for the metric.</p>
    pub dimensions: Option<Vec<Dimension>>,
    /// <p>The name of the metric.</p>
    pub metric_name: Option<String>,
    /// <p>The namespace of the metric.</p>
    pub namespace: Option<String>,
}

struct MetricDeserializer;
impl MetricDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Metric, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Metric::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Dimensions" => {
                        obj.dimensions = Some(try!(DimensionsDeserializer::deserialize(
                            "Dimensions",
                            stack
                        )));
                    }
                    "MetricName" => {
                        obj.metric_name = Some(try!(MetricNameDeserializer::deserialize(
                            "MetricName",
                            stack
                        )));
                    }
                    "Namespace" => {
                        obj.namespace =
                            Some(try!(NamespaceDeserializer::deserialize("Namespace", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `Metric` contents to a `SignedRequest`.
struct MetricSerializer;
impl MetricSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Metric) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.dimensions {
            DimensionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Dimensions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.metric_name {
            params.put(
                &format!("{}{}", prefix, "MetricName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.namespace {
            params.put(
                &format!("{}{}", prefix, "Namespace"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Represents an alarm.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricAlarm {
    /// <p>Indicates whether actions should be executed during any changes to the alarm state.</p>
    pub actions_enabled: Option<bool>,
    /// <p>The actions to execute when this alarm transitions to the <code>ALARM</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p>
    pub alarm_actions: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the alarm.</p>
    pub alarm_arn: Option<String>,
    /// <p>The time stamp of the last update to the alarm configuration.</p>
    pub alarm_configuration_updated_timestamp: Option<String>,
    /// <p>The description of the alarm.</p>
    pub alarm_description: Option<String>,
    /// <p>The name of the alarm.</p>
    pub alarm_name: Option<String>,
    /// <p>The arithmetic operation to use when comparing the specified statistic and threshold. The specified statistic value is used as the first operand.</p>
    pub comparison_operator: Option<String>,
    /// <p>The number of datapoints that must be breaching to trigger the alarm.</p>
    pub datapoints_to_alarm: Option<i64>,
    /// <p>The dimensions for the metric associated with the alarm.</p>
    pub dimensions: Option<Vec<Dimension>>,
    /// <p>Used only for alarms based on percentiles. If <code>ignore</code>, the alarm state does not change during periods with too few data points to be statistically significant. If <code>evaluate</code> or this parameter is not used, the alarm is always evaluated and possibly changes state no matter how many data points are available.</p>
    pub evaluate_low_sample_count_percentile: Option<String>,
    /// <p>The number of periods over which data is compared to the specified threshold.</p>
    pub evaluation_periods: Option<i64>,
    /// <p>The percentile statistic for the metric associated with the alarm. Specify a value between p0.0 and p100.</p>
    pub extended_statistic: Option<String>,
    /// <p>The actions to execute when this alarm transitions to the <code>INSUFFICIENT_DATA</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p>
    pub insufficient_data_actions: Option<Vec<String>>,
    /// <p>The name of the metric associated with the alarm.</p>
    pub metric_name: Option<String>,
    /// <p>The namespace of the metric associated with the alarm.</p>
    pub namespace: Option<String>,
    /// <p>The actions to execute when this alarm transitions to the <code>OK</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p>
    pub ok_actions: Option<Vec<String>>,
    /// <p>The period, in seconds, over which the statistic is applied.</p>
    pub period: Option<i64>,
    /// <p>An explanation for the alarm state, in text format.</p>
    pub state_reason: Option<String>,
    /// <p>An explanation for the alarm state, in JSON format.</p>
    pub state_reason_data: Option<String>,
    /// <p>The time stamp of the last update to the alarm state.</p>
    pub state_updated_timestamp: Option<String>,
    /// <p>The state value for the alarm.</p>
    pub state_value: Option<String>,
    /// <p>The statistic for the metric associated with the alarm, other than percentile. For percentile statistics, use <code>ExtendedStatistic</code>.</p>
    pub statistic: Option<String>,
    /// <p>The value to compare with the specified statistic.</p>
    pub threshold: Option<f64>,
    /// <p>Sets how this alarm is to handle missing data points. If this parameter is omitted, the default behavior of <code>missing</code> is used.</p>
    pub treat_missing_data: Option<String>,
    /// <p>The unit of the metric associated with the alarm.</p>
    pub unit: Option<String>,
}

struct MetricAlarmDeserializer;
impl MetricAlarmDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricAlarm, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MetricAlarm::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ActionsEnabled" => {
                        obj.actions_enabled = Some(try!(ActionsEnabledDeserializer::deserialize(
                            "ActionsEnabled",
                            stack
                        )));
                    }
                    "AlarmActions" => {
                        obj.alarm_actions = Some(try!(ResourceListDeserializer::deserialize(
                            "AlarmActions",
                            stack
                        )));
                    }
                    "AlarmArn" => {
                        obj.alarm_arn =
                            Some(try!(AlarmArnDeserializer::deserialize("AlarmArn", stack)));
                    }
                    "AlarmConfigurationUpdatedTimestamp" => {
                        obj.alarm_configuration_updated_timestamp =
                            Some(try!(TimestampDeserializer::deserialize(
                                "AlarmConfigurationUpdatedTimestamp",
                                stack
                            )));
                    }
                    "AlarmDescription" => {
                        obj.alarm_description = Some(try!(
                            AlarmDescriptionDeserializer::deserialize("AlarmDescription", stack)
                        ));
                    }
                    "AlarmName" => {
                        obj.alarm_name =
                            Some(try!(AlarmNameDeserializer::deserialize("AlarmName", stack)));
                    }
                    "ComparisonOperator" => {
                        obj.comparison_operator =
                            Some(try!(ComparisonOperatorDeserializer::deserialize(
                                "ComparisonOperator",
                                stack
                            )));
                    }
                    "DatapointsToAlarm" => {
                        obj.datapoints_to_alarm = Some(try!(
                            DatapointsToAlarmDeserializer::deserialize("DatapointsToAlarm", stack)
                        ));
                    }
                    "Dimensions" => {
                        obj.dimensions = Some(try!(DimensionsDeserializer::deserialize(
                            "Dimensions",
                            stack
                        )));
                    }
                    "EvaluateLowSampleCountPercentile" => {
                        obj.evaluate_low_sample_count_percentile = Some(try!(
                            EvaluateLowSampleCountPercentileDeserializer::deserialize(
                                "EvaluateLowSampleCountPercentile",
                                stack
                            )
                        ));
                    }
                    "EvaluationPeriods" => {
                        obj.evaluation_periods = Some(try!(
                            EvaluationPeriodsDeserializer::deserialize("EvaluationPeriods", stack)
                        ));
                    }
                    "ExtendedStatistic" => {
                        obj.extended_statistic = Some(try!(
                            ExtendedStatisticDeserializer::deserialize("ExtendedStatistic", stack)
                        ));
                    }
                    "InsufficientDataActions" => {
                        obj.insufficient_data_actions = Some(try!(
                            ResourceListDeserializer::deserialize("InsufficientDataActions", stack)
                        ));
                    }
                    "MetricName" => {
                        obj.metric_name = Some(try!(MetricNameDeserializer::deserialize(
                            "MetricName",
                            stack
                        )));
                    }
                    "Namespace" => {
                        obj.namespace =
                            Some(try!(NamespaceDeserializer::deserialize("Namespace", stack)));
                    }
                    "OKActions" => {
                        obj.ok_actions = Some(try!(ResourceListDeserializer::deserialize(
                            "OKActions",
                            stack
                        )));
                    }
                    "Period" => {
                        obj.period = Some(try!(PeriodDeserializer::deserialize("Period", stack)));
                    }
                    "StateReason" => {
                        obj.state_reason = Some(try!(StateReasonDeserializer::deserialize(
                            "StateReason",
                            stack
                        )));
                    }
                    "StateReasonData" => {
                        obj.state_reason_data = Some(try!(
                            StateReasonDataDeserializer::deserialize("StateReasonData", stack)
                        ));
                    }
                    "StateUpdatedTimestamp" => {
                        obj.state_updated_timestamp = Some(try!(
                            TimestampDeserializer::deserialize("StateUpdatedTimestamp", stack)
                        ));
                    }
                    "StateValue" => {
                        obj.state_value = Some(try!(StateValueDeserializer::deserialize(
                            "StateValue",
                            stack
                        )));
                    }
                    "Statistic" => {
                        obj.statistic =
                            Some(try!(StatisticDeserializer::deserialize("Statistic", stack)));
                    }
                    "Threshold" => {
                        obj.threshold =
                            Some(try!(ThresholdDeserializer::deserialize("Threshold", stack)));
                    }
                    "TreatMissingData" => {
                        obj.treat_missing_data = Some(try!(
                            TreatMissingDataDeserializer::deserialize("TreatMissingData", stack)
                        ));
                    }
                    "Unit" => {
                        obj.unit = Some(try!(StandardUnitDeserializer::deserialize("Unit", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MetricAlarmsDeserializer;
impl MetricAlarmsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricAlarm>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(MetricAlarmDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `MetricData` contents to a `SignedRequest`.
struct MetricDataSerializer;
impl MetricDataSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<MetricDatum>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            MetricDatumSerializer::serialize(params, &key, obj);
        }
    }
}

/// Serialize `MetricDataQueries` contents to a `SignedRequest`.
struct MetricDataQueriesSerializer;
impl MetricDataQueriesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<MetricDataQuery>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            MetricDataQuerySerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>This structure indicates the metric data to return, and whether this call is just retrieving a batch set of data for one metric, or is performing a math expression on metric data. A single <code>GetMetricData</code> call can include up to 100 <code>MetricDataQuery</code> structures.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricDataQuery {
    /// <p>The math expression to be performed on the returned data, if this structure is performing a math expression. For more information about metric math expressions, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/using-metric-math.html#metric-math-syntax">Metric Math Syntax and Functions</a> in the <i>Amazon CloudWatch User Guide</i>.</p> <p>Within one MetricDataQuery structure, you must specify either <code>Expression</code> or <code>MetricStat</code> but not both.</p>
    pub expression: Option<String>,
    /// <p>A short name used to tie this structure to the results in the response. This name must be unique within a single call to <code>GetMetricData</code>. If you are performing math expressions on this set of data, this name represents that data and can serve as a variable in the mathematical expression. The valid characters are letters, numbers, and underscore. The first character must be a lowercase letter.</p>
    pub id: String,
    /// <p>A human-readable label for this metric or expression. This is especially useful if this is an expression, so that you know what the value represents. If the metric or expression is shown in a CloudWatch dashboard widget, the label is shown. If Label is omitted, CloudWatch generates a default.</p>
    pub label: Option<String>,
    /// <p>The metric to be returned, along with statistics, period, and units. Use this parameter only if this structure is performing a data retrieval and not performing a math expression on the returned data.</p> <p>Within one MetricDataQuery structure, you must specify either <code>Expression</code> or <code>MetricStat</code> but not both.</p>
    pub metric_stat: Option<MetricStat>,
    /// <p>Indicates whether to return the time stamps and raw data values of this metric. If you are performing this call just to do math expressions and do not also need the raw data returned, you can specify <code>False</code>. If you omit this, the default of <code>True</code> is used.</p>
    pub return_data: Option<bool>,
}

/// Serialize `MetricDataQuery` contents to a `SignedRequest`.
struct MetricDataQuerySerializer;
impl MetricDataQuerySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &MetricDataQuery) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.expression {
            params.put(
                &format!("{}{}", prefix, "Expression"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(&format!("{}{}", prefix, "Id"), &obj.id.replace("+", "%2B"));
        if let Some(ref field_value) = obj.label {
            params.put(
                &format!("{}{}", prefix, "Label"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.metric_stat {
            MetricStatSerializer::serialize(
                params,
                &format!("{}{}", prefix, "MetricStat"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.return_data {
            params.put(
                &format!("{}{}", prefix, "ReturnData"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p>A <code>GetMetricData</code> call returns an array of <code>MetricDataResult</code> structures. Each of these structures includes the data points for that metric, along with the time stamps of those data points and other identifying information.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricDataResult {
    /// <p>The short name you specified to represent this metric.</p>
    pub id: Option<String>,
    /// <p>The human-readable label associated with the data.</p>
    pub label: Option<String>,
    /// <p>A list of messages with additional information about the data returned.</p>
    pub messages: Option<Vec<MessageData>>,
    /// <p>The status of the returned data. <code>Complete</code> indicates that all data points in the requested time range were returned. <code>PartialData</code> means that an incomplete set of data points were returned. You can use the <code>NextToken</code> value that was returned and repeat your request to get more data points. <code>NextToken</code> is not returned if you are performing a math expression. <code>InternalError</code> indicates that an error occurred. Retry your request using <code>NextToken</code>, if present.</p>
    pub status_code: Option<String>,
    /// <p>The time stamps for the data points, formatted in Unix timestamp format. The number of time stamps always matches the number of values and the value for Timestamps[x] is Values[x].</p>
    pub timestamps: Option<Vec<String>>,
    /// <p>The data points for the metric corresponding to <code>Timestamps</code>. The number of values always matches the number of time stamps and the time stamp for Values[x] is Timestamps[x].</p>
    pub values: Option<Vec<f64>>,
}

struct MetricDataResultDeserializer;
impl MetricDataResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricDataResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MetricDataResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Id" => {
                        obj.id = Some(try!(MetricIdDeserializer::deserialize("Id", stack)));
                    }
                    "Label" => {
                        obj.label =
                            Some(try!(MetricLabelDeserializer::deserialize("Label", stack)));
                    }
                    "Messages" => {
                        obj.messages = Some(try!(
                            MetricDataResultMessagesDeserializer::deserialize("Messages", stack)
                        ));
                    }
                    "StatusCode" => {
                        obj.status_code = Some(try!(StatusCodeDeserializer::deserialize(
                            "StatusCode",
                            stack
                        )));
                    }
                    "Timestamps" => {
                        obj.timestamps = Some(try!(TimestampsDeserializer::deserialize(
                            "Timestamps",
                            stack
                        )));
                    }
                    "Values" => {
                        obj.values = Some(try!(DatapointValuesDeserializer::deserialize(
                            "Values", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MetricDataResultMessagesDeserializer;
impl MetricDataResultMessagesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MessageData>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(MessageDataDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct MetricDataResultsDeserializer;
impl MetricDataResultsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricDataResult>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(MetricDataResultDeserializer::deserialize(
                            "member", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
/// <p>Encapsulates the information sent to either create a metric or add new values to be aggregated into an existing metric.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricDatum {
    /// <p>The dimensions associated with the metric.</p>
    pub dimensions: Option<Vec<Dimension>>,
    /// <p>The name of the metric.</p>
    pub metric_name: String,
    /// <p>The statistical values for the metric.</p>
    pub statistic_values: Option<StatisticSet>,
    /// <p>Valid values are 1 and 60. Setting this to 1 specifies this metric as a high-resolution metric, so that CloudWatch stores the metric with sub-minute resolution down to one second. Setting this to 60 specifies this metric as a regular-resolution metric, which CloudWatch stores at 1-minute resolution. Currently, high resolution is available only for custom metrics. For more information about high-resolution metrics, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html#high-resolution-metrics">High-Resolution Metrics</a> in the <i>Amazon CloudWatch User Guide</i>. </p> <p>This field is optional, if you do not specify it the default of 60 is used.</p>
    pub storage_resolution: Option<i64>,
    /// <p>The time the metric data was received, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>
    pub timestamp: Option<String>,
    /// <p>The unit of the metric.</p>
    pub unit: Option<String>,
    /// <p>The value for the metric.</p> <p>Although the parameter accepts numbers of type Double, CloudWatch rejects values that are either too small or too large. Values must be in the range of 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2). In addition, special values (for example, NaN, +Infinity, -Infinity) are not supported.</p>
    pub value: Option<f64>,
}

/// Serialize `MetricDatum` contents to a `SignedRequest`.
struct MetricDatumSerializer;
impl MetricDatumSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &MetricDatum) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.dimensions {
            DimensionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Dimensions"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "MetricName"),
            &obj.metric_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.statistic_values {
            StatisticSetSerializer::serialize(
                params,
                &format!("{}{}", prefix, "StatisticValues"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.storage_resolution {
            params.put(
                &format!("{}{}", prefix, "StorageResolution"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.timestamp {
            params.put(
                &format!("{}{}", prefix, "Timestamp"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.unit {
            params.put(
                &format!("{}{}", prefix, "Unit"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.value {
            params.put(
                &format!("{}{}", prefix, "Value"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

struct MetricIdDeserializer;
impl MetricIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MetricLabelDeserializer;
impl MetricLabelDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct MetricNameDeserializer;
impl MetricNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>This structure defines the metric to be returned, along with the statistics, period, and units.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricStat {
    /// <p>The metric to return, including the metric name, namespace, and dimensions.</p>
    pub metric: Metric,
    /// <p>The period to use when retrieving the metric.</p>
    pub period: i64,
    /// <p>The statistic to return. It can include any CloudWatch statistic or extended statistic.</p>
    pub stat: String,
    /// <p>The unit to use for the returned data points.</p>
    pub unit: Option<String>,
}

/// Serialize `MetricStat` contents to a `SignedRequest`.
struct MetricStatSerializer;
impl MetricStatSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &MetricStat) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        MetricSerializer::serialize(params, &format!("{}{}", prefix, "Metric"), &obj.metric);
        params.put(
            &format!("{}{}", prefix, "Period"),
            &obj.period.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Stat"),
            &obj.stat.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.unit {
            params.put(
                &format!("{}{}", prefix, "Unit"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct MetricsDeserializer;
impl MetricsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Metric>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(MetricDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct NamespaceDeserializer;
impl NamespaceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct NextTokenDeserializer;
impl NextTokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct PeriodDeserializer;
impl PeriodDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutDashboardInput {
    /// <p>The detailed information about the dashboard in JSON format, including the widgets to include and their location on the dashboard. This parameter is required.</p> <p>For more information about the syntax, see <a>CloudWatch-Dashboard-Body-Structure</a>.</p>
    pub dashboard_body: String,
    /// <p>The name of the dashboard. If a dashboard with this name already exists, this call modifies that dashboard, replacing its current contents. Otherwise, a new dashboard is created. The maximum length is 255, and valid characters are A-Z, a-z, 0-9, "-", and "_". This parameter is required.</p>
    pub dashboard_name: String,
}

/// Serialize `PutDashboardInput` contents to a `SignedRequest`.
struct PutDashboardInputSerializer;
impl PutDashboardInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PutDashboardInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DashboardBody"),
            &obj.dashboard_body.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "DashboardName"),
            &obj.dashboard_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutDashboardOutput {
    /// <p>If the input for <code>PutDashboard</code> was correct and the dashboard was successfully created or modified, this result is empty.</p> <p>If this result includes only warning messages, then the input was valid enough for the dashboard to be created or modified, but some elements of the dashboard may not render.</p> <p>If this result includes error messages, the input was not valid and the operation failed.</p>
    pub dashboard_validation_messages: Option<Vec<DashboardValidationMessage>>,
}

struct PutDashboardOutputDeserializer;
impl PutDashboardOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutDashboardOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PutDashboardOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "DashboardValidationMessages" => {
                        obj.dashboard_validation_messages =
                            Some(try!(DashboardValidationMessagesDeserializer::deserialize(
                                "DashboardValidationMessages",
                                stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutMetricAlarmInput {
    /// <p>Indicates whether actions should be executed during any changes to the alarm state.</p>
    pub actions_enabled: Option<bool>,
    /// <p>The actions to execute when this alarm transitions to the <code>ALARM</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p> <p>Valid Values: arn:aws:automate:<i>region</i>:ec2:stop | arn:aws:automate:<i>region</i>:ec2:terminate | arn:aws:automate:<i>region</i>:ec2:recover | arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i> | arn:aws:autoscaling:<i>region</i>:<i>account-id</i>:scalingPolicy:<i>policy-id</i> autoScalingGroupName/<i>group-friendly-name</i>:policyName/<i>policy-friendly-name</i> </p> <p>Valid Values (for use with IAM roles): arn:aws:swf:<i>region</i>:{<i>account-id</i>}:action/actions/AWS_EC2.InstanceId.Stop/1.0 | arn:aws:swf:<i>region</i>:{<i>account-id</i>}:action/actions/AWS_EC2.InstanceId.Terminate/1.0 | arn:aws:swf:<i>region</i>:{<i>account-id</i>}:action/actions/AWS_EC2.InstanceId.Reboot/1.0</p>
    pub alarm_actions: Option<Vec<String>>,
    /// <p>The description for the alarm.</p>
    pub alarm_description: Option<String>,
    /// <p>The name for the alarm. This name must be unique within the AWS account.</p>
    pub alarm_name: String,
    /// <p> The arithmetic operation to use when comparing the specified statistic and threshold. The specified statistic value is used as the first operand.</p>
    pub comparison_operator: String,
    /// <p>The number of datapoints that must be breaching to trigger the alarm. This is used only if you are setting an "M out of N" alarm. In that case, this value is the M. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#alarm-evaluation">Evaluating an Alarm</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
    pub datapoints_to_alarm: Option<i64>,
    /// <p>The dimensions for the metric associated with the alarm.</p>
    pub dimensions: Option<Vec<Dimension>>,
    /// <p> Used only for alarms based on percentiles. If you specify <code>ignore</code>, the alarm state does not change during periods with too few data points to be statistically significant. If you specify <code>evaluate</code> or omit this parameter, the alarm is always evaluated and possibly changes state no matter how many data points are available. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#percentiles-with-low-samples">Percentile-Based CloudWatch Alarms and Low Data Samples</a>.</p> <p>Valid Values: <code>evaluate | ignore</code> </p>
    pub evaluate_low_sample_count_percentile: Option<String>,
    /// <p>The number of periods over which data is compared to the specified threshold. If you are setting an alarm which requires that a number of consecutive data points be breaching to trigger the alarm, this value specifies that number. If you are setting an "M out of N" alarm, this value is the N.</p> <p>An alarm's total current evaluation period can be no longer than one day, so this number multiplied by <code>Period</code> cannot be more than 86,400 seconds.</p>
    pub evaluation_periods: i64,
    /// <p>The percentile statistic for the metric associated with the alarm. Specify a value between p0.0 and p100. When you call <code>PutMetricAlarm</code>, you must specify either <code>Statistic</code> or <code>ExtendedStatistic,</code> but not both.</p>
    pub extended_statistic: Option<String>,
    /// <p>The actions to execute when this alarm transitions to the <code>INSUFFICIENT_DATA</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p> <p>Valid Values: arn:aws:automate:<i>region</i>:ec2:stop | arn:aws:automate:<i>region</i>:ec2:terminate | arn:aws:automate:<i>region</i>:ec2:recover | arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i> | arn:aws:autoscaling:<i>region</i>:<i>account-id</i>:scalingPolicy:<i>policy-id</i> autoScalingGroupName/<i>group-friendly-name</i>:policyName/<i>policy-friendly-name</i> </p> <p>Valid Values (for use with IAM roles): arn:aws:swf:<i>region</i>:{<i>account-id</i>}:action/actions/AWS_EC2.InstanceId.Stop/1.0 | arn:aws:swf:<i>region</i>:{<i>account-id</i>}:action/actions/AWS_EC2.InstanceId.Terminate/1.0 | arn:aws:swf:<i>region</i>:{<i>account-id</i>}:action/actions/AWS_EC2.InstanceId.Reboot/1.0</p>
    pub insufficient_data_actions: Option<Vec<String>>,
    /// <p>The name for the metric associated with the alarm.</p>
    pub metric_name: String,
    /// <p>The namespace for the metric associated with the alarm.</p>
    pub namespace: String,
    /// <p>The actions to execute when this alarm transitions to an <code>OK</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p> <p>Valid Values: arn:aws:automate:<i>region</i>:ec2:stop | arn:aws:automate:<i>region</i>:ec2:terminate | arn:aws:automate:<i>region</i>:ec2:recover | arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i> | arn:aws:autoscaling:<i>region</i>:<i>account-id</i>:scalingPolicy:<i>policy-id</i> autoScalingGroupName/<i>group-friendly-name</i>:policyName/<i>policy-friendly-name</i> </p> <p>Valid Values (for use with IAM roles): arn:aws:swf:<i>region</i>:{<i>account-id</i>}:action/actions/AWS_EC2.InstanceId.Stop/1.0 | arn:aws:swf:<i>region</i>:{<i>account-id</i>}:action/actions/AWS_EC2.InstanceId.Terminate/1.0 | arn:aws:swf:<i>region</i>:{<i>account-id</i>}:action/actions/AWS_EC2.InstanceId.Reboot/1.0</p>
    pub ok_actions: Option<Vec<String>>,
    /// <p>The period, in seconds, over which the specified statistic is applied. Valid values are 10, 30, and any multiple of 60.</p> <p>Be sure to specify 10 or 30 only for metrics that are stored by a <code>PutMetricData</code> call with a <code>StorageResolution</code> of 1. If you specify a period of 10 or 30 for a metric that does not have sub-minute resolution, the alarm still attempts to gather data at the period rate that you specify. In this case, it does not receive data for the attempts that do not correspond to a one-minute data resolution, and the alarm may often lapse into INSUFFICENT_DATA status. Specifying 10 or 30 also sets this alarm as a high-resolution alarm, which has a higher charge than other alarms. For more information about pricing, see <a href="https://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> <p>An alarm's total current evaluation period can be no longer than one day, so <code>Period</code> multiplied by <code>EvaluationPeriods</code> cannot be more than 86,400 seconds.</p>
    pub period: i64,
    /// <p>The statistic for the metric associated with the alarm, other than percentile. For percentile statistics, use <code>ExtendedStatistic</code>. When you call <code>PutMetricAlarm</code>, you must specify either <code>Statistic</code> or <code>ExtendedStatistic,</code> but not both.</p>
    pub statistic: Option<String>,
    /// <p>The value against which the specified statistic is compared.</p>
    pub threshold: f64,
    /// <p> Sets how this alarm is to handle missing data points. If <code>TreatMissingData</code> is omitted, the default behavior of <code>missing</code> is used. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#alarms-and-missing-data">Configuring How CloudWatch Alarms Treats Missing Data</a>.</p> <p>Valid Values: <code>breaching | notBreaching | ignore | missing</code> </p>
    pub treat_missing_data: Option<String>,
    /// <p>The unit of measure for the statistic. For example, the units for the Amazon EC2 NetworkIn metric are Bytes because NetworkIn tracks the number of bytes that an instance receives on all network interfaces. You can also specify a unit when you create a custom metric. Units help provide conceptual meaning to your data. Metric data points that specify a unit of measure, such as Percent, are aggregated separately.</p> <p>If you specify a unit, you must use a unit that is appropriate for the metric. Otherwise, the CloudWatch alarm can get stuck in the <code>INSUFFICIENT DATA</code> state. </p>
    pub unit: Option<String>,
}

/// Serialize `PutMetricAlarmInput` contents to a `SignedRequest`.
struct PutMetricAlarmInputSerializer;
impl PutMetricAlarmInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PutMetricAlarmInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.actions_enabled {
            params.put(
                &format!("{}{}", prefix, "ActionsEnabled"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.alarm_actions {
            ResourceListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AlarmActions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.alarm_description {
            params.put(
                &format!("{}{}", prefix, "AlarmDescription"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "AlarmName"),
            &obj.alarm_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "ComparisonOperator"),
            &obj.comparison_operator.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.datapoints_to_alarm {
            params.put(
                &format!("{}{}", prefix, "DatapointsToAlarm"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.dimensions {
            DimensionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Dimensions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.evaluate_low_sample_count_percentile {
            params.put(
                &format!("{}{}", prefix, "EvaluateLowSampleCountPercentile"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "EvaluationPeriods"),
            &obj.evaluation_periods.to_string().replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.extended_statistic {
            params.put(
                &format!("{}{}", prefix, "ExtendedStatistic"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.insufficient_data_actions {
            ResourceListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "InsufficientDataActions"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "MetricName"),
            &obj.metric_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Namespace"),
            &obj.namespace.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.ok_actions {
            ResourceListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OKActions"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "Period"),
            &obj.period.to_string().replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.statistic {
            params.put(
                &format!("{}{}", prefix, "Statistic"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Threshold"),
            &obj.threshold.to_string().replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.treat_missing_data {
            params.put(
                &format!("{}{}", prefix, "TreatMissingData"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.unit {
            params.put(
                &format!("{}{}", prefix, "Unit"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutMetricDataInput {
    /// <p>The data for the metric.</p>
    pub metric_data: Vec<MetricDatum>,
    /// <p>The namespace for the metric data.</p> <p>You cannot specify a namespace that begins with "AWS/". Namespaces that begin with "AWS/" are reserved for use by Amazon Web Services products.</p>
    pub namespace: String,
}

/// Serialize `PutMetricDataInput` contents to a `SignedRequest`.
struct PutMetricDataInputSerializer;
impl PutMetricDataInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PutMetricDataInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        MetricDataSerializer::serialize(
            params,
            &format!("{}{}", prefix, "MetricData"),
            &obj.metric_data,
        );
        params.put(
            &format!("{}{}", prefix, "Namespace"),
            &obj.namespace.replace("+", "%2B"),
        );
    }
}

struct ResourceListDeserializer;
impl ResourceListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(ResourceNameDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `ResourceList` contents to a `SignedRequest`.
struct ResourceListSerializer;
impl ResourceListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct ResourceNameDeserializer;
impl ResourceNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetAlarmStateInput {
    /// <p>The name for the alarm. This name must be unique within the AWS account. The maximum length is 255 characters.</p>
    pub alarm_name: String,
    /// <p>The reason that this alarm is set to this specific state, in text format.</p>
    pub state_reason: String,
    /// <p>The reason that this alarm is set to this specific state, in JSON format.</p>
    pub state_reason_data: Option<String>,
    /// <p>The value of the state.</p>
    pub state_value: String,
}

/// Serialize `SetAlarmStateInput` contents to a `SignedRequest`.
struct SetAlarmStateInputSerializer;
impl SetAlarmStateInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetAlarmStateInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "AlarmName"),
            &obj.alarm_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "StateReason"),
            &obj.state_reason.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.state_reason_data {
            params.put(
                &format!("{}{}", prefix, "StateReasonData"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "StateValue"),
            &obj.state_value.replace("+", "%2B"),
        );
    }
}

struct SizeDeserializer;
impl SizeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct StandardUnitDeserializer;
impl StandardUnitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct StateReasonDeserializer;
impl StateReasonDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct StateReasonDataDeserializer;
impl StateReasonDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct StateValueDeserializer;
impl StateValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct StatisticDeserializer;
impl StatisticDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Represents a set of statistics that describes a specific metric. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StatisticSet {
    /// <p>The maximum value of the sample set.</p>
    pub maximum: f64,
    /// <p>The minimum value of the sample set.</p>
    pub minimum: f64,
    /// <p>The number of samples used for the statistic set.</p>
    pub sample_count: f64,
    /// <p>The sum of values for the sample set.</p>
    pub sum: f64,
}

/// Serialize `StatisticSet` contents to a `SignedRequest`.
struct StatisticSetSerializer;
impl StatisticSetSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &StatisticSet) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "Maximum"),
            &obj.maximum.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Minimum"),
            &obj.minimum.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "SampleCount"),
            &obj.sample_count.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Sum"),
            &obj.sum.to_string().replace("+", "%2B"),
        );
    }
}

/// Serialize `Statistics` contents to a `SignedRequest`.
struct StatisticsSerializer;
impl StatisticsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct StatusCodeDeserializer;
impl StatusCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ThresholdDeserializer;
impl ThresholdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<f64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct TimestampDeserializer;
impl TimestampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct TimestampsDeserializer;
impl TimestampsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(TimestampDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct TreatMissingDataDeserializer;
impl TreatMissingDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// Errors returned by DeleteAlarms
#[derive(Debug, PartialEq)]
pub enum DeleteAlarmsError {
    /// <p>The named resource does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteAlarmsError {
    pub fn from_body(body: &str) -> DeleteAlarmsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceNotFound" => {
                    DeleteAlarmsError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => DeleteAlarmsError::Unknown(String::from(body)),
            },
            Err(_) => DeleteAlarmsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteAlarmsError {
    fn from(err: XmlParseError) -> DeleteAlarmsError {
        let XmlParseError(message) = err;
        DeleteAlarmsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteAlarmsError {
    fn from(err: CredentialsError) -> DeleteAlarmsError {
        DeleteAlarmsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAlarmsError {
    fn from(err: HttpDispatchError) -> DeleteAlarmsError {
        DeleteAlarmsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAlarmsError {
    fn from(err: io::Error) -> DeleteAlarmsError {
        DeleteAlarmsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAlarmsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAlarmsError {
    fn description(&self) -> &str {
        match *self {
            DeleteAlarmsError::ResourceNotFound(ref cause) => cause,
            DeleteAlarmsError::Validation(ref cause) => cause,
            DeleteAlarmsError::Credentials(ref err) => err.description(),
            DeleteAlarmsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAlarmsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDashboards
#[derive(Debug, PartialEq)]
pub enum DeleteDashboardsError {
    /// <p>The specified dashboard does not exist.</p>
    DashboardNotFoundError(String),
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteDashboardsError {
    pub fn from_body(body: &str) -> DeleteDashboardsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceNotFound" => DeleteDashboardsError::DashboardNotFoundError(String::from(
                    parsed_error.message,
                )),
                "InternalServiceError" => {
                    DeleteDashboardsError::InternalServiceFault(String::from(parsed_error.message))
                }
                "InvalidParameterValue" => {
                    DeleteDashboardsError::InvalidParameterValue(String::from(parsed_error.message))
                }
                _ => DeleteDashboardsError::Unknown(String::from(body)),
            },
            Err(_) => DeleteDashboardsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteDashboardsError {
    fn from(err: XmlParseError) -> DeleteDashboardsError {
        let XmlParseError(message) = err;
        DeleteDashboardsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteDashboardsError {
    fn from(err: CredentialsError) -> DeleteDashboardsError {
        DeleteDashboardsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDashboardsError {
    fn from(err: HttpDispatchError) -> DeleteDashboardsError {
        DeleteDashboardsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDashboardsError {
    fn from(err: io::Error) -> DeleteDashboardsError {
        DeleteDashboardsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDashboardsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDashboardsError {
    fn description(&self) -> &str {
        match *self {
            DeleteDashboardsError::DashboardNotFoundError(ref cause) => cause,
            DeleteDashboardsError::InternalServiceFault(ref cause) => cause,
            DeleteDashboardsError::InvalidParameterValue(ref cause) => cause,
            DeleteDashboardsError::Validation(ref cause) => cause,
            DeleteDashboardsError::Credentials(ref err) => err.description(),
            DeleteDashboardsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDashboardsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAlarmHistory
#[derive(Debug, PartialEq)]
pub enum DescribeAlarmHistoryError {
    /// <p>The next token specified is invalid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAlarmHistoryError {
    pub fn from_body(body: &str) -> DescribeAlarmHistoryError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidNextToken" => {
                    DescribeAlarmHistoryError::InvalidNextToken(String::from(parsed_error.message))
                }
                _ => DescribeAlarmHistoryError::Unknown(String::from(body)),
            },
            Err(_) => DescribeAlarmHistoryError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeAlarmHistoryError {
    fn from(err: XmlParseError) -> DescribeAlarmHistoryError {
        let XmlParseError(message) = err;
        DescribeAlarmHistoryError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeAlarmHistoryError {
    fn from(err: CredentialsError) -> DescribeAlarmHistoryError {
        DescribeAlarmHistoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAlarmHistoryError {
    fn from(err: HttpDispatchError) -> DescribeAlarmHistoryError {
        DescribeAlarmHistoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAlarmHistoryError {
    fn from(err: io::Error) -> DescribeAlarmHistoryError {
        DescribeAlarmHistoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAlarmHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAlarmHistoryError {
    fn description(&self) -> &str {
        match *self {
            DescribeAlarmHistoryError::InvalidNextToken(ref cause) => cause,
            DescribeAlarmHistoryError::Validation(ref cause) => cause,
            DescribeAlarmHistoryError::Credentials(ref err) => err.description(),
            DescribeAlarmHistoryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAlarmHistoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAlarms
#[derive(Debug, PartialEq)]
pub enum DescribeAlarmsError {
    /// <p>The next token specified is invalid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAlarmsError {
    pub fn from_body(body: &str) -> DescribeAlarmsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidNextToken" => {
                    DescribeAlarmsError::InvalidNextToken(String::from(parsed_error.message))
                }
                _ => DescribeAlarmsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeAlarmsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeAlarmsError {
    fn from(err: XmlParseError) -> DescribeAlarmsError {
        let XmlParseError(message) = err;
        DescribeAlarmsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeAlarmsError {
    fn from(err: CredentialsError) -> DescribeAlarmsError {
        DescribeAlarmsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAlarmsError {
    fn from(err: HttpDispatchError) -> DescribeAlarmsError {
        DescribeAlarmsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAlarmsError {
    fn from(err: io::Error) -> DescribeAlarmsError {
        DescribeAlarmsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAlarmsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAlarmsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAlarmsError::InvalidNextToken(ref cause) => cause,
            DescribeAlarmsError::Validation(ref cause) => cause,
            DescribeAlarmsError::Credentials(ref err) => err.description(),
            DescribeAlarmsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeAlarmsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAlarmsForMetric
#[derive(Debug, PartialEq)]
pub enum DescribeAlarmsForMetricError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAlarmsForMetricError {
    pub fn from_body(body: &str) -> DescribeAlarmsForMetricError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeAlarmsForMetricError::Unknown(String::from(body)),
            },
            Err(_) => DescribeAlarmsForMetricError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DescribeAlarmsForMetricError {
    fn from(err: XmlParseError) -> DescribeAlarmsForMetricError {
        let XmlParseError(message) = err;
        DescribeAlarmsForMetricError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeAlarmsForMetricError {
    fn from(err: CredentialsError) -> DescribeAlarmsForMetricError {
        DescribeAlarmsForMetricError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAlarmsForMetricError {
    fn from(err: HttpDispatchError) -> DescribeAlarmsForMetricError {
        DescribeAlarmsForMetricError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAlarmsForMetricError {
    fn from(err: io::Error) -> DescribeAlarmsForMetricError {
        DescribeAlarmsForMetricError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAlarmsForMetricError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAlarmsForMetricError {
    fn description(&self) -> &str {
        match *self {
            DescribeAlarmsForMetricError::Validation(ref cause) => cause,
            DescribeAlarmsForMetricError::Credentials(ref err) => err.description(),
            DescribeAlarmsForMetricError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAlarmsForMetricError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableAlarmActions
#[derive(Debug, PartialEq)]
pub enum DisableAlarmActionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisableAlarmActionsError {
    pub fn from_body(body: &str) -> DisableAlarmActionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DisableAlarmActionsError::Unknown(String::from(body)),
            },
            Err(_) => DisableAlarmActionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DisableAlarmActionsError {
    fn from(err: XmlParseError) -> DisableAlarmActionsError {
        let XmlParseError(message) = err;
        DisableAlarmActionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DisableAlarmActionsError {
    fn from(err: CredentialsError) -> DisableAlarmActionsError {
        DisableAlarmActionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableAlarmActionsError {
    fn from(err: HttpDispatchError) -> DisableAlarmActionsError {
        DisableAlarmActionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableAlarmActionsError {
    fn from(err: io::Error) -> DisableAlarmActionsError {
        DisableAlarmActionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableAlarmActionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableAlarmActionsError {
    fn description(&self) -> &str {
        match *self {
            DisableAlarmActionsError::Validation(ref cause) => cause,
            DisableAlarmActionsError::Credentials(ref err) => err.description(),
            DisableAlarmActionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisableAlarmActionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableAlarmActions
#[derive(Debug, PartialEq)]
pub enum EnableAlarmActionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnableAlarmActionsError {
    pub fn from_body(body: &str) -> EnableAlarmActionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => EnableAlarmActionsError::Unknown(String::from(body)),
            },
            Err(_) => EnableAlarmActionsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for EnableAlarmActionsError {
    fn from(err: XmlParseError) -> EnableAlarmActionsError {
        let XmlParseError(message) = err;
        EnableAlarmActionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for EnableAlarmActionsError {
    fn from(err: CredentialsError) -> EnableAlarmActionsError {
        EnableAlarmActionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableAlarmActionsError {
    fn from(err: HttpDispatchError) -> EnableAlarmActionsError {
        EnableAlarmActionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableAlarmActionsError {
    fn from(err: io::Error) -> EnableAlarmActionsError {
        EnableAlarmActionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableAlarmActionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableAlarmActionsError {
    fn description(&self) -> &str {
        match *self {
            EnableAlarmActionsError::Validation(ref cause) => cause,
            EnableAlarmActionsError::Credentials(ref err) => err.description(),
            EnableAlarmActionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EnableAlarmActionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDashboard
#[derive(Debug, PartialEq)]
pub enum GetDashboardError {
    /// <p>The specified dashboard does not exist.</p>
    DashboardNotFoundError(String),
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDashboardError {
    pub fn from_body(body: &str) -> GetDashboardError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ResourceNotFound" => {
                    GetDashboardError::DashboardNotFoundError(String::from(parsed_error.message))
                }
                "InternalServiceError" => {
                    GetDashboardError::InternalServiceFault(String::from(parsed_error.message))
                }
                "InvalidParameterValue" => {
                    GetDashboardError::InvalidParameterValue(String::from(parsed_error.message))
                }
                _ => GetDashboardError::Unknown(String::from(body)),
            },
            Err(_) => GetDashboardError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetDashboardError {
    fn from(err: XmlParseError) -> GetDashboardError {
        let XmlParseError(message) = err;
        GetDashboardError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetDashboardError {
    fn from(err: CredentialsError) -> GetDashboardError {
        GetDashboardError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDashboardError {
    fn from(err: HttpDispatchError) -> GetDashboardError {
        GetDashboardError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDashboardError {
    fn from(err: io::Error) -> GetDashboardError {
        GetDashboardError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDashboardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDashboardError {
    fn description(&self) -> &str {
        match *self {
            GetDashboardError::DashboardNotFoundError(ref cause) => cause,
            GetDashboardError::InternalServiceFault(ref cause) => cause,
            GetDashboardError::InvalidParameterValue(ref cause) => cause,
            GetDashboardError::Validation(ref cause) => cause,
            GetDashboardError::Credentials(ref err) => err.description(),
            GetDashboardError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDashboardError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMetricData
#[derive(Debug, PartialEq)]
pub enum GetMetricDataError {
    /// <p>The next token specified is invalid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetMetricDataError {
    pub fn from_body(body: &str) -> GetMetricDataError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidNextToken" => {
                    GetMetricDataError::InvalidNextToken(String::from(parsed_error.message))
                }
                _ => GetMetricDataError::Unknown(String::from(body)),
            },
            Err(_) => GetMetricDataError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetMetricDataError {
    fn from(err: XmlParseError) -> GetMetricDataError {
        let XmlParseError(message) = err;
        GetMetricDataError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetMetricDataError {
    fn from(err: CredentialsError) -> GetMetricDataError {
        GetMetricDataError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMetricDataError {
    fn from(err: HttpDispatchError) -> GetMetricDataError {
        GetMetricDataError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMetricDataError {
    fn from(err: io::Error) -> GetMetricDataError {
        GetMetricDataError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMetricDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMetricDataError {
    fn description(&self) -> &str {
        match *self {
            GetMetricDataError::InvalidNextToken(ref cause) => cause,
            GetMetricDataError::Validation(ref cause) => cause,
            GetMetricDataError::Credentials(ref err) => err.description(),
            GetMetricDataError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetMetricDataError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMetricStatistics
#[derive(Debug, PartialEq)]
pub enum GetMetricStatisticsError {
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>Parameters were used together that cannot be used together.</p>
    InvalidParameterCombination(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
    /// <p>An input parameter that is required is missing.</p>
    MissingRequiredParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetMetricStatisticsError {
    pub fn from_body(body: &str) -> GetMetricStatisticsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InternalServiceError" => GetMetricStatisticsError::InternalServiceFault(
                    String::from(parsed_error.message),
                ),
                "InvalidParameterCombination" => {
                    GetMetricStatisticsError::InvalidParameterCombination(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidParameterValue" => GetMetricStatisticsError::InvalidParameterValue(
                    String::from(parsed_error.message),
                ),
                "MissingParameter" => GetMetricStatisticsError::MissingRequiredParameter(
                    String::from(parsed_error.message),
                ),
                _ => GetMetricStatisticsError::Unknown(String::from(body)),
            },
            Err(_) => GetMetricStatisticsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetMetricStatisticsError {
    fn from(err: XmlParseError) -> GetMetricStatisticsError {
        let XmlParseError(message) = err;
        GetMetricStatisticsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetMetricStatisticsError {
    fn from(err: CredentialsError) -> GetMetricStatisticsError {
        GetMetricStatisticsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMetricStatisticsError {
    fn from(err: HttpDispatchError) -> GetMetricStatisticsError {
        GetMetricStatisticsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMetricStatisticsError {
    fn from(err: io::Error) -> GetMetricStatisticsError {
        GetMetricStatisticsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMetricStatisticsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMetricStatisticsError {
    fn description(&self) -> &str {
        match *self {
            GetMetricStatisticsError::InternalServiceFault(ref cause) => cause,
            GetMetricStatisticsError::InvalidParameterCombination(ref cause) => cause,
            GetMetricStatisticsError::InvalidParameterValue(ref cause) => cause,
            GetMetricStatisticsError::MissingRequiredParameter(ref cause) => cause,
            GetMetricStatisticsError::Validation(ref cause) => cause,
            GetMetricStatisticsError::Credentials(ref err) => err.description(),
            GetMetricStatisticsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetMetricStatisticsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDashboards
#[derive(Debug, PartialEq)]
pub enum ListDashboardsError {
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDashboardsError {
    pub fn from_body(body: &str) -> ListDashboardsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InternalServiceError" => {
                    ListDashboardsError::InternalServiceFault(String::from(parsed_error.message))
                }
                "InvalidParameterValue" => {
                    ListDashboardsError::InvalidParameterValue(String::from(parsed_error.message))
                }
                _ => ListDashboardsError::Unknown(String::from(body)),
            },
            Err(_) => ListDashboardsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListDashboardsError {
    fn from(err: XmlParseError) -> ListDashboardsError {
        let XmlParseError(message) = err;
        ListDashboardsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListDashboardsError {
    fn from(err: CredentialsError) -> ListDashboardsError {
        ListDashboardsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDashboardsError {
    fn from(err: HttpDispatchError) -> ListDashboardsError {
        ListDashboardsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDashboardsError {
    fn from(err: io::Error) -> ListDashboardsError {
        ListDashboardsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDashboardsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDashboardsError {
    fn description(&self) -> &str {
        match *self {
            ListDashboardsError::InternalServiceFault(ref cause) => cause,
            ListDashboardsError::InvalidParameterValue(ref cause) => cause,
            ListDashboardsError::Validation(ref cause) => cause,
            ListDashboardsError::Credentials(ref err) => err.description(),
            ListDashboardsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDashboardsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListMetrics
#[derive(Debug, PartialEq)]
pub enum ListMetricsError {
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListMetricsError {
    pub fn from_body(body: &str) -> ListMetricsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InternalServiceError" => {
                    ListMetricsError::InternalServiceFault(String::from(parsed_error.message))
                }
                "InvalidParameterValue" => {
                    ListMetricsError::InvalidParameterValue(String::from(parsed_error.message))
                }
                _ => ListMetricsError::Unknown(String::from(body)),
            },
            Err(_) => ListMetricsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListMetricsError {
    fn from(err: XmlParseError) -> ListMetricsError {
        let XmlParseError(message) = err;
        ListMetricsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListMetricsError {
    fn from(err: CredentialsError) -> ListMetricsError {
        ListMetricsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListMetricsError {
    fn from(err: HttpDispatchError) -> ListMetricsError {
        ListMetricsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListMetricsError {
    fn from(err: io::Error) -> ListMetricsError {
        ListMetricsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListMetricsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListMetricsError {
    fn description(&self) -> &str {
        match *self {
            ListMetricsError::InternalServiceFault(ref cause) => cause,
            ListMetricsError::InvalidParameterValue(ref cause) => cause,
            ListMetricsError::Validation(ref cause) => cause,
            ListMetricsError::Credentials(ref err) => err.description(),
            ListMetricsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListMetricsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutDashboard
#[derive(Debug, PartialEq)]
pub enum PutDashboardError {
    /// <p>Some part of the dashboard data is invalid.</p>
    DashboardInvalidInputError(String),
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutDashboardError {
    pub fn from_body(body: &str) -> PutDashboardError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidParameterInput" => PutDashboardError::DashboardInvalidInputError(
                    String::from(parsed_error.message),
                ),
                "InternalServiceError" => {
                    PutDashboardError::InternalServiceFault(String::from(parsed_error.message))
                }
                _ => PutDashboardError::Unknown(String::from(body)),
            },
            Err(_) => PutDashboardError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutDashboardError {
    fn from(err: XmlParseError) -> PutDashboardError {
        let XmlParseError(message) = err;
        PutDashboardError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutDashboardError {
    fn from(err: CredentialsError) -> PutDashboardError {
        PutDashboardError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutDashboardError {
    fn from(err: HttpDispatchError) -> PutDashboardError {
        PutDashboardError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutDashboardError {
    fn from(err: io::Error) -> PutDashboardError {
        PutDashboardError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutDashboardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutDashboardError {
    fn description(&self) -> &str {
        match *self {
            PutDashboardError::DashboardInvalidInputError(ref cause) => cause,
            PutDashboardError::InternalServiceFault(ref cause) => cause,
            PutDashboardError::Validation(ref cause) => cause,
            PutDashboardError::Credentials(ref err) => err.description(),
            PutDashboardError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutDashboardError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutMetricAlarm
#[derive(Debug, PartialEq)]
pub enum PutMetricAlarmError {
    /// <p>The quota for alarms for this customer has already been reached.</p>
    LimitExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutMetricAlarmError {
    pub fn from_body(body: &str) -> PutMetricAlarmError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "LimitExceeded" => {
                    PutMetricAlarmError::LimitExceededFault(String::from(parsed_error.message))
                }
                _ => PutMetricAlarmError::Unknown(String::from(body)),
            },
            Err(_) => PutMetricAlarmError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutMetricAlarmError {
    fn from(err: XmlParseError) -> PutMetricAlarmError {
        let XmlParseError(message) = err;
        PutMetricAlarmError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutMetricAlarmError {
    fn from(err: CredentialsError) -> PutMetricAlarmError {
        PutMetricAlarmError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutMetricAlarmError {
    fn from(err: HttpDispatchError) -> PutMetricAlarmError {
        PutMetricAlarmError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutMetricAlarmError {
    fn from(err: io::Error) -> PutMetricAlarmError {
        PutMetricAlarmError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutMetricAlarmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutMetricAlarmError {
    fn description(&self) -> &str {
        match *self {
            PutMetricAlarmError::LimitExceededFault(ref cause) => cause,
            PutMetricAlarmError::Validation(ref cause) => cause,
            PutMetricAlarmError::Credentials(ref err) => err.description(),
            PutMetricAlarmError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutMetricAlarmError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutMetricData
#[derive(Debug, PartialEq)]
pub enum PutMetricDataError {
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>Parameters were used together that cannot be used together.</p>
    InvalidParameterCombination(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
    /// <p>An input parameter that is required is missing.</p>
    MissingRequiredParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutMetricDataError {
    pub fn from_body(body: &str) -> PutMetricDataError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InternalServiceError" => {
                    PutMetricDataError::InternalServiceFault(String::from(parsed_error.message))
                }
                "InvalidParameterCombination" => PutMetricDataError::InvalidParameterCombination(
                    String::from(parsed_error.message),
                ),
                "InvalidParameterValue" => {
                    PutMetricDataError::InvalidParameterValue(String::from(parsed_error.message))
                }
                "MissingParameter" => {
                    PutMetricDataError::MissingRequiredParameter(String::from(parsed_error.message))
                }
                _ => PutMetricDataError::Unknown(String::from(body)),
            },
            Err(_) => PutMetricDataError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutMetricDataError {
    fn from(err: XmlParseError) -> PutMetricDataError {
        let XmlParseError(message) = err;
        PutMetricDataError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutMetricDataError {
    fn from(err: CredentialsError) -> PutMetricDataError {
        PutMetricDataError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutMetricDataError {
    fn from(err: HttpDispatchError) -> PutMetricDataError {
        PutMetricDataError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutMetricDataError {
    fn from(err: io::Error) -> PutMetricDataError {
        PutMetricDataError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutMetricDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutMetricDataError {
    fn description(&self) -> &str {
        match *self {
            PutMetricDataError::InternalServiceFault(ref cause) => cause,
            PutMetricDataError::InvalidParameterCombination(ref cause) => cause,
            PutMetricDataError::InvalidParameterValue(ref cause) => cause,
            PutMetricDataError::MissingRequiredParameter(ref cause) => cause,
            PutMetricDataError::Validation(ref cause) => cause,
            PutMetricDataError::Credentials(ref err) => err.description(),
            PutMetricDataError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutMetricDataError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetAlarmState
#[derive(Debug, PartialEq)]
pub enum SetAlarmStateError {
    /// <p>Data was not syntactically valid JSON.</p>
    InvalidFormatFault(String),
    /// <p>The named resource does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetAlarmStateError {
    pub fn from_body(body: &str) -> SetAlarmStateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidFormat" => {
                    SetAlarmStateError::InvalidFormatFault(String::from(parsed_error.message))
                }
                "ResourceNotFound" => {
                    SetAlarmStateError::ResourceNotFound(String::from(parsed_error.message))
                }
                _ => SetAlarmStateError::Unknown(String::from(body)),
            },
            Err(_) => SetAlarmStateError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SetAlarmStateError {
    fn from(err: XmlParseError) -> SetAlarmStateError {
        let XmlParseError(message) = err;
        SetAlarmStateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetAlarmStateError {
    fn from(err: CredentialsError) -> SetAlarmStateError {
        SetAlarmStateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetAlarmStateError {
    fn from(err: HttpDispatchError) -> SetAlarmStateError {
        SetAlarmStateError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetAlarmStateError {
    fn from(err: io::Error) -> SetAlarmStateError {
        SetAlarmStateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetAlarmStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetAlarmStateError {
    fn description(&self) -> &str {
        match *self {
            SetAlarmStateError::InvalidFormatFault(ref cause) => cause,
            SetAlarmStateError::ResourceNotFound(ref cause) => cause,
            SetAlarmStateError::Validation(ref cause) => cause,
            SetAlarmStateError::Credentials(ref err) => err.description(),
            SetAlarmStateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SetAlarmStateError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the CloudWatch API. CloudWatch clients implement this trait.
pub trait CloudWatch {
    /// <p>Deletes the specified alarms. In the event of an error, no alarms are deleted.</p>
    fn delete_alarms(&self, input: DeleteAlarmsInput) -> RusotoFuture<(), DeleteAlarmsError>;

    /// <p>Deletes all dashboards that you specify. You may specify up to 100 dashboards to delete. If there is an error during this call, no dashboards are deleted.</p>
    fn delete_dashboards(
        &self,
        input: DeleteDashboardsInput,
    ) -> RusotoFuture<DeleteDashboardsOutput, DeleteDashboardsError>;

    /// <p>Retrieves the history for the specified alarm. You can filter the results by date range or item type. If an alarm name is not specified, the histories for all alarms are returned.</p> <p>CloudWatch retains the history of an alarm even if you delete the alarm.</p>
    fn describe_alarm_history(
        &self,
        input: DescribeAlarmHistoryInput,
    ) -> RusotoFuture<DescribeAlarmHistoryOutput, DescribeAlarmHistoryError>;

    /// <p>Retrieves the specified alarms. If no alarms are specified, all alarms are returned. Alarms can be retrieved by using only a prefix for the alarm name, the alarm state, or a prefix for any action.</p>
    fn describe_alarms(
        &self,
        input: DescribeAlarmsInput,
    ) -> RusotoFuture<DescribeAlarmsOutput, DescribeAlarmsError>;

    /// <p>Retrieves the alarms for the specified metric. To filter the results, specify a statistic, period, or unit.</p>
    fn describe_alarms_for_metric(
        &self,
        input: DescribeAlarmsForMetricInput,
    ) -> RusotoFuture<DescribeAlarmsForMetricOutput, DescribeAlarmsForMetricError>;

    /// <p>Disables the actions for the specified alarms. When an alarm's actions are disabled, the alarm actions do not execute when the alarm state changes.</p>
    fn disable_alarm_actions(
        &self,
        input: DisableAlarmActionsInput,
    ) -> RusotoFuture<(), DisableAlarmActionsError>;

    /// <p>Enables the actions for the specified alarms.</p>
    fn enable_alarm_actions(
        &self,
        input: EnableAlarmActionsInput,
    ) -> RusotoFuture<(), EnableAlarmActionsError>;

    /// <p>Displays the details of the dashboard that you specify.</p> <p>To copy an existing dashboard, use <code>GetDashboard</code>, and then use the data returned within <code>DashboardBody</code> as the template for the new dashboard when you call <code>PutDashboard</code> to create the copy.</p>
    fn get_dashboard(
        &self,
        input: GetDashboardInput,
    ) -> RusotoFuture<GetDashboardOutput, GetDashboardError>;

    /// <p>You can use the <code>GetMetricData</code> API to retrieve as many as 100 different metrics in a single request, with a total of as many as 100,800 datapoints. You can also optionally perform math expressions on the values of the returned statistics, to create new time series that represent new insights into your data. For example, using Lambda metrics, you could divide the Errors metric by the Invocations metric to get an error rate time series. For more information about metric math expressions, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/using-metric-math.html#metric-math-syntax">Metric Math Syntax and Functions</a> in the <i>Amazon CloudWatch User Guide</i>.</p> <p>Calls to the <code>GetMetricData</code> API have a different pricing structure than calls to <code>GetMetricStatistics</code>. For more information about pricing, see <a href="https://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p>
    fn get_metric_data(
        &self,
        input: GetMetricDataInput,
    ) -> RusotoFuture<GetMetricDataOutput, GetMetricDataError>;

    /// <p>Gets statistics for the specified metric.</p> <p>The maximum number of data points returned from a single call is 1,440. If you request more than 1,440 data points, CloudWatch returns an error. To reduce the number of data points, you can narrow the specified time range and make multiple requests across adjacent time ranges, or you can increase the specified period. Data points are not returned in chronological order.</p> <p>CloudWatch aggregates data points based on the length of the period that you specify. For example, if you request statistics with a one-hour period, CloudWatch aggregates all data points with time stamps that fall within each one-hour period. Therefore, the number of values aggregated by CloudWatch is larger than the number of data points returned.</p> <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you can only retrieve percentile statistics for this data if one of the following conditions is true:</p> <ul> <li> <p>The SampleCount value of the statistic set is 1.</p> </li> <li> <p>The Min and the Max values of the statistic set are equal.</p> </li> </ul> <p>Amazon CloudWatch retains metric data as follows:</p> <ul> <li> <p>Data points with a period of less than 60 seconds are available for 3 hours. These data points are high-resolution metrics and are available only for custom metrics that have been defined with a <code>StorageResolution</code> of 1.</p> </li> <li> <p>Data points with a period of 60 seconds (1-minute) are available for 15 days.</p> </li> <li> <p>Data points with a period of 300 seconds (5-minute) are available for 63 days.</p> </li> <li> <p>Data points with a period of 3600 seconds (1 hour) are available for 455 days (15 months).</p> </li> </ul> <p>Data points that are initially published with a shorter period are aggregated together for long-term storage. For example, if you collect data using a period of 1 minute, the data remains available for 15 days with 1-minute resolution. After 15 days, this data is still available, but is aggregated and retrievable only with a resolution of 5 minutes. After 63 days, the data is further aggregated and is available with a resolution of 1 hour.</p> <p>CloudWatch started retaining 5-minute and 1-hour metric data as of July 9, 2016.</p> <p>For information about metrics and dimensions supported by AWS services, see the <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CW_Support_For_AWS.html">Amazon CloudWatch Metrics and Dimensions Reference</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
    fn get_metric_statistics(
        &self,
        input: GetMetricStatisticsInput,
    ) -> RusotoFuture<GetMetricStatisticsOutput, GetMetricStatisticsError>;

    /// <p>Returns a list of the dashboards for your account. If you include <code>DashboardNamePrefix</code>, only those dashboards with names starting with the prefix are listed. Otherwise, all dashboards in your account are listed. </p>
    fn list_dashboards(
        &self,
        input: ListDashboardsInput,
    ) -> RusotoFuture<ListDashboardsOutput, ListDashboardsError>;

    /// <p>List the specified metrics. You can use the returned metrics with <a>GetMetricStatistics</a> to obtain statistical data.</p> <p>Up to 500 results are returned for any one call. To retrieve additional results, use the returned token with subsequent calls.</p> <p>After you create a metric, allow up to fifteen minutes before the metric appears. Statistics about the metric, however, are available sooner using <a>GetMetricStatistics</a>.</p>
    fn list_metrics(
        &self,
        input: ListMetricsInput,
    ) -> RusotoFuture<ListMetricsOutput, ListMetricsError>;

    /// <p>Creates a dashboard if it does not already exist, or updates an existing dashboard. If you update a dashboard, the entire contents are replaced with what you specify here.</p> <p>You can have up to 500 dashboards per account. All dashboards in your account are global, not region-specific.</p> <p>A simple way to create a dashboard using <code>PutDashboard</code> is to copy an existing dashboard. To copy an existing dashboard using the console, you can load the dashboard and then use the View/edit source command in the Actions menu to display the JSON block for that dashboard. Another way to copy a dashboard is to use <code>GetDashboard</code>, and then use the data returned within <code>DashboardBody</code> as the template for the new dashboard when you call <code>PutDashboard</code>.</p> <p>When you create a dashboard with <code>PutDashboard</code>, a good practice is to add a text widget at the top of the dashboard with a message that the dashboard was created by script and should not be changed in the console. This message could also point console users to the location of the <code>DashboardBody</code> script or the CloudFormation template used to create the dashboard.</p>
    fn put_dashboard(
        &self,
        input: PutDashboardInput,
    ) -> RusotoFuture<PutDashboardOutput, PutDashboardError>;

    /// <p>Creates or updates an alarm and associates it with the specified metric. Optionally, this operation can associate one or more Amazon SNS resources with the alarm.</p> <p>When this operation creates an alarm, the alarm state is immediately set to <code>INSUFFICIENT_DATA</code>. The alarm is evaluated and its state is set appropriately. Any actions associated with the state are then executed.</p> <p>When you update an existing alarm, its state is left unchanged, but the update completely overwrites the previous configuration of the alarm.</p> <p>If you are an IAM user, you must have Amazon EC2 permissions for some operations:</p> <ul> <li> <p> <code>iam:CreateServiceLinkedRole</code> for all alarms with EC2 actions</p> </li> <li> <p> <code>ec2:DescribeInstanceStatus</code> and <code>ec2:DescribeInstances</code> for all alarms on EC2 instance status metrics</p> </li> <li> <p> <code>ec2:StopInstances</code> for alarms with stop actions</p> </li> <li> <p> <code>ec2:TerminateInstances</code> for alarms with terminate actions</p> </li> <li> <p> <code>ec2:DescribeInstanceRecoveryAttribute</code> and <code>ec2:RecoverInstances</code> for alarms with recover actions</p> </li> </ul> <p>If you have read/write permissions for Amazon CloudWatch but not for Amazon EC2, you can still create an alarm, but the stop or terminate actions are not performed. However, if you are later granted the required permissions, the alarm actions that you created earlier are performed.</p> <p>If you are using an IAM role (for example, an EC2 instance profile), you cannot stop or terminate the instance using alarm actions. However, you can still see the alarm state and perform any other actions such as Amazon SNS notifications or Auto Scaling policies.</p> <p>If you are using temporary security credentials granted using AWS STS, you cannot stop or terminate an EC2 instance using alarm actions.</p> <p>You must create at least one stop, terminate, or reboot alarm using either the Amazon EC2 or CloudWatch consoles to create the <b>EC2ActionsAccess</b> IAM role. After this IAM role is created, you can create stop, terminate, or reboot alarms using a command-line interface or API.</p>
    fn put_metric_alarm(&self, input: PutMetricAlarmInput)
        -> RusotoFuture<(), PutMetricAlarmError>;

    /// <p><p>Publishes metric data points to Amazon CloudWatch. CloudWatch associates the data points with the specified metric. If the specified metric does not exist, CloudWatch creates the metric. When CloudWatch creates a metric, it can take up to fifteen minutes for the metric to appear in calls to <a>ListMetrics</a>.</p> <p>Each <code>PutMetricData</code> request is limited to 40 KB in size for HTTP POST requests.</p> <p>Although the <code>Value</code> parameter accepts numbers of type <code>Double</code>, CloudWatch rejects values that are either too small or too large. Values must be in the range of 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2). In addition, special values (for example, NaN, +Infinity, -Infinity) are not supported.</p> <p>You can use up to 10 dimensions per metric to further clarify what data the metric collects. For more information about specifying dimensions, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p> <p>Data points with time stamps from 24 hours ago or longer can take at least 48 hours to become available for <a>GetMetricStatistics</a> from the time they are submitted.</p> <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you can only retrieve percentile statistics for this data if one of the following conditions is true:</p> <ul> <li> <p>The SampleCount value of the statistic set is 1</p> </li> <li> <p>The Min and the Max values of the statistic set are equal</p> </li> </ul></p>
    fn put_metric_data(&self, input: PutMetricDataInput) -> RusotoFuture<(), PutMetricDataError>;

    /// <p>Temporarily sets the state of an alarm for testing purposes. When the updated state differs from the previous value, the action configured for the appropriate state is invoked. For example, if your alarm is configured to send an Amazon SNS message when an alarm is triggered, temporarily changing the alarm state to <code>ALARM</code> sends an SNS message. The alarm returns to its actual state (often within seconds). Because the alarm state change happens quickly, it is typically only visible in the alarm's <b>History</b> tab in the Amazon CloudWatch console or through <a>DescribeAlarmHistory</a>.</p>
    fn set_alarm_state(&self, input: SetAlarmStateInput) -> RusotoFuture<(), SetAlarmStateError>;
}
/// A client for the CloudWatch API.
pub struct CloudWatchClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl CloudWatchClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> CloudWatchClient {
        CloudWatchClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> CloudWatchClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CloudWatchClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> CloudWatch for CloudWatchClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Deletes the specified alarms. In the event of an error, no alarms are deleted.</p>
    fn delete_alarms(&self, input: DeleteAlarmsInput) -> RusotoFuture<(), DeleteAlarmsError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteAlarms");
        params.put("Version", "2010-08-01");
        DeleteAlarmsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAlarmsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes all dashboards that you specify. You may specify up to 100 dashboards to delete. If there is an error during this call, no dashboards are deleted.</p>
    fn delete_dashboards(
        &self,
        input: DeleteDashboardsInput,
    ) -> RusotoFuture<DeleteDashboardsOutput, DeleteDashboardsError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDashboards");
        params.put("Version", "2010-08-01");
        DeleteDashboardsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDashboardsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteDashboardsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteDashboardsOutputDeserializer::deserialize(
                        "DeleteDashboardsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the history for the specified alarm. You can filter the results by date range or item type. If an alarm name is not specified, the histories for all alarms are returned.</p> <p>CloudWatch retains the history of an alarm even if you delete the alarm.</p>
    fn describe_alarm_history(
        &self,
        input: DescribeAlarmHistoryInput,
    ) -> RusotoFuture<DescribeAlarmHistoryOutput, DescribeAlarmHistoryError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAlarmHistory");
        params.put("Version", "2010-08-01");
        DescribeAlarmHistoryInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAlarmHistoryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAlarmHistoryOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeAlarmHistoryOutputDeserializer::deserialize(
                        "DescribeAlarmHistoryResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the specified alarms. If no alarms are specified, all alarms are returned. Alarms can be retrieved by using only a prefix for the alarm name, the alarm state, or a prefix for any action.</p>
    fn describe_alarms(
        &self,
        input: DescribeAlarmsInput,
    ) -> RusotoFuture<DescribeAlarmsOutput, DescribeAlarmsError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAlarms");
        params.put("Version", "2010-08-01");
        DescribeAlarmsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAlarmsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAlarmsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeAlarmsOutputDeserializer::deserialize(
                        "DescribeAlarmsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the alarms for the specified metric. To filter the results, specify a statistic, period, or unit.</p>
    fn describe_alarms_for_metric(
        &self,
        input: DescribeAlarmsForMetricInput,
    ) -> RusotoFuture<DescribeAlarmsForMetricOutput, DescribeAlarmsForMetricError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAlarmsForMetric");
        params.put("Version", "2010-08-01");
        DescribeAlarmsForMetricInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAlarmsForMetricError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAlarmsForMetricOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeAlarmsForMetricOutputDeserializer::deserialize(
                        "DescribeAlarmsForMetricResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Disables the actions for the specified alarms. When an alarm's actions are disabled, the alarm actions do not execute when the alarm state changes.</p>
    fn disable_alarm_actions(
        &self,
        input: DisableAlarmActionsInput,
    ) -> RusotoFuture<(), DisableAlarmActionsError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DisableAlarmActions");
        params.put("Version", "2010-08-01");
        DisableAlarmActionsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisableAlarmActionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables the actions for the specified alarms.</p>
    fn enable_alarm_actions(
        &self,
        input: EnableAlarmActionsInput,
    ) -> RusotoFuture<(), EnableAlarmActionsError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "EnableAlarmActions");
        params.put("Version", "2010-08-01");
        EnableAlarmActionsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnableAlarmActionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Displays the details of the dashboard that you specify.</p> <p>To copy an existing dashboard, use <code>GetDashboard</code>, and then use the data returned within <code>DashboardBody</code> as the template for the new dashboard when you call <code>PutDashboard</code> to create the copy.</p>
    fn get_dashboard(
        &self,
        input: GetDashboardInput,
    ) -> RusotoFuture<GetDashboardOutput, GetDashboardError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetDashboard");
        params.put("Version", "2010-08-01");
        GetDashboardInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDashboardError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetDashboardOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetDashboardOutputDeserializer::deserialize(
                        "GetDashboardResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>You can use the <code>GetMetricData</code> API to retrieve as many as 100 different metrics in a single request, with a total of as many as 100,800 datapoints. You can also optionally perform math expressions on the values of the returned statistics, to create new time series that represent new insights into your data. For example, using Lambda metrics, you could divide the Errors metric by the Invocations metric to get an error rate time series. For more information about metric math expressions, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/using-metric-math.html#metric-math-syntax">Metric Math Syntax and Functions</a> in the <i>Amazon CloudWatch User Guide</i>.</p> <p>Calls to the <code>GetMetricData</code> API have a different pricing structure than calls to <code>GetMetricStatistics</code>. For more information about pricing, see <a href="https://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p>
    fn get_metric_data(
        &self,
        input: GetMetricDataInput,
    ) -> RusotoFuture<GetMetricDataOutput, GetMetricDataError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetMetricData");
        params.put("Version", "2010-08-01");
        GetMetricDataInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetMetricDataError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetMetricDataOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetMetricDataOutputDeserializer::deserialize(
                        "GetMetricDataResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets statistics for the specified metric.</p> <p>The maximum number of data points returned from a single call is 1,440. If you request more than 1,440 data points, CloudWatch returns an error. To reduce the number of data points, you can narrow the specified time range and make multiple requests across adjacent time ranges, or you can increase the specified period. Data points are not returned in chronological order.</p> <p>CloudWatch aggregates data points based on the length of the period that you specify. For example, if you request statistics with a one-hour period, CloudWatch aggregates all data points with time stamps that fall within each one-hour period. Therefore, the number of values aggregated by CloudWatch is larger than the number of data points returned.</p> <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you can only retrieve percentile statistics for this data if one of the following conditions is true:</p> <ul> <li> <p>The SampleCount value of the statistic set is 1.</p> </li> <li> <p>The Min and the Max values of the statistic set are equal.</p> </li> </ul> <p>Amazon CloudWatch retains metric data as follows:</p> <ul> <li> <p>Data points with a period of less than 60 seconds are available for 3 hours. These data points are high-resolution metrics and are available only for custom metrics that have been defined with a <code>StorageResolution</code> of 1.</p> </li> <li> <p>Data points with a period of 60 seconds (1-minute) are available for 15 days.</p> </li> <li> <p>Data points with a period of 300 seconds (5-minute) are available for 63 days.</p> </li> <li> <p>Data points with a period of 3600 seconds (1 hour) are available for 455 days (15 months).</p> </li> </ul> <p>Data points that are initially published with a shorter period are aggregated together for long-term storage. For example, if you collect data using a period of 1 minute, the data remains available for 15 days with 1-minute resolution. After 15 days, this data is still available, but is aggregated and retrievable only with a resolution of 5 minutes. After 63 days, the data is further aggregated and is available with a resolution of 1 hour.</p> <p>CloudWatch started retaining 5-minute and 1-hour metric data as of July 9, 2016.</p> <p>For information about metrics and dimensions supported by AWS services, see the <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CW_Support_For_AWS.html">Amazon CloudWatch Metrics and Dimensions Reference</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
    fn get_metric_statistics(
        &self,
        input: GetMetricStatisticsInput,
    ) -> RusotoFuture<GetMetricStatisticsOutput, GetMetricStatisticsError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetMetricStatistics");
        params.put("Version", "2010-08-01");
        GetMetricStatisticsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetMetricStatisticsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetMetricStatisticsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetMetricStatisticsOutputDeserializer::deserialize(
                        "GetMetricStatisticsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of the dashboards for your account. If you include <code>DashboardNamePrefix</code>, only those dashboards with names starting with the prefix are listed. Otherwise, all dashboards in your account are listed. </p>
    fn list_dashboards(
        &self,
        input: ListDashboardsInput,
    ) -> RusotoFuture<ListDashboardsOutput, ListDashboardsError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListDashboards");
        params.put("Version", "2010-08-01");
        ListDashboardsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListDashboardsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListDashboardsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListDashboardsOutputDeserializer::deserialize(
                        "ListDashboardsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>List the specified metrics. You can use the returned metrics with <a>GetMetricStatistics</a> to obtain statistical data.</p> <p>Up to 500 results are returned for any one call. To retrieve additional results, use the returned token with subsequent calls.</p> <p>After you create a metric, allow up to fifteen minutes before the metric appears. Statistics about the metric, however, are available sooner using <a>GetMetricStatistics</a>.</p>
    fn list_metrics(
        &self,
        input: ListMetricsInput,
    ) -> RusotoFuture<ListMetricsOutput, ListMetricsError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListMetrics");
        params.put("Version", "2010-08-01");
        ListMetricsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListMetricsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListMetricsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListMetricsOutputDeserializer::deserialize(
                        "ListMetricsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a dashboard if it does not already exist, or updates an existing dashboard. If you update a dashboard, the entire contents are replaced with what you specify here.</p> <p>You can have up to 500 dashboards per account. All dashboards in your account are global, not region-specific.</p> <p>A simple way to create a dashboard using <code>PutDashboard</code> is to copy an existing dashboard. To copy an existing dashboard using the console, you can load the dashboard and then use the View/edit source command in the Actions menu to display the JSON block for that dashboard. Another way to copy a dashboard is to use <code>GetDashboard</code>, and then use the data returned within <code>DashboardBody</code> as the template for the new dashboard when you call <code>PutDashboard</code>.</p> <p>When you create a dashboard with <code>PutDashboard</code>, a good practice is to add a text widget at the top of the dashboard with a message that the dashboard was created by script and should not be changed in the console. This message could also point console users to the location of the <code>DashboardBody</code> script or the CloudFormation template used to create the dashboard.</p>
    fn put_dashboard(
        &self,
        input: PutDashboardInput,
    ) -> RusotoFuture<PutDashboardOutput, PutDashboardError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutDashboard");
        params.put("Version", "2010-08-01");
        PutDashboardInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutDashboardError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = PutDashboardOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(PutDashboardOutputDeserializer::deserialize(
                        "PutDashboardResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates or updates an alarm and associates it with the specified metric. Optionally, this operation can associate one or more Amazon SNS resources with the alarm.</p> <p>When this operation creates an alarm, the alarm state is immediately set to <code>INSUFFICIENT_DATA</code>. The alarm is evaluated and its state is set appropriately. Any actions associated with the state are then executed.</p> <p>When you update an existing alarm, its state is left unchanged, but the update completely overwrites the previous configuration of the alarm.</p> <p>If you are an IAM user, you must have Amazon EC2 permissions for some operations:</p> <ul> <li> <p> <code>iam:CreateServiceLinkedRole</code> for all alarms with EC2 actions</p> </li> <li> <p> <code>ec2:DescribeInstanceStatus</code> and <code>ec2:DescribeInstances</code> for all alarms on EC2 instance status metrics</p> </li> <li> <p> <code>ec2:StopInstances</code> for alarms with stop actions</p> </li> <li> <p> <code>ec2:TerminateInstances</code> for alarms with terminate actions</p> </li> <li> <p> <code>ec2:DescribeInstanceRecoveryAttribute</code> and <code>ec2:RecoverInstances</code> for alarms with recover actions</p> </li> </ul> <p>If you have read/write permissions for Amazon CloudWatch but not for Amazon EC2, you can still create an alarm, but the stop or terminate actions are not performed. However, if you are later granted the required permissions, the alarm actions that you created earlier are performed.</p> <p>If you are using an IAM role (for example, an EC2 instance profile), you cannot stop or terminate the instance using alarm actions. However, you can still see the alarm state and perform any other actions such as Amazon SNS notifications or Auto Scaling policies.</p> <p>If you are using temporary security credentials granted using AWS STS, you cannot stop or terminate an EC2 instance using alarm actions.</p> <p>You must create at least one stop, terminate, or reboot alarm using either the Amazon EC2 or CloudWatch consoles to create the <b>EC2ActionsAccess</b> IAM role. After this IAM role is created, you can create stop, terminate, or reboot alarms using a command-line interface or API.</p>
    fn put_metric_alarm(
        &self,
        input: PutMetricAlarmInput,
    ) -> RusotoFuture<(), PutMetricAlarmError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutMetricAlarm");
        params.put("Version", "2010-08-01");
        PutMetricAlarmInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutMetricAlarmError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Publishes metric data points to Amazon CloudWatch. CloudWatch associates the data points with the specified metric. If the specified metric does not exist, CloudWatch creates the metric. When CloudWatch creates a metric, it can take up to fifteen minutes for the metric to appear in calls to <a>ListMetrics</a>.</p> <p>Each <code>PutMetricData</code> request is limited to 40 KB in size for HTTP POST requests.</p> <p>Although the <code>Value</code> parameter accepts numbers of type <code>Double</code>, CloudWatch rejects values that are either too small or too large. Values must be in the range of 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2). In addition, special values (for example, NaN, +Infinity, -Infinity) are not supported.</p> <p>You can use up to 10 dimensions per metric to further clarify what data the metric collects. For more information about specifying dimensions, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p> <p>Data points with time stamps from 24 hours ago or longer can take at least 48 hours to become available for <a>GetMetricStatistics</a> from the time they are submitted.</p> <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you can only retrieve percentile statistics for this data if one of the following conditions is true:</p> <ul> <li> <p>The SampleCount value of the statistic set is 1</p> </li> <li> <p>The Min and the Max values of the statistic set are equal</p> </li> </ul></p>
    fn put_metric_data(&self, input: PutMetricDataInput) -> RusotoFuture<(), PutMetricDataError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutMetricData");
        params.put("Version", "2010-08-01");
        PutMetricDataInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutMetricDataError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Temporarily sets the state of an alarm for testing purposes. When the updated state differs from the previous value, the action configured for the appropriate state is invoked. For example, if your alarm is configured to send an Amazon SNS message when an alarm is triggered, temporarily changing the alarm state to <code>ALARM</code> sends an SNS message. The alarm returns to its actual state (often within seconds). Because the alarm state change happens quickly, it is typically only visible in the alarm's <b>History</b> tab in the Amazon CloudWatch console or through <a>DescribeAlarmHistory</a>.</p>
    fn set_alarm_state(&self, input: SetAlarmStateInput) -> RusotoFuture<(), SetAlarmStateError> {
        let mut request = SignedRequest::new("POST", "monitoring", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetAlarmState");
        params.put("Version", "2010-08-01");
        SetAlarmStateInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetAlarmStateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[test]
    fn test_parse_error_cloudwatch_describe_alarm_history() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "cloudwatch-describe-alarm-history.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = CloudWatchClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAlarmHistoryInput::default();
        let result = client.describe_alarm_history(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudwatch_describe_alarm_history() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudwatch-describe-alarm-history.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudWatchClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAlarmHistoryInput::default();
        let result = client.describe_alarm_history(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudwatch_describe_alarms() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudwatch-describe-alarms.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudWatchClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAlarmsInput::default();
        let result = client.describe_alarms(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudwatch_list_metrics() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudwatch-list-metrics.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudWatchClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListMetricsInput::default();
        let result = client.list_metrics(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
