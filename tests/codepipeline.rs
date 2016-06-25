#![cfg(feature = "codepipeline")]

extern crate rusoto;

use rusoto::codepipeline::{CodePipelineClient, ListPipelinesInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_pipelines() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CodePipelineClient::new(credentials, Region::UsEast1);

    let request = ListPipelinesInput::default();

    match client.list_pipelines(&request) {
    	Ok(response) => {
    		println!("{:#?}", response); 
    		assert!(true)
    	},
    	Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}
