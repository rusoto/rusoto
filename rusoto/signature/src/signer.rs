use std::collections::BTreeSet;
use std::error::Error;

use log::trace;

use hyper::{
    header::{self, HeaderMap, HeaderValue, InvalidHeaderValue},
    Method, Uri,
};
use rusoto_credential::AwsCredentials;

use percent_encoding::utf8_percent_encode;
use time::OffsetDateTime;

use crate::signature::{
    encode_uri_strict, sign_string, string_to_sign, to_hexdigest, EMPTY_SHA256_HASH,
    STRICT_PATH_ENCODE_SET,
};
use bytes::Bytes;

pub const ISO_8601_DATETIME: &'static str = "%Y%m%dT%H%M%SZ";
pub const ISO_8601_DATE: &'static str = "%Y%m%d";

#[derive(Debug)]
pub struct SigningError {
    message: String,
}

#[derive(Debug)]
pub struct Signer {
    service: String,
    region: String,
}

impl Signer {
    pub fn new(service: String, region: String) -> Self {
        Signer { service, region }
    }

    pub fn generate_signed_headers(
        &self,
        uri: &Uri,
        method: Method,
        body: &Bytes,
        creds: &AwsCredentials,
    ) -> Result<HeaderMap, SigningError> {
        let init = SigningInit::new(uri, method, body)?;
        let canonical_request = CanonicalRequest::from(&init);
        let unsigned_string = UnsignedString::from(&self.service, &self.region, &canonical_request);
        let signed = unsigned_string.sign(creds.aws_secret_access_key());

        let mut signed_headers = init.headers;
        signed_headers.insert(header::AUTHORIZATION, to_header_value(signed.token)?);

        if let Some(token) = creds.token() {
            signed_headers.insert("x-amz-security-token", to_header_value(token.to_string())?);
        }

        signed_headers.insert(
            "x-amz-content-sha256",
            to_header_value(init.payload_hash.clone())?,
        );

        return Ok(signed_headers);
    }
}

struct SigningInit {
    uri: Uri,
    method: Method,
    payload_hash: String,
    headers: HeaderMap,
    date: OffsetDateTime,
}

impl SigningInit {
    fn new(uri: &Uri, method: Method, body: &Bytes) -> Result<Self, SigningError> {
        SigningInit::new_with_date(uri, method, body, OffsetDateTime::now_utc())
    }

    fn new_with_date(
        uri: &Uri,
        method: Method,
        body: &Bytes,
        date: OffsetDateTime,
    ) -> Result<Self, SigningError> {
        let mut headers = HeaderMap::new();
        let uri = uri.clone();

        headers.insert(
            header::HOST,
            to_header_value(uri.host().unwrap().to_string())?,
        );
        headers.insert(
            "x-amz-date",
            to_header_value(date.format(ISO_8601_DATETIME))?,
        );

        Ok(SigningInit {
            uri,
            method,
            payload_hash: payload_hash(body),
            date,
            headers,
        })
    }
}

fn payload_hash(body: &Bytes) -> String {
    if body.is_empty() {
        String::from(EMPTY_SHA256_HASH)
    } else {
        to_hexdigest(body)
    }
}

struct UnsignedString {
    service: String,
    region: String,
    string_to_sign: String,
    date: OffsetDateTime,
}

impl UnsignedString {
    fn from(service: &str, region: &str, canonical_request: &CanonicalRequest) -> Self {
        let scope = format!(
            "{}/{}/{}/aws4_request",
            canonical_request.date.format(ISO_8601_DATE),
            region,
            service
        );

        let string_to_sign =
            string_to_sign(canonical_request.date, &canonical_request.hash, &scope);

        UnsignedString {
            service: String::from(service),
            region: String::from(region),
            date: canonical_request.date.clone(),
            string_to_sign,
        }
    }

    fn sign(&self, secret_access_key: &str) -> RequestSignature {
        let token = sign_string(
            &self.string_to_sign,
            secret_access_key,
            self.date.date(),
            &self.region,
            &self.service,
        );

        RequestSignature { token }
    }
}

struct RequestSignature {
    token: String,
}

struct CanonicalRequest {
    date: OffsetDateTime,
    hash: String,
    #[cfg(test)]
    signed_headers: String,
}

impl CanonicalRequest {
    fn from(request: &SigningInit) -> Self {
        let mut header_list: Vec<String> = Vec::new();
        let mut signed_header_set: BTreeSet<String> = BTreeSet::new();

        for (key, value) in request.headers.iter() {
            header_list.push(format!("{}:{}", key, value.to_str().unwrap()));
            signed_header_set.insert(key.to_string());
        }

        header_list.sort();
        let mut signed_headers: Vec<String> = signed_header_set.into_iter().collect();
        signed_headers.sort();
        let signed_headers = signed_headers.join(";");

        let path = request.uri.path().to_string();
        let path: String = utf8_percent_encode(&path, &STRICT_PATH_ENCODE_SET).collect();
        let query = encode_query_string(&request.uri);

        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            request.method.as_str(),
            path,
            query,
            format!("{}\n", header_list.join("\n")),
            signed_headers,
            request.payload_hash
        );

        trace!("canonical_request: {}", canonical_request);
        let hash = to_hexdigest(&canonical_request);

        CanonicalRequest {
            hash,
            #[cfg(test)]
            signed_headers,
            date: request.date.clone(),
        }
    }
}

fn encode_query_string(uri: &Uri) -> String {
    let url = url::Url::parse(&uri.to_string()).unwrap();

    let mut params: Vec<String> = Vec::new();

    for (name, value) in url.query_pairs() {
        params.push(format!(
            "{}={}",
            encode_uri_strict(&name),
            encode_uri_strict(&value)
        ))
    }

    params.sort();
    params.join("&")
}

#[test]
fn test_encode_query_string() {
    assert_eq!(
        "",
        encode_query_string(&"http://localhost".parse().unwrap())
    );
    assert_eq!(
        "foo=bar",
        encode_query_string(&"http://localhost?foo=bar".parse().unwrap())
    );

    // both name and value are encoded
    assert_eq!(
        "%2Afoo=%2Abar",
        encode_query_string(&"http://localhost?*foo=*bar".parse().unwrap())
    );

    // ensure order
    assert_eq!(
        "%2Abar=%2Afoo&%2Afoo=%2Abar",
        encode_query_string(&"http://localhost?*foo=*bar&*bar=*foo".parse().unwrap())
    );

    // same key multiple times
    assert_eq!(
        "%2Afoo=%2Abar&%2Afoo=%2Afoo",
        encode_query_string(&"http://localhost?*foo=*bar&*foo=*foo".parse().unwrap())
    );
}

fn to_header_value<T: AsRef<str>>(value: T) -> Result<HeaderValue, InvalidHeaderValue> {
    HeaderValue::from_str(value.as_ref())
}

impl From<InvalidHeaderValue> for SigningError {
    fn from(v: InvalidHeaderValue) -> SigningError {
        SigningError {
            message: v.to_string(),
        }
    }
}

impl std::fmt::Display for SigningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SigningError {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::credential::AwsCredentials;
    fn init_test_logging() {
        let filter = log::LevelFilter::Info;
        // let filter = log::LevelFilter::Trace;
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(filter)
            .try_init();
    }

    fn make_aws_example_signing_init() -> SigningInit {
        let date = OffsetDateTime::from_unix_timestamp(1440938160);
        let uri = "https://iam.amazonaws.com/?Action=ListUsers&Version=2010-05-08"
            .parse()
            .unwrap();
        let body = Bytes::new();
        let mut siging_request =
            SigningInit::new_with_date(&uri, Method::GET, &body, date).unwrap();
        siging_request.headers.insert(
            header::CONTENT_TYPE,
            to_header_value("application/x-www-form-urlencoded; charset=utf-8").unwrap(),
        );
        siging_request
    }

    // Example taken from https://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html
    #[test]
    fn test_canonical_request() {
        init_test_logging();
        let siging_request = make_aws_example_signing_init();
        let canonical_request = CanonicalRequest::from(&siging_request);
        assert_eq!(
            "content-type;host;x-amz-date",
            canonical_request.signed_headers
        );
        assert_eq!(
            "f536975d06c0309214f805bb90ccff089219ecd68b2577efef23edd43b7e1a59",
            canonical_request.hash
        );
    }

    // Example taken from https://docs.aws.amazon.com/general/latest/gr/sigv4-create-string-to-sign.html
    #[test]
    fn test_create_string_to_sign() {
        init_test_logging();

        let expected = "AWS4-HMAC-SHA256
20150830T123600Z
20150830/us-east-1/iam/aws4_request
f536975d06c0309214f805bb90ccff089219ecd68b2577efef23edd43b7e1a59";

        let siging_request = make_aws_example_signing_init();
        let canonical_request = CanonicalRequest::from(&siging_request);
        let unsigned_string = UnsignedString::from("iam", "us-east-1", &canonical_request);

        assert_eq!(expected, unsigned_string.string_to_sign);
    }

    // Example taken from https://docs.aws.amazon.com/general/latest/gr/sigv4-calculate-signature.html
    #[test]
    fn test_creat_auth_token() {
        let example_secret = "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY";
        let siging_request = make_aws_example_signing_init();
        let canonical_request = CanonicalRequest::from(&siging_request);
        let unsigned_string = UnsignedString::from("iam", "us-east-1", &canonical_request);
        let token = unsigned_string.sign(example_secret);

        assert_eq!(
            "5d672d79c15b13162d9279b0855cfba6789a8edb4c82c400e06b5924a6f2b5d7",
            token.token
        );
    }

    #[test]
    fn verity_signer() {
        let signer = Signer::new(String::from("iam"), String::from("us-east-1"));
        let uri = "https://iam.amazonaws.com/?Action=ListUsers&Version=2010-05-08"
            .parse()
            .unwrap();
        let body = Bytes::new();
        let creds = AwsCredentials::new("foo_access_key", "foo_secret_key", None, None);

        signer
            .generate_signed_headers(&uri, Method::GET, &body, &creds)
            .unwrap();
    }
}
