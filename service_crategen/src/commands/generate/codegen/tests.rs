use inflector::Inflector;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use self::util::case_insensitive_btreemap_get;
use super::{FileWriter, IoResult};
use crate::util;
use crate::Service;

const BOTOCORE_ERROR_RESPONSE_TESTS_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/botocore/tests/unit/response_parsing/xml/errors/"
);
const BOTOCORE_VALID_RESPONSE_TESTS_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/botocore/tests/unit/response_parsing/xml/responses/"
);

pub fn generate_tests(writer: &mut FileWriter, service: &Service<'_>) -> IoResult {
    match generate_tests_body(service) {
        Some(test_bodies) => writeln!(
            writer,
            "
                    #[cfg(test)]
                    mod protocol_tests {{
                        {tests_body}
                    }}
                    ",
            tests_body = test_bodies
        ),
        None => Ok(()),
    }
}

fn generate_tests_body(service: &Service<'_>) -> Option<String> {
    let valid_tests = generate_response_tests(
        service,
        find_valid_responses_for_service(service),
        200,
        true,
    );
    // TODO: Find the true status code. Sadly, it isn't stored with the xml response.
    let error_tests = generate_response_tests(
        service,
        find_error_responses_for_service(service),
        400,
        false,
    );

    if valid_tests.is_some() || error_tests.is_some() {
        Some(format!(
            "
            extern crate rusoto_mock;

            use super::*;
            use self::rusoto_mock::*;
            use rusoto_core::Region as rusoto_region;

            {error_tests}
            {valid_tests}",
            error_tests = error_tests.unwrap_or_else(|| "".to_string()),
            valid_tests = valid_tests.unwrap_or_else(|| "".to_string())
        ))
    } else {
        None
    }
}

fn generate_response_tests(
    service: &Service<'_>,
    responses: Vec<Response>,
    status_code: i32,
    is_ok: bool,
) -> Option<String> {
    let our_responses: Vec<Response> = responses
        .into_iter()
        .filter(|r| r.service.to_lowercase() == service.service_type_name().to_lowercase())
        .map(|r| r.to_owned())
        .collect();

    let test_bodies: Vec<String> = our_responses
        .into_iter()
        .flat_map(|response| generate_response_parse_test(service, &response, status_code, is_ok))
        .collect();

    if !test_bodies.is_empty() {
        Some(test_bodies.join("\n\n"))
    } else {
        None
    }
}

fn generate_response_parse_test(
    service: &Service<'_>,
    response: &Response,
    status_code: i32,
    is_ok: bool,
) -> Option<String> {
    let maybe_operation = case_insensitive_btreemap_get(service.operations(), &response.action);

    maybe_operation?;

    let operation = maybe_operation.unwrap();
    let request_params;
    let request_constructor;
    if operation.input.is_some() {
        request_constructor = format!(
            "let request = {request_type}::default();",
            request_type = operation.input_shape()
        );
        request_params = "request".to_string();
    } else {
        request_constructor = "".to_string();
        request_params = "".to_string();
    }

    Some(format!("
        #[tokio::test]
        async fn test_parse_{error_or_valid}_{service_name}_{action}() {{
            let mock_response =  MockResponseReader::read_response(\"test_resources/generated/{error_or_valid}\", \"{response_file_name}\");
            let mock = MockRequestDispatcher::with_status({status_code}).with_body(&mock_response);
            let client = {client_type}::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            {request_constructor}
            let result = client.{action}({request_params}).await;
            assert!({is_ok}result.is_ok(), \"parse error: {{:?}}\", result);
        }}",
        error_or_valid = if is_ok { "valid" } else { "error" },
        service_name = response.service.to_snake_case(),
        action = response.action.to_snake_case(),
        status_code = status_code,
        response_file_name = response.file_name,
        client_type = service.client_type_name(),
        request_constructor = request_constructor,
        request_params = request_params,
        is_ok = if is_ok { "" } else { "!" }))
}

#[derive(Debug, Clone)]
pub struct Response {
    pub service: String,
    pub action: String,
    pub file_name: String,
    pub full_path: PathBuf,
}

impl Response {
    pub fn from_response_path(path: &Path) -> Option<Response> {
        if let Some(file_name) = path.file_name() {
            if let Some(file_stem) = path.file_stem() {
                let file_stem = file_stem.to_string_lossy().into_owned();

                let file_name_parts: Vec<&str> = file_stem.split('-').collect();

                let service_name = file_name_parts.get(0).map(|s| util::capitalize_first(*s));

                let action = Some(
                    file_name_parts
                        .into_iter()
                        .skip(1)
                        .map(util::capitalize_first)
                        .collect(),
                );

                service_name.and_then(|s| {
                    action.map(|a| Response {
                        service: s,
                        action: a,
                        file_name: file_name.to_string_lossy().into_owned(),
                        full_path: path.to_owned(),
                    })
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn find_responses_in_dir(dir_path: &Path) -> Vec<Response> {
    let dir = fs::read_dir(dir_path).expect("read_dir");

    let mut responses = dir
        .filter_map(std::result::Result::ok)
        .filter(|d| d.path().extension().map(|ex| ex == "xml").unwrap_or(false))
        .filter_map(|d| Response::from_response_path(&d.path()))
        .collect::<Vec<_>>();

    responses.sort_by_key(|e| e.full_path.clone());

    responses
}

pub fn find_error_responses_for_service(service: &Service<'_>) -> Vec<Response> {
    let mut responses = Vec::new();

    for r in find_responses_in_dir(Path::new(BOTOCORE_ERROR_RESPONSE_TESTS_DIR)) {
        if r.service.to_lowercase() == service.service_type_name().to_lowercase() {
            responses.push(r);
        }
    }

    responses
}

pub fn find_valid_responses_for_service(service: &Service<'_>) -> Vec<Response> {
    let mut responses = Vec::new();

    for r in find_responses_in_dir(Path::new(BOTOCORE_VALID_RESPONSE_TESTS_DIR)) {
        if r.service.to_lowercase() == service.service_type_name().to_lowercase() {
            responses.push(r);
        }
    }

    responses
}
