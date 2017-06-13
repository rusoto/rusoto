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

use ring::{digest, hmac};
use rustc_serialize::hex::ToHex;
use time::Tm;
use time::now_utc;
use url::percent_encoding::{utf8_percent_encode, EncodeSet};

use param::Params;
use region::Region;
use credential::AwsCredentials;

/// A data structure for all the elements of an HTTP request that are involved in
/// the Amazon Signature Version 4 signing process
#[derive(Debug)]
pub struct SignedRequest {
    pub method: String,
    pub service: String,
    pub region: Region,
    pub path: String,
    pub headers: BTreeMap<String, Vec<Vec<u8>>>,
    pub params: Params,
    pub scheme: Option<String>,
    pub hostname: Option<String>,
    pub payload: Option<Vec<u8>>,
    pub canonical_query_string: String,
    pub canonical_uri: String,
}

impl SignedRequest {
    /// Default constructor
    pub fn new(method: &str, service: &str, region: Region, path: &str) -> SignedRequest {
        SignedRequest {
            method: method.to_string(),
            service: service.to_string(),
            region: region,
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
    pub fn set_content_type(&mut self, content_type: String) {
        self.add_header("content-type", &content_type);
    }

    pub fn set_hostname(&mut self, hostname: Option<String>) {
        self.hostname = hostname;
    }

    pub fn set_endpoint_prefix(&mut self, endpoint_prefix: String) {
        self.hostname = Some(build_hostname(&endpoint_prefix, self.region));
    }

    pub fn set_payload(&mut self, payload: Option<Vec<u8>>) {
        self.payload = payload;
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn canonical_path(&self) -> String {
        canonical_uri(&self.path)
    }

    pub fn canonical_uri(&self) -> &str {
        &self.canonical_uri
    }

    pub fn canonical_query_string(&self) -> &str {
        &self.canonical_query_string
    }

    pub fn headers(&self) -> &BTreeMap<String, Vec<Vec<u8>>> {
        &self.headers
    }

    pub fn scheme(&self) -> String {
        match self.scheme {
            Some(ref p) => p.to_string(),
            None => build_scheme(self.region)
        }
    }

    pub fn hostname(&self) -> String {
        match self.hostname {
            Some(ref h) => h.to_string(),
            None => build_hostname(&self.service, self.region),
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

    pub fn add_param<S>(&mut self, key: S, value: S)
        where S: Into<String> {
        self.params.insert(key.into(), Some(value.into()));
    }

    pub fn set_params(&mut self, params: Params) {
        self.params = params;
    }

    pub fn sign(&mut self, creds: &AwsCredentials) {
        debug!("Creating request to send to AWS.");
        let hostname = match self.hostname {
            Some(ref h) => h.to_string(),
            None => build_hostname(&self.service, self.region),
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
        let canonical_headers = canonical_headers(&self.headers);

        let canonical_request: String;

        if self.payload.is_none() {
            canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}",
                                        &self.method,
                                        self.canonical_uri,
                                        self.canonical_query_string,
                                        canonical_headers,
                                        signed_headers,
                                        &to_hexdigest(""));
            self.remove_header("x-amz-content-sha256");
            self.add_header("x-amz-content-sha256", &to_hexdigest(""));
        } else {
            let (digest, len) = digest_payload(&self.payload);
            canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}",
                                        &self.method,
                                        self.canonical_uri,
                                        self.canonical_query_string,
                                        canonical_headers,
                                        signed_headers,
                                        &digest);
            self.remove_header("x-amz-content-sha256");
            self.add_header("x-amz-content-sha256", &digest);
            self.remove_header("content-length");
            self.add_header("content-length", &format!("{}", len));
        }

        // use the hashed canonical request to build the string to sign
        let hashed_canonical_request = to_hexdigest(&canonical_request);
        let scope = format!("{}/{}/{}/aws4_request",
                            date.strftime("%Y%m%d").unwrap(),
                            self.region,
                            &self.service);
        let string_to_sign = string_to_sign(date, &hashed_canonical_request, &scope);

        // construct the signing key and sign the string with it
        let signing_key = signing_key(creds.aws_secret_access_key(),
                                      date,
                                      &self.region.to_string(),
                                      &self.service);
        let signature = signature(&string_to_sign, &signing_key);

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

fn digest_payload(payload: &Option<Vec<u8>>) -> (String, usize) {
    let payload = payload.as_ref().unwrap();
    let digest = to_hexdigest(payload);
    let len = payload.len();
    (digest, len)
}

fn signature(string_to_sign: &str, signing_key: &hmac::SigningKey) -> String {
    hmac::sign(signing_key, string_to_sign.as_bytes()).as_ref().to_hex().to_string()
}

fn signing_key(secret: &str, date: Tm, region: &str, service: &str) -> hmac::SigningKey {
    let date_key = hmac::SigningKey::new(&digest::SHA256, format!("AWS4{}", secret).as_bytes());
    let date_hmac = hmac::sign(&date_key,
                               date.strftime("%Y%m%d").unwrap().to_string().as_bytes());

    let region_key = hmac::SigningKey::new(&digest::SHA256, date_hmac.as_ref());
    let region_hmac = hmac::sign(&region_key, region.as_bytes());

    let service_key = hmac::SigningKey::new(&digest::SHA256, region_hmac.as_ref());
    let service_hmac = hmac::sign(&service_key, service.as_bytes());

    let signing_key = hmac::SigningKey::new(&digest::SHA256, service_hmac.as_ref());
    let signing_hmac = hmac::sign(&signing_key, b"aws4_request");

    hmac::SigningKey::new(&digest::SHA256, signing_hmac.as_ref())
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

    for (key, _) in headers.iter() {
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
        canonical.push_str(format!("{}:{}\n",
                                   item.0.to_ascii_lowercase(),
                                   canonical_values(item.1))
            .as_ref());
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
        _ => encode_uri_path(path),
    }
}

fn build_canonical_query_string(params: &Params) -> String {
    if params.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    for (key, val) in params.iter() {
        if !output.is_empty() {
            output.push_str("&");
        }

        output.push_str(&encode_uri_strict(key));
        output.push_str("=");

        if val.is_some() {
            output.push_str(&encode_uri_strict(val.as_ref().unwrap()));
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
    utf8_percent_encode(uri, StrictPathEncodeSet).collect::<String>()
}

#[inline]
fn encode_uri_strict(uri: &str) -> String {
    utf8_percent_encode(uri, StrictEncodeSet).collect::<String>()
}

fn to_hexdigest<T: AsRef<[u8]>>(t: T) -> String {
    let h = digest::digest(&digest::SHA256, t.as_ref());
    h.as_ref().to_hex().to_string()
}

fn build_scheme(region: Region) -> String {
    match region {
        Region::Local(_) => "http".to_owned(),
        _                => "https".to_owned()
    }
}

fn build_hostname(service: &str, region: Region) -> String {
    //iam has only 1 endpoint, other services have region-based endpoints
    match service {
        "iam" => {
            match region {
                Region::Local(hostname) => hostname.to_owned(),
                Region::CnNorth1 => format!("{}.{}.amazonaws.com.cn", service, region),
                _ => format!("{}.amazonaws.com", service),
            }
        }
        "s3" => {
            match region {
                Region::Local(hostname) => hostname.to_owned(),
                Region::UsEast1 => "s3.amazonaws.com".to_string(),
                Region::CnNorth1 => format!("s3.{}.amazonaws.com.cn", region),
                _ => format!("s3-{}.amazonaws.com", region),
            }
        }
        _ => {
            match region {
                Region::Local(hostname) => hostname.to_owned(),
                Region::CnNorth1 => format!("{}.{}.amazonaws.com.cn", service, region),
                _ => format!("{}.{}.amazonaws.com", service, region),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use credential::{ProvideAwsCredentials, ProfileProvider};
    use Region;
    use ::param::Params;

    use super::{SignedRequest,build_canonical_query_string};

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
    fn path_percent_encoded() {
        let provider = ProfileProvider::with_configuration("test_resources/multiple_profile_credentials",
                                                           "foo");
        let mut request = SignedRequest::new("GET",
                                             "s3",
                                             Region::UsEast1,
                                             "/path with spaces: the sequel");
        request.sign(provider.credentials().as_ref().unwrap());
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
            let expected = format!("k=%{:02X}", code);
            assert_eq!(expected, enc);
        }
    }
    #[test]
    fn query_percent_encoded() {
        let mut request = SignedRequest::new("GET",
                                             "s3",
                                             Region::UsEast1,
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
}
