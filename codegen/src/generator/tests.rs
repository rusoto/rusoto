use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

use super::{capitalize_first, FileWriter, IoResult};
use botocore::Service;
use util::case_insensitive_btreemap_get;
use inflector::Inflector;

const BOTOCORE_TESTS_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"),
                                                 "/botocore/tests/unit/response_parsing/xml/responses/");
const OUR_TESTS_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/unit/responses/");


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

fn generate_tests_body(service: &Service) -> Option<String> {
    let responses: HashMap<String, Response> = find_responses();

    let our_responses: Vec<Response> = responses.values()
        .into_iter()
        .filter(|r| r.service.to_lowercase() == service.service_type_name().to_lowercase())
        .map(|r| r.to_owned())
        .collect();

    let test_bodies: Vec<String> = our_responses.into_iter()
        .flat_map(|response| generate_response_parse_test(service, response))
        .collect();

    if !test_bodies.is_empty() {
        let tests_str = test_bodies.join("\n\n");

        Some(format!("
                use rusoto::mock::*;
                use super::*;
                use super::Region as rusoto_region;

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
            let mock_response =  MockResponseReader::read_response(r#\"{response_dir_name}\"#, \"{response_file_name}\");
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(&mock_response);
            let client = {client_type}::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            {request_constructor}
            let result = client.{action}({request_params});
            assert!(result.is_ok(), \"parse error: {{:?}}\", result);
        }}",
        service_name = response.service.to_snake_case(),
        action = response.action.to_snake_case(),
        response_dir_name = response.dir_name,
        response_file_name = response.file_name,
        client_type = service.client_type_name(),
        request_constructor = request_constructor,
        request_params = request_params))
}

#[derive(Debug, Clone)]
pub struct Response {
    pub service: String,
    pub action: String,
    pub dir_name: String,
    pub file_name: String,
    pub extension: String,
}

impl Response {
    pub fn from_response_file_name(directory: &Path, f: &str) -> Option<Response> {
        let maybe_file_name_and_extension: Vec<&str> = f.split('.').collect();

        let mut service_name = None;
        let mut action = None;
        let extension = maybe_file_name_and_extension.get(1);

        if let Some(file_name) = maybe_file_name_and_extension.get(0) {
            let file_name_parts: Vec<&str> = file_name.split('-').collect();

            service_name = file_name_parts.get(0).map(|s| capitalize_first(*s));

            action = Some(file_name_parts.into_iter().skip(1).map(capitalize_first).collect());
        }

        service_name.and_then(|s| {
            action.and_then(|a| {
                extension.and_then(|e| {
                    Some(Response {
                        service: s,
                        action: a,
                        dir_name: directory.to_str().unwrap().to_owned(),
                        file_name: f.to_owned(),
                        extension: e.to_string(),
                    })
                })
            })
        })
    }
}

pub fn find_responses_in_dir(dir_path: &Path) -> Vec<Response> {
    let dir = fs::read_dir(dir_path).expect("read_dir");

    dir.map(|d| d.expect("direntry").file_name().into_string().expect("osstr"))
        .flat_map(|f| Response::from_response_file_name(dir_path, &f))
        .filter(|r| r.extension == "xml")
        .collect()
}

pub fn find_responses() -> HashMap<String, Response> {
    let mut responses = HashMap::new();

    for r in find_responses_in_dir(Path::new(BOTOCORE_TESTS_DIR)) {
        responses.insert(r.file_name.clone(), r);
    }

    for r in find_responses_in_dir(Path::new(OUR_TESTS_DIR)) {
        responses.insert(r.file_name.clone(), r);
    }

    responses
}
