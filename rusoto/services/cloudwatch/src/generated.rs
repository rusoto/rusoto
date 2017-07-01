
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

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use std::str::FromStr;
use xml::EventReader;
use xml::reader::ParserConfig;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use xml::reader::XmlEvent;
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::xmlutil::{characters, end_element, start_element, skip_tree, peek_at_name};
use rusoto_core::xmlerror::*;

enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
pub type ActionPrefix = String;
pub type ActionsEnabled = bool;
struct ActionsEnabledDeserializer;
impl ActionsEnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ActionsEnabled, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type AlarmArn = String;
struct AlarmArnDeserializer;
impl AlarmArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AlarmArn, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type AlarmDescription = String;
struct AlarmDescriptionDeserializer;
impl AlarmDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AlarmDescription, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[doc="<p>Represents the history of a specific alarm.</p>"]
#[derive(Default,Debug,Clone)]
pub struct AlarmHistoryItem {
    #[doc="<p>The descriptive name for the alarm.</p>"]
    pub alarm_name: Option<AlarmName>,
    #[doc="<p>Data about the alarm, in JSON format.</p>"]
    pub history_data: Option<HistoryData>,
    #[doc="<p>The type of alarm history item.</p>"]
    pub history_item_type: Option<HistoryItemType>,
    #[doc="<p>A summary of the alarm history, in text format.</p>"]
    pub history_summary: Option<HistorySummary>,
    #[doc="<p>The time stamp for the alarm history item.</p>"]
    pub timestamp: Option<Timestamp>,
}

struct AlarmHistoryItemDeserializer;
impl AlarmHistoryItemDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AlarmHistoryItem, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AlarmName" => {
                            obj.alarm_name = Some(try!(AlarmNameDeserializer::deserialize("AlarmName",
                                                                                          stack)));
                        }
                        "HistoryData" => {
                            obj.history_data =
                                Some(try!(HistoryDataDeserializer::deserialize("HistoryData",
                                                                               stack)));
                        }
                        "HistoryItemType" => {
                            obj.history_item_type =
                                Some(try!(HistoryItemTypeDeserializer::deserialize("HistoryItemType",
                                                                                   stack)));
                        }
                        "HistorySummary" => {
                            obj.history_summary =
                                Some(try!(HistorySummaryDeserializer::deserialize("HistorySummary",
                                                                                  stack)));
                        }
                        "Timestamp" => {
                            obj.timestamp = Some(try!(TimestampDeserializer::deserialize("Timestamp",
                                                                                         stack)));
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
pub type AlarmHistoryItems = Vec<AlarmHistoryItem>;
struct AlarmHistoryItemsDeserializer;
impl AlarmHistoryItemsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AlarmHistoryItems, XmlParseError> {

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
                        obj.push(try!(AlarmHistoryItemDeserializer::deserialize("member", stack)));
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
pub type AlarmName = String;
struct AlarmNameDeserializer;
impl AlarmNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AlarmName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type AlarmNamePrefix = String;
pub type AlarmNames = Vec<AlarmName>;

/// Serialize `AlarmNames` contents to a `SignedRequest`.
struct AlarmNamesSerializer;
impl AlarmNamesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AlarmNames) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

pub type AwsQueryErrorMessage = String;
pub type ComparisonOperator = String;
struct ComparisonOperatorDeserializer;
impl ComparisonOperatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ComparisonOperator, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[doc="<p>Encapsulates the statistical data that Amazon CloudWatch computes from metric data.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Datapoint {
    #[doc="<p>The average of the metric values that correspond to the data point.</p>"]
    pub average: Option<DatapointValue>,
    #[doc="<p>The percentile statistic for the data point.</p>"]
    pub extended_statistics: Option<DatapointValueMap>,
    #[doc="<p>The maximum metric value for the data point.</p>"]
    pub maximum: Option<DatapointValue>,
    #[doc="<p>The minimum metric value for the data point.</p>"]
    pub minimum: Option<DatapointValue>,
    #[doc="<p>The number of metric values that contributed to the aggregate value of this data point.</p>"]
    pub sample_count: Option<DatapointValue>,
    #[doc="<p>The sum of the metric values for the data point.</p>"]
    pub sum: Option<DatapointValue>,
    #[doc="<p>The time stamp used for the data point.</p>"]
    pub timestamp: Option<Timestamp>,
    #[doc="<p>The standard unit for the data point.</p>"]
    pub unit: Option<StandardUnit>,
}

struct DatapointDeserializer;
impl DatapointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Datapoint, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Average" => {
                            obj.average =
                                Some(try!(DatapointValueDeserializer::deserialize("Average",
                                                                                  stack)));
                        }
                        "ExtendedStatistics" => {
                            obj.extended_statistics =
                                Some(try!(DatapointValueMapDeserializer::deserialize("ExtendedStatistics",
                                                                                     stack)));
                        }
                        "Maximum" => {
                            obj.maximum =
                                Some(try!(DatapointValueDeserializer::deserialize("Maximum",
                                                                                  stack)));
                        }
                        "Minimum" => {
                            obj.minimum =
                                Some(try!(DatapointValueDeserializer::deserialize("Minimum",
                                                                                  stack)));
                        }
                        "SampleCount" => {
                            obj.sample_count =
                                Some(try!(DatapointValueDeserializer::deserialize("SampleCount",
                                                                                  stack)));
                        }
                        "Sum" => {
                            obj.sum = Some(try!(DatapointValueDeserializer::deserialize("Sum",
                                                                                        stack)));
                        }
                        "Timestamp" => {
                            obj.timestamp = Some(try!(TimestampDeserializer::deserialize("Timestamp",
                                                                                         stack)));
                        }
                        "Unit" => {
                            obj.unit = Some(try!(StandardUnitDeserializer::deserialize("Unit",
                                                                                       stack)));
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
pub type DatapointValue = f64;
struct DatapointValueDeserializer;
impl DatapointValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DatapointValue, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type DatapointValueMap = ::std::collections::HashMap<ExtendedStatistic, DatapointValue>;
struct DatapointValueMapDeserializer;
impl DatapointValueMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DatapointValueMap, XmlParseError> {
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
pub type Datapoints = Vec<Datapoint>;
struct DatapointsDeserializer;
impl DatapointsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Datapoints, XmlParseError> {

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
#[derive(Default,Debug,Clone)]
pub struct DeleteAlarmsInput {
    #[doc="<p>The alarms to be deleted.</p>"]
    pub alarm_names: AlarmNames,
}


/// Serialize `DeleteAlarmsInput` contents to a `SignedRequest`.
struct DeleteAlarmsInputSerializer;
impl DeleteAlarmsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteAlarmsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AlarmNamesSerializer::serialize(params,
                                        &format!("{}{}", prefix, "AlarmNames"),
                                        &obj.alarm_names);

    }
}

#[derive(Default,Debug,Clone)]
pub struct DescribeAlarmHistoryInput {
    #[doc="<p>The name of the alarm.</p>"]
    pub alarm_name: Option<AlarmName>,
    #[doc="<p>The ending date to retrieve alarm history.</p>"]
    pub end_date: Option<Timestamp>,
    #[doc="<p>The type of alarm histories to retrieve.</p>"]
    pub history_item_type: Option<HistoryItemType>,
    #[doc="<p>The maximum number of alarm history records to retrieve.</p>"]
    pub max_records: Option<MaxRecords>,
    #[doc="<p>The token returned by a previous call to indicate that there is more data available.</p>"]
    pub next_token: Option<NextToken>,
    #[doc="<p>The starting date to retrieve alarm history.</p>"]
    pub start_date: Option<Timestamp>,
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
            params.put(&format!("{}{}", prefix, "AlarmName"), &field_value);
        }
        if let Some(ref field_value) = obj.end_date {
            params.put(&format!("{}{}", prefix, "EndDate"), &field_value);
        }
        if let Some(ref field_value) = obj.history_item_type {
            params.put(&format!("{}{}", prefix, "HistoryItemType"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        if let Some(ref field_value) = obj.start_date {
            params.put(&format!("{}{}", prefix, "StartDate"), &field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct DescribeAlarmHistoryOutput {
    #[doc="<p>The alarm histories, in JSON format.</p>"]
    pub alarm_history_items: Option<AlarmHistoryItems>,
    #[doc="<p>The token that marks the start of the next batch of returned results.</p>"]
    pub next_token: Option<NextToken>,
}

struct DescribeAlarmHistoryOutputDeserializer;
impl DescribeAlarmHistoryOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DescribeAlarmHistoryOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AlarmHistoryItems" => {
                            obj.alarm_history_items =
                                Some(try!(AlarmHistoryItemsDeserializer::deserialize("AlarmHistoryItems",
                                                                                     stack)));
                        }
                        "NextToken" => {
                            obj.next_token = Some(try!(NextTokenDeserializer::deserialize("NextToken",
                                                                                          stack)));
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
#[derive(Default,Debug,Clone)]
pub struct DescribeAlarmsForMetricInput {
    #[doc="<p>The dimensions associated with the metric. If the metric has any associated dimensions, you must specify them in order for the call to succeed.</p>"]
    pub dimensions: Option<Dimensions>,
    #[doc="<p>The percentile statistic for the metric. Specify a value between p0.0 and p100.</p>"]
    pub extended_statistic: Option<ExtendedStatistic>,
    #[doc="<p>The name of the metric.</p>"]
    pub metric_name: MetricName,
    #[doc="<p>The namespace of the metric.</p>"]
    pub namespace: Namespace,
    #[doc="<p>The period, in seconds, over which the statistic is applied.</p>"]
    pub period: Option<Period>,
    #[doc="<p>The statistic for the metric, other than percentiles. For percentile statistics, use <code>ExtendedStatistics</code>.</p>"]
    pub statistic: Option<Statistic>,
    #[doc="<p>The unit for the metric.</p>"]
    pub unit: Option<StandardUnit>,
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
            DimensionsSerializer::serialize(params,
                                            &format!("{}{}", prefix, "Dimensions"),
                                            field_value);
        }
        if let Some(ref field_value) = obj.extended_statistic {
            params.put(&format!("{}{}", prefix, "ExtendedStatistic"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "MetricName"), &obj.metric_name);
        params.put(&format!("{}{}", prefix, "Namespace"), &obj.namespace);
        if let Some(ref field_value) = obj.period {
            params.put(&format!("{}{}", prefix, "Period"), &field_value.to_string());
        }
        if let Some(ref field_value) = obj.statistic {
            params.put(&format!("{}{}", prefix, "Statistic"), &field_value);
        }
        if let Some(ref field_value) = obj.unit {
            params.put(&format!("{}{}", prefix, "Unit"), &field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct DescribeAlarmsForMetricOutput {
    #[doc="<p>The information for each alarm with the specified metric.</p>"]
    pub metric_alarms: Option<MetricAlarms>,
}

struct DescribeAlarmsForMetricOutputDeserializer;
impl DescribeAlarmsForMetricOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DescribeAlarmsForMetricOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "MetricAlarms" => {
                            obj.metric_alarms =
                                Some(try!(MetricAlarmsDeserializer::deserialize("MetricAlarms",
                                                                                stack)));
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
#[derive(Default,Debug,Clone)]
pub struct DescribeAlarmsInput {
    #[doc="<p>The action name prefix.</p>"]
    pub action_prefix: Option<ActionPrefix>,
    #[doc="<p>The alarm name prefix. You cannot specify <code>AlarmNames</code> if this parameter is specified.</p>"]
    pub alarm_name_prefix: Option<AlarmNamePrefix>,
    #[doc="<p>The names of the alarms.</p>"]
    pub alarm_names: Option<AlarmNames>,
    #[doc="<p>The maximum number of alarm descriptions to retrieve.</p>"]
    pub max_records: Option<MaxRecords>,
    #[doc="<p>The token returned by a previous call to indicate that there is more data available.</p>"]
    pub next_token: Option<NextToken>,
    #[doc="<p>The state value to be used in matching alarms.</p>"]
    pub state_value: Option<StateValue>,
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
            params.put(&format!("{}{}", prefix, "ActionPrefix"), &field_value);
        }
        if let Some(ref field_value) = obj.alarm_name_prefix {
            params.put(&format!("{}{}", prefix, "AlarmNamePrefix"), &field_value);
        }
        if let Some(ref field_value) = obj.alarm_names {
            AlarmNamesSerializer::serialize(params,
                                            &format!("{}{}", prefix, "AlarmNames"),
                                            field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        if let Some(ref field_value) = obj.state_value {
            params.put(&format!("{}{}", prefix, "StateValue"), &field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct DescribeAlarmsOutput {
    #[doc="<p>The information for the specified alarms.</p>"]
    pub metric_alarms: Option<MetricAlarms>,
    #[doc="<p>The token that marks the start of the next batch of returned results.</p>"]
    pub next_token: Option<NextToken>,
}

struct DescribeAlarmsOutputDeserializer;
impl DescribeAlarmsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DescribeAlarmsOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "MetricAlarms" => {
                            obj.metric_alarms =
                                Some(try!(MetricAlarmsDeserializer::deserialize("MetricAlarms",
                                                                                stack)));
                        }
                        "NextToken" => {
                            obj.next_token = Some(try!(NextTokenDeserializer::deserialize("NextToken",
                                                                                          stack)));
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
#[doc="<p>Expands the identity of a metric.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Dimension {
    #[doc="<p>The name of the dimension.</p>"]
    pub name: DimensionName,
    #[doc="<p>The value representing the dimension measurement.</p>"]
    pub value: DimensionValue,
}

struct DimensionDeserializer;
impl DimensionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Dimension, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Name" => {
                            obj.name = try!(DimensionNameDeserializer::deserialize("Name", stack));
                        }
                        "Value" => {
                            obj.value = try!(DimensionValueDeserializer::deserialize("Value",
                                                                                     stack));
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

/// Serialize `Dimension` contents to a `SignedRequest`.
struct DimensionSerializer;
impl DimensionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Dimension) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        params.put(&format!("{}{}", prefix, "Value"), &obj.value);

    }
}

#[doc="<p>Represents filters for a dimension.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DimensionFilter {
    #[doc="<p>The dimension name to be matched.</p>"]
    pub name: DimensionName,
    #[doc="<p>The value of the dimension to be matched.</p>"]
    pub value: Option<DimensionValue>,
}


/// Serialize `DimensionFilter` contents to a `SignedRequest`.
struct DimensionFilterSerializer;
impl DimensionFilterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DimensionFilter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }

    }
}

pub type DimensionFilters = Vec<DimensionFilter>;

/// Serialize `DimensionFilters` contents to a `SignedRequest`.
struct DimensionFiltersSerializer;
impl DimensionFiltersSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DimensionFilters) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            DimensionFilterSerializer::serialize(params, &key, obj);
        }
    }
}

pub type DimensionName = String;
struct DimensionNameDeserializer;
impl DimensionNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DimensionName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type DimensionValue = String;
struct DimensionValueDeserializer;
impl DimensionValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DimensionValue, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type Dimensions = Vec<Dimension>;
struct DimensionsDeserializer;
impl DimensionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Dimensions, XmlParseError> {

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
    fn serialize(params: &mut Params, name: &str, obj: &Dimensions) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            DimensionSerializer::serialize(params, &key, obj);
        }
    }
}

#[derive(Default,Debug,Clone)]
pub struct DisableAlarmActionsInput {
    #[doc="<p>The names of the alarms.</p>"]
    pub alarm_names: AlarmNames,
}


/// Serialize `DisableAlarmActionsInput` contents to a `SignedRequest`.
struct DisableAlarmActionsInputSerializer;
impl DisableAlarmActionsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DisableAlarmActionsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AlarmNamesSerializer::serialize(params,
                                        &format!("{}{}", prefix, "AlarmNames"),
                                        &obj.alarm_names);

    }
}

#[derive(Default,Debug,Clone)]
pub struct EnableAlarmActionsInput {
    #[doc="<p>The names of the alarms.</p>"]
    pub alarm_names: AlarmNames,
}


/// Serialize `EnableAlarmActionsInput` contents to a `SignedRequest`.
struct EnableAlarmActionsInputSerializer;
impl EnableAlarmActionsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &EnableAlarmActionsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AlarmNamesSerializer::serialize(params,
                                        &format!("{}{}", prefix, "AlarmNames"),
                                        &obj.alarm_names);

    }
}

pub type ErrorMessage = String;
pub type EvaluateLowSampleCountPercentile = String;
struct EvaluateLowSampleCountPercentileDeserializer;
impl EvaluateLowSampleCountPercentileDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<EvaluateLowSampleCountPercentile, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type EvaluationPeriods = i64;
struct EvaluationPeriodsDeserializer;
impl EvaluationPeriodsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<EvaluationPeriods, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type ExtendedStatistic = String;
struct ExtendedStatisticDeserializer;
impl ExtendedStatisticDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ExtendedStatistic, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type ExtendedStatistics = Vec<ExtendedStatistic>;

/// Serialize `ExtendedStatistics` contents to a `SignedRequest`.
struct ExtendedStatisticsSerializer;
impl ExtendedStatisticsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ExtendedStatistics) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

pub type FaultDescription = String;
#[derive(Default,Debug,Clone)]
pub struct GetMetricStatisticsInput {
    #[doc="<p>The dimensions. If the metric contains multiple dimensions, you must include a value for each dimension. CloudWatch treats each unique combination of dimensions as a separate metric. You can't retrieve statistics using combinations of dimensions that were not specially published. You must specify the same dimensions that were used when the metrics were created. For an example, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#dimension-combinations\">Dimension Combinations</a> in the <i>Amazon CloudWatch User Guide</i>. For more information on specifying dimensions, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html\">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p>"]
    pub dimensions: Option<Dimensions>,
    #[doc="<p>The time stamp that determines the last data point to return.</p> <p>The value specified is exclusive; results will include data points up to the specified time stamp. The time stamp must be in ISO 8601 UTC format (for example, 2016-10-10T23:00:00Z).</p>"]
    pub end_time: Timestamp,
    #[doc="<p>The percentile statistics. Specify values between p0.0 and p100.</p>"]
    pub extended_statistics: Option<ExtendedStatistics>,
    #[doc="<p>The name of the metric, with or without spaces.</p>"]
    pub metric_name: MetricName,
    #[doc="<p>The namespace of the metric, with or without spaces.</p>"]
    pub namespace: Namespace,
    #[doc="<p>The granularity, in seconds, of the returned data points. A period can be as short as one minute (60 seconds) and must be a multiple of 60. The default value is 60.</p> <p>If the <code>StartTime</code> parameter specifies a time stamp that is greater than 15 days ago, you must specify the period as follows or no data points in that time range is returned:</p> <ul> <li> <p>Start time between 15 and 63 days ago - Use a multiple of 300 seconds (5 minutes).</p> </li> <li> <p>Start time greater than 63 days ago - Use a multiple of 3600 seconds (1 hour).</p> </li> </ul>"]
    pub period: Period,
    #[doc="<p>The time stamp that determines the first data point to return. Note that start times are evaluated relative to the time that CloudWatch receives the request.</p> <p>The value specified is inclusive; results include data points with the specified time stamp. The time stamp must be in ISO 8601 UTC format (for example, 2016-10-03T23:00:00Z).</p> <p>CloudWatch rounds the specified time stamp as follows:</p> <ul> <li> <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p> </li> <li> <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p> </li> <li> <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p> </li> </ul>"]
    pub start_time: Timestamp,
    #[doc="<p>The metric statistics, other than percentile. For percentile statistics, use <code>ExtendedStatistic</code>.</p>"]
    pub statistics: Option<Statistics>,
    #[doc="<p>The unit for a given metric. Metrics may be reported in multiple units. Not supplying a unit results in all units being returned. If the metric only ever reports one unit, specifying a unit has no effect.</p>"]
    pub unit: Option<StandardUnit>,
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
            DimensionsSerializer::serialize(params,
                                            &format!("{}{}", prefix, "Dimensions"),
                                            field_value);
        }
        params.put(&format!("{}{}", prefix, "EndTime"), &obj.end_time);
        if let Some(ref field_value) = obj.extended_statistics {
            ExtendedStatisticsSerializer::serialize(params,
                                                    &format!("{}{}", prefix, "ExtendedStatistics"),
                                                    field_value);
        }
        params.put(&format!("{}{}", prefix, "MetricName"), &obj.metric_name);
        params.put(&format!("{}{}", prefix, "Namespace"), &obj.namespace);
        params.put(&format!("{}{}", prefix, "Period"), &obj.period.to_string());
        params.put(&format!("{}{}", prefix, "StartTime"), &obj.start_time);
        if let Some(ref field_value) = obj.statistics {
            StatisticsSerializer::serialize(params,
                                            &format!("{}{}", prefix, "Statistics"),
                                            field_value);
        }
        if let Some(ref field_value) = obj.unit {
            params.put(&format!("{}{}", prefix, "Unit"), &field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct GetMetricStatisticsOutput {
    #[doc="<p>The data points for the specified metric.</p>"]
    pub datapoints: Option<Datapoints>,
    #[doc="<p>A label for the specified metric.</p>"]
    pub label: Option<MetricLabel>,
}

struct GetMetricStatisticsOutputDeserializer;
impl GetMetricStatisticsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetMetricStatisticsOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Datapoints" => {
                            obj.datapoints = Some(try!(DatapointsDeserializer::deserialize("Datapoints",
                                                                                           stack)));
                        }
                        "Label" => {
                            obj.label = Some(try!(MetricLabelDeserializer::deserialize("Label",
                                                                                       stack)));
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
pub type HistoryData = String;
struct HistoryDataDeserializer;
impl HistoryDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<HistoryData, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type HistoryItemType = String;
struct HistoryItemTypeDeserializer;
impl HistoryItemTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<HistoryItemType, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type HistorySummary = String;
struct HistorySummaryDeserializer;
impl HistorySummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<HistorySummary, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug,Clone)]
pub struct ListMetricsInput {
    #[doc="<p>The dimensions to filter against.</p>"]
    pub dimensions: Option<DimensionFilters>,
    #[doc="<p>The name of the metric to filter against.</p>"]
    pub metric_name: Option<MetricName>,
    #[doc="<p>The namespace to filter against.</p>"]
    pub namespace: Option<Namespace>,
    #[doc="<p>The token returned by a previous call to indicate that there is more data available.</p>"]
    pub next_token: Option<NextToken>,
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
            DimensionFiltersSerializer::serialize(params,
                                                  &format!("{}{}", prefix, "Dimensions"),
                                                  field_value);
        }
        if let Some(ref field_value) = obj.metric_name {
            params.put(&format!("{}{}", prefix, "MetricName"), &field_value);
        }
        if let Some(ref field_value) = obj.namespace {
            params.put(&format!("{}{}", prefix, "Namespace"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct ListMetricsOutput {
    #[doc="<p>The metrics.</p>"]
    pub metrics: Option<Metrics>,
    #[doc="<p>The token that marks the start of the next batch of returned results.</p>"]
    pub next_token: Option<NextToken>,
}

struct ListMetricsOutputDeserializer;
impl ListMetricsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ListMetricsOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Metrics" => {
                            obj.metrics = Some(try!(MetricsDeserializer::deserialize("Metrics",
                                                                                     stack)));
                        }
                        "NextToken" => {
                            obj.next_token = Some(try!(NextTokenDeserializer::deserialize("NextToken",
                                                                                          stack)));
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
pub type MaxRecords = i64;
#[doc="<p>Represents a specific metric.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Metric {
    #[doc="<p>The dimensions for the metric.</p>"]
    pub dimensions: Option<Dimensions>,
    #[doc="<p>The name of the metric.</p>"]
    pub metric_name: Option<MetricName>,
    #[doc="<p>The namespace of the metric.</p>"]
    pub namespace: Option<Namespace>,
}

struct MetricDeserializer;
impl MetricDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Metric, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Dimensions" => {
                            obj.dimensions = Some(try!(DimensionsDeserializer::deserialize("Dimensions",
                                                                                           stack)));
                        }
                        "MetricName" => {
                            obj.metric_name =
                                Some(try!(MetricNameDeserializer::deserialize("MetricName",
                                                                              stack)));
                        }
                        "Namespace" => {
                            obj.namespace = Some(try!(NamespaceDeserializer::deserialize("Namespace",
                                                                                         stack)));
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
#[doc="<p>Represents an alarm.</p>"]
#[derive(Default,Debug,Clone)]
pub struct MetricAlarm {
    #[doc="<p>Indicates whether actions should be executed during any changes to the alarm state.</p>"]
    pub actions_enabled: Option<ActionsEnabled>,
    #[doc="<p>The actions to execute when this alarm transitions to the <code>ALARM</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p>"]
    pub alarm_actions: Option<ResourceList>,
    #[doc="<p>The Amazon Resource Name (ARN) of the alarm.</p>"]
    pub alarm_arn: Option<AlarmArn>,
    #[doc="<p>The time stamp of the last update to the alarm configuration.</p>"]
    pub alarm_configuration_updated_timestamp: Option<Timestamp>,
    #[doc="<p>The description of the alarm.</p>"]
    pub alarm_description: Option<AlarmDescription>,
    #[doc="<p>The name of the alarm.</p>"]
    pub alarm_name: Option<AlarmName>,
    #[doc="<p>The arithmetic operation to use when comparing the specified statistic and threshold. The specified statistic value is used as the first operand.</p>"]
    pub comparison_operator: Option<ComparisonOperator>,
    #[doc="<p>The dimensions for the metric associated with the alarm.</p>"]
    pub dimensions: Option<Dimensions>,
    pub evaluate_low_sample_count_percentile: Option<EvaluateLowSampleCountPercentile>,
    #[doc="<p>The number of periods over which data is compared to the specified threshold.</p>"]
    pub evaluation_periods: Option<EvaluationPeriods>,
    #[doc="<p>The percentile statistic for the metric associated with the alarm. Specify a value between p0.0 and p100.</p>"]
    pub extended_statistic: Option<ExtendedStatistic>,
    #[doc="<p>The actions to execute when this alarm transitions to the <code>INSUFFICIENT_DATA</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p>"]
    pub insufficient_data_actions: Option<ResourceList>,
    #[doc="<p>The name of the metric associated with the alarm.</p>"]
    pub metric_name: Option<MetricName>,
    #[doc="<p>The namespace of the metric associated with the alarm.</p>"]
    pub namespace: Option<Namespace>,
    #[doc="<p>The actions to execute when this alarm transitions to the <code>OK</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p>"]
    pub ok_actions: Option<ResourceList>,
    #[doc="<p>The period, in seconds, over which the statistic is applied.</p>"]
    pub period: Option<Period>,
    #[doc="<p>An explanation for the alarm state, in text format.</p>"]
    pub state_reason: Option<StateReason>,
    #[doc="<p>An explanation for the alarm state, in JSON format.</p>"]
    pub state_reason_data: Option<StateReasonData>,
    #[doc="<p>The time stamp of the last update to the alarm state.</p>"]
    pub state_updated_timestamp: Option<Timestamp>,
    #[doc="<p>The state value for the alarm.</p>"]
    pub state_value: Option<StateValue>,
    #[doc="<p>The statistic for the metric associated with the alarm, other than percentile. For percentile statistics, use <code>ExtendedStatistic</code>.</p>"]
    pub statistic: Option<Statistic>,
    #[doc="<p>The value to compare with the specified statistic.</p>"]
    pub threshold: Option<Threshold>,
    pub treat_missing_data: Option<TreatMissingData>,
    #[doc="<p>The unit of the metric associated with the alarm.</p>"]
    pub unit: Option<StandardUnit>,
}

struct MetricAlarmDeserializer;
impl MetricAlarmDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<MetricAlarm, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ActionsEnabled" => {
                            obj.actions_enabled =
                                Some(try!(ActionsEnabledDeserializer::deserialize("ActionsEnabled",
                                                                                  stack)));
                        }
                        "AlarmActions" => {
                            obj.alarm_actions =
                                Some(try!(ResourceListDeserializer::deserialize("AlarmActions",
                                                                                stack)));
                        }
                        "AlarmArn" => {
                            obj.alarm_arn = Some(try!(AlarmArnDeserializer::deserialize("AlarmArn",
                                                                                        stack)));
                        }
                        "AlarmConfigurationUpdatedTimestamp" => {
                            obj.alarm_configuration_updated_timestamp =
                                Some(try!(TimestampDeserializer::deserialize("AlarmConfigurationUpdatedTimestamp",
                                                                             stack)));
                        }
                        "AlarmDescription" => {
                            obj.alarm_description =
                                Some(try!(AlarmDescriptionDeserializer::deserialize("AlarmDescription",
                                                                                    stack)));
                        }
                        "AlarmName" => {
                            obj.alarm_name = Some(try!(AlarmNameDeserializer::deserialize("AlarmName",
                                                                                          stack)));
                        }
                        "ComparisonOperator" => {
                            obj.comparison_operator =
                                Some(try!(ComparisonOperatorDeserializer::deserialize("ComparisonOperator",
                                                                                      stack)));
                        }
                        "Dimensions" => {
                            obj.dimensions = Some(try!(DimensionsDeserializer::deserialize("Dimensions",
                                                                                           stack)));
                        }
                        "EvaluateLowSampleCountPercentile" => {
                            obj.evaluate_low_sample_count_percentile = Some(try!(EvaluateLowSampleCountPercentileDeserializer::deserialize("EvaluateLowSampleCountPercentile", stack)));
                        }
                        "EvaluationPeriods" => {
                            obj.evaluation_periods =
                                Some(try!(EvaluationPeriodsDeserializer::deserialize("EvaluationPeriods",
                                                                                     stack)));
                        }
                        "ExtendedStatistic" => {
                            obj.extended_statistic =
                                Some(try!(ExtendedStatisticDeserializer::deserialize("ExtendedStatistic",
                                                                                     stack)));
                        }
                        "InsufficientDataActions" => {
                            obj.insufficient_data_actions =
                                Some(try!(ResourceListDeserializer::deserialize("InsufficientDataActions",
                                                                                stack)));
                        }
                        "MetricName" => {
                            obj.metric_name =
                                Some(try!(MetricNameDeserializer::deserialize("MetricName",
                                                                              stack)));
                        }
                        "Namespace" => {
                            obj.namespace = Some(try!(NamespaceDeserializer::deserialize("Namespace",
                                                                                         stack)));
                        }
                        "OKActions" => {
                            obj.ok_actions =
                                Some(try!(ResourceListDeserializer::deserialize("OKActions",
                                                                                stack)));
                        }
                        "Period" => {
                            obj.period = Some(try!(PeriodDeserializer::deserialize("Period",
                                                                                   stack)));
                        }
                        "StateReason" => {
                            obj.state_reason =
                                Some(try!(StateReasonDeserializer::deserialize("StateReason",
                                                                               stack)));
                        }
                        "StateReasonData" => {
                            obj.state_reason_data =
                                Some(try!(StateReasonDataDeserializer::deserialize("StateReasonData",
                                                                                   stack)));
                        }
                        "StateUpdatedTimestamp" => {
                            obj.state_updated_timestamp =
                                Some(try!(TimestampDeserializer::deserialize("StateUpdatedTimestamp",
                                                                             stack)));
                        }
                        "StateValue" => {
                            obj.state_value =
                                Some(try!(StateValueDeserializer::deserialize("StateValue",
                                                                              stack)));
                        }
                        "Statistic" => {
                            obj.statistic = Some(try!(StatisticDeserializer::deserialize("Statistic",
                                                                                         stack)));
                        }
                        "Threshold" => {
                            obj.threshold = Some(try!(ThresholdDeserializer::deserialize("Threshold",
                                                                                         stack)));
                        }
                        "TreatMissingData" => {
                            obj.treat_missing_data =
                                Some(try!(TreatMissingDataDeserializer::deserialize("TreatMissingData",
                                                                                    stack)));
                        }
                        "Unit" => {
                            obj.unit = Some(try!(StandardUnitDeserializer::deserialize("Unit",
                                                                                       stack)));
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
pub type MetricAlarms = Vec<MetricAlarm>;
struct MetricAlarmsDeserializer;
impl MetricAlarmsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<MetricAlarms, XmlParseError> {

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
pub type MetricData = Vec<MetricDatum>;

/// Serialize `MetricData` contents to a `SignedRequest`.
struct MetricDataSerializer;
impl MetricDataSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &MetricData) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            MetricDatumSerializer::serialize(params, &key, obj);
        }
    }
}

#[doc="<p>Encapsulates the information sent to either create a metric or add new values to be aggregated into an existing metric.</p>"]
#[derive(Default,Debug,Clone)]
pub struct MetricDatum {
    #[doc="<p>The dimensions associated with the metric.</p>"]
    pub dimensions: Option<Dimensions>,
    #[doc="<p>The name of the metric.</p>"]
    pub metric_name: MetricName,
    #[doc="<p>The statistical values for the metric.</p>"]
    pub statistic_values: Option<StatisticSet>,
    #[doc="<p>The time the metric data was received, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    pub timestamp: Option<Timestamp>,
    #[doc="<p>The unit of the metric.</p>"]
    pub unit: Option<StandardUnit>,
    #[doc="<p>The value for the metric.</p> <p>Although the parameter accepts numbers of type Double, Amazon CloudWatch rejects values that are either too small or too large. Values must be in the range of 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2). In addition, special values (for example, NaN, +Infinity, -Infinity) are not supported.</p>"]
    pub value: Option<DatapointValue>,
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
            DimensionsSerializer::serialize(params,
                                            &format!("{}{}", prefix, "Dimensions"),
                                            field_value);
        }
        params.put(&format!("{}{}", prefix, "MetricName"), &obj.metric_name);
        if let Some(ref field_value) = obj.statistic_values {
            StatisticSetSerializer::serialize(params,
                                              &format!("{}{}", prefix, "StatisticValues"),
                                              field_value);
        }
        if let Some(ref field_value) = obj.timestamp {
            params.put(&format!("{}{}", prefix, "Timestamp"), &field_value);
        }
        if let Some(ref field_value) = obj.unit {
            params.put(&format!("{}{}", prefix, "Unit"), &field_value);
        }
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value.to_string());
        }

    }
}

pub type MetricLabel = String;
struct MetricLabelDeserializer;
impl MetricLabelDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<MetricLabel, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type MetricName = String;
struct MetricNameDeserializer;
impl MetricNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<MetricName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type Metrics = Vec<Metric>;
struct MetricsDeserializer;
impl MetricsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Metrics, XmlParseError> {

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
pub type Namespace = String;
struct NamespaceDeserializer;
impl NamespaceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Namespace, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type NextToken = String;
struct NextTokenDeserializer;
impl NextTokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<NextToken, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type Period = i64;
struct PeriodDeserializer;
impl PeriodDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Period, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug,Clone)]
pub struct PutMetricAlarmInput {
    #[doc="<p>Indicates whether actions should be executed during any changes to the alarm state.</p>"]
    pub actions_enabled: Option<ActionsEnabled>,
    #[doc="<p>The actions to execute when this alarm transitions to the <code>ALARM</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p> <p>Valid Values: arn:aws:automate:<i>region</i>:ec2:stop | arn:aws:automate:<i>region</i>:ec2:terminate | arn:aws:automate:<i>region</i>:ec2:recover</p> <p>Valid Values (for use with IAM roles): arn:aws:swf:us-east-1:{<i>customer-account</i>}:action/actions/AWS_EC2.InstanceId.Stop/1.0 | arn:aws:swf:us-east-1:{<i>customer-account</i>}:action/actions/AWS_EC2.InstanceId.Terminate/1.0 | arn:aws:swf:us-east-1:{<i>customer-account</i>}:action/actions/AWS_EC2.InstanceId.Reboot/1.0</p>"]
    pub alarm_actions: Option<ResourceList>,
    #[doc="<p>The description for the alarm.</p>"]
    pub alarm_description: Option<AlarmDescription>,
    #[doc="<p>The name for the alarm. This name must be unique within the AWS account.</p>"]
    pub alarm_name: AlarmName,
    #[doc="<p> The arithmetic operation to use when comparing the specified statistic and threshold. The specified statistic value is used as the first operand.</p>"]
    pub comparison_operator: ComparisonOperator,
    #[doc="<p>The dimensions for the metric associated with the alarm.</p>"]
    pub dimensions: Option<Dimensions>,
    #[doc="<p> Used only for alarms based on percentiles. If you specify <code>ignore</code>, the alarm state will not change during periods with too few data points to be statistically significant. If you specify <code>evaluate</code> or omit this parameter, the alarm will always be evaluated and possibly change state no matter how many data points are available. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#percentiles-with-low-samples\">Percentile-Based CloudWatch Alarms and Low Data Samples</a>.</p> <p>Valid Values: <code>evaluate | ignore</code> </p>"]
    pub evaluate_low_sample_count_percentile: Option<EvaluateLowSampleCountPercentile>,
    #[doc="<p>The number of periods over which data is compared to the specified threshold.</p>"]
    pub evaluation_periods: EvaluationPeriods,
    #[doc="<p>The percentile statistic for the metric associated with the alarm. Specify a value between p0.0 and p100.</p>"]
    pub extended_statistic: Option<ExtendedStatistic>,
    #[doc="<p>The actions to execute when this alarm transitions to the <code>INSUFFICIENT_DATA</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p> <p>Valid Values: arn:aws:automate:<i>region</i>:ec2:stop | arn:aws:automate:<i>region</i>:ec2:terminate | arn:aws:automate:<i>region</i>:ec2:recover</p> <p>Valid Values (for use with IAM roles): arn:aws:swf:us-east-1:{<i>customer-account</i>}:action/actions/AWS_EC2.InstanceId.Stop/1.0 | arn:aws:swf:us-east-1:{<i>customer-account</i>}:action/actions/AWS_EC2.InstanceId.Terminate/1.0 | arn:aws:swf:us-east-1:{<i>customer-account</i>}:action/actions/AWS_EC2.InstanceId.Reboot/1.0</p>"]
    pub insufficient_data_actions: Option<ResourceList>,
    #[doc="<p>The name for the metric associated with the alarm.</p>"]
    pub metric_name: MetricName,
    #[doc="<p>The namespace for the metric associated with the alarm.</p>"]
    pub namespace: Namespace,
    #[doc="<p>The actions to execute when this alarm transitions to an <code>OK</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p> <p>Valid Values: arn:aws:automate:<i>region</i>:ec2:stop | arn:aws:automate:<i>region</i>:ec2:terminate | arn:aws:automate:<i>region</i>:ec2:recover</p> <p>Valid Values (for use with IAM roles): arn:aws:swf:us-east-1:{<i>customer-account</i>}:action/actions/AWS_EC2.InstanceId.Stop/1.0 | arn:aws:swf:us-east-1:{<i>customer-account</i>}:action/actions/AWS_EC2.InstanceId.Terminate/1.0 | arn:aws:swf:us-east-1:{<i>customer-account</i>}:action/actions/AWS_EC2.InstanceId.Reboot/1.0</p>"]
    pub ok_actions: Option<ResourceList>,
    #[doc="<p>The period, in seconds, over which the specified statistic is applied.</p>"]
    pub period: Period,
    #[doc="<p>The statistic for the metric associated with the alarm, other than percentile. For percentile statistics, use <code>ExtendedStatistic</code>.</p>"]
    pub statistic: Option<Statistic>,
    #[doc="<p>The value against which the specified statistic is compared.</p>"]
    pub threshold: Threshold,
    #[doc="<p> Sets how this alarm is to handle missing data points. If <code>TreatMissingData</code> is omitted, the default behavior of <code>missing</code> is used. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#alarms-and-missing-data\">Configuring How CloudWatch Alarms Treats Missing Data</a>.</p> <p>Valid Values: <code>breaching | notBreaching | ignore | missing</code> </p>"]
    pub treat_missing_data: Option<TreatMissingData>,
    #[doc="<p>The unit of measure for the statistic. For example, the units for the Amazon EC2 NetworkIn metric are Bytes because NetworkIn tracks the number of bytes that an instance receives on all network interfaces. You can also specify a unit when you create a custom metric. Units help provide conceptual meaning to your data. Metric data points that specify a unit of measure, such as Percent, are aggregated separately.</p> <p>If you specify a unit, you must use a unit that is appropriate for the metric. Otherwise, the Amazon CloudWatch alarm can get stuck in the <code>INSUFFICIENT DATA</code> state. </p>"]
    pub unit: Option<StandardUnit>,
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
            params.put(&format!("{}{}", prefix, "ActionsEnabled"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.alarm_actions {
            ResourceListSerializer::serialize(params,
                                              &format!("{}{}", prefix, "AlarmActions"),
                                              field_value);
        }
        if let Some(ref field_value) = obj.alarm_description {
            params.put(&format!("{}{}", prefix, "AlarmDescription"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "AlarmName"), &obj.alarm_name);
        params.put(&format!("{}{}", prefix, "ComparisonOperator"),
                   &obj.comparison_operator);
        if let Some(ref field_value) = obj.dimensions {
            DimensionsSerializer::serialize(params,
                                            &format!("{}{}", prefix, "Dimensions"),
                                            field_value);
        }
        if let Some(ref field_value) = obj.evaluate_low_sample_count_percentile {
            params.put(&format!("{}{}", prefix, "EvaluateLowSampleCountPercentile"),
                       &field_value);
        }
        params.put(&format!("{}{}", prefix, "EvaluationPeriods"),
                   &obj.evaluation_periods.to_string());
        if let Some(ref field_value) = obj.extended_statistic {
            params.put(&format!("{}{}", prefix, "ExtendedStatistic"), &field_value);
        }
        if let Some(ref field_value) = obj.insufficient_data_actions {
            ResourceListSerializer::serialize(params,
                                              &format!("{}{}", prefix, "InsufficientDataActions"),
                                              field_value);
        }
        params.put(&format!("{}{}", prefix, "MetricName"), &obj.metric_name);
        params.put(&format!("{}{}", prefix, "Namespace"), &obj.namespace);
        if let Some(ref field_value) = obj.ok_actions {
            ResourceListSerializer::serialize(params,
                                              &format!("{}{}", prefix, "OKActions"),
                                              field_value);
        }
        params.put(&format!("{}{}", prefix, "Period"), &obj.period.to_string());
        if let Some(ref field_value) = obj.statistic {
            params.put(&format!("{}{}", prefix, "Statistic"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Threshold"),
                   &obj.threshold.to_string());
        if let Some(ref field_value) = obj.treat_missing_data {
            params.put(&format!("{}{}", prefix, "TreatMissingData"), &field_value);
        }
        if let Some(ref field_value) = obj.unit {
            params.put(&format!("{}{}", prefix, "Unit"), &field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct PutMetricDataInput {
    #[doc="<p>The data for the metric.</p>"]
    pub metric_data: MetricData,
    #[doc="<p>The namespace for the metric data.</p> <p>You cannot specify a namespace that begins with \"AWS/\". Namespaces that begin with \"AWS/\" are reserved for use by Amazon Web Services products.</p>"]
    pub namespace: Namespace,
}


/// Serialize `PutMetricDataInput` contents to a `SignedRequest`.
struct PutMetricDataInputSerializer;
impl PutMetricDataInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PutMetricDataInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        MetricDataSerializer::serialize(params,
                                        &format!("{}{}", prefix, "MetricData"),
                                        &obj.metric_data);
        params.put(&format!("{}{}", prefix, "Namespace"), &obj.namespace);

    }
}

pub type ResourceList = Vec<ResourceName>;
struct ResourceListDeserializer;
impl ResourceListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ResourceList, XmlParseError> {

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
    fn serialize(params: &mut Params, name: &str, obj: &ResourceList) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

pub type ResourceName = String;
struct ResourceNameDeserializer;
impl ResourceNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ResourceName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug,Clone)]
pub struct SetAlarmStateInput {
    #[doc="<p>The name for the alarm. This name must be unique within the AWS account. The maximum length is 255 characters.</p>"]
    pub alarm_name: AlarmName,
    #[doc="<p>The reason that this alarm is set to this specific state, in text format.</p>"]
    pub state_reason: StateReason,
    #[doc="<p>The reason that this alarm is set to this specific state, in JSON format.</p>"]
    pub state_reason_data: Option<StateReasonData>,
    #[doc="<p>The value of the state.</p>"]
    pub state_value: StateValue,
}


/// Serialize `SetAlarmStateInput` contents to a `SignedRequest`.
struct SetAlarmStateInputSerializer;
impl SetAlarmStateInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetAlarmStateInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "AlarmName"), &obj.alarm_name);
        params.put(&format!("{}{}", prefix, "StateReason"), &obj.state_reason);
        if let Some(ref field_value) = obj.state_reason_data {
            params.put(&format!("{}{}", prefix, "StateReasonData"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StateValue"), &obj.state_value);

    }
}

pub type StandardUnit = String;
struct StandardUnitDeserializer;
impl StandardUnitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<StandardUnit, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type StateReason = String;
struct StateReasonDeserializer;
impl StateReasonDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<StateReason, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type StateReasonData = String;
struct StateReasonDataDeserializer;
impl StateReasonDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<StateReasonData, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type StateValue = String;
struct StateValueDeserializer;
impl StateValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<StateValue, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type Statistic = String;
struct StatisticDeserializer;
impl StatisticDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Statistic, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[doc="<p>Represents a set of statistics that describes a specific metric. </p>"]
#[derive(Default,Debug,Clone)]
pub struct StatisticSet {
    #[doc="<p>The maximum value of the sample set.</p>"]
    pub maximum: DatapointValue,
    #[doc="<p>The minimum value of the sample set.</p>"]
    pub minimum: DatapointValue,
    #[doc="<p>The number of samples used for the statistic set.</p>"]
    pub sample_count: DatapointValue,
    #[doc="<p>The sum of values for the sample set.</p>"]
    pub sum: DatapointValue,
}


/// Serialize `StatisticSet` contents to a `SignedRequest`.
struct StatisticSetSerializer;
impl StatisticSetSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &StatisticSet) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Maximum"),
                   &obj.maximum.to_string());
        params.put(&format!("{}{}", prefix, "Minimum"),
                   &obj.minimum.to_string());
        params.put(&format!("{}{}", prefix, "SampleCount"),
                   &obj.sample_count.to_string());
        params.put(&format!("{}{}", prefix, "Sum"), &obj.sum.to_string());

    }
}

pub type Statistics = Vec<Statistic>;

/// Serialize `Statistics` contents to a `SignedRequest`.
struct StatisticsSerializer;
impl StatisticsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Statistics) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

pub type Threshold = f64;
struct ThresholdDeserializer;
impl ThresholdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Threshold, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type Timestamp = String;
struct TimestampDeserializer;
impl TimestampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Timestamp, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type TreatMissingData = String;
struct TreatMissingDataDeserializer;
impl TreatMissingDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<TreatMissingData, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
/// Errors returned by DeleteAlarms
#[derive(Debug, PartialEq)]
pub enum DeleteAlarmsError {
    ///<p>The named resource does not exist.</p>
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
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "ResourceNotFound" => {
                        DeleteAlarmsError::ResourceNotFound(String::from(parsed_error.message))
                    }
                    _ => DeleteAlarmsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAlarmsError::Unknown(body.to_string()),
        }
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
/// Errors returned by DescribeAlarmHistory
#[derive(Debug, PartialEq)]
pub enum DescribeAlarmHistoryError {
    ///<p>The next token specified is invalid.</p>
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
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => DescribeAlarmHistoryError::InvalidNextToken(String::from(parsed_error.message)),
                    _ => DescribeAlarmHistoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAlarmHistoryError::Unknown(body.to_string()),
        }
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
    ///<p>The next token specified is invalid.</p>
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
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        DescribeAlarmsError::InvalidNextToken(String::from(parsed_error.message))
                    }
                    _ => DescribeAlarmsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAlarmsError::Unknown(body.to_string()),
        }
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
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DescribeAlarmsForMetricError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAlarmsForMetricError::Unknown(body.to_string()),
        }
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
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DisableAlarmActionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableAlarmActionsError::Unknown(body.to_string()),
        }
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
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => EnableAlarmActionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableAlarmActionsError::Unknown(body.to_string()),
        }
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
/// Errors returned by GetMetricStatistics
#[derive(Debug, PartialEq)]
pub enum GetMetricStatisticsError {
    ///<p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    ///<p>Parameters that cannot be used together were used together.</p>
    InvalidParameterCombination(String),
    ///<p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
    ///<p>An input parameter that is required is missing.</p>
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
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InternalServiceFault" => GetMetricStatisticsError::InternalServiceFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => GetMetricStatisticsError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => GetMetricStatisticsError::InvalidParameterValue(String::from(parsed_error.message)),
                    "MissingRequiredParameterException" => GetMetricStatisticsError::MissingRequiredParameter(String::from(parsed_error.message)),
                    _ => GetMetricStatisticsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetMetricStatisticsError::Unknown(body.to_string()),
        }
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
/// Errors returned by ListMetrics
#[derive(Debug, PartialEq)]
pub enum ListMetricsError {
    ///<p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    ///<p>The value of an input parameter is bad or out-of-range.</p>
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
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InternalServiceFault" => {
                        ListMetricsError::InternalServiceFault(String::from(parsed_error.message))
                    }
                    "InvalidParameterValueException" => {
                        ListMetricsError::InvalidParameterValue(String::from(parsed_error.message))
                    }
                    _ => ListMetricsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListMetricsError::Unknown(body.to_string()),
        }
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
/// Errors returned by PutMetricAlarm
#[derive(Debug, PartialEq)]
pub enum PutMetricAlarmError {
    ///<p>The quota for alarms for this customer has already been reached.</p>
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
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "LimitExceededFault" => {
                        PutMetricAlarmError::LimitExceededFault(String::from(parsed_error.message))
                    }
                    _ => PutMetricAlarmError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutMetricAlarmError::Unknown(body.to_string()),
        }
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
    ///<p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    ///<p>Parameters that cannot be used together were used together.</p>
    InvalidParameterCombination(String),
    ///<p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
    ///<p>An input parameter that is required is missing.</p>
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
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InternalServiceFault" => {
                        PutMetricDataError::InternalServiceFault(String::from(parsed_error.message))
                    }
                    "InvalidParameterCombinationException" => PutMetricDataError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => PutMetricDataError::InvalidParameterValue(String::from(parsed_error.message)),
                    "MissingRequiredParameterException" => PutMetricDataError::MissingRequiredParameter(String::from(parsed_error.message)),
                    _ => PutMetricDataError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutMetricDataError::Unknown(body.to_string()),
        }
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
    ///<p>Data was not syntactically valid JSON.</p>
    InvalidFormatFault(String),
    ///<p>The named resource does not exist.</p>
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
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InvalidFormatFault" => {
                        SetAlarmStateError::InvalidFormatFault(String::from(parsed_error.message))
                    }
                    "ResourceNotFound" => {
                        SetAlarmStateError::ResourceNotFound(String::from(parsed_error.message))
                    }
                    _ => SetAlarmStateError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetAlarmStateError::Unknown(body.to_string()),
        }
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
    #[doc="<p>Deletes the specified alarms. In the event of an error, no alarms are deleted.</p>"]
    fn delete_alarms(&self, input: &DeleteAlarmsInput) -> Result<(), DeleteAlarmsError>;


    #[doc="<p>Retrieves the history for the specified alarm. You can filter the results by date range or item type. If an alarm name is not specified, the histories for all alarms are returned.</p> <p>Note that Amazon CloudWatch retains the history of an alarm even if you delete the alarm.</p>"]
    fn describe_alarm_history(&self,
                              input: &DescribeAlarmHistoryInput)
                              -> Result<DescribeAlarmHistoryOutput, DescribeAlarmHistoryError>;


    #[doc="<p>Retrieves the specified alarms. If no alarms are specified, all alarms are returned. Alarms can be retrieved by using only a prefix for the alarm name, the alarm state, or a prefix for any action.</p>"]
    fn describe_alarms(&self,
                       input: &DescribeAlarmsInput)
                       -> Result<DescribeAlarmsOutput, DescribeAlarmsError>;


    #[doc="<p>Retrieves the alarms for the specified metric. Specify a statistic, period, or unit to filter the results.</p>"]
    fn describe_alarms_for_metric
        (&self,
         input: &DescribeAlarmsForMetricInput)
         -> Result<DescribeAlarmsForMetricOutput, DescribeAlarmsForMetricError>;


    #[doc="<p>Disables the actions for the specified alarms. When an alarm's actions are disabled, the alarm actions do not execute when the alarm state changes.</p>"]
    fn disable_alarm_actions(&self,
                             input: &DisableAlarmActionsInput)
                             -> Result<(), DisableAlarmActionsError>;


    #[doc="<p>Enables the actions for the specified alarms.</p>"]
    fn enable_alarm_actions(&self,
                            input: &EnableAlarmActionsInput)
                            -> Result<(), EnableAlarmActionsError>;


    #[doc="<p>Gets statistics for the specified metric.</p> <p>Amazon CloudWatch retains metric data as follows:</p> <ul> <li> <p>Data points with a period of 60 seconds (1 minute) are available for 15 days</p> </li> <li> <p>Data points with a period of 300 seconds (5 minute) are available for 63 days</p> </li> <li> <p>Data points with a period of 3600 seconds (1 hour) are available for 455 days (15 months)</p> </li> </ul> <p>Note that CloudWatch started retaining 5-minute and 1-hour metric data as of 9 July 2016.</p> <p>The maximum number of data points returned from a single call is 1,440. If you request more than 1,440 data points, Amazon CloudWatch returns an error. To reduce the number of data points, you can narrow the specified time range and make multiple requests across adjacent time ranges, or you can increase the specified period. A period can be as short as one minute (60 seconds). Note that data points are not returned in chronological order.</p> <p>Amazon CloudWatch aggregates data points based on the length of the period that you specify. For example, if you request statistics with a one-hour period, Amazon CloudWatch aggregates all data points with time stamps that fall within each one-hour period. Therefore, the number of values aggregated by CloudWatch is larger than the number of data points returned.</p> <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you cannot retrieve percentile statistics for this data unless one of the following conditions is true:</p> <ul> <li> <p>The SampleCount of the statistic set is 1</p> </li> <li> <p>The Min and the Max of the statistic set are equal</p> </li> </ul> <p>For a list of metrics and dimensions supported by AWS services, see the <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CW_Support_For_AWS.html\">Amazon CloudWatch Metrics and Dimensions Reference</a> in the <i>Amazon CloudWatch User Guide</i>.</p>"]
    fn get_metric_statistics(&self,
                             input: &GetMetricStatisticsInput)
                             -> Result<GetMetricStatisticsOutput, GetMetricStatisticsError>;


    #[doc="<p>List the specified metrics. You can use the returned metrics with <a>GetMetricStatistics</a> to obtain statistical data.</p> <p>Up to 500 results are returned for any one call. To retrieve additional results, use the returned token with subsequent calls.</p> <p>After you create a metric, allow up to fifteen minutes before the metric appears. Statistics about the metric, however, are available sooner using <a>GetMetricStatistics</a>.</p>"]
    fn list_metrics(&self,
                    input: &ListMetricsInput)
                    -> Result<ListMetricsOutput, ListMetricsError>;


    #[doc="<p>Creates or updates an alarm and associates it with the specified metric. Optionally, this operation can associate one or more Amazon SNS resources with the alarm.</p> <p>When this operation creates an alarm, the alarm state is immediately set to <code>INSUFFICIENT_DATA</code>. The alarm is evaluated and its state is set appropriately. Any actions associated with the state are then executed.</p> <p>When you update an existing alarm, its state is left unchanged, but the update completely overwrites the previous configuration of the alarm.</p> <p>If you are an AWS Identity and Access Management (IAM) user, you must have Amazon EC2 permissions for some operations:</p> <ul> <li> <p> <code>ec2:DescribeInstanceStatus</code> and <code>ec2:DescribeInstances</code> for all alarms on EC2 instance status metrics</p> </li> <li> <p> <code>ec2:StopInstances</code> for alarms with stop actions</p> </li> <li> <p> <code>ec2:TerminateInstances</code> for alarms with terminate actions</p> </li> <li> <p> <code>ec2:DescribeInstanceRecoveryAttribute</code> and <code>ec2:RecoverInstances</code> for alarms with recover actions</p> </li> </ul> <p>If you have read/write permissions for Amazon CloudWatch but not for Amazon EC2, you can still create an alarm, but the stop or terminate actions won't be performed. However, if you are later granted the required permissions, the alarm actions that you created earlier will be performed.</p> <p>If you are using an IAM role (for example, an Amazon EC2 instance profile), you cannot stop or terminate the instance using alarm actions. However, you can still see the alarm state and perform any other actions such as Amazon SNS notifications or Auto Scaling policies.</p> <p>If you are using temporary security credentials granted using the AWS Security Token Service (AWS STS), you cannot stop or terminate an Amazon EC2 instance using alarm actions.</p> <p>Note that you must create at least one stop, terminate, or reboot alarm using the Amazon EC2 or CloudWatch console to create the <b>EC2ActionsAccess</b> IAM role. After this IAM role is created, you can create stop, terminate, or reboot alarms using a command-line interface or an API.</p>"]
    fn put_metric_alarm(&self, input: &PutMetricAlarmInput) -> Result<(), PutMetricAlarmError>;


    #[doc="<p>Publishes metric data points to Amazon CloudWatch. Amazon CloudWatch associates the data points with the specified metric. If the specified metric does not exist, Amazon CloudWatch creates the metric. When Amazon CloudWatch creates a metric, it can take up to fifteen minutes for the metric to appear in calls to <a>ListMetrics</a>.</p> <p>Each <code>PutMetricData</code> request is limited to 40 KB in size for HTTP POST requests.</p> <p>Although the <code>Value</code> parameter accepts numbers of type <code>Double</code>, Amazon CloudWatch rejects values that are either too small or too large. Values must be in the range of 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2). In addition, special values (e.g., NaN, +Infinity, -Infinity) are not supported.</p> <p>You can use up to 10 dimensions per metric to further clarify what data the metric collects. For more information on specifying dimensions, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html\">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p> <p>Data points with time stamps from 24 hours ago or longer can take at least 48 hours to become available for <a>GetMetricStatistics</a> from the time they are submitted.</p> <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you cannot retrieve percentile statistics for this data unless one of the following conditions is true:</p> <ul> <li> <p>The SampleCount of the statistic set is 1</p> </li> <li> <p>The Min and the Max of the statistic set are equal</p> </li> </ul>"]
    fn put_metric_data(&self, input: &PutMetricDataInput) -> Result<(), PutMetricDataError>;


    #[doc="<p>Temporarily sets the state of an alarm for testing purposes. When the updated state differs from the previous value, the action configured for the appropriate state is invoked. For example, if your alarm is configured to send an Amazon SNS message when an alarm is triggered, temporarily changing the alarm state to <code>ALARM</code> sends an Amazon SNS message. The alarm returns to its actual state (often within seconds). Because the alarm state change happens very quickly, it is typically only visible in the alarm's <b>History</b> tab in the Amazon CloudWatch console or through <a>DescribeAlarmHistory</a>.</p>"]
    fn set_alarm_state(&self, input: &SetAlarmStateInput) -> Result<(), SetAlarmStateError>;
}
/// A client for the CloudWatch API.
pub struct CloudWatchClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> CloudWatchClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CloudWatchClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> CloudWatch for CloudWatchClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Deletes the specified alarms. In the event of an error, no alarms are deleted.</p>"]
    fn delete_alarms(&self, input: &DeleteAlarmsInput) -> Result<(), DeleteAlarmsError> {
        let mut request = SignedRequest::new("POST", "monitoring", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteAlarms");
        params.put("Version", "2010-08-01");
        DeleteAlarmsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {
                let result = ();
                Ok(result)
            }
            _ => {
                Err(DeleteAlarmsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the history for the specified alarm. You can filter the results by date range or item type. If an alarm name is not specified, the histories for all alarms are returned.</p> <p>Note that Amazon CloudWatch retains the history of an alarm even if you delete the alarm.</p>"]
    fn describe_alarm_history(&self,
                              input: &DescribeAlarmHistoryInput)
                              -> Result<DescribeAlarmHistoryOutput, DescribeAlarmHistoryError> {
        let mut request = SignedRequest::new("POST", "monitoring", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAlarmHistory");
        params.put("Version", "2010-08-01");
        DescribeAlarmHistoryInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = DescribeAlarmHistoryOutput::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeAlarmHistoryOutputDeserializer::deserialize("DescribeAlarmHistoryResult",
                                                                                      &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(DescribeAlarmHistoryError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the specified alarms. If no alarms are specified, all alarms are returned. Alarms can be retrieved by using only a prefix for the alarm name, the alarm state, or a prefix for any action.</p>"]
    fn describe_alarms(&self,
                       input: &DescribeAlarmsInput)
                       -> Result<DescribeAlarmsOutput, DescribeAlarmsError> {
        let mut request = SignedRequest::new("POST", "monitoring", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAlarms");
        params.put("Version", "2010-08-01");
        DescribeAlarmsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = DescribeAlarmsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeAlarmsOutputDeserializer::deserialize("DescribeAlarmsResult",
                                                                                &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(DescribeAlarmsError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves the alarms for the specified metric. Specify a statistic, period, or unit to filter the results.</p>"]
    fn describe_alarms_for_metric
        (&self,
         input: &DescribeAlarmsForMetricInput)
         -> Result<DescribeAlarmsForMetricOutput, DescribeAlarmsForMetricError> {
        let mut request = SignedRequest::new("POST", "monitoring", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAlarmsForMetric");
        params.put("Version", "2010-08-01");
        DescribeAlarmsForMetricInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = DescribeAlarmsForMetricOutput::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result =
                        try!(DescribeAlarmsForMetricOutputDeserializer::deserialize("DescribeAlarmsForMetricResult",
                                                                                    &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(DescribeAlarmsForMetricError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Disables the actions for the specified alarms. When an alarm's actions are disabled, the alarm actions do not execute when the alarm state changes.</p>"]
    fn disable_alarm_actions(&self,
                             input: &DisableAlarmActionsInput)
                             -> Result<(), DisableAlarmActionsError> {
        let mut request = SignedRequest::new("POST", "monitoring", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DisableAlarmActions");
        params.put("Version", "2010-08-01");
        DisableAlarmActionsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {
                let result = ();
                Ok(result)
            }
            _ => {
                Err(DisableAlarmActionsError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Enables the actions for the specified alarms.</p>"]
    fn enable_alarm_actions(&self,
                            input: &EnableAlarmActionsInput)
                            -> Result<(), EnableAlarmActionsError> {
        let mut request = SignedRequest::new("POST", "monitoring", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "EnableAlarmActions");
        params.put("Version", "2010-08-01");
        EnableAlarmActionsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {
                let result = ();
                Ok(result)
            }
            _ => {
                Err(EnableAlarmActionsError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Gets statistics for the specified metric.</p> <p>Amazon CloudWatch retains metric data as follows:</p> <ul> <li> <p>Data points with a period of 60 seconds (1 minute) are available for 15 days</p> </li> <li> <p>Data points with a period of 300 seconds (5 minute) are available for 63 days</p> </li> <li> <p>Data points with a period of 3600 seconds (1 hour) are available for 455 days (15 months)</p> </li> </ul> <p>Note that CloudWatch started retaining 5-minute and 1-hour metric data as of 9 July 2016.</p> <p>The maximum number of data points returned from a single call is 1,440. If you request more than 1,440 data points, Amazon CloudWatch returns an error. To reduce the number of data points, you can narrow the specified time range and make multiple requests across adjacent time ranges, or you can increase the specified period. A period can be as short as one minute (60 seconds). Note that data points are not returned in chronological order.</p> <p>Amazon CloudWatch aggregates data points based on the length of the period that you specify. For example, if you request statistics with a one-hour period, Amazon CloudWatch aggregates all data points with time stamps that fall within each one-hour period. Therefore, the number of values aggregated by CloudWatch is larger than the number of data points returned.</p> <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you cannot retrieve percentile statistics for this data unless one of the following conditions is true:</p> <ul> <li> <p>The SampleCount of the statistic set is 1</p> </li> <li> <p>The Min and the Max of the statistic set are equal</p> </li> </ul> <p>For a list of metrics and dimensions supported by AWS services, see the <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CW_Support_For_AWS.html\">Amazon CloudWatch Metrics and Dimensions Reference</a> in the <i>Amazon CloudWatch User Guide</i>.</p>"]
    fn get_metric_statistics(&self,
                             input: &GetMetricStatisticsInput)
                             -> Result<GetMetricStatisticsOutput, GetMetricStatisticsError> {
        let mut request = SignedRequest::new("POST", "monitoring", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetMetricStatistics");
        params.put("Version", "2010-08-01");
        GetMetricStatisticsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = GetMetricStatisticsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetMetricStatisticsOutputDeserializer::deserialize("GetMetricStatisticsResult",
                                                                                     &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(GetMetricStatisticsError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>List the specified metrics. You can use the returned metrics with <a>GetMetricStatistics</a> to obtain statistical data.</p> <p>Up to 500 results are returned for any one call. To retrieve additional results, use the returned token with subsequent calls.</p> <p>After you create a metric, allow up to fifteen minutes before the metric appears. Statistics about the metric, however, are available sooner using <a>GetMetricStatistics</a>.</p>"]
    fn list_metrics(&self,
                    input: &ListMetricsInput)
                    -> Result<ListMetricsOutput, ListMetricsError> {
        let mut request = SignedRequest::new("POST", "monitoring", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListMetrics");
        params.put("Version", "2010-08-01");
        ListMetricsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = ListMetricsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListMetricsOutputDeserializer::deserialize("ListMetricsResult",
                                                                             &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => Err(ListMetricsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates or updates an alarm and associates it with the specified metric. Optionally, this operation can associate one or more Amazon SNS resources with the alarm.</p> <p>When this operation creates an alarm, the alarm state is immediately set to <code>INSUFFICIENT_DATA</code>. The alarm is evaluated and its state is set appropriately. Any actions associated with the state are then executed.</p> <p>When you update an existing alarm, its state is left unchanged, but the update completely overwrites the previous configuration of the alarm.</p> <p>If you are an AWS Identity and Access Management (IAM) user, you must have Amazon EC2 permissions for some operations:</p> <ul> <li> <p> <code>ec2:DescribeInstanceStatus</code> and <code>ec2:DescribeInstances</code> for all alarms on EC2 instance status metrics</p> </li> <li> <p> <code>ec2:StopInstances</code> for alarms with stop actions</p> </li> <li> <p> <code>ec2:TerminateInstances</code> for alarms with terminate actions</p> </li> <li> <p> <code>ec2:DescribeInstanceRecoveryAttribute</code> and <code>ec2:RecoverInstances</code> for alarms with recover actions</p> </li> </ul> <p>If you have read/write permissions for Amazon CloudWatch but not for Amazon EC2, you can still create an alarm, but the stop or terminate actions won't be performed. However, if you are later granted the required permissions, the alarm actions that you created earlier will be performed.</p> <p>If you are using an IAM role (for example, an Amazon EC2 instance profile), you cannot stop or terminate the instance using alarm actions. However, you can still see the alarm state and perform any other actions such as Amazon SNS notifications or Auto Scaling policies.</p> <p>If you are using temporary security credentials granted using the AWS Security Token Service (AWS STS), you cannot stop or terminate an Amazon EC2 instance using alarm actions.</p> <p>Note that you must create at least one stop, terminate, or reboot alarm using the Amazon EC2 or CloudWatch console to create the <b>EC2ActionsAccess</b> IAM role. After this IAM role is created, you can create stop, terminate, or reboot alarms using a command-line interface or an API.</p>"]
    fn put_metric_alarm(&self, input: &PutMetricAlarmInput) -> Result<(), PutMetricAlarmError> {
        let mut request = SignedRequest::new("POST", "monitoring", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutMetricAlarm");
        params.put("Version", "2010-08-01");
        PutMetricAlarmInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {
                let result = ();
                Ok(result)
            }
            _ => {
                Err(PutMetricAlarmError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Publishes metric data points to Amazon CloudWatch. Amazon CloudWatch associates the data points with the specified metric. If the specified metric does not exist, Amazon CloudWatch creates the metric. When Amazon CloudWatch creates a metric, it can take up to fifteen minutes for the metric to appear in calls to <a>ListMetrics</a>.</p> <p>Each <code>PutMetricData</code> request is limited to 40 KB in size for HTTP POST requests.</p> <p>Although the <code>Value</code> parameter accepts numbers of type <code>Double</code>, Amazon CloudWatch rejects values that are either too small or too large. Values must be in the range of 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2). In addition, special values (e.g., NaN, +Infinity, -Infinity) are not supported.</p> <p>You can use up to 10 dimensions per metric to further clarify what data the metric collects. For more information on specifying dimensions, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html\">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p> <p>Data points with time stamps from 24 hours ago or longer can take at least 48 hours to become available for <a>GetMetricStatistics</a> from the time they are submitted.</p> <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you cannot retrieve percentile statistics for this data unless one of the following conditions is true:</p> <ul> <li> <p>The SampleCount of the statistic set is 1</p> </li> <li> <p>The Min and the Max of the statistic set are equal</p> </li> </ul>"]
    fn put_metric_data(&self, input: &PutMetricDataInput) -> Result<(), PutMetricDataError> {
        let mut request = SignedRequest::new("POST", "monitoring", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutMetricData");
        params.put("Version", "2010-08-01");
        PutMetricDataInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {
                let result = ();
                Ok(result)
            }
            _ => {
                Err(PutMetricDataError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Temporarily sets the state of an alarm for testing purposes. When the updated state differs from the previous value, the action configured for the appropriate state is invoked. For example, if your alarm is configured to send an Amazon SNS message when an alarm is triggered, temporarily changing the alarm state to <code>ALARM</code> sends an Amazon SNS message. The alarm returns to its actual state (often within seconds). Because the alarm state change happens very quickly, it is typically only visible in the alarm's <b>History</b> tab in the Amazon CloudWatch console or through <a>DescribeAlarmHistory</a>.</p>"]
    fn set_alarm_state(&self, input: &SetAlarmStateInput) -> Result<(), SetAlarmStateError> {
        let mut request = SignedRequest::new("POST", "monitoring", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetAlarmState");
        params.put("Version", "2010-08-01");
        SetAlarmStateInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {
                let result = ();
                Ok(result)
            }
            _ => {
                Err(SetAlarmStateError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use super::*;
    use self::rusoto_mock::*;
    use rusoto_core::Region as rusoto_region;


    #[test]
    fn test_parse_error_cloudwatch_describe_alarm_history() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/error",
                                                              "cloudwatch-describe-alarm-history.xml");
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = CloudWatchClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAlarmHistoryInput::default();
        let result = client.describe_alarm_history(&request);
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudwatch_describe_alarm_history() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "cloudwatch-describe-alarm-history.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudWatchClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAlarmHistoryInput::default();
        let result = client.describe_alarm_history(&request);
        assert!(result.is_ok(), "parse error: {:?}", result);
    }


    #[test]
    fn test_parse_valid_cloudwatch_describe_alarms() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "cloudwatch-describe-alarms.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudWatchClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAlarmsInput::default();
        let result = client.describe_alarms(&request);
        assert!(result.is_ok(), "parse error: {:?}", result);
    }


    #[test]
    fn test_parse_valid_cloudwatch_list_metrics() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "cloudwatch-list-metrics.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudWatchClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListMetricsInput::default();
        let result = client.list_metrics(&request);
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
