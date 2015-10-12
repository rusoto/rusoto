//! A request to AWS, pre-signed
use hyper::Client;
use hyper::client::Response;
use hyper::client::RedirectPolicy;
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

    // println!("Full request: \n method: {}\n final_uri: {}\n payload: {:?}\nHeaders:\n",
    // 	hyper_method, final_uri, signed_request.get_payload());
    // for h in hyper_headers.iter() {
    //     println!("{}:{}", h.name(), h.value_string());
    // }

    let mut client = Client::new();
    client.set_redirect_policy(RedirectPolicy::FollowNone);

    match signed_request.get_payload() {
        None => client.request(hyper_method, &final_uri).headers(hyper_headers).body("").send().unwrap(),
        Some(payload_contents) => client.request(hyper_method, &final_uri).headers(hyper_headers).body(payload_contents).send().unwrap(),
    }
}
