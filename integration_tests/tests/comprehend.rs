#![cfg(feature = "comprehend")]

extern crate rusoto_comprehend;
extern crate rusoto_core;

use rusoto_comprehend::{
    Comprehend, ComprehendClient, DetectSentimentRequest, LanguageCode, SentimentType,
};
use rusoto_core::Region;

#[tokio::test]
async fn should_detect_sentiment() {
    let client = ComprehendClient::new(Region::UsEast1);

    let request = DetectSentimentRequest {
        language_code: LanguageCode::En,
        text: "everything is awesome".to_owned(),
        ..Default::default()
    };

    match client.detect_sentiment(request).await {
        Ok(response) => {
            println!("{:#?}", response);
            assert_eq!(
                SentimentType::Positive,
                response
                    .sentiment
                    .expect("Should have a sentiment in response")
            );
        }
        Err(err) => panic!("Expected OK response, got {:#?}", err),
    };
}
