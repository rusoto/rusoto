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
    let file = File::open("tests/sample-data/list_queues_with_queue.xml").unwrap();
    let file = BufReader::new(file);

    let mut my_parser  = EventReader::new(file);
    let mut my_stack = my_parser.events().peekable();

    let mut reader = XmlResponseFromFile::new(my_stack);

    // try peek_at_name
    match peek_at_name(&mut reader) {
        Ok(data) => println!("Got {}", data),
        Err(_) => panic!("Couldn't peek at name")
    }
}

pub struct XmlResponseFromFile<'a> {
	xml_stack: Peekable<Events<'a, BufReader<File>>>,
}

// I cannot explain how these lifetimes work to a child, therefore I need to understand them better:
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
