use super::generate_field_name;
use crate::botocore::{Member, Operation, Shape, ShapeType};
use crate::Service;
use inflector::Inflector;
use lazy_static::lazy_static;
use regex::Regex;

// Add request headers for any shape members marked as headers
pub fn generate_headers(service: &Service<'_>, operation: &Operation) -> Option<String> {
    operation.input.as_ref()?;

    let shape = service
        .get_shape(&operation.input.as_ref().unwrap().shape)
        .unwrap();

    Some(shape.members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| {
            member.location.as_ref()?;
            match &member.location.as_ref().unwrap()[..] {
                "header" => {
                    if shape.required(member_name) {
                        Some(format!("request.add_header(\"{location_name}\", 
                                      &input.{field_name});",
                                     location_name = member.location_name.as_ref().unwrap(),
                                     field_name = generate_field_name(member_name)))
                    } else {
                        Some(format!("
                        if let Some(ref {field_name}) = 
                                      input.{field_name} {{
                                      request.add_header(\"{location_name}\", 
                                      &{field_name}.to_string());
                        }}",
                                     location_name = member.location_name.as_ref().unwrap(),
                                     field_name = generate_field_name(member_name)))
                    }
                },
                "headers" => {
                    if shape.required(member_name) {
                        Some(format!("for (header_name, header_value) in input.{field_name}.iter() {{
                                              let header = format!(\"{location_name}{{}}\", header_name);
                                              request.add_header(header,
                                                                 header_value);
                                          }}
                                      ",
                                     location_name = member.location_name.as_ref().unwrap(),
                                     field_name = generate_field_name(member_name)))
                    } else {
                        Some(format!("
                        if let Some(ref {field_name}) = 
                                      input.{field_name} {{
                                          for (header_name, header_value) in {field_name}.iter() {{
                                              let header = format!(\"{location_name}{{}}\", header_name);
                                              request.add_header(header,
                                                                 header_value);
                                          }}

                        }}",
                                     location_name = member.location_name.as_ref().unwrap(),
                                     field_name = generate_field_name(member_name)))
                    }
                }
                _ => None,
            }
        })
        .collect::<Vec<String>>()
        .join("\n"))
}

pub fn generate_params_loading_string(
    service: &Service<'_>,
    operation: &Operation,
) -> Option<String> {
    operation.input.as_ref()?;

    let input_type = operation.input_shape();
    let input_shape = service.get_shape(input_type).unwrap();

    // Construct a list of strings which will be used to load request
    // parameters from the input struct into a `Params` vec, which will
    // then be added to the request.
    let mut param_strings = generate_shape_member_param_strings(service, input_shape);
    param_strings.append(&mut generate_static_param_strings(operation));

    let load_params = match param_strings.len() {
        0 => "".to_owned(),
        _ => format!(
            "let mut params = Params::new();
                {param_strings}
                request.set_params(params);",
            param_strings = param_strings.join("\n")
        ),
    };

    Some(load_params)
}

pub fn generate_uri_formatter(
    request_uri: &str,
    service: &Service<'_>,
    operation: &Operation,
) -> Option<String> {
    if operation.input.is_some() {
        let input_shape = &service.get_shape(operation.input_shape()).unwrap();
        let uri_strings = generate_shape_member_uri_strings(input_shape);

        if !uri_strings.is_empty() {
            // massage for Route53.
            // See https://github.com/rusoto/rusoto/issues/997 .
            let replace_em = match service.name().to_ascii_lowercase().as_ref() {
                "route 53" => {
                    ".replace(\"/hostedzone/hostedzone/\", \"/hostedzone/\").replace(\"/change/change/\", \"/change/\")"
                }
                _ => ""
            };
            return Some(format!(
                "let request_uri = format!(\"{request_uri}\", {uri_strings}){replace_if_needed};",
                request_uri = generate_snake_case_uri(request_uri),
                uri_strings = uri_strings.join(", "),
                replace_if_needed = replace_em
            ));
        }
    }

    Some(format!(
        "let request_uri = \"{request_uri}\";",
        request_uri = request_uri
    ))
}

fn generate_shape_member_uri_strings(shape: &Shape) -> Vec<String> {
    shape
        .members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| {
            generate_member_format_string(&generate_field_name(member_name), member)
        })
        .collect::<Vec<String>>()
}

fn generate_member_format_string(member_name: &str, member: &Member) -> Option<String> {
    match member.location {
        Some(ref x) if x == "uri" => match member.location_name {
            Some(ref loc_name) => Some(format!(
                "{member_name} = input.{field_name}",
                field_name = member_name,
                member_name = loc_name.to_snake_case(),
            )),
            None => Some(format!(
                "{member_name} = input.{field_name}",
                field_name = member_name,
                member_name = member_name.to_snake_case(),
            )),
        },
        Some(_) | None => None,
    }
}

fn generate_static_param_strings(operation: &Operation) -> Vec<String> {
    // botocore includes + for greedy parameters and we don't care about it
    let (_, maybe_params) = parse_query_string(&operation.http.request_uri);

    let mut static_params = vec![];

    // Sometimes the key has the query param already set, like "list-type=2"
    // These need to get parsed out of the URI and added to the params map
    // with the other "queryparam" members
    if let Some(ref params) = maybe_params {
        for param in params.split('&') {
            match param.find('=') {
                Some(_) => {
                    let key_val_vec: Vec<&str> = param.split('=').collect();
                    static_params.push(format!(
                        "params.put(\"{}\", \"{}\");",
                        key_val_vec[0], key_val_vec[1]
                    ))
                }
                None => static_params.push(format!("params.put_key(\"{}\");", param)),
            }
        }
    };

    static_params
}

// This is a bit messy and could use refactoring.
fn generate_snake_case_uri(request_uri: &str) -> String {
    lazy_static! {
        static ref URI_ARGS_SNAKE_REGEX: Regex = Regex::new(r"\{([\w\d]+)\}").unwrap();
        static ref URI_ARGS_DASH_TO_UNDERSCORE_REGEX: Regex = Regex::new(r"\{\w+\-\w+\}").unwrap();
    }
    let mut snake: String = request_uri.to_string().clone();
    // convert fooBar to foo_bar
    for caps in URI_ARGS_SNAKE_REGEX.captures_iter(request_uri) {
        let to_find = caps.get(0).expect("nothing captured").as_str();
        // Wrap with curly braces again:
        let replacement = format!(
            "{{{}}}",
            Inflector::to_snake_case(caps.get(0).unwrap().as_str())
        );
        snake = snake.replace(to_find, &replacement);
    }

    // convert foo-bar to foo_bar
    let temp_snake = snake.clone();
    for caps in URI_ARGS_DASH_TO_UNDERSCORE_REGEX.captures_iter(&temp_snake) {
        let to_find = caps.get(0).unwrap().as_str();
        let replacement = caps.get(0).unwrap().as_str().replace("-", "_");
        snake = snake.replace(to_find, &replacement);
    }

    snake
}

fn generate_shape_member_param_strings(service: &Service<'_>, shape: &Shape) -> Vec<String> {
    shape
        .members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| {
            member.location.as_ref().and_then(|loc| {
                if !member.deprecated() && loc == "querystring" {
                    let member_shape = service.shape_for_member(member).unwrap();
                    Some(generate_param_load_string(
                        member_name,
                        member_shape,
                        member,
                        shape.required(member_name),
                    ))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<String>>()
}

fn generate_param_load_string(
    member_name: &str,
    member_shape: &Shape,
    member: &Member,
    is_required: bool,
) -> String {
    let field_name = generate_field_name(member_name);
    let param_name = match member.location_name {
        Some(ref location) => location.to_owned(),
        None => member_name.to_owned(),
    };
    match (member_shape.shape_type, is_required) {
        (ShapeType::List, true) => format!(
            "for item in input.{field_name}.iter() {{
                     params.put(\"{param_name}\", item);
                }}",
            param_name = param_name,
            field_name = field_name
        ),
        (ShapeType::List, false) => format!(
            "if let Some(ref x) = input.{field_name} {{
                    for item in x.iter() {{
                        params.put(\"{param_name}\", item);
                    }}
                }}",
            param_name = param_name,
            field_name = field_name,
        ),
        (ShapeType::Map, true) => format!(
            "for (key, val) in input.{field_name}.iter() {{
                     params.put(key, val);
                }}",
            field_name = field_name
        ),
        (ShapeType::Map, false) => format!(
            "if let Some(ref x) = input.{field_name} {{
                    for (key, val) in x.iter() {{
                        params.put(key, val);
                    }}
                }}",
            field_name = field_name,
        ),
        (_, true) => format!(
            "params.put(\"{param_name}\", &input.{field_name});",
            param_name = param_name,
            field_name = field_name
        ),
        (_, false) => format!(
            "if let Some(ref x) = input.{field_name} {{
                    params.put(\"{param_name}\", x);
                }}",
            param_name = param_name,
            field_name = field_name,
        ),
    }
}

pub fn parse_query_string(uri: &str) -> (String, Option<String>) {
    // botocore query strings for S3 are variations on "/{Bucket}/{Key+}?foobar"
    // the query string needs to be split out and put in the params hash,
    // and the + isn't useful information for us
    let base_uri = uri.replace("+", "");
    let parts: Vec<&str> = base_uri.split('?').collect();

    match parts.len() {
        1 => (parts[0].to_owned(), None),
        2 => (parts[0].to_owned(), Some(parts[1].to_owned())),
        _ => panic!("Unknown uri structure {}", uri),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uri_snakeification_works() {
        let basic = "application-id";
        assert_eq!("application-id", generate_snake_case_uri(&basic));

        let simple_uri = "/v1/apps/{application-id}";
        assert_eq!(
            "/v1/apps/{application_id}",
            generate_snake_case_uri(&simple_uri)
        );

        let two_items_uri = "/v1/ap-ps/{application-id}/endpoints/{endpoint-id}";
        assert_eq!(
            "/v1/ap-ps/{application_id}/endpoints/{endpoint_id}",
            generate_snake_case_uri(&two_items_uri)
        );

        // existing behavior
        let snake_case = "{usageplanId}";
        assert_eq!("{usageplan_id}", generate_snake_case_uri(&snake_case));

        let lower_snake_case = "{api_Key}";
        assert_eq!("{api_key}", generate_snake_case_uri(&lower_snake_case));

        let no_touching_non_params = "/2017-03-25/distributionsByWebACLId/{WebACLId}";
        assert_eq!(
            "/2017-03-25/distributionsByWebACLId/{web_acl_id}",
            generate_snake_case_uri(&no_touching_non_params)
        );
    }

    #[test]
    fn generate_static_param_strings_parses_mixed_params() {
        let service_json = "{\
            \"name\": \"Search\",
            \"http\":{
                \"method\":\"GET\",
                \"requestUri\":\"/2013-01-01/suggest?format=sdk&pretty\"
            }\
        }";
        let operation = serde_json::from_str(service_json).expect("failed to parse operation json");

        let static_params = generate_static_param_strings(&operation);

        assert_eq!(static_params.len(), 2);
        assert_eq!(
            static_params.get(0),
            Some(&"params.put(\"format\", \"sdk\");".to_owned())
        );
        assert_eq!(
            static_params.get(1),
            Some(&"params.put_key(\"pretty\");".to_owned())
        );
    }
}
