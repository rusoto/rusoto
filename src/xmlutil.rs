//! Tools for handling XML from AWS with helper functions for testing
//!
//! Wraps an XML stack via traits.  Also provides a method of supplying an XML stack from a file
//! for testing purposes.
//!

use std::iter::Peekable;
use std::num::ParseIntError;
use hyper::client::response::*;
use std::collections::HashMap;
use xml::reader::*;
use std::io::BufReader;
use std::fs::File;
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
pub type XmlStack<'a> = Peekable<Events<'a, Response>>;

/// Peek at next items in the XML stack
pub trait Peek {
    fn peek(&mut self) -> Option<&XmlEvent>;
}

/// Move to the next part of the XML stack
pub trait Next {
	fn next(&mut self) -> Option<XmlEvent>;
}

/// Wraps the Hyper Response type
pub struct XmlResponseFromAws<'b> {
	xml_stack: Peekable<Events<'b, Response>> // refactor to use XmlStack type?
}

impl <'b>XmlResponseFromAws<'b> {
	pub fn new<'c>(stack: Peekable<Events<'b, Response>>) -> XmlResponseFromAws {
		XmlResponseFromAws {
			xml_stack: stack,
		}
	}
}

impl <'b>Peek for XmlResponseFromAws<'b> {
	fn peek(&mut self) -> Option<&XmlEvent> {
		return self.xml_stack.peek();
	}
}

impl <'b> Next for XmlResponseFromAws<'b> {
	fn next(&mut self) -> Option<XmlEvent> {
		return self.xml_stack.next();
	}
}

impl From<ParseIntError> for XmlParseError{
    fn from(_e:ParseIntError) -> XmlParseError { XmlParseError::new("ParseIntError") }
}

/// Testing helper, reads from file
pub struct XmlResponseFromFile<'a> {
	xml_stack: Peekable<Events<'a, BufReader<File>>>,
}

impl <'a>XmlResponseFromFile<'a> {
	pub fn new<'c>(my_stack: Peekable<Events<'a, BufReader<File>>>) -> XmlResponseFromFile {
		return XmlResponseFromFile { xml_stack: my_stack };
	}
}

// Need peek and next implemented.
impl <'b> Peek for XmlResponseFromFile <'b> {
	fn peek(&mut self) -> Option<&XmlEvent> {
		return self.xml_stack.peek();
	}
}

impl <'b> Next for XmlResponseFromFile <'b> {
	fn next(&mut self) -> Option<XmlEvent> {
		return self.xml_stack.next();
	}
}
// /testing helper

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

/// get the name of the current element in the stack.  throw a parse error if it's not a StartElement
pub fn peek_at_name<T: Peek + Next>(stack: &mut T) -> Result<String, XmlParseError> {
	let current = stack.peek();
	if let Some(&XmlEvent::StartElement{ref name, ..}) = current {
		Ok(name.local_name.to_string())
	} else {
		Ok("".to_string())
	}
}

/// consume a StartElement with a specific name or throw an XmlParseError
pub fn start_element<T: Peek + Next>(element_name: &str, stack: &mut T)  -> Result<HashMap<String, String>, XmlParseError> {
	println!("in start_element looking for {}", element_name);
	let next = stack.next();
	println!("next set...");
	if let Some(XmlEvent::StartElement { name, attributes, .. }) = next {
		println!("name.local_name is {}", name.local_name);
		if name.local_name != element_name {
			println!("erroring out deal with it");
			Err(XmlParseError::new(&format!("Expected {} got {}", element_name, name.local_name)))
		} else {
			println!("doing hasmap stuff");
			let mut attr_map = HashMap::new();
			for attr in attributes {
				attr_map.insert(attr.name.local_name, attr.value);
			}
			println!("done with hashmap stuff");
			Ok(attr_map)
		}
	} else {
		println!("oh god things are terrible");
		Err(XmlParseError::new(&format!("Expected StartElement {}", element_name)))
	}
}

/// consume an EndElement with a specific name or throw an XmlParseError
pub fn end_element<T: Peek + Next>(element_name: &str, stack: &mut T)  -> Result<(), XmlParseError> {
	let next = stack.next();
	if let Some(XmlEvent::EndElement { name, .. }) = next {
		if name.local_name != element_name {
			Err(XmlParseError::new(&format!("Expected {} got {}", element_name, name.local_name)))
		} else {
			Ok(())
		}
	}else {
		Err(XmlParseError::new(&format!("Expected EndElement {} got {:?}", element_name, next)))
	}
}

#[cfg(test)]
mod tests {
    use super::*;
	use xml::reader::*;
	use std::io::BufReader;
	use std::fs::File;

	#[test]
	fn peek_at_name_happy_path() {
	    let file = File::open("tests/sample-data/list_queues_with_queue.xml").unwrap();
	    let file = BufReader::new(file);
	    let mut my_parser  = EventReader::new(file);
	    let my_stack = my_parser.events().peekable();
	    let mut reader = XmlResponseFromFile::new(my_stack);

	    loop {
	        reader.next();
	        match peek_at_name(&mut reader) {
	            Ok(data) => {
	                // println!("Got {}", data);
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
	    let file = File::open("tests/sample-data/list_queues_with_queue.xml").unwrap();
	    let file = BufReader::new(file);
	    let mut my_parser  = EventReader::new(file);
	    let my_stack = my_parser.events().peekable();
	    let mut reader = XmlResponseFromFile::new(my_stack);

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
	    let file = File::open("tests/sample-data/list_queues_with_queue.xml").unwrap();
	    let file = BufReader::new(file);
	    let mut my_parser  = EventReader::new(file);
	    let my_stack = my_parser.events().peekable();
	    let mut reader = XmlResponseFromFile::new(my_stack);

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
	    let file = File::open("tests/sample-data/list_queues_with_queue.xml").unwrap();
	    let file = BufReader::new(file);
	    let mut my_parser  = EventReader::new(file);
	    let my_stack = my_parser.events().peekable();
	    let mut reader = XmlResponseFromFile::new(my_stack);

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
