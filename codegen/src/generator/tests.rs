use std::fs;
use std::collections::HashMap;
use std::path::Path;

use super::capitalize_first;

const BOTOCORE_TESTS_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/botocore/tests/unit/response_parsing/xml/responses/");
const OUR_TESTS_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/unit/responses/");

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

        service_name
            .and_then(|s| action
                .and_then(|a| extension
                    .and_then(|e|
                        Some(Response {
                            service: s,
                            action: a,
                            dir_name: directory.to_str().unwrap().to_owned(),
                            file_name: f.to_owned(),
                            extension: e.to_string(),
                        })
                    )
                )
            )
    }
}

pub fn find_responses_in_dir(dir_path: &Path) -> Vec<Response> {
    let dir = fs::read_dir(dir_path)
        .expect("read_dir");

    dir
        .map(|d| d.expect("direntry").file_name().into_string().expect("osstr"))
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
