#![cfg(feature = "lambda")]

extern crate rusoto_core;
extern crate rusoto_lambda;

use rusoto_core::{Region, RusotoError};
use rusoto_lambda::{InvocationRequest, InvokeError, Lambda, LambdaClient, ListFunctionsRequest};

#[tokio::test]
async fn should_list_functions() {
    let client = LambdaClient::new(Region::UsEast1);
    let request = ListFunctionsRequest::default();

    let result = client.list_functions(request).await.unwrap();
    println!("{:#?}", result);
}

#[tokio::test]
async fn should_function_not_found() {
    let client = LambdaClient::new(Region::UsEast1);
    {
        let request = InvocationRequest {
            function_name: "no-such-a-function".to_string(),
            invocation_type: Some("RequestResponse".to_string()),
            ..Default::default()
        };

        let result = client.invoke(request).await;

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
        let request = InvocationRequest {
            function_name: "function:no-such-a-function".to_string(),
            ..Default::default()
        };

        let result = client.invoke(request).await;

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
