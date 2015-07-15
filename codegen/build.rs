extern crate regex;
extern crate wsdlutil;
extern crate xml;
extern crate xmlutil;

use xml::*;
use xml::reader::ParserConfig;
use wsdlutil::*;
use std::ascii::AsciiExt;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::collections::HashMap;
use std::env;
use regex::{Regex, Captures};


/// generate code to parse the element from an XmlStack
fn elem_to_parser_code(elem: &Element) -> String {
	if elem.ref_to.is_some() {
		// element is a reference to another complex type
		let ref_to = elem.ref_to.clone().unwrap();
		let var_type = ref_to.replace("tns:", "");
		let var_name = c_to_s(&var_type);
		to_parser_code(&var_type, &var_type, &var_name, elem.min_occurs.clone(), elem.max_occurs.clone())
	} else {
		// element is a primitive type 
		let elem_name = elem.name.clone().unwrap();
		let var_name = c_to_s(&elem_name);
		let var_type = get_var_type(&elem.elem_type.clone().unwrap());
		to_parser_code(&elem_name, &var_type, &var_name, elem.min_occurs.clone(), elem.max_occurs.clone())		
	} 
}


/// generate rust code to parse an element from an XmlStack
fn to_parser_code(raw_name: &str, var_type: &str, var_name: &str, min_occurs: Option<String>, max_occurs: Option<String>) -> String {
	let mut code = "".to_string();
	
	if max_occurs.unwrap_or("".to_string()) == "unbounded" {
		// add elements to a vector while the next tag matches
		code.push_str(&format!("while try!(peek_at_name(stack)) == \"{}\" {{ ", raw_name));
		code.push_str(&format!("obj.{}.push({});", var_name, parse_single(raw_name, var_type)));
		code.push_str("}");
	} else if min_occurs.unwrap_or("".to_string()) == "0" {
		code.push_str(&format!("if try!(peek_at_name(stack)) == \"{}\" {{ obj.{} = Some({}) }}", raw_name, var_name, parse_single(raw_name, var_type)));
	} else {
		code.push_str(&format!("obj.{} = {};", var_name, parse_single(raw_name, var_type)));			
	}
	code
}


/// generate a variable declaration for use inside a struct
fn to_var_declaration(elem: &Element) -> String {

	let var_name;
	let var_type;
	if elem.ref_to.is_some() {
		// element is a reference to another complex type
		var_type = elem.ref_to.clone().unwrap().replace("tns:", "");
		var_name = c_to_s(&var_type);
	} else {
		// element is a primitive type 
		var_name = c_to_s(&elem.name.clone().unwrap());
		var_type = get_var_type(&elem.elem_type.clone().unwrap());
	}	
	format!("\tpub {}: {},\n", var_name, to_rust_type(&var_type, elem.min_occurs.clone(), elem.max_occurs.clone()))
}

/// generate rust code to parse a single required element from an XmlStack
fn parse_single(raw_name: &str, var_type: &str) -> String {
	if var_type == "String" {
		format!("try!(string_field(\"{}\", stack))", raw_name)
	} else if var_type == "i32" { 		
		format!("i32::from_str(try!(string_field(\"{}\", stack)).as_ref()).unwrap()", raw_name)
	} else {							
		format!("try!({}::parse_xml(stack))", var_type)
	}			
}

/// generate the rust type for an xsd element type
fn to_rust_type(var_type: &str, min_occurs: Option<String>, max_occurs: Option<String>) -> String {
	if max_occurs.unwrap_or("".to_string()) == "unbounded" {
		format!("Vec<{}>", var_type)
	} else if min_occurs.unwrap_or("".to_string()) == "0" {
		format!("Option<{}>", var_type)
	} else {
		var_type.to_string()
	}
}

/// get a rust type name for an xsd element type
fn get_var_type(elem_type: &str) -> String {
	if elem_type.starts_with("tns:") {
		elem_type.replace("tns:","")
	} else {
		match elem_type {
			"xs:integer" => "i32".to_string(),
			// default everything else to String for now and deal with it later
			_ => "String".to_string()
		}
	}		
}


/// produce a rust struct declaration and a parse method for this type
fn to_struct(ct: &ComplexType, name: &str) -> String {
	let elements = &ct.sequence.elements;

	let mut rust = format!("#[derive(Debug, Default)]\npub struct {}",  name);

	if elements.is_empty() && ct.sequence.choice.is_none() {
		rust.push_str(";\n\n");
	} else {
		rust.push_str(" {\n");
		for child in elements {
			rust.push_str(&to_var_declaration(&child));
		}

		if let Some(ref choice) = ct.sequence.choice {
			for child in &choice.elements {
				rust.push_str(&to_var_declaration(&child));
			}	
		};

		rust.push_str("}\n\n");
	}				
	
	rust.push_str(&format!("impl XmlParser for {} {{\n\tfn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {{", name));
	rust.push_str(&format!("\n\t\tlet mut obj = {}::default();", name));
	rust.push_str(&format!("\n\t\ttry!(start_element(\"{}\", stack));", name));

	for child in elements {
		rust.push_str("\n\t\t");
		rust.push_str(&elem_to_parser_code(&child));
	}

	rust.push_str(&format!("\n\t\ttry!(end_element(\"{}\", stack));", name));
	rust.push_str("\n\t\tOk(obj)\n\t}\n}\n\n");
		
	rust 
}


fn main() {

	let mut file = File::open("QueueService.wsdl").unwrap();
	let mut contents:Vec<u8> = Vec::new();
	let _ = file.read_to_end(&mut contents);
	let mut cfg:ParserConfig = ParserConfig::new();
    cfg.trim_whitespace = true;

	let mut reader = EventReader::with_config(contents.as_ref(), cfg);
    //let mut reader = EventReader::new(contents.as_ref());
    let mut stack = reader.events().peekable();
    
    // skip the document start tag
    stack.next();
    
    let result = Definitions::parse_xml(&mut stack);
    
	if let Ok(wsdl) = result {

		let mut actions = Vec::new();

		for operation in wsdl.port_type.operations {
			actions.push(operation.name);
		}

		println!("{:#?}", actions);

		// output rust code for each element in the XSD
		let mut data_model = String::new();
		for element in wsdl.types.schema.elements {

			if let Some(ref elem_name) = element.name {
				if elem_name == "ErrorResponse" || elem_name == "Error" {
					continue
				}
				
				if let Some(ref ct) = element.complex_type {
					data_model.push_str(&format!("{}", to_struct(&ct, &elem_name)));
					if actions.contains(&elem_name) && !skipped_elements(elem_name) {
						data_model.push_str(&impl_sqs_request(elem_name, ct));
					}
				}
			}
		}

		// output rust code for freefloating complex types in the XSD
		for complex_type in wsdl.types.schema.complex_types {
			data_model.push_str(&format!("{}", to_struct(&complex_type, &complex_type.name.clone().unwrap())));
		}

		// write generated code to a file for the next phase of compilation
		let out_dir = env::var("OUT_DIR").unwrap();
	    let dest_path = Path::new(&out_dir).join("generated_sqs.rs");
	    
	    let mut f = File::create(&dest_path).unwrap();
	    f.write_all(data_model.as_bytes()).unwrap();
	}
	else {
		println!("{:#?}", result);
	} 
}

/// implement the SQSRequest trait for the element with the appropriate response object and to_params() method
fn impl_sqs_request(elem_name: &str, ct: &ComplexType) -> String {
	let mut code = String::new();

	code.push_str(&format!("impl SQSRequest<{}Response> for {} {{", elem_name, elem_name));
	code.push_str("\n\tfn to_params(&self) -> Params {"); 
	code.push_str("\n\t\tlet mut params = Params::new();");
	code.push_str(&format!("\n\t\tparams.put(\"Action\", \"{}\");", elem_name));

	for elem in &ct.sequence.elements {
		code.push_str(&format!("\n\t\t{}", to_params(elem)));
	}

	code.push_str("\n\t\tparams\n\t}\n}\n\n");
	code
}


fn to_params(elem: &Element) -> String {
	if let Some(ref name) = elem.name {
		if let Some(ref _max_occurs) = elem.max_occurs {
			format!("let mut index = 1; for ref elem in &self.{} {{ params.put(&format!(\"{}.{{}}\", index), &elem); index += 1; }}", c_to_s(name), name)
		} else if let Some(ref _min_occurs) = elem.min_occurs {
			format!("params.optional_put(\"{}\", &self.{});", name, c_to_s(name))			
		} else {
			format!("params.put(\"{}\", &self.{}.to_string());", name, c_to_s(name))
		}			
	} else if let Some(ref ref_to) = elem.ref_to {
		let var_name = c_to_s(&ref_to.clone().replace("tns:",""));
		format!("serialize_{}(&mut params, &self.{});", var_name, var_name)
	} 
	else {
		"".to_string()
	}
	
}

/// convert a CamelCase string to a snake_case string
fn c_to_s(name: &str) -> String {

	let re = Regex::new(r"([a-z0-9])([A-Z])").unwrap();
	let result = re.replace_all(name, |caps: &Captures| {
    	format!("{}_{}", caps.at(1).unwrap_or(""), caps.at(2).unwrap_or(""))
	});
	result.to_ascii_lowercase()
}

/// some elements require hand-written code, don't implement SQSRequest for them
fn skipped_elements(element: &str) -> bool {
    ["ChangeMessageVisibilityBatch"].contains(&element)
}
