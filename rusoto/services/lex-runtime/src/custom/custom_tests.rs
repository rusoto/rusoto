extern crate rusoto_mock;

use crate::generated::{LexRuntime, LexRuntimeClient, PostTextRequest, PostTextResponse};
use rusoto_core::Region;
use std::collections::HashMap;

use self::rusoto_mock::*;

#[tokio::test]
async fn test_post_text_resposnse_serialization() {
    let mock_resp_body = r#"{
      "dialogState": "ElicitSlot",
      "intentName": "BookCar",
      "message": "In what city do you need to rent a car?",
      "messageFormat": "PlainText",
      "responseCard": null,
      "sessionAttributes": {},
      "slotToElicit": "PickUpCity",
      "slots": {
        "CarType": null,
        "PickUpCity": "Boston"
      }
    }"#;
    let mock_request = MockRequestDispatcher::with_status(200).with_body(mock_resp_body);

    let lex_client =
        LexRuntimeClient::new_with(mock_request, MockCredentialsProvider, Region::UsEast1);

    let post_text_req = PostTextRequest {
        input_text: "Book a car".to_owned(),
        user_id: "rs".to_owned(),
        ..Default::default()
    };

    let mut slots = HashMap::new();
    slots.insert("CarType".to_owned(), None);
    slots.insert("PickUpCity".to_owned(), Some("Boston".to_owned()));

    let expected = PostTextResponse {
        dialog_state: Some("ElicitSlot".to_owned()),
        intent_name: Some("BookCar".to_owned()),
        message: Some("In what city do you need to rent a car?".to_owned()),
        message_format: Some("PlainText".to_owned()),
        slot_to_elicit: Some("PickUpCity".to_owned()),
        slots: Some(slots),
        response_card: None,
        session_attributes: Some(HashMap::new()),
        sentiment_response: None,
        session_id: None,
    };

    let result: PostTextResponse = lex_client.post_text(post_text_req).await.unwrap();
    assert_eq!(result, expected);
}