#![cfg(feature = "lambda")]

extern crate rusoto_core;
extern crate rusoto_lambda;

use rusoto_lambda::{Lambda, LambdaClient, ListFunctionsRequest, InvocationRequest, InvokeError};
use rusoto_core::Region;

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
        let request = InvocationRequest{
            function_name: "no-such-a-function".to_string(),
            invocation_type: Some("RequestResponse".to_string()),
            ..Default::default()
        };

        let result = client.invoke(request).sync();

        assert!(result.is_err());
        if let Err(InvokeError::Unknown(resp)) = result {
            assert_eq!(resp.status, 404);
        }else{
            assert!(false, format!("expect Err(InvokeError::Unknown(_), found {:?}", result));
        }
    }
    {
        // ARN with colons
        let request = InvocationRequest{
            function_name: "function:no-such-a-function".to_string(),
            ..Default::default()
        };

        let result = client.invoke(request).sync();

        assert!(result.is_err());
        if let Err(InvokeError::Unknown(resp)) = result {
            assert_eq!(resp.status, 404);
        }else{
            assert!(false, format!("expect Err(InvokeError::Unknown(_), found {:?}", result));
        }
    }
}
