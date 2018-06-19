// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use rusoto_core::xmlerror::*;
use rusoto_core::xmlutil::{
    characters, end_element, find_start_element, peek_at_name, skip_tree, start_element,
};
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::reader::XmlEvent;
use xml::EventReader;

enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}

/// Serialize `AWSAccountIdList` contents to a `SignedRequest`.
struct AWSAccountIdListSerializer;
impl AWSAccountIdListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// Serialize `ActionNameList` contents to a `SignedRequest`.
struct ActionNameListSerializer;
impl ActionNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AddPermissionRequest {
    /// <p>The AWS account number of the <a href="http://docs.aws.amazon.com/general/latest/gr/glos-chap.html#P">principal</a> who is given permission. The principal must have an AWS account, but does not need to be signed up for Amazon SQS. For information about locating the AWS account identification, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/AWSCredentials.html">Your AWS Identifiers</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    pub aws_account_ids: Vec<String>,
    /// <p>The action the client wants to allow for the specified principal. The following values are valid:</p> <ul> <li> <p> <code>*</code> </p> </li> <li> <p> <code>ChangeMessageVisibility</code> </p> </li> <li> <p> <code>DeleteMessage</code> </p> </li> <li> <p> <code>GetQueueAttributes</code> </p> </li> <li> <p> <code>GetQueueUrl</code> </p> </li> <li> <p> <code>ReceiveMessage</code> </p> </li> <li> <p> <code>SendMessage</code> </p> </li> </ul> <p>For more information about these actions, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/acp-overview.html#PermissionTypes">Understanding Permissions</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>Specifying <code>SendMessage</code>, <code>DeleteMessage</code>, or <code>ChangeMessageVisibility</code> for <code>ActionName.n</code> also grants permissions for the corresponding batch versions of those actions: <code>SendMessageBatch</code>, <code>DeleteMessageBatch</code>, and <code>ChangeMessageVisibilityBatch</code>.</p>
    pub actions: Vec<String>,
    /// <p>The unique identification of the permission you're setting (for example, <code>AliceSendMessage</code>). Maximum 80 characters. Allowed characters include alphanumeric characters, hyphens (<code>-</code>), and underscores (<code>_</code>).</p>
    pub label: String,
    /// <p>The URL of the Amazon SQS queue to which permissions are added.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
}

/// Serialize `AddPermissionRequest` contents to a `SignedRequest`.
struct AddPermissionRequestSerializer;
impl AddPermissionRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AddPermissionRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AWSAccountIdListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "AWSAccountId"),
            &obj.aws_account_ids,
        );
        ActionNameListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ActionName"),
            &obj.actions,
        );
        params.put(
            &format!("{}{}", prefix, "Label"),
            &obj.label.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
    }
}

/// Serialize `AttributeNameList` contents to a `SignedRequest`.
struct AttributeNameListSerializer;
impl AttributeNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>This is used in the responses of batch API to give a detailed description of the result of an action on each entry in the request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BatchResultErrorEntry {
    /// <p>An error code representing why the action failed on this entry.</p>
    pub code: String,
    /// <p>The <code>Id</code> of an entry in a batch request.</p>
    pub id: String,
    /// <p>A message explaining why the action failed on this entry.</p>
    pub message: Option<String>,
    /// <p>Specifies whether the error happened due to the sender's fault.</p>
    pub sender_fault: bool,
}

struct BatchResultErrorEntryDeserializer;
impl BatchResultErrorEntryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BatchResultErrorEntry, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = BatchResultErrorEntry::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Code" => {
                        obj.code = try!(StringDeserializer::deserialize("Code", stack));
                    }
                    "Id" => {
                        obj.id = try!(StringDeserializer::deserialize("Id", stack));
                    }
                    "Message" => {
                        obj.message = Some(try!(StringDeserializer::deserialize("Message", stack)));
                    }
                    "SenderFault" => {
                        obj.sender_fault =
                            try!(BooleanDeserializer::deserialize("SenderFault", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct BatchResultErrorEntryListDeserializer;
impl BatchResultErrorEntryListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<BatchResultErrorEntry>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(BatchResultErrorEntryDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
struct BinaryDeserializer;
impl BinaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<u8>, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack)).into_bytes();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct BinaryListDeserializer;
impl BinaryListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Vec<u8>>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "BinaryListValue" {
                        obj.push(try!(BinaryDeserializer::deserialize(
                            "BinaryListValue",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `BinaryList` contents to a `SignedRequest`.
struct BinaryListSerializer;
impl BinaryListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Vec<u8>>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, ::std::str::from_utf8(&obj).unwrap());
        }
    }
}

struct BooleanDeserializer;
impl BooleanDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeMessageVisibilityBatchRequest {
    /// <p>A list of receipt handles of the messages for which the visibility timeout must be changed.</p>
    pub entries: Vec<ChangeMessageVisibilityBatchRequestEntry>,
    /// <p>The URL of the Amazon SQS queue whose messages' visibility is changed.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
}

/// Serialize `ChangeMessageVisibilityBatchRequest` contents to a `SignedRequest`.
struct ChangeMessageVisibilityBatchRequestSerializer;
impl ChangeMessageVisibilityBatchRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ChangeMessageVisibilityBatchRequestEntryListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ChangeMessageVisibilityBatchRequestEntry"),
            &obj.entries,
        );
        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
    }
}

/// <p>Encloses a receipt handle and an entry id for each message in <code> <a>ChangeMessageVisibilityBatch</a>.</code> </p> <important> <p>All of the following list parameters must be prefixed with <code>ChangeMessageVisibilityBatchRequestEntry.n</code>, where <code>n</code> is an integer value starting with <code>1</code>. For example, a parameter list for this action might look like this:</p> </important> <p> <code>&amp;amp;ChangeMessageVisibilityBatchRequestEntry.1.Id=change_visibility_msg_2</code> </p> <p> <code>&amp;amp;ChangeMessageVisibilityBatchRequestEntry.1.ReceiptHandle=&lt;replaceable&gt;Your_Receipt_Handle&lt;/replaceable&gt;</code> </p> <p> <code>&amp;amp;ChangeMessageVisibilityBatchRequestEntry.1.VisibilityTimeout=45</code> </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeMessageVisibilityBatchRequestEntry {
    /// <p><p>An identifier for this particular receipt handle used to communicate the result.</p> <note> <p>The <code>Id</code>s of a batch request need to be unique within a request</p> </note></p>
    pub id: String,
    /// <p>A receipt handle.</p>
    pub receipt_handle: String,
    /// <p>The new value (in seconds) for the message's visibility timeout.</p>
    pub visibility_timeout: Option<i64>,
}

/// Serialize `ChangeMessageVisibilityBatchRequestEntry` contents to a `SignedRequest`.
struct ChangeMessageVisibilityBatchRequestEntrySerializer;
impl ChangeMessageVisibilityBatchRequestEntrySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchRequestEntry) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Id"), &obj.id.replace("+", "%2B"));
        params.put(
            &format!("{}{}", prefix, "ReceiptHandle"),
            &obj.receipt_handle.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.visibility_timeout {
            params.put(
                &format!("{}{}", prefix, "VisibilityTimeout"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// Serialize `ChangeMessageVisibilityBatchRequestEntryList` contents to a `SignedRequest`.
struct ChangeMessageVisibilityBatchRequestEntryListSerializer;
impl ChangeMessageVisibilityBatchRequestEntryListSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &Vec<ChangeMessageVisibilityBatchRequestEntry>,
    ) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            ChangeMessageVisibilityBatchRequestEntrySerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>For each message in the batch, the response contains a <code> <a>ChangeMessageVisibilityBatchResultEntry</a> </code> tag if the message succeeds or a <code> <a>BatchResultErrorEntry</a> </code> tag if the message fails.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeMessageVisibilityBatchResult {
    /// <p>A list of <code> <a>BatchResultErrorEntry</a> </code> items.</p>
    pub failed: Vec<BatchResultErrorEntry>,
    /// <p>A list of <code> <a>ChangeMessageVisibilityBatchResultEntry</a> </code> items.</p>
    pub successful: Vec<ChangeMessageVisibilityBatchResultEntry>,
}

struct ChangeMessageVisibilityBatchResultDeserializer;
impl ChangeMessageVisibilityBatchResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ChangeMessageVisibilityBatchResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ChangeMessageVisibilityBatchResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "BatchResultErrorEntry" => {
                            obj.failed = try!(BatchResultErrorEntryListDeserializer::deserialize(
                                "BatchResultErrorEntry",
                                stack
                            ));
                        }
                        "ChangeMessageVisibilityBatchResultEntry" => {
                            obj.successful = try!(ChangeMessageVisibilityBatchResultEntryListDeserializer::deserialize("ChangeMessageVisibilityBatchResultEntry", stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Encloses the <code>Id</code> of an entry in <code> <a>ChangeMessageVisibilityBatch</a>.</code> </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeMessageVisibilityBatchResultEntry {
    /// <p>Represents a message whose visibility timeout has been changed successfully.</p>
    pub id: String,
}

struct ChangeMessageVisibilityBatchResultEntryDeserializer;
impl ChangeMessageVisibilityBatchResultEntryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ChangeMessageVisibilityBatchResultEntry, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ChangeMessageVisibilityBatchResultEntry::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Id" => {
                        obj.id = try!(StringDeserializer::deserialize("Id", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ChangeMessageVisibilityBatchResultEntryListDeserializer;
impl ChangeMessageVisibilityBatchResultEntryListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ChangeMessageVisibilityBatchResultEntry>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(
                    ChangeMessageVisibilityBatchResultEntryDeserializer::deserialize(
                        tag_name, stack
                    )
                ));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeMessageVisibilityRequest {
    /// <p>The URL of the Amazon SQS queue whose message's visibility is changed.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
    /// <p>The receipt handle associated with the message whose visibility timeout is changed. This parameter is returned by the <code> <a>ReceiveMessage</a> </code> action.</p>
    pub receipt_handle: String,
    /// <p>The new value for the message's visibility timeout (in seconds). Values values: <code>0</code> to <code>43200</code>. Maximum: 12 hours.</p>
    pub visibility_timeout: i64,
}

/// Serialize `ChangeMessageVisibilityRequest` contents to a `SignedRequest`.
struct ChangeMessageVisibilityRequestSerializer;
impl ChangeMessageVisibilityRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "ReceiptHandle"),
            &obj.receipt_handle.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "VisibilityTimeout"),
            &obj.visibility_timeout.to_string().replace("+", "%2B"),
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateQueueRequest {
    /// <p><p>A map of attributes with their corresponding values.</p> <p>The following lists the names, descriptions, and values of the special request parameters that the <code>CreateQueue</code> action uses:</p> <ul> <li> <p> <code>DelaySeconds</code> - The length of time, in seconds, for which the delivery of all messages in the queue is delayed. Valid values: An integer from 0 to 900 seconds (15 minutes). The default is 0 (zero). </p> </li> <li> <p> <code>MaximumMessageSize</code> - The limit of how many bytes a message can contain before Amazon SQS rejects it. Valid values: An integer from 1,024 bytes (1 KiB) to 262,144 bytes (256 KiB). The default is 262,144 (256 KiB). </p> </li> <li> <p> <code>MessageRetentionPeriod</code> - The length of time, in seconds, for which Amazon SQS retains a message. Valid values: An integer from 60 seconds (1 minute) to 1,209,600 seconds (14 days). The default is 345,600 (4 days). </p> </li> <li> <p> <code>Policy</code> - The queue&#39;s policy. A valid AWS policy. For more information about policy structure, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/PoliciesOverview.html">Overview of AWS IAM Policies</a> in the <i>Amazon IAM User Guide</i>. </p> </li> <li> <p> <code>ReceiveMessageWaitTimeSeconds</code> - The length of time, in seconds, for which a <code> <a>ReceiveMessage</a> </code> action waits for a message to arrive. Valid values: An integer from 0 to 20 (seconds). The default is 0 (zero). </p> </li> <li> <p> <code>RedrivePolicy</code> - The string that includes the parameters for the dead-letter queue functionality of the source queue. For more information about the redrive policy and dead-letter queues, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">Using Amazon SQS Dead-Letter Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p> <ul> <li> <p> <code>deadLetterTargetArn</code> - The Amazon Resource Name (ARN) of the dead-letter queue to which Amazon SQS moves messages after the value of <code>maxReceiveCount</code> is exceeded.</p> </li> <li> <p> <code>maxReceiveCount</code> - The number of times a message is delivered to the source queue before being moved to the dead-letter queue.</p> </li> </ul> <note> <p>The dead-letter queue of a FIFO queue must also be a FIFO queue. Similarly, the dead-letter queue of a standard queue must also be a standard queue.</p> </note> </li> <li> <p> <code>VisibilityTimeout</code> - The visibility timeout for the queue. Valid values: An integer from 0 to 43,200 (12 hours). The default is 30. For more information about the visibility timeout, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> </li> </ul> <p>The following attributes apply only to <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html">server-side-encryption</a>:</p> <ul> <li> <p> <code>KmsMasterKeyId</code> - The ID of an AWS-managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms">Key Terms</a>. While the alias of the AWS-managed CMK for Amazon SQS is always <code>alias/aws/sqs</code>, the alias of a custom CMK can, for example, be <code>alias/<i>MyAlias</i> </code>. For more examples, see <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>AWS Key Management Service API Reference</i>. </p> </li> <li> <p> <code>KmsDataKeyReusePeriodSeconds</code> - The length of time, in seconds, for which Amazon SQS can reuse a <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-keys">data key</a> to encrypt or decrypt messages before calling AWS KMS again. An integer representing seconds, between 60 seconds (1 minute) and 86,400 seconds (24 hours). The default is 300 (5 minutes). A shorter time period provides better security but results in more calls to KMS which might incur charges after Free Tier. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-how-does-the-data-key-reuse-period-work">How Does the Data Key Reuse Period Work?</a>. </p> </li> </ul> <p>The following attributes apply only to <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO (first-in-first-out) queues</a>:</p> <ul> <li> <p> <code>FifoQueue</code> - Designates a queue as FIFO. Valid values: <code>true</code>, <code>false</code>. You can provide this attribute only during queue creation. You can&#39;t change it for an existing queue. When you set this attribute, you must also provide the <code>MessageGroupId</code> for your messages explicitly.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-understanding-logic">FIFO Queue Logic</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> </li> <li> <p> <code>ContentBasedDeduplication</code> - Enables content-based deduplication. Valid values: <code>true</code>, <code>false</code>. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-exactly-once-processing">Exactly-Once Processing</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p> <ul> <li> <p>Every message must have a unique <code>MessageDeduplicationId</code>,</p> <ul> <li> <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p> </li> <li> <p>If you aren&#39;t able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message). </p> </li> <li> <p>If you don&#39;t provide a <code>MessageDeduplicationId</code> and the queue doesn&#39;t have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p> </li> <li> <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p> </li> </ul> </li> <li> <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p> </li> <li> <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered. </p> </li> </ul> </li> </ul> <p>Any other valid special request parameters (such as the following) are ignored:</p> <ul> <li> <p> <code>ApproximateNumberOfMessages</code> </p> </li> <li> <p> <code>ApproximateNumberOfMessagesDelayed</code> </p> </li> <li> <p> <code>ApproximateNumberOfMessagesNotVisible</code> </p> </li> <li> <p> <code>CreatedTimestamp</code> </p> </li> <li> <p> <code>LastModifiedTimestamp</code> </p> </li> <li> <p> <code>QueueArn</code> </p> </li> </ul></p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the new queue. The following limits apply to this name:</p> <ul> <li> <p>A queue name can have up to 80 characters.</p> </li> <li> <p>Valid values: alphanumeric characters, hyphens (<code>-</code>), and underscores (<code>_</code>).</p> </li> <li> <p>A FIFO queue name must end with the <code>.fifo</code> suffix.</p> </li> </ul> <p>Queue names are case-sensitive.</p>
    pub queue_name: String,
}

/// Serialize `CreateQueueRequest` contents to a `SignedRequest`.
struct CreateQueueRequestSerializer;
impl CreateQueueRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateQueueRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.attributes {
            QueueAttributeMapSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Attribute"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "QueueName"),
            &obj.queue_name.replace("+", "%2B"),
        );
    }
}

/// <p>Returns the <code>QueueUrl</code> attribute of the created queue.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateQueueResult {
    /// <p>The URL of the created Amazon SQS queue.</p>
    pub queue_url: Option<String>,
}

struct CreateQueueResultDeserializer;
impl CreateQueueResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateQueueResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateQueueResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "QueueUrl" => {
                        obj.queue_url =
                            Some(try!(StringDeserializer::deserialize("QueueUrl", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteMessageBatchRequest {
    /// <p>A list of receipt handles for the messages to be deleted.</p>
    pub entries: Vec<DeleteMessageBatchRequestEntry>,
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
}

/// Serialize `DeleteMessageBatchRequest` contents to a `SignedRequest`.
struct DeleteMessageBatchRequestSerializer;
impl DeleteMessageBatchRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteMessageBatchRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        DeleteMessageBatchRequestEntryListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "DeleteMessageBatchRequestEntry"),
            &obj.entries,
        );
        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
    }
}

/// <p>Encloses a receipt handle and an identifier for it.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteMessageBatchRequestEntry {
    /// <p><p>An identifier for this particular receipt handle. This is used to communicate the result.</p> <note> <p>The <code>Id</code>s of a batch request need to be unique within a request</p> </note></p>
    pub id: String,
    /// <p>A receipt handle.</p>
    pub receipt_handle: String,
}

/// Serialize `DeleteMessageBatchRequestEntry` contents to a `SignedRequest`.
struct DeleteMessageBatchRequestEntrySerializer;
impl DeleteMessageBatchRequestEntrySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteMessageBatchRequestEntry) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Id"), &obj.id.replace("+", "%2B"));
        params.put(
            &format!("{}{}", prefix, "ReceiptHandle"),
            &obj.receipt_handle.replace("+", "%2B"),
        );
    }
}

/// Serialize `DeleteMessageBatchRequestEntryList` contents to a `SignedRequest`.
struct DeleteMessageBatchRequestEntryListSerializer;
impl DeleteMessageBatchRequestEntryListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<DeleteMessageBatchRequestEntry>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            DeleteMessageBatchRequestEntrySerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>For each message in the batch, the response contains a <code> <a>DeleteMessageBatchResultEntry</a> </code> tag if the message is deleted or a <code> <a>BatchResultErrorEntry</a> </code> tag if the message can't be deleted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteMessageBatchResult {
    /// <p>A list of <code> <a>BatchResultErrorEntry</a> </code> items.</p>
    pub failed: Vec<BatchResultErrorEntry>,
    /// <p>A list of <code> <a>DeleteMessageBatchResultEntry</a> </code> items.</p>
    pub successful: Vec<DeleteMessageBatchResultEntry>,
}

struct DeleteMessageBatchResultDeserializer;
impl DeleteMessageBatchResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteMessageBatchResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteMessageBatchResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "BatchResultErrorEntry" => {
                        obj.failed = try!(BatchResultErrorEntryListDeserializer::deserialize(
                            "BatchResultErrorEntry",
                            stack
                        ));
                    }
                    "DeleteMessageBatchResultEntry" => {
                        obj.successful =
                            try!(DeleteMessageBatchResultEntryListDeserializer::deserialize(
                                "DeleteMessageBatchResultEntry",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Encloses the <code>Id</code> of an entry in <code> <a>DeleteMessageBatch</a>.</code> </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteMessageBatchResultEntry {
    /// <p>Represents a successfully deleted message.</p>
    pub id: String,
}

struct DeleteMessageBatchResultEntryDeserializer;
impl DeleteMessageBatchResultEntryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteMessageBatchResultEntry, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteMessageBatchResultEntry::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Id" => {
                        obj.id = try!(StringDeserializer::deserialize("Id", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DeleteMessageBatchResultEntryListDeserializer;
impl DeleteMessageBatchResultEntryListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DeleteMessageBatchResultEntry>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(
                    DeleteMessageBatchResultEntryDeserializer::deserialize(tag_name, stack)
                ));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteMessageRequest {
    /// <p>The URL of the Amazon SQS queue from which messages are deleted.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
    /// <p>The receipt handle associated with the message to delete.</p>
    pub receipt_handle: String,
}

/// Serialize `DeleteMessageRequest` contents to a `SignedRequest`.
struct DeleteMessageRequestSerializer;
impl DeleteMessageRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteMessageRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "ReceiptHandle"),
            &obj.receipt_handle.replace("+", "%2B"),
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteQueueRequest {
    /// <p>The URL of the Amazon SQS queue to delete.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
}

/// Serialize `DeleteQueueRequest` contents to a `SignedRequest`.
struct DeleteQueueRequestSerializer;
impl DeleteQueueRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteQueueRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetQueueAttributesRequest {
    /// <p><p>A list of attributes for which to retrieve information.</p> <note> <p>In the future, new attributes might be added. If you write code that calls this action, we recommend that you structure your code so that it can handle new attributes gracefully.</p> </note> <p>The following attributes are supported:</p> <ul> <li> <p> <code>All</code> - Returns all values. </p> </li> <li> <p> <code>ApproximateNumberOfMessages</code> - Returns the approximate number of visible messages in a queue. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-resources-required-process-messages.html">Resources Required to Process Messages</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p> </li> <li> <p> <code>ApproximateNumberOfMessagesDelayed</code> - Returns the approximate number of messages that are waiting to be added to the queue. </p> </li> <li> <p> <code>ApproximateNumberOfMessagesNotVisible</code> - Returns the approximate number of messages that have not timed-out and aren&#39;t deleted. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-resources-required-process-messages.html">Resources Required to Process Messages</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p> </li> <li> <p> <code>CreatedTimestamp</code> - Returns the time when the queue was created in seconds (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a>).</p> </li> <li> <p> <code>DelaySeconds</code> - Returns the default delay on the queue in seconds.</p> </li> <li> <p> <code>LastModifiedTimestamp</code> - Returns the time when the queue was last changed in seconds (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a>).</p> </li> <li> <p> <code>MaximumMessageSize</code> - Returns the limit of how many bytes a message can contain before Amazon SQS rejects it.</p> </li> <li> <p> <code>MessageRetentionPeriod</code> - Returns the length of time, in seconds, for which Amazon SQS retains a message.</p> </li> <li> <p> <code>Policy</code> - Returns the policy of the queue.</p> </li> <li> <p> <code>QueueArn</code> - Returns the Amazon resource name (ARN) of the queue.</p> </li> <li> <p> <code>ReceiveMessageWaitTimeSeconds</code> - Returns the length of time, in seconds, for which the <code>ReceiveMessage</code> action waits for a message to arrive. </p> </li> <li> <p> <code>RedrivePolicy</code> - Returns the string that includes the parameters for dead-letter queue functionality of the source queue. For more information about the redrive policy and dead-letter queues, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">Using Amazon SQS Dead-Letter Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p> <ul> <li> <p> <code>deadLetterTargetArn</code> - The Amazon Resource Name (ARN) of the dead-letter queue to which Amazon SQS moves messages after the value of <code>maxReceiveCount</code> is exceeded.</p> </li> <li> <p> <code>maxReceiveCount</code> - The number of times a message is delivered to the source queue before being moved to the dead-letter queue.</p> </li> </ul> </li> <li> <p> <code>VisibilityTimeout</code> - Returns the visibility timeout for the queue. For more information about the visibility timeout, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p> </li> </ul> <p>The following attributes apply only to <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html">server-side-encryption</a>:</p> <ul> <li> <p> <code>KmsMasterKeyId</code> - Returns the ID of an AWS-managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms">Key Terms</a>. </p> </li> <li> <p> <code>KmsDataKeyReusePeriodSeconds</code> - Returns the length of time, in seconds, for which Amazon SQS can reuse a data key to encrypt or decrypt messages before calling AWS KMS again. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-how-does-the-data-key-reuse-period-work">How Does the Data Key Reuse Period Work?</a>. </p> </li> </ul> <p>The following attributes apply only to <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO (first-in-first-out) queues</a>:</p> <ul> <li> <p> <code>FifoQueue</code> - Returns whether the queue is FIFO. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-understanding-logic">FIFO Queue Logic</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <note> <p>To determine whether a queue is <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO</a>, you can check whether <code>QueueName</code> ends with the <code>.fifo</code> suffix.</p> </note> </li> <li> <p> <code>ContentBasedDeduplication</code> - Returns whether content-based deduplication is enabled for the queue. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-exactly-once-processing">Exactly-Once Processing</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p> </li> </ul></p>
    pub attribute_names: Option<Vec<String>>,
    /// <p>The URL of the Amazon SQS queue whose attribute information is retrieved.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
}

/// Serialize `GetQueueAttributesRequest` contents to a `SignedRequest`.
struct GetQueueAttributesRequestSerializer;
impl GetQueueAttributesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetQueueAttributesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.attribute_names {
            AttributeNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AttributeName"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
    }
}

/// <p>A list of returned queue attributes.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetQueueAttributesResult {
    /// <p>A map of attributes to their respective values.</p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

struct GetQueueAttributesResultDeserializer;
impl GetQueueAttributesResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetQueueAttributesResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetQueueAttributesResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Attribute" => {
                        obj.attributes = Some(try!(QueueAttributeMapDeserializer::deserialize(
                            "Attribute",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetQueueUrlRequest {
    /// <p>The name of the queue whose URL must be fetched. Maximum 80 characters. Valid values: alphanumeric characters, hyphens (<code>-</code>), and underscores (<code>_</code>).</p> <p>Queue names are case-sensitive.</p>
    pub queue_name: String,
    /// <p>The AWS account ID of the account that created the queue.</p>
    pub queue_owner_aws_account_id: Option<String>,
}

/// Serialize `GetQueueUrlRequest` contents to a `SignedRequest`.
struct GetQueueUrlRequestSerializer;
impl GetQueueUrlRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetQueueUrlRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "QueueName"),
            &obj.queue_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.queue_owner_aws_account_id {
            params.put(
                &format!("{}{}", prefix, "QueueOwnerAWSAccountId"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/UnderstandingResponses.html">Responses</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetQueueUrlResult {
    /// <p>The URL of the queue.</p>
    pub queue_url: Option<String>,
}

struct GetQueueUrlResultDeserializer;
impl GetQueueUrlResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetQueueUrlResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetQueueUrlResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "QueueUrl" => {
                        obj.queue_url =
                            Some(try!(StringDeserializer::deserialize("QueueUrl", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListDeadLetterSourceQueuesRequest {
    /// <p>The URL of a dead-letter queue.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
}

/// Serialize `ListDeadLetterSourceQueuesRequest` contents to a `SignedRequest`.
struct ListDeadLetterSourceQueuesRequestSerializer;
impl ListDeadLetterSourceQueuesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListDeadLetterSourceQueuesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
    }
}

/// <p>A list of your dead letter source queues.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListDeadLetterSourceQueuesResult {
    /// <p>A list of source queue URLs that have the <code>RedrivePolicy</code> queue attribute configured with a dead-letter queue.</p>
    pub queue_urls: Vec<String>,
}

struct ListDeadLetterSourceQueuesResultDeserializer;
impl ListDeadLetterSourceQueuesResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDeadLetterSourceQueuesResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListDeadLetterSourceQueuesResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "QueueUrl" => {
                        obj.queue_urls =
                            try!(QueueUrlListDeserializer::deserialize("QueueUrl", stack));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListQueueTagsRequest {
    /// <p>The URL of the queue.</p>
    pub queue_url: String,
}

/// Serialize `ListQueueTagsRequest` contents to a `SignedRequest`.
struct ListQueueTagsRequestSerializer;
impl ListQueueTagsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListQueueTagsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListQueueTagsResult {
    /// <p>The list of all tags added to the specified queue.</p>
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

struct ListQueueTagsResultDeserializer;
impl ListQueueTagsResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListQueueTagsResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListQueueTagsResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Tag" => {
                        obj.tags = Some(try!(TagMapDeserializer::deserialize("Tag", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListQueuesRequest {
    /// <p>A string to use for filtering the list results. Only those queues whose name begins with the specified string are returned.</p> <p>Queue names are case-sensitive.</p>
    pub queue_name_prefix: Option<String>,
}

/// Serialize `ListQueuesRequest` contents to a `SignedRequest`.
struct ListQueuesRequestSerializer;
impl ListQueuesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListQueuesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.queue_name_prefix {
            params.put(
                &format!("{}{}", prefix, "QueueNamePrefix"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>A list of your queues.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListQueuesResult {
    /// <p>A list of queue URLs, up to 1,000 entries.</p>
    pub queue_urls: Option<Vec<String>>,
}

struct ListQueuesResultDeserializer;
impl ListQueuesResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListQueuesResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListQueuesResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "QueueUrl" => {
                        obj.queue_urls = Some(try!(QueueUrlListDeserializer::deserialize(
                            "QueueUrl", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>An Amazon SQS message.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Message {
    /// <p> <code>SenderId</code>, <code>SentTimestamp</code>, <code>ApproximateReceiveCount</code>, and/or <code>ApproximateFirstReceiveTimestamp</code>. <code>SentTimestamp</code> and <code>ApproximateFirstReceiveTimestamp</code> are each returned as an integer representing the <a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a> in milliseconds.</p>
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The message's contents (not URL-encoded).</p>
    pub body: Option<String>,
    /// <p>An MD5 digest of the non-URL-encoded message body string.</p>
    pub md5_of_body: Option<String>,
    /// <p>An MD5 digest of the non-URL-encoded message attribute string. You can use this attribute to verify that Amazon SQS received the message correctly. Amazon SQS URL-decodes the message before creating the MD5 digest. For information about MD5, see <a href="https://www.ietf.org/rfc/rfc1321.txt">RFC1321</a>.</p>
    pub md5_of_message_attributes: Option<String>,
    /// <p>Each message attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-attributes.html#message-attributes-items-validation">Message Attribute Items and Validation</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    pub message_attributes: Option<::std::collections::HashMap<String, MessageAttributeValue>>,
    /// <p>A unique identifier for the message. A <code>MessageId</code>is considered unique across all AWS accounts for an extended period of time.</p>
    pub message_id: Option<String>,
    /// <p>An identifier associated with the act of receiving the message. A new receipt handle is returned every time you receive a message. When deleting a message, you provide the last received receipt handle to delete the message.</p>
    pub receipt_handle: Option<String>,
}

struct MessageDeserializer;
impl MessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Message, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Message::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Attribute" => {
                        obj.attributes = Some(try!(
                            MessageSystemAttributeMapDeserializer::deserialize("Attribute", stack)
                        ));
                    }
                    "Body" => {
                        obj.body = Some(try!(StringDeserializer::deserialize("Body", stack)));
                    }
                    "MD5OfBody" => {
                        obj.md5_of_body =
                            Some(try!(StringDeserializer::deserialize("MD5OfBody", stack)));
                    }
                    "MD5OfMessageAttributes" => {
                        obj.md5_of_message_attributes = Some(try!(
                            StringDeserializer::deserialize("MD5OfMessageAttributes", stack)
                        ));
                    }
                    "MessageAttribute" => {
                        obj.message_attributes =
                            Some(try!(MessageBodyAttributeMapDeserializer::deserialize(
                                "MessageAttribute",
                                stack
                            )));
                    }
                    "MessageId" => {
                        obj.message_id =
                            Some(try!(StringDeserializer::deserialize("MessageId", stack)));
                    }
                    "ReceiptHandle" => {
                        obj.receipt_handle = Some(try!(StringDeserializer::deserialize(
                            "ReceiptHandle",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `MessageAttributeNameList` contents to a `SignedRequest`.
struct MessageAttributeNameListSerializer;
impl MessageAttributeNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>The user-specified message attribute value. For string data types, the <code>Value</code> attribute has the same restrictions on the content as the message body. For more information, see <code> <a>SendMessage</a>.</code> </p> <p> <code>Name</code>, <code>type</code>, <code>value</code> and the message body must not be empty or null. All parts of the message attribute, including <code>Name</code>, <code>Type</code>, and <code>Value</code>, are part of the message size restriction (256 KB or 262,144 bytes).</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MessageAttributeValue {
    /// <p>Not implemented. Reserved for future use.</p>
    pub binary_list_values: Option<Vec<Vec<u8>>>,
    /// <p>Binary type attributes can store any binary data, such as compressed data, encrypted data, or images.</p>
    pub binary_value: Option<Vec<u8>>,
    /// <p>Amazon SQS supports the following logical data types: <code>String</code>, <code>Number</code>, and <code>Binary</code>. For the <code>Number</code> data type, you must use <code>StringValue</code>.</p> <p>You can also append custom labels. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-attributes.html#message-attributes-data-types-validation">Message Attribute Data Types and Validation</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    pub data_type: String,
    /// <p>Not implemented. Reserved for future use.</p>
    pub string_list_values: Option<Vec<String>>,
    /// <p>Strings are Unicode with UTF-8 binary encoding. For a list of code values, see <a href="http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters">ASCII Printable Characters</a>.</p>
    pub string_value: Option<String>,
}

struct MessageAttributeValueDeserializer;
impl MessageAttributeValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MessageAttributeValue, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MessageAttributeValue::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "BinaryListValue" => {
                        obj.binary_list_values = Some(try!(BinaryListDeserializer::deserialize(
                            "BinaryListValue",
                            stack
                        )));
                    }
                    "BinaryValue" => {
                        obj.binary_value =
                            Some(try!(BinaryDeserializer::deserialize("BinaryValue", stack)));
                    }
                    "DataType" => {
                        obj.data_type = try!(StringDeserializer::deserialize("DataType", stack));
                    }
                    "StringListValue" => {
                        obj.string_list_values = Some(try!(StringListDeserializer::deserialize(
                            "StringListValue",
                            stack
                        )));
                    }
                    "StringValue" => {
                        obj.string_value =
                            Some(try!(StringDeserializer::deserialize("StringValue", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `MessageAttributeValue` contents to a `SignedRequest`.
struct MessageAttributeValueSerializer;
impl MessageAttributeValueSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &MessageAttributeValue) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.binary_list_values {
            BinaryListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "BinaryListValue"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.binary_value {
            params.put(
                &format!("{}{}", prefix, "BinaryValue"),
                ::std::str::from_utf8(&field_value)
                    .unwrap()
                    .replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "DataType"),
            &obj.data_type.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.string_list_values {
            StringListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "StringListValue"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.string_value {
            params.put(
                &format!("{}{}", prefix, "StringValue"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct MessageBodyAttributeMapDeserializer;
impl MessageBodyAttributeMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, MessageAttributeValue>, XmlParseError> {
        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "entry" {
            try!(start_element("entry", stack));
            let key = try!(StringDeserializer::deserialize("Name", stack));
            let value = try!(MessageAttributeValueDeserializer::deserialize(
                "Value", stack
            ));
            obj.insert(key, value);
            try!(end_element("entry", stack));
        }

        Ok(obj)
    }
}

/// Serialize `MessageBodyAttributeMap` contents to a `SignedRequest`.
struct MessageBodyAttributeMapSerializer;
impl MessageBodyAttributeMapSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &::std::collections::HashMap<String, MessageAttributeValue>,
    ) {
        for (index, (key, value)) in obj.iter().enumerate() {
            let prefix = format!("{}.{}", name, index + 1);
            params.put(&format!("{}.{}", prefix, "Name"), &key);
            MessageAttributeValueSerializer::serialize(
                params,
                &format!("{}.{}", prefix, "Value"),
                value,
            );
        }
    }
}

struct MessageListDeserializer;
impl MessageListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Message>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(MessageDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
struct MessageSystemAttributeMapDeserializer;
impl MessageSystemAttributeMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "Attribute" {
            try!(start_element("Attribute", stack));
            let key = try!(MessageSystemAttributeNameDeserializer::deserialize(
                "Name", stack
            ));
            let value = try!(StringDeserializer::deserialize("Value", stack));
            obj.insert(key, value);
            try!(end_element("Attribute", stack));
        }

        Ok(obj)
    }
}
struct MessageSystemAttributeNameDeserializer;
impl MessageSystemAttributeNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PurgeQueueRequest {
    /// <p>The URL of the queue from which the <code>PurgeQueue</code> action deletes messages.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
}

/// Serialize `PurgeQueueRequest` contents to a `SignedRequest`.
struct PurgeQueueRequestSerializer;
impl PurgeQueueRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PurgeQueueRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
    }
}

struct QueueAttributeMapDeserializer;
impl QueueAttributeMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "Attribute" {
            try!(start_element("Attribute", stack));
            let key = try!(QueueAttributeNameDeserializer::deserialize("Name", stack));
            let value = try!(StringDeserializer::deserialize("Value", stack));
            obj.insert(key, value);
            try!(end_element("Attribute", stack));
        }

        Ok(obj)
    }
}

/// Serialize `QueueAttributeMap` contents to a `SignedRequest`.
struct QueueAttributeMapSerializer;
impl QueueAttributeMapSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &::std::collections::HashMap<String, String>,
    ) {
        for (index, (key, value)) in obj.iter().enumerate() {
            let prefix = format!("{}.{}", name, index + 1);
            params.put(&format!("{}.{}", prefix, "Name"), &key);
            params.put(&key, &value);
        }
    }
}

struct QueueAttributeNameDeserializer;
impl QueueAttributeNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct QueueUrlListDeserializer;
impl QueueUrlListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(StringDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReceiveMessageRequest {
    /// <p><p>A list of attributes that need to be returned along with each message. These attributes include:</p> <ul> <li> <p> <code>All</code> - Returns all values.</p> </li> <li> <p> <code>ApproximateFirstReceiveTimestamp</code> - Returns the time the message was first received from the queue (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a> in milliseconds).</p> </li> <li> <p> <code>ApproximateReceiveCount</code> - Returns the number of times a message has been received from the queue but not deleted.</p> </li> <li> <p> <code>SenderId</code> </p> <ul> <li> <p>For an IAM user, returns the IAM user ID, for example <code>ABCDEFGHI1JKLMNOPQ23R</code>.</p> </li> <li> <p>For an IAM role, returns the IAM role ID, for example <code>ABCDE1F2GH3I4JK5LMNOP:i-a123b456</code>.</p> </li> </ul> </li> <li> <p> <code>SentTimestamp</code> - Returns the time the message was sent to the queue (<a href="http://en.wikipedia.org/wiki/Unix_time">epoch time</a> in milliseconds).</p> </li> <li> <p> <code>MessageDeduplicationId</code> - Returns the value provided by the sender that calls the <code> <a>SendMessage</a> </code> action.</p> </li> <li> <p> <code>MessageGroupId</code> - Returns the value provided by the sender that calls the <code> <a>SendMessage</a> </code> action. Messages with the same <code>MessageGroupId</code> are returned in sequence.</p> </li> <li> <p> <code>SequenceNumber</code> - Returns the value provided by Amazon SQS.</p> </li> </ul> <p>Any other valid special request parameters (such as the following) are ignored:</p> <ul> <li> <p> <code>ApproximateNumberOfMessages</code> </p> </li> <li> <p> <code>ApproximateNumberOfMessagesDelayed</code> </p> </li> <li> <p> <code>ApproximateNumberOfMessagesNotVisible</code> </p> </li> <li> <p> <code>CreatedTimestamp</code> </p> </li> <li> <p> <code>ContentBasedDeduplication</code> </p> </li> <li> <p> <code>DelaySeconds</code> </p> </li> <li> <p> <code>FifoQueue</code> </p> </li> <li> <p> <code>LastModifiedTimestamp</code> </p> </li> <li> <p> <code>MaximumMessageSize</code> </p> </li> <li> <p> <code>MessageRetentionPeriod</code> </p> </li> <li> <p> <code>Policy</code> </p> </li> <li> <p> <code>QueueArn</code>, </p> </li> <li> <p> <code>ReceiveMessageWaitTimeSeconds</code> </p> </li> <li> <p> <code>RedrivePolicy</code> </p> </li> <li> <p> <code>VisibilityTimeout</code> </p> </li> </ul></p>
    pub attribute_names: Option<Vec<String>>,
    /// <p>The maximum number of messages to return. Amazon SQS never returns more messages than this value (however, fewer messages might be returned). Valid values are 1 to 10. Default is 1.</p>
    pub max_number_of_messages: Option<i64>,
    /// <p>The name of the message attribute, where <i>N</i> is the index.</p> <ul> <li> <p>The name can contain alphanumeric characters and the underscore (<code>_</code>), hyphen (<code>-</code>), and period (<code>.</code>).</p> </li> <li> <p>The name is case-sensitive and must be unique among all attribute names for the message.</p> </li> <li> <p>The name must not start with AWS-reserved prefixes such as <code>AWS.</code> or <code>Amazon.</code> (or any casing variants).</p> </li> <li> <p>The name must not start or end with a period (<code>.</code>), and it should not have periods in succession (<code>..</code>).</p> </li> <li> <p>The name can be up to 256 characters long.</p> </li> </ul> <p>When using <code>ReceiveMessage</code>, you can send a list of attribute names to receive, or you can return all of the attributes by specifying <code>All</code> or <code>.*</code> in your request. You can also use all message attributes starting with a prefix, for example <code>bar.*</code>.</p>
    pub message_attribute_names: Option<Vec<String>>,
    /// <p>The URL of the Amazon SQS queue from which messages are received.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
    /// <p>This parameter applies only to FIFO (first-in-first-out) queues.</p> <p>The token used for deduplication of <code>ReceiveMessage</code> calls. If a networking issue occurs after a <code>ReceiveMessage</code> action, and instead of a response you receive a generic error, you can retry the same action with an identical <code>ReceiveRequestAttemptId</code> to retrieve the same set of messages, even if their visibility timeout has not yet expired.</p> <ul> <li> <p>You can use <code>ReceiveRequestAttemptId</code> only for 5 minutes after a <code>ReceiveMessage</code> action.</p> </li> <li> <p>When you set <code>FifoQueue</code>, a caller of the <code>ReceiveMessage</code> action can provide a <code>ReceiveRequestAttemptId</code> explicitly.</p> </li> <li> <p>If a caller of the <code>ReceiveMessage</code> action doesn't provide a <code>ReceiveRequestAttemptId</code>, Amazon SQS generates a <code>ReceiveRequestAttemptId</code>.</p> </li> <li> <p>You can retry the <code>ReceiveMessage</code> action with the same <code>ReceiveRequestAttemptId</code> if none of the messages have been modified (deleted or had their visibility changes).</p> </li> <li> <p>During a visibility timeout, subsequent calls with the same <code>ReceiveRequestAttemptId</code> return the same messages and receipt handles. If a retry occurs within the deduplication interval, it resets the visibility timeout. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <important> <p>If a caller of the <code>ReceiveMessage</code> action is still processing messages when the visibility timeout expires and messages become visible, another worker reading from the same queue can receive the same messages and therefore process duplicates. Also, if a reader whose message processing time is longer than the visibility timeout tries to delete the processed messages, the action fails with an error.</p> <p>To mitigate this effect, ensure that your application observes a safe threshold before the visibility timeout expires and extend the visibility timeout as necessary.</p> </important> </li> <li> <p>While messages with a particular <code>MessageGroupId</code> are invisible, no more messages belonging to the same <code>MessageGroupId</code> are returned until the visibility timeout expires. You can still receive messages with another <code>MessageGroupId</code> as long as it is also visible.</p> </li> <li> <p>If a caller of <code>ReceiveMessage</code> can't track the <code>ReceiveRequestAttemptId</code>, no retries work until the original visibility timeout expires. As a result, delays might occur but the messages in the queue remain in a strict order.</p> </li> </ul> <p>The length of <code>ReceiveRequestAttemptId</code> is 128 characters. <code>ReceiveRequestAttemptId</code> can contain alphanumeric characters (<code>a-z</code>, <code>A-Z</code>, <code>0-9</code>) and punctuation (<code>!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~</code>).</p> <p>For best practices of using <code>ReceiveRequestAttemptId</code>, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queue-recommendations.html#using-receiverequestattemptid-request-parameter">Using the ReceiveRequestAttemptId Request Parameter</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    pub receive_request_attempt_id: Option<String>,
    /// <p>The duration (in seconds) that the received messages are hidden from subsequent retrieve requests after being retrieved by a <code>ReceiveMessage</code> request.</p>
    pub visibility_timeout: Option<i64>,
    /// <p>The duration (in seconds) for which the call waits for a message to arrive in the queue before returning. If a message is available, the call returns sooner than <code>WaitTimeSeconds</code>. If no messages are available and the wait time expires, the call returns successfully with an empty list of messages.</p>
    pub wait_time_seconds: Option<i64>,
}

/// Serialize `ReceiveMessageRequest` contents to a `SignedRequest`.
struct ReceiveMessageRequestSerializer;
impl ReceiveMessageRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ReceiveMessageRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.attribute_names {
            AttributeNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AttributeName"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.max_number_of_messages {
            params.put(
                &format!("{}{}", prefix, "MaxNumberOfMessages"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.message_attribute_names {
            MessageAttributeNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "MessageAttributeName"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.receive_request_attempt_id {
            params.put(
                &format!("{}{}", prefix, "ReceiveRequestAttemptId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.visibility_timeout {
            params.put(
                &format!("{}{}", prefix, "VisibilityTimeout"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.wait_time_seconds {
            params.put(
                &format!("{}{}", prefix, "WaitTimeSeconds"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p>A list of received messages.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReceiveMessageResult {
    /// <p>A list of messages.</p>
    pub messages: Option<Vec<Message>>,
}

struct ReceiveMessageResultDeserializer;
impl ReceiveMessageResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReceiveMessageResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReceiveMessageResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Message" => {
                        obj.messages =
                            Some(try!(MessageListDeserializer::deserialize("Message", stack)));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RemovePermissionRequest {
    /// <p>The identification of the permission to remove. This is the label added using the <code> <a>AddPermission</a> </code> action.</p>
    pub label: String,
    /// <p>The URL of the Amazon SQS queue from which permissions are removed.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
}

/// Serialize `RemovePermissionRequest` contents to a `SignedRequest`.
struct RemovePermissionRequestSerializer;
impl RemovePermissionRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RemovePermissionRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "Label"),
            &obj.label.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SendMessageBatchRequest {
    /// <p>A list of <code> <a>SendMessageBatchRequestEntry</a> </code> items.</p>
    pub entries: Vec<SendMessageBatchRequestEntry>,
    /// <p>The URL of the Amazon SQS queue to which batched messages are sent.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
}

/// Serialize `SendMessageBatchRequest` contents to a `SignedRequest`.
struct SendMessageBatchRequestSerializer;
impl SendMessageBatchRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SendMessageBatchRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        SendMessageBatchRequestEntryListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "SendMessageBatchRequestEntry"),
            &obj.entries,
        );
        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
    }
}

/// <p>Contains the details of a single Amazon SQS message along with an <code>Id</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SendMessageBatchRequestEntry {
    /// <p><p>The length of time, in seconds, for which a specific message is delayed. Valid values: 0 to 900. Maximum: 15 minutes. Messages with a positive <code>DelaySeconds</code> value become available for processing after the delay period is finished. If you don&#39;t specify a value, the default value for the queue is applied. </p> <note> <p>When you set <code>FifoQueue</code>, you can&#39;t set <code>DelaySeconds</code> per message. You can set this parameter only on a queue level.</p> </note></p>
    pub delay_seconds: Option<i64>,
    /// <p><p>An identifier for a message in this batch used to communicate the result.</p> <note> <p>The <code>Id</code>s of a batch request need to be unique within a request</p> </note></p>
    pub id: String,
    /// <p>Each message attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-attributes.html#message-attributes-items-validation">Message Attribute Items and Validation</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    pub message_attributes: Option<::std::collections::HashMap<String, MessageAttributeValue>>,
    /// <p>The body of the message.</p>
    pub message_body: String,
    /// <p>This parameter applies only to FIFO (first-in-first-out) queues.</p> <p>The token used for deduplication of messages within a 5-minute minimum deduplication interval. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, subsequent messages with the same <code>MessageDeduplicationId</code> are accepted successfully but aren't delivered. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-exactly-once-processing"> Exactly-Once Processing</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <ul> <li> <p>Every message must have a unique <code>MessageDeduplicationId</code>,</p> <ul> <li> <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p> </li> <li> <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message). </p> </li> <li> <p>If you don't provide a <code>MessageDeduplicationId</code> and the queue doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p> </li> <li> <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p> </li> </ul> </li> <li> <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p> </li> <li> <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered. </p> </li> </ul> <note> <p>The <code>MessageDeduplicationId</code> is available to the recipient of the message (this can be useful for troubleshooting delivery issues).</p> <p>If a message is sent successfully but the acknowledgement is lost and the message is resent with the same <code>MessageDeduplicationId</code> after the deduplication interval, Amazon SQS can't detect duplicate messages.</p> </note> <p>The length of <code>MessageDeduplicationId</code> is 128 characters. <code>MessageDeduplicationId</code> can contain alphanumeric characters (<code>a-z</code>, <code>A-Z</code>, <code>0-9</code>) and punctuation (<code>!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~</code>).</p> <p>For best practices of using <code>MessageDeduplicationId</code>, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queue-recommendations.html#using-messagededuplicationid-property">Using the MessageDeduplicationId Property</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    pub message_deduplication_id: Option<String>,
    /// <p><p>This parameter applies only to FIFO (first-in-first-out) queues.</p> <p>The tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). To interleave multiple ordered streams within a single queue, use <code>MessageGroupId</code> values (for example, session data for multiple users). In this scenario, multiple readers can process the queue, but the session data of each user is processed in a FIFO fashion.</p> <ul> <li> <p>You must associate a non-empty <code>MessageGroupId</code> with a message. If you don&#39;t provide a <code>MessageGroupId</code>, the action fails.</p> </li> <li> <p> <code>ReceiveMessage</code> might return messages with multiple <code>MessageGroupId</code> values. For each <code>MessageGroupId</code>, the messages are sorted by time sent. The caller can&#39;t specify a <code>MessageGroupId</code>.</p> </li> </ul> <p>The length of <code>MessageGroupId</code> is 128 characters. Valid values are alphanumeric characters and punctuation <code>(!&quot;#$%&amp;&#39;()*+,-./:;&lt;=&gt;?@[]^_`{|}~)</code>.</p> <p>For best practices of using <code>MessageGroupId</code>, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queue-recommendations.html#using-messagegroupid-property">Using the MessageGroupId Property</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <important> <p> <code>MessageGroupId</code> is required for FIFO queues. You can&#39;t use it for Standard queues.</p> </important></p>
    pub message_group_id: Option<String>,
}

/// Serialize `SendMessageBatchRequestEntry` contents to a `SignedRequest`.
struct SendMessageBatchRequestEntrySerializer;
impl SendMessageBatchRequestEntrySerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SendMessageBatchRequestEntry) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.delay_seconds {
            params.put(
                &format!("{}{}", prefix, "DelaySeconds"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(&format!("{}{}", prefix, "Id"), &obj.id.replace("+", "%2B"));
        if let Some(ref field_value) = obj.message_attributes {
            MessageBodyAttributeMapSerializer::serialize(
                params,
                &format!("{}{}", prefix, "MessageAttribute"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "MessageBody"),
            &obj.message_body.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.message_deduplication_id {
            params.put(
                &format!("{}{}", prefix, "MessageDeduplicationId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.message_group_id {
            params.put(
                &format!("{}{}", prefix, "MessageGroupId"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// Serialize `SendMessageBatchRequestEntryList` contents to a `SignedRequest`.
struct SendMessageBatchRequestEntryListSerializer;
impl SendMessageBatchRequestEntryListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<SendMessageBatchRequestEntry>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            SendMessageBatchRequestEntrySerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>For each message in the batch, the response contains a <code> <a>SendMessageBatchResultEntry</a> </code> tag if the message succeeds or a <code> <a>BatchResultErrorEntry</a> </code> tag if the message fails.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SendMessageBatchResult {
    /// <p>A list of <code> <a>BatchResultErrorEntry</a> </code> items with error details about each message that can't be enqueued.</p>
    pub failed: Vec<BatchResultErrorEntry>,
    /// <p>A list of <code> <a>SendMessageBatchResultEntry</a> </code> items.</p>
    pub successful: Vec<SendMessageBatchResultEntry>,
}

struct SendMessageBatchResultDeserializer;
impl SendMessageBatchResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendMessageBatchResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SendMessageBatchResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "BatchResultErrorEntry" => {
                        obj.failed = try!(BatchResultErrorEntryListDeserializer::deserialize(
                            "BatchResultErrorEntry",
                            stack
                        ));
                    }
                    "SendMessageBatchResultEntry" => {
                        obj.successful =
                            try!(SendMessageBatchResultEntryListDeserializer::deserialize(
                                "SendMessageBatchResultEntry",
                                stack
                            ));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Encloses a <code>MessageId</code> for a successfully-enqueued message in a <code> <a>SendMessageBatch</a>.</code> </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SendMessageBatchResultEntry {
    /// <p>An identifier for the message in this batch.</p>
    pub id: String,
    /// <p>An MD5 digest of the non-URL-encoded message attribute string. You can use this attribute to verify that Amazon SQS received the message correctly. Amazon SQS URL-decodes the message before creating the MD5 digest. For information about MD5, see <a href="https://www.ietf.org/rfc/rfc1321.txt">RFC1321</a>.</p>
    pub md5_of_message_attributes: Option<String>,
    /// <p>An MD5 digest of the non-URL-encoded message attribute string. You can use this attribute to verify that Amazon SQS received the message correctly. Amazon SQS URL-decodes the message before creating the MD5 digest. For information about MD5, see <a href="https://www.ietf.org/rfc/rfc1321.txt">RFC1321</a>.</p>
    pub md5_of_message_body: String,
    /// <p>An identifier for the message.</p>
    pub message_id: String,
    /// <p>This parameter applies only to FIFO (first-in-first-out) queues.</p> <p>The large, non-consecutive number that Amazon SQS assigns to each message.</p> <p>The length of <code>SequenceNumber</code> is 128 bits. As <code>SequenceNumber</code> continues to increase for a particular <code>MessageGroupId</code>.</p>
    pub sequence_number: Option<String>,
}

struct SendMessageBatchResultEntryDeserializer;
impl SendMessageBatchResultEntryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendMessageBatchResultEntry, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SendMessageBatchResultEntry::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Id" => {
                        obj.id = try!(StringDeserializer::deserialize("Id", stack));
                    }
                    "MD5OfMessageAttributes" => {
                        obj.md5_of_message_attributes = Some(try!(
                            StringDeserializer::deserialize("MD5OfMessageAttributes", stack)
                        ));
                    }
                    "MD5OfMessageBody" => {
                        obj.md5_of_message_body =
                            try!(StringDeserializer::deserialize("MD5OfMessageBody", stack));
                    }
                    "MessageId" => {
                        obj.message_id = try!(StringDeserializer::deserialize("MessageId", stack));
                    }
                    "SequenceNumber" => {
                        obj.sequence_number = Some(try!(StringDeserializer::deserialize(
                            "SequenceNumber",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SendMessageBatchResultEntryListDeserializer;
impl SendMessageBatchResultEntryListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SendMessageBatchResultEntry>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(SendMessageBatchResultEntryDeserializer::deserialize(
                    tag_name, stack
                )));
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SendMessageRequest {
    /// <p><p> The length of time, in seconds, for which to delay a specific message. Valid values: 0 to 900. Maximum: 15 minutes. Messages with a positive <code>DelaySeconds</code> value become available for processing after the delay period is finished. If you don&#39;t specify a value, the default value for the queue applies. </p> <note> <p>When you set <code>FifoQueue</code>, you can&#39;t set <code>DelaySeconds</code> per message. You can set this parameter only on a queue level.</p> </note></p>
    pub delay_seconds: Option<i64>,
    /// <p>Each message attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-attributes.html#message-attributes-items-validation">Message Attribute Items and Validation</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    pub message_attributes: Option<::std::collections::HashMap<String, MessageAttributeValue>>,
    /// <p><p>The message to send. The maximum string size is 256 KB.</p> <important> <p>A message can include only XML, JSON, and unformatted text. The following Unicode characters are allowed:</p> <p> <code>#x9</code> | <code>#xA</code> | <code>#xD</code> | <code>#x20</code> to <code>#xD7FF</code> | <code>#xE000</code> to <code>#xFFFD</code> | <code>#x10000</code> to <code>#x10FFFF</code> </p> <p>Any characters not included in this list will be rejected. For more information, see the <a href="http://www.w3.org/TR/REC-xml/#charsets">W3C specification for characters</a>.</p> </important></p>
    pub message_body: String,
    /// <p>This parameter applies only to FIFO (first-in-first-out) queues.</p> <p>The token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any messages sent with the same <code>MessageDeduplicationId</code> are accepted successfully but aren't delivered during the 5-minute deduplication interval. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-exactly-once-processing"> Exactly-Once Processing</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <ul> <li> <p>Every message must have a unique <code>MessageDeduplicationId</code>,</p> <ul> <li> <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p> </li> <li> <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message). </p> </li> <li> <p>If you don't provide a <code>MessageDeduplicationId</code> and the queue doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p> </li> <li> <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p> </li> </ul> </li> <li> <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p> </li> <li> <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered. </p> </li> </ul> <note> <p>The <code>MessageDeduplicationId</code> is available to the recipient of the message (this can be useful for troubleshooting delivery issues).</p> <p>If a message is sent successfully but the acknowledgement is lost and the message is resent with the same <code>MessageDeduplicationId</code> after the deduplication interval, Amazon SQS can't detect duplicate messages.</p> </note> <p>The length of <code>MessageDeduplicationId</code> is 128 characters. <code>MessageDeduplicationId</code> can contain alphanumeric characters (<code>a-z</code>, <code>A-Z</code>, <code>0-9</code>) and punctuation (<code>!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~</code>).</p> <p>For best practices of using <code>MessageDeduplicationId</code>, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queue-recommendations.html#using-messagededuplicationid-property">Using the MessageDeduplicationId Property</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    pub message_deduplication_id: Option<String>,
    /// <p><p>This parameter applies only to FIFO (first-in-first-out) queues.</p> <p>The tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). To interleave multiple ordered streams within a single queue, use <code>MessageGroupId</code> values (for example, session data for multiple users). In this scenario, multiple readers can process the queue, but the session data of each user is processed in a FIFO fashion.</p> <ul> <li> <p>You must associate a non-empty <code>MessageGroupId</code> with a message. If you don&#39;t provide a <code>MessageGroupId</code>, the action fails.</p> </li> <li> <p> <code>ReceiveMessage</code> might return messages with multiple <code>MessageGroupId</code> values. For each <code>MessageGroupId</code>, the messages are sorted by time sent. The caller can&#39;t specify a <code>MessageGroupId</code>.</p> </li> </ul> <p>The length of <code>MessageGroupId</code> is 128 characters. Valid values are alphanumeric characters and punctuation <code>(!&quot;#$%&amp;&#39;()*+,-./:;&lt;=&gt;?@[]^_`{|}~)</code>.</p> <p>For best practices of using <code>MessageGroupId</code>, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queue-recommendations.html#using-messagegroupid-property">Using the MessageGroupId Property</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <important> <p> <code>MessageGroupId</code> is required for FIFO queues. You can&#39;t use it for Standard queues.</p> </important></p>
    pub message_group_id: Option<String>,
    /// <p>The URL of the Amazon SQS queue to which a message is sent.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
}

/// Serialize `SendMessageRequest` contents to a `SignedRequest`.
struct SendMessageRequestSerializer;
impl SendMessageRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SendMessageRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.delay_seconds {
            params.put(
                &format!("{}{}", prefix, "DelaySeconds"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.message_attributes {
            MessageBodyAttributeMapSerializer::serialize(
                params,
                &format!("{}{}", prefix, "MessageAttribute"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "MessageBody"),
            &obj.message_body.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.message_deduplication_id {
            params.put(
                &format!("{}{}", prefix, "MessageDeduplicationId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.message_group_id {
            params.put(
                &format!("{}{}", prefix, "MessageGroupId"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
    }
}

/// <p>The <code>MD5OfMessageBody</code> and <code>MessageId</code> elements.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SendMessageResult {
    /// <p>An MD5 digest of the non-URL-encoded message attribute string. You can use this attribute to verify that Amazon SQS received the message correctly. Amazon SQS URL-decodes the message before creating the MD5 digest. For information about MD5, see <a href="https://www.ietf.org/rfc/rfc1321.txt">RFC1321</a>.</p>
    pub md5_of_message_attributes: Option<String>,
    /// <p>An MD5 digest of the non-URL-encoded message attribute string. You can use this attribute to verify that Amazon SQS received the message correctly. Amazon SQS URL-decodes the message before creating the MD5 digest. For information about MD5, see <a href="https://www.ietf.org/rfc/rfc1321.txt">RFC1321</a>.</p>
    pub md5_of_message_body: Option<String>,
    /// <p>An attribute containing the <code>MessageId</code> of the message sent to the queue. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-queue-message-identifiers.html">Queue and Message Identifiers</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p>
    pub message_id: Option<String>,
    /// <p>This parameter applies only to FIFO (first-in-first-out) queues.</p> <p>The large, non-consecutive number that Amazon SQS assigns to each message.</p> <p>The length of <code>SequenceNumber</code> is 128 bits. <code>SequenceNumber</code> continues to increase for a particular <code>MessageGroupId</code>.</p>
    pub sequence_number: Option<String>,
}

struct SendMessageResultDeserializer;
impl SendMessageResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SendMessageResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SendMessageResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "MD5OfMessageAttributes" => {
                        obj.md5_of_message_attributes = Some(try!(
                            StringDeserializer::deserialize("MD5OfMessageAttributes", stack)
                        ));
                    }
                    "MD5OfMessageBody" => {
                        obj.md5_of_message_body = Some(try!(StringDeserializer::deserialize(
                            "MD5OfMessageBody",
                            stack
                        )));
                    }
                    "MessageId" => {
                        obj.message_id =
                            Some(try!(StringDeserializer::deserialize("MessageId", stack)));
                    }
                    "SequenceNumber" => {
                        obj.sequence_number = Some(try!(StringDeserializer::deserialize(
                            "SequenceNumber",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetQueueAttributesRequest {
    /// <p><p>A map of attributes to set.</p> <p>The following lists the names, descriptions, and values of the special request parameters that the <code>SetQueueAttributes</code> action uses:</p> <ul> <li> <p> <code>DelaySeconds</code> - The length of time, in seconds, for which the delivery of all messages in the queue is delayed. Valid values: An integer from 0 to 900 (15 minutes). The default is 0 (zero). </p> </li> <li> <p> <code>MaximumMessageSize</code> - The limit of how many bytes a message can contain before Amazon SQS rejects it. Valid values: An integer from 1,024 bytes (1 KiB) up to 262,144 bytes (256 KiB). The default is 262,144 (256 KiB). </p> </li> <li> <p> <code>MessageRetentionPeriod</code> - The length of time, in seconds, for which Amazon SQS retains a message. Valid values: An integer representing seconds, from 60 (1 minute) to 1,209,600 (14 days). The default is 345,600 (4 days). </p> </li> <li> <p> <code>Policy</code> - The queue&#39;s policy. A valid AWS policy. For more information about policy structure, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/PoliciesOverview.html">Overview of AWS IAM Policies</a> in the <i>Amazon IAM User Guide</i>. </p> </li> <li> <p> <code>ReceiveMessageWaitTimeSeconds</code> - The length of time, in seconds, for which a <code> <a>ReceiveMessage</a> </code> action waits for a message to arrive. Valid values: an integer from 0 to 20 (seconds). The default is 0. </p> </li> <li> <p> <code>RedrivePolicy</code> - The string that includes the parameters for the dead-letter queue functionality of the source queue. For more information about the redrive policy and dead-letter queues, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">Using Amazon SQS Dead-Letter Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p> <ul> <li> <p> <code>deadLetterTargetArn</code> - The Amazon Resource Name (ARN) of the dead-letter queue to which Amazon SQS moves messages after the value of <code>maxReceiveCount</code> is exceeded.</p> </li> <li> <p> <code>maxReceiveCount</code> - The number of times a message is delivered to the source queue before being moved to the dead-letter queue.</p> </li> </ul> <note> <p>The dead-letter queue of a FIFO queue must also be a FIFO queue. Similarly, the dead-letter queue of a standard queue must also be a standard queue.</p> </note> </li> <li> <p> <code>VisibilityTimeout</code> - The visibility timeout for the queue. Valid values: an integer from 0 to 43,200 (12 hours). The default is 30. For more information about the visibility timeout, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> </li> </ul> <p>The following attributes apply only to <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html">server-side-encryption</a>:</p> <ul> <li> <p> <code>KmsMasterKeyId</code> - The ID of an AWS-managed customer master key (CMK) for Amazon SQS or a custom CMK. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-sse-key-terms">Key Terms</a>. While the alias of the AWS-managed CMK for Amazon SQS is always <code>alias/aws/sqs</code>, the alias of a custom CMK can, for example, be <code>alias/<i>MyAlias</i> </code>. For more examples, see <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">KeyId</a> in the <i>AWS Key Management Service API Reference</i>. </p> </li> <li> <p> <code>KmsDataKeyReusePeriodSeconds</code> - The length of time, in seconds, for which Amazon SQS can reuse a <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-keys">data key</a> to encrypt or decrypt messages before calling AWS KMS again. An integer representing seconds, between 60 seconds (1 minute) and 86,400 seconds (24 hours). The default is 300 (5 minutes). A shorter time period provides better security but results in more calls to KMS which might incur charges after Free Tier. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-server-side-encryption.html#sqs-how-does-the-data-key-reuse-period-work">How Does the Data Key Reuse Period Work?</a>. </p> </li> </ul> <p>The following attribute applies only to <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO (first-in-first-out) queues</a>:</p> <ul> <li> <p> <code>ContentBasedDeduplication</code> - Enables content-based deduplication. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-exactly-once-processing">Exactly-Once Processing</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p> <ul> <li> <p>Every message must have a unique <code>MessageDeduplicationId</code>,</p> <ul> <li> <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p> </li> <li> <p>If you aren&#39;t able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message). </p> </li> <li> <p>If you don&#39;t provide a <code>MessageDeduplicationId</code> and the queue doesn&#39;t have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p> </li> <li> <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p> </li> </ul> </li> <li> <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p> </li> <li> <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered. </p> </li> </ul> </li> </ul> <p>Any other valid special request parameters (such as the following) are ignored:</p> <ul> <li> <p> <code>ApproximateNumberOfMessages</code> </p> </li> <li> <p> <code>ApproximateNumberOfMessagesDelayed</code> </p> </li> <li> <p> <code>ApproximateNumberOfMessagesNotVisible</code> </p> </li> <li> <p> <code>CreatedTimestamp</code> </p> </li> <li> <p> <code>LastModifiedTimestamp</code> </p> </li> <li> <p> <code>QueueArn</code> </p> </li> </ul></p>
    pub attributes: ::std::collections::HashMap<String, String>,
    /// <p>The URL of the Amazon SQS queue whose attributes are set.</p> <p>Queue URLs are case-sensitive.</p>
    pub queue_url: String,
}

/// Serialize `SetQueueAttributesRequest` contents to a `SignedRequest`.
struct SetQueueAttributesRequestSerializer;
impl SetQueueAttributesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetQueueAttributesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        QueueAttributeMapSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Attribute"),
            &obj.attributes,
        );
        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
    }
}

struct StringDeserializer;
impl StringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct StringListDeserializer;
impl StringListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "StringListValue" {
                        obj.push(try!(StringDeserializer::deserialize(
                            "StringListValue",
                            stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}

/// Serialize `StringList` contents to a `SignedRequest`.
struct StringListSerializer;
impl StringListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct TagKeyDeserializer;
impl TagKeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `TagKeyList` contents to a `SignedRequest`.
struct TagKeyListSerializer;
impl TagKeyListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct TagMapDeserializer;
impl TagMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, String>, XmlParseError> {
        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == "Tag" {
            try!(start_element("Tag", stack));
            let key = try!(TagKeyDeserializer::deserialize("Key", stack));
            let value = try!(TagValueDeserializer::deserialize("Value", stack));
            obj.insert(key, value);
            try!(end_element("Tag", stack));
        }

        Ok(obj)
    }
}

/// Serialize `TagMap` contents to a `SignedRequest`.
struct TagMapSerializer;
impl TagMapSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &::std::collections::HashMap<String, String>,
    ) {
        for (index, (key, value)) in obj.iter().enumerate() {
            let prefix = format!("{}.{}", name, index + 1);
            params.put(&format!("{}.{}", prefix, "Key"), &key);
            params.put(&key, &value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TagQueueRequest {
    /// <p>The URL of the queue.</p>
    pub queue_url: String,
    /// <p>The list of tags to be added to the specified queue.</p>
    pub tags: ::std::collections::HashMap<String, String>,
}

/// Serialize `TagQueueRequest` contents to a `SignedRequest`.
struct TagQueueRequestSerializer;
impl TagQueueRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TagQueueRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
        TagMapSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), &obj.tags);
    }
}

struct TagValueDeserializer;
impl TagValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UntagQueueRequest {
    /// <p>The URL of the queue.</p>
    pub queue_url: String,
    /// <p>The list of tags to be removed from the specified queue.</p>
    pub tag_keys: Vec<String>,
}

/// Serialize `UntagQueueRequest` contents to a `SignedRequest`.
struct UntagQueueRequestSerializer;
impl UntagQueueRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UntagQueueRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "QueueUrl"),
            &obj.queue_url.replace("+", "%2B"),
        );
        TagKeyListSerializer::serialize(params, &format!("{}{}", prefix, "TagKey"), &obj.tag_keys);
    }
}

/// Errors returned by AddPermission
#[derive(Debug, PartialEq)]
pub enum AddPermissionError {
    /// <p>The action that you requested would violate a limit. For example, <code>ReceiveMessage</code> returns this error if the maximum number of inflight messages is reached. <code> <a>AddPermission</a> </code> returns this error if the maximum number of permissions for the queue is reached.</p>
    OverLimit(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddPermissionError {
    pub fn from_body(body: &str) -> AddPermissionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "OverLimit" => AddPermissionError::OverLimit(String::from(parsed_error.message)),
                _ => AddPermissionError::Unknown(String::from(body)),
            },
            Err(_) => AddPermissionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for AddPermissionError {
    fn from(err: XmlParseError) -> AddPermissionError {
        let XmlParseError(message) = err;
        AddPermissionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AddPermissionError {
    fn from(err: CredentialsError) -> AddPermissionError {
        AddPermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddPermissionError {
    fn from(err: HttpDispatchError) -> AddPermissionError {
        AddPermissionError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddPermissionError {
    fn from(err: io::Error) -> AddPermissionError {
        AddPermissionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddPermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddPermissionError {
    fn description(&self) -> &str {
        match *self {
            AddPermissionError::OverLimit(ref cause) => cause,
            AddPermissionError::Validation(ref cause) => cause,
            AddPermissionError::Credentials(ref err) => err.description(),
            AddPermissionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddPermissionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ChangeMessageVisibility
#[derive(Debug, PartialEq)]
pub enum ChangeMessageVisibilityError {
    /// <p>The message referred to isn't in flight.</p>
    MessageNotInflight(String),
    /// <p>The receipt handle provided isn't valid.</p>
    ReceiptHandleIsInvalid(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ChangeMessageVisibilityError {
    pub fn from_body(body: &str) -> ChangeMessageVisibilityError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AWS.SimpleQueueService.MessageNotInflight" => {
                    ChangeMessageVisibilityError::MessageNotInflight(String::from(
                        parsed_error.message,
                    ))
                }
                "ReceiptHandleIsInvalid" => ChangeMessageVisibilityError::ReceiptHandleIsInvalid(
                    String::from(parsed_error.message),
                ),
                _ => ChangeMessageVisibilityError::Unknown(String::from(body)),
            },
            Err(_) => ChangeMessageVisibilityError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ChangeMessageVisibilityError {
    fn from(err: XmlParseError) -> ChangeMessageVisibilityError {
        let XmlParseError(message) = err;
        ChangeMessageVisibilityError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ChangeMessageVisibilityError {
    fn from(err: CredentialsError) -> ChangeMessageVisibilityError {
        ChangeMessageVisibilityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ChangeMessageVisibilityError {
    fn from(err: HttpDispatchError) -> ChangeMessageVisibilityError {
        ChangeMessageVisibilityError::HttpDispatch(err)
    }
}
impl From<io::Error> for ChangeMessageVisibilityError {
    fn from(err: io::Error) -> ChangeMessageVisibilityError {
        ChangeMessageVisibilityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ChangeMessageVisibilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ChangeMessageVisibilityError {
    fn description(&self) -> &str {
        match *self {
            ChangeMessageVisibilityError::MessageNotInflight(ref cause) => cause,
            ChangeMessageVisibilityError::ReceiptHandleIsInvalid(ref cause) => cause,
            ChangeMessageVisibilityError::Validation(ref cause) => cause,
            ChangeMessageVisibilityError::Credentials(ref err) => err.description(),
            ChangeMessageVisibilityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ChangeMessageVisibilityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ChangeMessageVisibilityBatch
#[derive(Debug, PartialEq)]
pub enum ChangeMessageVisibilityBatchError {
    /// <p>Two or more batch entries in the request have the same <code>Id</code>.</p>
    BatchEntryIdsNotDistinct(String),
    /// <p>The batch request doesn't contain any entries.</p>
    EmptyBatchRequest(String),
    /// <p>The <code>Id</code> of a batch entry in a batch request doesn't abide by the specification.</p>
    InvalidBatchEntryId(String),
    /// <p>The batch request contains more entries than permissible.</p>
    TooManyEntriesInBatchRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ChangeMessageVisibilityBatchError {
    pub fn from_body(body: &str) -> ChangeMessageVisibilityBatchError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AWS.SimpleQueueService.BatchEntryIdsNotDistinct" => {
                    ChangeMessageVisibilityBatchError::BatchEntryIdsNotDistinct(String::from(
                        parsed_error.message,
                    ))
                }
                "AWS.SimpleQueueService.EmptyBatchRequest" => {
                    ChangeMessageVisibilityBatchError::EmptyBatchRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "AWS.SimpleQueueService.InvalidBatchEntryId" => {
                    ChangeMessageVisibilityBatchError::InvalidBatchEntryId(String::from(
                        parsed_error.message,
                    ))
                }
                "AWS.SimpleQueueService.TooManyEntriesInBatchRequest" => {
                    ChangeMessageVisibilityBatchError::TooManyEntriesInBatchRequest(String::from(
                        parsed_error.message,
                    ))
                }
                _ => ChangeMessageVisibilityBatchError::Unknown(String::from(body)),
            },
            Err(_) => ChangeMessageVisibilityBatchError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ChangeMessageVisibilityBatchError {
    fn from(err: XmlParseError) -> ChangeMessageVisibilityBatchError {
        let XmlParseError(message) = err;
        ChangeMessageVisibilityBatchError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ChangeMessageVisibilityBatchError {
    fn from(err: CredentialsError) -> ChangeMessageVisibilityBatchError {
        ChangeMessageVisibilityBatchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ChangeMessageVisibilityBatchError {
    fn from(err: HttpDispatchError) -> ChangeMessageVisibilityBatchError {
        ChangeMessageVisibilityBatchError::HttpDispatch(err)
    }
}
impl From<io::Error> for ChangeMessageVisibilityBatchError {
    fn from(err: io::Error) -> ChangeMessageVisibilityBatchError {
        ChangeMessageVisibilityBatchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ChangeMessageVisibilityBatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ChangeMessageVisibilityBatchError {
    fn description(&self) -> &str {
        match *self {
            ChangeMessageVisibilityBatchError::BatchEntryIdsNotDistinct(ref cause) => cause,
            ChangeMessageVisibilityBatchError::EmptyBatchRequest(ref cause) => cause,
            ChangeMessageVisibilityBatchError::InvalidBatchEntryId(ref cause) => cause,
            ChangeMessageVisibilityBatchError::TooManyEntriesInBatchRequest(ref cause) => cause,
            ChangeMessageVisibilityBatchError::Validation(ref cause) => cause,
            ChangeMessageVisibilityBatchError::Credentials(ref err) => err.description(),
            ChangeMessageVisibilityBatchError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ChangeMessageVisibilityBatchError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateQueue
#[derive(Debug, PartialEq)]
pub enum CreateQueueError {
    /// <p>You must wait 60 seconds after deleting a queue before you can create another one with the same name.</p>
    QueueDeletedRecently(String),
    /// <p>A queue already exists with this name. Amazon SQS returns this error only if the request includes attributes whose values differ from those of the existing queue.</p>
    QueueNameExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateQueueError {
    pub fn from_body(body: &str) -> CreateQueueError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AWS.SimpleQueueService.QueueDeletedRecently" => {
                    CreateQueueError::QueueDeletedRecently(String::from(parsed_error.message))
                }
                "QueueAlreadyExists" => {
                    CreateQueueError::QueueNameExists(String::from(parsed_error.message))
                }
                _ => CreateQueueError::Unknown(String::from(body)),
            },
            Err(_) => CreateQueueError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateQueueError {
    fn from(err: XmlParseError) -> CreateQueueError {
        let XmlParseError(message) = err;
        CreateQueueError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateQueueError {
    fn from(err: CredentialsError) -> CreateQueueError {
        CreateQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateQueueError {
    fn from(err: HttpDispatchError) -> CreateQueueError {
        CreateQueueError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateQueueError {
    fn from(err: io::Error) -> CreateQueueError {
        CreateQueueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateQueueError {
    fn description(&self) -> &str {
        match *self {
            CreateQueueError::QueueDeletedRecently(ref cause) => cause,
            CreateQueueError::QueueNameExists(ref cause) => cause,
            CreateQueueError::Validation(ref cause) => cause,
            CreateQueueError::Credentials(ref err) => err.description(),
            CreateQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateQueueError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteMessage
#[derive(Debug, PartialEq)]
pub enum DeleteMessageError {
    /// <p>The receipt handle isn't valid for the current version.</p>
    InvalidIdFormat(String),
    /// <p>The receipt handle provided isn't valid.</p>
    ReceiptHandleIsInvalid(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteMessageError {
    pub fn from_body(body: &str) -> DeleteMessageError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidIdFormat" => {
                    DeleteMessageError::InvalidIdFormat(String::from(parsed_error.message))
                }
                "ReceiptHandleIsInvalid" => {
                    DeleteMessageError::ReceiptHandleIsInvalid(String::from(parsed_error.message))
                }
                _ => DeleteMessageError::Unknown(String::from(body)),
            },
            Err(_) => DeleteMessageError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteMessageError {
    fn from(err: XmlParseError) -> DeleteMessageError {
        let XmlParseError(message) = err;
        DeleteMessageError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteMessageError {
    fn from(err: CredentialsError) -> DeleteMessageError {
        DeleteMessageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteMessageError {
    fn from(err: HttpDispatchError) -> DeleteMessageError {
        DeleteMessageError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteMessageError {
    fn from(err: io::Error) -> DeleteMessageError {
        DeleteMessageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMessageError {
    fn description(&self) -> &str {
        match *self {
            DeleteMessageError::InvalidIdFormat(ref cause) => cause,
            DeleteMessageError::ReceiptHandleIsInvalid(ref cause) => cause,
            DeleteMessageError::Validation(ref cause) => cause,
            DeleteMessageError::Credentials(ref err) => err.description(),
            DeleteMessageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteMessageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteMessageBatch
#[derive(Debug, PartialEq)]
pub enum DeleteMessageBatchError {
    /// <p>Two or more batch entries in the request have the same <code>Id</code>.</p>
    BatchEntryIdsNotDistinct(String),
    /// <p>The batch request doesn't contain any entries.</p>
    EmptyBatchRequest(String),
    /// <p>The <code>Id</code> of a batch entry in a batch request doesn't abide by the specification.</p>
    InvalidBatchEntryId(String),
    /// <p>The batch request contains more entries than permissible.</p>
    TooManyEntriesInBatchRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteMessageBatchError {
    pub fn from_body(body: &str) -> DeleteMessageBatchError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AWS.SimpleQueueService.BatchEntryIdsNotDistinct" => {
                    DeleteMessageBatchError::BatchEntryIdsNotDistinct(String::from(
                        parsed_error.message,
                    ))
                }
                "AWS.SimpleQueueService.EmptyBatchRequest" => {
                    DeleteMessageBatchError::EmptyBatchRequest(String::from(parsed_error.message))
                }
                "AWS.SimpleQueueService.InvalidBatchEntryId" => {
                    DeleteMessageBatchError::InvalidBatchEntryId(String::from(parsed_error.message))
                }
                "AWS.SimpleQueueService.TooManyEntriesInBatchRequest" => {
                    DeleteMessageBatchError::TooManyEntriesInBatchRequest(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteMessageBatchError::Unknown(String::from(body)),
            },
            Err(_) => DeleteMessageBatchError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteMessageBatchError {
    fn from(err: XmlParseError) -> DeleteMessageBatchError {
        let XmlParseError(message) = err;
        DeleteMessageBatchError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteMessageBatchError {
    fn from(err: CredentialsError) -> DeleteMessageBatchError {
        DeleteMessageBatchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteMessageBatchError {
    fn from(err: HttpDispatchError) -> DeleteMessageBatchError {
        DeleteMessageBatchError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteMessageBatchError {
    fn from(err: io::Error) -> DeleteMessageBatchError {
        DeleteMessageBatchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteMessageBatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMessageBatchError {
    fn description(&self) -> &str {
        match *self {
            DeleteMessageBatchError::BatchEntryIdsNotDistinct(ref cause) => cause,
            DeleteMessageBatchError::EmptyBatchRequest(ref cause) => cause,
            DeleteMessageBatchError::InvalidBatchEntryId(ref cause) => cause,
            DeleteMessageBatchError::TooManyEntriesInBatchRequest(ref cause) => cause,
            DeleteMessageBatchError::Validation(ref cause) => cause,
            DeleteMessageBatchError::Credentials(ref err) => err.description(),
            DeleteMessageBatchError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteMessageBatchError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteQueue
#[derive(Debug, PartialEq)]
pub enum DeleteQueueError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteQueueError {
    pub fn from_body(body: &str) -> DeleteQueueError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteQueueError::Unknown(String::from(body)),
            },
            Err(_) => DeleteQueueError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for DeleteQueueError {
    fn from(err: XmlParseError) -> DeleteQueueError {
        let XmlParseError(message) = err;
        DeleteQueueError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteQueueError {
    fn from(err: CredentialsError) -> DeleteQueueError {
        DeleteQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteQueueError {
    fn from(err: HttpDispatchError) -> DeleteQueueError {
        DeleteQueueError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteQueueError {
    fn from(err: io::Error) -> DeleteQueueError {
        DeleteQueueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteQueueError {
    fn description(&self) -> &str {
        match *self {
            DeleteQueueError::Validation(ref cause) => cause,
            DeleteQueueError::Credentials(ref err) => err.description(),
            DeleteQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteQueueError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetQueueAttributes
#[derive(Debug, PartialEq)]
pub enum GetQueueAttributesError {
    /// <p>The attribute referred to doesn't exist.</p>
    InvalidAttributeName(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetQueueAttributesError {
    pub fn from_body(body: &str) -> GetQueueAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidAttributeName" => GetQueueAttributesError::InvalidAttributeName(
                    String::from(parsed_error.message),
                ),
                _ => GetQueueAttributesError::Unknown(String::from(body)),
            },
            Err(_) => GetQueueAttributesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetQueueAttributesError {
    fn from(err: XmlParseError) -> GetQueueAttributesError {
        let XmlParseError(message) = err;
        GetQueueAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetQueueAttributesError {
    fn from(err: CredentialsError) -> GetQueueAttributesError {
        GetQueueAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetQueueAttributesError {
    fn from(err: HttpDispatchError) -> GetQueueAttributesError {
        GetQueueAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetQueueAttributesError {
    fn from(err: io::Error) -> GetQueueAttributesError {
        GetQueueAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetQueueAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetQueueAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetQueueAttributesError::InvalidAttributeName(ref cause) => cause,
            GetQueueAttributesError::Validation(ref cause) => cause,
            GetQueueAttributesError::Credentials(ref err) => err.description(),
            GetQueueAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetQueueAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetQueueUrl
#[derive(Debug, PartialEq)]
pub enum GetQueueUrlError {
    /// <p>The queue referred to doesn't exist.</p>
    QueueDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetQueueUrlError {
    pub fn from_body(body: &str) -> GetQueueUrlError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AWS.SimpleQueueService.NonExistentQueue" => {
                    GetQueueUrlError::QueueDoesNotExist(String::from(parsed_error.message))
                }
                _ => GetQueueUrlError::Unknown(String::from(body)),
            },
            Err(_) => GetQueueUrlError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetQueueUrlError {
    fn from(err: XmlParseError) -> GetQueueUrlError {
        let XmlParseError(message) = err;
        GetQueueUrlError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetQueueUrlError {
    fn from(err: CredentialsError) -> GetQueueUrlError {
        GetQueueUrlError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetQueueUrlError {
    fn from(err: HttpDispatchError) -> GetQueueUrlError {
        GetQueueUrlError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetQueueUrlError {
    fn from(err: io::Error) -> GetQueueUrlError {
        GetQueueUrlError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetQueueUrlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetQueueUrlError {
    fn description(&self) -> &str {
        match *self {
            GetQueueUrlError::QueueDoesNotExist(ref cause) => cause,
            GetQueueUrlError::Validation(ref cause) => cause,
            GetQueueUrlError::Credentials(ref err) => err.description(),
            GetQueueUrlError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetQueueUrlError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDeadLetterSourceQueues
#[derive(Debug, PartialEq)]
pub enum ListDeadLetterSourceQueuesError {
    /// <p>The queue referred to doesn't exist.</p>
    QueueDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDeadLetterSourceQueuesError {
    pub fn from_body(body: &str) -> ListDeadLetterSourceQueuesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AWS.SimpleQueueService.NonExistentQueue" => {
                    ListDeadLetterSourceQueuesError::QueueDoesNotExist(String::from(
                        parsed_error.message,
                    ))
                }
                _ => ListDeadLetterSourceQueuesError::Unknown(String::from(body)),
            },
            Err(_) => ListDeadLetterSourceQueuesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListDeadLetterSourceQueuesError {
    fn from(err: XmlParseError) -> ListDeadLetterSourceQueuesError {
        let XmlParseError(message) = err;
        ListDeadLetterSourceQueuesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListDeadLetterSourceQueuesError {
    fn from(err: CredentialsError) -> ListDeadLetterSourceQueuesError {
        ListDeadLetterSourceQueuesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDeadLetterSourceQueuesError {
    fn from(err: HttpDispatchError) -> ListDeadLetterSourceQueuesError {
        ListDeadLetterSourceQueuesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDeadLetterSourceQueuesError {
    fn from(err: io::Error) -> ListDeadLetterSourceQueuesError {
        ListDeadLetterSourceQueuesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDeadLetterSourceQueuesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeadLetterSourceQueuesError {
    fn description(&self) -> &str {
        match *self {
            ListDeadLetterSourceQueuesError::QueueDoesNotExist(ref cause) => cause,
            ListDeadLetterSourceQueuesError::Validation(ref cause) => cause,
            ListDeadLetterSourceQueuesError::Credentials(ref err) => err.description(),
            ListDeadLetterSourceQueuesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDeadLetterSourceQueuesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListQueueTags
#[derive(Debug, PartialEq)]
pub enum ListQueueTagsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListQueueTagsError {
    pub fn from_body(body: &str) -> ListQueueTagsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListQueueTagsError::Unknown(String::from(body)),
            },
            Err(_) => ListQueueTagsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListQueueTagsError {
    fn from(err: XmlParseError) -> ListQueueTagsError {
        let XmlParseError(message) = err;
        ListQueueTagsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListQueueTagsError {
    fn from(err: CredentialsError) -> ListQueueTagsError {
        ListQueueTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListQueueTagsError {
    fn from(err: HttpDispatchError) -> ListQueueTagsError {
        ListQueueTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListQueueTagsError {
    fn from(err: io::Error) -> ListQueueTagsError {
        ListQueueTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListQueueTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListQueueTagsError {
    fn description(&self) -> &str {
        match *self {
            ListQueueTagsError::Validation(ref cause) => cause,
            ListQueueTagsError::Credentials(ref err) => err.description(),
            ListQueueTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListQueueTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListQueues
#[derive(Debug, PartialEq)]
pub enum ListQueuesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListQueuesError {
    pub fn from_body(body: &str) -> ListQueuesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListQueuesError::Unknown(String::from(body)),
            },
            Err(_) => ListQueuesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListQueuesError {
    fn from(err: XmlParseError) -> ListQueuesError {
        let XmlParseError(message) = err;
        ListQueuesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListQueuesError {
    fn from(err: CredentialsError) -> ListQueuesError {
        ListQueuesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListQueuesError {
    fn from(err: HttpDispatchError) -> ListQueuesError {
        ListQueuesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListQueuesError {
    fn from(err: io::Error) -> ListQueuesError {
        ListQueuesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListQueuesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListQueuesError {
    fn description(&self) -> &str {
        match *self {
            ListQueuesError::Validation(ref cause) => cause,
            ListQueuesError::Credentials(ref err) => err.description(),
            ListQueuesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListQueuesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PurgeQueue
#[derive(Debug, PartialEq)]
pub enum PurgeQueueError {
    /// <p>Indicates that the specified queue previously received a <code>PurgeQueue</code> request within the last 60 seconds (the time it can take to delete the messages in the queue).</p>
    PurgeQueueInProgress(String),
    /// <p>The queue referred to doesn't exist.</p>
    QueueDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PurgeQueueError {
    pub fn from_body(body: &str) -> PurgeQueueError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AWS.SimpleQueueService.PurgeQueueInProgress" => {
                    PurgeQueueError::PurgeQueueInProgress(String::from(parsed_error.message))
                }
                "AWS.SimpleQueueService.NonExistentQueue" => {
                    PurgeQueueError::QueueDoesNotExist(String::from(parsed_error.message))
                }
                _ => PurgeQueueError::Unknown(String::from(body)),
            },
            Err(_) => PurgeQueueError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for PurgeQueueError {
    fn from(err: XmlParseError) -> PurgeQueueError {
        let XmlParseError(message) = err;
        PurgeQueueError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PurgeQueueError {
    fn from(err: CredentialsError) -> PurgeQueueError {
        PurgeQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PurgeQueueError {
    fn from(err: HttpDispatchError) -> PurgeQueueError {
        PurgeQueueError::HttpDispatch(err)
    }
}
impl From<io::Error> for PurgeQueueError {
    fn from(err: io::Error) -> PurgeQueueError {
        PurgeQueueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PurgeQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PurgeQueueError {
    fn description(&self) -> &str {
        match *self {
            PurgeQueueError::PurgeQueueInProgress(ref cause) => cause,
            PurgeQueueError::QueueDoesNotExist(ref cause) => cause,
            PurgeQueueError::Validation(ref cause) => cause,
            PurgeQueueError::Credentials(ref err) => err.description(),
            PurgeQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PurgeQueueError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ReceiveMessage
#[derive(Debug, PartialEq)]
pub enum ReceiveMessageError {
    /// <p>The action that you requested would violate a limit. For example, <code>ReceiveMessage</code> returns this error if the maximum number of inflight messages is reached. <code> <a>AddPermission</a> </code> returns this error if the maximum number of permissions for the queue is reached.</p>
    OverLimit(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ReceiveMessageError {
    pub fn from_body(body: &str) -> ReceiveMessageError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "OverLimit" => ReceiveMessageError::OverLimit(String::from(parsed_error.message)),
                _ => ReceiveMessageError::Unknown(String::from(body)),
            },
            Err(_) => ReceiveMessageError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ReceiveMessageError {
    fn from(err: XmlParseError) -> ReceiveMessageError {
        let XmlParseError(message) = err;
        ReceiveMessageError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ReceiveMessageError {
    fn from(err: CredentialsError) -> ReceiveMessageError {
        ReceiveMessageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ReceiveMessageError {
    fn from(err: HttpDispatchError) -> ReceiveMessageError {
        ReceiveMessageError::HttpDispatch(err)
    }
}
impl From<io::Error> for ReceiveMessageError {
    fn from(err: io::Error) -> ReceiveMessageError {
        ReceiveMessageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ReceiveMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ReceiveMessageError {
    fn description(&self) -> &str {
        match *self {
            ReceiveMessageError::OverLimit(ref cause) => cause,
            ReceiveMessageError::Validation(ref cause) => cause,
            ReceiveMessageError::Credentials(ref err) => err.description(),
            ReceiveMessageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ReceiveMessageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemovePermission
#[derive(Debug, PartialEq)]
pub enum RemovePermissionError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemovePermissionError {
    pub fn from_body(body: &str) -> RemovePermissionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => RemovePermissionError::Unknown(String::from(body)),
            },
            Err(_) => RemovePermissionError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for RemovePermissionError {
    fn from(err: XmlParseError) -> RemovePermissionError {
        let XmlParseError(message) = err;
        RemovePermissionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RemovePermissionError {
    fn from(err: CredentialsError) -> RemovePermissionError {
        RemovePermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemovePermissionError {
    fn from(err: HttpDispatchError) -> RemovePermissionError {
        RemovePermissionError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemovePermissionError {
    fn from(err: io::Error) -> RemovePermissionError {
        RemovePermissionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemovePermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemovePermissionError {
    fn description(&self) -> &str {
        match *self {
            RemovePermissionError::Validation(ref cause) => cause,
            RemovePermissionError::Credentials(ref err) => err.description(),
            RemovePermissionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RemovePermissionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendMessage
#[derive(Debug, PartialEq)]
pub enum SendMessageError {
    /// <p>The message contains characters outside the allowed set.</p>
    InvalidMessageContents(String),
    /// <p>Error code 400. Unsupported operation.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendMessageError {
    pub fn from_body(body: &str) -> SendMessageError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidMessageContents" => {
                    SendMessageError::InvalidMessageContents(String::from(parsed_error.message))
                }
                "AWS.SimpleQueueService.UnsupportedOperation" => {
                    SendMessageError::UnsupportedOperation(String::from(parsed_error.message))
                }
                _ => SendMessageError::Unknown(String::from(body)),
            },
            Err(_) => SendMessageError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SendMessageError {
    fn from(err: XmlParseError) -> SendMessageError {
        let XmlParseError(message) = err;
        SendMessageError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SendMessageError {
    fn from(err: CredentialsError) -> SendMessageError {
        SendMessageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendMessageError {
    fn from(err: HttpDispatchError) -> SendMessageError {
        SendMessageError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendMessageError {
    fn from(err: io::Error) -> SendMessageError {
        SendMessageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendMessageError {
    fn description(&self) -> &str {
        match *self {
            SendMessageError::InvalidMessageContents(ref cause) => cause,
            SendMessageError::UnsupportedOperation(ref cause) => cause,
            SendMessageError::Validation(ref cause) => cause,
            SendMessageError::Credentials(ref err) => err.description(),
            SendMessageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SendMessageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendMessageBatch
#[derive(Debug, PartialEq)]
pub enum SendMessageBatchError {
    /// <p>Two or more batch entries in the request have the same <code>Id</code>.</p>
    BatchEntryIdsNotDistinct(String),
    /// <p>The length of all the messages put together is more than the limit.</p>
    BatchRequestTooLong(String),
    /// <p>The batch request doesn't contain any entries.</p>
    EmptyBatchRequest(String),
    /// <p>The <code>Id</code> of a batch entry in a batch request doesn't abide by the specification.</p>
    InvalidBatchEntryId(String),
    /// <p>The batch request contains more entries than permissible.</p>
    TooManyEntriesInBatchRequest(String),
    /// <p>Error code 400. Unsupported operation.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendMessageBatchError {
    pub fn from_body(body: &str) -> SendMessageBatchError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AWS.SimpleQueueService.BatchEntryIdsNotDistinct" => {
                    SendMessageBatchError::BatchEntryIdsNotDistinct(String::from(
                        parsed_error.message,
                    ))
                }
                "AWS.SimpleQueueService.BatchRequestTooLong" => {
                    SendMessageBatchError::BatchRequestTooLong(String::from(parsed_error.message))
                }
                "AWS.SimpleQueueService.EmptyBatchRequest" => {
                    SendMessageBatchError::EmptyBatchRequest(String::from(parsed_error.message))
                }
                "AWS.SimpleQueueService.InvalidBatchEntryId" => {
                    SendMessageBatchError::InvalidBatchEntryId(String::from(parsed_error.message))
                }
                "AWS.SimpleQueueService.TooManyEntriesInBatchRequest" => {
                    SendMessageBatchError::TooManyEntriesInBatchRequest(String::from(
                        parsed_error.message,
                    ))
                }
                "AWS.SimpleQueueService.UnsupportedOperation" => {
                    SendMessageBatchError::UnsupportedOperation(String::from(parsed_error.message))
                }
                _ => SendMessageBatchError::Unknown(String::from(body)),
            },
            Err(_) => SendMessageBatchError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SendMessageBatchError {
    fn from(err: XmlParseError) -> SendMessageBatchError {
        let XmlParseError(message) = err;
        SendMessageBatchError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SendMessageBatchError {
    fn from(err: CredentialsError) -> SendMessageBatchError {
        SendMessageBatchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendMessageBatchError {
    fn from(err: HttpDispatchError) -> SendMessageBatchError {
        SendMessageBatchError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendMessageBatchError {
    fn from(err: io::Error) -> SendMessageBatchError {
        SendMessageBatchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendMessageBatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendMessageBatchError {
    fn description(&self) -> &str {
        match *self {
            SendMessageBatchError::BatchEntryIdsNotDistinct(ref cause) => cause,
            SendMessageBatchError::BatchRequestTooLong(ref cause) => cause,
            SendMessageBatchError::EmptyBatchRequest(ref cause) => cause,
            SendMessageBatchError::InvalidBatchEntryId(ref cause) => cause,
            SendMessageBatchError::TooManyEntriesInBatchRequest(ref cause) => cause,
            SendMessageBatchError::UnsupportedOperation(ref cause) => cause,
            SendMessageBatchError::Validation(ref cause) => cause,
            SendMessageBatchError::Credentials(ref err) => err.description(),
            SendMessageBatchError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SendMessageBatchError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetQueueAttributes
#[derive(Debug, PartialEq)]
pub enum SetQueueAttributesError {
    /// <p>The attribute referred to doesn't exist.</p>
    InvalidAttributeName(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetQueueAttributesError {
    pub fn from_body(body: &str) -> SetQueueAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidAttributeName" => SetQueueAttributesError::InvalidAttributeName(
                    String::from(parsed_error.message),
                ),
                _ => SetQueueAttributesError::Unknown(String::from(body)),
            },
            Err(_) => SetQueueAttributesError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for SetQueueAttributesError {
    fn from(err: XmlParseError) -> SetQueueAttributesError {
        let XmlParseError(message) = err;
        SetQueueAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetQueueAttributesError {
    fn from(err: CredentialsError) -> SetQueueAttributesError {
        SetQueueAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetQueueAttributesError {
    fn from(err: HttpDispatchError) -> SetQueueAttributesError {
        SetQueueAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetQueueAttributesError {
    fn from(err: io::Error) -> SetQueueAttributesError {
        SetQueueAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetQueueAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetQueueAttributesError {
    fn description(&self) -> &str {
        match *self {
            SetQueueAttributesError::InvalidAttributeName(ref cause) => cause,
            SetQueueAttributesError::Validation(ref cause) => cause,
            SetQueueAttributesError::Credentials(ref err) => err.description(),
            SetQueueAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetQueueAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagQueue
#[derive(Debug, PartialEq)]
pub enum TagQueueError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TagQueueError {
    pub fn from_body(body: &str) -> TagQueueError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => TagQueueError::Unknown(String::from(body)),
            },
            Err(_) => TagQueueError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for TagQueueError {
    fn from(err: XmlParseError) -> TagQueueError {
        let XmlParseError(message) = err;
        TagQueueError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for TagQueueError {
    fn from(err: CredentialsError) -> TagQueueError {
        TagQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagQueueError {
    fn from(err: HttpDispatchError) -> TagQueueError {
        TagQueueError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagQueueError {
    fn from(err: io::Error) -> TagQueueError {
        TagQueueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagQueueError {
    fn description(&self) -> &str {
        match *self {
            TagQueueError::Validation(ref cause) => cause,
            TagQueueError::Credentials(ref err) => err.description(),
            TagQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagQueueError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagQueue
#[derive(Debug, PartialEq)]
pub enum UntagQueueError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UntagQueueError {
    pub fn from_body(body: &str) -> UntagQueueError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => UntagQueueError::Unknown(String::from(body)),
            },
            Err(_) => UntagQueueError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UntagQueueError {
    fn from(err: XmlParseError) -> UntagQueueError {
        let XmlParseError(message) = err;
        UntagQueueError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UntagQueueError {
    fn from(err: CredentialsError) -> UntagQueueError {
        UntagQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagQueueError {
    fn from(err: HttpDispatchError) -> UntagQueueError {
        UntagQueueError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagQueueError {
    fn from(err: io::Error) -> UntagQueueError {
        UntagQueueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagQueueError {
    fn description(&self) -> &str {
        match *self {
            UntagQueueError::Validation(ref cause) => cause,
            UntagQueueError::Credentials(ref err) => err.description(),
            UntagQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagQueueError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon SQS API. Amazon SQS clients implement this trait.
pub trait Sqs {
    /// <p><p>Adds a permission to a queue for a specific <a href="http://docs.aws.amazon.com/general/latest/gr/glos-chap.html#P">principal</a>. This allows sharing access to the queue.</p> <p>When you create a queue, you have full control access rights for the queue. Only you, the owner of the queue, can grant or deny permissions to the queue. For more information about these permissions, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/acp-overview.html">Shared Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <note> <p> <code>AddPermission</code> writes an Amazon-SQS-generated policy. If you want to write your own policy, use <code> <a>SetQueueAttributes</a> </code> to upload your policy. For more information about writing your own policy, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/AccessPolicyLanguage.html">Using The Access Policy Language</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>Some actions take lists of parameters. These lists are specified using the <code>param.n</code> notation. Values of <code>n</code> are integers starting from 1. For example, a parameter list with two elements looks like this:</p> <p> <code>&amp;Attribute.1=this</code> </p> <p> <code>&amp;Attribute.2=that</code> </p> </note></p>
    fn add_permission(&self, input: AddPermissionRequest) -> RusotoFuture<(), AddPermissionError>;

    /// <p><p>Changes the visibility timeout of a specified message in a queue to a new value. The maximum allowed timeout value is 12 hours. Thus, you can&#39;t extend the timeout of a message in an existing queue to more than a total visibility timeout of 12 hours. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>For example, you have a message with a visibility timeout of 5 minutes. After 3 minutes, you call <code>ChangeMessageVisiblity</code> with a timeout of 10 minutes. At that time, the timeout for the message is extended by 10 minutes beyond the time of the <code>ChangeMessageVisibility</code> action. This results in a total visibility timeout of 13 minutes. You can continue to call the <code>ChangeMessageVisibility</code> to extend the visibility timeout to a maximum of 12 hours. If you try to extend the visibility timeout beyond 12 hours, your request is rejected.</p> <p>A message is considered to be <i>in flight</i> after it&#39;s received from a queue by a consumer, but not yet deleted from the queue.</p> <p>For standard queues, there can be a maximum of 120,000 inflight messages per queue. If you reach this limit, Amazon SQS returns the <code>OverLimit</code> error message. To avoid reaching the limit, you should delete messages from the queue after they&#39;re processed. You can also increase the number of queues you use to process your messages.</p> <p>For FIFO queues, there can be a maximum of 20,000 inflight messages per queue. If you reach this limit, Amazon SQS returns no error messages.</p> <important> <p>If you attempt to set the <code>VisibilityTimeout</code> to a value greater than the maximum time left, Amazon SQS returns an error. Amazon SQS doesn&#39;t automatically recalculate and increase the timeout to the maximum remaining time.</p> <p>Unlike with a queue, when you change the visibility timeout for a specific message the timeout value is applied immediately but isn&#39;t saved in memory for that message. If you don&#39;t delete a message after it is received, the visibility timeout for the message reverts to the original timeout value (not to the value you set using the <code>ChangeMessageVisibility</code> action) the next time the message is received.</p> </important></p>
    fn change_message_visibility(
        &self,
        input: ChangeMessageVisibilityRequest,
    ) -> RusotoFuture<(), ChangeMessageVisibilityError>;

    /// <p><p>Changes the visibility timeout of multiple messages. This is a batch version of <code> <a>ChangeMessageVisibility</a>.</code> The result of the action on each message is reported individually in the response. You can send up to 10 <code> <a>ChangeMessageVisibility</a> </code> requests with each <code>ChangeMessageVisibilityBatch</code> action.</p> <important> <p>Because the batch request can result in a combination of successful and unsuccessful actions, you should check for batch errors even when the call returns an HTTP status code of <code>200</code>.</p> </important> <note> <p>Some actions take lists of parameters. These lists are specified using the <code>param.n</code> notation. Values of <code>n</code> are integers starting from 1. For example, a parameter list with two elements looks like this:</p> <p> <code>&amp;Attribute.1=this</code> </p> <p> <code>&amp;Attribute.2=that</code> </p> </note></p>
    fn change_message_visibility_batch(
        &self,
        input: ChangeMessageVisibilityBatchRequest,
    ) -> RusotoFuture<ChangeMessageVisibilityBatchResult, ChangeMessageVisibilityBatchError>;

    /// <p><p>Creates a new standard or FIFO queue. You can pass one or more attributes in the request. Keep the following caveats in mind:</p> <ul> <li> <p>If you don&#39;t specify the <code>FifoQueue</code> attribute, Amazon SQS creates a standard queue.</p> <note> <p> You can&#39;t change the queue type after you create it and you can&#39;t convert an existing standard queue into a FIFO queue. You must either create a new FIFO queue for your application or delete your existing standard queue and recreate it as a FIFO queue. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-moving"> Moving From a Standard Queue to a FIFO Queue</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p> </note> </li> <li> <p>If you don&#39;t provide a value for an attribute, the queue is created with the default value for the attribute.</p> </li> <li> <p>If you delete a queue, you must wait at least 60 seconds before creating a queue with the same name.</p> </li> </ul> <p>To successfully create a new queue, you must provide a queue name that adheres to the <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/limits-queues.html">limits related to queues</a> and is unique within the scope of your queues.</p> <p>To get the queue URL, use the <code> <a>GetQueueUrl</a> </code> action. <code> <a>GetQueueUrl</a> </code> requires only the <code>QueueName</code> parameter. be aware of existing queue names:</p> <ul> <li> <p>If you provide the name of an existing queue along with the exact names and values of all the queue&#39;s attributes, <code>CreateQueue</code> returns the queue URL for the existing queue.</p> </li> <li> <p>If the queue name, attribute names, or attribute values don&#39;t match an existing queue, <code>CreateQueue</code> returns an error.</p> </li> </ul> <note> <p>Some actions take lists of parameters. These lists are specified using the <code>param.n</code> notation. Values of <code>n</code> are integers starting from 1. For example, a parameter list with two elements looks like this:</p> <p> <code>&amp;Attribute.1=this</code> </p> <p> <code>&amp;Attribute.2=that</code> </p> </note></p>
    fn create_queue(
        &self,
        input: CreateQueueRequest,
    ) -> RusotoFuture<CreateQueueResult, CreateQueueError>;

    /// <p><p>Deletes the specified message from the specified queue. You specify the message by using the message&#39;s <i>receipt handle</i> and not the <i>MessageId</i> you receive when you send the message. Even if the message is locked by another reader due to the visibility timeout setting, it is still deleted from the queue. If you leave a message in the queue for longer than the queue&#39;s configured retention period, Amazon SQS automatically deletes the message. </p> <note> <p> The receipt handle is associated with a specific instance of receiving the message. If you receive a message more than once, the receipt handle you get each time you receive the message is different. If you don&#39;t provide the most recently received receipt handle for the message when you use the <code>DeleteMessage</code> action, the request succeeds, but the message might not be deleted.</p> <p>For standard queues, it is possible to receive a message even after you delete it. This might happen on rare occasions if one of the servers storing a copy of the message is unavailable when you send the request to delete the message. The copy remains on the server and might be returned to you on a subsequent receive request. You should ensure that your application is idempotent, so that receiving a message more than once does not cause issues.</p> </note></p>
    fn delete_message(&self, input: DeleteMessageRequest) -> RusotoFuture<(), DeleteMessageError>;

    /// <p><p>Deletes up to ten messages from the specified queue. This is a batch version of <code> <a>DeleteMessage</a>.</code> The result of the action on each message is reported individually in the response.</p> <important> <p>Because the batch request can result in a combination of successful and unsuccessful actions, you should check for batch errors even when the call returns an HTTP status code of <code>200</code>.</p> </important> <note> <p>Some actions take lists of parameters. These lists are specified using the <code>param.n</code> notation. Values of <code>n</code> are integers starting from 1. For example, a parameter list with two elements looks like this:</p> <p> <code>&amp;Attribute.1=this</code> </p> <p> <code>&amp;Attribute.2=that</code> </p> </note></p>
    fn delete_message_batch(
        &self,
        input: DeleteMessageBatchRequest,
    ) -> RusotoFuture<DeleteMessageBatchResult, DeleteMessageBatchError>;

    /// <p>Deletes the queue specified by the <code>QueueUrl</code>, regardless of the queue's contents. If the specified queue doesn't exist, Amazon SQS returns a successful response.</p> <important> <p>Be careful with the <code>DeleteQueue</code> action: When you delete a queue, any messages in the queue are no longer available. </p> </important> <p>When you delete a queue, the deletion process takes up to 60 seconds. Requests you send involving that queue during the 60 seconds might succeed. For example, a <code> <a>SendMessage</a> </code> request might succeed, but after 60 seconds the queue and the message you sent no longer exist.</p> <p>When you delete a queue, you must wait at least 60 seconds before creating a queue with the same name. </p>
    fn delete_queue(&self, input: DeleteQueueRequest) -> RusotoFuture<(), DeleteQueueError>;

    /// <p><p>Gets attributes for the specified queue.</p> <note> <p>To determine whether a queue is <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO</a>, you can check whether <code>QueueName</code> ends with the <code>.fifo</code> suffix.</p> </note> <note> <p>Some actions take lists of parameters. These lists are specified using the <code>param.n</code> notation. Values of <code>n</code> are integers starting from 1. For example, a parameter list with two elements looks like this:</p> <p> <code>&amp;Attribute.1=this</code> </p> <p> <code>&amp;Attribute.2=that</code> </p> </note></p>
    fn get_queue_attributes(
        &self,
        input: GetQueueAttributesRequest,
    ) -> RusotoFuture<GetQueueAttributesResult, GetQueueAttributesError>;

    /// <p>Returns the URL of an existing queue. This action provides a simple way to retrieve the URL of an Amazon SQS queue.</p> <p>To access a queue that belongs to another AWS account, use the <code>QueueOwnerAWSAccountId</code> parameter to specify the account ID of the queue's owner. The queue's owner must grant you permission to access the queue. For more information about shared queue access, see <code> <a>AddPermission</a> </code> or see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/acp-overview.html">Shared Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p>
    fn get_queue_url(
        &self,
        input: GetQueueUrlRequest,
    ) -> RusotoFuture<GetQueueUrlResult, GetQueueUrlError>;

    /// <p>Returns a list of your queues that have the <code>RedrivePolicy</code> queue attribute configured with a dead-letter queue.</p> <p>For more information about using dead-letter queues, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">Using Amazon SQS Dead-Letter Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    fn list_dead_letter_source_queues(
        &self,
        input: ListDeadLetterSourceQueuesRequest,
    ) -> RusotoFuture<ListDeadLetterSourceQueuesResult, ListDeadLetterSourceQueuesError>;

    /// <p>List all cost allocation tags added to the specified Amazon SQS queue. For an overview, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-tagging-queues.html">Tagging Amazon SQS Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>When you use queue tags, keep the following guidelines in mind:</p> <ul> <li> <p>Adding more than 50 tags to a queue isn't recommended.</p> </li> <li> <p>Tags don't have any semantic meaning. Amazon SQS interprets tags as character strings.</p> </li> <li> <p>Tags are case-sensitive.</p> </li> <li> <p>A new tag with a key identical to that of an existing tag overwrites the existing tag.</p> </li> <li> <p>Tagging API actions are limited to 5 TPS per AWS account. If your application requires a higher throughput, file a <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=technical">technical support request</a>.</p> </li> </ul> <p>For a full list of tag restrictions, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/limits-queues.html">Limits Related to Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    fn list_queue_tags(
        &self,
        input: ListQueueTagsRequest,
    ) -> RusotoFuture<ListQueueTagsResult, ListQueueTagsError>;

    /// <p>Returns a list of your queues. The maximum number of queues that can be returned is 1,000. If you specify a value for the optional <code>QueueNamePrefix</code> parameter, only queues with a name that begins with the specified value are returned.</p>
    fn list_queues(
        &self,
        input: ListQueuesRequest,
    ) -> RusotoFuture<ListQueuesResult, ListQueuesError>;

    /// <p>Deletes the messages in a queue specified by the <code>QueueURL</code> parameter.</p> <important> <p>When you use the <code>PurgeQueue</code> action, you can't retrieve a message deleted from a queue.</p> </important> <p>When you purge a queue, the message deletion process takes up to 60 seconds. All messages sent to the queue before calling the <code>PurgeQueue</code> action are deleted. Messages sent to the queue while it is being purged might be deleted. While the queue is being purged, messages sent to the queue before <code>PurgeQueue</code> is called might be received, but are deleted within the next minute.</p>
    fn purge_queue(&self, input: PurgeQueueRequest) -> RusotoFuture<(), PurgeQueueError>;

    /// <p><p>Retrieves one or more messages (up to 10), from the specified queue. Using the <code>WaitTimeSeconds</code> parameter enables long-poll support. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-long-polling.html">Amazon SQS Long Polling</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p> <p>Short poll is the default behavior where a weighted random set of machines is sampled on a <code>ReceiveMessage</code> call. Thus, only the messages on the sampled machines are returned. If the number of messages in the queue is small (fewer than 1,000), you most likely get fewer messages than you requested per <code>ReceiveMessage</code> call. If the number of messages in the queue is extremely small, you might not receive any messages in a particular <code>ReceiveMessage</code> response. If this happens, repeat the request. </p> <p>For each message returned, the response includes the following:</p> <ul> <li> <p>The message body.</p> </li> <li> <p>An MD5 digest of the message body. For information about MD5, see <a href="https://www.ietf.org/rfc/rfc1321.txt">RFC1321</a>.</p> </li> <li> <p>The <code>MessageId</code> you received when you sent the message to the queue.</p> </li> <li> <p>The receipt handle.</p> </li> <li> <p>The message attributes.</p> </li> <li> <p>An MD5 digest of the message attributes.</p> </li> </ul> <p>The receipt handle is the identifier you must provide when deleting the message. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-queue-message-identifiers.html">Queue and Message Identifiers</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>You can provide the <code>VisibilityTimeout</code> parameter in your request. The parameter is applied to the messages that Amazon SQS returns in the response. If you don&#39;t include the parameter, the overall visibility timeout for the queue is used for the returned messages. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>A message that isn&#39;t deleted or a message whose visibility isn&#39;t extended before the visibility timeout expires counts as a failed receive. Depending on the configuration of the queue, the message might be sent to the dead-letter queue.</p> <note> <p>In the future, new attributes might be added. If you write code that calls this action, we recommend that you structure your code so that it can handle new attributes gracefully.</p> </note></p>
    fn receive_message(
        &self,
        input: ReceiveMessageRequest,
    ) -> RusotoFuture<ReceiveMessageResult, ReceiveMessageError>;

    /// <p>Revokes any permissions in the queue policy that matches the specified <code>Label</code> parameter. Only the owner of the queue can remove permissions.</p>
    fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> RusotoFuture<(), RemovePermissionError>;

    /// <p><p>Delivers a message to the specified queue.</p> <important> <p>A message can include only XML, JSON, and unformatted text. The following Unicode characters are allowed:</p> <p> <code>#x9</code> | <code>#xA</code> | <code>#xD</code> | <code>#x20</code> to <code>#xD7FF</code> | <code>#xE000</code> to <code>#xFFFD</code> | <code>#x10000</code> to <code>#x10FFFF</code> </p> <p>Any characters not included in this list will be rejected. For more information, see the <a href="http://www.w3.org/TR/REC-xml/#charsets">W3C specification for characters</a>.</p> </important></p>
    fn send_message(
        &self,
        input: SendMessageRequest,
    ) -> RusotoFuture<SendMessageResult, SendMessageError>;

    /// <p><p>Delivers up to ten messages to the specified queue. This is a batch version of <code> <a>SendMessage</a>.</code> For a FIFO queue, multiple messages within a single batch are enqueued in the order they are sent.</p> <p>The result of sending each message is reported individually in the response. Because the batch request can result in a combination of successful and unsuccessful actions, you should check for batch errors even when the call returns an HTTP status code of <code>200</code>.</p> <p>The maximum allowed individual message size and the maximum total payload size (the sum of the individual lengths of all of the batched messages) are both 256 KB (262,144 bytes).</p> <important> <p>A message can include only XML, JSON, and unformatted text. The following Unicode characters are allowed:</p> <p> <code>#x9</code> | <code>#xA</code> | <code>#xD</code> | <code>#x20</code> to <code>#xD7FF</code> | <code>#xE000</code> to <code>#xFFFD</code> | <code>#x10000</code> to <code>#x10FFFF</code> </p> <p>Any characters not included in this list will be rejected. For more information, see the <a href="http://www.w3.org/TR/REC-xml/#charsets">W3C specification for characters</a>.</p> </important> <p>If you don&#39;t specify the <code>DelaySeconds</code> parameter for an entry, Amazon SQS uses the default value for the queue.</p> <note> <p>Some actions take lists of parameters. These lists are specified using the <code>param.n</code> notation. Values of <code>n</code> are integers starting from 1. For example, a parameter list with two elements looks like this:</p> <p> <code>&amp;Attribute.1=this</code> </p> <p> <code>&amp;Attribute.2=that</code> </p> </note></p>
    fn send_message_batch(
        &self,
        input: SendMessageBatchRequest,
    ) -> RusotoFuture<SendMessageBatchResult, SendMessageBatchError>;

    /// <p><p>Sets the value of one or more queue attributes. When you change a queue&#39;s attributes, the change can take up to 60 seconds for most of the attributes to propagate throughout the Amazon SQS system. Changes made to the <code>MessageRetentionPeriod</code> attribute can take up to 15 minutes.</p> <note> <p>In the future, new attributes might be added. If you write code that calls this action, we recommend that you structure your code so that it can handle new attributes gracefully.</p> </note></p>
    fn set_queue_attributes(
        &self,
        input: SetQueueAttributesRequest,
    ) -> RusotoFuture<(), SetQueueAttributesError>;

    /// <p>Add cost allocation tags to the specified Amazon SQS queue. For an overview, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-tagging-queues.html">Tagging Amazon SQS Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>When you use queue tags, keep the following guidelines in mind:</p> <ul> <li> <p>Adding more than 50 tags to a queue isn't recommended.</p> </li> <li> <p>Tags don't have any semantic meaning. Amazon SQS interprets tags as character strings.</p> </li> <li> <p>Tags are case-sensitive.</p> </li> <li> <p>A new tag with a key identical to that of an existing tag overwrites the existing tag.</p> </li> <li> <p>Tagging API actions are limited to 5 TPS per AWS account. If your application requires a higher throughput, file a <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=technical">technical support request</a>.</p> </li> </ul> <p>For a full list of tag restrictions, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/limits-queues.html">Limits Related to Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    fn tag_queue(&self, input: TagQueueRequest) -> RusotoFuture<(), TagQueueError>;

    /// <p>Remove cost allocation tags from the specified Amazon SQS queue. For an overview, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-tagging-queues.html">Tagging Amazon SQS Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>When you use queue tags, keep the following guidelines in mind:</p> <ul> <li> <p>Adding more than 50 tags to a queue isn't recommended.</p> </li> <li> <p>Tags don't have any semantic meaning. Amazon SQS interprets tags as character strings.</p> </li> <li> <p>Tags are case-sensitive.</p> </li> <li> <p>A new tag with a key identical to that of an existing tag overwrites the existing tag.</p> </li> <li> <p>Tagging API actions are limited to 5 TPS per AWS account. If your application requires a higher throughput, file a <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=technical">technical support request</a>.</p> </li> </ul> <p>For a full list of tag restrictions, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/limits-queues.html">Limits Related to Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    fn untag_queue(&self, input: UntagQueueRequest) -> RusotoFuture<(), UntagQueueError>;
}
/// A client for the Amazon SQS API.
pub struct SqsClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl SqsClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> SqsClient {
        SqsClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> SqsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        SqsClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Sqs for SqsClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p><p>Adds a permission to a queue for a specific <a href="http://docs.aws.amazon.com/general/latest/gr/glos-chap.html#P">principal</a>. This allows sharing access to the queue.</p> <p>When you create a queue, you have full control access rights for the queue. Only you, the owner of the queue, can grant or deny permissions to the queue. For more information about these permissions, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/acp-overview.html">Shared Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <note> <p> <code>AddPermission</code> writes an Amazon-SQS-generated policy. If you want to write your own policy, use <code> <a>SetQueueAttributes</a> </code> to upload your policy. For more information about writing your own policy, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/AccessPolicyLanguage.html">Using The Access Policy Language</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>Some actions take lists of parameters. These lists are specified using the <code>param.n</code> notation. Values of <code>n</code> are integers starting from 1. For example, a parameter list with two elements looks like this:</p> <p> <code>&amp;Attribute.1=this</code> </p> <p> <code>&amp;Attribute.2=that</code> </p> </note></p>
    fn add_permission(&self, input: AddPermissionRequest) -> RusotoFuture<(), AddPermissionError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddPermission");
        params.put("Version", "2012-11-05");
        AddPermissionRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddPermissionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Changes the visibility timeout of a specified message in a queue to a new value. The maximum allowed timeout value is 12 hours. Thus, you can&#39;t extend the timeout of a message in an existing queue to more than a total visibility timeout of 12 hours. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>For example, you have a message with a visibility timeout of 5 minutes. After 3 minutes, you call <code>ChangeMessageVisiblity</code> with a timeout of 10 minutes. At that time, the timeout for the message is extended by 10 minutes beyond the time of the <code>ChangeMessageVisibility</code> action. This results in a total visibility timeout of 13 minutes. You can continue to call the <code>ChangeMessageVisibility</code> to extend the visibility timeout to a maximum of 12 hours. If you try to extend the visibility timeout beyond 12 hours, your request is rejected.</p> <p>A message is considered to be <i>in flight</i> after it&#39;s received from a queue by a consumer, but not yet deleted from the queue.</p> <p>For standard queues, there can be a maximum of 120,000 inflight messages per queue. If you reach this limit, Amazon SQS returns the <code>OverLimit</code> error message. To avoid reaching the limit, you should delete messages from the queue after they&#39;re processed. You can also increase the number of queues you use to process your messages.</p> <p>For FIFO queues, there can be a maximum of 20,000 inflight messages per queue. If you reach this limit, Amazon SQS returns no error messages.</p> <important> <p>If you attempt to set the <code>VisibilityTimeout</code> to a value greater than the maximum time left, Amazon SQS returns an error. Amazon SQS doesn&#39;t automatically recalculate and increase the timeout to the maximum remaining time.</p> <p>Unlike with a queue, when you change the visibility timeout for a specific message the timeout value is applied immediately but isn&#39;t saved in memory for that message. If you don&#39;t delete a message after it is received, the visibility timeout for the message reverts to the original timeout value (not to the value you set using the <code>ChangeMessageVisibility</code> action) the next time the message is received.</p> </important></p>
    fn change_message_visibility(
        &self,
        input: ChangeMessageVisibilityRequest,
    ) -> RusotoFuture<(), ChangeMessageVisibilityError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ChangeMessageVisibility");
        params.put("Version", "2012-11-05");
        ChangeMessageVisibilityRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ChangeMessageVisibilityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Changes the visibility timeout of multiple messages. This is a batch version of <code> <a>ChangeMessageVisibility</a>.</code> The result of the action on each message is reported individually in the response. You can send up to 10 <code> <a>ChangeMessageVisibility</a> </code> requests with each <code>ChangeMessageVisibilityBatch</code> action.</p> <important> <p>Because the batch request can result in a combination of successful and unsuccessful actions, you should check for batch errors even when the call returns an HTTP status code of <code>200</code>.</p> </important> <note> <p>Some actions take lists of parameters. These lists are specified using the <code>param.n</code> notation. Values of <code>n</code> are integers starting from 1. For example, a parameter list with two elements looks like this:</p> <p> <code>&amp;Attribute.1=this</code> </p> <p> <code>&amp;Attribute.2=that</code> </p> </note></p>
    fn change_message_visibility_batch(
        &self,
        input: ChangeMessageVisibilityBatchRequest,
    ) -> RusotoFuture<ChangeMessageVisibilityBatchResult, ChangeMessageVisibilityBatchError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ChangeMessageVisibilityBatch");
        params.put("Version", "2012-11-05");
        ChangeMessageVisibilityBatchRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ChangeMessageVisibilityBatchError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ChangeMessageVisibilityBatchResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ChangeMessageVisibilityBatchResultDeserializer::deserialize(
                        "ChangeMessageVisibilityBatchResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a new standard or FIFO queue. You can pass one or more attributes in the request. Keep the following caveats in mind:</p> <ul> <li> <p>If you don&#39;t specify the <code>FifoQueue</code> attribute, Amazon SQS creates a standard queue.</p> <note> <p> You can&#39;t change the queue type after you create it and you can&#39;t convert an existing standard queue into a FIFO queue. You must either create a new FIFO queue for your application or delete your existing standard queue and recreate it as a FIFO queue. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html#FIFO-queues-moving"> Moving From a Standard Queue to a FIFO Queue</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p> </note> </li> <li> <p>If you don&#39;t provide a value for an attribute, the queue is created with the default value for the attribute.</p> </li> <li> <p>If you delete a queue, you must wait at least 60 seconds before creating a queue with the same name.</p> </li> </ul> <p>To successfully create a new queue, you must provide a queue name that adheres to the <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/limits-queues.html">limits related to queues</a> and is unique within the scope of your queues.</p> <p>To get the queue URL, use the <code> <a>GetQueueUrl</a> </code> action. <code> <a>GetQueueUrl</a> </code> requires only the <code>QueueName</code> parameter. be aware of existing queue names:</p> <ul> <li> <p>If you provide the name of an existing queue along with the exact names and values of all the queue&#39;s attributes, <code>CreateQueue</code> returns the queue URL for the existing queue.</p> </li> <li> <p>If the queue name, attribute names, or attribute values don&#39;t match an existing queue, <code>CreateQueue</code> returns an error.</p> </li> </ul> <note> <p>Some actions take lists of parameters. These lists are specified using the <code>param.n</code> notation. Values of <code>n</code> are integers starting from 1. For example, a parameter list with two elements looks like this:</p> <p> <code>&amp;Attribute.1=this</code> </p> <p> <code>&amp;Attribute.2=that</code> </p> </note></p>
    fn create_queue(
        &self,
        input: CreateQueueRequest,
    ) -> RusotoFuture<CreateQueueResult, CreateQueueError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateQueue");
        params.put("Version", "2012-11-05");
        CreateQueueRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateQueueError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateQueueResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateQueueResultDeserializer::deserialize(
                        "CreateQueueResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes the specified message from the specified queue. You specify the message by using the message&#39;s <i>receipt handle</i> and not the <i>MessageId</i> you receive when you send the message. Even if the message is locked by another reader due to the visibility timeout setting, it is still deleted from the queue. If you leave a message in the queue for longer than the queue&#39;s configured retention period, Amazon SQS automatically deletes the message. </p> <note> <p> The receipt handle is associated with a specific instance of receiving the message. If you receive a message more than once, the receipt handle you get each time you receive the message is different. If you don&#39;t provide the most recently received receipt handle for the message when you use the <code>DeleteMessage</code> action, the request succeeds, but the message might not be deleted.</p> <p>For standard queues, it is possible to receive a message even after you delete it. This might happen on rare occasions if one of the servers storing a copy of the message is unavailable when you send the request to delete the message. The copy remains on the server and might be returned to you on a subsequent receive request. You should ensure that your application is idempotent, so that receiving a message more than once does not cause issues.</p> </note></p>
    fn delete_message(&self, input: DeleteMessageRequest) -> RusotoFuture<(), DeleteMessageError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteMessage");
        params.put("Version", "2012-11-05");
        DeleteMessageRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteMessageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes up to ten messages from the specified queue. This is a batch version of <code> <a>DeleteMessage</a>.</code> The result of the action on each message is reported individually in the response.</p> <important> <p>Because the batch request can result in a combination of successful and unsuccessful actions, you should check for batch errors even when the call returns an HTTP status code of <code>200</code>.</p> </important> <note> <p>Some actions take lists of parameters. These lists are specified using the <code>param.n</code> notation. Values of <code>n</code> are integers starting from 1. For example, a parameter list with two elements looks like this:</p> <p> <code>&amp;Attribute.1=this</code> </p> <p> <code>&amp;Attribute.2=that</code> </p> </note></p>
    fn delete_message_batch(
        &self,
        input: DeleteMessageBatchRequest,
    ) -> RusotoFuture<DeleteMessageBatchResult, DeleteMessageBatchError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteMessageBatch");
        params.put("Version", "2012-11-05");
        DeleteMessageBatchRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteMessageBatchError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteMessageBatchResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteMessageBatchResultDeserializer::deserialize(
                        "DeleteMessageBatchResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the queue specified by the <code>QueueUrl</code>, regardless of the queue's contents. If the specified queue doesn't exist, Amazon SQS returns a successful response.</p> <important> <p>Be careful with the <code>DeleteQueue</code> action: When you delete a queue, any messages in the queue are no longer available. </p> </important> <p>When you delete a queue, the deletion process takes up to 60 seconds. Requests you send involving that queue during the 60 seconds might succeed. For example, a <code> <a>SendMessage</a> </code> request might succeed, but after 60 seconds the queue and the message you sent no longer exist.</p> <p>When you delete a queue, you must wait at least 60 seconds before creating a queue with the same name. </p>
    fn delete_queue(&self, input: DeleteQueueRequest) -> RusotoFuture<(), DeleteQueueError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteQueue");
        params.put("Version", "2012-11-05");
        DeleteQueueRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteQueueError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Gets attributes for the specified queue.</p> <note> <p>To determine whether a queue is <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO</a>, you can check whether <code>QueueName</code> ends with the <code>.fifo</code> suffix.</p> </note> <note> <p>Some actions take lists of parameters. These lists are specified using the <code>param.n</code> notation. Values of <code>n</code> are integers starting from 1. For example, a parameter list with two elements looks like this:</p> <p> <code>&amp;Attribute.1=this</code> </p> <p> <code>&amp;Attribute.2=that</code> </p> </note></p>
    fn get_queue_attributes(
        &self,
        input: GetQueueAttributesRequest,
    ) -> RusotoFuture<GetQueueAttributesResult, GetQueueAttributesError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetQueueAttributes");
        params.put("Version", "2012-11-05");
        GetQueueAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetQueueAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetQueueAttributesResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetQueueAttributesResultDeserializer::deserialize(
                        "GetQueueAttributesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the URL of an existing queue. This action provides a simple way to retrieve the URL of an Amazon SQS queue.</p> <p>To access a queue that belongs to another AWS account, use the <code>QueueOwnerAWSAccountId</code> parameter to specify the account ID of the queue's owner. The queue's owner must grant you permission to access the queue. For more information about shared queue access, see <code> <a>AddPermission</a> </code> or see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/acp-overview.html">Shared Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p>
    fn get_queue_url(
        &self,
        input: GetQueueUrlRequest,
    ) -> RusotoFuture<GetQueueUrlResult, GetQueueUrlError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetQueueUrl");
        params.put("Version", "2012-11-05");
        GetQueueUrlRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetQueueUrlError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetQueueUrlResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetQueueUrlResultDeserializer::deserialize(
                        "GetQueueUrlResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of your queues that have the <code>RedrivePolicy</code> queue attribute configured with a dead-letter queue.</p> <p>For more information about using dead-letter queues, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">Using Amazon SQS Dead-Letter Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    fn list_dead_letter_source_queues(
        &self,
        input: ListDeadLetterSourceQueuesRequest,
    ) -> RusotoFuture<ListDeadLetterSourceQueuesResult, ListDeadLetterSourceQueuesError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListDeadLetterSourceQueues");
        params.put("Version", "2012-11-05");
        ListDeadLetterSourceQueuesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListDeadLetterSourceQueuesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListDeadLetterSourceQueuesResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListDeadLetterSourceQueuesResultDeserializer::deserialize(
                        "ListDeadLetterSourceQueuesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>List all cost allocation tags added to the specified Amazon SQS queue. For an overview, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-tagging-queues.html">Tagging Amazon SQS Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>When you use queue tags, keep the following guidelines in mind:</p> <ul> <li> <p>Adding more than 50 tags to a queue isn't recommended.</p> </li> <li> <p>Tags don't have any semantic meaning. Amazon SQS interprets tags as character strings.</p> </li> <li> <p>Tags are case-sensitive.</p> </li> <li> <p>A new tag with a key identical to that of an existing tag overwrites the existing tag.</p> </li> <li> <p>Tagging API actions are limited to 5 TPS per AWS account. If your application requires a higher throughput, file a <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=technical">technical support request</a>.</p> </li> </ul> <p>For a full list of tag restrictions, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/limits-queues.html">Limits Related to Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    fn list_queue_tags(
        &self,
        input: ListQueueTagsRequest,
    ) -> RusotoFuture<ListQueueTagsResult, ListQueueTagsError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListQueueTags");
        params.put("Version", "2012-11-05");
        ListQueueTagsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListQueueTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListQueueTagsResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListQueueTagsResultDeserializer::deserialize(
                        "ListQueueTagsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of your queues. The maximum number of queues that can be returned is 1,000. If you specify a value for the optional <code>QueueNamePrefix</code> parameter, only queues with a name that begins with the specified value are returned.</p>
    fn list_queues(
        &self,
        input: ListQueuesRequest,
    ) -> RusotoFuture<ListQueuesResult, ListQueuesError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListQueues");
        params.put("Version", "2012-11-05");
        ListQueuesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListQueuesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListQueuesResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListQueuesResultDeserializer::deserialize(
                        "ListQueuesResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the messages in a queue specified by the <code>QueueURL</code> parameter.</p> <important> <p>When you use the <code>PurgeQueue</code> action, you can't retrieve a message deleted from a queue.</p> </important> <p>When you purge a queue, the message deletion process takes up to 60 seconds. All messages sent to the queue before calling the <code>PurgeQueue</code> action are deleted. Messages sent to the queue while it is being purged might be deleted. While the queue is being purged, messages sent to the queue before <code>PurgeQueue</code> is called might be received, but are deleted within the next minute.</p>
    fn purge_queue(&self, input: PurgeQueueRequest) -> RusotoFuture<(), PurgeQueueError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PurgeQueue");
        params.put("Version", "2012-11-05");
        PurgeQueueRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PurgeQueueError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Retrieves one or more messages (up to 10), from the specified queue. Using the <code>WaitTimeSeconds</code> parameter enables long-poll support. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-long-polling.html">Amazon SQS Long Polling</a> in the <i>Amazon Simple Queue Service Developer Guide</i>. </p> <p>Short poll is the default behavior where a weighted random set of machines is sampled on a <code>ReceiveMessage</code> call. Thus, only the messages on the sampled machines are returned. If the number of messages in the queue is small (fewer than 1,000), you most likely get fewer messages than you requested per <code>ReceiveMessage</code> call. If the number of messages in the queue is extremely small, you might not receive any messages in a particular <code>ReceiveMessage</code> response. If this happens, repeat the request. </p> <p>For each message returned, the response includes the following:</p> <ul> <li> <p>The message body.</p> </li> <li> <p>An MD5 digest of the message body. For information about MD5, see <a href="https://www.ietf.org/rfc/rfc1321.txt">RFC1321</a>.</p> </li> <li> <p>The <code>MessageId</code> you received when you sent the message to the queue.</p> </li> <li> <p>The receipt handle.</p> </li> <li> <p>The message attributes.</p> </li> <li> <p>An MD5 digest of the message attributes.</p> </li> </ul> <p>The receipt handle is the identifier you must provide when deleting the message. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-queue-message-identifiers.html">Queue and Message Identifiers</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>You can provide the <code>VisibilityTimeout</code> parameter in your request. The parameter is applied to the messages that Amazon SQS returns in the response. If you don&#39;t include the parameter, the overall visibility timeout for the queue is used for the returned messages. For more information, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html">Visibility Timeout</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>A message that isn&#39;t deleted or a message whose visibility isn&#39;t extended before the visibility timeout expires counts as a failed receive. Depending on the configuration of the queue, the message might be sent to the dead-letter queue.</p> <note> <p>In the future, new attributes might be added. If you write code that calls this action, we recommend that you structure your code so that it can handle new attributes gracefully.</p> </note></p>
    fn receive_message(
        &self,
        input: ReceiveMessageRequest,
    ) -> RusotoFuture<ReceiveMessageResult, ReceiveMessageError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ReceiveMessage");
        params.put("Version", "2012-11-05");
        ReceiveMessageRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ReceiveMessageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ReceiveMessageResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ReceiveMessageResultDeserializer::deserialize(
                        "ReceiveMessageResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Revokes any permissions in the queue policy that matches the specified <code>Label</code> parameter. Only the owner of the queue can remove permissions.</p>
    fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> RusotoFuture<(), RemovePermissionError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemovePermission");
        params.put("Version", "2012-11-05");
        RemovePermissionRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemovePermissionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Delivers a message to the specified queue.</p> <important> <p>A message can include only XML, JSON, and unformatted text. The following Unicode characters are allowed:</p> <p> <code>#x9</code> | <code>#xA</code> | <code>#xD</code> | <code>#x20</code> to <code>#xD7FF</code> | <code>#xE000</code> to <code>#xFFFD</code> | <code>#x10000</code> to <code>#x10FFFF</code> </p> <p>Any characters not included in this list will be rejected. For more information, see the <a href="http://www.w3.org/TR/REC-xml/#charsets">W3C specification for characters</a>.</p> </important></p>
    fn send_message(
        &self,
        input: SendMessageRequest,
    ) -> RusotoFuture<SendMessageResult, SendMessageError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendMessage");
        params.put("Version", "2012-11-05");
        SendMessageRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SendMessageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SendMessageResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SendMessageResultDeserializer::deserialize(
                        "SendMessageResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Delivers up to ten messages to the specified queue. This is a batch version of <code> <a>SendMessage</a>.</code> For a FIFO queue, multiple messages within a single batch are enqueued in the order they are sent.</p> <p>The result of sending each message is reported individually in the response. Because the batch request can result in a combination of successful and unsuccessful actions, you should check for batch errors even when the call returns an HTTP status code of <code>200</code>.</p> <p>The maximum allowed individual message size and the maximum total payload size (the sum of the individual lengths of all of the batched messages) are both 256 KB (262,144 bytes).</p> <important> <p>A message can include only XML, JSON, and unformatted text. The following Unicode characters are allowed:</p> <p> <code>#x9</code> | <code>#xA</code> | <code>#xD</code> | <code>#x20</code> to <code>#xD7FF</code> | <code>#xE000</code> to <code>#xFFFD</code> | <code>#x10000</code> to <code>#x10FFFF</code> </p> <p>Any characters not included in this list will be rejected. For more information, see the <a href="http://www.w3.org/TR/REC-xml/#charsets">W3C specification for characters</a>.</p> </important> <p>If you don&#39;t specify the <code>DelaySeconds</code> parameter for an entry, Amazon SQS uses the default value for the queue.</p> <note> <p>Some actions take lists of parameters. These lists are specified using the <code>param.n</code> notation. Values of <code>n</code> are integers starting from 1. For example, a parameter list with two elements looks like this:</p> <p> <code>&amp;Attribute.1=this</code> </p> <p> <code>&amp;Attribute.2=that</code> </p> </note></p>
    fn send_message_batch(
        &self,
        input: SendMessageBatchRequest,
    ) -> RusotoFuture<SendMessageBatchResult, SendMessageBatchError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SendMessageBatch");
        params.put("Version", "2012-11-05");
        SendMessageBatchRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SendMessageBatchError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SendMessageBatchResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(SendMessageBatchResultDeserializer::deserialize(
                        "SendMessageBatchResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Sets the value of one or more queue attributes. When you change a queue&#39;s attributes, the change can take up to 60 seconds for most of the attributes to propagate throughout the Amazon SQS system. Changes made to the <code>MessageRetentionPeriod</code> attribute can take up to 15 minutes.</p> <note> <p>In the future, new attributes might be added. If you write code that calls this action, we recommend that you structure your code so that it can handle new attributes gracefully.</p> </note></p>
    fn set_queue_attributes(
        &self,
        input: SetQueueAttributesRequest,
    ) -> RusotoFuture<(), SetQueueAttributesError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetQueueAttributes");
        params.put("Version", "2012-11-05");
        SetQueueAttributesRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetQueueAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Add cost allocation tags to the specified Amazon SQS queue. For an overview, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-tagging-queues.html">Tagging Amazon SQS Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>When you use queue tags, keep the following guidelines in mind:</p> <ul> <li> <p>Adding more than 50 tags to a queue isn't recommended.</p> </li> <li> <p>Tags don't have any semantic meaning. Amazon SQS interprets tags as character strings.</p> </li> <li> <p>Tags are case-sensitive.</p> </li> <li> <p>A new tag with a key identical to that of an existing tag overwrites the existing tag.</p> </li> <li> <p>Tagging API actions are limited to 5 TPS per AWS account. If your application requires a higher throughput, file a <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=technical">technical support request</a>.</p> </li> </ul> <p>For a full list of tag restrictions, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/limits-queues.html">Limits Related to Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    fn tag_queue(&self, input: TagQueueRequest) -> RusotoFuture<(), TagQueueError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "TagQueue");
        params.put("Version", "2012-11-05");
        TagQueueRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TagQueueError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Remove cost allocation tags from the specified Amazon SQS queue. For an overview, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-tagging-queues.html">Tagging Amazon SQS Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p> <p>When you use queue tags, keep the following guidelines in mind:</p> <ul> <li> <p>Adding more than 50 tags to a queue isn't recommended.</p> </li> <li> <p>Tags don't have any semantic meaning. Amazon SQS interprets tags as character strings.</p> </li> <li> <p>Tags are case-sensitive.</p> </li> <li> <p>A new tag with a key identical to that of an existing tag overwrites the existing tag.</p> </li> <li> <p>Tagging API actions are limited to 5 TPS per AWS account. If your application requires a higher throughput, file a <a href="https://console.aws.amazon.com/support/home#/case/create?issueType=technical">technical support request</a>.</p> </li> </ul> <p>For a full list of tag restrictions, see <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/limits-queues.html">Limits Related to Queues</a> in the <i>Amazon Simple Queue Service Developer Guide</i>.</p>
    fn untag_queue(&self, input: UntagQueueRequest) -> RusotoFuture<(), UntagQueueError> {
        let mut request = SignedRequest::new("POST", "sqs", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UntagQueue");
        params.put("Version", "2012-11-05");
        UntagQueueRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UntagQueueError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[test]
    fn test_parse_error_sqs_delete_queue() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "sqs-delete-queue.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeleteQueueRequest::default();
        let result = client.delete_queue(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sqs_add_permission() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sqs-add-permission.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = AddPermissionRequest::default();
        let result = client.add_permission(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sqs_change_message_visibility_batch() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sqs-change-message-visibility-batch.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ChangeMessageVisibilityBatchRequest::default();
        let result = client.change_message_visibility_batch(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sqs_create_queue() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sqs-create-queue.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateQueueRequest::default();
        let result = client.create_queue(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sqs_delete_message_batch() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sqs-delete-message-batch.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeleteMessageBatchRequest::default();
        let result = client.delete_message_batch(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sqs_get_queue_attributes() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sqs-get-queue-attributes.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetQueueAttributesRequest::default();
        let result = client.get_queue_attributes(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sqs_get_queue_url() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sqs-get-queue-url.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetQueueUrlRequest::default();
        let result = client.get_queue_url(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sqs_list_queues() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sqs-list-queues.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListQueuesRequest::default();
        let result = client.list_queues(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sqs_receive_message() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sqs-receive-message.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ReceiveMessageRequest::default();
        let result = client.receive_message(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sqs_send_message_batch() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sqs-send-message-batch.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = SendMessageBatchRequest::default();
        let result = client.send_message_batch(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_sqs_send_message() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "sqs-send-message.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = SendMessageRequest::default();
        let result = client.send_message(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
