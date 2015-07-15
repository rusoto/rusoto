#![allow(unused_variables, unused_mut)]
use credentials::*;
use xml::*;
use signature::*;
use params::*;
use error::*;
use xmlutil::*;
use std::str::FromStr;

mod add_permission;
mod create_queue;
mod delete_message;
mod get_queue_url;
mod list_queues;
mod receive_message;
mod send_message;

// include the code generated from the SQS WSDL
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/sqs.rs"));

pub trait SQSRequest<R>  {
        fn to_params(&self) -> Params;
}

pub struct SQSClient {
	creds: AWSCredentials,
	region: String
}

impl SQSClient {
	pub fn new<S>(credentials:AWSCredentials, region:S) -> SQSClient where S:Into<String> {
		SQSClient { creds: credentials, region: region.into() }
	}

	pub fn change_message_visibility(&self, queue_url: &str, receipt_handle: &str, visibility_timeout: i32) -> Result<ChangeMessageVisibilityResponse, SQSError> {
		let mut req = ChangeMessageVisibility::default();
		req.receipt_handle = receipt_handle.to_string();
		req.visibility_timeout = visibility_timeout;
		
		// the WSDL specifies a Vec<Attribute> as well, but it's undocumented otherwise, so ignore
		
		self.queue_request(queue_url, req)
	}
	
	pub fn delete_queue(&self, queue_url: &str) -> Result<DeleteQueueResponse, SQSError> {
		self.queue_request(queue_url, DeleteQueue::default())
	}
	
	pub fn get_all_queue_attributes(&self, queue_url: &str) -> Result<GetQueueAttributesResponse, SQSError> {
		self.get_queue_attributes(queue_url, vec!["All"])
	}
	
	pub fn get_queue_attributes(&self, queue_url: &str, attributes: Vec<&str>) -> Result<GetQueueAttributesResponse, SQSError> {
		let mut req = GetQueueAttributes::default();
		let mut req_attrs = Vec::new();
		for attr in attributes {
			req_attrs.push(attr.to_string());
		}
		req.attribute_name = req_attrs;
		self.queue_request(queue_url, req)
	}
		
	pub fn get_queue_url(&self, queue_name: &str) -> Result<GetQueueUrlResponse, SQSError> {
		self.service_request(GetQueueUrl::new(queue_name))
	}

	pub fn list_dead_letter_source_queues(&self, queue_url: &str) -> Result<ListDeadLetterSourceQueuesResponse, SQSError> {
		self.queue_request(queue_url, ListDeadLetterSourceQueues::default())
	}
	
	pub fn remove_permission(&self, queue_url: &str, label: &str) -> Result<RemovePermissionResponse, SQSError> {
		let mut req = RemovePermission::default();
		req.label = label.to_string();
		self.queue_request(queue_url, req)
	}

	pub fn service_request<T:SQSRequest<R>,R: Default + XmlParser>(&self, req:T) -> Result<R, SQSError> {
		self.request(req, None)
	}
	
	pub fn queue_request<T:SQSRequest<R>,R: Default + XmlParser>(&self, queue_url: &str, req:T) -> Result<R, SQSError> {
		self.request(req, Some(queue_url.to_string()))
	}

	/// Execute an SQS request and parse the result into the appropriate response object
	fn request<T:SQSRequest<R>,R: Default + XmlParser>(&self, req:T, queue_url:Option<String>) -> Result<R, SQSError> {
		let mut response = <R as Default>::default();
		let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
		let mut params = req.to_params();
		
		if let Some(url) = queue_url {
			params.put("QueueUrl", &url);
		} 
		
		request.set_params(params);

		let (status, output) = try!(request.sign_and_execute(&self.creds));

		//println!("{}", output);

		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();

		// skip the XmlEvent::StartDocument
		stack.next();

		match status {
			200 => {
				// parse the expected object from a valid response
				match <R as XmlParser>::parse_xml(&mut stack) {
					Ok(response) => Ok(response),
					Err(XmlParseError(msg)) => Err(SQSError::new(format!("XmlParseError: {} ", msg)))
				}
			}
			_ => {
				// parse an error object from any non-OK response
				match ErrorResponse::parse_xml(&mut stack) {
					Ok(err) => Err(SQSError::new(err.error.code)),
					_ => Err(SQSError::new(format!("Got response status {} ", status)))
				}
			}
		}
	}

}

#[derive(Debug)]
pub struct SQSError(String);

impl SQSError {
	fn new<S>(msg:S) -> SQSError where S:Into<String>{
		SQSError(msg.into())
	}
}

impl From<URIParseError> for SQSError {
        fn from(err: URIParseError) -> SQSError {
                SQSError(format!("{:?}", err))
        }
}

fn serialize_attribute(params: &mut Params, attributes: &Vec<Attribute>) {
	let mut index = 1;
	for attr in attributes {
		params.put(&format!("Attribute.{}.Name", index), &attr.name);
		params.put(&format!("Attribute.{}.Value", index), &attr.value);
		index += 1;		
	}
}

fn serialize_message_attribute(params: &mut Params, message_attributes: &Vec<MessageAttribute>) {
	let mut index = 1;
	for attr in message_attributes {
		//todo: binary attributes
		params.put(&format!("MessageAttribute.{}.Name", index), &attr.name);
		params.put(&format!("MessageAttribute.{}.Value.StringValue", index), &attr.value.string_value);		
		params.put(&format!("MessageAttribute.{}.Value.DataType", index), "String");						
		index += 1;
	}	
}

fn serialize_delete_message_batch_request_entry(params: &mut Params, entries: &Vec<DeleteMessageBatchRequestEntry>) {
	let mut index = 1;
	for entry in entries {
		params.put(&format!("DeleteMessageBatchRequestEntry.{}.Id", index), &entry.id);
		params.put(&format!("DeleteMessageBatchRequestEntry.{}.ReceiptHandle", index), &entry.receipt_handle);		
		index += 1;
	}	
}


fn serialize_send_message_batch_request_entry(params: &mut Params, entries: &Vec<SendMessageBatchRequestEntry>) {
	let mut index = 1;
	for entry in entries {
		params.put(&format!("SendMessageBatchRequestEntry.{}.Id", index), &entry.id);
		params.put(&format!("SendMessageBatchRequestEntry.{}.MessageBody", index), &entry.message_body);
		params.optional_put(&format!("SendMessageBatchRequestEntry.{}.DelaySeconds", index), &entry.delay_seconds);
		for attr in &entry.message_attribute {
			//todo: binary attributes
			// yes, it really is specified as being this verbose
			params.put(&format!("SendMessageBatchRequestEntry.{}.MessageAttribute.Name", index), &attr.name);
			params.put(&format!("SendMessageBatchRequestEntry.{}.MessageAttribute.Value.StringValue", index), &attr.value.string_value);		
			params.put(&format!("SendMessageBatchRequestEntry.{}.MessageAttribute.Value.DataType", index), "String");						
		}	
		index += 1;
	}		
}


