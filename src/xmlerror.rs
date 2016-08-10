use xmlutil::{XmlParseError, Peek, Next};
use xmlutil::{characters, start_element, end_element, string_field, peek_at_name};

#[derive(Default, Debug)]
pub struct XmlError {
	pub error_type: String,
	pub code: String,
	pub message: String,
	pub detail: Option<String>
}

pub struct XmlErrorDeserializer;
impl XmlErrorDeserializer {
	pub fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<XmlError, XmlParseError> {
		try!(start_element(tag_name, stack));

		let mut obj = XmlError::default();

		loop {
			match &try!(peek_at_name(stack))[..] {
				"Type" => {
					obj.error_type = try!(string_field("Type", stack));
					continue;
				},
				"Code" => {
					obj.code = try!(string_field("Code", stack));
					continue;
				},
				"Message" => {
					obj.message = try!(string_field("Message", stack));
					continue;
				},
				"Detail" => {
					try!(start_element("Detail", stack));
					if let Ok(characters) = characters(stack){
						obj.detail = Some(characters.to_string());
						try!(end_element("Detail", stack));
					}
					continue;
				},
				_ => break
			}
		}

		try!(end_element(tag_name, stack));

		Ok(obj)
	}
}
