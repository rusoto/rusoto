
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

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[doc="<p>Represents an option to be shown on the client platform (Facebook, Slack, etc.)</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Button {
    #[doc="<p>Text that is visible to the user on the button.</p>"]
    #[serde(rename="text")]
    pub text: String,
    #[doc="<p>The value sent to Amazon Lex when a user chooses the button. For example, consider button text \"NYC.\" When the user chooses the button, the value sent can be \"New York City.\"</p>"]
    #[serde(rename="value")]
    pub value: String,
}

#[doc="<p>Represents an option rendered to the user when a prompt is shown. It could be an image, a button, a link, or text. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GenericAttachment {
    #[doc="<p>The URL of an attachment to the response card.</p>"]
    #[serde(rename="attachmentLinkUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attachment_link_url: Option<String>,
    #[doc="<p>The list of options to show to the user.</p>"]
    #[serde(rename="buttons")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub buttons: Option<Vec<Button>>,
    #[doc="<p>The URL of an image that is displayed to the user.</p>"]
    #[serde(rename="imageUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_url: Option<String>,
    #[doc="<p>The subtitle shown below the title.</p>"]
    #[serde(rename="subTitle")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sub_title: Option<String>,
    #[doc="<p>The title of the option.</p>"]
    #[serde(rename="title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PostContentRequest {
    #[doc="<p> You pass this value as the <code>Accept</code> HTTP header. </p> <p> The message Amazon Lex returns in the response can be either text or speech based on the <code>Accept</code> HTTP header value in the request. </p> <ul> <li> <p> If the value is <code>text/plain; charset=utf-8</code>, Amazon Lex returns text in the response. </p> </li> <li> <p> If the value begins with <code>audio/</code>, Amazon Lex returns speech in the response. Amazon Lex uses Amazon Polly to generate the speech (using the configuration you specified in the <code>Accept</code> header). For example, if you specify <code>audio/mpeg</code> as the value, Amazon Lex returns speech in the MPEG format.</p> <p>The following are the accepted values:</p> <ul> <li> <p>audio/mpeg</p> </li> <li> <p>audio/ogg</p> </li> <li> <p>audio/pcm</p> </li> <li> <p>text/plain; charset=utf-8</p> </li> <li> <p>audio/* (defaults to mpeg)</p> </li> </ul> </li> </ul>"]
    #[serde(rename="accept")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept: Option<String>,
    #[doc="<p>Alias of the Amazon Lex bot.</p>"]
    #[serde(rename="botAlias")]
    pub bot_alias: String,
    #[doc="<p>Name of the Amazon Lex bot.</p>"]
    #[serde(rename="botName")]
    pub bot_name: String,
    #[doc="<p> You pass this values as the <code>Content-Type</code> HTTP header. </p> <p> Indicates the audio format or text. The header value must start with one of the following prefixes: </p> <ul> <li> <p>PCM format</p> <ul> <li> <p>audio/l16; rate=16000; channels=1</p> </li> <li> <p>audio/x-l16; sample-rate=16000; channel-count=1</p> </li> </ul> </li> <li> <p>Opus format</p> <ul> <li> <p>audio/x-cbr-opus-with-preamble; preamble-size=0; bit-rate=1; frame-size-milliseconds=1.1</p> </li> </ul> </li> <li> <p>Text format</p> <ul> <li> <p>text/plain; charset=utf-8</p> </li> </ul> </li> </ul>"]
    #[serde(rename="contentType")]
    pub content_type: String,
    #[doc="<p> User input in PCM or Opus audio format or text format as described in the <code>Content-Type</code> HTTP header. </p>"]
    #[serde(rename="inputStream")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub input_stream: Vec<u8>,
    #[doc="<p>You pass this value in the <code>x-amz-lex-session-attributes</code> HTTP header. The value must be map (keys and values must be strings) that is JSON serialized and then base64 encoded.</p> <p> A session represents dialog between a user and Amazon Lex. At runtime, a client application can pass contextual information, in the request to Amazon Lex. For example, </p> <ul> <li> <p>You might use session attributes to track the requestID of user requests.</p> </li> <li> <p>In Getting Started Exercise 1, the example bot uses the price session attribute to maintain the price of flowers ordered (for example, \"price\":25). The code hook (Lambda function) sets this attribute based on the type of flowers ordered. For more information, see <a href=\"http://docs.aws.amazon.com/lex/latest/dg/gs-bp-details-after-lambda.html\">Review the Details of Information Flow</a>. </p> </li> <li> <p>In the BookTrip bot exercise, the bot uses the <code>currentReservation</code> session attribute to maintains the slot data during the in-progress conversation to book a hotel or book a car. For more information, see <a href=\"http://docs.aws.amazon.com/lex/latest/dg/book-trip-detail-flow.html\">Details of Information Flow</a>. </p> </li> </ul> <p> Amazon Lex passes these session attributes to the Lambda functions configured for the intent In the your Lambda function, you can use the session attributes for initialization and customization (prompts). Some examples are: </p> <ul> <li> <p> Initialization - In a pizza ordering bot, if you pass user location (for example, <code>\"Location : 111 Maple Street\"</code>), then your Lambda function might use this information to determine the closest pizzeria to place the order (and perhaps set the storeAddress slot value as well). </p> <p> Personalized prompts - For example, you can configure prompts to refer to the user by name (for example, \"Hey [firstName], what toppings would you like?\"). You can pass the user's name as a session attribute (\"firstName\": \"Joe\") so that Amazon Lex can substitute the placeholder to provide a personalized prompt to the user (\"Hey Joe, what toppings would you like?\"). </p> </li> </ul> <note> <p> Amazon Lex does not persist session attributes. </p> <p> If you configured a code hook for the intent, Amazon Lex passes the incoming session attributes to the Lambda function. The Lambda function must return these session attributes if you want Amazon Lex to return them to the client. </p> <p> If there is no code hook configured for the intent Amazon Lex simply returns the session attributes to the client application. </p> </note>"]
    #[serde(rename="sessionAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session_attributes: Option<String>,
    #[doc="<p>ID of the client application user. Typically, each of your application users should have a unique ID. The application developer decides the user IDs. At runtime, each request must include the user ID. Note the following considerations:</p> <ul> <li> <p> If you want a user to start conversation on one device and continue the conversation on another device, you might choose a user-specific identifier, such as the user's login, or Amazon Cognito user ID (assuming your application is using Amazon Cognito). </p> </li> <li> <p> If you want the same user to be able to have two independent conversations on two different devices, you might choose device-specific identifier, such as device ID, or some globally unique identifier. </p> </li> </ul>"]
    #[serde(rename="userId")]
    pub user_id: String,
}

#[derive(Default,Debug,Clone)]
pub struct PostContentResponse {
    #[doc="<p>The prompt (or statement) to convey to the user. This is based on the bot configuration and context. For example, if Amazon Lex did not understand the user intent, it sends the <code>clarificationPrompt</code> configured for the bot. If the intent requires confirmation before taking the fulfillment action, it sends the <code>confirmationPrompt</code>. Another example: Suppose that the Lambda function successfully fulfilled the intent, and sent a message to convey to the user. Then Amazon Lex sends that message in the response. </p>"]
    pub audio_stream: Option<Vec<u8>>,
    #[doc="<p>Content type as specified in the <code>Accept</code> HTTP header in the request.</p>"]
    pub content_type: Option<String>,
    #[doc="<p>Identifies the current state of the user interaction. Amazon Lex returns one of the following values as <code>dialogState</code>. The client can optionally use this information to customize the user interface. </p> <ul> <li> <p> <code>ElicitIntent</code> – Amazon Lex wants to elicit the user's intent. Consider the following examples: </p> <p> For example, a user might utter an intent (\"I want to order a pizza\"). If Amazon Lex cannot infer the user intent from this utterance, it will return this dialog state. </p> </li> <li> <p> <code>ConfirmIntent</code> – Amazon Lex is expecting a \"yes\" or \"no\" response. </p> <p>For example, Amazon Lex wants user confirmation before fulfilling an intent. Instead of a simple \"yes\" or \"no\" response, a user might respond with additional information. For example, \"yes, but make it a thick crust pizza\" or \"no, I want to order a drink.\" Amazon Lex can process such additional information (in these examples, update the crust type slot or change the intent from OrderPizza to OrderDrink). </p> </li> <li> <p> <code>ElicitSlot</code> – Amazon Lex is expecting the value of a slot for the current intent. </p> <p> For example, suppose that in the response Amazon Lex sends this message: \"What size pizza would you like?\". A user might reply with the slot value (e.g., \"medium\"). The user might also provide additional information in the response (e.g., \"medium thick crust pizza\"). Amazon Lex can process such additional information appropriately. </p> </li> <li> <p> <code>Fulfilled</code> – Conveys that the Lambda function has successfully fulfilled the intent. </p> </li> <li> <p> <code>ReadyForFulfillment</code> – Conveys that the client has to fullfill the request. </p> </li> <li> <p> <code>Failed</code> – Conveys that the conversation with the user failed. </p> <p> This can happen for various reasons, including that the user does not provide an appropriate response to prompts from the service (you can configure how many times Amazon Lex can prompt a user for specific information), or if the Lambda function fails to fulfill the intent. </p> </li> </ul>"]
    pub dialog_state: Option<String>,
    #[doc="<p>Transcript of the voice input to the operation.</p>"]
    pub input_transcript: Option<String>,
    #[doc="<p>Current user intent that Amazon Lex is aware of.</p>"]
    pub intent_name: Option<String>,
    #[doc="<p> Message to convey to the user. It can come from the bot's configuration or a code hook (Lambda function). If the current intent is not configured with a code hook or if the code hook returned <code>Delegate</code> as the <code>dialogAction.type</code> in its response, then Amazon Lex decides the next course of action and selects an appropriate message from the bot configuration based on the current user interaction context. For example, if Amazon Lex is not able to understand the user input, it uses a clarification prompt message (For more information, see the Error Handling section in the Amazon Lex console). Another example: if the intent requires confirmation before fulfillment, then Amazon Lex uses the confirmation prompt message in the intent configuration. If the code hook returns a message, Amazon Lex passes it as-is in its response to the client. </p>"]
    pub message: Option<String>,
    #[doc="<p> Map of key/value pairs representing the session-specific context information. </p>"]
    pub session_attributes: Option<String>,
    #[doc="<p> If the <code>dialogState</code> value is <code>ElicitSlot</code>, returns the name of the slot for which Amazon Lex is eliciting a value. </p>"]
    pub slot_to_elicit: Option<String>,
    #[doc="<p>Map of zero or more intent slots (name/value pairs) Amazon Lex detected from the user input during the conversation.</p>"]
    pub slots: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PostTextRequest {
    #[doc="<p>The alias of the Amazon Lex bot.</p>"]
    #[serde(rename="botAlias")]
    pub bot_alias: String,
    #[doc="<p>The name of the Amazon Lex bot.</p>"]
    #[serde(rename="botName")]
    pub bot_name: String,
    #[doc="<p>The text that the user entered (Amazon Lex interprets this text).</p>"]
    #[serde(rename="inputText")]
    pub input_text: String,
    #[doc="<p> By using session attributes, a client application can pass contextual information in the request to Amazon Lex For example, </p> <ul> <li> <p>In Getting Started Exercise 1, the example bot uses the <code>price</code> session attribute to maintain the price of the flowers ordered (for example, \"Price\":25). The code hook (the Lambda function) sets this attribute based on the type of flowers ordered. For more information, see <a href=\"http://docs.aws.amazon.com/lex/latest/dg/gs-bp-details-after-lambda.html\">Review the Details of Information Flow</a>. </p> </li> <li> <p>In the BookTrip bot exercise, the bot uses the <code>currentReservation</code> session attribute to maintain slot data during the in-progress conversation to book a hotel or book a car. For more information, see <a href=\"http://docs.aws.amazon.com/lex/latest/dg/book-trip-detail-flow.html\">Details of Information Flow</a>. </p> </li> <li> <p>You might use the session attributes (key, value pairs) to track the requestID of user requests.</p> </li> </ul> <p> Amazon Lex simply passes these session attributes to the Lambda functions configured for the intent.</p> <p>In your Lambda function, you can also use the session attributes for initialization and customization (prompts and response cards). Some examples are:</p> <ul> <li> <p> Initialization - In a pizza ordering bot, if you can pass the user location as a session attribute (for example, <code>\"Location\" : \"111 Maple street\"</code>), then your Lambda function might use this information to determine the closest pizzeria to place the order (perhaps to set the storeAddress slot value). </p> </li> <li> <p> Personalize prompts - For example, you can configure prompts to refer to the user name. (For example, \"Hey [FirstName], what toppings would you like?\"). You can pass the user name as a session attribute (<code>\"FirstName\" : \"Joe\"</code>) so that Amazon Lex can substitute the placeholder to provide a personalize prompt to the user (\"Hey Joe, what toppings would you like?\"). </p> </li> </ul> <note> <p> Amazon Lex does not persist session attributes. </p> <p> If you configure a code hook for the intent, Amazon Lex passes the incoming session attributes to the Lambda function. If you want Amazon Lex to return these session attributes back to the client, the Lambda function must return them. </p> <p> If there is no code hook configured for the intent, Amazon Lex simply returns the session attributes back to the client application. </p> </note>"]
    #[serde(rename="sessionAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session_attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The ID of the client application user. The application developer decides the user IDs. At runtime, each request must include the user ID. Typically, each of your application users should have a unique ID. Note the following considerations: </p> <ul> <li> <p> If you want a user to start a conversation on one device and continue the conversation on another device, you might choose a user-specific identifier, such as a login or Amazon Cognito user ID (assuming your application is using Amazon Cognito). </p> </li> <li> <p> If you want the same user to be able to have two independent conversations on two different devices, you might choose a device-specific identifier, such as device ID, or some globally unique identifier. </p> </li> </ul>"]
    #[serde(rename="userId")]
    pub user_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct PostTextResponse {
    #[doc="<p> Identifies the current state of the user interaction. Amazon Lex returns one of the following values as <code>dialogState</code>. The client can optionally use this information to customize the user interface. </p> <ul> <li> <p> <code>ElicitIntent</code> – Amazon Lex wants to elicit user intent. </p> <p>For example, a user might utter an intent (\"I want to order a pizza\"). If Amazon Lex cannot infer the user intent from this utterance, it will return this dialogState.</p> </li> <li> <p> <code>ConfirmIntent</code> – Amazon Lex is expecting a \"yes\" or \"no\" response. </p> <p> For example, Amazon Lex wants user confirmation before fulfilling an intent. </p> <p>Instead of a simple \"yes\" or \"no,\" a user might respond with additional information. For example, \"yes, but make it thick crust pizza\" or \"no, I want to order a drink\". Amazon Lex can process such additional information (in these examples, update the crust type slot value, or change intent from OrderPizza to OrderDrink).</p> </li> <li> <p> <code>ElicitSlot</code> – Amazon Lex is expecting a slot value for the current intent. </p> <p>For example, suppose that in the response Amazon Lex sends this message: \"What size pizza would you like?\". A user might reply with the slot value (e.g., \"medium\"). The user might also provide additional information in the response (e.g., \"medium thick crust pizza\"). Amazon Lex can process such additional information appropriately. </p> </li> <li> <p> <code>Fulfilled</code> – Conveys that the Lambda function configured for the intent has successfully fulfilled the intent. </p> </li> <li> <p> <code>ReadyForFulfillment</code> – Conveys that the client has to fulfill the intent. </p> </li> <li> <p> <code>Failed</code> – Conveys that the conversation with the user failed. </p> <p> This can happen for various reasons including that the user did not provide an appropriate response to prompts from the service (you can configure how many times Amazon Lex can prompt a user for specific information), or the Lambda function failed to fulfill the intent. </p> </li> </ul>"]
    #[serde(rename="dialogState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dialog_state: Option<String>,
    #[doc="<p>The current user intent that Amazon Lex is aware of.</p>"]
    #[serde(rename="intentName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub intent_name: Option<String>,
    #[doc="<p> A message to convey to the user. It can come from the bot's configuration or a code hook (Lambda function). If the current intent is not configured with a code hook or the code hook returned <code>Delegate</code> as the <code>dialogAction.type</code> in its response, then Amazon Lex decides the next course of action and selects an appropriate message from the bot configuration based on the current user interaction context. For example, if Amazon Lex is not able to understand the user input, it uses a clarification prompt message (for more information, see the Error Handling section in the Amazon Lex console). Another example: if the intent requires confirmation before fulfillment, then Amazon Lex uses the confirmation prompt message in the intent configuration. If the code hook returns a message, Amazon Lex passes it as-is in its response to the client. </p>"]
    #[serde(rename="message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[doc="<p>Represents the options that the user has to respond to the current prompt. Response Card can come from the bot configuration (in the Amazon Lex console, choose the settings button next to a slot) or from a code hook (Lambda function). </p>"]
    #[serde(rename="responseCard")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub response_card: Option<ResponseCard>,
    #[doc="<p>A map of key-value pairs representing the session-specific context information.</p>"]
    #[serde(rename="sessionAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session_attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>If the <code>dialogState</code> value is <code>ElicitSlot</code>, returns the name of the slot for which Amazon Lex is eliciting a value. </p>"]
    #[serde(rename="slotToElicit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slot_to_elicit: Option<String>,
    #[doc="<p> The intent slots (name/value pairs) that Amazon Lex detected so far from the user input in the conversation. </p>"]
    #[serde(rename="slots")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slots: Option<::std::collections::HashMap<String, String>>,
}

#[doc="<p>If you configure a response card when creating your bots, Amazon Lex substitutes the session attributes and slot values that are available, and then returns it. The response card can also come from a Lambda function ( <code>dialogCodeHook</code> and <code>fulfillmentActivity</code> on an intent).</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ResponseCard {
    #[doc="<p>The content type of the response.</p>"]
    #[serde(rename="contentType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<String>,
    #[doc="<p>An array of attachment objects representing options.</p>"]
    #[serde(rename="genericAttachments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub generic_attachments: Option<Vec<GenericAttachment>>,
    #[doc="<p>The version of the response card format.</p>"]
    #[serde(rename="version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,
}

/// Errors returned by PostContent
#[derive(Debug, PartialEq)]
pub enum PostContentError {
    ///<p>Either the Amazon Lex bot is still building, or one of the dependent services (Amazon Polly, AWS Lambda) failed with an internal service error.</p>
    BadGateway(String),
    ///<p> Request validation failed, there is no usable message in the context, or the bot build failed. </p>
    BadRequest(String),
    ///<p> Two clients are using the same AWS account, Amazon Lex bot, and user ID. </p>
    Conflict(String),
    ///<p> One of the downstream dependencies, such as AWS Lambda or Amazon Polly, threw an exception. For example, if Amazon Lex does not have sufficient permissions to call a Lambda function, it results in Lambda throwing an exception. </p>
    DependencyFailed(String),
    ///<p>Internal service error. Retry the call.</p>
    InternalFailure(String),
    ///<p>Exceeded a limit.</p>
    LimitExceeded(String),
    ///<p>Lambda fulfilment function returned <code>DelegateDialogAction</code> to Amazon Lex without changing any slot values. </p>
    LoopDetected(String),
    ///<p>The accept header in the request does not have a valid value.</p>
    NotAcceptable(String),
    ///<p>The resource (such as the Amazon Lex bot or an alias) that is referred to is not found.</p>
    NotFound(String),
    ///<p>The input speech is too long.</p>
    RequestTimeout(String),
    ///<p>The Content-Type header (<code>PostContent</code> API) has an invalid value. </p>
    UnsupportedMediaType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PostContentError {
    pub fn from_body(body: &str) -> PostContentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadGatewayException" => {
                        PostContentError::BadGateway(String::from(error_message))
                    }
                    "BadRequestException" => {
                        PostContentError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => PostContentError::Conflict(String::from(error_message)),
                    "DependencyFailedException" => {
                        PostContentError::DependencyFailed(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        PostContentError::InternalFailure(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PostContentError::LimitExceeded(String::from(error_message))
                    }
                    "LoopDetectedException" => {
                        PostContentError::LoopDetected(String::from(error_message))
                    }
                    "NotAcceptableException" => {
                        PostContentError::NotAcceptable(String::from(error_message))
                    }
                    "NotFoundException" => PostContentError::NotFound(String::from(error_message)),
                    "RequestTimeoutException" => {
                        PostContentError::RequestTimeout(String::from(error_message))
                    }
                    "UnsupportedMediaTypeException" => {
                        PostContentError::UnsupportedMediaType(String::from(error_message))
                    }
                    "ValidationException" => {
                        PostContentError::Validation(error_message.to_string())
                    }
                    _ => PostContentError::Unknown(String::from(body)),
                }
            }
            Err(_) => PostContentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PostContentError {
    fn from(err: serde_json::error::Error) -> PostContentError {
        PostContentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PostContentError {
    fn from(err: CredentialsError) -> PostContentError {
        PostContentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PostContentError {
    fn from(err: HttpDispatchError) -> PostContentError {
        PostContentError::HttpDispatch(err)
    }
}
impl From<io::Error> for PostContentError {
    fn from(err: io::Error) -> PostContentError {
        PostContentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PostContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PostContentError {
    fn description(&self) -> &str {
        match *self {
            PostContentError::BadGateway(ref cause) => cause,
            PostContentError::BadRequest(ref cause) => cause,
            PostContentError::Conflict(ref cause) => cause,
            PostContentError::DependencyFailed(ref cause) => cause,
            PostContentError::InternalFailure(ref cause) => cause,
            PostContentError::LimitExceeded(ref cause) => cause,
            PostContentError::LoopDetected(ref cause) => cause,
            PostContentError::NotAcceptable(ref cause) => cause,
            PostContentError::NotFound(ref cause) => cause,
            PostContentError::RequestTimeout(ref cause) => cause,
            PostContentError::UnsupportedMediaType(ref cause) => cause,
            PostContentError::Validation(ref cause) => cause,
            PostContentError::Credentials(ref err) => err.description(),
            PostContentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PostContentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PostText
#[derive(Debug, PartialEq)]
pub enum PostTextError {
    ///<p>Either the Amazon Lex bot is still building, or one of the dependent services (Amazon Polly, AWS Lambda) failed with an internal service error.</p>
    BadGateway(String),
    ///<p> Request validation failed, there is no usable message in the context, or the bot build failed. </p>
    BadRequest(String),
    ///<p> Two clients are using the same AWS account, Amazon Lex bot, and user ID. </p>
    Conflict(String),
    ///<p> One of the downstream dependencies, such as AWS Lambda or Amazon Polly, threw an exception. For example, if Amazon Lex does not have sufficient permissions to call a Lambda function, it results in Lambda throwing an exception. </p>
    DependencyFailed(String),
    ///<p>Internal service error. Retry the call.</p>
    InternalFailure(String),
    ///<p>Exceeded a limit.</p>
    LimitExceeded(String),
    ///<p>Lambda fulfilment function returned <code>DelegateDialogAction</code> to Amazon Lex without changing any slot values. </p>
    LoopDetected(String),
    ///<p>The resource (such as the Amazon Lex bot or an alias) that is referred to is not found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PostTextError {
    pub fn from_body(body: &str) -> PostTextError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadGatewayException" => PostTextError::BadGateway(String::from(error_message)),
                    "BadRequestException" => PostTextError::BadRequest(String::from(error_message)),
                    "ConflictException" => PostTextError::Conflict(String::from(error_message)),
                    "DependencyFailedException" => {
                        PostTextError::DependencyFailed(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        PostTextError::InternalFailure(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PostTextError::LimitExceeded(String::from(error_message))
                    }
                    "LoopDetectedException" => {
                        PostTextError::LoopDetected(String::from(error_message))
                    }
                    "NotFoundException" => PostTextError::NotFound(String::from(error_message)),
                    "ValidationException" => PostTextError::Validation(error_message.to_string()),
                    _ => PostTextError::Unknown(String::from(body)),
                }
            }
            Err(_) => PostTextError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PostTextError {
    fn from(err: serde_json::error::Error) -> PostTextError {
        PostTextError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PostTextError {
    fn from(err: CredentialsError) -> PostTextError {
        PostTextError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PostTextError {
    fn from(err: HttpDispatchError) -> PostTextError {
        PostTextError::HttpDispatch(err)
    }
}
impl From<io::Error> for PostTextError {
    fn from(err: io::Error) -> PostTextError {
        PostTextError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PostTextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PostTextError {
    fn description(&self) -> &str {
        match *self {
            PostTextError::BadGateway(ref cause) => cause,
            PostTextError::BadRequest(ref cause) => cause,
            PostTextError::Conflict(ref cause) => cause,
            PostTextError::DependencyFailed(ref cause) => cause,
            PostTextError::InternalFailure(ref cause) => cause,
            PostTextError::LimitExceeded(ref cause) => cause,
            PostTextError::LoopDetected(ref cause) => cause,
            PostTextError::NotFound(ref cause) => cause,
            PostTextError::Validation(ref cause) => cause,
            PostTextError::Credentials(ref err) => err.description(),
            PostTextError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PostTextError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Lex Runtime Service API. Amazon Lex Runtime Service clients implement this trait.
pub trait LexRuntime {
    #[doc="<p> Sends user input (text or speech) to Amazon Lex. Clients use this API to send requests to Amazon Lex at runtime. Amazon Lex interprets the user input using the machine learning model that it built for the bot. </p> <p> In response, Amazon Lex returns the next message to convey to the user. Consider the following example messages: </p> <ul> <li> <p> For a user input \"I would like a pizza,\" Amazon Lex might return a response with a message eliciting slot data (for example, <code>PizzaSize</code>): \"What size pizza would you like?\". </p> </li> <li> <p> After the user provides all of the pizza order information, Amazon Lex might return a response with a message to get user confirmation: \"Order the pizza?\". </p> </li> <li> <p> After the user replies \"Yes\" to the confirmation prompt, Amazon Lex might return a conclusion statement: \"Thank you, your cheese pizza has been ordered.\". </p> </li> </ul> <p> Not all Amazon Lex messages require a response from the user. For example, conclusion statements do not require a response. Some messages require only a yes or no response. In addition to the <code>message</code>, Amazon Lex provides additional context about the message in the response that you can use to enhance client behavior, such as displaying the appropriate client user interface. Consider the following examples: </p> <ul> <li> <p> If the message is to elicit slot data, Amazon Lex returns the following context information: </p> <ul> <li> <p> <code>x-amz-lex-dialog-state</code> header set to <code>ElicitSlot</code> </p> </li> <li> <p> <code>x-amz-lex-intent-name</code> header set to the intent name in the current context </p> </li> <li> <p> <code>x-amz-lex-slot-to-elicit</code> header set to the slot name for which the <code>message</code> is eliciting information </p> </li> <li> <p> <code>x-amz-lex-slots</code> header set to a map of slots configured for the intent with their current values </p> </li> </ul> </li> <li> <p> If the message is a confirmation prompt, the <code>x-amz-lex-dialog-state</code> header is set to <code>Confirmation</code> and the <code>x-amz-lex-slot-to-elicit</code> header is omitted. </p> </li> <li> <p> If the message is a clarification prompt configured for the intent, indicating that the user intent is not understood, the <code>x-amz-dialog-state</code> header is set to <code>ElicitIntent</code> and the <code>x-amz-slot-to-elicit</code> header is omitted. </p> </li> </ul> <p> In addition, Amazon Lex also returns your application-specific <code>sessionAttributes</code>. For more information, see <a href=\"http://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html\">Managing Conversation Context</a>. </p>"]
    fn post_content(&self,
                    input: &PostContentRequest)
                    -> Result<PostContentResponse, PostContentError>;


    #[doc="<p>Sends user input (text-only) to Amazon Lex. Client applications can use this API to send requests to Amazon Lex at runtime. Amazon Lex then interprets the user input using the machine learning model it built for the bot. </p> <p> In response, Amazon Lex returns the next <code>message</code> to convey to the user an optional <code>responseCard</code> to display. Consider the following example messages: </p> <ul> <li> <p> For a user input \"I would like a pizza\", Amazon Lex might return a response with a message eliciting slot data (for example, PizzaSize): \"What size pizza would you like?\" </p> </li> <li> <p> After the user provides all of the pizza order information, Amazon Lex might return a response with a message to obtain user confirmation \"Proceed with the pizza order?\". </p> </li> <li> <p> After the user replies to a confirmation prompt with a \"yes\", Amazon Lex might return a conclusion statement: \"Thank you, your cheese pizza has been ordered.\". </p> </li> </ul> <p> Not all Amazon Lex messages require a user response. For example, a conclusion statement does not require a response. Some messages require only a \"yes\" or \"no\" user response. In addition to the <code>message</code>, Amazon Lex provides additional context about the message in the response that you might use to enhance client behavior, for example, to display the appropriate client user interface. These are the <code>slotToElicit</code>, <code>dialogState</code>, <code>intentName</code>, and <code>slots</code> fields in the response. Consider the following examples: </p> <ul> <li> <p>If the message is to elicit slot data, Amazon Lex returns the following context information:</p> <ul> <li> <p> <code>dialogState</code> set to ElicitSlot </p> </li> <li> <p> <code>intentName</code> set to the intent name in the current context </p> </li> <li> <p> <code>slotToElicit</code> set to the slot name for which the <code>message</code> is eliciting information </p> </li> <li> <p> <code>slots</code> set to a map of slots, configured for the intent, with currently known values </p> </li> </ul> </li> <li> <p> If the message is a confirmation prompt, the <code>dialogState</code> is set to ConfirmIntent and <code>SlotToElicit</code> is set to null. </p> </li> <li> <p>If the message is a clarification prompt (configured for the intent) that indicates that user intent is not understood, the <code>dialogState</code> is set to ElicitIntent and <code>slotToElicit</code> is set to null. </p> </li> </ul> <p> In addition, Amazon Lex also returns your application-specific <code>sessionAttributes</code>. For more information, see <a href=\"http://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html\">Managing Conversation Context</a>. </p>"]
    fn post_text(&self, input: &PostTextRequest) -> Result<PostTextResponse, PostTextError>;
}
/// A client for the Amazon Lex Runtime Service API.
pub struct LexRuntimeClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> LexRuntimeClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        LexRuntimeClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> LexRuntime for LexRuntimeClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p> Sends user input (text or speech) to Amazon Lex. Clients use this API to send requests to Amazon Lex at runtime. Amazon Lex interprets the user input using the machine learning model that it built for the bot. </p> <p> In response, Amazon Lex returns the next message to convey to the user. Consider the following example messages: </p> <ul> <li> <p> For a user input \"I would like a pizza,\" Amazon Lex might return a response with a message eliciting slot data (for example, <code>PizzaSize</code>): \"What size pizza would you like?\". </p> </li> <li> <p> After the user provides all of the pizza order information, Amazon Lex might return a response with a message to get user confirmation: \"Order the pizza?\". </p> </li> <li> <p> After the user replies \"Yes\" to the confirmation prompt, Amazon Lex might return a conclusion statement: \"Thank you, your cheese pizza has been ordered.\". </p> </li> </ul> <p> Not all Amazon Lex messages require a response from the user. For example, conclusion statements do not require a response. Some messages require only a yes or no response. In addition to the <code>message</code>, Amazon Lex provides additional context about the message in the response that you can use to enhance client behavior, such as displaying the appropriate client user interface. Consider the following examples: </p> <ul> <li> <p> If the message is to elicit slot data, Amazon Lex returns the following context information: </p> <ul> <li> <p> <code>x-amz-lex-dialog-state</code> header set to <code>ElicitSlot</code> </p> </li> <li> <p> <code>x-amz-lex-intent-name</code> header set to the intent name in the current context </p> </li> <li> <p> <code>x-amz-lex-slot-to-elicit</code> header set to the slot name for which the <code>message</code> is eliciting information </p> </li> <li> <p> <code>x-amz-lex-slots</code> header set to a map of slots configured for the intent with their current values </p> </li> </ul> </li> <li> <p> If the message is a confirmation prompt, the <code>x-amz-lex-dialog-state</code> header is set to <code>Confirmation</code> and the <code>x-amz-lex-slot-to-elicit</code> header is omitted. </p> </li> <li> <p> If the message is a clarification prompt configured for the intent, indicating that the user intent is not understood, the <code>x-amz-dialog-state</code> header is set to <code>ElicitIntent</code> and the <code>x-amz-slot-to-elicit</code> header is omitted. </p> </li> </ul> <p> In addition, Amazon Lex also returns your application-specific <code>sessionAttributes</code>. For more information, see <a href=\"http://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html\">Managing Conversation Context</a>. </p>"]
    fn post_content(&self,
                    input: &PostContentRequest)
                    -> Result<PostContentResponse, PostContentError> {
        let request_uri = format!("/bot/{bot_name}/alias/{bot_alias}/user/{user_id}/content",
                                  bot_alias = input.bot_alias,
                                  bot_name = input.bot_name,
                                  user_id = input.user_id);

        let mut request = SignedRequest::new("POST", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("runtime.lex".to_string());
        let encoded = Some(input.input_stream.to_owned());
        request.set_payload(encoded);

        if let Some(ref accept) = input.accept {
            request.add_header("Accept", &accept.to_string());
        }
        request.add_header("Content-Type", &input.content_type);

        if let Some(ref session_attributes) = input.session_attributes {
            request.add_header("x-amz-lex-session-attributes",
                               &session_attributes.to_string());
        }


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut result = PostContentResponse::default();

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                result.audio_stream = Some(body);

                if let Some(content_type) = response.headers.get("Content-Type") {
                    let value = content_type.to_owned();
                    result.content_type = Some(value)
                };
                if let Some(dialog_state) = response.headers.get("x-amz-lex-dialog-state") {
                    let value = dialog_state.to_owned();
                    result.dialog_state = Some(value)
                };
                if let Some(input_transcript) =
                    response.headers.get("x-amz-lex-input-transcript") {
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
                if let Some(session_attributes) =
                    response.headers.get("x-amz-lex-session-attributes") {
                    let value = session_attributes.to_owned();
                    result.session_attributes = Some(value)
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
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PostContentError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Sends user input (text-only) to Amazon Lex. Client applications can use this API to send requests to Amazon Lex at runtime. Amazon Lex then interprets the user input using the machine learning model it built for the bot. </p> <p> In response, Amazon Lex returns the next <code>message</code> to convey to the user an optional <code>responseCard</code> to display. Consider the following example messages: </p> <ul> <li> <p> For a user input \"I would like a pizza\", Amazon Lex might return a response with a message eliciting slot data (for example, PizzaSize): \"What size pizza would you like?\" </p> </li> <li> <p> After the user provides all of the pizza order information, Amazon Lex might return a response with a message to obtain user confirmation \"Proceed with the pizza order?\". </p> </li> <li> <p> After the user replies to a confirmation prompt with a \"yes\", Amazon Lex might return a conclusion statement: \"Thank you, your cheese pizza has been ordered.\". </p> </li> </ul> <p> Not all Amazon Lex messages require a user response. For example, a conclusion statement does not require a response. Some messages require only a \"yes\" or \"no\" user response. In addition to the <code>message</code>, Amazon Lex provides additional context about the message in the response that you might use to enhance client behavior, for example, to display the appropriate client user interface. These are the <code>slotToElicit</code>, <code>dialogState</code>, <code>intentName</code>, and <code>slots</code> fields in the response. Consider the following examples: </p> <ul> <li> <p>If the message is to elicit slot data, Amazon Lex returns the following context information:</p> <ul> <li> <p> <code>dialogState</code> set to ElicitSlot </p> </li> <li> <p> <code>intentName</code> set to the intent name in the current context </p> </li> <li> <p> <code>slotToElicit</code> set to the slot name for which the <code>message</code> is eliciting information </p> </li> <li> <p> <code>slots</code> set to a map of slots, configured for the intent, with currently known values </p> </li> </ul> </li> <li> <p> If the message is a confirmation prompt, the <code>dialogState</code> is set to ConfirmIntent and <code>SlotToElicit</code> is set to null. </p> </li> <li> <p>If the message is a clarification prompt (configured for the intent) that indicates that user intent is not understood, the <code>dialogState</code> is set to ElicitIntent and <code>slotToElicit</code> is set to null. </p> </li> </ul> <p> In addition, Amazon Lex also returns your application-specific <code>sessionAttributes</code>. For more information, see <a href=\"http://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html\">Managing Conversation Context</a>. </p>"]
    fn post_text(&self, input: &PostTextRequest) -> Result<PostTextResponse, PostTextError> {
        let request_uri = format!("/bot/{bot_name}/alias/{bot_alias}/user/{user_id}/text",
                                  bot_alias = input.bot_alias,
                                  bot_name = input.bot_name,
                                  user_id = input.user_id);

        let mut request = SignedRequest::new("POST", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("runtime.lex".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);



        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<PostTextResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PostTextError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
