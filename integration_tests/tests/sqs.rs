#![cfg(feature = "sqs")]

extern crate env_logger;
extern crate rand;
extern crate rusoto_core;
extern crate rusoto_sqs;
extern crate time;

use std::collections::HashMap;
use std::time::Duration;

use rusoto_core::Region;
use rusoto_sqs::{CreateQueueRequest, GetQueueUrlRequest, ListQueuesRequest, SendMessageRequest};
use rusoto_sqs::{
    DeleteMessageBatchRequest, DeleteMessageBatchRequestEntry, SendMessageBatchRequest,
    SendMessageBatchRequestEntry,
};
use rusoto_sqs::{
    DeleteMessageRequest, DeleteQueueRequest, GetQueueAttributesRequest, ReceiveMessageRequest,
};
use rusoto_sqs::{Sqs, SqsClient};

#[tokio::test]
async fn list_queues() {
    let sqs = SqsClient::new(Region::UsEast1);

    let request = ListQueuesRequest {
        ..Default::default()
    };

    let result = sqs.list_queues(request).await.expect("List queues failed");
    println!("{:#?}", result);
}

#[tokio::test]
async fn sqs_roundtrip_tests() {
    let _ = env_logger::try_init();
    let sqs = SqsClient::new(Region::UsEast1);

    // create a new queue
    let q_name = &format!("test_q_{}", rand::random::<u64>());
    let mut attrs = HashMap::new();
    attrs.insert(
        String::from("ReceiveMessageWaitTimeSeconds"),
        String::from("1"),
    );
    let q_creation_req = CreateQueueRequest {
        attributes: Some(attrs),
        queue_name: q_name.clone(),
        ..Default::default()
    };

    let response = sqs
        .create_queue(q_creation_req)
        .await
        .expect("Create queue failed");
    println!(
        "Created queue {} with url {}",
        q_name,
        response
            .queue_url
            .clone()
            .expect("Queue url wasn't available in response")
    );
    // q_url_from_aws looks like https://sqs.us-east-1.amazonaws.com/acct_id_here/test_q_1495776719
    assert!(response.queue_url.unwrap().clone().ends_with(q_name));

    // query it by name
    let get_q_by_name_request = GetQueueUrlRequest {
        queue_name: q_name.clone(),
        ..Default::default()
    };
    let response = sqs
        .get_queue_url(get_q_by_name_request)
        .await
        .expect("Get queue by URL request failed");
    let queue_url = &response
        .queue_url
        .expect("Queue url should be available from list queues");
    println!(
        "Verified queue url {} for queue name {}",
        queue_url.clone(),
        q_name
    );

    // queue attributes
    let queue_attributes_req = GetQueueAttributesRequest {
        queue_url: queue_url.clone(),
        attribute_names: Some(vec!["All".to_string()]),
    };
    match sqs.get_queue_attributes(queue_attributes_req).await {
        Ok(result) => println!("Queue attributes: {:?}", result),
        Err(e) => panic!("Error getting queue attributes: {:?}", e),
    }

    // send it a message
    let msg_str = String::from("lorem ipsum dolor sit amet");
    let send_msg_request = SendMessageRequest {
        message_body: msg_str.clone(),
        queue_url: queue_url.clone(),
        ..Default::default()
    };
    let response = sqs.send_message(send_msg_request).await;
    println!(
        "Sent message with body '{}' and created message_id {}",
        msg_str,
        response.unwrap().message_id.unwrap()
    );

    // message_attribute_names is for testing https://github.com/rusoto/rusoto/issues/586
    let receive_request = ReceiveMessageRequest {
        queue_url: queue_url.clone(),
        message_attribute_names: Some(vec!["All".to_string()]),
        ..Default::default()
    };

    let response = sqs.receive_message(receive_request).await;
    for msg in response
        .expect("Expected to have a receive message response")
        .messages
        .expect("message should be available")
    {
        println!(
            "Received message '{}' with id {}",
            msg.body.clone().unwrap(),
            msg.message_id.clone().unwrap()
        );
        println!("Receipt handle is {:?}", msg.receipt_handle);

        assert!(msg.body.unwrap().eq(&msg_str));

        let delete_message_request = DeleteMessageRequest {
            queue_url: queue_url.clone(),
            receipt_handle: msg.receipt_handle.clone().unwrap(),
        };
        match sqs.delete_message(delete_message_request).await {
            Ok(_) => println!(
                "Deleted message via receipt handle {:?}",
                msg.receipt_handle
            ),
            Err(e) => panic!("Couldn't delete message: {:?}", e),
        }
    }

    let queue_deletion_req = DeleteQueueRequest {
        queue_url: queue_url.clone(),
    };
    match sqs.delete_queue(queue_deletion_req).await {
        Ok(_) => (),
        Err(e) => panic!("Couldn't delete queue: {:?}", e),
    }
    println!("Queue {} deleted", queue_url.clone());
}

#[tokio::test]
async fn sqs_timeout_test() {
    let _ = env_logger::try_init();
    let sqs = SqsClient::new(Region::UsEast1);

    let q_name = &format!("test_q_{}", rand::random::<u64>());

    let q_creation_req = CreateQueueRequest {
        queue_name: q_name.clone(),
        ..Default::default()
    };
    let response = sqs
        .create_queue(q_creation_req)
        .await
        .expect("create queue failed");
    let queue_url = response.queue_url.unwrap();
    assert!(queue_url.ends_with(q_name));

    let receive_request = ReceiveMessageRequest {
        queue_url: queue_url.clone(),
        wait_time_seconds: Some(10),
        ..Default::default()
    };
    let result = tokio::time::timeout(Duration::from_secs(2), sqs
        .receive_message(receive_request))
        .await;
    println!("sqs receive result: {:?}", result);

    let err = result.err().expect("receive did not fail as expected");
    assert!(err.to_string().find("Request timed out").is_some());

    let queue_deletion_req = DeleteQueueRequest {
        queue_url: queue_url.clone(),
    };
    sqs.delete_queue(queue_deletion_req)
        .await
        .expect("delete queue failed");
}

#[tokio::test]
async fn sqs_bulk_roundtrip_tests() {
    let _ = env_logger::try_init();
    let sqs = SqsClient::new(Region::UsEast1);

    // create a new queue
    let q_name = &format!("test_q_{}", rand::random::<u64>());
    let q_creation_req = CreateQueueRequest {
        queue_name: q_name.clone(),
        ..Default::default()
    };

    let response = sqs
        .create_queue(q_creation_req)
        .await
        .expect("Create queue failed");
    println!(
        "Created queue {} with url {}",
        q_name,
        response
            .queue_url
            .clone()
            .expect("Queue url wasn't available in response")
    );
    // q_url_from_aws looks like https://sqs.us-east-1.amazonaws.com/acct_id_here/test_q_1495776719
    assert!(response.queue_url.unwrap().clone().ends_with(q_name));

    // query it by name
    let get_q_by_name_request = GetQueueUrlRequest {
        queue_name: q_name.clone(),
        ..Default::default()
    };
    let response = sqs
        .get_queue_url(get_q_by_name_request)
        .await
        .expect("Get queue by URL request failed");
    let queue_url = &response
        .queue_url
        .expect("Queue url should be available from list queues");
    println!(
        "Verified queue url {} for queue name {}",
        queue_url.clone(),
        q_name
    );

    // queue attributes
    let queue_attributes_req = GetQueueAttributesRequest {
        queue_url: queue_url.clone(),
        attribute_names: Some(vec!["All".to_string()]),
    };
    match sqs.get_queue_attributes(queue_attributes_req).await {
        Ok(result) => println!("Queue attributes: {:?}", result),
        Err(e) => panic!("Error getting queue attributes: {:?}", e),
    }

    // send it a message
    let msg_str = String::from("lorem ipsum dolor sit amet");
    let send_msg_request_entry_1 = SendMessageBatchRequestEntry {
        message_body: msg_str.clone(),
        id: "1".to_string(),
        ..Default::default()
    };
    let send_msg_request_entry_2 = SendMessageBatchRequestEntry {
        message_body: msg_str.clone(),
        id: "2".to_string(),
        ..Default::default()
    };
    let send_msg_request = SendMessageBatchRequest {
        queue_url: queue_url.clone(),
        entries: vec![send_msg_request_entry_1, send_msg_request_entry_2],
        ..Default::default()
    };
    let response = sqs.send_message_batch(send_msg_request).await;
    println!(
        "Sent message with body '{}' and created messages {:?}",
        msg_str,
        response.unwrap()
    );

    // message_attribute_names is for testing https://github.com/rusoto/rusoto/issues/586
    let receive_request = ReceiveMessageRequest {
        queue_url: queue_url.clone(),
        message_attribute_names: Some(vec!["All".to_string()]),
        ..Default::default()
    };

    let response = sqs.receive_message(receive_request).await;
    let mut delete_entries: Vec<DeleteMessageBatchRequestEntry> = Vec::new();
    for msg in response
        .expect("Expected to have a receive message response")
        .messages
        .expect("message should be available")
    {
        println!(
            "Received message '{}' with id {}",
            msg.body.clone().unwrap(),
            msg.message_id.clone().unwrap()
        );

        assert!(msg.body.unwrap().eq(&msg_str));
        delete_entries.push(DeleteMessageBatchRequestEntry {
            receipt_handle: msg.receipt_handle.clone().unwrap(),
            id: msg.message_id.clone().unwrap(),
        });
    }
    let delete_message_request = DeleteMessageBatchRequest {
        queue_url: queue_url.clone(),
        entries: delete_entries.clone(),
    };
    match sqs.delete_message_batch(delete_message_request).await {
        Ok(_) => println!("Deleted messages via receipt handle {:?}", delete_entries),
        Err(e) => panic!("Couldn't delete message: {:?}", e),
    }

    let queue_deletion_req = DeleteQueueRequest {
        queue_url: queue_url.clone(),
    };
    match sqs.delete_queue(queue_deletion_req).await {
        Ok(_) => (),
        Err(e) => panic!("Couldn't delete queue: {:?}", e),
    }
    println!("Queue {} deleted", queue_url.clone());
}
