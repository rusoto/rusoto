extern crate futures;
extern crate reqwest;
extern crate rusoto_core;

use futures::Future;

use rusoto_core::credential::{DefaultCredentialsProvider, ProvideAwsCredentials};
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use rusoto_core::Region;

#[test]
fn get_caller_identity_presigned() {
    let provider = DefaultCredentialsProvider::new().unwrap();
    let credentials = provider.credentials().wait().unwrap();

    let mut request = SignedRequest::new("GET", "sts", &Region::UsEast1, "/");
    let mut params = Params::new();
    params.put("Action", "GetCallerIdentity");
    params.put("Version", "2011-06-15");
    request.set_params(params);
    request.add_header("x-test-header", "foobar");
    let url =
        request.generate_presigned_url(&credentials, &std::time::Duration::from_secs(60), true);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("x-test-header", "foobar")
        .send()
        .expect("to succeed");
    assert!(
        response.status().is_success(),
        "presigned url should succeed when used"
    );
}
