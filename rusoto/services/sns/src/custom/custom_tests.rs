extern crate rusoto_mock;

use crate::generated::*;

use std::collections::HashMap;

use self::rusoto_mock::*;
use rusoto_core::param::Params;
use rusoto_core::signature::SignedRequest;
use rusoto_core::signature::SignedRequestPayload;
use rusoto_core::Region;

#[tokio::test]
async fn should_serialize_map_parameters_in_create_platform_application_request_body() {
    let mock =
        MockRequestDispatcher::with_status(200).with_request_checker(|request: &SignedRequest| {
            assert_eq!("POST", request.method);

            if let Some(SignedRequestPayload::Buffer(ref buffer)) = request.payload {
                let params: Params = serde_urlencoded::from_bytes(buffer).unwrap();
                for idx in 1..2 {
                    let key = params.get(format!("Attributes.entry.{}.key", idx).as_str());
                    match key.unwrap().as_ref().unwrap().as_str() {
                        "PlatformCredential" => {
                            assert_eq!(
                                Some(&Some("YOUR_PLATFORM_CREDENTIAL".to_owned())),
                                params.get(format!("Attributes.entry.{}.value", idx).as_str())
                            );
                        }
                        "PlatformPrincipal" => {
                            assert_eq!(
                                Some(&Some("YOUR_PLATFORM_PRINCIPAL".to_owned())),
                                params.get(format!("Attributes.entry.{}.value", idx).as_str())
                            );
                        }
                        _ => panic!("invalid params. {:?}", params),
                    }
                }
            } else {
                panic!("Unexpected request.payload: {:?}", request.payload);
            }
        });

    let client = SnsClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let mut sns_input_attrib = HashMap::new();
    sns_input_attrib.insert(
        "PlatformCredential".to_string(),
        "YOUR_PLATFORM_CREDENTIAL".to_string(),
    );
    sns_input_attrib.insert(
        "PlatformPrincipal".to_string(),
        "YOUR_PLATFORM_PRINCIPAL".to_string(),
    );

    let sns_input = CreatePlatformApplicationInput {
        name: "example_platform_name".to_string(),
        platform: "GCM".to_string(),
        attributes: sns_input_attrib,
    };
    client.create_platform_application(sns_input).await.unwrap();
}

#[tokio::test]
async fn should_serialize_map_parameters_in_create_platform_endpoint_request_body() {
    let mock =
        MockRequestDispatcher::with_status(200).with_request_checker(|request: &SignedRequest| {
            assert_eq!("POST", request.method);

            if let Some(SignedRequestPayload::Buffer(ref buffer)) = request.payload {
                let params: Params = serde_urlencoded::from_bytes(buffer).unwrap();
                assert_eq!(
                    Some(&Some("Enabled".to_owned())),
                    params.get("Attributes.entry.1.key")
                );
                assert_eq!(
                    Some(&Some("false".to_owned())),
                    params.get("Attributes.entry.1.value")
                );
            } else {
                panic!("Unexpected request.payload: {:?}", request.payload);
            }
        });

    let client = SnsClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let mut sns_input_attrib = HashMap::new();
    sns_input_attrib.insert("Enabled".to_string(), "false".to_string());

    let sns_input = CreatePlatformEndpointInput {
        platform_application_arn: "ARN/GCM/example_platform_name".to_string(),
        token: "aaa".to_string(),
        custom_user_data: None,
        attributes: Some(sns_input_attrib),
    };
    client.create_platform_endpoint(sns_input).await.unwrap();
}

#[tokio::test]
async fn should_serialize_map_parameters_in_set_platform_application_attributes_request_body() {
    let mock =
        MockRequestDispatcher::with_status(200).with_request_checker(|request: &SignedRequest| {
            assert_eq!("POST", request.method);

            if let Some(SignedRequestPayload::Buffer(ref buffer)) = request.payload {
                let params: Params = serde_urlencoded::from_bytes(buffer).unwrap();
                assert_eq!(
                    Some(&Some("PlatformCredential".to_owned())),
                    params.get("Attributes.entry.1.key")
                );
                assert_eq!(
                    Some(&Some("YOUR_PLATFORM_CREDENTIAL".to_owned())),
                    params.get("Attributes.entry.1.value")
                );
            } else {
                panic!("Unexpected request.payload: {:?}", request.payload);
            }
        });

    let client = SnsClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let mut sns_input_attrib = HashMap::new();
    sns_input_attrib.insert(
        "PlatformCredential".to_string(),
        "YOUR_PLATFORM_CREDENTIAL".to_string(),
    );

    let sns_input = SetPlatformApplicationAttributesInput {
        platform_application_arn: "ARN/GCM/example_platform_name".to_string(),
        attributes: sns_input_attrib,
    };
    client
        .set_platform_application_attributes(sns_input)
        .await
        .unwrap();
}

#[tokio::test]
async fn should_serialize_map_parameters_in_set_endpoint_attributes_request_body() {
    let mock =
        MockRequestDispatcher::with_status(200).with_request_checker(|request: &SignedRequest| {
            assert_eq!("POST", request.method);

            if let Some(SignedRequestPayload::Buffer(ref buffer)) = request.payload {
                let params: Params = serde_urlencoded::from_bytes(buffer).unwrap();
                assert_eq!(
                    Some(&Some("Enabled".to_owned())),
                    params.get("Attributes.entry.1.key")
                );
                assert_eq!(
                    Some(&Some("false".to_owned())),
                    params.get("Attributes.entry.1.value")
                );
            } else {
                panic!("Unexpected request.payload: {:?}", request.payload);
            }
        });

    let client = SnsClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let mut sns_input_attrib = HashMap::new();
    sns_input_attrib.insert("Enabled".to_string(), "false".to_string());

    let sns_input = SetEndpointAttributesInput {
        endpoint_arn: "ARN/GCM/example_platform_name/ID".to_string(),
        attributes: sns_input_attrib,
    };
    client.set_endpoint_attributes(sns_input).await.unwrap();
}

#[tokio::test]
async fn should_serialize_map_parameters_in_get_sms_attributes_request_body() {
    let mock = MockRequestDispatcher::with_status(200).with_body(
        r#"<GetSMSAttributesResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
              <GetSMSAttributesResult>
                <attributes>
                  <entry>
                    <key>DefaultSMSType</key>
                    <value>Promotional</value>
                  </entry>
                </attributes>
              </GetSMSAttributesResult>
              <ResponseMetadata>
                <RequestId>7e20a670-be23-5b05-9364-f02cc763f409</RequestId>
              </ResponseMetadata>
            </GetSMSAttributesResponse>"#,
    );

    let client = SnsClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let sns_input = GetSMSAttributesInput {
        attributes: Some(vec!["DefaultSMSType".to_string()]),
    };
    let _ = match client.get_sms_attributes(sns_input).await {
        Ok(output) => assert_eq!(
            output.attributes.unwrap().get("DefaultSMSType"),
            Some(&("Promotional".to_string()))
        ),
        Err(e) => panic!("invalid response data: {:?}", e),
    };
}

#[tokio::test]
async fn should_serialize_map_parameters_in_set_sms_attributes_request_body() {
    let mock =
        MockRequestDispatcher::with_status(200).with_request_checker(|request: &SignedRequest| {
            assert_eq!("POST", request.method);

            if let Some(SignedRequestPayload::Buffer(ref buffer)) = request.payload {
                let params: Params = serde_urlencoded::from_bytes(buffer).unwrap();
                assert_eq!(
                    Some(&Some("DefaultSMSType".to_owned())),
                    params.get("attributes.entry.1.key")
                );
                assert_eq!(
                    Some(&Some("Promotional".to_owned())),
                    params.get("attributes.entry.1.value")
                );
            } else {
                panic!("Unexpected request.payload: {:?}", request.payload);
            }
        });

    let client = SnsClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let mut sns_input_attrib = HashMap::new();
    sns_input_attrib.insert("DefaultSMSType".to_string(), "Promotional".to_string());
    let sns_input = SetSMSAttributesInput {
        attributes: sns_input_attrib,
    };
    client.set_sms_attributes(sns_input).await.unwrap();
}

#[tokio::test]
async fn should_serialize_map_parameters_in_get_subscription_attributes_request_body() {
    let mock = MockRequestDispatcher::with_status(200)
        .with_body(
            r#"<GetSubscriptionAttributesResponse xmlns="https://sns.amazonaws.com/doc/2010-03-31/">
                <GetSubscriptionAttributesResult>
                    <Attributes>
                        <entry>
                            <key>Owner</key>
                            <value>123456789012</value>
                        </entry>
                        <entry>
                            <key>DeliveryPolicy</key>
                            <value>{&quot;healthyRetryPolicy&quot;:{&quot;numRetries&quot;:10}}</value>
                        </entry>
                        <entry>
                            <key>SubscriptionArn</key>
                            <value>arn:aws:sns:us-east-2:123456789012:My-Topic:80289ba6-0fd4-4079-afb4-ce8c8260f0ca</value>
                        </entry>
                    </Attributes>
                </GetSubscriptionAttributesResult>
                <ResponseMetadata>
                    <RequestId>057f074c-33a7-11df-9540-99d0768312d3</RequestId>
                </ResponseMetadata>
            </GetSubscriptionAttributesResponse>"#,
        );

    let client = SnsClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let sns_input = GetSubscriptionAttributesInput {
        subscription_arn: "ARN:SUBSCRIPTION_ID".to_string(),
    };
    let _ = match client.get_subscription_attributes(sns_input).await {
        Ok(output) => {
            assert_eq!(
                output.clone().attributes.unwrap().get("Owner").unwrap(),
                &("123456789012".to_string())
            );
            assert_eq!(
                output
                    .clone()
                    .attributes
                    .unwrap()
                    .get("DeliveryPolicy")
                    .unwrap(),
                &(r#"{"healthyRetryPolicy":{"numRetries":10}}"#.to_string())
            );
        }
        Err(e) => panic!("invalid response data: {:?}", e),
    };
}
