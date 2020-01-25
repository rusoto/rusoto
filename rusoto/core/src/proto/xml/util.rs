//! Tools for handling XML from AWS with helper functions for testing.
//!
//! Wraps an XML stack via traits.
//! Also provides a method of supplying an XML stack from a file for testing purposes.

use std::collections::HashMap;
use std::iter::Peekable;
use std::num::ParseIntError;
use xml;
use xml::reader::{Events, XmlEvent};

/// generic Error for XML parsing
#[derive(Debug)]
pub struct XmlParseError(pub String);

impl XmlParseError {
    pub fn new(msg: &str) -> XmlParseError {
        XmlParseError(msg.to_string())
    }
}

/// syntactic sugar for the XML event stack we pass around
pub type XmlStack<'a> = Peekable<Events<&'a [u8]>>;

/// Peek at next items in the XML stack
pub trait Peek {
    fn peek(&mut self) -> Option<&Result<XmlEvent, xml::reader::Error>>;
}

/// Move to the next part of the XML stack
pub trait Next {
    fn next(&mut self) -> Option<Result<XmlEvent, xml::reader::Error>>;
}

/// Wraps the Hyper Response type
pub struct XmlResponse<'b> {
    xml_stack: Peekable<Events<&'b [u8]>>, // refactor to use XmlStack type?
}

impl<'b> XmlResponse<'b> {
    pub fn new(stack: Peekable<Events<&'b [u8]>>) -> XmlResponse {
        XmlResponse { xml_stack: stack }
    }
}

impl<'b> Peek for XmlResponse<'b> {
    fn peek(&mut self) -> Option<&Result<XmlEvent, xml::reader::Error>> {
        while let Some(&Ok(XmlEvent::Whitespace(_))) = self.xml_stack.peek() {
            self.xml_stack.next();
        }
        self.xml_stack.peek()
    }
}

impl<'b> Next for XmlResponse<'b> {
    fn next(&mut self) -> Option<Result<XmlEvent, xml::reader::Error>> {
        let mut maybe_event;
        loop {
            maybe_event = self.xml_stack.next();
            match maybe_event {
                Some(Ok(XmlEvent::Whitespace(_))) => {}
                _ => break,
            }
        }
        maybe_event
    }
}

impl From<ParseIntError> for XmlParseError {
    fn from(_e: ParseIntError) -> XmlParseError {
        XmlParseError::new("ParseIntError")
    }
}

/// return a string field with the right name or throw a parse error
pub fn string_field<T: Peek + Next>(name: &str, stack: &mut T) -> Result<String, XmlParseError> {
    start_element(name, stack)?;
    let value = characters(stack)?;
    end_element(name, stack)?;
    Ok(value)
}

/// return some XML Characters
pub fn characters<T: Peek + Next>(stack: &mut T) -> Result<String, XmlParseError> {
    {
        // Lexical lifetime
        // Check to see if the next element is an end tag.
        // If it is, return an empty string.
        let current = stack.peek();
        if let Some(&Ok(XmlEvent::EndElement { .. })) = current {
            return Ok("".to_string());
        }
    }
    if let Some(Ok(XmlEvent::Characters(data))) = stack.next() {
        Ok(data)
    } else {
        Err(XmlParseError::new("Expected characters"))
    }
}

/// get the name of the current element in the stack.  throw a parse error if it's not a `StartElement`
pub fn peek_at_name<T: Peek + Next>(stack: &mut T) -> Result<String, XmlParseError> {
    let current = stack.peek();
    if let Some(&Ok(XmlEvent::StartElement { ref name, .. })) = current {
        Ok(name.local_name.to_string())
    } else {
        Ok("".to_string())
    }
}

/// consume a `StartElement` with a specific name or throw an `XmlParseError`
pub fn start_element<T: Peek + Next>(
    element_name: &str,
    stack: &mut T,
) -> Result<HashMap<String, String>, XmlParseError> {
    let next = stack.next();

    if let Some(Ok(XmlEvent::StartElement {
        name, attributes, ..
    })) = next
    {
        if name.local_name == element_name {
            let mut attr_map = HashMap::new();
            for attr in attributes {
                attr_map.insert(attr.name.local_name, attr.value);
            }
            Ok(attr_map)
        } else {
            Err(XmlParseError::new(&format!(
                "START Expected {} got {}",
                element_name, name.local_name
            )))
        }
    } else {
        Err(XmlParseError::new(&format!(
            "Expected StartElement {} got {:#?}",
            element_name, next
        )))
    }
}

/// consume an `EndElement` with a specific name or throw an `XmlParseError`
pub fn end_element<T: Peek + Next>(element_name: &str, stack: &mut T) -> Result<(), XmlParseError> {
    let next = stack.next();
    if let Some(Ok(XmlEvent::EndElement { name, .. })) = next {
        if name.local_name == element_name {
            Ok(())
        } else {
            Err(XmlParseError::new(&format!(
                "END Expected {} got {}",
                element_name, name.local_name
            )))
        }
    } else {
        Err(XmlParseError::new(&format!(
            "Expected EndElement {} got {:?}",
            element_name, next
        )))
    }
}

/// skip a tag and all its children
pub fn skip_tree<T: Peek + Next>(stack: &mut T) {
    let mut deep: usize = 0;

    loop {
        match stack.next() {
            None => break,
            Some(Ok(XmlEvent::StartElement { .. })) => deep += 1,
            Some(Ok(XmlEvent::EndElement { .. })) => {
                if deep > 1 {
                    deep -= 1;
                } else {
                    break;
                }
            }
            _ => (),
        }
    }
}

/// skip all elements until a start element is encountered
///
/// Errors and end-of-stream are ignored.
pub fn find_start_element<T: Peek + Next>(stack: &mut T) {
    loop {
        match stack.peek() {
            Some(&Ok(XmlEvent::StartElement { .. })) => break,
            Some(&Ok(_)) => {
                stack.next().unwrap().unwrap();
            }
            Some(&Err(_)) => break,
            None => break,
        }
    }
}

enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}

pub fn deserialize_elements<T, S, F>(
    tag_name: &str,
    stack: &mut T,
    mut handle_element: F,
) -> Result<S, XmlParseError>
where
    T: Peek + Next,
    S: Default,
    F: FnMut(&str, &mut T, &mut S) -> Result<(), XmlParseError>,
{
    let mut obj = S::default();

    start_element(tag_name, stack)?;

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
                handle_element(&name[..], stack, &mut obj)?;
            }
            DeserializerNext::Close => break,
            DeserializerNext::Skip => {
                stack.next();
            }
        }
    }

    end_element(tag_name, stack)?;

    Ok(obj)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use xml::reader::EventReader;

    #[test]
    fn peek_at_name_happy_path() {
        let mut file = File::open("test_resources/list_queues_with_queue.xml").unwrap();
        let mut body = String::new();
        let _size = file.read_to_string(&mut body);
        let my_parser = EventReader::new(body.as_bytes());
        let my_stack = my_parser.into_iter().peekable();
        let mut reader = XmlResponse::new(my_stack);

        loop {
            reader.next();
            match peek_at_name(&mut reader) {
                Ok(data) => {
                    if data == "QueueUrl" {
                        return;
                    }
                }
                Err(_) => panic!("Couldn't peek at name"),
            }
        }
    }

    #[test]
    fn start_element_happy_path() {
        let mut file = File::open("test_resources/list_queues_with_queue.xml").unwrap();
        let mut body = String::new();
        let _size = file.read_to_string(&mut body);
        let my_parser = EventReader::new(body.as_bytes());
        let my_stack = my_parser.into_iter().peekable();
        let mut reader = XmlResponse::new(my_stack);

        // skip two leading fields since we ignore them (xml declaration, return type declaration)
        reader.next();
        reader.next();

        match start_element("ListQueuesResult", &mut reader) {
            Ok(_) => (),
            Err(_) => panic!("Couldn't find start element"),
        }
    }

    #[test]
    fn string_field_happy_path() {
        let mut file = File::open("test_resources/list_queues_with_queue.xml").unwrap();
        let mut body = String::new();
        let _size = file.read_to_string(&mut body);
        let my_parser = EventReader::new(body.as_bytes());
        let my_stack = my_parser.into_iter().peekable();
        let mut reader = XmlResponse::new(my_stack);

        // skip two leading fields since we ignore them (xml declaration, return type declaration)
        reader.next();
        reader.next();

        reader.next(); // reader now at ListQueuesResult

        // now we're set up to use string:
        let my_chars = string_field("QueueUrl", &mut reader).unwrap();
        assert_eq!(
            my_chars,
            "https://sqs.us-east-1.amazonaws.com/347452556413/testqueue"
        )
    }

    #[test]
    fn end_element_happy_path() {
        let mut file = File::open("test_resources/list_queues_with_queue.xml").unwrap();
        let mut body = String::new();
        let _size = file.read_to_string(&mut body);
        let my_parser = EventReader::new(body.as_bytes());
        let my_stack = my_parser.into_iter().peekable();
        let mut reader = XmlResponse::new(my_stack);

        // skip two leading fields since we ignore them (xml declaration, return type declaration)
        reader.next();
        reader.next();

        // TODO: this is fragile and not good: do some looping to find end element?
        // But need to do it without being dependent on peek_at_name.
        reader.next();
        reader.next();
        reader.next();
        reader.next();

        match end_element("ListQueuesResult", &mut reader) {
            Ok(_) => (),
            Err(_) => panic!("Couldn't find end element"),
        }
    }

    #[test]
    fn test_find_start_element() {
        let body = include_bytes!("../../../test_resources/list_queues_with_queue.xml");
        let parser = EventReader::new(&body[..]);
        let stack = parser.into_iter().peekable();
        let mut reader = XmlResponse::new(stack);

        // skip first two elements
        find_start_element(&mut reader);
        assert_eq!(peek_at_name(&mut reader).unwrap(), "ListQueuesResponse");

        // already at start element
        find_start_element(&mut reader);
        assert_eq!(peek_at_name(&mut reader).unwrap(), "ListQueuesResponse");
    }
}
