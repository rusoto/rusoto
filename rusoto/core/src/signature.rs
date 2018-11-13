//! AWS API request signatures.
//!
//! Follows [AWS Signature 4](http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html)
//! algorithm.
//!
//! If needed, the request will be re-issued to a temporary redirect endpoint.  This can happen with
//! newly created S3 buckets not in us-standard/us-east-1.

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fmt;
use std::io;
use std::str;
use std::time::Duration;

use futures::Stream;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use time::Tm;
use time::now_utc;
use url::percent_encoding::{utf8_percent_encode, percent_decode, EncodeSet};
use hex;
use md5;
use base64;

use param::{Params, ServiceParams};
use region::Region;
use credential::AwsCredentials;

/// Possible payloads included in a `SignedRequest`.
pub enum SignedRequestPayload {
    /// Transfer payload in a single chunk
    Buffer(Vec<u8>),
    /// Transfer payload in multiple chunks
    Stream(Option<usize>, Box<Stream<Item=Vec<u8>, Error=io::Error> + Send>)
}

impl fmt::Debug for SignedRequestPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SignedRequestPayload::Buffer(ref buf) =>
                write!(f, "SignedRequestPayload::Buffer(len = {})", buf.len()),
            &SignedRequestPayload::Stream(len, _) =>
                write!(f, "SignedRequestPayload::Stream(len_hint = {:?})", len),
        }
    }
}

/// A data structure for all the elements of an HTTP request that are involved in
/// the Amazon Signature Version 4 signing process
#[derive(Debug)]
pub struct SignedRequest {
    /// The HTTP Method
    pub method: String,
    /// The AWS Service
    pub service: String,
    /// The AWS Region
    pub region: Region,
    /// The HTTP request path
    pub path: String,
    /// The HTTP Request Headers
    pub headers: BTreeMap<String, Vec<Vec<u8>>>,
    /// The HTTP request paramaters
    pub params: Params,
    /// The HTTP/HTTPS protocol
    pub scheme: Option<String>,
    /// The AWS hostname
    pub hostname: Option<String>,
    /// The HTTP Content
    pub payload: Option<SignedRequestPayload>,
    /// The Standardised query string
    pub canonical_query_string: String,
    /// The Standardised URI
    pub canonical_uri: String,
}

impl SignedRequest {
    /// Default constructor
    pub fn new(method: &str, service: &str, region: &Region, path: &str) -> SignedRequest {
        SignedRequest {
            method: method.to_string(),
            service: service.to_string(),
            region: region.clone(),
            path: path.to_string(),
            headers: BTreeMap::new(),
            params: Params::new(),
            scheme: None,
            hostname: None,
            payload: None,
            canonical_query_string: String::new(),
            canonical_uri: String::new(),
        }
    }

    /// Sets the value of the "content-type" header.
    pub fn set_content_type(&mut self, content_type: String) {
        self.add_header("content-type", &content_type);
    }

    /// Sets the target hostname
    pub fn set_hostname(&mut self, hostname: Option<String>) {
        self.hostname = hostname;
    }

    /// Sets the target hostname using the current service type and region
    ///
    /// See the implementation of build_hostname to see how this is done
    pub fn set_endpoint_prefix(&mut self, endpoint_prefix: String) {
        self.hostname = Some(build_hostname(&endpoint_prefix, &self.region));
    }

    /// Sets the new body (payload)
    pub fn set_payload(&mut self, payload: Option<Vec<u8>>) {
        self.payload = payload.map(SignedRequestPayload::Buffer);
    }

    /// Sets the new body (payload) as a stream
    pub fn set_payload_stream(&mut self, len_hint: Option<usize>, stream: Box<Stream<Item=Vec<u8>, Error=io::Error> + Send>) {
        self.payload = Some(SignedRequestPayload::Stream(len_hint, stream));
    }

    /// Computes and sets the Content-MD5 header based on the current payload.
    ///
    /// Has no effect if the payload is not set, or is not a buffer.
    pub fn set_content_md5_header(&mut self) {
        let digest;
        if let Some(SignedRequestPayload::Buffer(ref payload)) = self.payload {
            digest = Some(md5::compute(payload));
        } else {
            digest = None;
        }
        if let Some(digest) = digest {
            // need to deref digest and then pass that reference:
            self.add_header("Content-MD5", &base64::encode(&(*digest)));
        }
    }

    /// Returns the current HTTP method
    pub fn method(&self) -> &str {
        &self.method
    }

    /// Returns the current path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Invokes `canonical_uri(path)` to return a canonical path
    pub fn canonical_path(&self) -> String {
        canonical_uri(&self.path)
    }

    /// Returns the current canonical URI
    pub fn canonical_uri(&self) -> &str {
        &self.canonical_uri
    }

    /// Returns the current query string
    ///
    /// Converts a paramater such as "example param": "examplekey" into "&example+param=examplekey"
    pub fn canonical_query_string(&self) -> &str {
        &self.canonical_query_string
    }

    /// Returns the current headers
    pub fn headers(&self) -> &BTreeMap<String, Vec<Vec<u8>>> {
        &self.headers
    }

    /// Returns the current http scheme (https or http)
    pub fn scheme(&self) -> String {
        match self.scheme {
            Some(ref p) => p.to_string(),
            None => {
                match self.region {
                    Region::Custom { ref endpoint, .. } => {
                        if endpoint.starts_with("http://") {
                            "http".to_owned()
                        } else {
                            "https".to_owned()
                        }
                    },
                    _ => "https".to_owned()
                }
            }
        }
    }

    /// Converts hostname to String if it exists, else it invokes build_hostname()
    pub fn hostname(&self) -> String {
        // hostname may be already set by an endpoint prefix
        match self.hostname {
            Some(ref h) => h.to_string(),
            None => build_hostname(&self.service, &self.region),
        }
    }

    /// If the key exists in headers, set it to blank/unoccupied:
    pub fn remove_header(&mut self, key: &str) {
        let key_lower = key.to_ascii_lowercase().to_string();
        self.headers.remove(&key_lower);
    }

    /// Add a value to the array of headers for the specified key.
    /// Headers are kept sorted by key name for use at signing (BTreeMap)
    pub fn add_header<K: ToString>(&mut self, key: K, value: &str) {
        let key_lower = key.to_string().to_ascii_lowercase();
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

    /// Adds parameter to the HTTP Request
    pub fn add_param<S>(&mut self, key: S, value: S)
        where S: Into<String> {
        self.params.insert(key.into(), Some(value.into()));
    }

    /// Sets paramaters with a given variable of `Params` type
    pub fn set_params(&mut self, params: Params) {
        self.params = params;
    }


    /// http://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html
    pub fn generate_presigned_url(&mut self, creds: &AwsCredentials, expires_in: &Duration) -> String {
        debug!("Presigning request URL");

        self.sign(creds);
        let hostname = match self.hostname {
            Some(ref h) => h.to_string(),
            None => build_hostname(&self.service, &self.region),
        };

        let current_time = now_utc();
        let current_time_fmted = current_time.strftime("%Y%m%dT%H%M%SZ").unwrap();
        let current_time_fmted = format!("{}", &current_time_fmted);
        let current_date = current_time.strftime("%Y%m%d").unwrap();

        self.remove_header("X-Amz-Content-Sha256");

        self.remove_header("X-Amz-Date");

        self.remove_header("Content-Type");

        if let Some(ref token) = *creds.token() {
            self.remove_header("x-amz-security-token");
            self.params.put("x-amz-security-token", encode_uri_strict(token));
        }

        self.remove_header("X-Amz-Algorithm");
        self.params.put("X-Amz-Algorithm", "AWS4-HMAC-SHA256");

        self.remove_header("X-Amz-Credential");
        self.params.put("X-Amz-Credential", format!("{}/{}/{}/{}/aws4_request", &creds.aws_access_key_id(), &current_date, self.region.name(), self.service));

        self.remove_header("X-Amz-Expires");
        let expiration_time = format!("{}", expires_in.as_secs());
        self.params.put("X-Amz-Expires", expiration_time);

        self.canonical_uri = canonical_uri(&self.path);
        let canonical_headers = canonical_headers(&self.headers);

        let signed_headers = signed_headers(&self.headers);
        self.params.put("X-Amz-SignedHeaders", &signed_headers);

        self.params.put("X-Amz-Date", current_time_fmted);

        self.canonical_query_string = build_canonical_query_string(&self.params);

        debug!("canonical_uri: {:?}", self.canonical_uri);
        debug!("canonical_headers: {:?}", canonical_headers);
        debug!("signed_headers: {:?}", signed_headers);
        debug!("canonical_query_string: {:?}", self.canonical_query_string);

        let canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}",
                                        &self.method,
                                        self.canonical_uri,
                                        self.canonical_query_string,
                                        canonical_headers,
                                        &signed_headers,
                                        "UNSIGNED-PAYLOAD");

        debug!("canonical_request: {:?}", canonical_request);

        // use the hashed canonical request to build the string to sign
        let hashed_canonical_request = to_hexdigest(&canonical_request);

        debug!("hashed_canonical_request: {:?}", hashed_canonical_request);

        let scope = format!("{}/{}/{}/aws4_request",
                            current_date,
                            self.region.name(),
                            &self.service);

        debug!("scope: {}", scope);

        let string_to_sign = string_to_sign(current_time, &hashed_canonical_request, &scope);

        debug!("string_to_sign: {}", string_to_sign);

        let signature = sign_string(&string_to_sign,
                                    creds.aws_secret_access_key(),
                                    current_time,
                                    &self.region.name(),
                                    &self.service);
        self.params.put("X-Amz-Signature", signature);

        format!("{}://{}{}?{}", self.scheme(), hostname, self.canonical_uri, build_canonical_query_string(&self.params))
    }

    /// Signs the request using Amazon Signature version 4 to verify identity.
    /// Authorization header uses AWS4-HMAC-SHA256 for signing.
    pub fn sign(&mut self, creds: &AwsCredentials) {
        self.sign_with_plus(creds, false)
    }

    /// Signs the request using Amazon Signature version 4 to verify identity.
    /// Authorization header uses AWS4-HMAC-SHA256 for signing.
    pub fn sign_with_plus(&mut self, creds: &AwsCredentials, should_treat_plus_literally: bool) {
        debug!("Creating request to send to AWS.");
        let hostname = match self.hostname {
            Some(ref h) => h.to_string(),
            None => build_hostname(&self.service, &self.region),
        };

        // Gotta remove and re-add headers since by default they append the value.  If we're following
        // a 307 redirect we end up with Three Stooges in the headers with duplicate values.
        self.remove_header("host");
        self.add_header("host", &hostname);

        if let Some(ref token) = *creds.token() {
            self.remove_header("X-Amz-Security-Token");
            self.add_header("X-Amz-Security-Token", token);
        }

        self.canonical_query_string = build_canonical_query_string_with_plus(&self.params, should_treat_plus_literally);

        let date = now_utc();
        self.remove_header("x-amz-date");
        self.add_header("x-amz-date",
                        &date.strftime("%Y%m%dT%H%M%SZ").unwrap().to_string());

        // if there's no content-type header set, set it to the default value
        if let Entry::Vacant(entry) = self.headers.entry("content-type".to_owned()) {
            let mut values = Vec::new();
            values.push(b"application/octet-stream".to_vec());
            entry.insert(values);
        }

        // build the canonical request
        let signed_headers = signed_headers(&self.headers);
        self.canonical_uri = canonical_uri(&self.path);
        // Normalize URI paths according to RFC 3986. Remove redundant and relative path components. Each path segment must be URI-encoded twice (except for Amazon S3 which only gets URI-encoded once).
        // see https://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html
        let canonical_uri = if &self.service != "s3" {
            utf8_percent_encode(&self.canonical_uri, StrictPathEncodeSet).collect::<String>()
        }else{
            self.canonical_uri.clone()
        };
        let canonical_headers = canonical_headers(&self.headers);

        let canonical_request: String;

        let (digest, len) = match self.payload {
            None => {
                canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}",
                                            &self.method,
                                            canonical_uri,
                                            self.canonical_query_string,
                                            canonical_headers,
                                            signed_headers,
                                            &to_hexdigest(""));
                (Some(to_hexdigest("")), Some(0))
            }
            Some(SignedRequestPayload::Buffer(ref payload)) => {
                let (digest, len) = digest_payload(&payload);
                canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}",
                                            &self.method,
                                            canonical_uri,
                                            self.canonical_query_string,
                                            canonical_headers,
                                            signed_headers,
                                            &digest);
                (Some(digest), Some(len))
            }
            Some(SignedRequestPayload::Stream(len, _)) => {
                canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}",
                                            &self.method,
                                            canonical_uri,
                                            self.canonical_query_string,
                                            canonical_headers,
                                            signed_headers,
                                            "UNSIGNED-PAYLOAD");
                (Some("UNSIGNED-PAYLOAD".to_owned()), len)
            }
        };

        if let Some(digest) = digest {
            self.remove_header("x-amz-content-sha256");
            self.add_header("x-amz-content-sha256", &digest);
        }

        if let Some(len) = len {
            self.remove_header("content-length");
            self.add_header("content-length", &format!("{}", len));
        }

        // use the hashed canonical request to build the string to sign
        let hashed_canonical_request = to_hexdigest(&canonical_request);
        let scope = format!("{}/{}/{}/aws4_request",
                            date.strftime("%Y%m%d").unwrap(),
                            self.region.name(),
                            &self.service);
        let string_to_sign = string_to_sign(date, &hashed_canonical_request, &scope);

        // sign the string
        let signature = sign_string(&string_to_sign,
                                    creds.aws_secret_access_key(),
                                    date,
                                    &self.region.name(),
                                    &self.service);

        // build the actual auth header
        let auth_header = format!("AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
                                  &creds.aws_access_key_id(),
                                  scope,
                                  signed_headers,
                                  signature);
        self.remove_header("authorization");
        self.add_header("authorization", &auth_header);
    }
}

/// Convert payload from Char array to useable <payload, len> format.
fn digest_payload(payload: &[u8]) -> (String, usize) {
    let digest = to_hexdigest(payload);
    let len = payload.len();
    (digest, len)
}

#[inline]
fn hmac(secret: &[u8], message: &[u8]) -> Hmac<Sha256> {
    let mut hmac = Hmac::<Sha256>::new(secret).expect("failed to create hmac");
    hmac.input(message);
    hmac
}

/// Takes a message and signs it using AWS secret, time, region keys and service keys.
fn sign_string(string_to_sign: &str, secret: &str, date: Tm, region: &str, service: &str) -> String {
    let date_str = date.strftime("%Y%m%d").unwrap().to_string();
    let date_hmac = hmac(format!("AWS4{}", secret).as_bytes(), date_str.as_bytes())
        .result().code();
    let region_hmac = hmac(date_hmac.as_ref(), region.as_bytes())
        .result().code();
    let service_hmac = hmac(region_hmac.as_ref(), service.as_bytes())
        .result().code();
    let signing_hmac = hmac(service_hmac.as_ref(), b"aws4_request")
        .result().code();
    hex::encode(hmac(signing_hmac.as_ref(), string_to_sign.as_bytes())
        .result().code().as_ref())
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
    headers.iter()
        .filter(|&(ref key, _)| !skipped_headers(&key))
        .for_each(|(key, _)| {
            if !signed.is_empty() {
                signed.push(';');
            }
            signed.push_str(key);
    });
    signed
}

/// Canonicalizes headers into the AWS Canonical Form.
///
/// Read more about it: [HERE](http://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html)
fn canonical_headers(headers: &BTreeMap<String, Vec<Vec<u8>>>) -> String {
    let mut canonical = String::new();

    for (key, value) in headers.iter() {
        if skipped_headers(key) {
            continue;
        }
        canonical.push_str(format!("{}:{}\n",
                                   key,
                                   canonical_values(value))
            .as_ref());
    }
    canonical
}

/// Canonicalizes values into the AWS Canonical Form.
///
/// Read more about it: [HERE](http://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html)
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

/// Returns standardised URI
fn canonical_uri(path: &str) -> String {
    match path {
        "" => "/".to_string(),
        _ => encode_uri_path(path),
    }
}

/// Canonicalizes query while iterating through the given paramaters
///
/// Read more about it: [HERE](http://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html#query-string-auth-v4-signing)
fn build_canonical_query_string(params: &Params) -> String {
    build_canonical_query_string_with_plus(params, false)
}

/// Canonicalizes query while iterating through the given parameters.
///
/// Read more about it: [HERE](http://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html#query-string-auth-v4-signing)
fn build_canonical_query_string_with_plus(params: &Params, should_treat_plus_literally: bool) -> String {
    if params.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    for (key, val) in params.iter() {
        if !output.is_empty() {
            output.push_str("&");
        }
        if should_treat_plus_literally {
            output.push_str(&encode_uri_strict(&key));
        } else {
            output.push_str(&encode_uri_strict(&key.replace("+", " ")));
        }
        output.push_str("=");

        if let &Some(ref unwrapped_val) = val {
            if should_treat_plus_literally {
                output.push_str(&encode_uri_strict(&unwrapped_val));
            } else {
                output.push_str(&encode_uri_strict(&unwrapped_val.replace("+", " ")));
            }
        }
    }

    output
}

// http://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html
//
// Do not URI-encode any of the unreserved characters that RFC 3986 defines:
// A-Z, a-z, 0-9, hyphen ( - ), underscore ( _ ), period ( . ), and tilde ( ~ ).
//
// Percent-encode all other characters with %XY, where X and Y are hexadecimal
// characters (0-9 and uppercase A-F). For example, the space character must be
// encoded as %20 (not using '+', as some encoding schemes do) and extended UTF-8
// characters must be in the form %XY%ZA%BC
#[derive(Clone)]
/// This struct is used to maintain the strict URI encoding standard as proposed by RFC 3986
pub struct StrictEncodeSet;

impl EncodeSet for StrictEncodeSet {
    #[inline]
    fn contains(&self, byte: u8) -> bool {
        let upper = byte >= 0x41 && byte <= 0x5a;
        let lower = byte >= 0x61 && byte <= 0x7a;
        let numeric = byte >= 0x30 && byte <= 0x39;
        let hyphen = byte == 0x2d;
        let underscore = byte == 0x5f;
        let tilde = byte == 0x7e;
        let period = byte == 0x2e;
        !(upper || lower || numeric || hyphen || underscore || tilde || period)
    }
}

#[derive(Clone)]
/// This struct is used to maintain the URI path encoding
pub struct StrictPathEncodeSet;

impl EncodeSet for StrictPathEncodeSet {
    #[inline]
    fn contains(&self, byte: u8) -> bool {
        let slash = byte == b'/';
        !slash && StrictEncodeSet.contains(byte)
    }
}

#[inline]
#[doc(hidden)]
pub fn encode_uri_path(uri: &str) -> String {
    utf8_percent_encode(&decode_uri(uri), StrictPathEncodeSet).collect::<String>()
}

#[inline]
fn encode_uri_strict(uri: &str) -> String {
    utf8_percent_encode(&decode_uri(uri), StrictEncodeSet).collect::<String>()
}

#[inline]
fn decode_uri(uri: &str) -> String {
    let decoder = percent_decode(uri.as_bytes());
    if let Ok(decoded) = decoder.decode_utf8() {
        decoded.to_string()
    } else {
        uri.to_owned()
    }
}

fn to_hexdigest<T: AsRef<[u8]>>(t: T) -> String {
    let h = Sha256::digest(t.as_ref());
    hex::encode(h.as_ref())
}

fn remove_scheme_from_custom_hostname(hostname: &str) -> String {
    if hostname.starts_with("https://") {
        hostname[8..].to_owned()
    } else if hostname.starts_with("http://") {
        hostname[7..].to_owned()
    } else {
        hostname.to_owned()
    }
}

/// Takes a `Region` enum and a service and formas a vaild DNS name.
/// E.g. `Region::ApNortheast1` and `s3` produces `s3.ap-northeast-1.amazonaws.com.cn`
fn build_hostname(service: &str, region: &Region) -> String {
    //iam & cloudfront have only 1 endpoint, other services have region-based endpoints
    match service {
        "iam" => {
            match *region {
                Region::Custom { ref endpoint, .. } => {
                    remove_scheme_from_custom_hostname(endpoint)
                }
                Region::CnNorth1 | Region::CnNorthwest1 => format!("{}.{}.amazonaws.com.cn", service, region.name()),
                _ => format!("{}.amazonaws.com", service),
            }
        }
        "cloudfront" => {
            match *region {
                Region::Custom { ref endpoint, .. } => {
                    remove_scheme_from_custom_hostname(endpoint).to_owned()
                }
                _ => format!("{}.amazonaws.com", service),
            }
        }
        "s3" => {
            match *region {
                Region::Custom { ref endpoint, .. } => {
                    remove_scheme_from_custom_hostname(endpoint)
                }
                Region::UsEast1 => "s3.amazonaws.com".to_string(),
                Region::CnNorth1 | Region::CnNorthwest1 => format!("s3.{}.amazonaws.com.cn", region.name()),
                _ => format!("s3-{}.amazonaws.com", region.name()),
            }
        }
        "route53" => {
            match *region {
                Region::Custom { ref endpoint, .. } => {
                    remove_scheme_from_custom_hostname(endpoint)
                }
                _ => "route53.amazonaws.com".to_owned(),
            }
        }
        "sdb" => {
            match *region {
                Region::Custom { ref endpoint, .. } => {
                    remove_scheme_from_custom_hostname(endpoint).to_owned()
                }
                Region::UsEast1 => "sdb.amazonaws.com".to_string(),
                _ => format!("sdb.{}.amazonaws.com", region.name()),
            }
        }
        _ => {
            match *region {
                Region::Custom { ref endpoint, .. } => {
                    remove_scheme_from_custom_hostname(endpoint)
                }
                Region::CnNorth1 | Region::CnNorthwest1 => format!("{}.{}.amazonaws.com.cn", service, region.name()),
                _ => format!("{}.{}.amazonaws.com", service, region.name()),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use futures::Future;
    use std::collections::BTreeMap;
    use time::empty_tm;

    use credential::{ProvideAwsCredentials, ProfileProvider};
    use Region;
    use ::param::Params;

    use super::{SignedRequest,build_canonical_query_string};

    #[test]
    fn get_hostname_none_present() {
        let request = SignedRequest::new("POST", "sqs", &Region::UsEast1, "/");
        assert_eq!("sqs.us-east-1.amazonaws.com", request.hostname());
    }

    #[test]
    fn path_percent_encoded() {
        let provider = ProfileProvider::with_configuration("test_resources/multiple_profile_credentials",
                                                           "foo");
        let mut request = SignedRequest::new("GET",
                                             "s3",
                                             &Region::UsEast1,
                                             "/path with spaces: the sequel");
        request.sign(provider.credentials().wait().as_ref().unwrap());
        assert_eq!("/path%20with%20spaces%3A%20the%20sequel",
                   request.canonical_uri());
    }
    #[test]
    fn query_encoding_escaped_chars() {
        query_encoding_escaped_chars_range(0u8, 45u8); // \0 to '-'
        query_encoding_escaped_chars_range(47u8, 48u8); // '/' to '0'
        query_encoding_escaped_chars_range(58u8, 65u8); // '0' to 'A'
        query_encoding_escaped_chars_range(91u8, 95u8); // '[' to '_'
        query_encoding_escaped_chars_range(96u8, 97u8); // '`' to 'a'
        query_encoding_escaped_chars_range(123u8, 126u8); // '{' to '~'
        query_encoding_escaped_chars_range(127u8, 128u8); // DEL
    }
    fn query_encoding_escaped_chars_range(start: u8, end: u8) {
        let mut params = Params::new();
        for code in start..end {
            params.insert("k".to_owned(), Some((code as char).to_string()));
            let enc = build_canonical_query_string(&params);
            let expected = if (code as char) == '+' {
                "k=%20".to_owned()
            } else {
                format!("k=%{:02X}", code)
            };
            assert_eq!(expected, enc);
        }
    }
    #[test]
    fn query_string_encoding_outliers() {
        let mut request = SignedRequest::new("GET",
            "s3", &Region::UsEast1, "/pathwith%20already%20existing%20encoding and some not encoded values");
        request.add_param("arg1%7B", "arg1%7B");
        request.add_param("arg2%7B+%2B", "+%2B");
        assert_eq!(super::build_canonical_query_string(&request.params),
            "arg1%7B=arg1%7B&arg2%7B%20%2B=%20%2B");
        assert_eq!(super::canonical_uri(&request.path),
            "/pathwith%20already%20existing%20encoding%20and%20some%20not%20encoded%20values");
    }
    #[test]
    fn query_percent_encoded() {
        let mut request = SignedRequest::new("GET",
                                             "s3",
                                             &Region::UsEast1,
                                             "/path with spaces: the sequel++");
        request.add_param("key:with@funny&characters",
                          "value with/funny%characters/Рускии");
        let canonical_query_string = super::build_canonical_query_string(&request.params);
        assert_eq!("key%3Awith%40funny%26characters=value%20with%2Ffunny%25characters%2F%D0%A0%D1%83%D1%81%D0%BA%D0%B8%D0%B8",
                   canonical_query_string);
        let canonical_uri_string = super::canonical_uri(&request.path);
        assert_eq!("/path%20with%20spaces%3A%20the%20sequel%2B%2B",
                   canonical_uri_string);
    }

    #[test]
    fn signature_generation() {
        let signature_foo = super::sign_string("foo", "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY", empty_tm(), "us-west-1", "s3");
        assert_eq!(signature_foo, "29673d1d856a7684ff6f0f53c542bae0bfbb1e564f531aff7568be9fd206383b".to_string());
        let signature_bar = super::sign_string("bar", "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY", empty_tm(), "us-west-1", "s3");
        assert_eq!(signature_bar, "2ba6879cd9e769d73df721dc90aafdaa843005d23f5b6c91d0744f804962e44f".to_string());
    }

    #[test]
    fn signed_headers_unsigned_first() {
        let mut headers = BTreeMap::new();

        // This header is excluded from signing
        headers.insert("content-length".to_owned(), vec![vec![]]);

        headers.insert("content-type".to_owned(), vec![vec![]]);
        headers.insert("x-amz-date".to_owned(), vec![vec![]]);
        assert_eq!(super::signed_headers(&headers), "content-type;x-amz-date");
    }

    #[test]
    fn signed_headers_unsigned_in_center() {
        let mut headers = BTreeMap::new();
        headers.insert("cache-control".to_owned(), vec![vec![]]);

        // This header is excluded from signing
        headers.insert("content-length".to_owned(), vec![vec![]]);

        headers.insert("content-type".to_owned(), vec![vec![]]);
        headers.insert("host".to_owned(), vec![vec![]]);
        headers.insert("x-amz-date".to_owned(), vec![vec![]]);

        assert_eq!(super::signed_headers(&headers), "cache-control;content-type;host;x-amz-date");
    }

    #[test]
    fn signed_headers_unsigned_last() {
        let mut headers = BTreeMap::new();
        headers.insert("cache-control".to_owned(), vec![vec![]]);

        // This header is excluded from signing
        headers.insert("content-length".to_owned(), vec![vec![]]);

        assert_eq!(super::signed_headers(&headers), "cache-control");
    }

    #[test]
    fn remove_scheme_from_custom_hostname() {
        assert_eq!(
            super::remove_scheme_from_custom_hostname("hostname.with.no.scheme"),
            "hostname.with.no.scheme"
        );
        assert_eq!(
            super::remove_scheme_from_custom_hostname("http://hostname.with.scheme"),
            "hostname.with.scheme"
        );
        assert_eq!(
            super::remove_scheme_from_custom_hostname("https://hostname.with.scheme"),
            "hostname.with.scheme"
        );
    }
}
