#![cfg(feature = "sqs")]

extern crate rusoto;
extern crate rusoto_helpers;
extern crate time;

#[macro_use]
extern crate log;
extern crate env_logger;

use rusoto::{AwsResult, ChainProvider, Region, ProvideAwsCredentials};
use rusoto_helpers::sqs::SqsHelper;
use time::get_time;

#[test]
fn main() {
    let _ = env_logger::try_init();
    let provider = ChainProvider::new().unwrap();
    let mut sqs = SqsHelper::new(provider, Region::UsWest2);

    match sqs_roundtrip_tests(&mut sqs) {
        Ok(_) => {
            println!("Everything worked.");
        }
        Err(err) => {
            panic!("Got error: {}", err);
        }
    }
}

fn sqs_roundtrip_tests <P: ProvideAwsCredentials> (sqs: &mut SqsHelper<P>) -> AwsResult<()> {
    debug!("Test logging");
    // list existing queues
    let response = sqs.list_queues()?;
    for q in response.queue_urls {
        println!("Existing queue: {:?}", q);
    }

    // create a new queue
    let q_name = &format!("test_q_{}", get_time().sec);
    let response = sqs.create_queue(q_name)?;
    println!("Created queue {} with url {:?}", q_name, response.queue_url);

    // query it by name
    let response = sqs.get_queue_url(q_name)?;
    let queue_url = response.queue_url.unwrap();
    println!("Verified queue url {:?} for queue name {}", queue_url, q_name);

    // send it a message
    let msg_str = "lorem ipsum dolor sit amet";
    let response = sqs.send_message(&queue_url, msg_str)?;
    println!("Send message with body '{}' and created message_id {:?}",
             msg_str,
             response.message_id);

    // receive a message
    let response = sqs.receive_message(&queue_url)?;
    match response.messages {
        Some(messages) => {
            for msg in messages {
                println!("Received message '{:?}' with id {}", msg.body, msg.message_id.unwrap());
                sqs.delete_message(&queue_url, &msg.receipt_handle.unwrap())?;
            }
        },
        None => println!("no messages")
    }

    // delete the queue
    sqs.delete_queue(&queue_url)?;
    println!("Queue {} deleted", &queue_url);

    Ok(())
}
