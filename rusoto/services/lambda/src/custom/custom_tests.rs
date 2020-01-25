extern crate rusoto_mock;

use bytes::Bytes;

use crate::generated::{
    GetPolicyRequest, GetPolicyResponse, InvocationRequest, Lambda, LambdaClient,
};

use self::rusoto_mock::*;
use rusoto_core::signature::{SignedRequest, SignedRequestPayload};
use rusoto_core::Region;

#[tokio::test]
async fn serialize_get_policy_response() {
    let policy = GetPolicyResponse {
        policy: Some("policy".into()),
        ..GetPolicyResponse::default()
    };
    let mock = MockRequestDispatcher::with_status(200).with_json_body(policy.clone());
    let client = LambdaClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client
        .get_policy(GetPolicyRequest {
            function_name: "test-func".into(),
            ..GetPolicyRequest::default()
        })
        .await
        .unwrap();
    assert_eq!(result, policy);
}

/// Ensures that rest-json codegen handles the response body,
/// headers, and status code properly
#[tokio::test]
async fn should_parse_invocation_response() {
    let mock = MockRequestDispatcher::with_status(200)
        .with_body(r#"{"arbitrary":"json"}"#)
        .with_header("X-Amz-Function-Error", "Handled")
        .with_header("X-Amz-Log-Result", "foo bar baz")
        .with_request_checker(|request: &SignedRequest| {
            assert_eq!("POST", request.method);
            if let Some(SignedRequestPayload::Buffer(ref buffer)) = request.payload {
                assert_eq!(b"raw payload", buffer.as_ref());
            } else {
                panic!("request payload is not a buffer");
            }
            assert_eq!("/2015-03-31/functions/foo/invocations", request.path);
        });

    let request = InvocationRequest {
        function_name: "foo".to_owned(),
        client_context: Some("context".to_owned()),
        payload: Some("raw payload".to_owned().into()),
        ..Default::default()
    };

    let client = LambdaClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.invoke(request).await.unwrap();

    assert_eq!(
        Some(Bytes::from_static(br#"{"arbitrary":"json"}"#)),
        result.payload
    );
    assert_eq!(Some("foo bar baz".to_owned()), result.log_result);
    assert_eq!(Some("Handled".to_owned()), result.function_error);
    assert_eq!(Some(200), result.status_code);
}