use serde::Deserialize;
use serde_json::{from_slice, Value};

use super::super::super::request::BufferedHttpResponse;

#[derive(Deserialize)]
struct RawError {
    #[serde(rename = "__type", default)]
    typ: Option<String>,
    #[serde(alias = "Message", default)]
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
                typ: (*typ).to_string(),
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
    use http::StatusCode;

    let payload = r#"{"__type":"com.amazonaws.dynamodb.v20120810#ResourceNotFoundException",
"message":"Requested resource not found: Table: tablename not found"}"#;
    let response = BufferedHttpResponse {
        status: StatusCode::OK,
        body: payload.into(),
        headers: Default::default(),
    };

    let error = Error::parse(&response).unwrap();

    assert_eq!(error.typ, "ResourceNotFoundException");
    assert_eq!(
        error.msg,
        "Requested resource not found: Table: tablename not found"
    );
}

#[test]
fn deserialize_athena_error() {
    use http::StatusCode;

    let payload = r#"{"__type":"InvalidRequestException","AthenaErrorCode":"MALFORMED_QUERY","ErrorCode":"MALFORMED_QUERY","Message":"line 6:18: mismatched input '.' expecting {<EOF>, ',', 'ADD', 'AS', 'ALL', 'SOME', 'ANY', 'WHERE', 'GROUP', 'ORDER', 'HAVING', 'LIMIT', 'AT', 'NO', 'SUBSTRING', 'POSITION', 'TINYINT', 'SMALLINT', 'INTEGER', 'DATE', 'TIME', 'TIMESTAMP', 'INTERVAL', 'YEAR', 'MONTH', 'DAY', 'HOUR', 'MINUTE', 'SECOND', 'ZONE', 'JOIN', 'CROSS', 'INNER', 'LEFT', 'RIGHT', 'FULL', 'NATURAL', 'FILTER', 'OVER', 'PARTITION', 'RANGE', 'ROWS', 'PRECEDING', 'FOLLOWING', 'CURRENT', 'ROW', 'SCHEMA', 'COMMENT', 'VIEW', 'REPLACE', 'GRANT', 'REVOKE', 'PRIVILEGES', 'PUBLIC', 'OPTION', 'EXPLAIN', 'ANALYZE', 'FORMAT', 'TYPE', 'TEXT', 'GRAPHVIZ', 'LOGICAL', 'DISTRIBUTED', 'VALIDATE', 'SHOW', 'TABLES', 'VIEWS', 'SCHEMAS', 'CATALOGS', 'COLUMNS', 'COLUMN', 'USE', 'PARTITIONS', 'FUNCTIONS', 'UNION', 'EXCEPT', 'INTERSECT', 'TO', 'SYSTEM', 'BERNOULLI', 'POISSONIZED', 'TABLESAMPLE', 'ARRAY', 'MAP', 'SET', 'RESET', 'SESSION', 'DATA', 'START', 'TRANSACTION', 'COMMIT', 'ROLLBACK', 'WORK', 'ISOLATION', 'LEVEL', 'SERIALIZABLE', 'REPEATABLE', 'COMMITTED', 'UNCOMMITTED', 'READ', 'WRITE', 'ONLY', 'CALL', 'INPUT', 'OUTPUT', 'CASCADE', 'RESTRICT', 'INCLUDING', 'EXCLUDING', 'PROPERTIES', 'FUNCTION', 'RETURNS', 'LANGUAGE', 'OPTIONS', 'SCALAR', 'AGGREGATE', 'WINDOW', 'NFD', 'NFC', 'NFKD', 'NFKC', 'IF', 'NULLIF', 'COALESCE', IDENTIFIER, DIGIT_IDENTIFIER, QUOTED_IDENTIFIER, BACKQUOTED_IDENTIFIER}"}"#;
    let response = BufferedHttpResponse {
        status: StatusCode::NOT_FOUND,
        body: payload.into(),
        headers: Default::default(),
    };

    let error = Error::parse(&response).unwrap();

    assert_eq!(error.typ, "InvalidRequestException");
    assert_eq!(
        error.msg,
        r#"line 6:18: mismatched input '.' expecting {<EOF>, ',', 'ADD', 'AS', 'ALL', 'SOME', 'ANY', 'WHERE', 'GROUP', 'ORDER', 'HAVING', 'LIMIT', 'AT', 'NO', 'SUBSTRING', 'POSITION', 'TINYINT', 'SMALLINT', 'INTEGER', 'DATE', 'TIME', 'TIMESTAMP', 'INTERVAL', 'YEAR', 'MONTH', 'DAY', 'HOUR', 'MINUTE', 'SECOND', 'ZONE', 'JOIN', 'CROSS', 'INNER', 'LEFT', 'RIGHT', 'FULL', 'NATURAL', 'FILTER', 'OVER', 'PARTITION', 'RANGE', 'ROWS', 'PRECEDING', 'FOLLOWING', 'CURRENT', 'ROW', 'SCHEMA', 'COMMENT', 'VIEW', 'REPLACE', 'GRANT', 'REVOKE', 'PRIVILEGES', 'PUBLIC', 'OPTION', 'EXPLAIN', 'ANALYZE', 'FORMAT', 'TYPE', 'TEXT', 'GRAPHVIZ', 'LOGICAL', 'DISTRIBUTED', 'VALIDATE', 'SHOW', 'TABLES', 'VIEWS', 'SCHEMAS', 'CATALOGS', 'COLUMNS', 'COLUMN', 'USE', 'PARTITIONS', 'FUNCTIONS', 'UNION', 'EXCEPT', 'INTERSECT', 'TO', 'SYSTEM', 'BERNOULLI', 'POISSONIZED', 'TABLESAMPLE', 'ARRAY', 'MAP', 'SET', 'RESET', 'SESSION', 'DATA', 'START', 'TRANSACTION', 'COMMIT', 'ROLLBACK', 'WORK', 'ISOLATION', 'LEVEL', 'SERIALIZABLE', 'REPEATABLE', 'COMMITTED', 'UNCOMMITTED', 'READ', 'WRITE', 'ONLY', 'CALL', 'INPUT', 'OUTPUT', 'CASCADE', 'RESTRICT', 'INCLUDING', 'EXCLUDING', 'PROPERTIES', 'FUNCTION', 'RETURNS', 'LANGUAGE', 'OPTIONS', 'SCALAR', 'AGGREGATE', 'WINDOW', 'NFD', 'NFC', 'NFKD', 'NFKC', 'IF', 'NULLIF', 'COALESCE', IDENTIFIER, DIGIT_IDENTIFIER, QUOTED_IDENTIFIER, BACKQUOTED_IDENTIFIER}"#
    );
}
