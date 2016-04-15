//! AWS API requests.
//!
//! Wraps the Hyper library to send PUT, POST, DELETE and GET requests.

use std::io::Read;

use hyper::Client;
use hyper::client::Response;
use hyper::client::RedirectPolicy;
use hyper::header::Headers;
use hyper::method::Method;
use signature::SignedRequest;
use log::LogLevel::Debug;

/// Takes a fully formed and signed request and executes it.
pub fn send_request(signed_request: &SignedRequest) -> Response {
    let hyper_method = match signed_request.method().as_ref() {
        "POST" => Method::Post,
        "PUT" => Method::Put,
        "DELETE" => Method::Delete,
        "GET" | _ => Method::Get, // TODO: make the catch-all case unreachable
    };

    // translate the headers map to a format Hyper likes
    let mut hyper_headers = Headers::new();
    for h in signed_request.headers().iter() {
        hyper_headers.set_raw(h.0.to_owned(), h.1.to_owned());
    }

    let mut final_uri = format!("https://{}{}", signed_request.hostname(), signed_request.canonical_uri());
    if !signed_request.canonical_query_string().is_empty() {
        final_uri = final_uri + &format!("?{}", signed_request.canonical_query_string());
    }

    if log_enabled!(Debug) {
        let payload = signed_request.payload().map(|mut payload_bytes| {
            let mut payload_string = String::new();

            payload_bytes.read_to_string(&mut payload_string).expect(
                "Failed to read payload to string"
            );

            payload_string
        });

        debug!("Full request: \n method: {}\n final_uri: {}\n payload: {:?}\nHeaders:\n",
            hyper_method, final_uri, payload);
        for h in hyper_headers.iter() {
            debug!("{}:{}", h.name(), h.value_string());
        }
    }

    let mut client = Client::new();
    client.set_redirect_policy(RedirectPolicy::FollowNone);

    match signed_request.payload() {
        None => client.request(hyper_method, &final_uri).headers(hyper_headers).body("").send().unwrap(),
        Some(payload_contents) => client.request(hyper_method, &final_uri).headers(hyper_headers).body(payload_contents).send().unwrap(),
    }
}
