//! Tools for handling XML from AWS with helper functions for testing.
//!
//! Wraps an XML stack via traits.
//! Also provides a method of supplying an XML stack from a file for testing purposes.

use std::iter::Peekable;
use std::num::ParseIntError;
use std::collections::HashMap;
use xml::reader::*;
use xml::reader::events::*;

/// generic Error for XML parsing
#[derive(Debug)]
pub struct XmlParseError(pub String);

impl XmlParseError {
    pub fn new(msg: &str) -> XmlParseError {
        XmlParseError(msg.to_string())
    }
}

/// syntactic sugar for the XML event stack we pass around
pub type XmlStack<'a> = Peekable<Events<'a, &'a[u8]>>;

/// Peek at next items in the XML stack
pub trait Peek {
    fn peek(&mut self) -> Option<&XmlEvent>;
}

/// Move to the next part of the XML stack
pub trait Next {
    fn next(&mut self) -> Option<XmlEvent>;
}

/// Wraps the Hyper Response type
pub struct XmlResponse<'b> {
    xml_stack: Peekable<Events<'b, &'b [u8]>> // refactor to use XmlStack type?
}

impl <'b>XmlResponse<'b> {
    pub fn new(stack: Peekable<Events<'b, &'b [u8]>>) -> XmlResponse {
        XmlResponse {
            xml_stack: stack,
        }
    }
}

impl <'b>Peek for XmlResponse<'b> {
    fn peek(&mut self) -> Option<&XmlEvent> {
        loop {
            match self.xml_stack.peek() {
                Some(&XmlEvent::Whitespace(_)) => {  },
                _ => break
            }
            self.xml_stack.next();
        }
        self.xml_stack.peek()
    }
}

impl <'b> Next for XmlResponse<'b> {
    fn next(&mut self) -> Option<XmlEvent> {
        let mut maybe_event;
        loop {
            maybe_event = self.xml_stack.next();
            match maybe_event {
                Some(XmlEvent::Whitespace(_)) => {},
                _ => break
            }
        }
        maybe_event
    }
}

impl From<ParseIntError> for XmlParseError{
    fn from(_e:ParseIntError) -> XmlParseError { XmlParseError::new("ParseIntError") }
}


/// parse Some(String) if the next tag has the right name, otherwise None
pub fn optional_string_field<T: Peek + Next>(field_name: &str, stack: &mut T) -> Result<Option<String>, XmlParseError> {
    if try!(peek_at_name(stack)) == field_name {
        let val = try!(string_field(field_name, stack));
        Ok(Some(val))
    } else {
        Ok(None)
    }
}

/// return a string field with the right name or throw a parse error
pub fn string_field<T: Peek + Next>(name: &str, stack: &mut T) -> Result<String, XmlParseError> {
    try!(start_element(name, stack));
    let value = try!(characters(stack));
    try!(end_element(name, stack));
    Ok(value)
}

/// return some XML Characters
pub fn characters<T: Peek + Next>(stack: &mut T) -> Result<String, XmlParseError> {
    if let Some(XmlEvent::Characters(data)) = stack.next() {
        Ok(data.to_string())
    } else { 
         Err(XmlParseError::new("Expected characters"))
    }
}

/// get the name of the current element in the stack.  throw a parse error if it's not a `StartElement`
pub fn peek_at_name<T: Peek + Next>(stack: &mut T) -> Result<String, XmlParseError> {
    let current = stack.peek();
    if let Some(&XmlEvent::StartElement{ref name, ..}) = current {
        Ok(name.local_name.to_string())
    } else {
        Ok("".to_string())
    }
}

/// consume a `StartElement` with a specific name or throw an `XmlParseError`
pub fn start_element<T: Peek + Next>(element_name: &str, stack: &mut T)  -> Result<HashMap<String, String>, XmlParseError> {
    let next = stack.next();

    if let Some(XmlEvent::StartElement { name, attributes, .. }) = next {
        if name.local_name == element_name {
            let mut attr_map = HashMap::new();
            for attr in attributes {
                attr_map.insert(attr.name.local_name, attr.value);
            }
            Ok(attr_map)
        } else {
            Err(XmlParseError::new(&format!("START Expected {} got {}", element_name, name.local_name)))
        }
    } else {
        Err(XmlParseError::new(&format!("Expected StartElement {}", element_name)))
    }
}

/// consume an `EndElement` with a specific name or throw an `XmlParseError`
pub fn end_element<T: Peek + Next>(element_name: &str, stack: &mut T)  -> Result<(), XmlParseError> {
    let next = stack.next();
    if let Some(XmlEvent::EndElement { name, .. }) = next {
        if name.local_name == element_name {
            Ok(())
        } else {
            Err(XmlParseError::new(&format!("END Expected {} got {}", element_name, name.local_name)))
        }
    }else {
        Err(XmlParseError::new(&format!("Expected EndElement {} got {:?}", element_name, next)))
    }
}

/// skip a tag and all its children
pub fn skip_tree<T: Peek + Next>(stack: &mut T) {

    let mut deep: usize = 0;

    loop {
        match stack.next() {
            None => break,
            Some(XmlEvent::StartElement { .. }) => deep += 1,
            Some(XmlEvent::EndElement { ..}) => {
                if deep > 1 {
                    deep -= 1;
                } else {
                    break;
                }
            },
            _ => (),
        }
    }

}
#[cfg(test)]
mod tests {
    use super::*;
    use xml::reader::*;
    use std::io::Read;
    use std::fs::File;

    #[test]
    fn peek_at_name_happy_path() {
        let mut file = File::open("tests/sample-data/list_queues_with_queue.xml").unwrap();
        let mut body = String::new();
        let _size = file.read_to_string(&mut body);
        let mut my_parser  = EventReader::new(body.as_bytes());
        let my_stack = my_parser.events().peekable();
        let mut reader = XmlResponse::new(my_stack);

        loop {
            reader.next();
            match peek_at_name(&mut reader) {
                Ok(data) => {
                    if data == "QueueUrl" {
                        return;
                    }
                }
                Err(_) => panic!("Couldn't peek at name")
            }
        }
    }

    #[test]
    fn start_element_happy_path() {
        let mut file = File::open("tests/sample-data/list_queues_with_queue.xml").unwrap();
        let mut body = String::new();
        let _size = file.read_to_string(&mut body);
        let mut my_parser  = EventReader::new(body.as_bytes());
        let my_stack = my_parser.events().peekable();
        let mut reader = XmlResponse::new(my_stack);

        // skip two leading fields since we ignore them (xml declaration, return type declaration)
        reader.next();
        reader.next();

        match start_element("ListQueuesResult", &mut reader) {
            Ok(_) => (),
            Err(_) => panic!("Couldn't find start element")
        }
    }

    #[test]
    fn string_field_happy_path() {
        let mut file = File::open("tests/sample-data/list_queues_with_queue.xml").unwrap();
        let mut body = String::new();
        let _size = file.read_to_string(&mut body);
        let mut my_parser  = EventReader::new(body.as_bytes());
        let my_stack = my_parser.events().peekable();
        let mut reader = XmlResponse::new(my_stack);

        // skip two leading fields since we ignore them (xml declaration, return type declaration)
        reader.next();
        reader.next();

        reader.next(); // reader now at ListQueuesResult

        // now we're set up to use string:
        let my_chars = string_field("QueueUrl", &mut reader).unwrap();
        assert_eq!(my_chars, "https://sqs.us-east-1.amazonaws.com/347452556413/testqueue")
    }

    #[test]
    fn end_element_happy_path() {
        let mut file = File::open("tests/sample-data/list_queues_with_queue.xml").unwrap();
        let mut body = String::new();
        let _size = file.read_to_string(&mut body);
        let mut my_parser  = EventReader::new(body.as_bytes());
        let my_stack = my_parser.events().peekable();
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
            Err(_) => panic!("Couldn't find end element")
        }
    }

}
