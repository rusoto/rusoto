use hyper::Client;
use hyper::client::Response;
use hyper::header::Headers;
use hyper::method::Method;
use signature::SignedRequest;

pub fn send_request(signed_request: &SignedRequest) -> Response {
    let hyper_method = match signed_request.get_method().as_ref() {
        "POST" => Method::Post,
        "PUT" => Method::Put,
        "DELETE" => Method::Delete,
        "GET" => Method::Get,
        _ => Method::Get, // make this unreachable! ?
    };

    // translate the headers map to a format Hyper likes
    let mut hyper_headers = Headers::new();
    for h in signed_request.get_headers().iter() {
        hyper_headers.set_raw(h.0.to_owned(), h.1.to_owned());
    }

    let mut final_uri = format!("https://{}{}", signed_request.get_hostname(), signed_request.get_canonical_uri());
    if signed_request.get_canonical_query_string().len() > 0 {
        final_uri = final_uri + &format!("?{}", signed_request.get_canonical_query_string());
    }

    // println!("Full request: \n method: {}\n final_uri: {}\n payload: {:?}\n",
    // 	hyper_method, final_uri, signed_request.get_payload());

    let client = Client::new();

    match signed_request.get_payload() {
        None => client.request(hyper_method, &final_uri).headers(hyper_headers).body("").send().unwrap(),
        Some(payload_contents) => client.request(hyper_method, &final_uri).headers(hyper_headers).body(payload_contents).send().unwrap(),
    }
}
