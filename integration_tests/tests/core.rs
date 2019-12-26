#![cfg(feature = "core")]

use rusoto_core::request::{HttpClient, HttpResponse};
use rusoto_core::credential::{DefaultCredentialsProvider, ProvideAwsCredentials};
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use rusoto_core::{Client, Region};

#[tokio::test]
async fn get_caller_identity_presigned() {
    let provider = DefaultCredentialsProvider::new().unwrap();
    let credentials = provider.credentials().await.unwrap();

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
        .await
        .expect("to succeed");
    assert!(
        response.status().is_success(),
        "presigned url should succeed when used"
    );
}

#[tokio::test]
async fn with_signature() {
    let client = Client::shared();
    let mut request = SignedRequest::new("GET", "sts", &Region::UsEast1, "/");
    let mut params = Params::new();
    params.put("Action", "GetCallerIdentity");
    params.put("Version", "2011-06-15");
    request.set_params(params);
    let response = client
        .sign_and_dispatch(request)
        .await;
    assert!(response.is_ok(), response.err());
    let response: HttpResponse = response.unwrap();
    assert!(response.status == 200, format!("Signed request should succeed with status code 200. Got status code: {:?}, headers {:?}", response.status, response.headers));
}

#[tokio::test]
async fn without_signature() {
    let client =
        Client::new_not_signing(HttpClient::new().expect("failed to create request dispatcher"));
    let mut request = SignedRequest::new("GET", "sts", &Region::UsEast1, "/");
    let mut params = Params::new();
    params.put("Action", "GetCallerIdentity");
    params.put("Version", "2011-06-15");
    request.set_params(params);
    let response = client
        .sign_and_dispatch(request)
        .await;
    assert!(response.is_ok(), response.err());
    let response: HttpResponse = response.unwrap();
    assert!(response.status == 403, format!("Unsigned API request must fail with status request 403. Got status code: {:?}, headers {:?}", response.status, response.headers));
}
