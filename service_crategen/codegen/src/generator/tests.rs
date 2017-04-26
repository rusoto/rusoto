use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::{capitalize_first, FileWriter, IoResult};
use botocore::Service;
use util::case_insensitive_btreemap_get;
use inflector::Inflector;

const BOTOCORE_TESTS_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"),
                                                 "/botocore/tests/unit/response_parsing/xml/responses/");

pub fn generate_tests(writer: &mut FileWriter, service: &Service) -> IoResult {
    writeln!(writer,
             "
            #[cfg(test)]
            mod protocol_tests {{
                {tests_body}
            }}
            ",
             tests_body = generate_tests_body(service).unwrap_or("".to_string()))
}

pub fn generate_tests_body(service: &Service) -> Option<String> {
    let responses = find_responses_for_service(service);

    let test_bodies: Vec<String> = responses.into_iter()
        .flat_map(|response| generate_response_parse_test(service, response))
        .collect();
    
    if !test_bodies.is_empty() {
        let tests_str = test_bodies.join("\n\n");

        Some(format!("
                extern crate rusoto_mock;

                use super::*;
                use self::rusoto_mock::*;
                use rusoto_core::Region as rusoto_region;

                {test_bodies}",
                     test_bodies = tests_str))
    } else {
        None
    }
}

fn generate_response_parse_test(service: &Service, response: Response) -> Option<String> {
    let maybe_operation = case_insensitive_btreemap_get(&service.operations, &response.action);

    if maybe_operation.is_none() {
        return None;
    }

    let operation = maybe_operation.unwrap();
    let request_params;
    let request_constructor;
    if operation.input.is_some() {
        request_constructor = format!("let request = {request_type}::default();",
                                      request_type = operation.input_shape());
        request_params = "&request".to_string();
    } else {
        request_constructor = "".to_string();
        request_params = "".to_string();
    }

    Some(format!("
        #[test]
        fn test_parse_{service_name}_{action}() {{
            let mock_response =  MockResponseReader::read_response(\"test_resources/generated\", \"{response_file_name}\");
            let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
            let client = {client_type}::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            {request_constructor}
            let result = client.{action}({request_params});
            assert!(result.is_ok(), \"parse error: {{:?}}\", result);
        }}",
        service_name = response.service.to_snake_case(),
        action = response.action.to_snake_case(),
        response_file_name = response.file_name,
        client_type = service.client_type_name(),
        request_constructor = request_constructor,
        request_params = request_params))
}

#[derive(Debug, Clone)]
pub struct Response {
    pub service: String,
    pub action: String,
    pub file_name: String,
    pub full_path: PathBuf
}

impl Response {
    pub fn from_response_path(path: &Path) -> Option<Response> {
        if let Some(file_name) = path.file_name() {
            if let Some(file_stem) = path.file_stem() {
                let file_stem = file_stem.to_string_lossy().into_owned();

                let file_name_parts: Vec<&str> = file_stem.split('-').collect();

                let service_name = file_name_parts.get(0).map(|s| capitalize_first(*s));

                let action = Some(file_name_parts.into_iter().skip(1).map(capitalize_first).collect());

                service_name.and_then(|s| {
                    action.and_then(|a| {
                        Some(Response {
                            service: s,
                            action: a,
                            file_name: file_name.to_string_lossy().into_owned(),
                            full_path: path.to_owned()
                        })
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

    dir.filter_map(|e| e.ok())
        .filter(|d| d.path().extension().map(|ex| ex == "xml").unwrap_or(false))
        .filter_map(|d| Response::from_response_path(&d.path()))
        .collect()
}

pub fn find_responses_for_service(service: &Service) -> Vec<Response> {
    let mut responses = Vec::new();

    for r in find_responses_in_dir(Path::new(BOTOCORE_TESTS_DIR)) {
        if r.service.to_lowercase() == service.service_type_name().to_lowercase() {
            responses.push(r);
        }
    }

    responses
}