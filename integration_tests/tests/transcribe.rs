#![cfg(feature = "transcribe")]

extern crate rusoto_core;
extern crate rusoto_transcribe;

use rusoto_core::Region;
use rusoto_transcribe::{ListVocabulariesRequest, Transcribe, TranscribeClient};

#[tokio::test]
async fn should_list_vocabs() {
    let client = TranscribeClient::new(Region::UsEast1);
    let request = ListVocabulariesRequest::default();

    println!("{:?}", client.list_vocabularies(request).await.unwrap());
}
