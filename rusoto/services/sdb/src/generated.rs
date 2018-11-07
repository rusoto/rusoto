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
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use rusoto_core::xmlerror::*;
use rusoto_core::xmlutil::{
    characters, end_element, find_start_element, peek_at_name, skip_tree, start_element,
};
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use serde_urlencoded;
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::reader::XmlEvent;
use xml::EventReader;

enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
/// <p></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Attribute {
    /// <p></p>
    pub alternate_name_encoding: Option<String>,
    /// <p></p>
    pub alternate_value_encoding: Option<String>,
    /// <p>The name of the attribute.</p>
    pub name: String,
    /// <p>The value of the attribute.</p>
    pub value: String,
}

struct AttributeDeserializer;
impl AttributeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Attribute, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Attribute::default();

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
                        "AlternateNameEncoding" => {
                            obj.alternate_name_encoding = Some(try!(
                                StringDeserializer::deserialize("AlternateNameEncoding", stack)
                            ));
                        }
                        "AlternateValueEncoding" => {
                            obj.alternate_value_encoding = Some(try!(
                                StringDeserializer::deserialize("AlternateValueEncoding", stack)
                            ));
                        }
                        "Name" => {
                            obj.name = try!(StringDeserializer::deserialize("Name", stack));
                        }
                        "Value" => {
                            obj.value = try!(StringDeserializer::deserialize("Value", stack));
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

/// Serialize `Attribute` contents to a `SignedRequest`.
struct AttributeSerializer;
impl AttributeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Attribute) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.alternate_name_encoding {
            params.put(
                &format!("{}{}", prefix, "AlternateNameEncoding"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.alternate_value_encoding {
            params.put(
                &format!("{}{}", prefix, "AlternateValueEncoding"),
                &field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        params.put(&format!("{}{}", prefix, "Value"), &obj.value);
    }
}

struct AttributeListDeserializer;
impl AttributeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Attribute>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(AttributeDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

/// Serialize `AttributeList` contents to a `SignedRequest`.
struct AttributeListSerializer;
impl AttributeListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Attribute>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            AttributeSerializer::serialize(params, &key, obj);
        }
    }
}

/// Serialize `AttributeNameList` contents to a `SignedRequest`.
struct AttributeNameListSerializer;
impl AttributeNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BatchDeleteAttributesRequest {
    /// <p>The name of the domain in which the attributes are being deleted.</p>
    pub domain_name: String,
    /// <p>A list of items on which to perform the operation.</p>
    pub items: Vec<DeletableItem>,
}

/// Serialize `BatchDeleteAttributesRequest` contents to a `SignedRequest`.
struct BatchDeleteAttributesRequestSerializer;
impl BatchDeleteAttributesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &BatchDeleteAttributesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        DeletableItemListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Item"),
            &obj.items,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BatchPutAttributesRequest {
    /// <p>The name of the domain in which the attributes are being stored.</p>
    pub domain_name: String,
    /// <p>A list of items on which to perform the operation.</p>
    pub items: Vec<ReplaceableItem>,
}

/// Serialize `BatchPutAttributesRequest` contents to a `SignedRequest`.
struct BatchPutAttributesRequestSerializer;
impl BatchPutAttributesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &BatchPutAttributesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        ReplaceableItemListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Item"),
            &obj.items,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateDomainRequest {
    /// <p>The name of the domain to create. The name can range between 3 and 255 characters and can contain the following characters: a-z, A-Z, 0-9, &#39;_&#39;, &#39;-&#39;, and &#39;.&#39;.</p>
    pub domain_name: String,
}

/// Serialize `CreateDomainRequest` contents to a `SignedRequest`.
struct CreateDomainRequestSerializer;
impl CreateDomainRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateDomainRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeletableItem {
    pub attributes: Option<Vec<Attribute>>,
    pub name: String,
}

/// Serialize `DeletableItem` contents to a `SignedRequest`.
struct DeletableItemSerializer;
impl DeletableItemSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeletableItem) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.attributes {
            AttributeListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Attribute"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "ItemName"), &obj.name);
    }
}

/// Serialize `DeletableItemList` contents to a `SignedRequest`.
struct DeletableItemListSerializer;
impl DeletableItemListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<DeletableItem>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            DeletableItemSerializer::serialize(params, &key, obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteAttributesRequest {
    /// <p>A list of Attributes. Similar to columns on a spreadsheet, attributes represent categories of data that can be assigned to items.</p>
    pub attributes: Option<Vec<Attribute>>,
    /// <p>The name of the domain in which to perform the operation.</p>
    pub domain_name: String,
    /// <p>The update condition which, if specified, determines whether the specified attributes will be deleted or not. The update condition must be satisfied in order for this request to be processed and the attributes to be deleted.</p>
    pub expected: Option<UpdateCondition>,
    /// <p>The name of the item. Similar to rows on a spreadsheet, items represent individual objects that contain one or more value-attribute pairs.</p>
    pub item_name: String,
}

/// Serialize `DeleteAttributesRequest` contents to a `SignedRequest`.
struct DeleteAttributesRequestSerializer;
impl DeleteAttributesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteAttributesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.attributes {
            AttributeListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Attribute"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        if let Some(ref field_value) = obj.expected {
            UpdateConditionSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Expected"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "ItemName"), &obj.item_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDomainRequest {
    /// <p>The name of the domain to delete.</p>
    pub domain_name: String,
}

/// Serialize `DeleteDomainRequest` contents to a `SignedRequest`.
struct DeleteDomainRequestSerializer;
impl DeleteDomainRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteDomainRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DomainMetadataRequest {
    /// <p>The name of the domain for which to display the metadata of.</p>
    pub domain_name: String,
}

/// Serialize `DomainMetadataRequest` contents to a `SignedRequest`.
struct DomainMetadataRequestSerializer;
impl DomainMetadataRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DomainMetadataRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DomainMetadataResult {
    /// <p>The number of unique attribute names in the domain.</p>
    pub attribute_name_count: Option<i64>,
    /// <p>The total size of all unique attribute names in the domain, in bytes.</p>
    pub attribute_names_size_bytes: Option<i64>,
    /// <p>The number of all attribute name/value pairs in the domain.</p>
    pub attribute_value_count: Option<i64>,
    /// <p>The total size of all attribute values in the domain, in bytes.</p>
    pub attribute_values_size_bytes: Option<i64>,
    /// <p>The number of all items in the domain.</p>
    pub item_count: Option<i64>,
    /// <p>The total size of all item names in the domain, in bytes.</p>
    pub item_names_size_bytes: Option<i64>,
    /// <p>The data and time when metadata was calculated, in Epoch (UNIX) seconds.</p>
    pub timestamp: Option<i64>,
}

struct DomainMetadataResultDeserializer;
impl DomainMetadataResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DomainMetadataResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DomainMetadataResult::default();

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
                        "AttributeNameCount" => {
                            obj.attribute_name_count = Some(try!(
                                IntegerDeserializer::deserialize("AttributeNameCount", stack)
                            ));
                        }
                        "AttributeNamesSizeBytes" => {
                            obj.attribute_names_size_bytes = Some(try!(
                                LongDeserializer::deserialize("AttributeNamesSizeBytes", stack)
                            ));
                        }
                        "AttributeValueCount" => {
                            obj.attribute_value_count = Some(try!(
                                IntegerDeserializer::deserialize("AttributeValueCount", stack)
                            ));
                        }
                        "AttributeValuesSizeBytes" => {
                            obj.attribute_values_size_bytes = Some(try!(
                                LongDeserializer::deserialize("AttributeValuesSizeBytes", stack)
                            ));
                        }
                        "ItemCount" => {
                            obj.item_count =
                                Some(try!(IntegerDeserializer::deserialize("ItemCount", stack)));
                        }
                        "ItemNamesSizeBytes" => {
                            obj.item_names_size_bytes = Some(try!(LongDeserializer::deserialize(
                                "ItemNamesSizeBytes",
                                stack
                            )));
                        }
                        "Timestamp" => {
                            obj.timestamp =
                                Some(try!(IntegerDeserializer::deserialize("Timestamp", stack)));
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
struct DomainNameListDeserializer;
impl DomainNameListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(StringDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetAttributesRequest {
    /// <p>The names of the attributes.</p>
    pub attribute_names: Option<Vec<String>>,
    /// <p>Determines whether or not strong consistency should be enforced when data is read from SimpleDB. If <code>true</code>, any data previously written to SimpleDB will be returned. Otherwise, results will be consistent eventually, and the client may not see data that was written immediately before your read.</p>
    pub consistent_read: Option<bool>,
    /// <p>The name of the domain in which to perform the operation.</p>
    pub domain_name: String,
    /// <p>The name of the item.</p>
    pub item_name: String,
}

/// Serialize `GetAttributesRequest` contents to a `SignedRequest`.
struct GetAttributesRequestSerializer;
impl GetAttributesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetAttributesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.attribute_names {
            AttributeNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AttributeName"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.consistent_read {
            params.put(
                &format!("{}{}", prefix, "ConsistentRead"),
                &field_value.to_string(),
            );
        }
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        params.put(&format!("{}{}", prefix, "ItemName"), &obj.item_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetAttributesResult {
    /// <p>The list of attributes returned by the operation.</p>
    pub attributes: Option<Vec<Attribute>>,
}

struct GetAttributesResultDeserializer;
impl GetAttributesResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetAttributesResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetAttributesResult::default();

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
                    "Attribute" => {
                        obj.attributes = match obj.attributes {
                            Some(existing) => Some(existing.append(try!(
                                AttributeListDeserializer::deserialize("Attribute", stack)
                            ))),
                            None => Some(try!(AttributeListDeserializer::deserialize(
                                "Attribute",
                                stack
                            ))),
                        };
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
struct IntegerDeserializer;
impl IntegerDeserializer {
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
/// <p></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Item {
    /// <p></p>
    pub alternate_name_encoding: Option<String>,
    /// <p>A list of attributes.</p>
    pub attributes: Vec<Attribute>,
    /// <p>The name of the item.</p>
    pub name: String,
}

struct ItemDeserializer;
impl ItemDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Item, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Item::default();

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
                    "AlternateNameEncoding" => {
                        obj.alternate_name_encoding = Some(try!(StringDeserializer::deserialize(
                            "AlternateNameEncoding",
                            stack
                        )));
                    }
                    "Attribute" => {
                        obj.attributes = match obj.attributes {
                            Some(existing) => Some(existing.append(try!(
                                AttributeListDeserializer::deserialize("Attribute", stack)
                            ))),
                            None => Some(try!(AttributeListDeserializer::deserialize(
                                "Attribute",
                                stack
                            ))),
                        };
                    }
                    "Name" => {
                        obj.name = try!(StringDeserializer::deserialize("Name", stack));
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
struct ItemListDeserializer;
impl ItemListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Item>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(ItemDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListDomainsRequest {
    /// <p>The maximum number of domain names you want returned. The range is 1 to 100. The default setting is 100.</p>
    pub max_number_of_domains: Option<i64>,
    /// <p>A string informing Amazon SimpleDB where to start the next list of domain names.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListDomainsRequest` contents to a `SignedRequest`.
struct ListDomainsRequestSerializer;
impl ListDomainsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListDomainsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_number_of_domains {
            params.put(
                &format!("{}{}", prefix, "MaxNumberOfDomains"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListDomainsResult {
    /// <p>A list of domain names that match the expression.</p>
    pub domain_names: Option<Vec<String>>,
    /// <p>An opaque token indicating that there are more domains than the specified <code>MaxNumberOfDomains</code> still available.</p>
    pub next_token: Option<String>,
}

struct ListDomainsResultDeserializer;
impl ListDomainsResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDomainsResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListDomainsResult::default();

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
                    "DomainName" => {
                        obj.domain_names = match obj.domain_names {
                            Some(existing) => Some(existing.append(try!(
                                DomainNameListDeserializer::deserialize("DomainName", stack)
                            ))),
                            None => Some(try!(DomainNameListDeserializer::deserialize(
                                "DomainName",
                                stack
                            ))),
                        };
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(StringDeserializer::deserialize("NextToken", stack)));
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
struct LongDeserializer;
impl LongDeserializer {
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
pub struct PutAttributesRequest {
    /// <p>The list of attributes.</p>
    pub attributes: Vec<ReplaceableAttribute>,
    /// <p>The name of the domain in which to perform the operation.</p>
    pub domain_name: String,
    /// <p>The update condition which, if specified, determines whether the specified attributes will be updated or not. The update condition must be satisfied in order for this request to be processed and the attributes to be updated.</p>
    pub expected: Option<UpdateCondition>,
    /// <p>The name of the item.</p>
    pub item_name: String,
}

/// Serialize `PutAttributesRequest` contents to a `SignedRequest`.
struct PutAttributesRequestSerializer;
impl PutAttributesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PutAttributesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ReplaceableAttributeListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Attribute"),
            &obj.attributes,
        );
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        if let Some(ref field_value) = obj.expected {
            UpdateConditionSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Expected"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "ItemName"), &obj.item_name);
    }
}

/// <p></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReplaceableAttribute {
    /// <p>The name of the replaceable attribute.</p>
    pub name: String,
    /// <p>A flag specifying whether or not to replace the attribute/value pair or to add a new attribute/value pair. The default setting is <code>false</code>.</p>
    pub replace: Option<bool>,
    /// <p>The value of the replaceable attribute.</p>
    pub value: String,
}

/// Serialize `ReplaceableAttribute` contents to a `SignedRequest`.
struct ReplaceableAttributeSerializer;
impl ReplaceableAttributeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ReplaceableAttribute) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        if let Some(ref field_value) = obj.replace {
            params.put(
                &format!("{}{}", prefix, "Replace"),
                &field_value.to_string(),
            );
        }
        params.put(&format!("{}{}", prefix, "Value"), &obj.value);
    }
}

/// Serialize `ReplaceableAttributeList` contents to a `SignedRequest`.
struct ReplaceableAttributeListSerializer;
impl ReplaceableAttributeListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<ReplaceableAttribute>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            ReplaceableAttributeSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReplaceableItem {
    /// <p>The list of attributes for a replaceable item.</p>
    pub attributes: Vec<ReplaceableAttribute>,
    /// <p>The name of the replaceable item.</p>
    pub name: String,
}

/// Serialize `ReplaceableItem` contents to a `SignedRequest`.
struct ReplaceableItemSerializer;
impl ReplaceableItemSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ReplaceableItem) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ReplaceableAttributeListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Attribute"),
            &obj.attributes,
        );
        params.put(&format!("{}{}", prefix, "ItemName"), &obj.name);
    }
}

/// Serialize `ReplaceableItemList` contents to a `SignedRequest`.
struct ReplaceableItemListSerializer;
impl ReplaceableItemListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<ReplaceableItem>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            ReplaceableItemSerializer::serialize(params, &key, obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectRequest {
    /// <p>Determines whether or not strong consistency should be enforced when data is read from SimpleDB. If <code>true</code>, any data previously written to SimpleDB will be returned. Otherwise, results will be consistent eventually, and the client may not see data that was written immediately before your read.</p>
    pub consistent_read: Option<bool>,
    /// <p>A string informing Amazon SimpleDB where to start the next list of <code>ItemNames</code>.</p>
    pub next_token: Option<String>,
    /// <p>The expression used to query the domain.</p>
    pub select_expression: String,
}

/// Serialize `SelectRequest` contents to a `SignedRequest`.
struct SelectRequestSerializer;
impl SelectRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SelectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.consistent_read {
            params.put(
                &format!("{}{}", prefix, "ConsistentRead"),
                &field_value.to_string(),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "SelectExpression"),
            &obj.select_expression,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectResult {
    /// <p>A list of items that match the select expression.</p>
    pub items: Option<Vec<Item>>,
    /// <p>An opaque token indicating that more items than <code>MaxNumberOfItems</code> were matched, the response size exceeded 1 megabyte, or the execution time exceeded 5 seconds.</p>
    pub next_token: Option<String>,
}

struct SelectResultDeserializer;
impl SelectResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SelectResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SelectResult::default();

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
                    "Item" => {
                        obj.items = match obj.items {
                            Some(existing) => Some(
                                existing
                                    .append(try!(ItemListDeserializer::deserialize("Item", stack))),
                            ),
                            None => Some(try!(ItemListDeserializer::deserialize("Item", stack))),
                        };
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(StringDeserializer::deserialize("NextToken", stack)));
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
struct StringDeserializer;
impl StringDeserializer {
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
/// <p> Specifies the conditions under which data should be updated. If an update condition is specified for a request, the data will only be updated if the condition is satisfied. For example, if an attribute with a specific name and value exists, or if a specific attribute doesn't exist. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateCondition {
    /// <p>A value specifying whether or not the specified attribute must exist with the specified value in order for the update condition to be satisfied. Specify <code>true</code> if the attribute must exist for the update condition to be satisfied. Specify <code>false</code> if the attribute should not exist in order for the update condition to be satisfied.</p>
    pub exists: Option<bool>,
    /// <p>The name of the attribute involved in the condition.</p>
    pub name: Option<String>,
    /// <p>The value of an attribute. This value can only be specified when the <code>Exists</code> parameter is equal to <code>true</code>.</p>
    pub value: Option<String>,
}

/// Serialize `UpdateCondition` contents to a `SignedRequest`.
struct UpdateConditionSerializer;
impl UpdateConditionSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateCondition) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.exists {
            params.put(&format!("{}{}", prefix, "Exists"), &field_value.to_string());
        }
        if let Some(ref field_value) = obj.name {
            params.put(&format!("{}{}", prefix, "Name"), &field_value);
        }
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }
    }
}

/// Errors returned by BatchDeleteAttributes
#[derive(Debug, PartialEq)]
pub enum BatchDeleteAttributesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl BatchDeleteAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchDeleteAttributesError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        BatchDeleteAttributesError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for BatchDeleteAttributesError {
    fn from(err: XmlParseError) -> BatchDeleteAttributesError {
        let XmlParseError(message) = err;
        BatchDeleteAttributesError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for BatchDeleteAttributesError {
    fn from(err: CredentialsError) -> BatchDeleteAttributesError {
        BatchDeleteAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchDeleteAttributesError {
    fn from(err: HttpDispatchError) -> BatchDeleteAttributesError {
        BatchDeleteAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchDeleteAttributesError {
    fn from(err: io::Error) -> BatchDeleteAttributesError {
        BatchDeleteAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchDeleteAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeleteAttributesError {
    fn description(&self) -> &str {
        match *self {
            BatchDeleteAttributesError::Validation(ref cause) => cause,
            BatchDeleteAttributesError::Credentials(ref err) => err.description(),
            BatchDeleteAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchDeleteAttributesError::ParseError(ref cause) => cause,
            BatchDeleteAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by BatchPutAttributes
#[derive(Debug, PartialEq)]
pub enum BatchPutAttributesError {
    /// <p>The item name was specified more than once. </p>
    DuplicateItemName(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain the specified missing parameter.</p>
    MissingParameter(String),
    /// <p>The specified domain does not exist.</p>
    NoSuchDomain(String),
    /// <p>Too many attributes in this domain.</p>
    NumberDomainAttributesExceeded(String),
    /// <p>Too many bytes in this domain.</p>
    NumberDomainBytesExceeded(String),
    /// <p>Too many attributes in this item.</p>
    NumberItemAttributesExceeded(String),
    /// <p>Too many attributes exist in a single call.</p>
    NumberSubmittedAttributesExceeded(String),
    /// <p>Too many items exist in a single call.</p>
    NumberSubmittedItemsExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl BatchPutAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchPutAttributesError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DuplicateItemName" => {
                        return BatchPutAttributesError::DuplicateItemName(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return BatchPutAttributesError::InvalidParameterValue(String::from(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameter" => {
                        return BatchPutAttributesError::MissingParameter(String::from(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDomain" => {
                        return BatchPutAttributesError::NoSuchDomain(String::from(
                            parsed_error.message,
                        ))
                    }
                    "NumberDomainAttributesExceeded" => {
                        return BatchPutAttributesError::NumberDomainAttributesExceeded(
                            String::from(parsed_error.message),
                        )
                    }
                    "NumberDomainBytesExceeded" => {
                        return BatchPutAttributesError::NumberDomainBytesExceeded(String::from(
                            parsed_error.message,
                        ))
                    }
                    "NumberItemAttributesExceeded" => {
                        return BatchPutAttributesError::NumberItemAttributesExceeded(String::from(
                            parsed_error.message,
                        ))
                    }
                    "NumberSubmittedAttributesExceeded" => {
                        return BatchPutAttributesError::NumberSubmittedAttributesExceeded(
                            String::from(parsed_error.message),
                        )
                    }
                    "NumberSubmittedItemsExceeded" => {
                        return BatchPutAttributesError::NumberSubmittedItemsExceeded(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        BatchPutAttributesError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for BatchPutAttributesError {
    fn from(err: XmlParseError) -> BatchPutAttributesError {
        let XmlParseError(message) = err;
        BatchPutAttributesError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for BatchPutAttributesError {
    fn from(err: CredentialsError) -> BatchPutAttributesError {
        BatchPutAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchPutAttributesError {
    fn from(err: HttpDispatchError) -> BatchPutAttributesError {
        BatchPutAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchPutAttributesError {
    fn from(err: io::Error) -> BatchPutAttributesError {
        BatchPutAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchPutAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchPutAttributesError {
    fn description(&self) -> &str {
        match *self {
            BatchPutAttributesError::DuplicateItemName(ref cause) => cause,
            BatchPutAttributesError::InvalidParameterValue(ref cause) => cause,
            BatchPutAttributesError::MissingParameter(ref cause) => cause,
            BatchPutAttributesError::NoSuchDomain(ref cause) => cause,
            BatchPutAttributesError::NumberDomainAttributesExceeded(ref cause) => cause,
            BatchPutAttributesError::NumberDomainBytesExceeded(ref cause) => cause,
            BatchPutAttributesError::NumberItemAttributesExceeded(ref cause) => cause,
            BatchPutAttributesError::NumberSubmittedAttributesExceeded(ref cause) => cause,
            BatchPutAttributesError::NumberSubmittedItemsExceeded(ref cause) => cause,
            BatchPutAttributesError::Validation(ref cause) => cause,
            BatchPutAttributesError::Credentials(ref err) => err.description(),
            BatchPutAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchPutAttributesError::ParseError(ref cause) => cause,
            BatchPutAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDomain
#[derive(Debug, PartialEq)]
pub enum CreateDomainError {
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain the specified missing parameter.</p>
    MissingParameter(String),
    /// <p>Too many domains exist per this account.</p>
    NumberDomainsExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDomainError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterValue" => {
                        return CreateDomainError::InvalidParameterValue(String::from(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameter" => {
                        return CreateDomainError::MissingParameter(String::from(
                            parsed_error.message,
                        ))
                    }
                    "NumberDomainsExceeded" => {
                        return CreateDomainError::NumberDomainsExceeded(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        CreateDomainError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateDomainError {
    fn from(err: XmlParseError) -> CreateDomainError {
        let XmlParseError(message) = err;
        CreateDomainError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for CreateDomainError {
    fn from(err: CredentialsError) -> CreateDomainError {
        CreateDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDomainError {
    fn from(err: HttpDispatchError) -> CreateDomainError {
        CreateDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDomainError {
    fn from(err: io::Error) -> CreateDomainError {
        CreateDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDomainError {
    fn description(&self) -> &str {
        match *self {
            CreateDomainError::InvalidParameterValue(ref cause) => cause,
            CreateDomainError::MissingParameter(ref cause) => cause,
            CreateDomainError::NumberDomainsExceeded(ref cause) => cause,
            CreateDomainError::Validation(ref cause) => cause,
            CreateDomainError::Credentials(ref err) => err.description(),
            CreateDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDomainError::ParseError(ref cause) => cause,
            CreateDomainError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteAttributes
#[derive(Debug, PartialEq)]
pub enum DeleteAttributesError {
    /// <p>The specified attribute does not exist.</p>
    AttributeDoesNotExist(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain the specified missing parameter.</p>
    MissingParameter(String),
    /// <p>The specified domain does not exist.</p>
    NoSuchDomain(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteAttributesError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AttributeDoesNotExist" => {
                        return DeleteAttributesError::AttributeDoesNotExist(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return DeleteAttributesError::InvalidParameterValue(String::from(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameter" => {
                        return DeleteAttributesError::MissingParameter(String::from(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDomain" => {
                        return DeleteAttributesError::NoSuchDomain(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        DeleteAttributesError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteAttributesError {
    fn from(err: XmlParseError) -> DeleteAttributesError {
        let XmlParseError(message) = err;
        DeleteAttributesError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DeleteAttributesError {
    fn from(err: CredentialsError) -> DeleteAttributesError {
        DeleteAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAttributesError {
    fn from(err: HttpDispatchError) -> DeleteAttributesError {
        DeleteAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAttributesError {
    fn from(err: io::Error) -> DeleteAttributesError {
        DeleteAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAttributesError {
    fn description(&self) -> &str {
        match *self {
            DeleteAttributesError::AttributeDoesNotExist(ref cause) => cause,
            DeleteAttributesError::InvalidParameterValue(ref cause) => cause,
            DeleteAttributesError::MissingParameter(ref cause) => cause,
            DeleteAttributesError::NoSuchDomain(ref cause) => cause,
            DeleteAttributesError::Validation(ref cause) => cause,
            DeleteAttributesError::Credentials(ref err) => err.description(),
            DeleteAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAttributesError::ParseError(ref cause) => cause,
            DeleteAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDomain
#[derive(Debug, PartialEq)]
pub enum DeleteDomainError {
    /// <p>The request must contain the specified missing parameter.</p>
    MissingParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDomainError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "MissingParameter" => {
                        return DeleteDomainError::MissingParameter(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        DeleteDomainError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteDomainError {
    fn from(err: XmlParseError) -> DeleteDomainError {
        let XmlParseError(message) = err;
        DeleteDomainError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DeleteDomainError {
    fn from(err: CredentialsError) -> DeleteDomainError {
        DeleteDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDomainError {
    fn from(err: HttpDispatchError) -> DeleteDomainError {
        DeleteDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDomainError {
    fn from(err: io::Error) -> DeleteDomainError {
        DeleteDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDomainError {
    fn description(&self) -> &str {
        match *self {
            DeleteDomainError::MissingParameter(ref cause) => cause,
            DeleteDomainError::Validation(ref cause) => cause,
            DeleteDomainError::Credentials(ref err) => err.description(),
            DeleteDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDomainError::ParseError(ref cause) => cause,
            DeleteDomainError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DomainMetadata
#[derive(Debug, PartialEq)]
pub enum DomainMetadataError {
    /// <p>The request must contain the specified missing parameter.</p>
    MissingParameter(String),
    /// <p>The specified domain does not exist.</p>
    NoSuchDomain(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DomainMetadataError {
    pub fn from_response(res: BufferedHttpResponse) -> DomainMetadataError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "MissingParameter" => {
                        return DomainMetadataError::MissingParameter(String::from(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDomain" => {
                        return DomainMetadataError::NoSuchDomain(String::from(parsed_error.message))
                    }
                    _ => {}
                }
            }
        }
        DomainMetadataError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DomainMetadataError {
    fn from(err: XmlParseError) -> DomainMetadataError {
        let XmlParseError(message) = err;
        DomainMetadataError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for DomainMetadataError {
    fn from(err: CredentialsError) -> DomainMetadataError {
        DomainMetadataError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DomainMetadataError {
    fn from(err: HttpDispatchError) -> DomainMetadataError {
        DomainMetadataError::HttpDispatch(err)
    }
}
impl From<io::Error> for DomainMetadataError {
    fn from(err: io::Error) -> DomainMetadataError {
        DomainMetadataError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DomainMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DomainMetadataError {
    fn description(&self) -> &str {
        match *self {
            DomainMetadataError::MissingParameter(ref cause) => cause,
            DomainMetadataError::NoSuchDomain(ref cause) => cause,
            DomainMetadataError::Validation(ref cause) => cause,
            DomainMetadataError::Credentials(ref err) => err.description(),
            DomainMetadataError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DomainMetadataError::ParseError(ref cause) => cause,
            DomainMetadataError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetAttributes
#[derive(Debug, PartialEq)]
pub enum GetAttributesError {
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain the specified missing parameter.</p>
    MissingParameter(String),
    /// <p>The specified domain does not exist.</p>
    NoSuchDomain(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> GetAttributesError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterValue" => {
                        return GetAttributesError::InvalidParameterValue(String::from(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameter" => {
                        return GetAttributesError::MissingParameter(String::from(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDomain" => {
                        return GetAttributesError::NoSuchDomain(String::from(parsed_error.message))
                    }
                    _ => {}
                }
            }
        }
        GetAttributesError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetAttributesError {
    fn from(err: XmlParseError) -> GetAttributesError {
        let XmlParseError(message) = err;
        GetAttributesError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for GetAttributesError {
    fn from(err: CredentialsError) -> GetAttributesError {
        GetAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAttributesError {
    fn from(err: HttpDispatchError) -> GetAttributesError {
        GetAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAttributesError {
    fn from(err: io::Error) -> GetAttributesError {
        GetAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetAttributesError::InvalidParameterValue(ref cause) => cause,
            GetAttributesError::MissingParameter(ref cause) => cause,
            GetAttributesError::NoSuchDomain(ref cause) => cause,
            GetAttributesError::Validation(ref cause) => cause,
            GetAttributesError::Credentials(ref err) => err.description(),
            GetAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAttributesError::ParseError(ref cause) => cause,
            GetAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDomains
#[derive(Debug, PartialEq)]
pub enum ListDomainsError {
    /// <p>The specified NextToken is not valid. </p>
    InvalidNextToken(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListDomainsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListDomainsError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return ListDomainsError::InvalidNextToken(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return ListDomainsError::InvalidParameterValue(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        ListDomainsError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListDomainsError {
    fn from(err: XmlParseError) -> ListDomainsError {
        let XmlParseError(message) = err;
        ListDomainsError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for ListDomainsError {
    fn from(err: CredentialsError) -> ListDomainsError {
        ListDomainsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDomainsError {
    fn from(err: HttpDispatchError) -> ListDomainsError {
        ListDomainsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDomainsError {
    fn from(err: io::Error) -> ListDomainsError {
        ListDomainsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDomainsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDomainsError {
    fn description(&self) -> &str {
        match *self {
            ListDomainsError::InvalidNextToken(ref cause) => cause,
            ListDomainsError::InvalidParameterValue(ref cause) => cause,
            ListDomainsError::Validation(ref cause) => cause,
            ListDomainsError::Credentials(ref err) => err.description(),
            ListDomainsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDomainsError::ParseError(ref cause) => cause,
            ListDomainsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutAttributes
#[derive(Debug, PartialEq)]
pub enum PutAttributesError {
    /// <p>The specified attribute does not exist.</p>
    AttributeDoesNotExist(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain the specified missing parameter.</p>
    MissingParameter(String),
    /// <p>The specified domain does not exist.</p>
    NoSuchDomain(String),
    /// <p>Too many attributes in this domain.</p>
    NumberDomainAttributesExceeded(String),
    /// <p>Too many bytes in this domain.</p>
    NumberDomainBytesExceeded(String),
    /// <p>Too many attributes in this item.</p>
    NumberItemAttributesExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> PutAttributesError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AttributeDoesNotExist" => {
                        return PutAttributesError::AttributeDoesNotExist(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return PutAttributesError::InvalidParameterValue(String::from(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameter" => {
                        return PutAttributesError::MissingParameter(String::from(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDomain" => {
                        return PutAttributesError::NoSuchDomain(String::from(parsed_error.message))
                    }
                    "NumberDomainAttributesExceeded" => {
                        return PutAttributesError::NumberDomainAttributesExceeded(String::from(
                            parsed_error.message,
                        ))
                    }
                    "NumberDomainBytesExceeded" => {
                        return PutAttributesError::NumberDomainBytesExceeded(String::from(
                            parsed_error.message,
                        ))
                    }
                    "NumberItemAttributesExceeded" => {
                        return PutAttributesError::NumberItemAttributesExceeded(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        PutAttributesError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PutAttributesError {
    fn from(err: XmlParseError) -> PutAttributesError {
        let XmlParseError(message) = err;
        PutAttributesError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for PutAttributesError {
    fn from(err: CredentialsError) -> PutAttributesError {
        PutAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutAttributesError {
    fn from(err: HttpDispatchError) -> PutAttributesError {
        PutAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutAttributesError {
    fn from(err: io::Error) -> PutAttributesError {
        PutAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutAttributesError {
    fn description(&self) -> &str {
        match *self {
            PutAttributesError::AttributeDoesNotExist(ref cause) => cause,
            PutAttributesError::InvalidParameterValue(ref cause) => cause,
            PutAttributesError::MissingParameter(ref cause) => cause,
            PutAttributesError::NoSuchDomain(ref cause) => cause,
            PutAttributesError::NumberDomainAttributesExceeded(ref cause) => cause,
            PutAttributesError::NumberDomainBytesExceeded(ref cause) => cause,
            PutAttributesError::NumberItemAttributesExceeded(ref cause) => cause,
            PutAttributesError::Validation(ref cause) => cause,
            PutAttributesError::Credentials(ref err) => err.description(),
            PutAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutAttributesError::ParseError(ref cause) => cause,
            PutAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by Select
#[derive(Debug, PartialEq)]
pub enum SelectError {
    /// <p>The specified NextToken is not valid. </p>
    InvalidNextToken(String),
    /// <p>Too many predicates exist in the query expression.</p>
    InvalidNumberPredicates(String),
    /// <p>Too many predicates exist in the query expression.</p>
    InvalidNumberValueTests(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified query expression syntax is not valid.</p>
    InvalidQueryExpression(String),
    /// <p>The request must contain the specified missing parameter.</p>
    MissingParameter(String),
    /// <p>The specified domain does not exist.</p>
    NoSuchDomain(String),
    /// <p>A timeout occurred when attempting to query the specified domain with specified query expression.</p>
    RequestTimeout(String),
    /// <p>Too many attributes requested.</p>
    TooManyRequestedAttributes(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl SelectError {
    pub fn from_response(res: BufferedHttpResponse) -> SelectError {
        {
            let reader = EventReader::new(res.body.as_slice());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return SelectError::InvalidNextToken(String::from(parsed_error.message))
                    }
                    "InvalidNumberPredicates" => {
                        return SelectError::InvalidNumberPredicates(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidNumberValueTests" => {
                        return SelectError::InvalidNumberValueTests(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return SelectError::InvalidParameterValue(String::from(
                            parsed_error.message,
                        ))
                    }
                    "InvalidQueryExpression" => {
                        return SelectError::InvalidQueryExpression(String::from(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameter" => {
                        return SelectError::MissingParameter(String::from(parsed_error.message))
                    }
                    "NoSuchDomain" => {
                        return SelectError::NoSuchDomain(String::from(parsed_error.message))
                    }
                    "RequestTimeout" => {
                        return SelectError::RequestTimeout(String::from(parsed_error.message))
                    }
                    "TooManyRequestedAttributes" => {
                        return SelectError::TooManyRequestedAttributes(String::from(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        SelectError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SelectError {
    fn from(err: XmlParseError) -> SelectError {
        let XmlParseError(message) = err;
        SelectError::ParseError(message.to_string())
    }
}
impl From<CredentialsError> for SelectError {
    fn from(err: CredentialsError) -> SelectError {
        SelectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SelectError {
    fn from(err: HttpDispatchError) -> SelectError {
        SelectError::HttpDispatch(err)
    }
}
impl From<io::Error> for SelectError {
    fn from(err: io::Error) -> SelectError {
        SelectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SelectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SelectError {
    fn description(&self) -> &str {
        match *self {
            SelectError::InvalidNextToken(ref cause) => cause,
            SelectError::InvalidNumberPredicates(ref cause) => cause,
            SelectError::InvalidNumberValueTests(ref cause) => cause,
            SelectError::InvalidParameterValue(ref cause) => cause,
            SelectError::InvalidQueryExpression(ref cause) => cause,
            SelectError::MissingParameter(ref cause) => cause,
            SelectError::NoSuchDomain(ref cause) => cause,
            SelectError::RequestTimeout(ref cause) => cause,
            SelectError::TooManyRequestedAttributes(ref cause) => cause,
            SelectError::Validation(ref cause) => cause,
            SelectError::Credentials(ref err) => err.description(),
            SelectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SelectError::ParseError(ref cause) => cause,
            SelectError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon SimpleDB API. Amazon SimpleDB clients implement this trait.
pub trait SimpleDb {
    /// <p> Performs multiple DeleteAttributes operations in a single call, which reduces round trips and latencies. This enables Amazon SimpleDB to optimize requests, which generally yields better throughput. </p> <p> The following limitations are enforced for this operation: <ul> <li>1 MB request size</li> <li>25 item limit per BatchDeleteAttributes operation</li> </ul> </p>
    fn batch_delete_attributes(
        &self,
        input: BatchDeleteAttributesRequest,
    ) -> RusotoFuture<(), BatchDeleteAttributesError>;

    /// <p> The <code>BatchPutAttributes</code> operation creates or replaces attributes within one or more items. By using this operation, the client can perform multiple <a>PutAttribute</a> operation with a single call. This helps yield savings in round trips and latencies, enabling Amazon SimpleDB to optimize requests and generally produce better throughput. </p> <p> The client may specify the item name with the <code>Item.X.ItemName</code> parameter. The client may specify new attributes using a combination of the <code>Item.X.Attribute.Y.Name</code> and <code>Item.X.Attribute.Y.Value</code> parameters. The client may specify the first attribute for the first item using the parameters <code>Item.0.Attribute.0.Name</code> and <code>Item.0.Attribute.0.Value</code>, and for the second attribute for the first item by the parameters <code>Item.0.Attribute.1.Name</code> and <code>Item.0.Attribute.1.Value</code>, and so on. </p> <p> Attributes are uniquely identified within an item by their name/value combination. For example, a single item can have the attributes <code>{ "first_name", "first_value" }</code> and <code>{ "first_name", "second_value" }</code>. However, it cannot have two attribute instances where both the <code>Item.X.Attribute.Y.Name</code> and <code>Item.X.Attribute.Y.Value</code> are the same. </p> <p> Optionally, the requester can supply the <code>Replace</code> parameter for each individual value. Setting this value to <code>true</code> will cause the new attribute values to replace the existing attribute values. For example, if an item <code>I</code> has the attributes <code>{ 'a', '1' }, { 'b', '2'}</code> and <code>{ 'b', '3' }</code> and the requester does a BatchPutAttributes of <code>{'I', 'b', '4' }</code> with the Replace parameter set to true, the final attributes of the item will be <code>{ 'a', '1' }</code> and <code>{ 'b', '4' }</code>, replacing the previous values of the 'b' attribute with the new value. </p> <important> This operation is vulnerable to exceeding the maximum URL size when making a REST request using the HTTP GET method. This operation does not support conditions using <code>Expected.X.Name</code>, <code>Expected.X.Value</code>, or <code>Expected.X.Exists</code>. </important> <p> You can execute multiple <code>BatchPutAttributes</code> operations and other operations in parallel. However, large numbers of concurrent <code>BatchPutAttributes</code> calls can result in Service Unavailable (503) responses. </p> <p> The following limitations are enforced for this operation: <ul> <li>256 attribute name-value pairs per item</li> <li>1 MB request size</li> <li>1 billion attributes per domain</li> <li>10 GB of total user data storage per domain</li> <li>25 item limit per <code>BatchPutAttributes</code> operation</li> </ul> </p>
    fn batch_put_attributes(
        &self,
        input: BatchPutAttributesRequest,
    ) -> RusotoFuture<(), BatchPutAttributesError>;

    /// <p> The <code>CreateDomain</code> operation creates a new domain. The domain name should be unique among the domains associated with the Access Key ID provided in the request. The <code>CreateDomain</code> operation may take 10 or more seconds to complete. </p> <p> The client can create up to 100 domains per account. </p> <p> If the client requires additional domains, go to <a href="http://aws.amazon.com/contact-us/simpledb-limit-request/"> http://aws.amazon.com/contact-us/simpledb-limit-request/</a>. </p>
    fn create_domain(&self, input: CreateDomainRequest) -> RusotoFuture<(), CreateDomainError>;

    /// <p> Deletes one or more attributes associated with an item. If all attributes of the item are deleted, the item is deleted. </p> <p> <code>DeleteAttributes</code> is an idempotent operation; running it multiple times on the same item or attribute does not result in an error response. </p> <p> Because Amazon SimpleDB makes multiple copies of item data and uses an eventual consistency update model, performing a <a>GetAttributes</a> or <a>Select</a> operation (read) immediately after a <code>DeleteAttributes</code> or <a>PutAttributes</a> operation (write) might not return updated item data. </p>
    fn delete_attributes(
        &self,
        input: DeleteAttributesRequest,
    ) -> RusotoFuture<(), DeleteAttributesError>;

    /// <p> The <code>DeleteDomain</code> operation deletes a domain. Any items (and their attributes) in the domain are deleted as well. The <code>DeleteDomain</code> operation might take 10 or more seconds to complete. </p>
    fn delete_domain(&self, input: DeleteDomainRequest) -> RusotoFuture<(), DeleteDomainError>;

    /// <p> Returns information about the domain, including when the domain was created, the number of items and attributes in the domain, and the size of the attribute names and values. </p>
    fn domain_metadata(
        &self,
        input: DomainMetadataRequest,
    ) -> RusotoFuture<DomainMetadataResult, DomainMetadataError>;

    /// <p> Returns all of the attributes associated with the specified item. Optionally, the attributes returned can be limited to one or more attributes by specifying an attribute name parameter. </p> <p> If the item does not exist on the replica that was accessed for this operation, an empty set is returned. The system does not return an error as it cannot guarantee the item does not exist on other replicas. </p>
    fn get_attributes(
        &self,
        input: GetAttributesRequest,
    ) -> RusotoFuture<GetAttributesResult, GetAttributesError>;

    /// <p> The <code>ListDomains</code> operation lists all domains associated with the Access Key ID. It returns domain names up to the limit set by <a href="#MaxNumberOfDomains">MaxNumberOfDomains</a>. A <a href="#NextToken">NextToken</a> is returned if there are more than <code>MaxNumberOfDomains</code> domains. Calling <code>ListDomains</code> successive times with the <code>NextToken</code> provided by the operation returns up to <code>MaxNumberOfDomains</code> more domain names with each successive operation call. </p>
    fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> RusotoFuture<ListDomainsResult, ListDomainsError>;

    /// <p> The PutAttributes operation creates or replaces attributes in an item. The client may specify new attributes using a combination of the <code>Attribute.X.Name</code> and <code>Attribute.X.Value</code> parameters. The client specifies the first attribute by the parameters <code>Attribute.0.Name</code> and <code>Attribute.0.Value</code>, the second attribute by the parameters <code>Attribute.1.Name</code> and <code>Attribute.1.Value</code>, and so on. </p> <p> Attributes are uniquely identified in an item by their name/value combination. For example, a single item can have the attributes <code>{ "first_name", "first_value" }</code> and <code>{ "first_name", second_value" }</code>. However, it cannot have two attribute instances where both the <code>Attribute.X.Name</code> and <code>Attribute.X.Value</code> are the same. </p> <p> Optionally, the requestor can supply the <code>Replace</code> parameter for each individual attribute. Setting this value to <code>true</code> causes the new attribute value to replace the existing attribute value(s). For example, if an item has the attributes <code>{ 'a', '1' }</code>, <code>{ 'b', '2'}</code> and <code>{ 'b', '3' }</code> and the requestor calls <code>PutAttributes</code> using the attributes <code>{ 'b', '4' }</code> with the <code>Replace</code> parameter set to true, the final attributes of the item are changed to <code>{ 'a', '1' }</code> and <code>{ 'b', '4' }</code>, which replaces the previous values of the 'b' attribute with the new value. </p> <p> You cannot specify an empty string as an attribute name. </p> <p> Because Amazon SimpleDB makes multiple copies of client data and uses an eventual consistency update model, an immediate <a>GetAttributes</a> or <a>Select</a> operation (read) immediately after a <a>PutAttributes</a> or <a>DeleteAttributes</a> operation (write) might not return the updated data. </p> <p> The following limitations are enforced for this operation: <ul> <li>256 total attribute name-value pairs per item</li> <li>One billion attributes per domain</li> <li>10 GB of total user data storage per domain</li> </ul> </p>
    fn put_attributes(&self, input: PutAttributesRequest) -> RusotoFuture<(), PutAttributesError>;

    /// <p> The <code>Select</code> operation returns a set of attributes for <code>ItemNames</code> that match the select expression. <code>Select</code> is similar to the standard SQL SELECT statement. </p> <p> The total size of the response cannot exceed 1 MB in total size. Amazon SimpleDB automatically adjusts the number of items returned per page to enforce this limit. For example, if the client asks to retrieve 2500 items, but each individual item is 10 kB in size, the system returns 100 items and an appropriate <code>NextToken</code> so the client can access the next page of results. </p> <p> For information on how to construct select expressions, see Using Select to Create Amazon SimpleDB Queries in the Developer Guide. </p>
    fn select(&self, input: SelectRequest) -> RusotoFuture<SelectResult, SelectError>;
}
/// A client for the Amazon SimpleDB API.
pub struct SimpleDbClient {
    client: Client,
    region: region::Region,
}

impl SimpleDbClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SimpleDbClient {
        SimpleDbClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SimpleDbClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        SimpleDbClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl SimpleDb for SimpleDbClient {
    /// <p> Performs multiple DeleteAttributes operations in a single call, which reduces round trips and latencies. This enables Amazon SimpleDB to optimize requests, which generally yields better throughput. </p> <p> The following limitations are enforced for this operation: <ul> <li>1 MB request size</li> <li>25 item limit per BatchDeleteAttributes operation</li> </ul> </p>
    fn batch_delete_attributes(
        &self,
        input: BatchDeleteAttributesRequest,
    ) -> RusotoFuture<(), BatchDeleteAttributesError> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "BatchDeleteAttributes");
        params.put("Version", "2009-04-15");
        BatchDeleteAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchDeleteAttributesError::from_response(response))
                    }),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p> The <code>BatchPutAttributes</code> operation creates or replaces attributes within one or more items. By using this operation, the client can perform multiple <a>PutAttribute</a> operation with a single call. This helps yield savings in round trips and latencies, enabling Amazon SimpleDB to optimize requests and generally produce better throughput. </p> <p> The client may specify the item name with the <code>Item.X.ItemName</code> parameter. The client may specify new attributes using a combination of the <code>Item.X.Attribute.Y.Name</code> and <code>Item.X.Attribute.Y.Value</code> parameters. The client may specify the first attribute for the first item using the parameters <code>Item.0.Attribute.0.Name</code> and <code>Item.0.Attribute.0.Value</code>, and for the second attribute for the first item by the parameters <code>Item.0.Attribute.1.Name</code> and <code>Item.0.Attribute.1.Value</code>, and so on. </p> <p> Attributes are uniquely identified within an item by their name/value combination. For example, a single item can have the attributes <code>{ "first_name", "first_value" }</code> and <code>{ "first_name", "second_value" }</code>. However, it cannot have two attribute instances where both the <code>Item.X.Attribute.Y.Name</code> and <code>Item.X.Attribute.Y.Value</code> are the same. </p> <p> Optionally, the requester can supply the <code>Replace</code> parameter for each individual value. Setting this value to <code>true</code> will cause the new attribute values to replace the existing attribute values. For example, if an item <code>I</code> has the attributes <code>{ 'a', '1' }, { 'b', '2'}</code> and <code>{ 'b', '3' }</code> and the requester does a BatchPutAttributes of <code>{'I', 'b', '4' }</code> with the Replace parameter set to true, the final attributes of the item will be <code>{ 'a', '1' }</code> and <code>{ 'b', '4' }</code>, replacing the previous values of the 'b' attribute with the new value. </p> <important> This operation is vulnerable to exceeding the maximum URL size when making a REST request using the HTTP GET method. This operation does not support conditions using <code>Expected.X.Name</code>, <code>Expected.X.Value</code>, or <code>Expected.X.Exists</code>. </important> <p> You can execute multiple <code>BatchPutAttributes</code> operations and other operations in parallel. However, large numbers of concurrent <code>BatchPutAttributes</code> calls can result in Service Unavailable (503) responses. </p> <p> The following limitations are enforced for this operation: <ul> <li>256 attribute name-value pairs per item</li> <li>1 MB request size</li> <li>1 billion attributes per domain</li> <li>10 GB of total user data storage per domain</li> <li>25 item limit per <code>BatchPutAttributes</code> operation</li> </ul> </p>
    fn batch_put_attributes(
        &self,
        input: BatchPutAttributesRequest,
    ) -> RusotoFuture<(), BatchPutAttributesError> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "BatchPutAttributes");
        params.put("Version", "2009-04-15");
        BatchPutAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchPutAttributesError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p> The <code>CreateDomain</code> operation creates a new domain. The domain name should be unique among the domains associated with the Access Key ID provided in the request. The <code>CreateDomain</code> operation may take 10 or more seconds to complete. </p> <p> The client can create up to 100 domains per account. </p> <p> If the client requires additional domains, go to <a href="http://aws.amazon.com/contact-us/simpledb-limit-request/"> http://aws.amazon.com/contact-us/simpledb-limit-request/</a>. </p>
    fn create_domain(&self, input: CreateDomainRequest) -> RusotoFuture<(), CreateDomainError> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDomain");
        params.put("Version", "2009-04-15");
        CreateDomainRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDomainError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p> Deletes one or more attributes associated with an item. If all attributes of the item are deleted, the item is deleted. </p> <p> <code>DeleteAttributes</code> is an idempotent operation; running it multiple times on the same item or attribute does not result in an error response. </p> <p> Because Amazon SimpleDB makes multiple copies of item data and uses an eventual consistency update model, performing a <a>GetAttributes</a> or <a>Select</a> operation (read) immediately after a <code>DeleteAttributes</code> or <a>PutAttributes</a> operation (write) might not return updated item data. </p>
    fn delete_attributes(
        &self,
        input: DeleteAttributesRequest,
    ) -> RusotoFuture<(), DeleteAttributesError> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteAttributes");
        params.put("Version", "2009-04-15");
        DeleteAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAttributesError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p> The <code>DeleteDomain</code> operation deletes a domain. Any items (and their attributes) in the domain are deleted as well. The <code>DeleteDomain</code> operation might take 10 or more seconds to complete. </p>
    fn delete_domain(&self, input: DeleteDomainRequest) -> RusotoFuture<(), DeleteDomainError> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDomain");
        params.put("Version", "2009-04-15");
        DeleteDomainRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDomainError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p> Returns information about the domain, including when the domain was created, the number of items and attributes in the domain, and the size of the attribute names and values. </p>
    fn domain_metadata(
        &self,
        input: DomainMetadataRequest,
    ) -> RusotoFuture<DomainMetadataResult, DomainMetadataError> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DomainMetadata");
        params.put("Version", "2009-04-15");
        DomainMetadataRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DomainMetadataError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DomainMetadataResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DomainMetadataResultDeserializer::deserialize(
                        "DomainMetadataResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p> Returns all of the attributes associated with the specified item. Optionally, the attributes returned can be limited to one or more attributes by specifying an attribute name parameter. </p> <p> If the item does not exist on the replica that was accessed for this operation, an empty set is returned. The system does not return an error as it cannot guarantee the item does not exist on other replicas. </p>
    fn get_attributes(
        &self,
        input: GetAttributesRequest,
    ) -> RusotoFuture<GetAttributesResult, GetAttributesError> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetAttributes");
        params.put("Version", "2009-04-15");
        GetAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAttributesError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetAttributesResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetAttributesResultDeserializer::deserialize(
                        "GetAttributesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p> The <code>ListDomains</code> operation lists all domains associated with the Access Key ID. It returns domain names up to the limit set by <a href="#MaxNumberOfDomains">MaxNumberOfDomains</a>. A <a href="#NextToken">NextToken</a> is returned if there are more than <code>MaxNumberOfDomains</code> domains. Calling <code>ListDomains</code> successive times with the <code>NextToken</code> provided by the operation returns up to <code>MaxNumberOfDomains</code> more domain names with each successive operation call. </p>
    fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> RusotoFuture<ListDomainsResult, ListDomainsError> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListDomains");
        params.put("Version", "2009-04-15");
        ListDomainsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDomainsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListDomainsResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListDomainsResultDeserializer::deserialize(
                        "ListDomainsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }

    /// <p> The PutAttributes operation creates or replaces attributes in an item. The client may specify new attributes using a combination of the <code>Attribute.X.Name</code> and <code>Attribute.X.Value</code> parameters. The client specifies the first attribute by the parameters <code>Attribute.0.Name</code> and <code>Attribute.0.Value</code>, the second attribute by the parameters <code>Attribute.1.Name</code> and <code>Attribute.1.Value</code>, and so on. </p> <p> Attributes are uniquely identified in an item by their name/value combination. For example, a single item can have the attributes <code>{ "first_name", "first_value" }</code> and <code>{ "first_name", second_value" }</code>. However, it cannot have two attribute instances where both the <code>Attribute.X.Name</code> and <code>Attribute.X.Value</code> are the same. </p> <p> Optionally, the requestor can supply the <code>Replace</code> parameter for each individual attribute. Setting this value to <code>true</code> causes the new attribute value to replace the existing attribute value(s). For example, if an item has the attributes <code>{ 'a', '1' }</code>, <code>{ 'b', '2'}</code> and <code>{ 'b', '3' }</code> and the requestor calls <code>PutAttributes</code> using the attributes <code>{ 'b', '4' }</code> with the <code>Replace</code> parameter set to true, the final attributes of the item are changed to <code>{ 'a', '1' }</code> and <code>{ 'b', '4' }</code>, which replaces the previous values of the 'b' attribute with the new value. </p> <p> You cannot specify an empty string as an attribute name. </p> <p> Because Amazon SimpleDB makes multiple copies of client data and uses an eventual consistency update model, an immediate <a>GetAttributes</a> or <a>Select</a> operation (read) immediately after a <a>PutAttributes</a> or <a>DeleteAttributes</a> operation (write) might not return the updated data. </p> <p> The following limitations are enforced for this operation: <ul> <li>256 total attribute name-value pairs per item</li> <li>One billion attributes per domain</li> <li>10 GB of total user data storage per domain</li> </ul> </p>
    fn put_attributes(&self, input: PutAttributesRequest) -> RusotoFuture<(), PutAttributesError> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutAttributes");
        params.put("Version", "2009-04-15");
        PutAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutAttributesError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p> The <code>Select</code> operation returns a set of attributes for <code>ItemNames</code> that match the select expression. <code>Select</code> is similar to the standard SQL SELECT statement. </p> <p> The total size of the response cannot exceed 1 MB in total size. Amazon SimpleDB automatically adjusts the number of items returned per page to enforce this limit. For example, if the client asks to retrieve 2500 items, but each individual item is 10 kB in size, the system returns 100 items and an appropriate <code>NextToken</code> so the client can access the next page of results. </p> <p> For information on how to construct select expressions, see Using Select to Create Amazon SimpleDB Queries in the Developer Guide. </p>
    fn select(&self, input: SelectRequest) -> RusotoFuture<SelectResult, SelectError> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "Select");
        params.put("Version", "2009-04-15");
        SelectRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(
            serde_urlencoded::to_string(&params).unwrap().into_bytes(),
        ));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SelectError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SelectResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SelectResultDeserializer::deserialize(
                        "SelectResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
