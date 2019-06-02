#![cfg(feature = "lambda")]

extern crate rusoto_core;
extern crate rusoto_lambda;

use rusoto_core::{Region, RusotoError};
use rusoto_lambda::{InvokeRequest, InvokeError, Lambda, LambdaClient, ListFunctionsRequest};

#[test]
fn should_list_functions() {
    let client = LambdaClient::new(Region::UsEast1);
    let request = ListFunctionsRequest::default();

    let result = client.list_functions(request).sync().unwrap();
    println!("{:#?}", result);
}

#[test]
fn should_function_not_found() {
    let client = LambdaClient::new(Region::UsEast1);
    {
        let request = InvokeRequest {
            function_name: "no-such-a-function".to_string(),
            invocation_type: Some("RequestResponse".to_string()),
            ..Default::default()
        };

        let result = client.invoke(request).sync();

        assert!(result.is_err());
        if let Err(RusotoError::Service(InvokeError::ResourceNotFound(resp))) = result {
            assert!(resp.contains("Function not found:"));
        } else {
            assert!(
                false,
                format!(
                    "expect Err(InvokeError::ResourceNotFound(_), found {:?}",
                    result
                )
            );
        }
    }
    {
        // ARN with colons
        let request = InvokeRequest {
            function_name: "function:no-such-a-function".to_string(),
            ..Default::default()
        };

        let result = client.invoke(request).sync();

        assert!(result.is_err());
        if let Err(RusotoError::Service(InvokeError::ResourceNotFound(resp))) = result {
            assert!(resp.contains("Function not found:"));
        } else {
            assert!(
                false,
                format!(
                    "expect Err(InvokeError::ResourceNotFound(_), found {:?}",
                    result
                )
            );
        }
    }
}
