use log::*;

use super::util::{characters, end_element, peek_at_name, skip_tree, start_element, string_field};
use super::util::{Next, Peek, XmlParseError};

#[derive(Default, Debug)]
pub struct XmlError {
    pub error_type: String,
    pub code: String,
    pub message: String,
    pub detail: Option<String>,
}

pub struct XmlErrorDeserializer;
impl XmlErrorDeserializer {
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<XmlError, XmlParseError> {
        start_element(tag_name, stack)?;

        let mut obj = XmlError::default();

        loop {
            match &peek_at_name(stack)?[..] {
                "Type" => {
                    obj.error_type = string_field("Type", stack)?;
                }
                "Code" => {
                    obj.code = string_field("Code", stack)?;
                }
                "Message" => {
                    obj.message = string_field("Message", stack)?;
                }
                "Detail" => {
                    start_element("Detail", stack)?;
                    if let Ok(characters) = characters(stack) {
                        obj.detail = Some(characters.to_string());
                        end_element("Detail", stack)?;
                    }
                }
                "" => break,
                unknown => {
                    debug!(
                        "Ignoring unknown XML element {:?} in error response.",
                        unknown
                    );
                    skip_tree(stack);
                }
            }
        }

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
