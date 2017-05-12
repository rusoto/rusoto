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
#[doc="<p></p>"]
#[derive(Default,Debug,Clone)]
            pub struct Attribute {
                #[doc="<p></p>"]
pub alternate_name_encoding: Option<String>,
#[doc="<p></p>"]
pub alternate_value_encoding: Option<String>,
#[doc="The name of the attribute."]
pub name: String,
#[doc="The value of the attribute."]
pub value: String,
            }
            
struct AttributeDeserializer;
            impl AttributeDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Attribute, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = Attribute::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AlternateNameEncoding" => {
                obj.alternate_name_encoding = Some(try!(StringDeserializer::deserialize("AlternateNameEncoding", stack)));
            }
"AlternateValueEncoding" => {
                obj.alternate_value_encoding = Some(try!(StringDeserializer::deserialize("AlternateValueEncoding", stack)));
            }
"Name" => {
                obj.name = try!(StringDeserializer::deserialize("Name", stack));
            }
"Value" => {
                obj.value = try!(StringDeserializer::deserialize("Value", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
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
                params.put(&format!("{}{}", prefix, "AlternateNameEncoding"), &field_value);
            }
if let Some(ref field_value) = obj.alternate_value_encoding {
                params.put(&format!("{}{}", prefix, "AlternateValueEncoding"), &field_value);
            }
params.put(&format!("{}{}", prefix, "Name"), &obj.name);
params.put(&format!("{}{}", prefix, "Value"), &obj.value);
        
                }
            }
            
pub type AttributeList = Vec<Attribute>;
struct AttributeListDeserializer;
            impl AttributeListDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<AttributeList, XmlParseError> {
                    
        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false
            };

            if consume_next_tag {
                obj.push(try!(AttributeDeserializer::deserialize(tag_name, stack)));
            } else {
                break
            }

        }

        Ok(obj)
        
                }
            }

            /// Serialize `AttributeList` contents to a `SignedRequest`.
            struct AttributeListSerializer;
            impl AttributeListSerializer {
                fn serialize(params: &mut Params, name: &str, obj: &AttributeList) {
                    for (index, obj) in obj.iter().enumerate() {
                    let key = format!("{}.member.{}", name, index+1);
AttributeSerializer::serialize(params, &key, obj);
}
                }
            }
            
pub type AttributeNameList = Vec<String>;

            /// Serialize `AttributeNameList` contents to a `SignedRequest`.
            struct AttributeNameListSerializer;
            impl AttributeNameListSerializer {
                fn serialize(params: &mut Params, name: &str, obj: &AttributeNameList) {
                    for (index, obj) in obj.iter().enumerate() {
                    let key = format!("{}.member.{}", name, index+1);
params.put(&key, &obj);
}
                }
            }
            
#[derive(Default,Debug,Clone)]
            pub struct BatchDeleteAttributesRequest {
                #[doc="The name of the domain in which the attributes are being deleted."]
pub domain_name: String,
#[doc="A list of items on which to perform the operation."]
pub items: DeletableItemList,
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
                &format!("{}{}", prefix, "Items"),
                &obj.items,
            );
        
                }
            }
            
#[derive(Default,Debug,Clone)]
            pub struct BatchPutAttributesRequest {
                #[doc="The name of the domain in which the attributes are being stored."]
pub domain_name: String,
#[doc="A list of items on which to perform the operation."]
pub items: ReplaceableItemList,
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
                &format!("{}{}", prefix, "Items"),
                &obj.items,
            );
        
                }
            }
            
pub type Boolean = bool;
#[derive(Default,Debug,Clone)]
            pub struct CreateDomainRequest {
                #[doc="The name of the domain to create. The name can range between 3 and 255 characters and can contain the following characters: a-z, A-Z, 0-9, '_', '-', and '.'."]
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
            
#[derive(Default,Debug,Clone)]
            pub struct DeletableItem {
                pub attributes: Option<AttributeList>,
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
                    &format!("{}{}", prefix, "Attributes"),
                    field_value,
                );
            }
params.put(&format!("{}{}", prefix, "ItemName"), &obj.name);
        
                }
            }
            
pub type DeletableItemList = Vec<DeletableItem>;

            /// Serialize `DeletableItemList` contents to a `SignedRequest`.
            struct DeletableItemListSerializer;
            impl DeletableItemListSerializer {
                fn serialize(params: &mut Params, name: &str, obj: &DeletableItemList) {
                    for (index, obj) in obj.iter().enumerate() {
                    let key = format!("{}.member.{}", name, index+1);
DeletableItemSerializer::serialize(params, &key, obj);
}
                }
            }
            
#[derive(Default,Debug,Clone)]
            pub struct DeleteAttributesRequest {
                #[doc="A list of Attributes. Similar to columns on a spreadsheet, attributes represent categories of data that can be assigned to items."]
pub attributes: Option<AttributeList>,
#[doc="The name of the domain in which to perform the operation."]
pub domain_name: String,
#[doc="The update condition which, if specified, determines whether the specified attributes will be deleted or not. The update condition must be satisfied in order for this request to be processed and the attributes to be deleted."]
pub expected: Option<UpdateCondition>,
#[doc="The name of the item. Similar to rows on a spreadsheet, items represent individual objects that contain one or more value-attribute pairs."]
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
                    &format!("{}{}", prefix, "Attributes"),
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
            
#[derive(Default,Debug,Clone)]
            pub struct DeleteDomainRequest {
                #[doc="The name of the domain to delete."]
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
            
#[derive(Default,Debug,Clone)]
            pub struct DomainMetadataRequest {
                #[doc="The name of the domain for which to display the metadata of."]
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
            
#[derive(Default,Debug,Clone)]
            pub struct DomainMetadataResult {
                #[doc="The number of unique attribute names in the domain."]
pub attribute_name_count: Option<Integer>,
#[doc="The total size of all unique attribute names in the domain, in bytes."]
pub attribute_names_size_bytes: Option<Long>,
#[doc="The number of all attribute name/value pairs in the domain."]
pub attribute_value_count: Option<Integer>,
#[doc="The total size of all attribute values in the domain, in bytes."]
pub attribute_values_size_bytes: Option<Long>,
#[doc="The number of all items in the domain."]
pub item_count: Option<Integer>,
#[doc="The total size of all item names in the domain, in bytes."]
pub item_names_size_bytes: Option<Long>,
#[doc="The data and time when metadata was calculated, in Epoch (UNIX) seconds."]
pub timestamp: Option<Integer>,
            }
            
struct DomainMetadataResultDeserializer;
            impl DomainMetadataResultDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DomainMetadataResult, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = DomainMetadataResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AttributeNameCount" => {
                obj.attribute_name_count = Some(try!(IntegerDeserializer::deserialize("AttributeNameCount", stack)));
            }
"AttributeNamesSizeBytes" => {
                obj.attribute_names_size_bytes = Some(try!(LongDeserializer::deserialize("AttributeNamesSizeBytes", stack)));
            }
"AttributeValueCount" => {
                obj.attribute_value_count = Some(try!(IntegerDeserializer::deserialize("AttributeValueCount", stack)));
            }
"AttributeValuesSizeBytes" => {
                obj.attribute_values_size_bytes = Some(try!(LongDeserializer::deserialize("AttributeValuesSizeBytes", stack)));
            }
"ItemCount" => {
                obj.item_count = Some(try!(IntegerDeserializer::deserialize("ItemCount", stack)));
            }
"ItemNamesSizeBytes" => {
                obj.item_names_size_bytes = Some(try!(LongDeserializer::deserialize("ItemNamesSizeBytes", stack)));
            }
"Timestamp" => {
                obj.timestamp = Some(try!(IntegerDeserializer::deserialize("Timestamp", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type DomainNameList = Vec<String>;
struct DomainNameListDeserializer;
            impl DomainNameListDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DomainNameList, XmlParseError> {
                    
        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false
            };

            if consume_next_tag {
                obj.push(try!(StringDeserializer::deserialize(tag_name, stack)));
            } else {
                break
            }

        }

        Ok(obj)
        
                }
            }
pub type Float = f32;
#[derive(Default,Debug,Clone)]
            pub struct GetAttributesRequest {
                #[doc="The names of the attributes."]
pub attribute_names: Option<AttributeNameList>,
#[doc="Determines whether or not strong consistency should be enforced when data is read from SimpleDB. If <code>true</code>, any data previously written to SimpleDB will be returned. Otherwise, results will be consistent eventually, and the client may not see data that was written immediately before your read."]
pub consistent_read: Option<Boolean>,
#[doc="The name of the domain in which to perform the operation."]
pub domain_name: String,
#[doc="The name of the item."]
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
                    &format!("{}{}", prefix, "AttributeNames"),
                    field_value,
                );
            }
if let Some(ref field_value) = obj.consistent_read {
                params.put(&format!("{}{}", prefix, "ConsistentRead"), &field_value.to_string());
            }
params.put(&format!("{}{}", prefix, "DomainName"), &obj.domain_name);
params.put(&format!("{}{}", prefix, "ItemName"), &obj.item_name);
        
                }
            }
            
#[derive(Default,Debug,Clone)]
            pub struct GetAttributesResult {
                #[doc="The list of attributes returned by the operation."]
pub attributes: Option<AttributeList>,
            }
            
struct GetAttributesResultDeserializer;
            impl GetAttributesResultDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetAttributesResult, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetAttributesResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Attribute" => {
                obj.attributes = Some(try!(AttributeListDeserializer::deserialize("Attribute", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type Integer = i64;
struct IntegerDeserializer;
            impl IntegerDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Integer, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p></p>"]
#[derive(Default,Debug,Clone)]
            pub struct Item {
                #[doc="<p></p>"]
pub alternate_name_encoding: Option<String>,
#[doc="A list of attributes."]
pub attributes: AttributeList,
#[doc="The name of the item."]
pub name: String,
            }
            
struct ItemDeserializer;
            impl ItemDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Item, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = Item::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AlternateNameEncoding" => {
                obj.alternate_name_encoding = Some(try!(StringDeserializer::deserialize("AlternateNameEncoding", stack)));
            }
"Attribute" => {
                obj.attributes = try!(AttributeListDeserializer::deserialize("Attribute", stack));
            }
"Name" => {
                obj.name = try!(StringDeserializer::deserialize("Name", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type ItemList = Vec<Item>;
struct ItemListDeserializer;
            impl ItemListDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ItemList, XmlParseError> {
                    
        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false
            };

            if consume_next_tag {
                obj.push(try!(ItemDeserializer::deserialize(tag_name, stack)));
            } else {
                break
            }

        }

        Ok(obj)
        
                }
            }
#[derive(Default,Debug,Clone)]
            pub struct ListDomainsRequest {
                #[doc="The maximum number of domain names you want returned. The range is 1 to 100. The default setting is 100."]
pub max_number_of_domains: Option<Integer>,
#[doc="A string informing Amazon SimpleDB where to start the next list of domain names."]
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
                params.put(&format!("{}{}", prefix, "MaxNumberOfDomains"), &field_value.to_string());
            }
if let Some(ref field_value) = obj.next_token {
                params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
            }
        
                }
            }
            
#[derive(Default,Debug,Clone)]
            pub struct ListDomainsResult {
                #[doc="A list of domain names that match the expression."]
pub domain_names: Option<DomainNameList>,
#[doc="An opaque token indicating that there are more domains than the specified <code>MaxNumberOfDomains</code> still available."]
pub next_token: Option<String>,
            }
            
struct ListDomainsResultDeserializer;
            impl ListDomainsResultDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListDomainsResult, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListDomainsResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DomainName" => {
                obj.domain_names = Some(try!(DomainNameListDeserializer::deserialize("DomainName", stack)));
            }
"NextToken" => {
                obj.next_token = Some(try!(StringDeserializer::deserialize("NextToken", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type Long = i64;
struct LongDeserializer;
            impl LongDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Long, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[derive(Default,Debug,Clone)]
            pub struct PutAttributesRequest {
                #[doc="The list of attributes."]
pub attributes: ReplaceableAttributeList,
#[doc="The name of the domain in which to perform the operation."]
pub domain_name: String,
#[doc="The update condition which, if specified, determines whether the specified attributes will be updated or not. The update condition must be satisfied in order for this request to be processed and the attributes to be updated."]
pub expected: Option<UpdateCondition>,
#[doc="The name of the item."]
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
                &format!("{}{}", prefix, "Attributes"),
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
            
#[doc="<p></p>"]
#[derive(Default,Debug,Clone)]
            pub struct ReplaceableAttribute {
                #[doc="The name of the replaceable attribute."]
pub name: String,
#[doc="A flag specifying whether or not to replace the attribute/value pair or to add a new attribute/value pair. The default setting is <code>false</code>."]
pub replace: Option<Boolean>,
#[doc="The value of the replaceable attribute."]
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
                params.put(&format!("{}{}", prefix, "Replace"), &field_value.to_string());
            }
params.put(&format!("{}{}", prefix, "Value"), &obj.value);
        
                }
            }
            
pub type ReplaceableAttributeList = Vec<ReplaceableAttribute>;

            /// Serialize `ReplaceableAttributeList` contents to a `SignedRequest`.
            struct ReplaceableAttributeListSerializer;
            impl ReplaceableAttributeListSerializer {
                fn serialize(params: &mut Params, name: &str, obj: &ReplaceableAttributeList) {
                    for (index, obj) in obj.iter().enumerate() {
                    let key = format!("{}.member.{}", name, index+1);
ReplaceableAttributeSerializer::serialize(params, &key, obj);
}
                }
            }
            
#[doc="<p></p>"]
#[derive(Default,Debug,Clone)]
            pub struct ReplaceableItem {
                #[doc="The list of attributes for a replaceable item."]
pub attributes: ReplaceableAttributeList,
#[doc="The name of the replaceable item."]
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
                &format!("{}{}", prefix, "Attributes"),
                &obj.attributes,
            );
params.put(&format!("{}{}", prefix, "ItemName"), &obj.name);
        
                }
            }
            
pub type ReplaceableItemList = Vec<ReplaceableItem>;

            /// Serialize `ReplaceableItemList` contents to a `SignedRequest`.
            struct ReplaceableItemListSerializer;
            impl ReplaceableItemListSerializer {
                fn serialize(params: &mut Params, name: &str, obj: &ReplaceableItemList) {
                    for (index, obj) in obj.iter().enumerate() {
                    let key = format!("{}.member.{}", name, index+1);
ReplaceableItemSerializer::serialize(params, &key, obj);
}
                }
            }
            
#[derive(Default,Debug,Clone)]
            pub struct SelectRequest {
                #[doc="Determines whether or not strong consistency should be enforced when data is read from SimpleDB. If <code>true</code>, any data previously written to SimpleDB will be returned. Otherwise, results will be consistent eventually, and the client may not see data that was written immediately before your read."]
pub consistent_read: Option<Boolean>,
#[doc="A string informing Amazon SimpleDB where to start the next list of <code>ItemNames</code>."]
pub next_token: Option<String>,
#[doc="The expression used to query the domain."]
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
                params.put(&format!("{}{}", prefix, "ConsistentRead"), &field_value.to_string());
            }
if let Some(ref field_value) = obj.next_token {
                params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
            }
params.put(&format!("{}{}", prefix, "SelectExpression"), &obj.select_expression);
        
                }
            }
            
#[derive(Default,Debug,Clone)]
            pub struct SelectResult {
                #[doc="A list of items that match the select expression."]
pub items: Option<ItemList>,
#[doc="An opaque token indicating that more items than <code>MaxNumberOfItems</code> were matched, the response size exceeded 1 megabyte, or the execution time exceeded 5 seconds."]
pub next_token: Option<String>,
            }
            
struct SelectResultDeserializer;
            impl SelectResultDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<SelectResult, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = SelectResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Item" => {
                obj.items = Some(try!(ItemListDeserializer::deserialize("Item", stack)));
            }
"NextToken" => {
                obj.next_token = Some(try!(StringDeserializer::deserialize("NextToken", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
struct StringDeserializer;
            impl StringDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<String, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p> Specifies the conditions under which data should be updated. If an update condition is specified for a request, the data will only be updated if the condition is satisfied. For example, if an attribute with a specific name and value exists, or if a specific attribute doesn't exist. </p>"]
#[derive(Default,Debug,Clone)]
            pub struct UpdateCondition {
                #[doc="<p>A value specifying whether or not the specified attribute must exist with the specified value in order for the update condition to be satisfied. Specify <code>true</code> if the attribute must exist for the update condition to be satisfied. Specify <code>false</code> if the attribute should not exist in order for the update condition to be satisfied.</p>"]
pub exists: Option<Boolean>,
#[doc="<p>The name of the attribute involved in the condition.</p>"]
pub name: Option<String>,
#[doc="<p>The value of an attribute. This value can only be specified when the <code>Exists</code> parameter is equal to <code>true</code>.</p>"]
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
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl BatchDeleteAttributesError {
                    pub fn from_body(body: &str) -> BatchDeleteAttributesError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    _ => BatchDeleteAttributesError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => BatchDeleteAttributesError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for BatchDeleteAttributesError {
                    fn from(err: XmlParseError) -> BatchDeleteAttributesError {
                        let XmlParseError(message) = err;
                        BatchDeleteAttributesError::Unknown(message.to_string())
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
                impl fmt::Display for BatchDeleteAttributesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for BatchDeleteAttributesError {
                    fn description(&self) -> &str {
                        match *self {
                            BatchDeleteAttributesError::Validation(ref cause) => cause,BatchDeleteAttributesError::Credentials(ref err) => err.description(),BatchDeleteAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),BatchDeleteAttributesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by BatchPutAttributes
                #[derive(Debug, PartialEq)]
                pub enum BatchPutAttributesError {
                    
///<p>Too many bytes in this domain.</p>
NumberDomainBytesExceeded(String),
///<p>The item name was specified more than once. </p>
DuplicateItemName(String),
///<p>Too many attributes in this domain.</p>
NumberDomainAttributesExceeded(String),
///<p>Too many attributes in this item.</p>
NumberItemAttributesExceeded(String),
///<p>Too many items exist in a single call.</p>
NumberSubmittedItemsExceeded(String),
///<p>The specified domain does not exist.</p>
NoSuchDomain(String),
///<p>The value for a parameter is invalid.</p>
InvalidParameterValue(String),
///<p>The request must contain the specified missing parameter.</p>
MissingParameter(String),
///<p>Too many attributes exist in a single call.</p>
NumberSubmittedAttributesExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl BatchPutAttributesError {
                    pub fn from_body(body: &str) -> BatchPutAttributesError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "MissingParameter" => BatchPutAttributesError::MissingParameter(String::from(parsed_error.message)),"NumberSubmittedAttributesExceeded" => BatchPutAttributesError::NumberSubmittedAttributesExceeded(String::from(parsed_error.message)),"InvalidParameterValue" => BatchPutAttributesError::InvalidParameterValue(String::from(parsed_error.message)),"NoSuchDomain" => BatchPutAttributesError::NoSuchDomain(String::from(parsed_error.message)),"NumberSubmittedItemsExceeded" => BatchPutAttributesError::NumberSubmittedItemsExceeded(String::from(parsed_error.message)),"NumberDomainBytesExceeded" => BatchPutAttributesError::NumberDomainBytesExceeded(String::from(parsed_error.message)),"NumberDomainAttributesExceeded" => BatchPutAttributesError::NumberDomainAttributesExceeded(String::from(parsed_error.message)),"DuplicateItemName" => BatchPutAttributesError::DuplicateItemName(String::from(parsed_error.message)),"NumberItemAttributesExceeded" => BatchPutAttributesError::NumberItemAttributesExceeded(String::from(parsed_error.message)),_ => BatchPutAttributesError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => BatchPutAttributesError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for BatchPutAttributesError {
                    fn from(err: XmlParseError) -> BatchPutAttributesError {
                        let XmlParseError(message) = err;
                        BatchPutAttributesError::Unknown(message.to_string())
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
                impl fmt::Display for BatchPutAttributesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for BatchPutAttributesError {
                    fn description(&self) -> &str {
                        match *self {
                            BatchPutAttributesError::NoSuchDomain(ref cause) => cause,BatchPutAttributesError::NumberItemAttributesExceeded(ref cause) => cause,BatchPutAttributesError::NumberSubmittedItemsExceeded(ref cause) => cause,BatchPutAttributesError::NumberDomainBytesExceeded(ref cause) => cause,BatchPutAttributesError::DuplicateItemName(ref cause) => cause,BatchPutAttributesError::NumberSubmittedAttributesExceeded(ref cause) => cause,BatchPutAttributesError::MissingParameter(ref cause) => cause,BatchPutAttributesError::NumberDomainAttributesExceeded(ref cause) => cause,BatchPutAttributesError::InvalidParameterValue(ref cause) => cause,BatchPutAttributesError::Validation(ref cause) => cause,BatchPutAttributesError::Credentials(ref err) => err.description(),BatchPutAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),BatchPutAttributesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateDomain
                #[derive(Debug, PartialEq)]
                pub enum CreateDomainError {
                    
///<p>The value for a parameter is invalid.</p>
InvalidParameterValue(String),
///<p>Too many domains exist per this account.</p>
NumberDomainsExceeded(String),
///<p>The request must contain the specified missing parameter.</p>
MissingParameter(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateDomainError {
                    pub fn from_body(body: &str) -> CreateDomainError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "MissingParameter" => CreateDomainError::MissingParameter(String::from(parsed_error.message)),"NumberDomainsExceeded" => CreateDomainError::NumberDomainsExceeded(String::from(parsed_error.message)),"InvalidParameterValue" => CreateDomainError::InvalidParameterValue(String::from(parsed_error.message)),_ => CreateDomainError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => CreateDomainError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for CreateDomainError {
                    fn from(err: XmlParseError) -> CreateDomainError {
                        let XmlParseError(message) = err;
                        CreateDomainError::Unknown(message.to_string())
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
                impl fmt::Display for CreateDomainError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateDomainError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateDomainError::NumberDomainsExceeded(ref cause) => cause,CreateDomainError::InvalidParameterValue(ref cause) => cause,CreateDomainError::MissingParameter(ref cause) => cause,CreateDomainError::Validation(ref cause) => cause,CreateDomainError::Credentials(ref err) => err.description(),CreateDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateDomainError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteAttributes
                #[derive(Debug, PartialEq)]
                pub enum DeleteAttributesError {
                    
///<p>The value for a parameter is invalid.</p>
InvalidParameterValue(String),
///<p>The specified domain does not exist.</p>
NoSuchDomain(String),
///<p>The specified attribute does not exist.</p>
AttributeDoesNotExist(String),
///<p>The request must contain the specified missing parameter.</p>
MissingParameter(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteAttributesError {
                    pub fn from_body(body: &str) -> DeleteAttributesError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidParameterValue" => DeleteAttributesError::InvalidParameterValue(String::from(parsed_error.message)),"NoSuchDomain" => DeleteAttributesError::NoSuchDomain(String::from(parsed_error.message)),"AttributeDoesNotExist" => DeleteAttributesError::AttributeDoesNotExist(String::from(parsed_error.message)),"MissingParameter" => DeleteAttributesError::MissingParameter(String::from(parsed_error.message)),_ => DeleteAttributesError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => DeleteAttributesError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for DeleteAttributesError {
                    fn from(err: XmlParseError) -> DeleteAttributesError {
                        let XmlParseError(message) = err;
                        DeleteAttributesError::Unknown(message.to_string())
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
                impl fmt::Display for DeleteAttributesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteAttributesError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteAttributesError::MissingParameter(ref cause) => cause,DeleteAttributesError::InvalidParameterValue(ref cause) => cause,DeleteAttributesError::AttributeDoesNotExist(ref cause) => cause,DeleteAttributesError::NoSuchDomain(ref cause) => cause,DeleteAttributesError::Validation(ref cause) => cause,DeleteAttributesError::Credentials(ref err) => err.description(),DeleteAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteAttributesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteDomain
                #[derive(Debug, PartialEq)]
                pub enum DeleteDomainError {
                    
///<p>The request must contain the specified missing parameter.</p>
MissingParameter(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteDomainError {
                    pub fn from_body(body: &str) -> DeleteDomainError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "MissingParameter" => DeleteDomainError::MissingParameter(String::from(parsed_error.message)),_ => DeleteDomainError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => DeleteDomainError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for DeleteDomainError {
                    fn from(err: XmlParseError) -> DeleteDomainError {
                        let XmlParseError(message) = err;
                        DeleteDomainError::Unknown(message.to_string())
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
                impl fmt::Display for DeleteDomainError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteDomainError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteDomainError::MissingParameter(ref cause) => cause,DeleteDomainError::Validation(ref cause) => cause,DeleteDomainError::Credentials(ref err) => err.description(),DeleteDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteDomainError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DomainMetadata
                #[derive(Debug, PartialEq)]
                pub enum DomainMetadataError {
                    
///<p>The request must contain the specified missing parameter.</p>
MissingParameter(String),
///<p>The specified domain does not exist.</p>
NoSuchDomain(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DomainMetadataError {
                    pub fn from_body(body: &str) -> DomainMetadataError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "NoSuchDomain" => DomainMetadataError::NoSuchDomain(String::from(parsed_error.message)),"MissingParameter" => DomainMetadataError::MissingParameter(String::from(parsed_error.message)),_ => DomainMetadataError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => DomainMetadataError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for DomainMetadataError {
                    fn from(err: XmlParseError) -> DomainMetadataError {
                        let XmlParseError(message) = err;
                        DomainMetadataError::Unknown(message.to_string())
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
                impl fmt::Display for DomainMetadataError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DomainMetadataError {
                    fn description(&self) -> &str {
                        match *self {
                            DomainMetadataError::NoSuchDomain(ref cause) => cause,DomainMetadataError::MissingParameter(ref cause) => cause,DomainMetadataError::Validation(ref cause) => cause,DomainMetadataError::Credentials(ref err) => err.description(),DomainMetadataError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DomainMetadataError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetAttributes
                #[derive(Debug, PartialEq)]
                pub enum GetAttributesError {
                    
///<p>The value for a parameter is invalid.</p>
InvalidParameterValue(String),
///<p>The specified domain does not exist.</p>
NoSuchDomain(String),
///<p>The request must contain the specified missing parameter.</p>
MissingParameter(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetAttributesError {
                    pub fn from_body(body: &str) -> GetAttributesError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "NoSuchDomain" => GetAttributesError::NoSuchDomain(String::from(parsed_error.message)),"InvalidParameterValue" => GetAttributesError::InvalidParameterValue(String::from(parsed_error.message)),"MissingParameter" => GetAttributesError::MissingParameter(String::from(parsed_error.message)),_ => GetAttributesError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetAttributesError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetAttributesError {
                    fn from(err: XmlParseError) -> GetAttributesError {
                        let XmlParseError(message) = err;
                        GetAttributesError::Unknown(message.to_string())
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
                impl fmt::Display for GetAttributesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetAttributesError {
                    fn description(&self) -> &str {
                        match *self {
                            GetAttributesError::NoSuchDomain(ref cause) => cause,GetAttributesError::MissingParameter(ref cause) => cause,GetAttributesError::InvalidParameterValue(ref cause) => cause,GetAttributesError::Validation(ref cause) => cause,GetAttributesError::Credentials(ref err) => err.description(),GetAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetAttributesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListDomains
                #[derive(Debug, PartialEq)]
                pub enum ListDomainsError {
                    
///<p>The specified NextToken is not valid. </p>
InvalidNextToken(String),
///<p>The value for a parameter is invalid.</p>
InvalidParameterValue(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListDomainsError {
                    pub fn from_body(body: &str) -> ListDomainsError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidParameterValue" => ListDomainsError::InvalidParameterValue(String::from(parsed_error.message)),"InvalidNextToken" => ListDomainsError::InvalidNextToken(String::from(parsed_error.message)),_ => ListDomainsError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListDomainsError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListDomainsError {
                    fn from(err: XmlParseError) -> ListDomainsError {
                        let XmlParseError(message) = err;
                        ListDomainsError::Unknown(message.to_string())
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
                impl fmt::Display for ListDomainsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListDomainsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListDomainsError::InvalidParameterValue(ref cause) => cause,ListDomainsError::InvalidNextToken(ref cause) => cause,ListDomainsError::Validation(ref cause) => cause,ListDomainsError::Credentials(ref err) => err.description(),ListDomainsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListDomainsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by PutAttributes
                #[derive(Debug, PartialEq)]
                pub enum PutAttributesError {
                    
///<p>The specified domain does not exist.</p>
NoSuchDomain(String),
///<p>Too many attributes in this domain.</p>
NumberDomainAttributesExceeded(String),
///<p>Too many attributes in this item.</p>
NumberItemAttributesExceeded(String),
///<p>The request must contain the specified missing parameter.</p>
MissingParameter(String),
///<p>Too many bytes in this domain.</p>
NumberDomainBytesExceeded(String),
///<p>The specified attribute does not exist.</p>
AttributeDoesNotExist(String),
///<p>The value for a parameter is invalid.</p>
InvalidParameterValue(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl PutAttributesError {
                    pub fn from_body(body: &str) -> PutAttributesError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "AttributeDoesNotExist" => PutAttributesError::AttributeDoesNotExist(String::from(parsed_error.message)),"NumberDomainBytesExceeded" => PutAttributesError::NumberDomainBytesExceeded(String::from(parsed_error.message)),"NumberItemAttributesExceeded" => PutAttributesError::NumberItemAttributesExceeded(String::from(parsed_error.message)),"MissingParameter" => PutAttributesError::MissingParameter(String::from(parsed_error.message)),"InvalidParameterValue" => PutAttributesError::InvalidParameterValue(String::from(parsed_error.message)),"NumberDomainAttributesExceeded" => PutAttributesError::NumberDomainAttributesExceeded(String::from(parsed_error.message)),"NoSuchDomain" => PutAttributesError::NoSuchDomain(String::from(parsed_error.message)),_ => PutAttributesError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => PutAttributesError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for PutAttributesError {
                    fn from(err: XmlParseError) -> PutAttributesError {
                        let XmlParseError(message) = err;
                        PutAttributesError::Unknown(message.to_string())
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
                impl fmt::Display for PutAttributesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for PutAttributesError {
                    fn description(&self) -> &str {
                        match *self {
                            PutAttributesError::NumberDomainBytesExceeded(ref cause) => cause,PutAttributesError::MissingParameter(ref cause) => cause,PutAttributesError::NumberItemAttributesExceeded(ref cause) => cause,PutAttributesError::NoSuchDomain(ref cause) => cause,PutAttributesError::AttributeDoesNotExist(ref cause) => cause,PutAttributesError::InvalidParameterValue(ref cause) => cause,PutAttributesError::NumberDomainAttributesExceeded(ref cause) => cause,PutAttributesError::Validation(ref cause) => cause,PutAttributesError::Credentials(ref err) => err.description(),PutAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),PutAttributesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by Select
                #[derive(Debug, PartialEq)]
                pub enum SelectError {
                    
///<p>Too many attributes requested.</p>
TooManyRequestedAttributes(String),
///<p>Too many predicates exist in the query expression.</p>
InvalidNumberValueTests(String),
///<p>The specified query expression syntax is not valid.</p>
InvalidQueryExpression(String),
///<p>The specified NextToken is not valid. </p>
InvalidNextToken(String),
///<p>A timeout occurred when attempting to query the specified domain with specified query expression.</p>
RequestTimeout(String),
///<p>The specified domain does not exist.</p>
NoSuchDomain(String),
///<p>The value for a parameter is invalid.</p>
InvalidParameterValue(String),
///<p>Too many predicates exist in the query expression.</p>
InvalidNumberPredicates(String),
///<p>The request must contain the specified missing parameter.</p>
MissingParameter(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl SelectError {
                    pub fn from_body(body: &str) -> SelectError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidNextToken" => SelectError::InvalidNextToken(String::from(parsed_error.message)),"InvalidNumberValueTests" => SelectError::InvalidNumberValueTests(String::from(parsed_error.message)),"InvalidQueryExpression" => SelectError::InvalidQueryExpression(String::from(parsed_error.message)),"NoSuchDomain" => SelectError::NoSuchDomain(String::from(parsed_error.message)),"InvalidNumberPredicates" => SelectError::InvalidNumberPredicates(String::from(parsed_error.message)),"InvalidParameterValue" => SelectError::InvalidParameterValue(String::from(parsed_error.message)),"TooManyRequestedAttributes" => SelectError::TooManyRequestedAttributes(String::from(parsed_error.message)),"MissingParameter" => SelectError::MissingParameter(String::from(parsed_error.message)),"RequestTimeout" => SelectError::RequestTimeout(String::from(parsed_error.message)),_ => SelectError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => SelectError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for SelectError {
                    fn from(err: XmlParseError) -> SelectError {
                        let XmlParseError(message) = err;
                        SelectError::Unknown(message.to_string())
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
                impl fmt::Display for SelectError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for SelectError {
                    fn description(&self) -> &str {
                        match *self {
                            SelectError::TooManyRequestedAttributes(ref cause) => cause,SelectError::InvalidNumberPredicates(ref cause) => cause,SelectError::MissingParameter(ref cause) => cause,SelectError::InvalidNextToken(ref cause) => cause,SelectError::InvalidNumberValueTests(ref cause) => cause,SelectError::InvalidParameterValue(ref cause) => cause,SelectError::InvalidQueryExpression(ref cause) => cause,SelectError::RequestTimeout(ref cause) => cause,SelectError::NoSuchDomain(ref cause) => cause,SelectError::Validation(ref cause) => cause,SelectError::Credentials(ref err) => err.description(),SelectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),SelectError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Trait representing the capabilities of the Amazon SimpleDB API. Amazon SimpleDB clients implement this trait.
        pub trait SimpleDb {
        

                #[doc="<p> Performs multiple DeleteAttributes operations in a single call, which reduces round trips and latencies. This enables Amazon SimpleDB to optimize requests, which generally yields better throughput. </p> <p> The following limitations are enforced for this operation: <ul> <li>1 MB request size</li> <li>25 item limit per BatchDeleteAttributes operation</li> </ul> </p>"]
                fn batch_delete_attributes(&self, input: &BatchDeleteAttributesRequest) -> Result<(), BatchDeleteAttributesError>;
                

                #[doc="<p> The <code>BatchPutAttributes</code> operation creates or replaces attributes within one or more items. By using this operation, the client can perform multiple <a>PutAttribute</a> operation with a single call. This helps yield savings in round trips and latencies, enabling Amazon SimpleDB to optimize requests and generally produce better throughput. </p> <p> The client may specify the item name with the <code>Item.X.ItemName</code> parameter. The client may specify new attributes using a combination of the <code>Item.X.Attribute.Y.Name</code> and <code>Item.X.Attribute.Y.Value</code> parameters. The client may specify the first attribute for the first item using the parameters <code>Item.0.Attribute.0.Name</code> and <code>Item.0.Attribute.0.Value</code>, and for the second attribute for the first item by the parameters <code>Item.0.Attribute.1.Name</code> and <code>Item.0.Attribute.1.Value</code>, and so on. </p> <p> Attributes are uniquely identified within an item by their name/value combination. For example, a single item can have the attributes <code>{ \"first_name\", \"first_value\" }</code> and <code>{ \"first_name\", \"second_value\" }</code>. However, it cannot have two attribute instances where both the <code>Item.X.Attribute.Y.Name</code> and <code>Item.X.Attribute.Y.Value</code> are the same. </p> <p> Optionally, the requester can supply the <code>Replace</code> parameter for each individual value. Setting this value to <code>true</code> will cause the new attribute values to replace the existing attribute values. For example, if an item <code>I</code> has the attributes <code>{ 'a', '1' }, { 'b', '2'}</code> and <code>{ 'b', '3' }</code> and the requester does a BatchPutAttributes of <code>{'I', 'b', '4' }</code> with the Replace parameter set to true, the final attributes of the item will be <code>{ 'a', '1' }</code> and <code>{ 'b', '4' }</code>, replacing the previous values of the 'b' attribute with the new value. </p> <important> This operation is vulnerable to exceeding the maximum URL size when making a REST request using the HTTP GET method. This operation does not support conditions using <code>Expected.X.Name</code>, <code>Expected.X.Value</code>, or <code>Expected.X.Exists</code>. </important> <p> You can execute multiple <code>BatchPutAttributes</code> operations and other operations in parallel. However, large numbers of concurrent <code>BatchPutAttributes</code> calls can result in Service Unavailable (503) responses. </p> <p> The following limitations are enforced for this operation: <ul> <li>256 attribute name-value pairs per item</li> <li>1 MB request size</li> <li>1 billion attributes per domain</li> <li>10 GB of total user data storage per domain</li> <li>25 item limit per <code>BatchPutAttributes</code> operation</li> </ul> </p>"]
                fn batch_put_attributes(&self, input: &BatchPutAttributesRequest) -> Result<(), BatchPutAttributesError>;
                

                #[doc="<p> The <code>CreateDomain</code> operation creates a new domain. The domain name should be unique among the domains associated with the Access Key ID provided in the request. The <code>CreateDomain</code> operation may take 10 or more seconds to complete. </p> <p> The client can create up to 100 domains per account. </p> <p> If the client requires additional domains, go to <a href=\"http://aws.amazon.com/contact-us/simpledb-limit-request/\"> http://aws.amazon.com/contact-us/simpledb-limit-request/</a>. </p>"]
                fn create_domain(&self, input: &CreateDomainRequest) -> Result<(), CreateDomainError>;
                

                #[doc="<p> Deletes one or more attributes associated with an item. If all attributes of the item are deleted, the item is deleted. </p> <p> <code>DeleteAttributes</code> is an idempotent operation; running it multiple times on the same item or attribute does not result in an error response. </p> <p> Because Amazon SimpleDB makes multiple copies of item data and uses an eventual consistency update model, performing a <a>GetAttributes</a> or <a>Select</a> operation (read) immediately after a <code>DeleteAttributes</code> or <a>PutAttributes</a> operation (write) might not return updated item data. </p>"]
                fn delete_attributes(&self, input: &DeleteAttributesRequest) -> Result<(), DeleteAttributesError>;
                

                #[doc="<p> The <code>DeleteDomain</code> operation deletes a domain. Any items (and their attributes) in the domain are deleted as well. The <code>DeleteDomain</code> operation might take 10 or more seconds to complete. </p>"]
                fn delete_domain(&self, input: &DeleteDomainRequest) -> Result<(), DeleteDomainError>;
                

                #[doc="<p> Returns information about the domain, including when the domain was created, the number of items and attributes in the domain, and the size of the attribute names and values. </p>"]
                fn domain_metadata(&self, input: &DomainMetadataRequest) -> Result<DomainMetadataResult, DomainMetadataError>;
                

                #[doc="<p> Returns all of the attributes associated with the specified item. Optionally, the attributes returned can be limited to one or more attributes by specifying an attribute name parameter. </p> <p> If the item does not exist on the replica that was accessed for this operation, an empty set is returned. The system does not return an error as it cannot guarantee the item does not exist on other replicas. </p>"]
                fn get_attributes(&self, input: &GetAttributesRequest) -> Result<GetAttributesResult, GetAttributesError>;
                

                #[doc="<p> The <code>ListDomains</code> operation lists all domains associated with the Access Key ID. It returns domain names up to the limit set by <a href=\"#MaxNumberOfDomains\">MaxNumberOfDomains</a>. A <a href=\"#NextToken\">NextToken</a> is returned if there are more than <code>MaxNumberOfDomains</code> domains. Calling <code>ListDomains</code> successive times with the <code>NextToken</code> provided by the operation returns up to <code>MaxNumberOfDomains</code> more domain names with each successive operation call. </p>"]
                fn list_domains(&self, input: &ListDomainsRequest) -> Result<ListDomainsResult, ListDomainsError>;
                

                #[doc="<p> The PutAttributes operation creates or replaces attributes in an item. The client may specify new attributes using a combination of the <code>Attribute.X.Name</code> and <code>Attribute.X.Value</code> parameters. The client specifies the first attribute by the parameters <code>Attribute.0.Name</code> and <code>Attribute.0.Value</code>, the second attribute by the parameters <code>Attribute.1.Name</code> and <code>Attribute.1.Value</code>, and so on. </p> <p> Attributes are uniquely identified in an item by their name/value combination. For example, a single item can have the attributes <code>{ \"first_name\", \"first_value\" }</code> and <code>{ \"first_name\", second_value\" }</code>. However, it cannot have two attribute instances where both the <code>Attribute.X.Name</code> and <code>Attribute.X.Value</code> are the same. </p> <p> Optionally, the requestor can supply the <code>Replace</code> parameter for each individual attribute. Setting this value to <code>true</code> causes the new attribute value to replace the existing attribute value(s). For example, if an item has the attributes <code>{ 'a', '1' }</code>, <code>{ 'b', '2'}</code> and <code>{ 'b', '3' }</code> and the requestor calls <code>PutAttributes</code> using the attributes <code>{ 'b', '4' }</code> with the <code>Replace</code> parameter set to true, the final attributes of the item are changed to <code>{ 'a', '1' }</code> and <code>{ 'b', '4' }</code>, which replaces the previous values of the 'b' attribute with the new value. </p> <p> You cannot specify an empty string as an attribute name. </p> <p> Because Amazon SimpleDB makes multiple copies of client data and uses an eventual consistency update model, an immediate <a>GetAttributes</a> or <a>Select</a> operation (read) immediately after a <a>PutAttributes</a> or <a>DeleteAttributes</a> operation (write) might not return the updated data. </p> <p> The following limitations are enforced for this operation: <ul> <li>256 total attribute name-value pairs per item</li> <li>One billion attributes per domain</li> <li>10 GB of total user data storage per domain</li> </ul> </p>"]
                fn put_attributes(&self, input: &PutAttributesRequest) -> Result<(), PutAttributesError>;
                

                #[doc="<p> The <code>Select</code> operation returns a set of attributes for <code>ItemNames</code> that match the select expression. <code>Select</code> is similar to the standard SQL SELECT statement. </p> <p> The total size of the response cannot exceed 1 MB in total size. Amazon SimpleDB automatically adjusts the number of items returned per page to enforce this limit. For example, if the client asks to retrieve 2500 items, but each individual item is 10 kB in size, the system returns 100 items and an appropriate <code>NextToken</code> so the client can access the next page of results. </p> <p> For information on how to construct select expressions, see Using Select to Create Amazon SimpleDB Queries in the Developer Guide. </p>"]
                fn select(&self, input: &SelectRequest) -> Result<SelectResult, SelectError>;
                
}
/// A client for the Amazon SimpleDB API.
        pub struct SimpleDbClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            credentials_provider: P,
            region: region::Region,
            dispatcher: D,
        }

        impl<P, D> SimpleDbClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
                  SimpleDbClient {
                    credentials_provider: credentials_provider,
                    region: region,
                    dispatcher: request_dispatcher
                }
            }
        }

        impl<P, D> SimpleDb for SimpleDbClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
        

                #[doc="<p> Performs multiple DeleteAttributes operations in a single call, which reduces round trips and latencies. This enables Amazon SimpleDB to optimize requests, which generally yields better throughput. </p> <p> The following limitations are enforced for this operation: <ul> <li>1 MB request size</li> <li>25 item limit per BatchDeleteAttributes operation</li> </ul> </p>"]
                fn batch_delete_attributes(&self, input: &BatchDeleteAttributesRequest) -> Result<(), BatchDeleteAttributesError> {
                    let mut request = SignedRequest::new("POST", "sdb", self.region, "/");
                    let mut params = Params::new();

                    params.put("Action", "BatchDeleteAttributes");
                    params.put("Version", "2009-04-15");
                    BatchDeleteAttributesRequestSerializer::serialize(&mut params, "", &input);
                    request.set_params(params);

                    request.sign(&try!(self.credentials_provider.credentials()));
                    let response = try!(self.dispatcher.dispatch(&request));
                    match response.status {
                        StatusCode::Ok => {
                            let result = ();
                            Ok(result)
                        }
                        _ => {
                            Err(BatchDeleteAttributesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
                    }
                }
                

                #[doc="<p> The <code>BatchPutAttributes</code> operation creates or replaces attributes within one or more items. By using this operation, the client can perform multiple <a>PutAttribute</a> operation with a single call. This helps yield savings in round trips and latencies, enabling Amazon SimpleDB to optimize requests and generally produce better throughput. </p> <p> The client may specify the item name with the <code>Item.X.ItemName</code> parameter. The client may specify new attributes using a combination of the <code>Item.X.Attribute.Y.Name</code> and <code>Item.X.Attribute.Y.Value</code> parameters. The client may specify the first attribute for the first item using the parameters <code>Item.0.Attribute.0.Name</code> and <code>Item.0.Attribute.0.Value</code>, and for the second attribute for the first item by the parameters <code>Item.0.Attribute.1.Name</code> and <code>Item.0.Attribute.1.Value</code>, and so on. </p> <p> Attributes are uniquely identified within an item by their name/value combination. For example, a single item can have the attributes <code>{ \"first_name\", \"first_value\" }</code> and <code>{ \"first_name\", \"second_value\" }</code>. However, it cannot have two attribute instances where both the <code>Item.X.Attribute.Y.Name</code> and <code>Item.X.Attribute.Y.Value</code> are the same. </p> <p> Optionally, the requester can supply the <code>Replace</code> parameter for each individual value. Setting this value to <code>true</code> will cause the new attribute values to replace the existing attribute values. For example, if an item <code>I</code> has the attributes <code>{ 'a', '1' }, { 'b', '2'}</code> and <code>{ 'b', '3' }</code> and the requester does a BatchPutAttributes of <code>{'I', 'b', '4' }</code> with the Replace parameter set to true, the final attributes of the item will be <code>{ 'a', '1' }</code> and <code>{ 'b', '4' }</code>, replacing the previous values of the 'b' attribute with the new value. </p> <important> This operation is vulnerable to exceeding the maximum URL size when making a REST request using the HTTP GET method. This operation does not support conditions using <code>Expected.X.Name</code>, <code>Expected.X.Value</code>, or <code>Expected.X.Exists</code>. </important> <p> You can execute multiple <code>BatchPutAttributes</code> operations and other operations in parallel. However, large numbers of concurrent <code>BatchPutAttributes</code> calls can result in Service Unavailable (503) responses. </p> <p> The following limitations are enforced for this operation: <ul> <li>256 attribute name-value pairs per item</li> <li>1 MB request size</li> <li>1 billion attributes per domain</li> <li>10 GB of total user data storage per domain</li> <li>25 item limit per <code>BatchPutAttributes</code> operation</li> </ul> </p>"]
                fn batch_put_attributes(&self, input: &BatchPutAttributesRequest) -> Result<(), BatchPutAttributesError> {
                    let mut request = SignedRequest::new("POST", "sdb", self.region, "/");
                    let mut params = Params::new();

                    params.put("Action", "BatchPutAttributes");
                    params.put("Version", "2009-04-15");
                    BatchPutAttributesRequestSerializer::serialize(&mut params, "", &input);
                    request.set_params(params);

                    request.sign(&try!(self.credentials_provider.credentials()));
                    let response = try!(self.dispatcher.dispatch(&request));
                    match response.status {
                        StatusCode::Ok => {
                            let result = ();
                            Ok(result)
                        }
                        _ => {
                            Err(BatchPutAttributesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
                    }
                }
                

                #[doc="<p> The <code>CreateDomain</code> operation creates a new domain. The domain name should be unique among the domains associated with the Access Key ID provided in the request. The <code>CreateDomain</code> operation may take 10 or more seconds to complete. </p> <p> The client can create up to 100 domains per account. </p> <p> If the client requires additional domains, go to <a href=\"http://aws.amazon.com/contact-us/simpledb-limit-request/\"> http://aws.amazon.com/contact-us/simpledb-limit-request/</a>. </p>"]
                fn create_domain(&self, input: &CreateDomainRequest) -> Result<(), CreateDomainError> {
                    let mut request = SignedRequest::new("POST", "sdb", self.region, "/");
                    let mut params = Params::new();

                    params.put("Action", "CreateDomain");
                    params.put("Version", "2009-04-15");
                    CreateDomainRequestSerializer::serialize(&mut params, "", &input);
                    request.set_params(params);

                    request.sign(&try!(self.credentials_provider.credentials()));
                    let response = try!(self.dispatcher.dispatch(&request));
                    match response.status {
                        StatusCode::Ok => {
                            let result = ();
                            Ok(result)
                        }
                        _ => {
                            Err(CreateDomainError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
                    }
                }
                

                #[doc="<p> Deletes one or more attributes associated with an item. If all attributes of the item are deleted, the item is deleted. </p> <p> <code>DeleteAttributes</code> is an idempotent operation; running it multiple times on the same item or attribute does not result in an error response. </p> <p> Because Amazon SimpleDB makes multiple copies of item data and uses an eventual consistency update model, performing a <a>GetAttributes</a> or <a>Select</a> operation (read) immediately after a <code>DeleteAttributes</code> or <a>PutAttributes</a> operation (write) might not return updated item data. </p>"]
                fn delete_attributes(&self, input: &DeleteAttributesRequest) -> Result<(), DeleteAttributesError> {
                    let mut request = SignedRequest::new("POST", "sdb", self.region, "/");
                    let mut params = Params::new();

                    params.put("Action", "DeleteAttributes");
                    params.put("Version", "2009-04-15");
                    DeleteAttributesRequestSerializer::serialize(&mut params, "", &input);
                    request.set_params(params);

                    request.sign(&try!(self.credentials_provider.credentials()));
                    let response = try!(self.dispatcher.dispatch(&request));
                    match response.status {
                        StatusCode::Ok => {
                            let result = ();
                            Ok(result)
                        }
                        _ => {
                            Err(DeleteAttributesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
                    }
                }
                

                #[doc="<p> The <code>DeleteDomain</code> operation deletes a domain. Any items (and their attributes) in the domain are deleted as well. The <code>DeleteDomain</code> operation might take 10 or more seconds to complete. </p>"]
                fn delete_domain(&self, input: &DeleteDomainRequest) -> Result<(), DeleteDomainError> {
                    let mut request = SignedRequest::new("POST", "sdb", self.region, "/");
                    let mut params = Params::new();

                    params.put("Action", "DeleteDomain");
                    params.put("Version", "2009-04-15");
                    DeleteDomainRequestSerializer::serialize(&mut params, "", &input);
                    request.set_params(params);

                    request.sign(&try!(self.credentials_provider.credentials()));
                    let response = try!(self.dispatcher.dispatch(&request));
                    match response.status {
                        StatusCode::Ok => {
                            let result = ();
                            Ok(result)
                        }
                        _ => {
                            Err(DeleteDomainError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
                    }
                }
                

                #[doc="<p> Returns information about the domain, including when the domain was created, the number of items and attributes in the domain, and the size of the attribute names and values. </p>"]
                fn domain_metadata(&self, input: &DomainMetadataRequest) -> Result<DomainMetadataResult, DomainMetadataError> {
                    let mut request = SignedRequest::new("POST", "sdb", self.region, "/");
                    let mut params = Params::new();

                    params.put("Action", "DomainMetadata");
                    params.put("Version", "2009-04-15");
                    DomainMetadataRequestSerializer::serialize(&mut params, "", &input);
                    request.set_params(params);

                    request.sign(&try!(self.credentials_provider.credentials()));
                    let response = try!(self.dispatcher.dispatch(&request));
                    match response.status {
                        StatusCode::Ok => {
                            
        let result;

        if response.body.is_empty() {
            result = DomainMetadataResult::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            try!(start_element(&actual_tag_name, &mut stack));
                     result = try!(DomainMetadataResultDeserializer::deserialize("DomainMetadataResult", &mut stack));
                     skip_tree(&mut stack);
                     try!(end_element(&actual_tag_name, &mut stack));
        }
                            Ok(result)
                        }
                        _ => {
                            Err(DomainMetadataError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
                    }
                }
                

                #[doc="<p> Returns all of the attributes associated with the specified item. Optionally, the attributes returned can be limited to one or more attributes by specifying an attribute name parameter. </p> <p> If the item does not exist on the replica that was accessed for this operation, an empty set is returned. The system does not return an error as it cannot guarantee the item does not exist on other replicas. </p>"]
                fn get_attributes(&self, input: &GetAttributesRequest) -> Result<GetAttributesResult, GetAttributesError> {
                    let mut request = SignedRequest::new("POST", "sdb", self.region, "/");
                    let mut params = Params::new();

                    params.put("Action", "GetAttributes");
                    params.put("Version", "2009-04-15");
                    GetAttributesRequestSerializer::serialize(&mut params, "", &input);
                    request.set_params(params);

                    request.sign(&try!(self.credentials_provider.credentials()));
                    let response = try!(self.dispatcher.dispatch(&request));
                    match response.status {
                        StatusCode::Ok => {
                            
        let result;

        if response.body.is_empty() {
            result = GetAttributesResult::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            try!(start_element(&actual_tag_name, &mut stack));
                     result = try!(GetAttributesResultDeserializer::deserialize("GetAttributesResult", &mut stack));
                     skip_tree(&mut stack);
                     try!(end_element(&actual_tag_name, &mut stack));
        }
                            Ok(result)
                        }
                        _ => {
                            Err(GetAttributesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
                    }
                }
                

                #[doc="<p> The <code>ListDomains</code> operation lists all domains associated with the Access Key ID. It returns domain names up to the limit set by <a href=\"#MaxNumberOfDomains\">MaxNumberOfDomains</a>. A <a href=\"#NextToken\">NextToken</a> is returned if there are more than <code>MaxNumberOfDomains</code> domains. Calling <code>ListDomains</code> successive times with the <code>NextToken</code> provided by the operation returns up to <code>MaxNumberOfDomains</code> more domain names with each successive operation call. </p>"]
                fn list_domains(&self, input: &ListDomainsRequest) -> Result<ListDomainsResult, ListDomainsError> {
                    let mut request = SignedRequest::new("POST", "sdb", self.region, "/");
                    let mut params = Params::new();

                    params.put("Action", "ListDomains");
                    params.put("Version", "2009-04-15");
                    ListDomainsRequestSerializer::serialize(&mut params, "", &input);
                    request.set_params(params);

                    request.sign(&try!(self.credentials_provider.credentials()));
                    let response = try!(self.dispatcher.dispatch(&request));
                    match response.status {
                        StatusCode::Ok => {
                            
        let result;

        if response.body.is_empty() {
            result = ListDomainsResult::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            try!(start_element(&actual_tag_name, &mut stack));
                     result = try!(ListDomainsResultDeserializer::deserialize("ListDomainsResult", &mut stack));
                     skip_tree(&mut stack);
                     try!(end_element(&actual_tag_name, &mut stack));
        }
                            Ok(result)
                        }
                        _ => {
                            Err(ListDomainsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
                    }
                }
                

                #[doc="<p> The PutAttributes operation creates or replaces attributes in an item. The client may specify new attributes using a combination of the <code>Attribute.X.Name</code> and <code>Attribute.X.Value</code> parameters. The client specifies the first attribute by the parameters <code>Attribute.0.Name</code> and <code>Attribute.0.Value</code>, the second attribute by the parameters <code>Attribute.1.Name</code> and <code>Attribute.1.Value</code>, and so on. </p> <p> Attributes are uniquely identified in an item by their name/value combination. For example, a single item can have the attributes <code>{ \"first_name\", \"first_value\" }</code> and <code>{ \"first_name\", second_value\" }</code>. However, it cannot have two attribute instances where both the <code>Attribute.X.Name</code> and <code>Attribute.X.Value</code> are the same. </p> <p> Optionally, the requestor can supply the <code>Replace</code> parameter for each individual attribute. Setting this value to <code>true</code> causes the new attribute value to replace the existing attribute value(s). For example, if an item has the attributes <code>{ 'a', '1' }</code>, <code>{ 'b', '2'}</code> and <code>{ 'b', '3' }</code> and the requestor calls <code>PutAttributes</code> using the attributes <code>{ 'b', '4' }</code> with the <code>Replace</code> parameter set to true, the final attributes of the item are changed to <code>{ 'a', '1' }</code> and <code>{ 'b', '4' }</code>, which replaces the previous values of the 'b' attribute with the new value. </p> <p> You cannot specify an empty string as an attribute name. </p> <p> Because Amazon SimpleDB makes multiple copies of client data and uses an eventual consistency update model, an immediate <a>GetAttributes</a> or <a>Select</a> operation (read) immediately after a <a>PutAttributes</a> or <a>DeleteAttributes</a> operation (write) might not return the updated data. </p> <p> The following limitations are enforced for this operation: <ul> <li>256 total attribute name-value pairs per item</li> <li>One billion attributes per domain</li> <li>10 GB of total user data storage per domain</li> </ul> </p>"]
                fn put_attributes(&self, input: &PutAttributesRequest) -> Result<(), PutAttributesError> {
                    let mut request = SignedRequest::new("POST", "sdb", self.region, "/");
                    let mut params = Params::new();

                    params.put("Action", "PutAttributes");
                    params.put("Version", "2009-04-15");
                    PutAttributesRequestSerializer::serialize(&mut params, "", &input);
                    request.set_params(params);

                    request.sign(&try!(self.credentials_provider.credentials()));
                    let response = try!(self.dispatcher.dispatch(&request));
                    match response.status {
                        StatusCode::Ok => {
                            let result = ();
                            Ok(result)
                        }
                        _ => {
                            Err(PutAttributesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
                    }
                }
                

                #[doc="<p> The <code>Select</code> operation returns a set of attributes for <code>ItemNames</code> that match the select expression. <code>Select</code> is similar to the standard SQL SELECT statement. </p> <p> The total size of the response cannot exceed 1 MB in total size. Amazon SimpleDB automatically adjusts the number of items returned per page to enforce this limit. For example, if the client asks to retrieve 2500 items, but each individual item is 10 kB in size, the system returns 100 items and an appropriate <code>NextToken</code> so the client can access the next page of results. </p> <p> For information on how to construct select expressions, see Using Select to Create Amazon SimpleDB Queries in the Developer Guide. </p>"]
                fn select(&self, input: &SelectRequest) -> Result<SelectResult, SelectError> {
                    let mut request = SignedRequest::new("POST", "sdb", self.region, "/");
                    let mut params = Params::new();

                    params.put("Action", "Select");
                    params.put("Version", "2009-04-15");
                    SelectRequestSerializer::serialize(&mut params, "", &input);
                    request.set_params(params);

                    request.sign(&try!(self.credentials_provider.credentials()));
                    let response = try!(self.dispatcher.dispatch(&request));
                    match response.status {
                        StatusCode::Ok => {
                            
        let result;

        if response.body.is_empty() {
            result = SelectResult::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            try!(start_element(&actual_tag_name, &mut stack));
                     result = try!(SelectResultDeserializer::deserialize("SelectResult", &mut stack));
                     skip_tree(&mut stack);
                     try!(end_element(&actual_tag_name, &mut stack));
        }
                            Ok(result)
                        }
                        _ => {
                            Err(SelectError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
                    }
                }
                
}

            #[cfg(test)]
            mod protocol_tests {
                
            }
            
