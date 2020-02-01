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

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto::xml::error::*;
use rusoto_core::proto::xml::util::{
    characters, deserialize_elements, end_element, find_start_element, peek_at_name, skip_tree,
    start_element,
};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::signature::SignedRequest;
use serde_urlencoded;
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::EventReader;

/// <p></p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Attribute, XmlParseError> {
        deserialize_elements::<_, Attribute, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AlternateNameEncoding" => {
                    obj.alternate_name_encoding = Some(StringDeserializer::deserialize(
                        "AlternateNameEncoding",
                        stack,
                    )?);
                }
                "AlternateValueEncoding" => {
                    obj.alternate_value_encoding = Some(StringDeserializer::deserialize(
                        "AlternateValueEncoding",
                        stack,
                    )?);
                }
                "Name" => {
                    obj.name = StringDeserializer::deserialize("Name", stack)?;
                }
                "Value" => {
                    obj.value = StringDeserializer::deserialize("Value", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Attribute>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(AttributeDeserializer::deserialize(tag_name, stack)?);
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DomainMetadataResult, XmlParseError> {
        deserialize_elements::<_, DomainMetadataResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AttributeNameCount" => {
                    obj.attribute_name_count = Some(IntegerDeserializer::deserialize(
                        "AttributeNameCount",
                        stack,
                    )?);
                }
                "AttributeNamesSizeBytes" => {
                    obj.attribute_names_size_bytes = Some(LongDeserializer::deserialize(
                        "AttributeNamesSizeBytes",
                        stack,
                    )?);
                }
                "AttributeValueCount" => {
                    obj.attribute_value_count = Some(IntegerDeserializer::deserialize(
                        "AttributeValueCount",
                        stack,
                    )?);
                }
                "AttributeValuesSizeBytes" => {
                    obj.attribute_values_size_bytes = Some(LongDeserializer::deserialize(
                        "AttributeValuesSizeBytes",
                        stack,
                    )?);
                }
                "ItemCount" => {
                    obj.item_count = Some(IntegerDeserializer::deserialize("ItemCount", stack)?);
                }
                "ItemNamesSizeBytes" => {
                    obj.item_names_size_bytes =
                        Some(LongDeserializer::deserialize("ItemNamesSizeBytes", stack)?);
                }
                "Timestamp" => {
                    obj.timestamp = Some(IntegerDeserializer::deserialize("Timestamp", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DomainNameListDeserializer;
impl DomainNameListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(StringDeserializer::deserialize(tag_name, stack)?);
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "ConsistentRead"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
        params.put(&format!("{}{}", prefix, "ItemName"), &obj.item_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GetAttributesResult {
    /// <p>The list of attributes returned by the operation.</p>
    pub attributes: Option<Vec<Attribute>>,
}

struct GetAttributesResultDeserializer;
impl GetAttributesResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetAttributesResult, XmlParseError> {
        deserialize_elements::<_, GetAttributesResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Attribute" => {
                    obj.attributes
                        .get_or_insert(vec![])
                        .extend(AttributeListDeserializer::deserialize("Attribute", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct IntegerDeserializer;
impl IntegerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p></p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Item, XmlParseError> {
        deserialize_elements::<_, Item, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AlternateNameEncoding" => {
                    obj.alternate_name_encoding = Some(StringDeserializer::deserialize(
                        "AlternateNameEncoding",
                        stack,
                    )?);
                }
                "Attribute" => {
                    obj.attributes
                        .extend(AttributeListDeserializer::deserialize("Attribute", stack)?);
                }
                "Name" => {
                    obj.name = StringDeserializer::deserialize("Name", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ItemListDeserializer;
impl ItemListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Item>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(ItemDeserializer::deserialize(tag_name, stack)?);
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "MaxNumberOfDomains"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ListDomainsResult {
    /// <p>A list of domain names that match the expression.</p>
    pub domain_names: Option<Vec<String>>,
    /// <p>An opaque token indicating that there are more domains than the specified <code>MaxNumberOfDomains</code> still available.</p>
    pub next_token: Option<String>,
}

struct ListDomainsResultDeserializer;
impl ListDomainsResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDomainsResult, XmlParseError> {
        deserialize_elements::<_, ListDomainsResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DomainName" => {
                    obj.domain_names.get_or_insert(vec![]).extend(
                        DomainNameListDeserializer::deserialize("DomainName", stack)?,
                    );
                }
                "NextToken" => {
                    obj.next_token = Some(StringDeserializer::deserialize("NextToken", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct LongDeserializer;
impl LongDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "Replace"), &field_value);
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "ConsistentRead"), &field_value);
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
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SelectResult {
    /// <p>A list of items that match the select expression.</p>
    pub items: Option<Vec<Item>>,
    /// <p>An opaque token indicating that more items than <code>MaxNumberOfItems</code> were matched, the response size exceeded 1 megabyte, or the execution time exceeded 5 seconds.</p>
    pub next_token: Option<String>,
}

struct SelectResultDeserializer;
impl SelectResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SelectResult, XmlParseError> {
        deserialize_elements::<_, SelectResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Item" => {
                    obj.items
                        .get_or_insert(vec![])
                        .extend(ItemListDeserializer::deserialize("Item", stack)?);
                }
                "NextToken" => {
                    obj.next_token = Some(StringDeserializer::deserialize("NextToken", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct StringDeserializer;
impl StringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p> Specifies the conditions under which data should be updated. If an update condition is specified for a request, the data will only be updated if the condition is satisfied. For example, if an attribute with a specific name and value exists, or if a specific attribute doesn't exist. </p>
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
            params.put(&format!("{}{}", prefix, "Exists"), &field_value);
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
pub enum BatchDeleteAttributesError {}

impl BatchDeleteAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeleteAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for BatchDeleteAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for BatchDeleteAttributesError {}
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
}

impl BatchPutAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchPutAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DuplicateItemName" => {
                        return RusotoError::Service(BatchPutAttributesError::DuplicateItemName(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            BatchPutAttributesError::InvalidParameterValue(parsed_error.message),
                        )
                    }
                    "MissingParameter" => {
                        return RusotoError::Service(BatchPutAttributesError::MissingParameter(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDomain" => {
                        return RusotoError::Service(BatchPutAttributesError::NoSuchDomain(
                            parsed_error.message,
                        ))
                    }
                    "NumberDomainAttributesExceeded" => {
                        return RusotoError::Service(
                            BatchPutAttributesError::NumberDomainAttributesExceeded(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NumberDomainBytesExceeded" => {
                        return RusotoError::Service(
                            BatchPutAttributesError::NumberDomainBytesExceeded(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NumberItemAttributesExceeded" => {
                        return RusotoError::Service(
                            BatchPutAttributesError::NumberItemAttributesExceeded(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NumberSubmittedAttributesExceeded" => {
                        return RusotoError::Service(
                            BatchPutAttributesError::NumberSubmittedAttributesExceeded(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NumberSubmittedItemsExceeded" => {
                        return RusotoError::Service(
                            BatchPutAttributesError::NumberSubmittedItemsExceeded(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for BatchPutAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchPutAttributesError::DuplicateItemName(ref cause) => write!(f, "{}", cause),
            BatchPutAttributesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            BatchPutAttributesError::MissingParameter(ref cause) => write!(f, "{}", cause),
            BatchPutAttributesError::NoSuchDomain(ref cause) => write!(f, "{}", cause),
            BatchPutAttributesError::NumberDomainAttributesExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchPutAttributesError::NumberDomainBytesExceeded(ref cause) => write!(f, "{}", cause),
            BatchPutAttributesError::NumberItemAttributesExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchPutAttributesError::NumberSubmittedAttributesExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchPutAttributesError::NumberSubmittedItemsExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchPutAttributesError {}
/// Errors returned by CreateDomain
#[derive(Debug, PartialEq)]
pub enum CreateDomainError {
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain the specified missing parameter.</p>
    MissingParameter(String),
    /// <p>Too many domains exist per this account.</p>
    NumberDomainsExceeded(String),
}

impl CreateDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDomainError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterValue" => {
                        return RusotoError::Service(CreateDomainError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameter" => {
                        return RusotoError::Service(CreateDomainError::MissingParameter(
                            parsed_error.message,
                        ))
                    }
                    "NumberDomainsExceeded" => {
                        return RusotoError::Service(CreateDomainError::NumberDomainsExceeded(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDomainError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateDomainError::MissingParameter(ref cause) => write!(f, "{}", cause),
            CreateDomainError::NumberDomainsExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDomainError {}
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
}

impl DeleteAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AttributeDoesNotExist" => {
                        return RusotoError::Service(DeleteAttributesError::AttributeDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(DeleteAttributesError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameter" => {
                        return RusotoError::Service(DeleteAttributesError::MissingParameter(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDomain" => {
                        return RusotoError::Service(DeleteAttributesError::NoSuchDomain(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAttributesError::AttributeDoesNotExist(ref cause) => write!(f, "{}", cause),
            DeleteAttributesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteAttributesError::MissingParameter(ref cause) => write!(f, "{}", cause),
            DeleteAttributesError::NoSuchDomain(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAttributesError {}
/// Errors returned by DeleteDomain
#[derive(Debug, PartialEq)]
pub enum DeleteDomainError {
    /// <p>The request must contain the specified missing parameter.</p>
    MissingParameter(String),
}

impl DeleteDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDomainError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "MissingParameter" => {
                        return RusotoError::Service(DeleteDomainError::MissingParameter(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDomainError::MissingParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDomainError {}
/// Errors returned by DomainMetadata
#[derive(Debug, PartialEq)]
pub enum DomainMetadataError {
    /// <p>The request must contain the specified missing parameter.</p>
    MissingParameter(String),
    /// <p>The specified domain does not exist.</p>
    NoSuchDomain(String),
}

impl DomainMetadataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DomainMetadataError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "MissingParameter" => {
                        return RusotoError::Service(DomainMetadataError::MissingParameter(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDomain" => {
                        return RusotoError::Service(DomainMetadataError::NoSuchDomain(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DomainMetadataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DomainMetadataError::MissingParameter(ref cause) => write!(f, "{}", cause),
            DomainMetadataError::NoSuchDomain(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DomainMetadataError {}
/// Errors returned by GetAttributes
#[derive(Debug, PartialEq)]
pub enum GetAttributesError {
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain the specified missing parameter.</p>
    MissingParameter(String),
    /// <p>The specified domain does not exist.</p>
    NoSuchDomain(String),
}

impl GetAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterValue" => {
                        return RusotoError::Service(GetAttributesError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameter" => {
                        return RusotoError::Service(GetAttributesError::MissingParameter(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDomain" => {
                        return RusotoError::Service(GetAttributesError::NoSuchDomain(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAttributesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetAttributesError::MissingParameter(ref cause) => write!(f, "{}", cause),
            GetAttributesError::NoSuchDomain(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAttributesError {}
/// Errors returned by ListDomains
#[derive(Debug, PartialEq)]
pub enum ListDomainsError {
    /// <p>The specified NextToken is not valid. </p>
    InvalidNextToken(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl ListDomainsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDomainsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(ListDomainsError::InvalidNextToken(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(ListDomainsError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListDomainsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDomainsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListDomainsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDomainsError {}
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
}

impl PutAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutAttributesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AttributeDoesNotExist" => {
                        return RusotoError::Service(PutAttributesError::AttributeDoesNotExist(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(PutAttributesError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameter" => {
                        return RusotoError::Service(PutAttributesError::MissingParameter(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDomain" => {
                        return RusotoError::Service(PutAttributesError::NoSuchDomain(
                            parsed_error.message,
                        ))
                    }
                    "NumberDomainAttributesExceeded" => {
                        return RusotoError::Service(
                            PutAttributesError::NumberDomainAttributesExceeded(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NumberDomainBytesExceeded" => {
                        return RusotoError::Service(PutAttributesError::NumberDomainBytesExceeded(
                            parsed_error.message,
                        ))
                    }
                    "NumberItemAttributesExceeded" => {
                        return RusotoError::Service(
                            PutAttributesError::NumberItemAttributesExceeded(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAttributesError::AttributeDoesNotExist(ref cause) => write!(f, "{}", cause),
            PutAttributesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            PutAttributesError::MissingParameter(ref cause) => write!(f, "{}", cause),
            PutAttributesError::NoSuchDomain(ref cause) => write!(f, "{}", cause),
            PutAttributesError::NumberDomainAttributesExceeded(ref cause) => write!(f, "{}", cause),
            PutAttributesError::NumberDomainBytesExceeded(ref cause) => write!(f, "{}", cause),
            PutAttributesError::NumberItemAttributesExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutAttributesError {}
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
}

impl SelectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SelectError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(SelectError::InvalidNextToken(
                            parsed_error.message,
                        ))
                    }
                    "InvalidNumberPredicates" => {
                        return RusotoError::Service(SelectError::InvalidNumberPredicates(
                            parsed_error.message,
                        ))
                    }
                    "InvalidNumberValueTests" => {
                        return RusotoError::Service(SelectError::InvalidNumberValueTests(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(SelectError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "InvalidQueryExpression" => {
                        return RusotoError::Service(SelectError::InvalidQueryExpression(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameter" => {
                        return RusotoError::Service(SelectError::MissingParameter(
                            parsed_error.message,
                        ))
                    }
                    "NoSuchDomain" => {
                        return RusotoError::Service(SelectError::NoSuchDomain(
                            parsed_error.message,
                        ))
                    }
                    "RequestTimeout" => {
                        return RusotoError::Service(SelectError::RequestTimeout(
                            parsed_error.message,
                        ))
                    }
                    "TooManyRequestedAttributes" => {
                        return RusotoError::Service(SelectError::TooManyRequestedAttributes(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SelectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SelectError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            SelectError::InvalidNumberPredicates(ref cause) => write!(f, "{}", cause),
            SelectError::InvalidNumberValueTests(ref cause) => write!(f, "{}", cause),
            SelectError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            SelectError::InvalidQueryExpression(ref cause) => write!(f, "{}", cause),
            SelectError::MissingParameter(ref cause) => write!(f, "{}", cause),
            SelectError::NoSuchDomain(ref cause) => write!(f, "{}", cause),
            SelectError::RequestTimeout(ref cause) => write!(f, "{}", cause),
            SelectError::TooManyRequestedAttributes(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SelectError {}
/// Trait representing the capabilities of the Amazon SimpleDB API. Amazon SimpleDB clients implement this trait.
#[async_trait]
pub trait SimpleDb {
    /// <p> Performs multiple DeleteAttributes operations in a single call, which reduces round trips and latencies. This enables Amazon SimpleDB to optimize requests, which generally yields better throughput. </p> <p> The following limitations are enforced for this operation: <ul> <li>1 MB request size</li> <li>25 item limit per BatchDeleteAttributes operation</li> </ul> </p>
    async fn batch_delete_attributes(
        &self,
        input: BatchDeleteAttributesRequest,
    ) -> Result<(), RusotoError<BatchDeleteAttributesError>>;

    /// <p> The <code>BatchPutAttributes</code> operation creates or replaces attributes within one or more items. By using this operation, the client can perform multiple <a>PutAttribute</a> operation with a single call. This helps yield savings in round trips and latencies, enabling Amazon SimpleDB to optimize requests and generally produce better throughput. </p> <p> The client may specify the item name with the <code>Item.X.ItemName</code> parameter. The client may specify new attributes using a combination of the <code>Item.X.Attribute.Y.Name</code> and <code>Item.X.Attribute.Y.Value</code> parameters. The client may specify the first attribute for the first item using the parameters <code>Item.0.Attribute.0.Name</code> and <code>Item.0.Attribute.0.Value</code>, and for the second attribute for the first item by the parameters <code>Item.0.Attribute.1.Name</code> and <code>Item.0.Attribute.1.Value</code>, and so on. </p> <p> Attributes are uniquely identified within an item by their name/value combination. For example, a single item can have the attributes <code>{ "first_name", "first_value" }</code> and <code>{ "first_name", "second_value" }</code>. However, it cannot have two attribute instances where both the <code>Item.X.Attribute.Y.Name</code> and <code>Item.X.Attribute.Y.Value</code> are the same. </p> <p> Optionally, the requester can supply the <code>Replace</code> parameter for each individual value. Setting this value to <code>true</code> will cause the new attribute values to replace the existing attribute values. For example, if an item <code>I</code> has the attributes <code>{ 'a', '1' }, { 'b', '2'}</code> and <code>{ 'b', '3' }</code> and the requester does a BatchPutAttributes of <code>{'I', 'b', '4' }</code> with the Replace parameter set to true, the final attributes of the item will be <code>{ 'a', '1' }</code> and <code>{ 'b', '4' }</code>, replacing the previous values of the 'b' attribute with the new value. </p> <important> This operation is vulnerable to exceeding the maximum URL size when making a REST request using the HTTP GET method. This operation does not support conditions using <code>Expected.X.Name</code>, <code>Expected.X.Value</code>, or <code>Expected.X.Exists</code>. </important> <p> You can execute multiple <code>BatchPutAttributes</code> operations and other operations in parallel. However, large numbers of concurrent <code>BatchPutAttributes</code> calls can result in Service Unavailable (503) responses. </p> <p> The following limitations are enforced for this operation: <ul> <li>256 attribute name-value pairs per item</li> <li>1 MB request size</li> <li>1 billion attributes per domain</li> <li>10 GB of total user data storage per domain</li> <li>25 item limit per <code>BatchPutAttributes</code> operation</li> </ul> </p>
    async fn batch_put_attributes(
        &self,
        input: BatchPutAttributesRequest,
    ) -> Result<(), RusotoError<BatchPutAttributesError>>;

    /// <p> The <code>CreateDomain</code> operation creates a new domain. The domain name should be unique among the domains associated with the Access Key ID provided in the request. The <code>CreateDomain</code> operation may take 10 or more seconds to complete. </p> <p> The client can create up to 100 domains per account. </p> <p> If the client requires additional domains, go to <a href="http://aws.amazon.com/contact-us/simpledb-limit-request/"> http://aws.amazon.com/contact-us/simpledb-limit-request/</a>. </p>
    async fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> Result<(), RusotoError<CreateDomainError>>;

    /// <p> Deletes one or more attributes associated with an item. If all attributes of the item are deleted, the item is deleted. </p> <p> <code>DeleteAttributes</code> is an idempotent operation; running it multiple times on the same item or attribute does not result in an error response. </p> <p> Because Amazon SimpleDB makes multiple copies of item data and uses an eventual consistency update model, performing a <a>GetAttributes</a> or <a>Select</a> operation (read) immediately after a <code>DeleteAttributes</code> or <a>PutAttributes</a> operation (write) might not return updated item data. </p>
    async fn delete_attributes(
        &self,
        input: DeleteAttributesRequest,
    ) -> Result<(), RusotoError<DeleteAttributesError>>;

    /// <p> The <code>DeleteDomain</code> operation deletes a domain. Any items (and their attributes) in the domain are deleted as well. The <code>DeleteDomain</code> operation might take 10 or more seconds to complete. </p>
    async fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> Result<(), RusotoError<DeleteDomainError>>;

    /// <p> Returns information about the domain, including when the domain was created, the number of items and attributes in the domain, and the size of the attribute names and values. </p>
    async fn domain_metadata(
        &self,
        input: DomainMetadataRequest,
    ) -> Result<DomainMetadataResult, RusotoError<DomainMetadataError>>;

    /// <p> Returns all of the attributes associated with the specified item. Optionally, the attributes returned can be limited to one or more attributes by specifying an attribute name parameter. </p> <p> If the item does not exist on the replica that was accessed for this operation, an empty set is returned. The system does not return an error as it cannot guarantee the item does not exist on other replicas. </p>
    async fn get_attributes(
        &self,
        input: GetAttributesRequest,
    ) -> Result<GetAttributesResult, RusotoError<GetAttributesError>>;

    /// <p> The <code>ListDomains</code> operation lists all domains associated with the Access Key ID. It returns domain names up to the limit set by <a href="#MaxNumberOfDomains">MaxNumberOfDomains</a>. A <a href="#NextToken">NextToken</a> is returned if there are more than <code>MaxNumberOfDomains</code> domains. Calling <code>ListDomains</code> successive times with the <code>NextToken</code> provided by the operation returns up to <code>MaxNumberOfDomains</code> more domain names with each successive operation call. </p>
    async fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> Result<ListDomainsResult, RusotoError<ListDomainsError>>;

    /// <p> The PutAttributes operation creates or replaces attributes in an item. The client may specify new attributes using a combination of the <code>Attribute.X.Name</code> and <code>Attribute.X.Value</code> parameters. The client specifies the first attribute by the parameters <code>Attribute.0.Name</code> and <code>Attribute.0.Value</code>, the second attribute by the parameters <code>Attribute.1.Name</code> and <code>Attribute.1.Value</code>, and so on. </p> <p> Attributes are uniquely identified in an item by their name/value combination. For example, a single item can have the attributes <code>{ "first_name", "first_value" }</code> and <code>{ "first_name", second_value" }</code>. However, it cannot have two attribute instances where both the <code>Attribute.X.Name</code> and <code>Attribute.X.Value</code> are the same. </p> <p> Optionally, the requestor can supply the <code>Replace</code> parameter for each individual attribute. Setting this value to <code>true</code> causes the new attribute value to replace the existing attribute value(s). For example, if an item has the attributes <code>{ 'a', '1' }</code>, <code>{ 'b', '2'}</code> and <code>{ 'b', '3' }</code> and the requestor calls <code>PutAttributes</code> using the attributes <code>{ 'b', '4' }</code> with the <code>Replace</code> parameter set to true, the final attributes of the item are changed to <code>{ 'a', '1' }</code> and <code>{ 'b', '4' }</code>, which replaces the previous values of the 'b' attribute with the new value. </p> <p> You cannot specify an empty string as an attribute name. </p> <p> Because Amazon SimpleDB makes multiple copies of client data and uses an eventual consistency update model, an immediate <a>GetAttributes</a> or <a>Select</a> operation (read) immediately after a <a>PutAttributes</a> or <a>DeleteAttributes</a> operation (write) might not return the updated data. </p> <p> The following limitations are enforced for this operation: <ul> <li>256 total attribute name-value pairs per item</li> <li>One billion attributes per domain</li> <li>10 GB of total user data storage per domain</li> </ul> </p>
    async fn put_attributes(
        &self,
        input: PutAttributesRequest,
    ) -> Result<(), RusotoError<PutAttributesError>>;

    /// <p> The <code>Select</code> operation returns a set of attributes for <code>ItemNames</code> that match the select expression. <code>Select</code> is similar to the standard SQL SELECT statement. </p> <p> The total size of the response cannot exceed 1 MB in total size. Amazon SimpleDB automatically adjusts the number of items returned per page to enforce this limit. For example, if the client asks to retrieve 2500 items, but each individual item is 10 kB in size, the system returns 100 items and an appropriate <code>NextToken</code> so the client can access the next page of results. </p> <p> For information on how to construct select expressions, see Using Select to Create Amazon SimpleDB Queries in the Developer Guide. </p>
    async fn select(&self, input: SelectRequest) -> Result<SelectResult, RusotoError<SelectError>>;
}
/// A client for the Amazon SimpleDB API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SimpleDbClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SimpleDbClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SimpleDbClient {
        SimpleDbClient { client, region }
    }
}

#[async_trait]
impl SimpleDb for SimpleDbClient {
    /// <p> Performs multiple DeleteAttributes operations in a single call, which reduces round trips and latencies. This enables Amazon SimpleDB to optimize requests, which generally yields better throughput. </p> <p> The following limitations are enforced for this operation: <ul> <li>1 MB request size</li> <li>25 item limit per BatchDeleteAttributes operation</li> </ul> </p>
    async fn batch_delete_attributes(
        &self,
        input: BatchDeleteAttributesRequest,
    ) -> Result<(), RusotoError<BatchDeleteAttributesError>> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "BatchDeleteAttributes");
        params.put("Version", "2009-04-15");
        BatchDeleteAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(BatchDeleteAttributesError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p> The <code>BatchPutAttributes</code> operation creates or replaces attributes within one or more items. By using this operation, the client can perform multiple <a>PutAttribute</a> operation with a single call. This helps yield savings in round trips and latencies, enabling Amazon SimpleDB to optimize requests and generally produce better throughput. </p> <p> The client may specify the item name with the <code>Item.X.ItemName</code> parameter. The client may specify new attributes using a combination of the <code>Item.X.Attribute.Y.Name</code> and <code>Item.X.Attribute.Y.Value</code> parameters. The client may specify the first attribute for the first item using the parameters <code>Item.0.Attribute.0.Name</code> and <code>Item.0.Attribute.0.Value</code>, and for the second attribute for the first item by the parameters <code>Item.0.Attribute.1.Name</code> and <code>Item.0.Attribute.1.Value</code>, and so on. </p> <p> Attributes are uniquely identified within an item by their name/value combination. For example, a single item can have the attributes <code>{ "first_name", "first_value" }</code> and <code>{ "first_name", "second_value" }</code>. However, it cannot have two attribute instances where both the <code>Item.X.Attribute.Y.Name</code> and <code>Item.X.Attribute.Y.Value</code> are the same. </p> <p> Optionally, the requester can supply the <code>Replace</code> parameter for each individual value. Setting this value to <code>true</code> will cause the new attribute values to replace the existing attribute values. For example, if an item <code>I</code> has the attributes <code>{ 'a', '1' }, { 'b', '2'}</code> and <code>{ 'b', '3' }</code> and the requester does a BatchPutAttributes of <code>{'I', 'b', '4' }</code> with the Replace parameter set to true, the final attributes of the item will be <code>{ 'a', '1' }</code> and <code>{ 'b', '4' }</code>, replacing the previous values of the 'b' attribute with the new value. </p> <important> This operation is vulnerable to exceeding the maximum URL size when making a REST request using the HTTP GET method. This operation does not support conditions using <code>Expected.X.Name</code>, <code>Expected.X.Value</code>, or <code>Expected.X.Exists</code>. </important> <p> You can execute multiple <code>BatchPutAttributes</code> operations and other operations in parallel. However, large numbers of concurrent <code>BatchPutAttributes</code> calls can result in Service Unavailable (503) responses. </p> <p> The following limitations are enforced for this operation: <ul> <li>256 attribute name-value pairs per item</li> <li>1 MB request size</li> <li>1 billion attributes per domain</li> <li>10 GB of total user data storage per domain</li> <li>25 item limit per <code>BatchPutAttributes</code> operation</li> </ul> </p>
    async fn batch_put_attributes(
        &self,
        input: BatchPutAttributesRequest,
    ) -> Result<(), RusotoError<BatchPutAttributesError>> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "BatchPutAttributes");
        params.put("Version", "2009-04-15");
        BatchPutAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(BatchPutAttributesError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p> The <code>CreateDomain</code> operation creates a new domain. The domain name should be unique among the domains associated with the Access Key ID provided in the request. The <code>CreateDomain</code> operation may take 10 or more seconds to complete. </p> <p> The client can create up to 100 domains per account. </p> <p> If the client requires additional domains, go to <a href="http://aws.amazon.com/contact-us/simpledb-limit-request/"> http://aws.amazon.com/contact-us/simpledb-limit-request/</a>. </p>
    async fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> Result<(), RusotoError<CreateDomainError>> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateDomain");
        params.put("Version", "2009-04-15");
        CreateDomainRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(CreateDomainError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p> Deletes one or more attributes associated with an item. If all attributes of the item are deleted, the item is deleted. </p> <p> <code>DeleteAttributes</code> is an idempotent operation; running it multiple times on the same item or attribute does not result in an error response. </p> <p> Because Amazon SimpleDB makes multiple copies of item data and uses an eventual consistency update model, performing a <a>GetAttributes</a> or <a>Select</a> operation (read) immediately after a <code>DeleteAttributes</code> or <a>PutAttributes</a> operation (write) might not return updated item data. </p>
    async fn delete_attributes(
        &self,
        input: DeleteAttributesRequest,
    ) -> Result<(), RusotoError<DeleteAttributesError>> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteAttributes");
        params.put("Version", "2009-04-15");
        DeleteAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteAttributesError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p> The <code>DeleteDomain</code> operation deletes a domain. Any items (and their attributes) in the domain are deleted as well. The <code>DeleteDomain</code> operation might take 10 or more seconds to complete. </p>
    async fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> Result<(), RusotoError<DeleteDomainError>> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDomain");
        params.put("Version", "2009-04-15");
        DeleteDomainRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeleteDomainError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p> Returns information about the domain, including when the domain was created, the number of items and attributes in the domain, and the size of the attribute names and values. </p>
    async fn domain_metadata(
        &self,
        input: DomainMetadataRequest,
    ) -> Result<DomainMetadataResult, RusotoError<DomainMetadataError>> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DomainMetadata");
        params.put("Version", "2009-04-15");
        DomainMetadataRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DomainMetadataError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DomainMetadataResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result =
                DomainMetadataResultDeserializer::deserialize("DomainMetadataResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p> Returns all of the attributes associated with the specified item. Optionally, the attributes returned can be limited to one or more attributes by specifying an attribute name parameter. </p> <p> If the item does not exist on the replica that was accessed for this operation, an empty set is returned. The system does not return an error as it cannot guarantee the item does not exist on other replicas. </p>
    async fn get_attributes(
        &self,
        input: GetAttributesRequest,
    ) -> Result<GetAttributesResult, RusotoError<GetAttributesError>> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetAttributes");
        params.put("Version", "2009-04-15");
        GetAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(GetAttributesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = GetAttributesResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result =
                GetAttributesResultDeserializer::deserialize("GetAttributesResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p> The <code>ListDomains</code> operation lists all domains associated with the Access Key ID. It returns domain names up to the limit set by <a href="#MaxNumberOfDomains">MaxNumberOfDomains</a>. A <a href="#NextToken">NextToken</a> is returned if there are more than <code>MaxNumberOfDomains</code> domains. Calling <code>ListDomains</code> successive times with the <code>NextToken</code> provided by the operation returns up to <code>MaxNumberOfDomains</code> more domain names with each successive operation call. </p>
    async fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> Result<ListDomainsResult, RusotoError<ListDomainsError>> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListDomains");
        params.put("Version", "2009-04-15");
        ListDomainsRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ListDomainsError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ListDomainsResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = ListDomainsResultDeserializer::deserialize("ListDomainsResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    /// <p> The PutAttributes operation creates or replaces attributes in an item. The client may specify new attributes using a combination of the <code>Attribute.X.Name</code> and <code>Attribute.X.Value</code> parameters. The client specifies the first attribute by the parameters <code>Attribute.0.Name</code> and <code>Attribute.0.Value</code>, the second attribute by the parameters <code>Attribute.1.Name</code> and <code>Attribute.1.Value</code>, and so on. </p> <p> Attributes are uniquely identified in an item by their name/value combination. For example, a single item can have the attributes <code>{ "first_name", "first_value" }</code> and <code>{ "first_name", second_value" }</code>. However, it cannot have two attribute instances where both the <code>Attribute.X.Name</code> and <code>Attribute.X.Value</code> are the same. </p> <p> Optionally, the requestor can supply the <code>Replace</code> parameter for each individual attribute. Setting this value to <code>true</code> causes the new attribute value to replace the existing attribute value(s). For example, if an item has the attributes <code>{ 'a', '1' }</code>, <code>{ 'b', '2'}</code> and <code>{ 'b', '3' }</code> and the requestor calls <code>PutAttributes</code> using the attributes <code>{ 'b', '4' }</code> with the <code>Replace</code> parameter set to true, the final attributes of the item are changed to <code>{ 'a', '1' }</code> and <code>{ 'b', '4' }</code>, which replaces the previous values of the 'b' attribute with the new value. </p> <p> You cannot specify an empty string as an attribute name. </p> <p> Because Amazon SimpleDB makes multiple copies of client data and uses an eventual consistency update model, an immediate <a>GetAttributes</a> or <a>Select</a> operation (read) immediately after a <a>PutAttributes</a> or <a>DeleteAttributes</a> operation (write) might not return the updated data. </p> <p> The following limitations are enforced for this operation: <ul> <li>256 total attribute name-value pairs per item</li> <li>One billion attributes per domain</li> <li>10 GB of total user data storage per domain</li> </ul> </p>
    async fn put_attributes(
        &self,
        input: PutAttributesRequest,
    ) -> Result<(), RusotoError<PutAttributesError>> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PutAttributes");
        params.put("Version", "2009-04-15");
        PutAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(PutAttributesError::from_response(response));
        }

        std::mem::drop(response);
        Ok(())
    }

    /// <p> The <code>Select</code> operation returns a set of attributes for <code>ItemNames</code> that match the select expression. <code>Select</code> is similar to the standard SQL SELECT statement. </p> <p> The total size of the response cannot exceed 1 MB in total size. Amazon SimpleDB automatically adjusts the number of items returned per page to enforce this limit. For example, if the client asks to retrieve 2500 items, but each individual item is 10 kB in size, the system returns 100 items and an appropriate <code>NextToken</code> so the client can access the next page of results. </p> <p> For information on how to construct select expressions, see Using Select to Create Amazon SimpleDB Queries in the Developer Guide. </p>
    async fn select(&self, input: SelectRequest) -> Result<SelectResult, RusotoError<SelectError>> {
        let mut request = SignedRequest::new("POST", "sdb", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "Select");
        params.put("Version", "2009-04-15");
        SelectRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(SelectError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = SelectResult::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            start_element(&actual_tag_name, &mut stack)?;
            result = SelectResultDeserializer::deserialize("SelectResult", &mut stack)?;
            skip_tree(&mut stack);
            end_element(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }
}
