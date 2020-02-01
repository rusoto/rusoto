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

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Represents an option to be shown on the client platform (Facebook, Slack, etc.)</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Button {
    /// <p>Text that is visible to the user on the button.</p>
    #[serde(rename = "text")]
    pub text: String,
    /// <p>The value sent to Amazon Lex when a user chooses the button. For example, consider button text "NYC." When the user chooses the button, the value sent can be "New York City."</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSessionRequest {
    /// <p>The alias in use for the bot that contains the session data.</p>
    #[serde(rename = "botAlias")]
    pub bot_alias: String,
    /// <p>The name of the bot that contains the session data.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>The identifier of the user associated with the session data.</p>
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSessionResponse {
    /// <p>The alias in use for the bot associated with the session data.</p>
    #[serde(rename = "botAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias: Option<String>,
    /// <p>The name of the bot associated with the session data.</p>
    #[serde(rename = "botName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    /// <p>The unique identifier for the session.</p>
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// <p>The ID of the client application user.</p>
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p>Describes the next action that the bot should take in its interaction with the user and provides information about the context in which the action takes place. Use the <code>DialogAction</code> data type to set the interaction to a specific state, or to return the interaction to a previous state.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DialogAction {
    /// <p><p>The fulfillment state of the intent. The possible values are:</p> <ul> <li> <p> <code>Failed</code> - The Lambda function associated with the intent failed to fulfill the intent.</p> </li> <li> <p> <code>Fulfilled</code> - The intent has fulfilled by the Lambda function associated with the intent. </p> </li> <li> <p> <code>ReadyForFulfillment</code> - All of the information necessary for the intent is present and the intent ready to be fulfilled by the client application.</p> </li> </ul></p>
    #[serde(rename = "fulfillmentState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_state: Option<String>,
    /// <p>The name of the intent.</p>
    #[serde(rename = "intentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    /// <p>The message that should be shown to the user. If you don't specify a message, Amazon Lex will use the message configured for the intent.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <ul> <li> <p> <code>PlainText</code> - The message contains plain UTF-8 text.</p> </li> <li> <p> <code>CustomPayload</code> - The message is a custom format for the client.</p> </li> <li> <p> <code>SSML</code> - The message contains text formatted for voice output.</p> </li> <li> <p> <code>Composite</code> - The message contains an escaped JSON object containing one or more messages. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/howitworks-manage-prompts.html">Message Groups</a>. </p> </li> </ul>
    #[serde(rename = "messageFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_format: Option<String>,
    /// <p>The name of the slot that should be elicited from the user.</p>
    #[serde(rename = "slotToElicit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_to_elicit: Option<String>,
    /// <p>Map of the slots that have been gathered and their values. </p>
    #[serde(rename = "slots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<::std::collections::HashMap<String, String>>,
    /// <p><p>The next action that the bot should take in its interaction with the user. The possible values are:</p> <ul> <li> <p> <code>ConfirmIntent</code> - The next action is asking the user if the intent is complete and ready to be fulfilled. This is a yes/no question such as &quot;Place the order?&quot;</p> </li> <li> <p> <code>Close</code> - Indicates that the there will not be a response from the user. For example, the statement &quot;Your order has been placed&quot; does not require a response.</p> </li> <li> <p> <code>Delegate</code> - The next action is determined by Amazon Lex.</p> </li> <li> <p> <code>ElicitIntent</code> - The next action is to determine the intent that the user wants to fulfill.</p> </li> <li> <p> <code>ElicitSlot</code> - The next action is to elicit a slot value from the user.</p> </li> </ul></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Represents an option rendered to the user when a prompt is shown. It could be an image, a button, a link, or text. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenericAttachment {
    /// <p>The URL of an attachment to the response card.</p>
    #[serde(rename = "attachmentLinkUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_link_url: Option<String>,
    /// <p>The list of options to show to the user.</p>
    #[serde(rename = "buttons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<Button>>,
    /// <p>The URL of an image that is displayed to the user.</p>
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The subtitle shown below the title.</p>
    #[serde(rename = "subTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_title: Option<String>,
    /// <p>The title of the option.</p>
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSessionRequest {
    /// <p>The alias in use for the bot that contains the session data.</p>
    #[serde(rename = "botAlias")]
    pub bot_alias: String,
    /// <p>The name of the bot that contains the session data.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>A string used to filter the intents returned in the <code>recentIntentSummaryView</code> structure. </p> <p>When you specify a filter, only intents with their <code>checkpointLabel</code> field set to that string are returned.</p>
    #[serde(rename = "checkpointLabelFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_label_filter: Option<String>,
    /// <p>The ID of the client application user. Amazon Lex uses this to identify a user's conversation with your bot. </p>
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSessionResponse {
    /// <p>Describes the current state of the bot.</p>
    #[serde(rename = "dialogAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_action: Option<DialogAction>,
    /// <p>An array of information about the intents used in the session. The array can contain a maximum of three summaries. If more than three intents are used in the session, the <code>recentIntentSummaryView</code> operation contains information about the last three intents used.</p> <p>If you set the <code>checkpointLabelFilter</code> parameter in the request, the array contains only the intents with the specified label.</p>
    #[serde(rename = "recentIntentSummaryView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_intent_summary_view: Option<Vec<IntentSummary>>,
    /// <p>Map of key/value pairs representing the session-specific context information. It contains application information passed between Amazon Lex and a client application.</p>
    #[serde(rename = "sessionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique identifier for the session.</p>
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// <p>Provides information about the state of an intent. You can use this information to get the current state of an intent so that you can process the intent, or so that you can return the intent to its previous state.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentSummary {
    /// <p>A user-defined label that identifies a particular intent. You can use this label to return to a previous intent. </p> <p>Use the <code>checkpointLabelFilter</code> parameter of the <code>GetSessionRequest</code> operation to filter the intents returned by the operation to those with only the specified label.</p>
    #[serde(rename = "checkpointLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_label: Option<String>,
    /// <p><p>The status of the intent after the user responds to the confirmation prompt. If the user confirms the intent, Amazon Lex sets this field to <code>Confirmed</code>. If the user denies the intent, Amazon Lex sets this value to <code>Denied</code>. The possible values are:</p> <ul> <li> <p> <code>Confirmed</code> - The user has responded &quot;Yes&quot; to the confirmation prompt, confirming that the intent is complete and that it is ready to be fulfilled.</p> </li> <li> <p> <code>Denied</code> - The user has responded &quot;No&quot; to the confirmation prompt.</p> </li> <li> <p> <code>None</code> - The user has never been prompted for confirmation; or, the user was prompted but did not confirm or deny the prompt.</p> </li> </ul></p>
    #[serde(rename = "confirmationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_status: Option<String>,
    /// <p><p>The next action that the bot should take in its interaction with the user. The possible values are:</p> <ul> <li> <p> <code>ConfirmIntent</code> - The next action is asking the user if the intent is complete and ready to be fulfilled. This is a yes/no question such as &quot;Place the order?&quot;</p> </li> <li> <p> <code>Close</code> - Indicates that the there will not be a response from the user. For example, the statement &quot;Your order has been placed&quot; does not require a response.</p> </li> <li> <p> <code>ElicitIntent</code> - The next action is to determine the intent that the user wants to fulfill.</p> </li> <li> <p> <code>ElicitSlot</code> - The next action is to elicit a slot value from the user.</p> </li> </ul></p>
    #[serde(rename = "dialogActionType")]
    pub dialog_action_type: String,
    /// <p><p>The fulfillment state of the intent. The possible values are:</p> <ul> <li> <p> <code>Failed</code> - The Lambda function associated with the intent failed to fulfill the intent.</p> </li> <li> <p> <code>Fulfilled</code> - The intent has fulfilled by the Lambda function associated with the intent. </p> </li> <li> <p> <code>ReadyForFulfillment</code> - All of the information necessary for the intent is present and the intent ready to be fulfilled by the client application.</p> </li> </ul></p>
    #[serde(rename = "fulfillmentState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_state: Option<String>,
    /// <p>The name of the intent.</p>
    #[serde(rename = "intentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    /// <p>The next slot to elicit from the user. If there is not slot to elicit, the field is blank.</p>
    #[serde(rename = "slotToElicit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_to_elicit: Option<String>,
    /// <p>Map of the slots that have been gathered and their values. </p>
    #[serde(rename = "slots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PostContentRequest {
    /// <p><p> You pass this value as the <code>Accept</code> HTTP header. </p> <p> The message Amazon Lex returns in the response can be either text or speech based on the <code>Accept</code> HTTP header value in the request. </p> <ul> <li> <p> If the value is <code>text/plain; charset=utf-8</code>, Amazon Lex returns text in the response. </p> </li> <li> <p> If the value begins with <code>audio/</code>, Amazon Lex returns speech in the response. Amazon Lex uses Amazon Polly to generate the speech (using the configuration you specified in the <code>Accept</code> header). For example, if you specify <code>audio/mpeg</code> as the value, Amazon Lex returns speech in the MPEG format.</p> </li> <li> <p>If the value is <code>audio/pcm</code>, the speech returned is <code>audio/pcm</code> in 16-bit, little endian format. </p> </li> <li> <p>The following are the accepted values:</p> <ul> <li> <p>audio/mpeg</p> </li> <li> <p>audio/ogg</p> </li> <li> <p>audio/pcm</p> </li> <li> <p>text/plain; charset=utf-8</p> </li> <li> <p>audio/* (defaults to mpeg)</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "accept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    /// <p>Alias of the Amazon Lex bot.</p>
    #[serde(rename = "botAlias")]
    pub bot_alias: String,
    /// <p>Name of the Amazon Lex bot.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p><p> You pass this value as the <code>Content-Type</code> HTTP header. </p> <p> Indicates the audio format or text. The header value must start with one of the following prefixes: </p> <ul> <li> <p>PCM format, audio data must be in little-endian byte order.</p> <ul> <li> <p>audio/l16; rate=16000; channels=1</p> </li> <li> <p>audio/x-l16; sample-rate=16000; channel-count=1</p> </li> <li> <p>audio/lpcm; sample-rate=8000; sample-size-bits=16; channel-count=1; is-big-endian=false </p> </li> </ul> </li> <li> <p>Opus format</p> <ul> <li> <p>audio/x-cbr-opus-with-preamble; preamble-size=0; bit-rate=256000; frame-size-milliseconds=4</p> </li> </ul> </li> <li> <p>Text format</p> <ul> <li> <p>text/plain; charset=utf-8</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// <p> User input in PCM or Opus audio format or text format as described in the <code>Content-Type</code> HTTP header. </p> <p>You can stream audio data to Amazon Lex or you can create a local buffer that captures all of the audio data before sending. In general, you get better performance if you stream audio data rather than buffering the data locally.</p>
    #[serde(rename = "inputStream")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub input_stream: bytes::Bytes,
    /// <p>You pass this value as the <code>x-amz-lex-request-attributes</code> HTTP header.</p> <p>Request-specific information passed between Amazon Lex and a client application. The value must be a JSON serialized and base64 encoded map with string keys and values. The total size of the <code>requestAttributes</code> and <code>sessionAttributes</code> headers is limited to 12 KB.</p> <p>The namespace <code>x-amz-lex:</code> is reserved for special attributes. Don't create any request attributes with the prefix <code>x-amz-lex:</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html#context-mgmt-request-attribs">Setting Request Attributes</a>.</p>
    #[serde(rename = "requestAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_attributes: Option<String>,
    /// <p>You pass this value as the <code>x-amz-lex-session-attributes</code> HTTP header.</p> <p>Application-specific information passed between Amazon Lex and a client application. The value must be a JSON serialized and base64 encoded map with string keys and values. The total size of the <code>sessionAttributes</code> and <code>requestAttributes</code> headers is limited to 12 KB.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html#context-mgmt-session-attribs">Setting Session Attributes</a>.</p>
    #[serde(rename = "sessionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_attributes: Option<String>,
    /// <p><p>The ID of the client application user. Amazon Lex uses this to identify a user&#39;s conversation with your bot. At runtime, each request must contain the <code>userID</code> field.</p> <p>To decide the user ID to use for your application, consider the following factors.</p> <ul> <li> <p>The <code>userID</code> field must not contain any personally identifiable information of the user, for example, name, personal identification numbers, or other end user personal information.</p> </li> <li> <p>If you want a user to start a conversation on one device and continue on another device, use a user-specific identifier.</p> </li> <li> <p>If you want the same user to be able to have two independent conversations on two different devices, choose a device-specific identifier.</p> </li> <li> <p>A user can&#39;t have two independent conversations with two different versions of the same bot. For example, a user can&#39;t have a conversation with the PROD and BETA versions of the same bot. If you anticipate that a user will need to have conversation with two different versions, for example, while testing, include the bot alias in the user ID to separate the two conversations.</p> </li> </ul></p>
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PostContentResponse {
    /// <p>The prompt (or statement) to convey to the user. This is based on the bot configuration and context. For example, if Amazon Lex did not understand the user intent, it sends the <code>clarificationPrompt</code> configured for the bot. If the intent requires confirmation before taking the fulfillment action, it sends the <code>confirmationPrompt</code>. Another example: Suppose that the Lambda function successfully fulfilled the intent, and sent a message to convey to the user. Then Amazon Lex sends that message in the response. </p>
    pub audio_stream: Option<bytes::Bytes>,
    /// <p>Content type as specified in the <code>Accept</code> HTTP header in the request.</p>
    pub content_type: Option<String>,
    /// <p><p>Identifies the current state of the user interaction. Amazon Lex returns one of the following values as <code>dialogState</code>. The client can optionally use this information to customize the user interface. </p> <ul> <li> <p> <code>ElicitIntent</code> - Amazon Lex wants to elicit the user&#39;s intent. Consider the following examples: </p> <p> For example, a user might utter an intent (&quot;I want to order a pizza&quot;). If Amazon Lex cannot infer the user intent from this utterance, it will return this dialog state. </p> </li> <li> <p> <code>ConfirmIntent</code> - Amazon Lex is expecting a &quot;yes&quot; or &quot;no&quot; response. </p> <p>For example, Amazon Lex wants user confirmation before fulfilling an intent. Instead of a simple &quot;yes&quot; or &quot;no&quot; response, a user might respond with additional information. For example, &quot;yes, but make it a thick crust pizza&quot; or &quot;no, I want to order a drink.&quot; Amazon Lex can process such additional information (in these examples, update the crust type slot or change the intent from OrderPizza to OrderDrink). </p> </li> <li> <p> <code>ElicitSlot</code> - Amazon Lex is expecting the value of a slot for the current intent. </p> <p> For example, suppose that in the response Amazon Lex sends this message: &quot;What size pizza would you like?&quot;. A user might reply with the slot value (e.g., &quot;medium&quot;). The user might also provide additional information in the response (e.g., &quot;medium thick crust pizza&quot;). Amazon Lex can process such additional information appropriately. </p> </li> <li> <p> <code>Fulfilled</code> - Conveys that the Lambda function has successfully fulfilled the intent. </p> </li> <li> <p> <code>ReadyForFulfillment</code> - Conveys that the client has to fulfill the request. </p> </li> <li> <p> <code>Failed</code> - Conveys that the conversation with the user failed. </p> <p> This can happen for various reasons, including that the user does not provide an appropriate response to prompts from the service (you can configure how many times Amazon Lex can prompt a user for specific information), or if the Lambda function fails to fulfill the intent. </p> </li> </ul></p>
    pub dialog_state: Option<String>,
    /// <p>The text used to process the request.</p> <p>If the input was an audio stream, the <code>inputTranscript</code> field contains the text extracted from the audio stream. This is the text that is actually processed to recognize intents and slot values. You can use this information to determine if Amazon Lex is correctly processing the audio that you send.</p>
    pub input_transcript: Option<String>,
    /// <p>Current user intent that Amazon Lex is aware of.</p>
    pub intent_name: Option<String>,
    /// <p>The message to convey to the user. The message can come from the bot's configuration or from a Lambda function.</p> <p>If the intent is not configured with a Lambda function, or if the Lambda function returned <code>Delegate</code> as the <code>dialogAction.type</code> in its response, Amazon Lex decides on the next course of action and selects an appropriate message from the bot's configuration based on the current interaction context. For example, if Amazon Lex isn't able to understand user input, it uses a clarification prompt message.</p> <p>When you create an intent you can assign messages to groups. When messages are assigned to groups Amazon Lex returns one message from each group in the response. The message field is an escaped JSON string containing the messages. For more information about the structure of the JSON string returned, see <a>msg-prompts-formats</a>.</p> <p>If the Lambda function returns a message, Amazon Lex passes it to the client in its response.</p>
    pub message: Option<String>,
    /// <p><p>The format of the response message. One of the following values:</p> <ul> <li> <p> <code>PlainText</code> - The message contains plain UTF-8 text.</p> </li> <li> <p> <code>CustomPayload</code> - The message is a custom format for the client.</p> </li> <li> <p> <code>SSML</code> - The message contains text formatted for voice output.</p> </li> <li> <p> <code>Composite</code> - The message contains an escaped JSON object containing one or more messages from the groups that messages were assigned to when the intent was created.</p> </li> </ul></p>
    pub message_format: Option<String>,
    /// <p>The sentiment expressed in and utterance.</p> <p>When the bot is configured to send utterances to Amazon Comprehend for sentiment analysis, this field contains the result of the analysis.</p>
    pub sentiment_response: Option<String>,
    /// <p> Map of key/value pairs representing the session-specific context information. </p>
    pub session_attributes: Option<String>,
    /// <p>The unique identifier for the session.</p>
    pub session_id: Option<String>,
    /// <p> If the <code>dialogState</code> value is <code>ElicitSlot</code>, returns the name of the slot for which Amazon Lex is eliciting a value. </p>
    pub slot_to_elicit: Option<String>,
    /// <p>Map of zero or more intent slots (name/value pairs) Amazon Lex detected from the user input during the conversation. The field is base-64 encoded.</p> <p>Amazon Lex creates a resolution list containing likely values for a slot. The value that it returns is determined by the <code>valueSelectionStrategy</code> selected when the slot type was created or updated. If <code>valueSelectionStrategy</code> is set to <code>ORIGINAL_VALUE</code>, the value provided by the user is returned, if the user value is similar to the slot values. If <code>valueSelectionStrategy</code> is set to <code>TOP_RESOLUTION</code> Amazon Lex returns the first value in the resolution list or, if there is no resolution list, null. If you don't specify a <code>valueSelectionStrategy</code>, the default is <code>ORIGINAL_VALUE</code>.</p>
    pub slots: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PostTextRequest {
    /// <p>The alias of the Amazon Lex bot.</p>
    #[serde(rename = "botAlias")]
    pub bot_alias: String,
    /// <p>The name of the Amazon Lex bot.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>The text that the user entered (Amazon Lex interprets this text).</p>
    #[serde(rename = "inputText")]
    pub input_text: String,
    /// <p>Request-specific information passed between Amazon Lex and a client application.</p> <p>The namespace <code>x-amz-lex:</code> is reserved for special attributes. Don't create any request attributes with the prefix <code>x-amz-lex:</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html#context-mgmt-request-attribs">Setting Request Attributes</a>.</p>
    #[serde(rename = "requestAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>Application-specific information passed between Amazon Lex and a client application.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html#context-mgmt-session-attribs">Setting Session Attributes</a>.</p>
    #[serde(rename = "sessionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p><p>The ID of the client application user. Amazon Lex uses this to identify a user&#39;s conversation with your bot. At runtime, each request must contain the <code>userID</code> field.</p> <p>To decide the user ID to use for your application, consider the following factors.</p> <ul> <li> <p>The <code>userID</code> field must not contain any personally identifiable information of the user, for example, name, personal identification numbers, or other end user personal information.</p> </li> <li> <p>If you want a user to start a conversation on one device and continue on another device, use a user-specific identifier.</p> </li> <li> <p>If you want the same user to be able to have two independent conversations on two different devices, choose a device-specific identifier.</p> </li> <li> <p>A user can&#39;t have two independent conversations with two different versions of the same bot. For example, a user can&#39;t have a conversation with the PROD and BETA versions of the same bot. If you anticipate that a user will need to have conversation with two different versions, for example, while testing, include the bot alias in the user ID to separate the two conversations.</p> </li> </ul></p>
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PostTextResponse {
    /// <p><p> Identifies the current state of the user interaction. Amazon Lex returns one of the following values as <code>dialogState</code>. The client can optionally use this information to customize the user interface. </p> <ul> <li> <p> <code>ElicitIntent</code> - Amazon Lex wants to elicit user intent. </p> <p>For example, a user might utter an intent (&quot;I want to order a pizza&quot;). If Amazon Lex cannot infer the user intent from this utterance, it will return this dialogState.</p> </li> <li> <p> <code>ConfirmIntent</code> - Amazon Lex is expecting a &quot;yes&quot; or &quot;no&quot; response. </p> <p> For example, Amazon Lex wants user confirmation before fulfilling an intent. </p> <p>Instead of a simple &quot;yes&quot; or &quot;no,&quot; a user might respond with additional information. For example, &quot;yes, but make it thick crust pizza&quot; or &quot;no, I want to order a drink&quot;. Amazon Lex can process such additional information (in these examples, update the crust type slot value, or change intent from OrderPizza to OrderDrink).</p> </li> <li> <p> <code>ElicitSlot</code> - Amazon Lex is expecting a slot value for the current intent. </p> <p>For example, suppose that in the response Amazon Lex sends this message: &quot;What size pizza would you like?&quot;. A user might reply with the slot value (e.g., &quot;medium&quot;). The user might also provide additional information in the response (e.g., &quot;medium thick crust pizza&quot;). Amazon Lex can process such additional information appropriately. </p> </li> <li> <p> <code>Fulfilled</code> - Conveys that the Lambda function configured for the intent has successfully fulfilled the intent. </p> </li> <li> <p> <code>ReadyForFulfillment</code> - Conveys that the client has to fulfill the intent. </p> </li> <li> <p> <code>Failed</code> - Conveys that the conversation with the user failed. </p> <p> This can happen for various reasons including that the user did not provide an appropriate response to prompts from the service (you can configure how many times Amazon Lex can prompt a user for specific information), or the Lambda function failed to fulfill the intent. </p> </li> </ul></p>
    #[serde(rename = "dialogState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_state: Option<String>,
    /// <p>The current user intent that Amazon Lex is aware of.</p>
    #[serde(rename = "intentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    /// <p>The message to convey to the user. The message can come from the bot's configuration or from a Lambda function.</p> <p>If the intent is not configured with a Lambda function, or if the Lambda function returned <code>Delegate</code> as the <code>dialogAction.type</code> its response, Amazon Lex decides on the next course of action and selects an appropriate message from the bot's configuration based on the current interaction context. For example, if Amazon Lex isn't able to understand user input, it uses a clarification prompt message.</p> <p>When you create an intent you can assign messages to groups. When messages are assigned to groups Amazon Lex returns one message from each group in the response. The message field is an escaped JSON string containing the messages. For more information about the structure of the JSON string returned, see <a>msg-prompts-formats</a>.</p> <p>If the Lambda function returns a message, Amazon Lex passes it to the client in its response.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The format of the response message. One of the following values:</p> <ul> <li> <p> <code>PlainText</code> - The message contains plain UTF-8 text.</p> </li> <li> <p> <code>CustomPayload</code> - The message is a custom format defined by the Lambda function.</p> </li> <li> <p> <code>SSML</code> - The message contains text formatted for voice output.</p> </li> <li> <p> <code>Composite</code> - The message contains an escaped JSON object containing one or more messages from the groups that messages were assigned to when the intent was created.</p> </li> </ul></p>
    #[serde(rename = "messageFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_format: Option<String>,
    /// <p>Represents the options that the user has to respond to the current prompt. Response Card can come from the bot configuration (in the Amazon Lex console, choose the settings button next to a slot) or from a code hook (Lambda function). </p>
    #[serde(rename = "responseCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_card: Option<ResponseCard>,
    /// <p>The sentiment expressed in and utterance.</p> <p>When the bot is configured to send utterances to Amazon Comprehend for sentiment analysis, this field contains the result of the analysis.</p>
    #[serde(rename = "sentimentResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_response: Option<SentimentResponse>,
    /// <p>A map of key-value pairs representing the session-specific context information.</p>
    #[serde(rename = "sessionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique identifier for the session.</p>
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// <p>If the <code>dialogState</code> value is <code>ElicitSlot</code>, returns the name of the slot for which Amazon Lex is eliciting a value. </p>
    #[serde(rename = "slotToElicit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_to_elicit: Option<String>,
    /// <p> The intent slots that Amazon Lex detected from the user input in the conversation. </p> <p>Amazon Lex creates a resolution list containing likely values for a slot. The value that it returns is determined by the <code>valueSelectionStrategy</code> selected when the slot type was created or updated. If <code>valueSelectionStrategy</code> is set to <code>ORIGINAL_VALUE</code>, the value provided by the user is returned, if the user value is similar to the slot values. If <code>valueSelectionStrategy</code> is set to <code>TOP_RESOLUTION</code> Amazon Lex returns the first value in the resolution list or, if there is no resolution list, null. If you don't specify a <code>valueSelectionStrategy</code>, the default is <code>ORIGINAL_VALUE</code>.</p>
    #[serde(rename = "slots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<::std::collections::HashMap<String, Option<String>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutSessionRequest {
    /// <p><p>The message that Amazon Lex returns in the response can be either text or speech based depending on the value of this field.</p> <ul> <li> <p>If the value is <code>text/plain; charset=utf-8</code>, Amazon Lex returns text in the response.</p> </li> <li> <p>If the value begins with <code>audio/</code>, Amazon Lex returns speech in the response. Amazon Lex uses Amazon Polly to generate the speech in the configuration that you specify. For example, if you specify <code>audio/mpeg</code> as the value, Amazon Lex returns speech in the MPEG format.</p> </li> <li> <p>If the value is <code>audio/pcm</code>, the speech is returned as <code>audio/pcm</code> in 16-bit, little endian format.</p> </li> <li> <p>The following are the accepted values:</p> <ul> <li> <p> <code>audio/mpeg</code> </p> </li> <li> <p> <code>audio/ogg</code> </p> </li> <li> <p> <code>audio/pcm</code> </p> </li> <li> <p> <code>audio/*</code> (defaults to mpeg)</p> </li> <li> <p> <code>text/plain; charset=utf-8</code> </p> </li> </ul> </li> </ul></p>
    #[serde(rename = "accept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    /// <p>The alias in use for the bot that contains the session data.</p>
    #[serde(rename = "botAlias")]
    pub bot_alias: String,
    /// <p>The name of the bot that contains the session data.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>Sets the next action that the bot should take to fulfill the conversation.</p>
    #[serde(rename = "dialogAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_action: Option<DialogAction>,
    /// <p>A summary of the recent intents for the bot. You can use the intent summary view to set a checkpoint label on an intent and modify attributes of intents. You can also use it to remove or add intent summary objects to the list.</p> <p>An intent that you modify or add to the list must make sense for the bot. For example, the intent name must be valid for the bot. You must provide valid values for:</p> <ul> <li> <p> <code>intentName</code> </p> </li> <li> <p>slot names</p> </li> <li> <p> <code>slotToElict</code> </p> </li> </ul> <p>If you send the <code>recentIntentSummaryView</code> parameter in a <code>PutSession</code> request, the contents of the new summary view replaces the old summary view. For example, if a <code>GetSession</code> request returns three intents in the summary view and you call <code>PutSession</code> with one intent in the summary view, the next call to <code>GetSession</code> will only return one intent.</p>
    #[serde(rename = "recentIntentSummaryView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_intent_summary_view: Option<Vec<IntentSummary>>,
    /// <p>Map of key/value pairs representing the session-specific context information. It contains application information passed between Amazon Lex and a client application.</p>
    #[serde(rename = "sessionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The ID of the client application user. Amazon Lex uses this to identify a user's conversation with your bot. </p>
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutSessionResponse {
    /// <p>The audio version of the message to convey to the user.</p>
    pub audio_stream: Option<bytes::Bytes>,
    /// <p>Content type as specified in the <code>Accept</code> HTTP header in the request.</p>
    pub content_type: Option<String>,
    /// <p><p/> <ul> <li> <p> <code>ConfirmIntent</code> - Amazon Lex is expecting a &quot;yes&quot; or &quot;no&quot; response to confirm the intent before fulfilling an intent.</p> </li> <li> <p> <code>ElicitIntent</code> - Amazon Lex wants to elicit the user&#39;s intent.</p> </li> <li> <p> <code>ElicitSlot</code> - Amazon Lex is expecting the value of a slot for the current intent.</p> </li> <li> <p> <code>Failed</code> - Conveys that the conversation with the user has failed. This can happen for various reasons, including the user does not provide an appropriate response to prompts from the service, or if the Lambda function fails to fulfill the intent.</p> </li> <li> <p> <code>Fulfilled</code> - Conveys that the Lambda function has sucessfully fulfilled the intent.</p> </li> <li> <p> <code>ReadyForFulfillment</code> - Conveys that the client has to fulfill the intent.</p> </li> </ul></p>
    pub dialog_state: Option<String>,
    /// <p>The name of the current intent.</p>
    pub intent_name: Option<String>,
    /// <p>The next message that should be presented to the user.</p>
    pub message: Option<String>,
    /// <p><p>The format of the response message. One of the following values:</p> <ul> <li> <p> <code>PlainText</code> - The message contains plain UTF-8 text.</p> </li> <li> <p> <code>CustomPayload</code> - The message is a custom format for the client.</p> </li> <li> <p> <code>SSML</code> - The message contains text formatted for voice output.</p> </li> <li> <p> <code>Composite</code> - The message contains an escaped JSON object containing one or more messages from the groups that messages were assigned to when the intent was created.</p> </li> </ul></p>
    pub message_format: Option<String>,
    /// <p>Map of key/value pairs representing session-specific context information.</p>
    pub session_attributes: Option<String>,
    /// <p>A unique identifier for the session.</p>
    pub session_id: Option<String>,
    /// <p>If the <code>dialogState</code> is <code>ElicitSlot</code>, returns the name of the slot for which Amazon Lex is eliciting a value.</p>
    pub slot_to_elicit: Option<String>,
    /// <p>Map of zero or more intent slots Amazon Lex detected from the user input during the conversation.</p> <p>Amazon Lex creates a resolution list containing likely values for a slot. The value that it returns is determined by the <code>valueSelectionStrategy</code> selected when the slot type was created or updated. If <code>valueSelectionStrategy</code> is set to <code>ORIGINAL_VALUE</code>, the value provided by the user is returned, if the user value is similar to the slot values. If <code>valueSelectionStrategy</code> is set to <code>TOP_RESOLUTION</code> Amazon Lex returns the first value in the resolution list or, if there is no resolution list, null. If you don't specify a <code>valueSelectionStrategy</code> the default is <code>ORIGINAL_VALUE</code>. </p>
    pub slots: Option<String>,
}

/// <p>If you configure a response card when creating your bots, Amazon Lex substitutes the session attributes and slot values that are available, and then returns it. The response card can also come from a Lambda function ( <code>dialogCodeHook</code> and <code>fulfillmentActivity</code> on an intent).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResponseCard {
    /// <p>The content type of the response.</p>
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>An array of attachment objects representing options.</p>
    #[serde(rename = "genericAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_attachments: Option<Vec<GenericAttachment>>,
    /// <p>The version of the response card format.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>The sentiment expressed in an utterance.</p> <p>When the bot is configured to send utterances to Amazon Comprehend for sentiment analysis, this field structure contains the result of the analysis.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SentimentResponse {
    /// <p>The inferred sentiment that Amazon Comprehend has the highest confidence in.</p>
    #[serde(rename = "sentimentLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_label: Option<String>,
    /// <p>The likelihood that the sentiment was correctly inferred.</p>
    #[serde(rename = "sentimentScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_score: Option<String>,
}

/// Errors returned by DeleteSession
#[derive(Debug, PartialEq)]
pub enum DeleteSessionError {
    /// <p> Request validation failed, there is no usable message in the context, or the bot build failed, is still in progress, or contains unbuilt changes. </p>
    BadRequest(String),
    /// <p> Two clients are using the same AWS account, Amazon Lex bot, and user ID. </p>
    Conflict(String),
    /// <p>Internal service error. Retry the call.</p>
    InternalFailure(String),
    /// <p>Exceeded a limit.</p>
    LimitExceeded(String),
    /// <p>The resource (such as the Amazon Lex bot or an alias) that is referred to is not found.</p>
    NotFound(String),
}

impl DeleteSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSessionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteSessionError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteSessionError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteSessionError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteSessionError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteSessionError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSessionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSessionError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteSessionError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteSessionError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteSessionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteSessionError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSessionError {}
/// Errors returned by GetSession
#[derive(Debug, PartialEq)]
pub enum GetSessionError {
    /// <p> Request validation failed, there is no usable message in the context, or the bot build failed, is still in progress, or contains unbuilt changes. </p>
    BadRequest(String),
    /// <p>Internal service error. Retry the call.</p>
    InternalFailure(String),
    /// <p>Exceeded a limit.</p>
    LimitExceeded(String),
    /// <p>The resource (such as the Amazon Lex bot or an alias) that is referred to is not found.</p>
    NotFound(String),
}

impl GetSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSessionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSessionError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetSessionError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetSessionError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSessionError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSessionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSessionError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetSessionError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetSessionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetSessionError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSessionError {}
/// Errors returned by PostContent
#[derive(Debug, PartialEq)]
pub enum PostContentError {
    /// <p>Either the Amazon Lex bot is still building, or one of the dependent services (Amazon Polly, AWS Lambda) failed with an internal service error.</p>
    BadGateway(String),
    /// <p> Request validation failed, there is no usable message in the context, or the bot build failed, is still in progress, or contains unbuilt changes. </p>
    BadRequest(String),
    /// <p> Two clients are using the same AWS account, Amazon Lex bot, and user ID. </p>
    Conflict(String),
    /// <p><p> One of the dependencies, such as AWS Lambda or Amazon Polly, threw an exception. For example, </p> <ul> <li> <p>If Amazon Lex does not have sufficient permissions to call a Lambda function.</p> </li> <li> <p>If a Lambda function takes longer than 30 seconds to execute.</p> </li> <li> <p>If a fulfillment Lambda function returns a <code>Delegate</code> dialog action without removing any slot values.</p> </li> </ul></p>
    DependencyFailed(String),
    /// <p>Internal service error. Retry the call.</p>
    InternalFailure(String),
    /// <p>Exceeded a limit.</p>
    LimitExceeded(String),
    /// <p>This exception is not used.</p>
    LoopDetected(String),
    /// <p>The accept header in the request does not have a valid value.</p>
    NotAcceptable(String),
    /// <p>The resource (such as the Amazon Lex bot or an alias) that is referred to is not found.</p>
    NotFound(String),
    /// <p>The input speech is too long.</p>
    RequestTimeout(String),
    /// <p>The Content-Type header (<code>PostContent</code> API) has an invalid value. </p>
    UnsupportedMediaType(String),
}

impl PostContentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PostContentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(PostContentError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(PostContentError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(PostContentError::Conflict(err.msg))
                }
                "DependencyFailedException" => {
                    return RusotoError::Service(PostContentError::DependencyFailed(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(PostContentError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PostContentError::LimitExceeded(err.msg))
                }
                "LoopDetectedException" => {
                    return RusotoError::Service(PostContentError::LoopDetected(err.msg))
                }
                "NotAcceptableException" => {
                    return RusotoError::Service(PostContentError::NotAcceptable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PostContentError::NotFound(err.msg))
                }
                "RequestTimeoutException" => {
                    return RusotoError::Service(PostContentError::RequestTimeout(err.msg))
                }
                "UnsupportedMediaTypeException" => {
                    return RusotoError::Service(PostContentError::UnsupportedMediaType(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PostContentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PostContentError::BadGateway(ref cause) => write!(f, "{}", cause),
            PostContentError::BadRequest(ref cause) => write!(f, "{}", cause),
            PostContentError::Conflict(ref cause) => write!(f, "{}", cause),
            PostContentError::DependencyFailed(ref cause) => write!(f, "{}", cause),
            PostContentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            PostContentError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PostContentError::LoopDetected(ref cause) => write!(f, "{}", cause),
            PostContentError::NotAcceptable(ref cause) => write!(f, "{}", cause),
            PostContentError::NotFound(ref cause) => write!(f, "{}", cause),
            PostContentError::RequestTimeout(ref cause) => write!(f, "{}", cause),
            PostContentError::UnsupportedMediaType(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PostContentError {}
/// Errors returned by PostText
#[derive(Debug, PartialEq)]
pub enum PostTextError {
    /// <p>Either the Amazon Lex bot is still building, or one of the dependent services (Amazon Polly, AWS Lambda) failed with an internal service error.</p>
    BadGateway(String),
    /// <p> Request validation failed, there is no usable message in the context, or the bot build failed, is still in progress, or contains unbuilt changes. </p>
    BadRequest(String),
    /// <p> Two clients are using the same AWS account, Amazon Lex bot, and user ID. </p>
    Conflict(String),
    /// <p><p> One of the dependencies, such as AWS Lambda or Amazon Polly, threw an exception. For example, </p> <ul> <li> <p>If Amazon Lex does not have sufficient permissions to call a Lambda function.</p> </li> <li> <p>If a Lambda function takes longer than 30 seconds to execute.</p> </li> <li> <p>If a fulfillment Lambda function returns a <code>Delegate</code> dialog action without removing any slot values.</p> </li> </ul></p>
    DependencyFailed(String),
    /// <p>Internal service error. Retry the call.</p>
    InternalFailure(String),
    /// <p>Exceeded a limit.</p>
    LimitExceeded(String),
    /// <p>This exception is not used.</p>
    LoopDetected(String),
    /// <p>The resource (such as the Amazon Lex bot or an alias) that is referred to is not found.</p>
    NotFound(String),
}

impl PostTextError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PostTextError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(PostTextError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(PostTextError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(PostTextError::Conflict(err.msg))
                }
                "DependencyFailedException" => {
                    return RusotoError::Service(PostTextError::DependencyFailed(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(PostTextError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PostTextError::LimitExceeded(err.msg))
                }
                "LoopDetectedException" => {
                    return RusotoError::Service(PostTextError::LoopDetected(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PostTextError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PostTextError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PostTextError::BadGateway(ref cause) => write!(f, "{}", cause),
            PostTextError::BadRequest(ref cause) => write!(f, "{}", cause),
            PostTextError::Conflict(ref cause) => write!(f, "{}", cause),
            PostTextError::DependencyFailed(ref cause) => write!(f, "{}", cause),
            PostTextError::InternalFailure(ref cause) => write!(f, "{}", cause),
            PostTextError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PostTextError::LoopDetected(ref cause) => write!(f, "{}", cause),
            PostTextError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PostTextError {}
/// Errors returned by PutSession
#[derive(Debug, PartialEq)]
pub enum PutSessionError {
    /// <p>Either the Amazon Lex bot is still building, or one of the dependent services (Amazon Polly, AWS Lambda) failed with an internal service error.</p>
    BadGateway(String),
    /// <p> Request validation failed, there is no usable message in the context, or the bot build failed, is still in progress, or contains unbuilt changes. </p>
    BadRequest(String),
    /// <p> Two clients are using the same AWS account, Amazon Lex bot, and user ID. </p>
    Conflict(String),
    /// <p><p> One of the dependencies, such as AWS Lambda or Amazon Polly, threw an exception. For example, </p> <ul> <li> <p>If Amazon Lex does not have sufficient permissions to call a Lambda function.</p> </li> <li> <p>If a Lambda function takes longer than 30 seconds to execute.</p> </li> <li> <p>If a fulfillment Lambda function returns a <code>Delegate</code> dialog action without removing any slot values.</p> </li> </ul></p>
    DependencyFailed(String),
    /// <p>Internal service error. Retry the call.</p>
    InternalFailure(String),
    /// <p>Exceeded a limit.</p>
    LimitExceeded(String),
    /// <p>The accept header in the request does not have a valid value.</p>
    NotAcceptable(String),
    /// <p>The resource (such as the Amazon Lex bot or an alias) that is referred to is not found.</p>
    NotFound(String),
}

impl PutSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutSessionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(PutSessionError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(PutSessionError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(PutSessionError::Conflict(err.msg))
                }
                "DependencyFailedException" => {
                    return RusotoError::Service(PutSessionError::DependencyFailed(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(PutSessionError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutSessionError::LimitExceeded(err.msg))
                }
                "NotAcceptableException" => {
                    return RusotoError::Service(PutSessionError::NotAcceptable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutSessionError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutSessionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutSessionError::BadGateway(ref cause) => write!(f, "{}", cause),
            PutSessionError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutSessionError::Conflict(ref cause) => write!(f, "{}", cause),
            PutSessionError::DependencyFailed(ref cause) => write!(f, "{}", cause),
            PutSessionError::InternalFailure(ref cause) => write!(f, "{}", cause),
            PutSessionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutSessionError::NotAcceptable(ref cause) => write!(f, "{}", cause),
            PutSessionError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutSessionError {}
/// Trait representing the capabilities of the Amazon Lex Runtime Service API. Amazon Lex Runtime Service clients implement this trait.
#[async_trait]
pub trait LexRuntime {
    /// <p>Removes session information for a specified bot, alias, and user ID. </p>
    async fn delete_session(
        &self,
        input: DeleteSessionRequest,
    ) -> Result<DeleteSessionResponse, RusotoError<DeleteSessionError>>;

    /// <p>Returns session information for a specified bot, alias, and user ID.</p>
    async fn get_session(
        &self,
        input: GetSessionRequest,
    ) -> Result<GetSessionResponse, RusotoError<GetSessionError>>;

    /// <p> Sends user input (text or speech) to Amazon Lex. Clients use this API to send text and audio requests to Amazon Lex at runtime. Amazon Lex interprets the user input using the machine learning model that it built for the bot. </p> <p>The <code>PostContent</code> operation supports audio input at 8kHz and 16kHz. You can use 8kHz audio to achieve higher speech recognition accuracy in telephone audio applications. </p> <p> In response, Amazon Lex returns the next message to convey to the user. Consider the following example messages: </p> <ul> <li> <p> For a user input "I would like a pizza," Amazon Lex might return a response with a message eliciting slot data (for example, <code>PizzaSize</code>): "What size pizza would you like?". </p> </li> <li> <p> After the user provides all of the pizza order information, Amazon Lex might return a response with a message to get user confirmation: "Order the pizza?". </p> </li> <li> <p> After the user replies "Yes" to the confirmation prompt, Amazon Lex might return a conclusion statement: "Thank you, your cheese pizza has been ordered.". </p> </li> </ul> <p> Not all Amazon Lex messages require a response from the user. For example, conclusion statements do not require a response. Some messages require only a yes or no response. In addition to the <code>message</code>, Amazon Lex provides additional context about the message in the response that you can use to enhance client behavior, such as displaying the appropriate client user interface. Consider the following examples: </p> <ul> <li> <p> If the message is to elicit slot data, Amazon Lex returns the following context information: </p> <ul> <li> <p> <code>x-amz-lex-dialog-state</code> header set to <code>ElicitSlot</code> </p> </li> <li> <p> <code>x-amz-lex-intent-name</code> header set to the intent name in the current context </p> </li> <li> <p> <code>x-amz-lex-slot-to-elicit</code> header set to the slot name for which the <code>message</code> is eliciting information </p> </li> <li> <p> <code>x-amz-lex-slots</code> header set to a map of slots configured for the intent with their current values </p> </li> </ul> </li> <li> <p> If the message is a confirmation prompt, the <code>x-amz-lex-dialog-state</code> header is set to <code>Confirmation</code> and the <code>x-amz-lex-slot-to-elicit</code> header is omitted. </p> </li> <li> <p> If the message is a clarification prompt configured for the intent, indicating that the user intent is not understood, the <code>x-amz-dialog-state</code> header is set to <code>ElicitIntent</code> and the <code>x-amz-slot-to-elicit</code> header is omitted. </p> </li> </ul> <p> In addition, Amazon Lex also returns your application-specific <code>sessionAttributes</code>. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html">Managing Conversation Context</a>. </p>
    async fn post_content(
        &self,
        input: PostContentRequest,
    ) -> Result<PostContentResponse, RusotoError<PostContentError>>;

    /// <p>Sends user input to Amazon Lex. Client applications can use this API to send requests to Amazon Lex at runtime. Amazon Lex then interprets the user input using the machine learning model it built for the bot. </p> <p> In response, Amazon Lex returns the next <code>message</code> to convey to the user an optional <code>responseCard</code> to display. Consider the following example messages: </p> <ul> <li> <p> For a user input "I would like a pizza", Amazon Lex might return a response with a message eliciting slot data (for example, PizzaSize): "What size pizza would you like?" </p> </li> <li> <p> After the user provides all of the pizza order information, Amazon Lex might return a response with a message to obtain user confirmation "Proceed with the pizza order?". </p> </li> <li> <p> After the user replies to a confirmation prompt with a "yes", Amazon Lex might return a conclusion statement: "Thank you, your cheese pizza has been ordered.". </p> </li> </ul> <p> Not all Amazon Lex messages require a user response. For example, a conclusion statement does not require a response. Some messages require only a "yes" or "no" user response. In addition to the <code>message</code>, Amazon Lex provides additional context about the message in the response that you might use to enhance client behavior, for example, to display the appropriate client user interface. These are the <code>slotToElicit</code>, <code>dialogState</code>, <code>intentName</code>, and <code>slots</code> fields in the response. Consider the following examples: </p> <ul> <li> <p>If the message is to elicit slot data, Amazon Lex returns the following context information:</p> <ul> <li> <p> <code>dialogState</code> set to ElicitSlot </p> </li> <li> <p> <code>intentName</code> set to the intent name in the current context </p> </li> <li> <p> <code>slotToElicit</code> set to the slot name for which the <code>message</code> is eliciting information </p> </li> <li> <p> <code>slots</code> set to a map of slots, configured for the intent, with currently known values </p> </li> </ul> </li> <li> <p> If the message is a confirmation prompt, the <code>dialogState</code> is set to ConfirmIntent and <code>SlotToElicit</code> is set to null. </p> </li> <li> <p>If the message is a clarification prompt (configured for the intent) that indicates that user intent is not understood, the <code>dialogState</code> is set to ElicitIntent and <code>slotToElicit</code> is set to null. </p> </li> </ul> <p> In addition, Amazon Lex also returns your application-specific <code>sessionAttributes</code>. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html">Managing Conversation Context</a>. </p>
    async fn post_text(
        &self,
        input: PostTextRequest,
    ) -> Result<PostTextResponse, RusotoError<PostTextError>>;

    /// <p>Creates a new session or modifies an existing session with an Amazon Lex bot. Use this operation to enable your application to set the state of the bot.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/how-session-api.html">Managing Sessions</a>.</p>
    async fn put_session(
        &self,
        input: PutSessionRequest,
    ) -> Result<PutSessionResponse, RusotoError<PutSessionError>>;
}
/// A client for the Amazon Lex Runtime Service API.
#[derive(Clone)]
pub struct LexRuntimeClient {
    client: Client,
    region: region::Region,
}

impl LexRuntimeClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> LexRuntimeClient {
        LexRuntimeClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> LexRuntimeClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        LexRuntimeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> LexRuntimeClient {
        LexRuntimeClient { client, region }
    }
}

#[async_trait]
impl LexRuntime for LexRuntimeClient {
    /// <p>Removes session information for a specified bot, alias, and user ID. </p>
    async fn delete_session(
        &self,
        input: DeleteSessionRequest,
    ) -> Result<DeleteSessionResponse, RusotoError<DeleteSessionError>> {
        let request_uri = format!(
            "/bot/{bot_name}/alias/{bot_alias}/user/{user_id}/session",
            bot_alias = input.bot_alias,
            bot_name = input.bot_name,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("runtime.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteSessionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSessionError::from_response(response))
        }
    }

    /// <p>Returns session information for a specified bot, alias, and user ID.</p>
    async fn get_session(
        &self,
        input: GetSessionRequest,
    ) -> Result<GetSessionResponse, RusotoError<GetSessionError>> {
        let request_uri = format!(
            "/bot/{bot_name}/alias/{bot_alias}/user/{user_id}/session/",
            bot_alias = input.bot_alias,
            bot_name = input.bot_name,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("runtime.lex".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.checkpoint_label_filter {
            params.put("checkpointLabelFilter", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSessionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSessionError::from_response(response))
        }
    }

    /// <p> Sends user input (text or speech) to Amazon Lex. Clients use this API to send text and audio requests to Amazon Lex at runtime. Amazon Lex interprets the user input using the machine learning model that it built for the bot. </p> <p>The <code>PostContent</code> operation supports audio input at 8kHz and 16kHz. You can use 8kHz audio to achieve higher speech recognition accuracy in telephone audio applications. </p> <p> In response, Amazon Lex returns the next message to convey to the user. Consider the following example messages: </p> <ul> <li> <p> For a user input "I would like a pizza," Amazon Lex might return a response with a message eliciting slot data (for example, <code>PizzaSize</code>): "What size pizza would you like?". </p> </li> <li> <p> After the user provides all of the pizza order information, Amazon Lex might return a response with a message to get user confirmation: "Order the pizza?". </p> </li> <li> <p> After the user replies "Yes" to the confirmation prompt, Amazon Lex might return a conclusion statement: "Thank you, your cheese pizza has been ordered.". </p> </li> </ul> <p> Not all Amazon Lex messages require a response from the user. For example, conclusion statements do not require a response. Some messages require only a yes or no response. In addition to the <code>message</code>, Amazon Lex provides additional context about the message in the response that you can use to enhance client behavior, such as displaying the appropriate client user interface. Consider the following examples: </p> <ul> <li> <p> If the message is to elicit slot data, Amazon Lex returns the following context information: </p> <ul> <li> <p> <code>x-amz-lex-dialog-state</code> header set to <code>ElicitSlot</code> </p> </li> <li> <p> <code>x-amz-lex-intent-name</code> header set to the intent name in the current context </p> </li> <li> <p> <code>x-amz-lex-slot-to-elicit</code> header set to the slot name for which the <code>message</code> is eliciting information </p> </li> <li> <p> <code>x-amz-lex-slots</code> header set to a map of slots configured for the intent with their current values </p> </li> </ul> </li> <li> <p> If the message is a confirmation prompt, the <code>x-amz-lex-dialog-state</code> header is set to <code>Confirmation</code> and the <code>x-amz-lex-slot-to-elicit</code> header is omitted. </p> </li> <li> <p> If the message is a clarification prompt configured for the intent, indicating that the user intent is not understood, the <code>x-amz-dialog-state</code> header is set to <code>ElicitIntent</code> and the <code>x-amz-slot-to-elicit</code> header is omitted. </p> </li> </ul> <p> In addition, Amazon Lex also returns your application-specific <code>sessionAttributes</code>. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html">Managing Conversation Context</a>. </p>
    async fn post_content(
        &self,
        input: PostContentRequest,
    ) -> Result<PostContentResponse, RusotoError<PostContentError>> {
        let request_uri = format!(
            "/bot/{bot_name}/alias/{bot_alias}/user/{user_id}/content",
            bot_alias = input.bot_alias,
            bot_name = input.bot_name,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("runtime.lex".to_string());
        let encoded = Some(input.input_stream.to_owned());
        request.set_payload(encoded);

        if let Some(ref accept) = input.accept {
            request.add_header("Accept", &accept.to_string());
        }
        request.add_header("Content-Type", &input.content_type);

        if let Some(ref request_attributes) = input.request_attributes {
            request.add_header(
                "x-amz-lex-request-attributes",
                &request_attributes.to_string(),
            );
        }

        if let Some(ref session_attributes) = input.session_attributes {
            request.add_header(
                "x-amz-lex-session-attributes",
                &session_attributes.to_string(),
            );
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = PostContentResponse::default();
            result.audio_stream = Some(response.body);

            if let Some(content_type) = response.headers.get("Content-Type") {
                let value = content_type.to_owned();
                result.content_type = Some(value)
            };
            if let Some(dialog_state) = response.headers.get("x-amz-lex-dialog-state") {
                let value = dialog_state.to_owned();
                result.dialog_state = Some(value)
            };
            if let Some(input_transcript) = response.headers.get("x-amz-lex-input-transcript") {
                let value = input_transcript.to_owned();
                result.input_transcript = Some(value)
            };
            if let Some(intent_name) = response.headers.get("x-amz-lex-intent-name") {
                let value = intent_name.to_owned();
                result.intent_name = Some(value)
            };
            if let Some(message) = response.headers.get("x-amz-lex-message") {
                let value = message.to_owned();
                result.message = Some(value)
            };
            if let Some(message_format) = response.headers.get("x-amz-lex-message-format") {
                let value = message_format.to_owned();
                result.message_format = Some(value)
            };
            if let Some(sentiment_response) = response.headers.get("x-amz-lex-sentiment") {
                let value = sentiment_response.to_owned();
                result.sentiment_response = Some(value)
            };
            if let Some(session_attributes) = response.headers.get("x-amz-lex-session-attributes") {
                let value = session_attributes.to_owned();
                result.session_attributes = Some(value)
            };
            if let Some(session_id) = response.headers.get("x-amz-lex-session-id") {
                let value = session_id.to_owned();
                result.session_id = Some(value)
            };
            if let Some(slot_to_elicit) = response.headers.get("x-amz-lex-slot-to-elicit") {
                let value = slot_to_elicit.to_owned();
                result.slot_to_elicit = Some(value)
            };
            if let Some(slots) = response.headers.get("x-amz-lex-slots") {
                let value = slots.to_owned();
                result.slots = Some(value)
            };

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PostContentError::from_response(response))
        }
    }

    /// <p>Sends user input to Amazon Lex. Client applications can use this API to send requests to Amazon Lex at runtime. Amazon Lex then interprets the user input using the machine learning model it built for the bot. </p> <p> In response, Amazon Lex returns the next <code>message</code> to convey to the user an optional <code>responseCard</code> to display. Consider the following example messages: </p> <ul> <li> <p> For a user input "I would like a pizza", Amazon Lex might return a response with a message eliciting slot data (for example, PizzaSize): "What size pizza would you like?" </p> </li> <li> <p> After the user provides all of the pizza order information, Amazon Lex might return a response with a message to obtain user confirmation "Proceed with the pizza order?". </p> </li> <li> <p> After the user replies to a confirmation prompt with a "yes", Amazon Lex might return a conclusion statement: "Thank you, your cheese pizza has been ordered.". </p> </li> </ul> <p> Not all Amazon Lex messages require a user response. For example, a conclusion statement does not require a response. Some messages require only a "yes" or "no" user response. In addition to the <code>message</code>, Amazon Lex provides additional context about the message in the response that you might use to enhance client behavior, for example, to display the appropriate client user interface. These are the <code>slotToElicit</code>, <code>dialogState</code>, <code>intentName</code>, and <code>slots</code> fields in the response. Consider the following examples: </p> <ul> <li> <p>If the message is to elicit slot data, Amazon Lex returns the following context information:</p> <ul> <li> <p> <code>dialogState</code> set to ElicitSlot </p> </li> <li> <p> <code>intentName</code> set to the intent name in the current context </p> </li> <li> <p> <code>slotToElicit</code> set to the slot name for which the <code>message</code> is eliciting information </p> </li> <li> <p> <code>slots</code> set to a map of slots, configured for the intent, with currently known values </p> </li> </ul> </li> <li> <p> If the message is a confirmation prompt, the <code>dialogState</code> is set to ConfirmIntent and <code>SlotToElicit</code> is set to null. </p> </li> <li> <p>If the message is a clarification prompt (configured for the intent) that indicates that user intent is not understood, the <code>dialogState</code> is set to ElicitIntent and <code>slotToElicit</code> is set to null. </p> </li> </ul> <p> In addition, Amazon Lex also returns your application-specific <code>sessionAttributes</code>. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html">Managing Conversation Context</a>. </p>
    async fn post_text(
        &self,
        input: PostTextRequest,
    ) -> Result<PostTextResponse, RusotoError<PostTextError>> {
        let request_uri = format!(
            "/bot/{bot_name}/alias/{bot_alias}/user/{user_id}/text",
            bot_alias = input.bot_alias,
            bot_name = input.bot_name,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("runtime.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PostTextResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PostTextError::from_response(response))
        }
    }

    /// <p>Creates a new session or modifies an existing session with an Amazon Lex bot. Use this operation to enable your application to set the state of the bot.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/how-session-api.html">Managing Sessions</a>.</p>
    async fn put_session(
        &self,
        input: PutSessionRequest,
    ) -> Result<PutSessionResponse, RusotoError<PutSessionError>> {
        let request_uri = format!(
            "/bot/{bot_name}/alias/{bot_alias}/user/{user_id}/session",
            bot_alias = input.bot_alias,
            bot_name = input.bot_name,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("runtime.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref accept) = input.accept {
            request.add_header("Accept", &accept.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = PutSessionResponse::default();
            result.audio_stream = Some(response.body);

            if let Some(content_type) = response.headers.get("Content-Type") {
                let value = content_type.to_owned();
                result.content_type = Some(value)
            };
            if let Some(dialog_state) = response.headers.get("x-amz-lex-dialog-state") {
                let value = dialog_state.to_owned();
                result.dialog_state = Some(value)
            };
            if let Some(intent_name) = response.headers.get("x-amz-lex-intent-name") {
                let value = intent_name.to_owned();
                result.intent_name = Some(value)
            };
            if let Some(message) = response.headers.get("x-amz-lex-message") {
                let value = message.to_owned();
                result.message = Some(value)
            };
            if let Some(message_format) = response.headers.get("x-amz-lex-message-format") {
                let value = message_format.to_owned();
                result.message_format = Some(value)
            };
            if let Some(session_attributes) = response.headers.get("x-amz-lex-session-attributes") {
                let value = session_attributes.to_owned();
                result.session_attributes = Some(value)
            };
            if let Some(session_id) = response.headers.get("x-amz-lex-session-id") {
                let value = session_id.to_owned();
                result.session_id = Some(value)
            };
            if let Some(slot_to_elicit) = response.headers.get("x-amz-lex-slot-to-elicit") {
                let value = slot_to_elicit.to_owned();
                result.slot_to_elicit = Some(value)
            };
            if let Some(slots) = response.headers.get("x-amz-lex-slots") {
                let value = slots.to_owned();
                result.slots = Some(value)
            };

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutSessionError::from_response(response))
        }
    }
}
