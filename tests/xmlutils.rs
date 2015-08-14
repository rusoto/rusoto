extern crate rusoto;
extern crate xml;

use xml::reader::*;
use rusoto::xmlutil::*;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::fs::File;
use std::fs;
use std::error::Error;
use std::iter::Peekable;
use xml::reader::events::*;

#[test]
fn peek_at_name_happy_path() {
    // open file
    let mut good_xml = open_xmlfile_for_test("tests/sampledata/sqs-happy.xml");
    // try peek_at_name

    match peek_at_name(&mut good_xml) {
        Ok(data) => println!("Got {}", data),
        Err(why) => panic!("Couldn't peek at name")
    }
}

fn open_xmlfile_for_test<'a>(file_location: &str) -> XmlResponseFromFile {
    let file = File::open(file_location).unwrap();
    let file = BufReader::new(file);

    let mut parser = EventReader::new(file);
    let mut XmlResponse = XmlResponseFromFile::new(parser.events().peekable());
    return XmlResponse;
}

pub struct XmlResponseFromFile<'a> {
	xml_stack: Peekable<Events<'a, BufReader<File>>>,
}

// I cannot explain how these lifetimes work to a child, therefore I need to understand them better:
impl <'a>XmlResponseFromFile<'a> {
	pub fn new<'c>(stack: Peekable<Events<'a, BufReader<File>>>) -> XmlResponseFromFile {
		XmlResponseFromFile {
			xml_stack: stack,
		}
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
