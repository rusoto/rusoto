//! AWS API request signatures.
//!
//! Follows [AWS Signature 4](http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html)
//! algorithm.
//!
//! If needed, the request will be re-issued to a temporary redirect endpoint.  This can happen with
//! newly created S3 buckets not in us-standard/us-east-1.

use std::ascii::AsciiExt;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::str;

use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use hyper::client::Response;
use hyper::status::StatusCode;
use rustc_serialize::hex::ToHex;
use time::Tm;
use time::now_utc;
use url::percent_encoding::{percent_encode_to, DEFAULT_ENCODE_SET, FORM_URLENCODED_ENCODE_SET};
use xmlutil::*;
use xml::reader::*;

use credential::AwsCredentials;
use error::AwsError;
use param::Params;
use region::Region;
use request::send_request;

const HTTP_TEMPORARY_REDIRECT: StatusCode = StatusCode::TemporaryRedirect;

/// A data structure for all the elements of an HTTP request that are involved in
/// the Amazon Signature Version 4 signing process
#[derive(Debug)]
pub struct SignedRequest<'a> {
    method: String,
    service: String,
    region: Region,
    path: String,
    headers: BTreeMap<String, Vec<Vec<u8>>>,
    params: Params,
    hostname: Option<String>,
    payload: Option<&'a [u8]>,
    content_type: Option<String>,
    canonical_query_string: String,
    canonical_uri: String,
}

impl <'a> SignedRequest <'a> {
    /// Default constructor
    pub fn new(method: &str, service: &str, region: Region, path: &str) -> SignedRequest<'a> {
        SignedRequest {
            method: method.to_string(),
            service: service.to_string(),
            region: region,
            path: path.to_string(),
            headers: BTreeMap::new(),
            params: Params::new(),
            hostname: None,
            payload: None,
            content_type: None,
            canonical_query_string: String::new(),
            canonical_uri: String::new(),
         }
    }

    pub fn set_content_type(&mut self, content_type: String) {
        self.content_type = Some(content_type);
    }

    pub fn set_hostname(&mut self, hostname: Option<String>) {
        self.hostname = hostname;
    }

    pub fn set_endpoint_prefix(&mut self, endpoint_prefix: String) {
        self.hostname = Some(build_hostname(&endpoint_prefix, self.region));
    }

    pub fn set_payload(&mut self, payload: Option<&'a [u8]>) {
        self.payload = payload;
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn canonical_uri(&self) -> &str {
        &self.canonical_uri
    }

    pub fn canonical_query_string(&self) -> &str {
        &self.canonical_query_string
    }

    pub fn payload(&self) -> Option<&'a [u8]> {
        self.payload
    }

    pub fn headers(&'a self) -> &'a BTreeMap<String, Vec<Vec<u8>>> {
        &self.headers
    }

    pub fn hostname(&self) -> String {
        match self.hostname {
            Some(ref h) => h.to_string(),
            None => build_hostname(&self.service, self.region)
        }
    }

    // If the key exists in headers, set it to blank/unoccupied:
    pub fn remove_header(&mut self, key: &str) {
        let key_lower = key.to_ascii_lowercase().to_string();
        self.headers.remove(&key_lower);
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
    pub fn sign_and_execute(&mut self, creds: AwsCredentials) -> Response {
        self.sign(&creds);
        self.execute(creds)
    }

    fn sign(&mut self, creds: &AwsCredentials) {
        debug!("Creating request to send to AWS.");
        let hostname = match self.hostname {
            Some(ref h) => h.to_string(),
            None => build_hostname(&self.service, self.region)
        };

        // Gotta remove and re-add headers since by default they append the value.  If we're following
        // a 307 redirect we end up with Three Stooges in the headers with duplicate values.
        self.remove_header("host");
        self.add_header("host", &hostname);

        if let Some(ref token) = *creds.token() {
            self.remove_header("X-Amz-Security-Token");
            self.add_header("X-Amz-Security-Token", token);
        }

        self.canonical_query_string = build_canonical_query_string(&self.params);

        let date = now_utc();
        self.remove_header("x-amz-date");
        self.add_header("x-amz-date", &date.strftime("%Y%m%dT%H%M%SZ").unwrap().to_string());

        // build the canonical request
        let signed_headers = signed_headers(&self.headers);
        self.canonical_uri = canonical_uri(&self.path);
        let canonical_headers = canonical_headers(&self.headers);

        let canonical_request : String;

        match self.payload {
            None => {
                canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}",
                    &self.method,
                    self.canonical_uri,
                    self.canonical_query_string,
                    canonical_headers,
                    signed_headers,
                    &to_hexdigest(""));
                self.remove_header("x-amz-content-sha256");
                self.add_header("x-amz-content-sha256", &to_hexdigest(""));
            }
            Some(payload) => {
                // This is hashing the payload twice, booo:
                canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}",
                    &self.method,
                    self.canonical_uri,
                    self.canonical_query_string,
                    canonical_headers,
                    signed_headers,
                    &to_hexdigest(payload));
                self.remove_header("x-amz-content-sha256");
                self.add_header("x-amz-content-sha256", &to_hexdigest(payload));
                self.remove_header("content-length");
                self.add_header("content-length", &format!("{}", payload.len()));
            }
        }

        self.remove_header("content-type");
        let ct = match self.content_type {
            Some(ref h) => h.to_string(),
            None => String::from("application/octet-stream")
        };

        self.add_header("content-type", &ct);

        // use the hashed canonical request to build the string to sign
        let hashed_canonical_request = to_hexdigest(&canonical_request);
        let scope = format!("{}/{}/{}/aws4_request", date.strftime("%Y%m%d").unwrap(), self.region, &self.service);
        let string_to_sign = string_to_sign(date, &hashed_canonical_request, &scope);

        // construct the signing key and sign the string with it
        let signing_key = signing_key(creds.aws_secret_access_key(), date, &self.region.to_string(), &self.service);
        let signature = signature(&string_to_sign, signing_key);

        // build the actual auth header
        let auth_header = format!("AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
                   &creds.aws_access_key_id(), scope, signed_headers, signature);
        self.remove_header("authorization");
        self.add_header("authorization", &auth_header);
    }

    fn execute(&mut self, creds: AwsCredentials) -> Response {
        let response = send_request(self);
        debug!("Sent request to AWS");

        if response.status == HTTP_TEMPORARY_REDIRECT {
            debug!("Got a redirect response, resending request.");
            // extract location from response, modify request and re-sign and resend.
            let new_hostname = extract_s3_redirect_location(response).unwrap();
            self.set_hostname(Some(new_hostname.to_string()));

            // This does a lot of appending and not clearing/creation, so we'll have to do that ourselves:
            self.sign(&creds);
            return self.execute(creds);
        }

        response
    }
}

fn hmac<D: Digest>(d: D, key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut hmac = Hmac::new(d, key);
    hmac.input(data);
    hmac.result().code().iter().map(|b| *b).collect::<Vec<u8>>()
}

fn signature(string_to_sign: &str, signing_key: Vec<u8>) -> String {
    hmac(Sha256::new(), &signing_key, string_to_sign.as_bytes()).to_hex().to_string()
}

fn signing_key(secret: &str, date: Tm, region: &str, service: &str) -> Vec<u8> {
    let k_date = hmac(Sha256::new(), format!("AWS4{}", secret).as_bytes(), date.strftime("%Y%m%d").unwrap().to_string().as_bytes());
    let k_region = hmac(Sha256::new(), &k_date, region.as_bytes());
    let k_service = hmac(Sha256::new(), &k_region, service.as_bytes());
    hmac(Sha256::new(), &k_service, b"aws4_request")
}

/// Mark string as AWS4-HMAC-SHA256 hashed
pub fn string_to_sign(date: Tm, hashed_canonical_request: &str, scope: &str) -> String {
    format!("AWS4-HMAC-SHA256\n{}\n{}\n{}",
        date.strftime("%Y%m%dT%H%M%SZ").unwrap(),
        scope,
        hashed_canonical_request)
}

fn signed_headers(headers: &BTreeMap<String, Vec<Vec<u8>>>) -> String {
    let mut signed = String::new();

    for (key,_) in headers.iter() {
        if !signed.is_empty() {
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

fn canonical_values(values: &[Vec<u8>]) -> String {
    let mut st = String::new();
    for v in values {
        let s = str::from_utf8(v).unwrap();
        if !st.is_empty() {
            st.push(',')
        }
        if s.starts_with('\"') {
            st.push_str(s);
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
        _ => encode_uri(path)
    }
}

fn build_canonical_query_string(params: &Params) -> String {
    if params.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    for item in params.iter() {
        if !output.is_empty() {
            output.push_str("&");
        }
        byte_serialize(item.0, &mut output);
        output.push_str("=");
        byte_serialize(item.1, &mut output);
    }

    output
}

#[inline]
fn encode_uri(uri: &str) -> String {
    let mut encoded_uri = String::new();
    for &byte in uri.as_bytes().iter() {
        percent_encode_to(&[byte], DEFAULT_ENCODE_SET, &mut encoded_uri);
    }
    encoded_uri
}

#[inline]
fn byte_serialize(input: &str, output: &mut String) {
    for &byte in input.as_bytes().iter() {
        percent_encode_to(&[byte], FORM_URLENCODED_ENCODE_SET, output)
    }
}

fn to_hexdigest<S: AsRef<[u8]>>(val: S) -> String {
    let mut hasher = Sha256::new();
    hasher.input(val.as_ref());
    hasher.result_str()
}

fn build_hostname(service: &str, region: Region) -> String {
    //iam has only 1 endpoint, other services have region-based endpoints
    match service {
        "iam" => format!("{}.amazonaws.com", service),
        "s3" => {
                match region {
                    Region::UsEast1 => "s3.amazonaws.com".to_string(),
                    _ => format!("s3-{}.amazonaws.com", region),
                }
            }
        _ => format!("{}.{}.amazonaws.com", service, region)
    }
}

/// `extract_s3_redirect_location` takes a Hyper `Response` and attempts to pull out the temporary endpoint.
fn extract_s3_redirect_location(response: Response) -> Result<String, AwsError> {
    // Double checking this feels like belts and suspenders since we're checking the status code
    // before calling this.  Remove this check?

    // Verify it's a 307 temporary redirect
    if response.status != HTTP_TEMPORARY_REDIRECT {
        return Err(AwsError::new("Trying to find temporary location when status is not 307 temp redirect."))
    }

    let mut reader = EventReader::new(response);
    let mut stack = XmlResponseFromAws::new(reader.events().peekable());
    stack.next(); // xml start tag

    // extract and return temporary endpoint location
    extract_s3_temporary_endpoint_from_xml(&mut stack)
}

fn field_in_s3_redirect(name: &str) -> bool {
    if name == "Code" || name == "Message" || name == "Bucket" || name == "RequestId" || name == "HostId" {
        return true;
    }
    false
}

/// `extract_s3_temporary_endpoint_from_xml` takes in XML and tries to find the value of the Endpoint node.
fn extract_s3_temporary_endpoint_from_xml<T: Peek + Next>(stack: &mut T) -> Result<String, AwsError> {
    try!(start_element(&"Error".to_string(), stack));

    // now find Endpoint contents
    // This may infinite loop if there's no endpoint in the response: how can we prevent that?
    loop {
        let current_name = try!(peek_at_name(stack));
        if current_name == "Endpoint" {
            let obj = try!(string_field("Endpoint", stack));
            return Ok(obj);
        }
        if field_in_s3_redirect(&current_name){
            // <foo>bar</foo>:
            stack.next(); // skip the start tag <foo>
            stack.next(); // skip contents bar
            stack.next(); // skip close tag </foo>
            continue;
        }
        break;
    }
    Err(AwsError::new("Couldn't find redirect location for S3 bucket"))
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use xml::reader::*;

    use region::Region;
    use xmlutil::*;

    use super::SignedRequest;
    use super::extract_s3_temporary_endpoint_from_xml;

    use super::super::ProfileProvider;
    use super::super::credential::ProvideAwsCredentials;

    #[test]
    fn get_hostname_none_present() {
        let request = SignedRequest::new("POST", "sqs", Region::UsEast1, "/");
        assert_eq!("sqs.us-east-1.amazonaws.com", request.hostname());
    }

    #[test]
    fn get_hostname_happy_path() {
        let mut request = SignedRequest::new("POST", "sqs", Region::UsEast1, "/");
        request.set_hostname(Some("test-hostname".to_string()));
        assert_eq!("test-hostname", request.hostname());
    }

    #[test]
    fn get_redirect_location_from_s3() {
        let file = File::open("tests/sample-data/s3_temp_redirect.xml").unwrap();
        let file = BufReader::new(file);
        let mut my_parser  = EventReader::new(file);
        let my_stack = my_parser.events().peekable();
        let mut reader = XmlResponseFromFile::new(my_stack);
        reader.next(); // xml start node
        let result = extract_s3_temporary_endpoint_from_xml(&mut reader);

        match result {
            Err(_) => panic!("Couldn't parse s3_temp_redirect.xml"),
            Ok(location) => {
                assert_eq!(location, "rusoto1441045966.s3-us-west-1.amazonaws.com");
            }
        }
    }

    #[test]
    fn path_percent_encoded() {
        let provider = ProfileProvider::with_configuration(
            "tests/sample-data/multiple_profile_credentials",
            "foo",
        );
        let mut request = SignedRequest::new("GET", "s3", Region::UsEast1, "/path with spaces");
        request.sign(provider.credentials().as_ref().unwrap());
        assert_eq!("/path%20with%20spaces", request.canonical_uri());
    }
}
