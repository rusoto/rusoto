extern crate rusoto;
extern crate xml;

use xml::reader::*;
use rusoto::xmlutil::*;
use std::io::BufReader;
use std::fs::File;
use std::iter::Peekable;
use xml::reader::events::*;

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

// #[test]
// fn optional_string_field_happy_path() {
//     panic!("Not implemented");
// }
//
// #[test]
// fn string_field_happy_path() {
//     panic!("Not implemented");
// }
//
// #[test]
// fn characters_happy_path() {
//     panic!("Not implemented");
// }

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
        Ok(_) => println!("Got start"),
        Err(_) => panic!("Couldn't find start element")
    }
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
        Ok(_) => println!("Got end"),
        Err(_) => panic!("Couldn't find end element")
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
