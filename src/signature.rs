extern crate regex;
use credentials::AWSCredentials;
use hyper::client::Response;
use openssl::crypto::hash::Type::SHA256;
use openssl::crypto::hash::hash;
use openssl::crypto::hmac::hmac;
use params::Params;
use serialize::hex::ToHex;
use std::ascii::AsciiExt;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::str;
use time::Tm;
use time::now_utc;
use url::percent_encoding::{percent_encode_to, FORM_URLENCODED_ENCODE_SET};
use regions::*;
use request::send_request;
// Debug:
// use std::io::Read;

/// A data structure for all the elements of an HTTP request that are involved in
/// the Amazon Signature Version 4 signing process
#[derive(Debug)]
pub struct SignedRequest<'a> {
	method: String,
	service: String,
	region: &'a Region,
	path: String,
	headers: BTreeMap<String, Vec<Vec<u8>>>,
	params: Params,
	hostname: Option<String>,
	payload: Option<&'a [u8]>,
	canonical_query_string: String,
	canonical_uri: String,
}

impl <'a> SignedRequest <'a> {
	/// Default constructor
	pub fn new<'b>(method: &str, service: &str, region: &'a Region, path: &str) -> SignedRequest<'a> {
		SignedRequest {
			method: method.to_string(),
			service: service.to_string(),
			region: region,
			path: path.to_string(),
			headers: BTreeMap::new(),
			params: Params::new(),
			hostname: None,
			payload: None,
			canonical_query_string: String::new(),
			canonical_uri: String::new(),
		 }
	}

	pub fn set_hostname(&mut self, hostname: Option<String>) {
		self.hostname = hostname;
	}

	pub fn set_payload(&mut self, payload: Option<&'a [u8]>) {
		self.payload = payload;
	}

	pub fn get_method(&self) -> &str {
		&self.method
	}

	pub fn get_canonical_uri(&self) -> &str {
		&self.canonical_uri
	}

	pub fn get_canonical_query_string(&self) -> &str {
		&self.canonical_query_string
	}

	pub fn get_payload(&self) -> Option<&'a [u8]> {
		self.payload
	}

	pub fn get_headers(&'a self) -> &'a BTreeMap<String, Vec<Vec<u8>>> {
		&self.headers
	}

	pub fn get_hostname(&self) -> String {
		match self.hostname {
			Some(ref h) => h.to_string(),
			None => build_hostname(&self.service, &self.region)
		}
	}

	/// Add a value to the array of headers for the specified key.
	/// Headers are kept sorted by key name for use at signing (BTreeMap)
	pub fn add_header(&mut self, key: &str, value: &str) {
		let key_lower = key.to_ascii_lowercase().to_string();
		let value_vec = value.as_bytes().to_vec();

		match self.headers.entry(key_lower) {
			Entry::Vacant(entry) => {
				let mut values = Vec::new();
				values.push(value_vec);
				entry.insert(values);
			}
			Entry::Occupied(entry) => {
				entry.into_mut().push(value_vec);
			}
		}
	}

	pub fn add_param<S>(&mut self, key: S, value: S)  where S: Into<String> {
		self.params.insert(key.into(), value.into());
	}

	pub fn set_params(&mut self, params: Params){
		self.params = params;
	}

	/// Calculate the signature from the credentials provided and the request data
	/// Add the calculated signature to the request headers and execute it
	/// Return the hyper HTTP response
	pub fn sign_and_execute(&mut self, creds: &AWSCredentials) -> Response {
		let hostname = match self.hostname {
			Some(ref h) => h.to_string(),
			None => build_hostname(&self.service, &self.region)
		};
		self.add_header("host", &hostname);

		if let Some(ref token) = *creds.get_token() {
			self.add_header("X-Amz-Security-Token", token);
		}

		self.canonical_query_string = build_canonical_query_string(&self.params);

		let date = now_utc();
		self.add_header("x-amz-date", &date.strftime("%Y%m%dT%H%M%SZ").unwrap().to_string());

		// build the canonical request
		let signed_headers = signed_headers(&self.headers);
		self.canonical_uri = canonical_uri(&self.path);
		let canonical_headers = canonical_headers(&self.headers);

		let mut canonical_request : String;

		match self.payload {
			None => {
				canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}",
					&self.method,
					self.canonical_uri,
					self.canonical_query_string,
					canonical_headers,
					signed_headers,
					&to_hexdigest_from_string(""));
				self.add_header("x-amz-content-sha256", &to_hexdigest_from_string(""));
			}
			Some(payload) => {
				// This is hashing the payload twice, booo:
				canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}",
					&self.method,
					self.canonical_uri,
					self.canonical_query_string,
					canonical_headers,
					signed_headers,
					&to_hexdigest_from_bytes(payload));
				self.add_header("x-amz-content-sha256", &to_hexdigest_from_bytes(payload));
				self.add_header("content-length", &format!("{}", payload.len()));
			}
		}
		self.add_header("content-type", "application/octet-stream");


		// use the hashed canonical request to build the string to sign
		let hashed_canonical_request = to_hexdigest_from_string(&canonical_request);
		let scope = format!("{}/{}/{}/aws4_request", date.strftime("%Y%m%d").unwrap(), region_in_aws_format(&self.region), &self.service);
		let string_to_sign = string_to_sign(date, &hashed_canonical_request, &scope);

		// construct the signing key and sign the string with it
		let signing_key = signing_key(&creds.get_aws_secret_key(), date, &region_in_aws_format(&self.region), &self.service);
		let signature = signature(&string_to_sign, signing_key);

		// build the actual auth header
		let auth_header = format!("AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
	               &creds.get_aws_access_key_id(), scope, signed_headers, signature);
		self.add_header("authorization", &auth_header);

		// TODO: if Response is a redirect, re-issue request to that location
		send_request(&self)
	}
}

fn signature(string_to_sign: &str, signing_key: Vec<u8>) -> String {
	hmac(SHA256, &signing_key, string_to_sign.as_bytes()).to_hex().to_string()
}

fn signing_key(secret: &str, date: Tm, region: &str, service: &str) -> Vec<u8> {
	let k_date = hmac(SHA256, format!("AWS4{}", secret).as_bytes(), date.strftime("%Y%m%d").unwrap().to_string().as_bytes());
	let k_region = hmac(SHA256, &k_date, region.as_bytes());
	let k_service = hmac(SHA256, &k_region, service.as_bytes());
	hmac(SHA256, &k_service, "aws4_request".as_bytes())
}

pub fn string_to_sign(date: Tm, hashed_canonical_request: &str, scope: &str) -> String {
	format!("AWS4-HMAC-SHA256\n{}\n{}\n{}",
		date.strftime("%Y%m%dT%H%M%SZ").unwrap(),
		scope,
		hashed_canonical_request)
}

fn signed_headers(headers: &BTreeMap<String, Vec<Vec<u8>>>) -> String {
	let mut signed = String::new();

	for (key,_) in headers.iter() {
		if signed.len() > 0 {
			signed.push(';')
		}

		if skipped_headers(key) {
			continue;
		}
		signed.push_str(&key.to_ascii_lowercase());
	}
    signed
}

fn canonical_headers(headers: &BTreeMap<String, Vec<Vec<u8>>>) -> String {
	let mut canonical = String::new();

	for item in headers.iter() {
		if skipped_headers(item.0) {
			continue;
		}
		canonical.push_str(format!("{}:{}\n", item.0.to_ascii_lowercase(), canonical_values(item.1)).as_ref());
	}
	canonical
}

fn canonical_values(values: &Vec<Vec<u8>>) -> String {
	let mut st = String::new();
    for v in values {
        let s = str::from_utf8(v).unwrap();
        if st.len() > 0 {
            st.push(',')
        }
        if s.starts_with("\""){
            st.push_str(&s);
        } else {
            st.push_str(s.replace("  ", " ").trim());
        }
    }
    st
}

fn skipped_headers(header: &str) -> bool {
    ["authorization", "content-length", "user-agent"].contains(&header)
}

fn canonical_uri(path: &str) -> String {
	match path {
		"" => "/".to_string(),
		_ => path.to_string()
	}
}

fn build_canonical_query_string(params: &Params) -> String {
	if params.len() == 0 {
		return String::new();
    }

	let mut output = String::new();
	for item in params.iter() {
		if output.len() > 0 {
			output.push_str("&");
		}
		byte_serialize(item.0, &mut output);
		output.push_str("=");
		byte_serialize(item.1, &mut output);
	}

	output
}

#[inline]
fn byte_serialize(input: &str, output: &mut String) {
	for &byte in input.as_bytes().iter() {
		percent_encode_to(&[byte], FORM_URLENCODED_ENCODE_SET, output)
	}
}

// TODO: consolidate these functions
fn to_hexdigest_from_string(val: &str) -> String {
    let h = hash(SHA256, val.as_bytes());
    h.to_hex().to_string()
}

fn to_hexdigest_from_bytes(val: &[u8]) -> String {
	let h = hash(SHA256, val);
    h.to_hex().to_string()
}

fn build_hostname(service: &str, region: &Region) -> String {
	//iam has only 1 endpoint, other services have region-based endpoints
	match service {
		"iam" => format!("{}.amazonaws.com", service),
		"s3" => {
				match *region {
					Region::UsEast1 => return "s3.amazonaws.com".to_string(),
					_ => return format!("s3-{}.amazonaws.com", region_in_aws_format(region)),
				}
			}
		_ => format!("{}.{}.amazonaws.com", service, region_in_aws_format(region))
	}
}


#[cfg(test)]
mod tests {
    use super::*;
	use regions::*;

	#[test]
	fn get_hostname_none_present() {
		let region = Region::UsEast1;
		let request = SignedRequest::new("POST", "sqs", &region, "/");
		assert_eq!("sqs.us-east-1.amazonaws.com", request.get_hostname());
	}

	#[test]
	fn get_hostname_happy_path() {
		let region = Region::UsEast1;
		let mut request = SignedRequest::new("POST", "sqs", &region, "/");
		request.set_hostname(Some("test-hostname".to_string()));
		assert_eq!("test-hostname", request.get_hostname());
	}

}
