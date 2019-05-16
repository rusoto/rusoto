use serde_json::{from_slice, Value};

use super::super::super::request::BufferedHttpResponse;

#[derive(Deserialize)]
struct RawError {
    #[serde(rename = "__type", default)]
    typ: Option<String>,
    #[serde(default)]
    message: Option<String>,
}

pub struct Error {
    pub typ: String,
    pub msg: String,
}

impl Error {
    pub fn parse(res: &BufferedHttpResponse) -> Option<Error> {
        if let Ok(raw_err) = from_slice::<RawError>(&res.body) {
            let raw_error_type = raw_err.typ.unwrap_or_else(|| "Unknown".to_owned());
            let msg = raw_err.message.unwrap_or_default();

            let pieces: Vec<&str> = raw_error_type.split('#').collect();
            let typ = pieces.last().expect("Expected error type");

            Some(Error {
                typ: typ.to_string(),
                msg,
            })
        } else {
            None
        }
    }

    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn parse_rest(res: &BufferedHttpResponse) -> Option<Error> {
        if let Ok(json) = from_slice::<Value>(&res.body) {
            let typ = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("Unknown"),
            };

            // message can come in either \"message\" or \"Message\"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let msg = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("")
                .to_string();

            Some(Error {
                typ: typ.to_string(),
                msg,
            })
        } else {
            None
        }
    }
}

#[test]
fn deserialize_dynamodb_error() {
    use super::super::super::request::Headers;
    use hyper::StatusCode;

    let payload = r#"{"__type":"com.amazonaws.dynamodb.v20120810#ResourceNotFoundException",
"message":"Requested resource not found: Table: tablename not found"}"#;
    let response = BufferedHttpResponse {
        status: StatusCode::OK,
        body: payload.into(),
        headers: Headers::default(),
    };

    let error = Error::parse(&response).unwrap();

    assert_eq!(error.typ, "ResourceNotFoundException");
    assert_eq!(
        error.msg,
        "Requested resource not found: Table: tablename not found"
    );
}
