extern crate rusoto_mock;

use ::{Lambda, LambdaClient, InvocationRequest};

use rusoto_core::{Region, SignedRequest};
use self::rusoto_mock::*;

/// Ensures that rest-json codegen handles the response body,
/// headers, and status code properly
#[test]
fn should_parse_invocation_response() {
    let mock = MockRequestDispatcher::with_status(200)
        .with_body(r#"{"arbitrary":"json"}"#)
        .with_header("X-Amz-Function-Error", "Handled")
        .with_header("X-Amz-Log-Result", "foo bar baz")
        .with_request_checker(|request: &SignedRequest| {
            assert_eq!("POST", request.method);
            assert_eq!(Some("raw payload".to_owned().into_bytes()), request.payload);
            assert_eq!("/2015-03-31/functions/foo/invocations", request.path);

        });

    let request = InvocationRequest {
        function_name: "foo".to_owned(),
        client_context: Some("context".to_owned()),
        payload: Some("raw payload".to_owned().into_bytes()),
        ..Default::default()
    };

    let client = LambdaClient::new(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.invoke(&request).sync().unwrap();

    assert_eq!(Some(r#"{"arbitrary":"json"}"#.to_owned().into_bytes()), result.payload);
    assert_eq!(Some("foo bar baz".to_owned()), result.log_result);
    assert_eq!(Some("Handled".to_owned()), result.function_error);
    assert_eq!(Some(200), result.status_code);

}