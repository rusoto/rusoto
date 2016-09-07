use std::fs;

fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

#[derive(Debug, Clone	)]
pub struct Response {
    pub service: String,
    pub action: String,
    pub file_name: String,
    pub extension: String,
}

impl Response {
    pub fn from_response_file_name(f: &str) -> Option<Response> {
        let maybe_file_name_and_extension: Vec<&str> = f.split(".").collect();

        let mut service_name = None;
        let mut action = None;
        let extension = maybe_file_name_and_extension.get(1);

        if let Some(file_name) = maybe_file_name_and_extension.get(0) {
            let file_name_parts: Vec<&str> = file_name.split("-").collect();

            service_name = file_name_parts.get(0).map(|s| capitalise(s));

            action = Some(file_name_parts.into_iter().skip(1).map(|w| capitalise(w)).collect());
        }

        service_name
            .and_then(|s| action
                .and_then(|a| extension
                    .and_then(|e|
                        Some(Response { 
                            service: s, 
                            action: a, 
                            file_name: f.to_owned(),
                            extension: e.to_string(),
                        })
                    )
                )
            )
    }
}

pub fn find_responses() -> Vec<Response> {
    let dir = fs::read_dir("./codegen/botocore/tests/unit/response_parsing/xml/responses")
        .expect("read_dir");

    dir
        .map(|d| d.expect("direntry").file_name().into_string().expect("osstr"))
        .flat_map(|f| Response::from_response_file_name(&f))
        .filter(|r| r.extension == "xml")
        .collect()
}