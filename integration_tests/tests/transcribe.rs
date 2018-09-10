#![cfg(feature = "transcribe")]

extern crate rusoto_core;
extern crate rusoto_transcribe;

use rusoto_transcribe::{Transcribe, TranscribeClient, ListVocabulariesRequest};
use rusoto_core::Region;

#[test]
fn should_list_vocabs() {
    let client = TranscribeClient::new(Region::UsEast1);
    let request = ListVocabulariesRequest::default();

    println!("{:?}", client.list_vocabularies(request).sync().unwrap());
}
